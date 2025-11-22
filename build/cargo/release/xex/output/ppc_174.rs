pub fn sub_82E555E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E555E8 size=12
    let mut pc: u32 = 0x82E555E8;
    'dispatch: loop {
        match pc {
            0x82E555E8 => {
    //   block [0x82E555E8..0x82E555F4)
	// 82E555E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E555EC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E555F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E555F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E555F8 size=12
    let mut pc: u32 = 0x82E555F8;
    'dispatch: loop {
        match pc {
            0x82E555F8 => {
    //   block [0x82E555F8..0x82E55604)
	// 82E555F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E555FC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E55600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55608 size=12
    let mut pc: u32 = 0x82E55608;
    'dispatch: loop {
        match pc {
            0x82E55608 => {
    //   block [0x82E55608..0x82E55614)
	// 82E55608: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5560C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E55610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55618 size=4
    let mut pc: u32 = 0x82E55618;
    'dispatch: loop {
        match pc {
            0x82E55618 => {
    //   block [0x82E55618..0x82E5561C)
	// 82E55618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55620 size=40
    let mut pc: u32 = 0x82E55620;
    'dispatch: loop {
        match pc {
            0x82E55620 => {
    //   block [0x82E55620..0x82E55648)
	// 82E55620: 89650010  lbz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E55624: 89460010  lbz r10, 0x10(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E55628: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E5562C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E55630: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82E55634: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82E55638: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E5563C: 4099000C  ble cr6, 0x82e55648
	if !ctx.cr[6].gt {
		sub_82E55648(ctx, base);
		return;
	}
	// 82E55640: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82E55644: 4800000C  b 0x82e55650
	sub_82E55648(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55648 size=112
    let mut pc: u32 = 0x82E55648;
    'dispatch: loop {
        match pc {
            0x82E55648 => {
    //   block [0x82E55648..0x82E556B8)
	// 82E55648: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82E5564C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E55650: 80E40030  lwz r7, 0x30(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E55654: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55658: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E5565C: 40990044  ble cr6, 0x82e556a0
	if !ctx.cr[6].gt {
	pc = 0x82E556A0; continue 'dispatch;
	}
	// 82E55660: 8124002C  lwz r9, 0x2c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E55664: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55668: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82E5566C: 409A0014  bne cr6, 0x82e55680
	if !ctx.cr[6].eq {
	pc = 0x82E55680; continue 'dispatch;
	}
	// 82E55670: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55674: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E55678: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E5567C: 419A0008  beq cr6, 0x82e55684
	if ctx.cr[6].eq {
	pc = 0x82E55684; continue 'dispatch;
	}
	// 82E55680: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E55684: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82E55688: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E5568C: 409A0018  bne cr6, 0x82e556a4
	if !ctx.cr[6].eq {
	pc = 0x82E556A4; continue 'dispatch;
	}
	// 82E55690: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55694: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82E55698: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82E5569C: 4198FFC8  blt cr6, 0x82e55664
	if ctx.cr[6].lt {
	pc = 0x82E55664; continue 'dispatch;
	}
	// 82E556A0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E556A4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E556A8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E556AC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E556B0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E556B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E556B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E556B8 size=72
    let mut pc: u32 = 0x82E556B8;
    'dispatch: loop {
        match pc {
            0x82E556B8 => {
    //   block [0x82E556B8..0x82E55700)
	// 82E556B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E556BC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E556C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E556C4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E556C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E556CC: 4099006C  ble cr6, 0x82e55738
	if !ctx.cr[6].gt {
		sub_82E55700(ctx, base);
		return;
	}
	// 82E556D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E556D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E556D8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82E556DC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E556E0: 7F091840  cmplw cr6, r9, r3
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82E556E4: 419A001C  beq cr6, 0x82e55700
	if ctx.cr[6].eq {
		sub_82E55700(ctx, base);
		return;
	}
	// 82E556E8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E556EC: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82E556F0: 419A0010  beq cr6, 0x82e55700
	if ctx.cr[6].eq {
		sub_82E55700(ctx, base);
		return;
	}
	// 82E556F4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82E556F8: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82E556FC: 48000030  b 0x82e5572c
	sub_82E55700(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55700 size=76
    let mut pc: u32 = 0x82E55700;
    'dispatch: loop {
        match pc {
            0x82E55700 => {
    //   block [0x82E55700..0x82E5574C)
	// 82E55700: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55704: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55708: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E5570C: 7CA85214  add r5, r8, r10
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82E55710: 55271838  slwi r7, r9, 3
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E55714: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82E55718: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5571C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55720: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E55724: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55728: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E5572C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55730: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E55734: 4198FFA0  blt cr6, 0x82e556d4
	if ctx.cr[6].lt {
		sub_82E556B8(ctx, base);
		return;
	}
	// 82E55738: 354BFFD0  addic. r10, r11, -0x30
	ctx.xer.ca = (ctx.r[11].u32 > (!(-48 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5573C: 40820008  bne 0x82e55744
	if !ctx.cr[0].eq {
	pc = 0x82E55744; continue 'dispatch;
	}
	// 82E55740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55744: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E55748: 4BF306D0  b 0x82d85e18
	sub_82D85E18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55750 size=8
    let mut pc: u32 = 0x82E55750;
    'dispatch: loop {
        match pc {
            0x82E55750 => {
    //   block [0x82E55750..0x82E55758)
	// 82E55750: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E55754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55758 size=168
    let mut pc: u32 = 0x82E55758;
    'dispatch: loop {
        match pc {
            0x82E55758 => {
    //   block [0x82E55758..0x82E55800)
	// 82E55758: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5575C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E55760: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E55764: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E55768: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5576C: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E55770: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E55774: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E55778: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E5577C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E55780: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E55784: 38CB6614  addi r6, r11, 0x6614
	ctx.r[6].s64 = ctx.r[11].s64 + 26132;
	// 82E55788: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5578C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E55790: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E55794: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E55798: 38AB6608  addi r5, r11, 0x6608
	ctx.r[5].s64 = ctx.r[11].s64 + 26120;
	// 82E5579C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E557A0: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E557A4: 394B65F4  addi r10, r11, 0x65f4
	ctx.r[10].s64 = ctx.r[11].s64 + 26100;
	// 82E557A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E557AC: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E557B0: 38E75D40  addi r7, r7, 0x5d40
	ctx.r[7].s64 = ctx.r[7].s64 + 23872;
	// 82E557B4: 392B65E8  addi r9, r11, 0x65e8
	ctx.r[9].s64 = ctx.r[11].s64 + 26088;
	// 82E557B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E557BC: 388B65DC  addi r4, r11, 0x65dc
	ctx.r[4].s64 = ctx.r[11].s64 + 26076;
	// 82E557C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E557C4: 390B65C4  addi r8, r11, 0x65c4
	ctx.r[8].s64 = ctx.r[11].s64 + 26052;
	// 82E557C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E557CC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E557D0: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E557D4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E557D8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E557DC: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E557E0: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E557E4: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E557E8: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E557EC: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E557F0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E557F4: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E557F8: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E557FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55800 size=440
    let mut pc: u32 = 0x82E55800;
    'dispatch: loop {
        match pc {
            0x82E55800 => {
    //   block [0x82E55800..0x82E559B8)
	// 82E55800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55804: 4BE53C05  bl 0x82ca9408
	ctx.lr = 0x82E55808;
	sub_82CA93D0(ctx, base);
	// 82E55808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5580C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55810: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E55814: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E55818: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5581C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E55820: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E55824: 80BF0038  lwz r5, 0x38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E55828: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5582C: 396B6614  addi r11, r11, 0x6614
	ctx.r[11].s64 = ctx.r[11].s64 + 26132;
	// 82E55830: 394A6608  addi r10, r10, 0x6608
	ctx.r[10].s64 = ctx.r[10].s64 + 26120;
	// 82E55834: 392965F4  addi r9, r9, 0x65f4
	ctx.r[9].s64 = ctx.r[9].s64 + 26100;
	// 82E55838: 390865E8  addi r8, r8, 0x65e8
	ctx.r[8].s64 = ctx.r[8].s64 + 26088;
	// 82E5583C: 38E765DC  addi r7, r7, 0x65dc
	ctx.r[7].s64 = ctx.r[7].s64 + 26076;
	// 82E55840: 38C665C4  addi r6, r6, 0x65c4
	ctx.r[6].s64 = ctx.r[6].s64 + 26052;
	// 82E55844: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55848: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5584C: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82E55850: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E55854: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E55858: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E5585C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E55860: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82E55864: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82E55868: 409900D4  ble cr6, 0x82e5593c
	if !ctx.cr[6].gt {
	pc = 0x82E5593C; continue 'dispatch;
	}
	// 82E5586C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E55870: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E55874: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E55878: 4BFFD989  bl 0x82e53200
	ctx.lr = 0x82E5587C;
	sub_82E53200(ctx, base);
	// 82E5587C: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55884: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55888: 40990040  ble cr6, 0x82e558c8
	if !ctx.cr[6].gt {
	pc = 0x82E558C8; continue 'dispatch;
	}
	// 82E5588C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55890: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55894: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E55898: 419A0018  beq cr6, 0x82e558b0
	if ctx.cr[6].eq {
	pc = 0x82E558B0; continue 'dispatch;
	}
	// 82E5589C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E558A0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E558A4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E558A8: 4198FFE8  blt cr6, 0x82e55890
	if ctx.cr[6].lt {
	pc = 0x82E55890; continue 'dispatch;
	}
	// 82E558AC: 4800001C  b 0x82e558c8
	pc = 0x82E558C8; continue 'dispatch;
	// 82E558B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E558B4: 41980014  blt cr6, 0x82e558c8
	if ctx.cr[6].lt {
	pc = 0x82E558C8; continue 'dispatch;
	}
	// 82E558B8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E558BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E558C0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E558C4: 4BF30555  bl 0x82d85e18
	ctx.lr = 0x82E558C8;
	sub_82D85E18(ctx, base);
	// 82E558C8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E558CC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E558D0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E558D4: 4BFFD92D  bl 0x82e53200
	ctx.lr = 0x82E558D8;
	sub_82E53200(ctx, base);
	// 82E558D8: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E558DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E558E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E558E4: 40990044  ble cr6, 0x82e55928
	if !ctx.cr[6].gt {
	pc = 0x82E55928; continue 'dispatch;
	}
	// 82E558E8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E558EC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E558F0: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E558F4: 419A0018  beq cr6, 0x82e5590c
	if ctx.cr[6].eq {
	pc = 0x82E5590C; continue 'dispatch;
	}
	// 82E558F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E558FC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E55900: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E55904: 4198FFE8  blt cr6, 0x82e558ec
	if ctx.cr[6].lt {
	pc = 0x82E558EC; continue 'dispatch;
	}
	// 82E55908: 48000020  b 0x82e55928
	pc = 0x82E55928; continue 'dispatch;
	// 82E5590C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55910: 41980018  blt cr6, 0x82e55928
	if ctx.cr[6].lt {
	pc = 0x82E55928; continue 'dispatch;
	}
	// 82E55914: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E55918: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5591C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E55920: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55924: 4BF304F5  bl 0x82d85e18
	ctx.lr = 0x82E55928;
	sub_82D85E18(ctx, base);
	// 82E55928: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5592C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E55930: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E55934: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E55938: 4198FF38  blt cr6, 0x82e55870
	if ctx.cr[6].lt {
	pc = 0x82E55870; continue 'dispatch;
	}
	// 82E5593C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E55940: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55944: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55948: 409A0020  bne cr6, 0x82e55968
	if !ctx.cr[6].eq {
	pc = 0x82E55968; continue 'dispatch;
	}
	// 82E5594C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55950: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E55954: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E55958: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5595C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E55960: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E55964: 4BEFF965  bl 0x82d552c8
	ctx.lr = 0x82E55968;
	sub_82D552C8(ctx, base);
	// 82E55968: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5596C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E55970: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E55974: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E55978: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5597C: 394A8604  addi r10, r10, -0x79fc
	ctx.r[10].s64 = ctx.r[10].s64 + -31228;
	// 82E55980: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E55984: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E55988: 3CC08202  lis r6, -0x7dfe
	ctx.r[6].s64 = -2113798144;
	// 82E5598C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55990: 39088610  addi r8, r8, -0x79f0
	ctx.r[8].s64 = ctx.r[8].s64 + -31216;
	// 82E55994: 38E785E8  addi r7, r7, -0x7a18
	ctx.r[7].s64 = ctx.r[7].s64 + -31256;
	// 82E55998: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E5599C: 38C639E0  addi r6, r6, 0x39e0
	ctx.r[6].s64 = ctx.r[6].s64 + 14816;
	// 82E559A0: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E559A4: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82E559A8: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E559AC: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E559B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E559B4: 4BE53AA4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E559B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E559B8 size=444
    let mut pc: u32 = 0x82E559B8;
    'dispatch: loop {
        match pc {
            0x82E559B8 => {
    //   block [0x82E559B8..0x82E55B74)
	// 82E559B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E559BC: 4BE53A4D  bl 0x82ca9408
	ctx.lr = 0x82E559C0;
	sub_82CA93D0(ctx, base);
	// 82E559C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E559C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E559C8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E559CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E559D0: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E559D4: 40990010  ble cr6, 0x82e559e4
	if !ctx.cr[6].gt {
	pc = 0x82E559E4; continue 'dispatch;
	}
	// 82E559D8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E559DC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E559E0: 4800000C  b 0x82e559ec
	pc = 0x82E559EC; continue 'dispatch;
	// 82E559E4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E559E8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E559EC: 3BFD0034  addi r31, r29, 0x34
	ctx.r[31].s64 = ctx.r[29].s64 + 52;
	// 82E559F0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82E559F4: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82E559F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E559FC: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55A00: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E55A04: 40990050  ble cr6, 0x82e55a54
	if !ctx.cr[6].gt {
	pc = 0x82E55A54; continue 'dispatch;
	}
	// 82E55A08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55A0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55A10: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E55A14: 409A0014  bne cr6, 0x82e55a28
	if !ctx.cr[6].eq {
	pc = 0x82E55A28; continue 'dispatch;
	}
	// 82E55A18: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55A1C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82E55A20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E55A24: 419A0008  beq cr6, 0x82e55a2c
	if ctx.cr[6].eq {
	pc = 0x82E55A2C; continue 'dispatch;
	}
	// 82E55A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E55A2C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E55A30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55A34: 409A0018  bne cr6, 0x82e55a4c
	if !ctx.cr[6].eq {
	pc = 0x82E55A4C; continue 'dispatch;
	}
	// 82E55A38: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E55A3C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E55A40: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E55A44: 4198FFC8  blt cr6, 0x82e55a0c
	if ctx.cr[6].lt {
	pc = 0x82E55A0C; continue 'dispatch;
	}
	// 82E55A48: 4800000C  b 0x82e55a54
	pc = 0x82E55A54; continue 'dispatch;
	// 82E55A4C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55A50: 4199011C  bgt cr6, 0x82e55b6c
	if ctx.cr[6].gt {
	pc = 0x82E55B6C; continue 'dispatch;
	}
	// 82E55A54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55A58: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55A5C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E55A60: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E55A64: 409A0010  bne cr6, 0x82e55a74
	if !ctx.cr[6].eq {
	pc = 0x82E55A74; continue 'dispatch;
	}
	// 82E55A68: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82E55A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55A70: 4BF01529  bl 0x82d56f98
	ctx.lr = 0x82E55A74;
	sub_82D56F98(ctx, base);
	// 82E55A74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55A78: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E55A7C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55A80: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E55A84: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E55A88: 7D4B492A  stdx r10, r11, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u64) };
	// 82E55A8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55A90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55A94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E55A98: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82E55A9C: 409A0008  bne cr6, 0x82e55aa4
	if !ctx.cr[6].eq {
	pc = 0x82E55AA4; continue 'dispatch;
	}
	// 82E55AA0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E55AA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55AA8: 4BFFD759  bl 0x82e53200
	ctx.lr = 0x82E55AAC;
	sub_82E53200(ctx, base);
	// 82E55AAC: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55AB4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55AB8: 40990030  ble cr6, 0x82e55ae8
	if !ctx.cr[6].gt {
	pc = 0x82E55AE8; continue 'dispatch;
	}
	// 82E55ABC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55AC0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55AC4: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82E55AC8: 419A0018  beq cr6, 0x82e55ae0
	if ctx.cr[6].eq {
	pc = 0x82E55AE0; continue 'dispatch;
	}
	// 82E55ACC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55AD0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E55AD4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E55AD8: 4198FFE8  blt cr6, 0x82e55ac0
	if ctx.cr[6].lt {
	pc = 0x82E55AC0; continue 'dispatch;
	}
	// 82E55ADC: 4800000C  b 0x82e55ae8
	pc = 0x82E55AE8; continue 'dispatch;
	// 82E55AE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55AE4: 4098001C  bge cr6, 0x82e55b00
	if !ctx.cr[6].lt {
	pc = 0x82E55B00; continue 'dispatch;
	}
	// 82E55AE8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E55AEC: 389D0030  addi r4, r29, 0x30
	ctx.r[4].s64 = ctx.r[29].s64 + 48;
	// 82E55AF0: 409A0008  bne cr6, 0x82e55af8
	if !ctx.cr[6].eq {
	pc = 0x82E55AF8; continue 'dispatch;
	}
	// 82E55AF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E55AF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55AFC: 4BF301ED  bl 0x82d85ce8
	ctx.lr = 0x82E55B00;
	sub_82D85CE8(ctx, base);
	// 82E55B00: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E55B04: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82E55B08: 409A0008  bne cr6, 0x82e55b10
	if !ctx.cr[6].eq {
	pc = 0x82E55B10; continue 'dispatch;
	}
	// 82E55B0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E55B10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E55B14: 4BFFD6ED  bl 0x82e53200
	ctx.lr = 0x82E55B18;
	sub_82E53200(ctx, base);
	// 82E55B18: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55B1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55B20: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55B24: 40990030  ble cr6, 0x82e55b54
	if !ctx.cr[6].gt {
	pc = 0x82E55B54; continue 'dispatch;
	}
	// 82E55B28: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55B2C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55B30: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82E55B34: 419A0018  beq cr6, 0x82e55b4c
	if ctx.cr[6].eq {
	pc = 0x82E55B4C; continue 'dispatch;
	}
	// 82E55B38: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55B3C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E55B40: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E55B44: 4198FFE8  blt cr6, 0x82e55b2c
	if ctx.cr[6].lt {
	pc = 0x82E55B2C; continue 'dispatch;
	}
	// 82E55B48: 4800000C  b 0x82e55b54
	pc = 0x82E55B54; continue 'dispatch;
	// 82E55B4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55B50: 4098001C  bge cr6, 0x82e55b6c
	if !ctx.cr[6].lt {
	pc = 0x82E55B6C; continue 'dispatch;
	}
	// 82E55B54: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E55B58: 389D0030  addi r4, r29, 0x30
	ctx.r[4].s64 = ctx.r[29].s64 + 48;
	// 82E55B5C: 409A0008  bne cr6, 0x82e55b64
	if !ctx.cr[6].eq {
	pc = 0x82E55B64; continue 'dispatch;
	}
	// 82E55B60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E55B64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E55B68: 4BF30181  bl 0x82d85ce8
	ctx.lr = 0x82E55B6C;
	sub_82D85CE8(ctx, base);
	// 82E55B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E55B70: 4BE538E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55B78 size=424
    let mut pc: u32 = 0x82E55B78;
    'dispatch: loop {
        match pc {
            0x82E55B78 => {
    //   block [0x82E55B78..0x82E55D20)
	// 82E55B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55B7C: 4BE53891  bl 0x82ca940c
	ctx.lr = 0x82E55B80;
	sub_82CA93D0(ctx, base);
	// 82E55B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55B84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E55B88: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E55B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55B90: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E55B94: 40990010  ble cr6, 0x82e55ba4
	if !ctx.cr[6].gt {
	pc = 0x82E55BA4; continue 'dispatch;
	}
	// 82E55B98: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E55B9C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E55BA0: 4800000C  b 0x82e55bac
	pc = 0x82E55BAC; continue 'dispatch;
	// 82E55BA4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E55BA8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E55BAC: 811F0038  lwz r8, 0x38(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E55BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55BB4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E55BB8: 40990160  ble cr6, 0x82e55d18
	if !ctx.cr[6].gt {
	pc = 0x82E55D18; continue 'dispatch;
	}
	// 82E55BBC: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E55BC0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55BC4: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E55BC8: 409A0014  bne cr6, 0x82e55bdc
	if !ctx.cr[6].eq {
	pc = 0x82E55BDC; continue 'dispatch;
	}
	// 82E55BCC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55BD0: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82E55BD4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E55BD8: 419A0008  beq cr6, 0x82e55be0
	if ctx.cr[6].eq {
	pc = 0x82E55BE0; continue 'dispatch;
	}
	// 82E55BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E55BE0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82E55BE4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55BE8: 409A001C  bne cr6, 0x82e55c04
	if !ctx.cr[6].eq {
	pc = 0x82E55C04; continue 'dispatch;
	}
	// 82E55BEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55BF0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82E55BF4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E55BF8: 4198FFC8  blt cr6, 0x82e55bc0
	if ctx.cr[6].lt {
	pc = 0x82E55BC0; continue 'dispatch;
	}
	// 82E55BFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55C00: 4BE5385C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82E55C04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55C08: 41980110  blt cr6, 0x82e55d18
	if ctx.cr[6].lt {
	pc = 0x82E55D18; continue 'dispatch;
	}
	// 82E55C0C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E55C10: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E55C14: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E55C18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55C1C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E55C20: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E55C24: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E55C28: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E55C2C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E55C30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55C34: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E55C38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55C3C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E55C40: 4BFFD5C1  bl 0x82e53200
	ctx.lr = 0x82E55C44;
	sub_82E53200(ctx, base);
	// 82E55C44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E55C48: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82E55C4C: 409A0008  bne cr6, 0x82e55c54
	if !ctx.cr[6].eq {
	pc = 0x82E55C54; continue 'dispatch;
	}
	// 82E55C50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E55C54: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E55C5C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55C60: 40990048  ble cr6, 0x82e55ca8
	if !ctx.cr[6].gt {
	pc = 0x82E55CA8; continue 'dispatch;
	}
	// 82E55C64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55C68: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55C6C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E55C70: 419A0018  beq cr6, 0x82e55c88
	if ctx.cr[6].eq {
	pc = 0x82E55C88; continue 'dispatch;
	}
	// 82E55C74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E55C78: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E55C7C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E55C80: 4198FFE8  blt cr6, 0x82e55c68
	if ctx.cr[6].lt {
	pc = 0x82E55C68; continue 'dispatch;
	}
	// 82E55C84: 48000024  b 0x82e55ca8
	pc = 0x82E55CA8; continue 'dispatch;
	// 82E55C88: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55C8C: 4198001C  blt cr6, 0x82e55ca8
	if ctx.cr[6].lt {
	pc = 0x82E55CA8; continue 'dispatch;
	}
	// 82E55C90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E55C94: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E55C98: 409A0008  bne cr6, 0x82e55ca0
	if !ctx.cr[6].eq {
	pc = 0x82E55CA0; continue 'dispatch;
	}
	// 82E55C9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E55CA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55CA4: 4BF30175  bl 0x82d85e18
	ctx.lr = 0x82E55CA8;
	sub_82D85E18(ctx, base);
	// 82E55CA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E55CAC: 4BFFD555  bl 0x82e53200
	ctx.lr = 0x82E55CB0;
	sub_82E53200(ctx, base);
	// 82E55CB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E55CB4: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82E55CB8: 409A0008  bne cr6, 0x82e55cc0
	if !ctx.cr[6].eq {
	pc = 0x82E55CC0; continue 'dispatch;
	}
	// 82E55CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E55CC0: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55CC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E55CC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E55CCC: 4099004C  ble cr6, 0x82e55d18
	if !ctx.cr[6].gt {
	pc = 0x82E55D18; continue 'dispatch;
	}
	// 82E55CD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55CD4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55CD8: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E55CDC: 419A001C  beq cr6, 0x82e55cf8
	if ctx.cr[6].eq {
	pc = 0x82E55CF8; continue 'dispatch;
	}
	// 82E55CE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E55CE4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E55CE8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E55CEC: 4198FFE8  blt cr6, 0x82e55cd4
	if ctx.cr[6].lt {
	pc = 0x82E55CD4; continue 'dispatch;
	}
	// 82E55CF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55CF4: 4BE53768  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82E55CF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55CFC: 4198001C  blt cr6, 0x82e55d18
	if ctx.cr[6].lt {
	pc = 0x82E55D18; continue 'dispatch;
	}
	// 82E55D00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E55D04: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E55D08: 409A0008  bne cr6, 0x82e55d10
	if !ctx.cr[6].eq {
	pc = 0x82E55D10; continue 'dispatch;
	}
	// 82E55D0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E55D10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E55D14: 4BF30105  bl 0x82d85e18
	ctx.lr = 0x82E55D18;
	sub_82D85E18(ctx, base);
	// 82E55D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55D1C: 4BE53740  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55D20 size=380
    let mut pc: u32 = 0x82E55D20;
    'dispatch: loop {
        match pc {
            0x82E55D20 => {
    //   block [0x82E55D20..0x82E55E9C)
	// 82E55D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55D24: 4BE536E5  bl 0x82ca9408
	ctx.lr = 0x82E55D28;
	sub_82CA93D0(ctx, base);
	// 82E55D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55D2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E55D30: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55D34: 556B17FE  rlwinm r11, r11, 2, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E55D38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55D3C: 409A0100  bne cr6, 0x82e55e3c
	if !ctx.cr[6].eq {
	pc = 0x82E55E3C; continue 'dispatch;
	}
	// 82E55D40: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55D44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E55D48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55D4C: 4099006C  ble cr6, 0x82e55db8
	if !ctx.cr[6].gt {
	pc = 0x82E55DB8; continue 'dispatch;
	}
	// 82E55D50: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E55D54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55D58: 7CBF582E  lwzx r5, r31, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E55D5C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E55D60: 419A0044  beq cr6, 0x82e55da4
	if ctx.cr[6].eq {
	pc = 0x82E55DA4; continue 'dispatch;
	}
	// 82E55D64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55D68: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E55D6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E55D70: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E55D74: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E55D78: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E55D7C: 41980010  blt cr6, 0x82e55d8c
	if ctx.cr[6].lt {
	pc = 0x82E55D8C; continue 'dispatch;
	}
	// 82E55D80: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E55D84: 4BEFF3A5  bl 0x82d55128
	ctx.lr = 0x82E55D88;
	sub_82D55128(ctx, base);
	// 82E55D88: 4800001C  b 0x82e55da4
	pc = 0x82E55DA4; continue 'dispatch;
	// 82E55D8C: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E55D90: 81430098  lwz r10, 0x98(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E55D94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55D98: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82E55D9C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E55DA0: 90A30098  stw r5, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[5].u32 ) };
	// 82E55DA4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55DA8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E55DAC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E55DB0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E55DB4: 4198FFA0  blt cr6, 0x82e55d54
	if ctx.cr[6].lt {
	pc = 0x82E55D54; continue 'dispatch;
	}
	// 82E55DB8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E55DBC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E55DC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55DC4: 40990078  ble cr6, 0x82e55e3c
	if !ctx.cr[6].gt {
	pc = 0x82E55E3C; continue 'dispatch;
	}
	// 82E55DC8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E55DCC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55DD0: 7FFD582E  lwzx r31, r29, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E55DD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E55DD8: 419A0050  beq cr6, 0x82e55e28
	if ctx.cr[6].eq {
	pc = 0x82E55E28; continue 'dispatch;
	}
	// 82E55DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55DE0: 4BFFFF41  bl 0x82e55d20
	ctx.lr = 0x82E55DE4;
	sub_82E55D20(ctx, base);
	// 82E55DE4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55DE8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E55DEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E55DF0: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E55DF4: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E55DF8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E55DFC: 41980014  blt cr6, 0x82e55e10
	if ctx.cr[6].lt {
	pc = 0x82E55E10; continue 'dispatch;
	}
	// 82E55E00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E55E04: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E55E08: 4BEFF321  bl 0x82d55128
	ctx.lr = 0x82E55E0C;
	sub_82D55128(ctx, base);
	// 82E55E0C: 4800001C  b 0x82e55e28
	pc = 0x82E55E28; continue 'dispatch;
	// 82E55E10: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E55E14: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E55E18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E55E1C: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E55E20: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E55E24: 93E30050  stw r31, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E55E28: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E55E2C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E55E30: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E55E34: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E55E38: 4198FF94  blt cr6, 0x82e55dcc
	if ctx.cr[6].lt {
	pc = 0x82E55DCC; continue 'dispatch;
	}
	// 82E55E3C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E55E40: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55E44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55E48: 409A0020  bne cr6, 0x82e55e68
	if !ctx.cr[6].eq {
	pc = 0x82E55E68; continue 'dispatch;
	}
	// 82E55E4C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55E50: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E55E54: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E55E58: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55E5C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E55E60: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E55E64: 4BEFF465  bl 0x82d552c8
	ctx.lr = 0x82E55E68;
	sub_82D552C8(ctx, base);
	// 82E55E68: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55E6C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55E70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55E74: 409A0020  bne cr6, 0x82e55e94
	if !ctx.cr[6].eq {
	pc = 0x82E55E94; continue 'dispatch;
	}
	// 82E55E78: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55E7C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E55E80: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E55E84: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55E88: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E55E8C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E55E90: 4BEFF439  bl 0x82d552c8
	ctx.lr = 0x82E55E94;
	sub_82D552C8(ctx, base);
	// 82E55E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E55E98: 4BE535C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55EA0 size=236
    let mut pc: u32 = 0x82E55EA0;
    'dispatch: loop {
        match pc {
            0x82E55EA0 => {
    //   block [0x82E55EA0..0x82E55F8C)
	// 82E55EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55EB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55EB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E55EB8: 396B6860  addi r11, r11, 0x6860
	ctx.r[11].s64 = ctx.r[11].s64 + 26720;
	// 82E55EBC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55EC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E55EC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55EC8: 419A0008  beq cr6, 0x82e55ed0
	if ctx.cr[6].eq {
	pc = 0x82E55ED0; continue 'dispatch;
	}
	// 82E55ECC: 4BF2A515  bl 0x82d803e0
	ctx.lr = 0x82E55ED0;
	sub_82D803E0(ctx, base);
	// 82E55ED0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55ED4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E55ED8: 419A0008  beq cr6, 0x82e55ee0
	if ctx.cr[6].eq {
	pc = 0x82E55EE0; continue 'dispatch;
	}
	// 82E55EDC: 4BF2A505  bl 0x82d803e0
	ctx.lr = 0x82E55EE0;
	sub_82D803E0(ctx, base);
	// 82E55EE0: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 82E55EE4: 4BFFFE3D  bl 0x82e55d20
	ctx.lr = 0x82E55EE8;
	sub_82E55D20(ctx, base);
	// 82E55EE8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E55EEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55EF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55EF4: 409A0020  bne cr6, 0x82e55f14
	if !ctx.cr[6].eq {
	pc = 0x82E55F14; continue 'dispatch;
	}
	// 82E55EF8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55EFC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E55F00: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E55F04: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E55F08: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E55F0C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E55F10: 4BEFF3B9  bl 0x82d552c8
	ctx.lr = 0x82E55F14;
	sub_82D552C8(ctx, base);
	// 82E55F14: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E55F18: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55F1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55F20: 409A0020  bne cr6, 0x82e55f40
	if !ctx.cr[6].eq {
	pc = 0x82E55F40; continue 'dispatch;
	}
	// 82E55F24: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55F28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E55F2C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E55F30: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E55F34: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E55F38: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E55F3C: 4BEFF38D  bl 0x82d552c8
	ctx.lr = 0x82E55F40;
	sub_82D552C8(ctx, base);
	// 82E55F40: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E55F44: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55F48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55F4C: 409A0020  bne cr6, 0x82e55f6c
	if !ctx.cr[6].eq {
	pc = 0x82E55F6C; continue 'dispatch;
	}
	// 82E55F50: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55F54: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E55F58: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E55F5C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E55F60: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E55F64: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E55F68: 4BEFF361  bl 0x82d552c8
	ctx.lr = 0x82E55F6C;
	sub_82D552C8(ctx, base);
	// 82E55F6C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E55F70: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E55F74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E55F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55F90 size=8
    let mut pc: u32 = 0x82E55F90;
    'dispatch: loop {
        match pc {
            0x82E55F90 => {
    //   block [0x82E55F90..0x82E55F98)
	// 82E55F90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E55F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55F98 size=144
    let mut pc: u32 = 0x82E55F98;
    'dispatch: loop {
        match pc {
            0x82E55F98 => {
    //   block [0x82E55F98..0x82E56028)
	// 82E55F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55FB0: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E55FB4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E55FB8: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E55FBC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55FC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55FC4: 419A0030  beq cr6, 0x82e55ff4
	if ctx.cr[6].eq {
	pc = 0x82E55FF4; continue 'dispatch;
	}
	// 82E55FC8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E55FCC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E55FD0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E55FD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55FD8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E55FDC: 409A0018  bne cr6, 0x82e55ff4
	if !ctx.cr[6].eq {
	pc = 0x82E55FF4; continue 'dispatch;
	}
	// 82E55FE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55FE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E55FE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55FF0: 4E800421  bctrl
	ctx.lr = 0x82E55FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55FF4: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E55FF8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E55FFC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E56000: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E56004: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E56008: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5600C: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82E56010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E56014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E56018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5601C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E56020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E56024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56028 size=188
    let mut pc: u32 = 0x82E56028;
    'dispatch: loop {
        match pc {
            0x82E56028 => {
    //   block [0x82E56028..0x82E560E4)
	// 82E56028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5602C: 4BE533E1  bl 0x82ca940c
	ctx.lr = 0x82E56030;
	sub_82CA93D0(ctx, base);
	// 82E56030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56034: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E56038: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5603C: 394A6950  addi r10, r10, 0x6950
	ctx.r[10].s64 = ctx.r[10].s64 + 26960;
	// 82E56040: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E56044: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E56048: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5604C: 4099005C  ble cr6, 0x82e560a8
	if !ctx.cr[6].gt {
	pc = 0x82E560A8; continue 'dispatch;
	}
	// 82E56050: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E56054: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E56058: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E5605C: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E56060: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56064: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E56068: 419A0030  beq cr6, 0x82e56098
	if ctx.cr[6].eq {
	pc = 0x82E56098; continue 'dispatch;
	}
	// 82E5606C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E56070: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E56074: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E56078: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5607C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E56080: 409A0018  bne cr6, 0x82e56098
	if !ctx.cr[6].eq {
	pc = 0x82E56098; continue 'dispatch;
	}
	// 82E56084: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56088: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5608C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E56094: 4E800421  bctrl
	ctx.lr = 0x82E56098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E56098: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82E5609C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E560A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E560A4: 409AFFB4  bne cr6, 0x82e56058
	if !ctx.cr[6].eq {
	pc = 0x82E56058; continue 'dispatch;
	}
	// 82E560A8: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E560AC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E560B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E560B4: 409A0020  bne cr6, 0x82e560d4
	if !ctx.cr[6].eq {
	pc = 0x82E560D4; continue 'dispatch;
	}
	// 82E560B8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E560BC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E560C0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E560C4: 809E0044  lwz r4, 0x44(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E560C8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E560CC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E560D0: 4BEFF1F9  bl 0x82d552c8
	ctx.lr = 0x82E560D4;
	sub_82D552C8(ctx, base);
	// 82E560D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E560D8: 4BF2CDF1  bl 0x82d82ec8
	ctx.lr = 0x82E560DC;
	sub_82D82EC8(ctx, base);
	// 82E560DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E560E0: 4BE5337C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E560E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E560E8 size=228
    let mut pc: u32 = 0x82E560E8;
    'dispatch: loop {
        match pc {
            0x82E560E8 => {
    //   block [0x82E560E8..0x82E561CC)
	// 82E560E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E560EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E560F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E560F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E560F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E560FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56100: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E56104: 4BF2CFFD  bl 0x82d83100
	ctx.lr = 0x82E56108;
	sub_82D83100(ctx, base);
	// 82E56108: 3BFF0044  addi r31, r31, 0x44
	ctx.r[31].s64 = ctx.r[31].s64 + 68;
	// 82E5610C: 3BDE0044  addi r30, r30, 0x44
	ctx.r[30].s64 = ctx.r[30].s64 + 68;
	// 82E56110: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E56114: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56118: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5611C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E56120: 40980060  bge cr6, 0x82e56180
	if !ctx.cr[6].lt {
	pc = 0x82E56180; continue 'dispatch;
	}
	// 82E56124: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E56128: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5612C: 409A0020  bne cr6, 0x82e5614c
	if !ctx.cr[6].eq {
	pc = 0x82E5614C; continue 'dispatch;
	}
	// 82E56130: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56134: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E56138: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5613C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56140: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E56144: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E56148: 4BEFF181  bl 0x82d552c8
	ctx.lr = 0x82E5614C;
	sub_82D552C8(ctx, base);
	// 82E5614C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56150: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E56154: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56158: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E5615C: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E56160: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E56164: 4BEFF0E5  bl 0x82d55248
	ctx.lr = 0x82E56168;
	sub_82D55248(ctx, base);
	// 82E56168: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5616C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E56170: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56174: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E56178: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E5617C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E56180: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56184: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56188: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5618C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E56190: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56194: 40990020  ble cr6, 0x82e561b4
	if !ctx.cr[6].gt {
	pc = 0x82E561B4; continue 'dispatch;
	}
	// 82E56198: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E5619C: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E561A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E561A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E561A8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E561AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E561B0: 409AFFEC  bne cr6, 0x82e5619c
	if !ctx.cr[6].eq {
	pc = 0x82E5619C; continue 'dispatch;
	}
	// 82E561B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E561B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E561BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E561C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E561C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E561C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E561D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E561D0 size=144
    let mut pc: u32 = 0x82E561D0;
    'dispatch: loop {
        match pc {
            0x82E561D0 => {
    //   block [0x82E561D0..0x82E56260)
	// 82E561D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E561D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E561D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E561DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E561E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E561E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E561E8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E561EC: 419A005C  beq cr6, 0x82e56248
	if ctx.cr[6].eq {
	pc = 0x82E56248; continue 'dispatch;
	}
	// 82E561F0: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E561F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E561F8: 419A0010  beq cr6, 0x82e56208
	if ctx.cr[6].eq {
	pc = 0x82E56208; continue 'dispatch;
	}
	// 82E561FC: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E56200: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E56204: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E56208: 3BE30044  addi r31, r3, 0x44
	ctx.r[31].s64 = ctx.r[3].s64 + 68;
	// 82E5620C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E56210: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56214: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E56218: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5621C: 409A0010  bne cr6, 0x82e5622c
	if !ctx.cr[6].eq {
	pc = 0x82E5622C; continue 'dispatch;
	}
	// 82E56220: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E56224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56228: 4BF00D71  bl 0x82d56f98
	ctx.lr = 0x82E5622C;
	sub_82D56F98(ctx, base);
	// 82E5622C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56230: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56234: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E56238: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E5623C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56240: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E56244: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E56248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5624C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E56250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56254: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E56258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5625C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E56260 size=116
    let mut pc: u32 = 0x82E56260;
    'dispatch: loop {
        match pc {
            0x82E56260 => {
    //   block [0x82E56260..0x82E562D4)
	// 82E56260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5626C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56270: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E56274: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E56278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5627C: 4BF34B0D  bl 0x82d8ad88
	ctx.lr = 0x82E56280;
	sub_82D8AD88(ctx, base);
	// 82E56280: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E56284: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82E56288: 392A6A04  addi r9, r10, 0x6a04
	ctx.r[9].s64 = ctx.r[10].s64 + 27140;
	// 82E5628C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E56290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56294: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E56298: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E5629C: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82E562A0: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82E562A4: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E562D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E562D8 size=112
    let mut pc: u32 = 0x82E562D8;
    'dispatch: loop {
        match pc {
            0x82E562D8 => {
    //   block [0x82E562D8..0x82E56348)
	// 82E562D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E562DC: 4BE53131  bl 0x82ca940c
	ctx.lr = 0x82E562E0;
	sub_82CA93D0(ctx, base);
	// 82E562E0: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E562E4: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E562E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E562EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E562F0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E562F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E562F8: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E562FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56300: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E56304: 4BF34A85  bl 0x82d8ad88
	ctx.lr = 0x82E56308;
	sub_82D8AD88(ctx, base);
	// 82E56308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5630C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82E56310: 396B6A04  addi r11, r11, 0x6a04
	ctx.r[11].s64 = ctx.r[11].s64 + 27140;
	// 82E56314: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82E56318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5631C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E56348 size=152
    let mut pc: u32 = 0x82E56348;
    'dispatch: loop {
        match pc {
            0x82E56348 => {
    //   block [0x82E56348..0x82E563E0)
	// 82E56348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E56354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5635C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E56360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56364: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56368: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E5636C: 419A000C  beq cr6, 0x82e56378
	if ctx.cr[6].eq {
	pc = 0x82E56378; continue 'dispatch;
	}
	// 82E56370: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E56374: 48000054  b 0x82e563c8
	pc = 0x82E563C8; continue 'dispatch;
	// 82E56378: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5637C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E56380: 409AFFF0  bne cr6, 0x82e56370
	if !ctx.cr[6].eq {
	pc = 0x82E56370; continue 'dispatch;
	}
	// 82E56384: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56388: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5638C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E56390: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82E56394: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E56398: 4BEFEEB1  bl 0x82d55248
	ctx.lr = 0x82E5639C;
	sub_82D55248(ctx, base);
	// 82E5639C: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 82E563A0: 38DF0030  addi r6, r31, 0x30
	ctx.r[6].s64 = ctx.r[31].s64 + 48;
	// 82E563A4: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82E563A8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E563AC: C05F0044  lfs f2, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E563B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E563B4: C03F0040  lfs f1, 0x40(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E563B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E563BC: 4BFFFF1D  bl 0x82e562d8
	ctx.lr = 0x82E563C0;
	sub_82E562D8(ctx, base);
	// 82E563C0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E563C4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E563C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E563CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E563D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E563D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E563D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E563DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E563E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E563E0 size=816
    let mut pc: u32 = 0x82E563E0;
    'dispatch: loop {
        match pc {
            0x82E563E0 => {
    //   block [0x82E563E0..0x82E56710)
	// 82E563E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E563E4: 4BE53029  bl 0x82ca940c
	ctx.lr = 0x82E563E8;
	sub_82CA93D0(ctx, base);
	// 82E563E8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E563EC: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E56710 size=108
    let mut pc: u32 = 0x82E56710;
    'dispatch: loop {
        match pc {
            0x82E56710 => {
    //   block [0x82E56710..0x82E5677C)
	// 82E56710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5671C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56724: 4BF30E3D  bl 0x82d87560
	ctx.lr = 0x82E56728;
	sub_82D87560(ctx, base);
	// 82E56728: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E5672C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E56730: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E56734: 396B6BE4  addi r11, r11, 0x6be4
	ctx.r[11].s64 = ctx.r[11].s64 + 27620;
	// 82E56738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5673C: C00A0C14  lfs f0, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E56740: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82E56744: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E56748: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5674C: C1AA9404  lfs f13, -0x6bfc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27644 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E56750: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E56754: D1BF0054  stfs f13, 0x54(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E56758: C18A0BF8  lfs f12, 0xbf8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3064 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E5675C: D19F0058  stfs f12, 0x58(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E56760: 993F005C  stb r9, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[9].u8 ) };
	// 82E56764: 993F005D  stb r9, 0x5d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(93 as u32), ctx.r[9].u8 ) };
	// 82E56768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5676C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E56770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E56778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56780 size=136
    let mut pc: u32 = 0x82E56780;
    'dispatch: loop {
        match pc {
            0x82E56780 => {
    //   block [0x82E56780..0x82E56808)
	// 82E56780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56784: 4BE52C85  bl 0x82ca9408
	ctx.lr = 0x82E56788;
	sub_82CA93D0(ctx, base);
	// 82E56788: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5678C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E56794: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E56798: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E5679C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E567A0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E567A4: 839F001C  lwz r28, 0x1c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E567A8: 388B00E0  addi r4, r11, 0xe0
	ctx.r[4].s64 = ctx.r[11].s64 + 224;
	// 82E567AC: 4BEFFC45  bl 0x82d563f0
	ctx.lr = 0x82E567B0;
	sub_82D563F0(ctx, base);
	// 82E567B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E567B4: 389C00E0  addi r4, r28, 0xe0
	ctx.r[4].s64 = ctx.r[28].s64 + 224;
	// 82E567B8: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E567BC: 4BEFFC35  bl 0x82d563f0
	ctx.lr = 0x82E567C0;
	sub_82D563F0(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56808 size=164
    let mut pc: u32 = 0x82E56808;
    'dispatch: loop {
        match pc {
            0x82E56808 => {
    //   block [0x82E56808..0x82E568AC)
	// 82E56808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5680C: 4BE52C01  bl 0x82ca940c
	ctx.lr = 0x82E56810;
	sub_82CA93D0(ctx, base);
	// 82E56810: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E568B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E568B0 size=200
    let mut pc: u32 = 0x82E568B0;
    'dispatch: loop {
        match pc {
            0x82E568B0 => {
    //   block [0x82E568B0..0x82E56978)
	// 82E568B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E568B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E568B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E568BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E568C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E568C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E568C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E568CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E568D0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E568D4: 419A000C  beq cr6, 0x82e568e0
	if ctx.cr[6].eq {
	pc = 0x82E568E0; continue 'dispatch;
	}
	// 82E568D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E568DC: 48000084  b 0x82e56960
	pc = 0x82E56960; continue 'dispatch;
	// 82E568E0: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E568E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E568E8: 409AFFF0  bne cr6, 0x82e568d8
	if !ctx.cr[6].eq {
	pc = 0x82E568D8; continue 'dispatch;
	}
	// 82E568EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E568F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E568F4: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E568F8: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E568FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E56900: 4BEFE949  bl 0x82d55248
	ctx.lr = 0x82E56904;
	sub_82D55248(ctx, base);
	// 82E56904: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E56908: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5690C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56910: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E56914: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56918: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5691C: 4BFFFDF5  bl 0x82e56710
	ctx.lr = 0x82E56920;
	sub_82E56710(ctx, base);
	// 82E56920: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E56924: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E56978 size=564
    let mut pc: u32 = 0x82E56978;
    'dispatch: loop {
        match pc {
            0x82E56978 => {
    //   block [0x82E56978..0x82E56BAC)
	// 82E56978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5697C: 4BE52A81  bl 0x82ca93fc
	ctx.lr = 0x82E56980;
	sub_82CA93D0(ctx, base);
	// 82E56980: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E56984: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E56BB0 size=104
    let mut pc: u32 = 0x82E56BB0;
    'dispatch: loop {
        match pc {
            0x82E56BB0 => {
    //   block [0x82E56BB0..0x82E56C18)
	// 82E56BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56BBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56BC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56BC4: 4BF3099D  bl 0x82d87560
	ctx.lr = 0x82E56BC8;
	sub_82D87560(ctx, base);
	// 82E56BC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E56BCC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E56BD0: 396B6FE4  addi r11, r11, 0x6fe4
	ctx.r[11].s64 = ctx.r[11].s64 + 28644;
	// 82E56BD4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E56BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56BDC: C00A0BF8  lfs f0, 0xbf8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E56BE0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E56BE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E56BE8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E56BEC: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E56BF0: 396B4C80  addi r11, r11, 0x4c80
	ctx.r[11].s64 = ctx.r[11].s64 + 19584;
	// 82E56BF4: C1AA0BE8  lfs f13, 0xbe8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3048 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E56BF8: D1BF0034  stfs f13, 0x34(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56C18 size=164
    let mut pc: u32 = 0x82E56C18;
    'dispatch: loop {
        match pc {
            0x82E56C18 => {
    //   block [0x82E56C18..0x82E56CBC)
	// 82E56C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56C20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E56C24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56C28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56C2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E56C30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56C34: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56C38: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E56C3C: 419A000C  beq cr6, 0x82e56c48
	if ctx.cr[6].eq {
	pc = 0x82E56C48; continue 'dispatch;
	}
	// 82E56C40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E56C44: 48000060  b 0x82e56ca4
	pc = 0x82E56CA4; continue 'dispatch;
	// 82E56C48: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56C4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E56C50: 409AFFF0  bne cr6, 0x82e56c40
	if !ctx.cr[6].eq {
	pc = 0x82E56C40; continue 'dispatch;
	}
	// 82E56C54: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56C58: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E56C5C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E56C60: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82E56C64: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E56C68: 4BEFE5E1  bl 0x82d55248
	ctx.lr = 0x82E56C6C;
	sub_82D55248(ctx, base);
	// 82E56C6C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82E56C70: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E56C74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56C78: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E56C7C: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56C80: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56C84: 4BFFFF2D  bl 0x82e56bb0
	ctx.lr = 0x82E56C88;
	sub_82E56BB0(ctx, base);
	// 82E56C88: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E56CC0 size=616
    let mut pc: u32 = 0x82E56CC0;
    'dispatch: loop {
        match pc {
            0x82E56CC0 => {
    //   block [0x82E56CC0..0x82E56F28)
	// 82E56CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56CC4: 4BE52745  bl 0x82ca9408
	ctx.lr = 0x82E56CC8;
	sub_82CA93D0(ctx, base);
	// 82E56CC8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E56CCC: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56F28 size=200
    let mut pc: u32 = 0x82E56F28;
    'dispatch: loop {
        match pc {
            0x82E56F28 => {
    //   block [0x82E56F28..0x82E56FF0)
	// 82E56F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56F30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56F34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56F3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E56F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E56F44: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E56F48: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E56F4C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E56F50: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E56F54: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E56F58: 396B709C  addi r11, r11, 0x709c
	ctx.r[11].s64 = ctx.r[11].s64 + 28828;
	// 82E56F5C: 394A7090  addi r10, r10, 0x7090
	ctx.r[10].s64 = ctx.r[10].s64 + 28816;
	// 82E56F60: 3929707C  addi r9, r9, 0x707c
	ctx.r[9].s64 = ctx.r[9].s64 + 28796;
	// 82E56F64: 39087070  addi r8, r8, 0x7070
	ctx.r[8].s64 = ctx.r[8].s64 + 28784;
	// 82E56F68: 38E77064  addi r7, r7, 0x7064
	ctx.r[7].s64 = ctx.r[7].s64 + 28772;
	// 82E56F6C: 38C67054  addi r6, r6, 0x7054
	ctx.r[6].s64 = ctx.r[6].s64 + 28756;
	// 82E56F70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E56F74: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E56F78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E56F7C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E56F80: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E56F84: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82E56F88: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82E56F8C: 419A003C  beq cr6, 0x82e56fc8
	if ctx.cr[6].eq {
	pc = 0x82E56FC8; continue 'dispatch;
	}
	// 82E56F90: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E56F98: 419A0030  beq cr6, 0x82e56fc8
	if ctx.cr[6].eq {
	pc = 0x82E56FC8; continue 'dispatch;
	}
	// 82E56F9C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E56FA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E56FA4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E56FA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E56FAC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E56FB0: 409A0018  bne cr6, 0x82e56fc8
	if !ctx.cr[6].eq {
	pc = 0x82E56FC8; continue 'dispatch;
	}
	// 82E56FB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56FB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E56FBC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E56FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E56FC4: 4E800421  bctrl
	ctx.lr = 0x82E56FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E56FC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E56FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56FD0: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82E56FD4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E56FD8: 4B808501  bl 0x8265f4d8
	ctx.lr = 0x82E56FDC;
	sub_8265F4D8(ctx, base);
	// 82E56FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E56FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E56FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E56FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56FF0 size=360
    let mut pc: u32 = 0x82E56FF0;
    'dispatch: loop {
        match pc {
            0x82E56FF0 => {
    //   block [0x82E56FF0..0x82E57158)
	// 82E56FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56FF4: 4BE52409  bl 0x82ca93fc
	ctx.lr = 0x82E56FF8;
	sub_82CA93D0(ctx, base);
	// 82E56FF8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56FFC: 89650018  lbz r11, 0x18(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E57000: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E57004: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E57008: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E5700C: 409A0014  bne cr6, 0x82e57020
	if !ctx.cr[6].eq {
	pc = 0x82E57020; continue 'dispatch;
	}
	// 82E57010: 89650010  lbz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57014: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E57018: 7F8B2A14  add r28, r11, r5
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82E5701C: 48000008  b 0x82e57024
	pc = 0x82E57024; continue 'dispatch;
	// 82E57020: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E57024: 89660018  lbz r11, 0x18(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E57028: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E5702C: 409A0014  bne cr6, 0x82e57040
	if !ctx.cr[6].eq {
	pc = 0x82E57040; continue 'dispatch;
	}
	// 82E57030: 89660010  lbz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57034: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E57038: 7FEB3214  add r31, r11, r6
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82E5703C: 48000008  b 0x82e57044
	pc = 0x82E57044; continue 'dispatch;
	// 82E57040: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E57044: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E57048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5704C: 419A003C  beq cr6, 0x82e57088
	if ctx.cr[6].eq {
	pc = 0x82E57088; continue 'dispatch;
	}
	// 82E57050: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E57058: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82E5705C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57060: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57064: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57068: 4E800421  bctrl
	ctx.lr = 0x82E5706C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5706C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57074: 409A0014  bne cr6, 0x82e57088
	if !ctx.cr[6].eq {
	pc = 0x82E57088; continue 'dispatch;
	}
	// 82E57078: 9B3A0000  stb r25, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82E5707C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E57080: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E57084: 4BE523C8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82E57088: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E5708C: 419A00B8  beq cr6, 0x82e57144
	if ctx.cr[6].eq {
	pc = 0x82E57144; continue 'dispatch;
	}
	// 82E57090: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E57094: 419A00B0  beq cr6, 0x82e57144
	if ctx.cr[6].eq {
	pc = 0x82E57144; continue 'dispatch;
	}
	// 82E57098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5709C: 4BF2E675  bl 0x82d85710
	ctx.lr = 0x82E570A0;
	sub_82D85710(ctx, base);
	// 82E570A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E570A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E570A8: 4BF2E669  bl 0x82d85710
	ctx.lr = 0x82E570AC;
	sub_82D85710(ctx, base);
	// 82E570AC: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82E570B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E570B4: 41990008  bgt cr6, 0x82e570bc
	if ctx.cr[6].gt {
	pc = 0x82E570BC; continue 'dispatch;
	}
	// 82E570B8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82E570BC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E570C0: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82E570C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E570C8: 409A000C  bne cr6, 0x82e570d4
	if !ctx.cr[6].eq {
	pc = 0x82E570D4; continue 'dispatch;
	}
	// 82E570CC: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82E570D0: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82E570D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E570D8: 4BF2E639  bl 0x82d85710
	ctx.lr = 0x82E570DC;
	sub_82D85710(ctx, base);
	// 82E570DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E570E0: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E570E4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E570E8: 4099005C  ble cr6, 0x82e57144
	if !ctx.cr[6].gt {
	pc = 0x82E57144; continue 'dispatch;
	}
	// 82E570EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E570F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E570F4: 4BF2E665  bl 0x82d85758
	ctx.lr = 0x82E570F8;
	sub_82D85758(ctx, base);
	// 82E570F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E570FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E57100: 419A0038  beq cr6, 0x82e57138
	if ctx.cr[6].eq {
	pc = 0x82E57138; continue 'dispatch;
	}
	// 82E57104: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E57108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5710C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57114: 4E800421  bctrl
	ctx.lr = 0x82E57118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E57118: 2F03000B  cmpwi cr6, r3, 0xb
	ctx.cr[6].compare_i32(ctx.r[3].s32, 11, &mut ctx.xer);
	// 82E5711C: 419A001C  beq cr6, 0x82e57138
	if ctx.cr[6].eq {
	pc = 0x82E57138; continue 'dispatch;
	}
	// 82E57120: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E57124: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E57128: 419AFF50  beq cr6, 0x82e57078
	if ctx.cr[6].eq {
	pc = 0x82E57078; continue 'dispatch;
	}
	// 82E5712C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E57130: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E57134: 419AFF44  beq cr6, 0x82e57078
	if ctx.cr[6].eq {
	pc = 0x82E57078; continue 'dispatch;
	}
	// 82E57138: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5713C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E57140: 4198FFAC  blt cr6, 0x82e570ec
	if ctx.cr[6].lt {
	pc = 0x82E570EC; continue 'dispatch;
	}
	// 82E57144: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E57148: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E5714C: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E57150: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E57154: 4BE522F8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57158 size=108
    let mut pc: u32 = 0x82E57158;
    'dispatch: loop {
        match pc {
            0x82E57158 => {
    //   block [0x82E57158..0x82E571C4)
	// 82E57158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5715C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E57160: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E57164: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57168: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E5716C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E57170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57174: 419A0030  beq cr6, 0x82e571a4
	if ctx.cr[6].eq {
	pc = 0x82E571A4; continue 'dispatch;
	}
	// 82E57178: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5717C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E57180: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82E57184: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57188: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5718C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57190: 4E800421  bctrl
	ctx.lr = 0x82E57194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E57194: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57198: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5719C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E571A0: 419A0008  beq cr6, 0x82e571a8
	if ctx.cr[6].eq {
	pc = 0x82E571A8; continue 'dispatch;
	}
	// 82E571A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E571A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E571AC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E571B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E571B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E571B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E571BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E571C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E571C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E571C8 size=116
    let mut pc: u32 = 0x82E571C8;
    'dispatch: loop {
        match pc {
            0x82E571C8 => {
    //   block [0x82E571C8..0x82E5723C)
	// 82E571C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E571CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E571D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E571D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E571D8: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E571DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E571E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E571E4: 419A0038  beq cr6, 0x82e5721c
	if ctx.cr[6].eq {
	pc = 0x82E5721C; continue 'dispatch;
	}
	// 82E571E8: 808100D4  lwz r4, 0xd4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82E571EC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E571F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E571F4: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82E571F8: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82E571FC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57200: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57208: 4E800421  bctrl
	ctx.lr = 0x82E5720C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5720C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57214: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E57218: 419A0008  beq cr6, 0x82e57220
	if ctx.cr[6].eq {
	pc = 0x82E57220; continue 'dispatch;
	}
	// 82E5721C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E57220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57224: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E57228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5722C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E57230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E57234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E57238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57240 size=108
    let mut pc: u32 = 0x82E57240;
    'dispatch: loop {
        match pc {
            0x82E57240 => {
    //   block [0x82E57240..0x82E572AC)
	// 82E57240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E57248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5724C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57250: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E57254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E57258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5725C: 419A0030  beq cr6, 0x82e5728c
	if ctx.cr[6].eq {
	pc = 0x82E5728C; continue 'dispatch;
	}
	// 82E57260: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57264: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E57268: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82E5726C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57270: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57278: 4E800421  bctrl
	ctx.lr = 0x82E5727C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5727C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57284: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E57288: 419A0008  beq cr6, 0x82e57290
	if ctx.cr[6].eq {
	pc = 0x82E57290; continue 'dispatch;
	}
	// 82E5728C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E57290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57294: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E57298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5729C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E572A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E572A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E572A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E572B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E572B0 size=108
    let mut pc: u32 = 0x82E572B0;
    'dispatch: loop {
        match pc {
            0x82E572B0 => {
    //   block [0x82E572B0..0x82E5731C)
	// 82E572B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E572B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E572B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E572BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E572C0: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E572C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E572C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E572CC: 419A0030  beq cr6, 0x82e572fc
	if ctx.cr[6].eq {
	pc = 0x82E572FC; continue 'dispatch;
	}
	// 82E572D0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E572D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E572D8: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 82E572DC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E572E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E572E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E572E8: 4E800421  bctrl
	ctx.lr = 0x82E572EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E572EC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E572F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E572F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E572F8: 419A0008  beq cr6, 0x82e57300
	if ctx.cr[6].eq {
	pc = 0x82E57300; continue 'dispatch;
	}
	// 82E572FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E57300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57304: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E57308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5730C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E57310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E57314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E57318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57320 size=100
    let mut pc: u32 = 0x82E57320;
    'dispatch: loop {
        match pc {
            0x82E57320 => {
    //   block [0x82E57320..0x82E57384)
	// 82E57320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E57328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5732C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57330: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E57334: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E57338: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5733C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57344: 4E800421  bctrl
	ctx.lr = 0x82E57348;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E57348: 2F03000B  cmpwi cr6, r3, 0xb
	ctx.cr[6].compare_i32(ctx.r[3].s32, 11, &mut ctx.xer);
	// 82E5734C: 419A0024  beq cr6, 0x82e57370
	if ctx.cr[6].eq {
	pc = 0x82E57370; continue 'dispatch;
	}
	// 82E57350: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E57354: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E57358: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82E5735C: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82E57360: 4BFAEEE1  bl 0x82e06240
	ctx.lr = 0x82E57364;
	sub_82E06240(ctx, base);
	// 82E57364: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E57368: 419A0008  beq cr6, 0x82e57370
	if ctx.cr[6].eq {
	pc = 0x82E57370; continue 'dispatch;
	}
	// 82E5736C: 4BF44825  bl 0x82d9bb90
	ctx.lr = 0x82E57370;
	sub_82D9BB90(ctx, base);
	// 82E57370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E57374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E57378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5737C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E57380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E57388 size=4
    let mut pc: u32 = 0x82E57388;
    'dispatch: loop {
        match pc {
            0x82E57388 => {
    //   block [0x82E57388..0x82E5738C)
	// 82E57388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E57390 size=196
    let mut pc: u32 = 0x82E57390;
    'dispatch: loop {
        match pc {
            0x82E57390 => {
    //   block [0x82E57390..0x82E57454)
	// 82E57390: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E57394: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E57398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5739C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E573A0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E573A4: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E573A8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E573AC: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E573B0: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E573B4: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E573B8: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E573BC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E573C0: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E573C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E573C8: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E573CC: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E573D0: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E573D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E573D8: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E573DC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E573E0: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E573E4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E573E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E573EC: 38E77044  addi r7, r7, 0x7044
	ctx.r[7].s64 = ctx.r[7].s64 + 28740;
	// 82E573F0: 396B709C  addi r11, r11, 0x709c
	ctx.r[11].s64 = ctx.r[11].s64 + 28828;
	// 82E573F4: 38C67090  addi r6, r6, 0x7090
	ctx.r[6].s64 = ctx.r[6].s64 + 28816;
	// 82E573F8: 394A707C  addi r10, r10, 0x707c
	ctx.r[10].s64 = ctx.r[10].s64 + 28796;
	// 82E573FC: 39297070  addi r9, r9, 0x7070
	ctx.r[9].s64 = ctx.r[9].s64 + 28784;
	// 82E57400: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82E57404: 38A57064  addi r5, r5, 0x7064
	ctx.r[5].s64 = ctx.r[5].s64 + 28772;
	// 82E57408: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E5740C: 39087054  addi r8, r8, 0x7054
	ctx.r[8].s64 = ctx.r[8].s64 + 28756;
	// 82E57410: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57414: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E57418: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E5741C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E57420: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E57424: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82E57428: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E5742C: 90830034  stw r4, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 82E57430: 419A001C  beq cr6, 0x82e5744c
	if ctx.cr[6].eq {
	pc = 0x82E5744C; continue 'dispatch;
	}
	// 82E57434: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5743C: 419A0010  beq cr6, 0x82e5744c
	if ctx.cr[6].eq {
	pc = 0x82E5744C; continue 'dispatch;
	}
	// 82E57440: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E57444: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E57448: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E5744C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82E57450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E57458 size=116
    let mut pc: u32 = 0x82E57458;
    'dispatch: loop {
        match pc {
            0x82E57458 => {
    //   block [0x82E57458..0x82E574CC)
	// 82E57458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E57460: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E57464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5746C: 4BF300F5  bl 0x82d87560
	ctx.lr = 0x82E57470;
	sub_82D87560(ctx, base);
	// 82E57470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E57474: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82E57478: 392A7134  addi r9, r10, 0x7134
	ctx.r[9].s64 = ctx.r[10].s64 + 28980;
	// 82E5747C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E57480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57484: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E57488: C00A0BF8  lfs f0, 0xbf8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E5748C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E57490: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82E57494: C1AA0BE8  lfs f13, 0xbe8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3048 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E57498: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82E5749C: D1BF0044  stfs f13, 0x44(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E574D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E574D0 size=188
    let mut pc: u32 = 0x82E574D0;
    'dispatch: loop {
        match pc {
            0x82E574D0 => {
    //   block [0x82E574D0..0x82E5758C)
	// 82E574D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E574D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E574D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E574DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E574E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E574E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E574E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E574EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E574F0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E574F4: 419A000C  beq cr6, 0x82e57500
	if ctx.cr[6].eq {
	pc = 0x82E57500; continue 'dispatch;
	}
	// 82E574F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E574FC: 48000078  b 0x82e57574
	pc = 0x82E57574; continue 'dispatch;
	// 82E57500: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57504: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E57508: 409AFFF0  bne cr6, 0x82e574f8
	if !ctx.cr[6].eq {
	pc = 0x82E574F8; continue 'dispatch;
	}
	// 82E5750C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57510: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E57514: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E57518: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E5751C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57520: 4BEFDD29  bl 0x82d55248
	ctx.lr = 0x82E57524;
	sub_82D55248(ctx, base);
	// 82E57524: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E57528: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5752C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57530: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57534: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57538: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5753C: 4BFFFF1D  bl 0x82e57458
	ctx.lr = 0x82E57540;
	sub_82E57458(ctx, base);
	// 82E57540: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E57544: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E57548: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E57590 size=404
    let mut pc: u32 = 0x82E57590;
    'dispatch: loop {
        match pc {
            0x82E57590 => {
    //   block [0x82E57590..0x82E57724)
	// 82E57590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57594: 4BE51E6D  bl 0x82ca9400
	ctx.lr = 0x82E57598;
	sub_82CA93D0(ctx, base);
	// 82E57598: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E5759C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E575A0: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E575A4: 3B600008  li r27, 8
	ctx.r[27].s64 = 8;
	// 82E575A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E575AC: 7D5BD02E  lwzx r10, r27, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E575B0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E575B4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E575B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E575BC: 40980020  bge cr6, 0x82e575dc
	if !ctx.cr[6].lt {
	pc = 0x82E575DC; continue 'dispatch;
	}
	// 82E575C0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E575C4: 39297390  addi r9, r9, 0x7390
	ctx.r[9].s64 = ctx.r[9].s64 + 29584;
	// 82E575C8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E575CC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E575D0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E575D4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E575D8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E575DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E575E0: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E575E4: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E575E8: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82E575EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E575F0: 83BF001C  lwz r29, 0x1c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E575F4: 389E00E0  addi r4, r30, 0xe0
	ctx.r[4].s64 = ctx.r[30].s64 + 224;
	// 82E575F8: C00B738C  lfs f0, 0x738c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E575FC: EFED0032  fmuls f31, f13, f0
	ctx.f[31].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E57600: 4BEFEDA9  bl 0x82d563a8
	ctx.lr = 0x82E57604;
	sub_82D563A8(ctx, base);
	// 82E57604: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82E57608: 389D00E0  addi r4, r29, 0xe0
	ctx.r[4].s64 = ctx.r[29].s64 + 224;
	// 82E5760C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E57610: 4BEFED99  bl 0x82d563a8
	ctx.lr = 0x82E57614;
	sub_82D563A8(ctx, base);
	// 82E57614: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E57618: 396001A0  li r11, 0x1a0
	ctx.r[11].s64 = 416;
	// 82E5761C: C01F0040  lfs f0, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E57620: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E57624: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E57628: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82E5762C: C01F0044  lfs f0, 0x44(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E57630: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E57634: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57728 size=72
    let mut pc: u32 = 0x82E57728;
    'dispatch: loop {
        match pc {
            0x82E57728 => {
    //   block [0x82E57728..0x82E57770)
	// 82E57728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E57730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E57734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57738: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5773C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E57740: 396B73A0  addi r11, r11, 0x73a0
	ctx.r[11].s64 = ctx.r[11].s64 + 29600;
	// 82E57744: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E57748: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5774C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57750: 419A000C  beq cr6, 0x82e5775c
	if ctx.cr[6].eq {
	pc = 0x82E5775C; continue 'dispatch;
	}
	// 82E57754: 4B9EE05D  bl 0x828457b0
	ctx.lr = 0x82E57758;
	sub_828457B0(ctx, base);
	// 82E57758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5775C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E57760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E57764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E57768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5776C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E57770 size=24
    let mut pc: u32 = 0x82E57770;
    'dispatch: loop {
        match pc {
            0x82E57770 => {
    //   block [0x82E57770..0x82E57788)
	// 82E57770: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E57774: 396BA088  addi r11, r11, -0x5f78
	ctx.r[11].s64 = ctx.r[11].s64 + -24440;
	// 82E57778: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E5777C: 409A000C  bne cr6, 0x82e57788
	if !ctx.cr[6].eq {
		sub_82E57788(ctx, base);
		return;
	}
	// 82E57780: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E57784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E57788 size=24
    let mut pc: u32 = 0x82E57788;
    'dispatch: loop {
        match pc {
            0x82E57788 => {
    //   block [0x82E57788..0x82E577A0)
	// 82E57788: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E5778C: 396B9A70  addi r11, r11, -0x6590
	ctx.r[11].s64 = ctx.r[11].s64 + -26000;
	// 82E57790: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E57794: 409A000C  bne cr6, 0x82e577a0
	if !ctx.cr[6].eq {
		sub_82E577A0(ctx, base);
		return;
	}
	// 82E57798: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82E5779C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E577A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E577A0 size=24
    let mut pc: u32 = 0x82E577A0;
    'dispatch: loop {
        match pc {
            0x82E577A0 => {
    //   block [0x82E577A0..0x82E577B8)
	// 82E577A0: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E577A4: 396BB6C8  addi r11, r11, -0x4938
	ctx.r[11].s64 = ctx.r[11].s64 + -18744;
	// 82E577A8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E577AC: 409A000C  bne cr6, 0x82e577b8
	if !ctx.cr[6].eq {
		sub_82E577B8(ctx, base);
		return;
	}
	// 82E577B0: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82E577B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E577B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E577B8 size=24
    let mut pc: u32 = 0x82E577B8;
    'dispatch: loop {
        match pc {
            0x82E577B8 => {
    //   block [0x82E577B8..0x82E577D0)
	// 82E577B8: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E577BC: 396BA8B8  addi r11, r11, -0x5748
	ctx.r[11].s64 = ctx.r[11].s64 + -22344;
	// 82E577C0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E577C4: 409A000C  bne cr6, 0x82e577d0
	if !ctx.cr[6].eq {
		sub_82E577D0(ctx, base);
		return;
	}
	// 82E577C8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82E577CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E577D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E577D0 size=24
    let mut pc: u32 = 0x82E577D0;
    'dispatch: loop {
        match pc {
            0x82E577D0 => {
    //   block [0x82E577D0..0x82E577E8)
	// 82E577D0: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E577D4: 396B9050  addi r11, r11, -0x6fb0
	ctx.r[11].s64 = ctx.r[11].s64 + -28592;
	// 82E577D8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E577DC: 409A000C  bne cr6, 0x82e577e8
	if !ctx.cr[6].eq {
		sub_82E577E8(ctx, base);
		return;
	}
	// 82E577E0: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82E577E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E577E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E577E8 size=24
    let mut pc: u32 = 0x82E577E8;
    'dispatch: loop {
        match pc {
            0x82E577E8 => {
    //   block [0x82E577E8..0x82E57800)
	// 82E577E8: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E577EC: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 82E577F0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E577F4: 409A000C  bne cr6, 0x82e57800
	if !ctx.cr[6].eq {
		sub_82E57800(ctx, base);
		return;
	}
	// 82E577F8: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82E577FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E57800 size=24
    let mut pc: u32 = 0x82E57800;
    'dispatch: loop {
        match pc {
            0x82E57800 => {
    //   block [0x82E57800..0x82E57818)
	// 82E57800: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E57804: 396B8770  addi r11, r11, -0x7890
	ctx.r[11].s64 = ctx.r[11].s64 + -30864;
	// 82E57808: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E5780C: 409A000C  bne cr6, 0x82e57818
	if !ctx.cr[6].eq {
		sub_82E57818(ctx, base);
		return;
	}
	// 82E57810: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82E57814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E57818 size=24
    let mut pc: u32 = 0x82E57818;
    'dispatch: loop {
        match pc {
            0x82E57818 => {
    //   block [0x82E57818..0x82E57830)
	// 82E57818: 3D6082E1  lis r11, -0x7d1f
	ctx.r[11].s64 = -2099183616;
	// 82E5781C: 396BC688  addi r11, r11, -0x3978
	ctx.r[11].s64 = ctx.r[11].s64 + -14712;
	// 82E57820: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82E57824: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E57828: 5563F738  rlwinm r3, r11, 0x1e, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82E5782C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E57830 size=416
    let mut pc: u32 = 0x82E57830;
    'dispatch: loop {
        match pc {
            0x82E57830 => {
    //   block [0x82E57830..0x82E57894)
	// 82E57830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57834: 4BE51BD9  bl 0x82ca940c
	ctx.lr = 0x82E57838;
	sub_82CA93D0(ctx, base);
	// 82E57838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5783C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E57840: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82E57844: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E57848: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E5784C: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82E57850: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57854: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57858: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E5785C: 41990168  bgt cr6, 0x82e579c4
	if ctx.cr[6].gt {
	pc = 0x82E579C4; continue 'dispatch;
	}
	// 82E57860: 3D8082E5  lis r12, -0x7d1b
	ctx.r[12].s64 = -2098921472;
	// 82E57864: 398C7878  addi r12, r12, 0x7878
	ctx.r[12].s64 = ctx.r[12].s64 + 30840;
	// 82E57868: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E5786C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E57870: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E57874: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82E579BC; continue 'dispatch;
		},
		1 => {
	pc = 0x82E579BC; continue 'dispatch;
		},
		2 => {
	pc = 0x82E578B8; continue 'dispatch;
		},
		3 => {
	pc = 0x82E578B8; continue 'dispatch;
		},
		4 => {
	pc = 0x82E578A0; continue 'dispatch;
		},
		5 => {
	pc = 0x82E578A0; continue 'dispatch;
		},
		6 => {
	pc = 0x82E57894; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E57878: 82E579BC  lwz r23, 0x79bc(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31164 as u32) ) } as u64;
	// 82E5787C: 82E579BC  lwz r23, 0x79bc(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31164 as u32) ) } as u64;
	// 82E57880: 82E578B8  lwz r23, 0x78b8(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(30904 as u32) ) } as u64;
	// 82E57884: 82E578B8  lwz r23, 0x78b8(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(30904 as u32) ) } as u64;
	// 82E57888: 82E578A0  lwz r23, 0x78a0(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(30880 as u32) ) } as u64;
	// 82E5788C: 82E578A0  lwz r23, 0x78a0(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(30880 as u32) ) } as u64;
	// 82E57890: 82E57894  lwz r23, 0x7894(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(30868 as u32) ) } as u64;
            }
            0x82E57894 => {
    //   block [0x82E57894..0x82E578A0)
	// 82E57894: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E57898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5789C: 4BE51BC0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E578A0 => {
    //   block [0x82E578A0..0x82E578B8)
	// 82E578A0: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82E578A4: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82E578A8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82E578AC: 556B06F6  rlwinm r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E578B0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82E578B4: 48000018  b 0x82e578cc
	pc = 0x82E578CC; continue 'dispatch;
            }
            0x82E578B8 => {
    //   block [0x82E578B8..0x82E5793C)
	// 82E578B8: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82E578BC: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82E578C0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82E578C4: 556B06F6  rlwinm r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E578C8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E578CC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E578D0: 896A0001  lbz r11, 1(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E578D4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E578D8: 5565103E  rotlwi r5, r11, 2
	ctx.r[5].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82E578DC: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82E578E0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E578E4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E578E8: 806B16DC  lwz r3, 0x16dc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5852 as u32) ) } as u64;
	// 82E578EC: 4BFFFE85  bl 0x82e57770
	ctx.lr = 0x82E578F0;
	sub_82E57770(ctx, base);
	// 82E578F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E578F4: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82E578F8: 2B080007  cmplwi cr6, r8, 7
	ctx.cr[6].compare_u32(ctx.r[8].u32, 7 as u32, &mut ctx.xer);
	// 82E578FC: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57900: 4199FF94  bgt cr6, 0x82e57894
	if ctx.cr[6].gt {
	pc = 0x82E57894; continue 'dispatch;
	}
	// 82E57904: 3D8082E5  lis r12, -0x7d1b
	ctx.r[12].s64 = -2098921472;
	// 82E57908: 398C791C  addi r12, r12, 0x791c
	ctx.r[12].s64 = ctx.r[12].s64 + 31004;
	// 82E5790C: 5500103A  slwi r0, r8, 2
	ctx.r[0].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E57910: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E57914: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E57918: 4E800420  bctr
	match ctx.r[8].u64 {
		0 => {
	pc = 0x82E5793C; continue 'dispatch;
		},
		1 => {
	pc = 0x82E5793C; continue 'dispatch;
		},
		2 => {
	pc = 0x82E5793C; continue 'dispatch;
		},
		3 => {
	pc = 0x82E5793C; continue 'dispatch;
		},
		4 => {
	pc = 0x82E57954; continue 'dispatch;
		},
		5 => {
	pc = 0x82E57994; continue 'dispatch;
		},
		6 => {
	pc = 0x82E57994; continue 'dispatch;
		},
		7 => {
	pc = 0x82E57994; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E5791C: 82E5793C  lwz r23, 0x793c(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31036 as u32) ) } as u64;
	// 82E57920: 82E5793C  lwz r23, 0x793c(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31036 as u32) ) } as u64;
	// 82E57924: 82E5793C  lwz r23, 0x793c(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31036 as u32) ) } as u64;
	// 82E57928: 82E5793C  lwz r23, 0x793c(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31036 as u32) ) } as u64;
	// 82E5792C: 82E57954  lwz r23, 0x7954(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31060 as u32) ) } as u64;
	// 82E57930: 82E57994  lwz r23, 0x7994(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31124 as u32) ) } as u64;
	// 82E57934: 82E57994  lwz r23, 0x7994(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31124 as u32) ) } as u64;
	// 82E57938: 82E57994  lwz r23, 0x7994(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(31124 as u32) ) } as u64;
            }
            0x82E5793C => {
    //   block [0x82E5793C..0x82E57954)
	// 82E5793C: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57940: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E57944: 896A0003  lbz r11, 3(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82E57948: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5794C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E57950: 4BE51B0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E57954 => {
    //   block [0x82E57954..0x82E57994)
	// 82E57954: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57958: 896A0003  lbz r11, 3(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82E5795C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E57960: 8969000B  lbz r11, 0xb(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(11 as u32) ) } as u64;
	// 82E57964: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E57968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5796C: 409A0058  bne cr6, 0x82e579c4
	if !ctx.cr[6].eq {
	pc = 0x82E579C4; continue 'dispatch;
	}
	// 82E57970: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82E57974: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E57978: 4BFB14B1  bl 0x82e08e28
	ctx.lr = 0x82E5797C;
	sub_82E08E28(ctx, base);
	// 82E5797C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E57980: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E57984: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E57988: 480000E9  bl 0x82e57a70
	ctx.lr = 0x82E5798C;
	sub_82E57A70(ctx, base);
	// 82E5798C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E57990: 4BE51ACC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E57994 => {
    //   block [0x82E57994..0x82E579BC)
	// 82E57994: 894A0003  lbz r10, 3(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82E57998: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E5799C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E579A0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E579A4: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E579A8: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E579AC: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E579B0: 480000C1  bl 0x82e57a70
	ctx.lr = 0x82E579B4;
	sub_82E57A70(ctx, base);
	// 82E579B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E579B8: 4BE51AA4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E579BC => {
    //   block [0x82E579BC..0x82E579D0)
	// 82E579BC: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82E579C0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E579C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E579C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E579CC: 4BE51A90  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E579D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E579D0 size=160
    let mut pc: u32 = 0x82E579D0;
    'dispatch: loop {
        match pc {
            0x82E579D0 => {
    //   block [0x82E579D0..0x82E57A70)
	// 82E579D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E579D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E579D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E579DC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82E579E0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82E579E4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82E579E8: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E579EC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E579F0: 419A0014  beq cr6, 0x82e57a04
	if ctx.cr[6].eq {
	pc = 0x82E57A04; continue 'dispatch;
	}
	// 82E579F4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E579F8: 409A0064  bne cr6, 0x82e57a5c
	if !ctx.cr[6].eq {
	pc = 0x82E57A5C; continue 'dispatch;
	}
	// 82E579FC: 388A0030  addi r4, r10, 0x30
	ctx.r[4].s64 = ctx.r[10].s64 + 48;
	// 82E57A00: 48000008  b 0x82e57a08
	pc = 0x82E57A08; continue 'dispatch;
	// 82E57A04: 388A0020  addi r4, r10, 0x20
	ctx.r[4].s64 = ctx.r[10].s64 + 32;
	// 82E57A08: 896A0001  lbz r11, 1(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E57A0C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57A10: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82E57A14: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E57A18: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57A1C: 7D2B3A14  add r9, r11, r7
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E57A20: 806916DC  lwz r3, 0x16dc(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5852 as u32) ) } as u64;
	// 82E57A24: 4BFFFD4D  bl 0x82e57770
	ctx.lr = 0x82E57A28;
	sub_82E57770(ctx, base);
	// 82E57A28: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E57A2C: 409A0030  bne cr6, 0x82e57a5c
	if !ctx.cr[6].eq {
	pc = 0x82E57A5C; continue 'dispatch;
	}
	// 82E57A30: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E57A34: 419A0028  beq cr6, 0x82e57a5c
	if ctx.cr[6].eq {
	pc = 0x82E57A5C; continue 'dispatch;
	}
	// 82E57A38: 816916B4  lwz r11, 0x16b4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5812 as u32) ) } as u64;
	// 82E57A3C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E57A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57A44: 4E800421  bctrl
	ctx.lr = 0x82E57A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E57A48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E57A4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E57A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E57A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E57A58: 4E800020  blr
	return;
	// 82E57A5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E57A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E57A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E57A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E57A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57A70 size=776
    let mut pc: u32 = 0x82E57A70;
    'dispatch: loop {
        match pc {
            0x82E57A70 => {
    //   block [0x82E57A70..0x82E57D78)
	// 82E57A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57A74: 4BE51969  bl 0x82ca93dc
	ctx.lr = 0x82E57A78;
	sub_82CA93D0(ctx, base);
	// 82E57A78: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57A7C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82E57A80: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82E57A84: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82E57A88: 81580008  lwz r10, 8(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57A8C: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57A90: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E57A94: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57A98: 40980024  bge cr6, 0x82e57abc
	if !ctx.cr[6].lt {
	pc = 0x82E57ABC; continue 'dispatch;
	}
	// 82E57A9C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E57AA0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E57AA4: 40980008  bge cr6, 0x82e57aac
	if !ctx.cr[6].lt {
	pc = 0x82E57AAC; continue 'dispatch;
	}
	// 82E57AA8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E57AAC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E57AB0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E57AB4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E57AB8: 4BEFF459  bl 0x82d56f10
	ctx.lr = 0x82E57ABC;
	sub_82D56F10(ctx, base);
	// 82E57ABC: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57AC0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E57AC4: 7FB6EB78  mr r22, r29
	ctx.r[22].u64 = ctx.r[29].u64;
	// 82E57AC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E57ACC: 40990238  ble cr6, 0x82e57d04
	if !ctx.cr[6].gt {
	pc = 0x82E57D04; continue 'dispatch;
	}
	// 82E57AD0: 7FB5EB78  mr r21, r29
	ctx.r[21].u64 = ctx.r[29].u64;
	// 82E57AD4: 3E808000  lis r20, -0x8000
	ctx.r[20].s64 = -2147483648;
	// 82E57AD8: 3A40FFFF  li r18, -1
	ctx.r[18].s64 = -1;
	// 82E57ADC: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57AE0: 7F35582E  lwzx r25, r21, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57AE4: 3B990010  addi r28, r25, 0x10
	ctx.r[28].s64 = ctx.r[25].s64 + 16;
	// 82E57AE8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57AEC: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82E57AF0: 3AEB0010  addi r23, r11, 0x10
	ctx.r[23].s64 = ctx.r[11].s64 + 16;
	// 82E57AF4: 7F1CB840  cmplw cr6, r28, r23
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82E57AF8: 40980158  bge cr6, 0x82e57c50
	if !ctx.cr[6].lt {
	pc = 0x82E57C50; continue 'dispatch;
	}
	// 82E57AFC: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57B00: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82E57B04: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E57B08: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E57B0C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82E57B10: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E57B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57B18: 419A001C  beq cr6, 0x82e57b34
	if ctx.cr[6].eq {
	pc = 0x82E57B34; continue 'dispatch;
	}
	// 82E57B1C: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57B20: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E57B24: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E57B28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57B2C: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E57B30: 48000010  b 0x82e57b40
	pc = 0x82E57B40; continue 'dispatch;
	// 82E57B34: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E57B38: 4BEFD519  bl 0x82d55050
	ctx.lr = 0x82E57B3C;
	sub_82D55050(ctx, base);
	// 82E57B3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E57B40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57B44: 419A0028  beq cr6, 0x82e57b6c
	if ctx.cr[6].eq {
	pc = 0x82E57B6C; continue 'dispatch;
	}
	// 82E57B48: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E57B4C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E57B50: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E57B54: 928B0008  stw r20, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[20].u32 ) };
	// 82E57B58: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E57B5C: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E57B60: 928B0014  stw r20, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[20].u32 ) };
	// 82E57B64: 924B0018  stw r18, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[18].u32 ) };
	// 82E57B68: 48000008  b 0x82e57b70
	pc = 0x82E57B70; continue 'dispatch;
	// 82E57B6C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E57B70: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82E57B74: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82E57B78: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82E57B7C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82E57B80: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 82E57B84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E57B88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E57B8C: 4BFFFCA5  bl 0x82e57830
	ctx.lr = 0x82E57B90;
	sub_82E57830(ctx, base);
	// 82E57B90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E57B94: 409A017C  bne cr6, 0x82e57d10
	if !ctx.cr[6].eq {
	pc = 0x82E57D10; continue 'dispatch;
	}
	// 82E57B98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57B9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E57BA0: 419A005C  beq cr6, 0x82e57bfc
	if ctx.cr[6].eq {
	pc = 0x82E57BFC; continue 'dispatch;
	}
	// 82E57BA4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E57BA8: 3BD8000C  addi r30, r24, 0xc
	ctx.r[30].s64 = ctx.r[24].s64 + 12;
	// 82E57BAC: 92DF0018  stw r22, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[22].u32 ) };
	// 82E57BB0: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 82E57BB4: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82E57BB8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E57BBC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57BC0: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57BC4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E57BC8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57BCC: 409A0010  bne cr6, 0x82e57bdc
	if !ctx.cr[6].eq {
	pc = 0x82E57BDC; continue 'dispatch;
	}
	// 82E57BD0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E57BD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E57BD8: 4BEFF3C1  bl 0x82d56f98
	ctx.lr = 0x82E57BDC;
	sub_82D56F98(ctx, base);
	// 82E57BDC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57BE0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57BE4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57BE8: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82E57BEC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57BF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E57BF4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E57BF8: 48000048  b 0x82e57c40
	pc = 0x82E57C40; continue 'dispatch;
	// 82E57BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57C00: 4BFFE121  bl 0x82e55d20
	ctx.lr = 0x82E57C04;
	sub_82E55D20(ctx, base);
	// 82E57C04: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E57C08: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57C0C: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E57C10: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E57C14: 41980014  blt cr6, 0x82e57c28
	if ctx.cr[6].lt {
	pc = 0x82E57C28; continue 'dispatch;
	}
	// 82E57C18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E57C1C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E57C20: 4BEFD509  bl 0x82d55128
	ctx.lr = 0x82E57C24;
	sub_82D55128(ctx, base);
	// 82E57C24: 4800001C  b 0x82e57c40
	pc = 0x82E57C40; continue 'dispatch;
	// 82E57C28: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57C2C: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E57C30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E57C34: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E57C38: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E57C3C: 93E30050  stw r31, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E57C40: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57C44: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E57C48: 7F1CB840  cmplw cr6, r28, r23
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82E57C4C: 4198FEB8  blt cr6, 0x82e57b04
	if ctx.cr[6].lt {
	pc = 0x82E57B04; continue 'dispatch;
	}
	// 82E57C50: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57C54: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57C58: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E57C5C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57C60: 409A0010  bne cr6, 0x82e57c70
	if !ctx.cr[6].eq {
	pc = 0x82E57C70; continue 'dispatch;
	}
	// 82E57C64: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E57C68: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E57C6C: 4BEFF32D  bl 0x82d56f98
	ctx.lr = 0x82E57C70;
	sub_82D56F98(ctx, base);
	// 82E57C70: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57C74: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E57C78: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57C7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E57C80: 91780004  stw r11, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E57C84: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E57C88: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E57C8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E57C90: 419A0020  beq cr6, 0x82e57cb0
	if ctx.cr[6].eq {
	pc = 0x82E57CB0; continue 'dispatch;
	}
	// 82E57C94: 812B009C  lwz r9, 0x9c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E57C98: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E57C9C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E57CA0: 912B009C  stw r9, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u32 ) };
	// 82E57CA4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57CA8: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82E57CAC: 48000010  b 0x82e57cbc
	pc = 0x82E57CBC; continue 'dispatch;
	// 82E57CB0: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E57CB4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E57CB8: 4BEFD399  bl 0x82d55050
	ctx.lr = 0x82E57CBC;
	sub_82D55050(ctx, base);
	// 82E57CBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E57CC0: 419A0010  beq cr6, 0x82e57cd0
	if ctx.cr[6].eq {
	pc = 0x82E57CD0; continue 'dispatch;
	}
	// 82E57CC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E57CC8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E57CCC: 48000008  b 0x82e57cd4
	pc = 0x82E57CD4; continue 'dispatch;
	// 82E57CD0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E57CD4: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57CD8: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E57CDC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E57CE0: 7D75512E  stwx r11, r21, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[21].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82E57CE4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57CE8: 7C75582E  lwzx r3, r21, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57CEC: 4BE51795  bl 0x82ca9480
	ctx.lr = 0x82E57CF0;
	sub_82CA9480(ctx, base);
	// 82E57CF0: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57CF4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82E57CF8: 3AB50004  addi r21, r21, 4
	ctx.r[21].s64 = ctx.r[21].s64 + 4;
	// 82E57CFC: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57D00: 4198FDDC  blt cr6, 0x82e57adc
	if ctx.cr[6].lt {
	pc = 0x82E57ADC; continue 'dispatch;
	}
	// 82E57D04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E57D08: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E57D0C: 4BE51720  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
	// 82E57D10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E57D14: 419A0058  beq cr6, 0x82e57d6c
	if ctx.cr[6].eq {
	pc = 0x82E57D6C; continue 'dispatch;
	}
	// 82E57D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57D1C: 4BFFE005  bl 0x82e55d20
	ctx.lr = 0x82E57D20;
	sub_82E55D20(ctx, base);
	// 82E57D20: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57D24: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E57D28: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57D2C: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57D30: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E57D34: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E57D38: 4198001C  blt cr6, 0x82e57d54
	if ctx.cr[6].lt {
	pc = 0x82E57D54; continue 'dispatch;
	}
	// 82E57D3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E57D40: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E57D44: 4BEFD3E5  bl 0x82d55128
	ctx.lr = 0x82E57D48;
	sub_82D55128(ctx, base);
	// 82E57D48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E57D4C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E57D50: 4BE516DC  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
	// 82E57D54: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57D58: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E57D5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E57D60: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E57D64: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E57D68: 93E30050  stw r31, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E57D6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E57D70: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E57D74: 4BE516B8  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57D78 size=384
    let mut pc: u32 = 0x82E57D78;
    'dispatch: loop {
        match pc {
            0x82E57D78 => {
    //   block [0x82E57D78..0x82E57EF8)
	// 82E57D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57D7C: 4BE5167D  bl 0x82ca93f8
	ctx.lr = 0x82E57D80;
	sub_82CA93D0(ctx, base);
	// 82E57D80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57D84: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E57D88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E57D8C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E57D90: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57D94: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57D98: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E57D9C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E57DA0: 40980024  bge cr6, 0x82e57dc4
	if !ctx.cr[6].lt {
	pc = 0x82E57DC4; continue 'dispatch;
	}
	// 82E57DA4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57DA8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57DAC: 41980008  blt cr6, 0x82e57db4
	if ctx.cr[6].lt {
	pc = 0x82E57DB4; continue 'dispatch;
	}
	// 82E57DB0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E57DB4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E57DB8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E57DBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E57DC0: 4BEFF151  bl 0x82d56f10
	ctx.lr = 0x82E57DC4;
	sub_82D56F10(ctx, base);
	// 82E57DC4: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E57DC8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E57DCC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57DD0: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E57DD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E57DD8: 40990094  ble cr6, 0x82e57e6c
	if !ctx.cr[6].gt {
	pc = 0x82E57E6C; continue 'dispatch;
	}
	// 82E57DDC: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57DE0: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82E57DE4: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E57DE8: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E57DEC: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E57DF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E57DF4: 419A0020  beq cr6, 0x82e57e14
	if ctx.cr[6].eq {
	pc = 0x82E57E14; continue 'dispatch;
	}
	// 82E57DF8: 812B009C  lwz r9, 0x9c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E57DFC: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E57E00: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E57E04: 912B009C  stw r9, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u32 ) };
	// 82E57E08: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57E0C: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82E57E10: 48000010  b 0x82e57e20
	pc = 0x82E57E20; continue 'dispatch;
	// 82E57E14: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E57E18: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E57E1C: 4BEFD235  bl 0x82d55050
	ctx.lr = 0x82E57E20;
	sub_82D55050(ctx, base);
	// 82E57E20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E57E24: 419A0010  beq cr6, 0x82e57e34
	if ctx.cr[6].eq {
	pc = 0x82E57E34; continue 'dispatch;
	}
	// 82E57E28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E57E2C: 93230000  stw r25, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E57E30: 48000008  b 0x82e57e38
	pc = 0x82E57E38; continue 'dispatch;
	// 82E57E34: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82E57E38: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57E3C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E57E40: 7D7F512E  stwx r11, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82E57E44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57E48: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57E4C: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57E50: 7C7F502E  lwzx r3, r31, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E57E54: 4BE5162D  bl 0x82ca9480
	ctx.lr = 0x82E57E58;
	sub_82CA9480(ctx, base);
	// 82E57E58: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57E5C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E57E60: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E57E64: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57E68: 4198FF80  blt cr6, 0x82e57de8
	if ctx.cr[6].lt {
	pc = 0x82E57DE8; continue 'dispatch;
	}
	// 82E57E6C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57E70: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E57E74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E57E78: 40990068  ble cr6, 0x82e57ee0
	if !ctx.cr[6].gt {
	pc = 0x82E57EE0; continue 'dispatch;
	}
	// 82E57E7C: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E57E80: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82E57E84: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E57E88: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57E8C: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57E90: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E57E94: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E57E98: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57E9C: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E57EA0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E57EA4: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82E57EA8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E57EAC: 419A0010  beq cr6, 0x82e57ebc
	if ctx.cr[6].eq {
	pc = 0x82E57EBC; continue 'dispatch;
	}
	// 82E57EB0: 93250000  stw r25, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E57EB4: 93250004  stw r25, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E57EB8: 93650008  stw r27, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82E57EBC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E57EC0: 4BFFFEB9  bl 0x82e57d78
	ctx.lr = 0x82E57EC4;
	sub_82E57D78(ctx, base);
	// 82E57EC4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82E57EC8: 419A0024  beq cr6, 0x82e57eec
	if ctx.cr[6].eq {
	pc = 0x82E57EEC; continue 'dispatch;
	}
	// 82E57ECC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E57ED0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E57ED4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E57ED8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57EDC: 4198FFA8  blt cr6, 0x82e57e84
	if ctx.cr[6].lt {
	pc = 0x82E57E84; continue 'dispatch;
	}
	// 82E57EE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E57EE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E57EE8: 4BE51560  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82E57EEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E57EF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E57EF4: 4BE51554  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E57EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E57EF8 size=672
    let mut pc: u32 = 0x82E57EF8;
    'dispatch: loop {
        match pc {
            0x82E57EF8 => {
    //   block [0x82E57EF8..0x82E58198)
	// 82E57EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E57EFC: 4BE51501  bl 0x82ca93fc
	ctx.lr = 0x82E57F00;
	sub_82CA93D0(ctx, base);
	// 82E57F00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E57F04: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E57F08: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E57F0C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E57F10: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E57F14: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57F18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57F1C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E57F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E57F24: 4E800421  bctrl
	ctx.lr = 0x82E57F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E57F28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E57F2C: 409A0260  bne cr6, 0x82e5818c
	if !ctx.cr[6].eq {
	pc = 0x82E5818C; continue 'dispatch;
	}
	// 82E57F30: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57F34: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E57F38: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E57F3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E57F40: 419A024C  beq cr6, 0x82e5818c
	if ctx.cr[6].eq {
	pc = 0x82E5818C; continue 'dispatch;
	}
	// 82E57F44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E57F48: 393E00F4  addi r9, r30, 0xf4
	ctx.r[9].s64 = ctx.r[30].s64 + 244;
	// 82E57F4C: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82E57F50: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E57F54: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82E57F58: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E57F5C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E57F60: 4BFFF8D1  bl 0x82e57830
	ctx.lr = 0x82E57F64;
	sub_82E57830(ctx, base);
	// 82E57F64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E57F68: 409A0224  bne cr6, 0x82e5818c
	if !ctx.cr[6].eq {
	pc = 0x82E5818C; continue 'dispatch;
	}
	// 82E57F6C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E57F70: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E57F74: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E57F78: 997E0019  stb r11, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82E57F7C: 419A0010  beq cr6, 0x82e57f8c
	if ctx.cr[6].eq {
	pc = 0x82E57F8C; continue 'dispatch;
	}
	// 82E57F80: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E57F84: 387E0074  addi r3, r30, 0x74
	ctx.r[3].s64 = ctx.r[30].s64 + 116;
	// 82E57F88: 4BF00DA9  bl 0x82d58d30
	ctx.lr = 0x82E57F8C;
	sub_82D58D30(ctx, base);
	// 82E57F8C: 3BFE0068  addi r31, r30, 0x68
	ctx.r[31].s64 = ctx.r[30].s64 + 104;
	// 82E57F90: 837A0008  lwz r27, 8(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57F94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E57F98: 83BB0024  lwz r29, 0x24(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E57F9C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E57FA0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E57FA4: 40980024  bge cr6, 0x82e57fc8
	if !ctx.cr[6].lt {
	pc = 0x82E57FC8; continue 'dispatch;
	}
	// 82E57FA8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E57FAC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E57FB0: 41980008  blt cr6, 0x82e57fb8
	if ctx.cr[6].lt {
	pc = 0x82E57FB8; continue 'dispatch;
	}
	// 82E57FB4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E57FB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E57FBC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E57FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E57FC4: 4BEFEF4D  bl 0x82d56f10
	ctx.lr = 0x82E57FC8;
	sub_82D56F10(ctx, base);
	// 82E57FC8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E57FCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E57FD0: 815E006C  lwz r10, 0x6c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E57FD4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E57FD8: 40990024  ble cr6, 0x82e57ffc
	if !ctx.cr[6].gt {
	pc = 0x82E57FFC; continue 'dispatch;
	}
	// 82E57FDC: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E57FE0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E57FE4: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E57FE8: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82E57FEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E57FF0: 815E006C  lwz r10, 0x6c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E57FF4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E57FF8: 4198FFE4  blt cr6, 0x82e57fdc
	if ctx.cr[6].lt {
	pc = 0x82E57FDC; continue 'dispatch;
	}
	// 82E57FFC: 83FB003C  lwz r31, 0x3c(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58000: 3BBE0050  addi r29, r30, 0x50
	ctx.r[29].s64 = ctx.r[30].s64 + 80;
	// 82E58004: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E58008: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5800C: 7F8B51D6  mullw r28, r11, r10
	ctx.r[28].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82E58010: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58014: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E58018: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E5801C: 40980024  bge cr6, 0x82e58040
	if !ctx.cr[6].lt {
	pc = 0x82E58040; continue 'dispatch;
	}
	// 82E58020: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58024: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58028: 41980008  blt cr6, 0x82e58030
	if ctx.cr[6].lt {
	pc = 0x82E58030; continue 'dispatch;
	}
	// 82E5802C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E58030: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E58034: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E58038: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5803C: 4BEFEED5  bl 0x82d56f10
	ctx.lr = 0x82E58040;
	sub_82D56F10(ctx, base);
	// 82E58040: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E58044: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E58048: 895F000A  lbz r10, 0xa(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E5804C: 556B283E  rotlwi r11, r11, 5
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(5)) as u64;
	// 82E58050: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58054: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58058: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E5805C: 7CAA49D6  mullw r5, r10, r9
	ctx.r[5].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82E58060: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82E58064: 4BF00CCD  bl 0x82d58d30
	ctx.lr = 0x82E58068;
	sub_82D58D30(ctx, base);
	// 82E58068: 3BBE005C  addi r29, r30, 0x5c
	ctx.r[29].s64 = ctx.r[30].s64 + 92;
	// 82E5806C: A39F0004  lhz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58070: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58074: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E58078: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E5807C: 40980024  bge cr6, 0x82e580a0
	if !ctx.cr[6].lt {
	pc = 0x82E580A0; continue 'dispatch;
	}
	// 82E58080: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58084: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58088: 41980008  blt cr6, 0x82e58090
	if ctx.cr[6].lt {
	pc = 0x82E58090; continue 'dispatch;
	}
	// 82E5808C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E58090: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82E58094: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E58098: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5809C: 4BEFEE75  bl 0x82d56f10
	ctx.lr = 0x82E580A0;
	sub_82D56F10(ctx, base);
	// 82E580A0: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E580A4: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E580A8: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E580AC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E580B0: 5565283E  rotlwi r5, r11, 5
	ctx.r[5].u64 = ((ctx.r[11].u32).rotate_left(5)) as u64;
	// 82E580B4: 4BF00C7D  bl 0x82d58d30
	ctx.lr = 0x82E580B8;
	sub_82D58D30(ctx, base);
	// 82E580B8: 817B003C  lwz r11, 0x3c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E580BC: 395E0020  addi r10, r30, 0x20
	ctx.r[10].s64 = ctx.r[30].s64 + 32;
	// 82E580C0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82E580C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E580C8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E580CC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E580D0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82E580D4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82E580D8: 4200FFF0  bdnz 0x82e580c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E580C8; continue 'dispatch;
	}
	// 82E580DC: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E580E0: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E580E4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E580E8: 409A0014  bne cr6, 0x82e580fc
	if !ctx.cr[6].eq {
	pc = 0x82E580FC; continue 'dispatch;
	}
	// 82E580EC: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E580F0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E580F4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E580F8: 48000008  b 0x82e58100
	pc = 0x82E58100; continue 'dispatch;
	// 82E580FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E58100: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E58104: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E58108: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E5810C: 409A0014  bne cr6, 0x82e58120
	if !ctx.cr[6].eq {
	pc = 0x82E58120; continue 'dispatch;
	}
	// 82E58110: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58114: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E58118: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E5811C: 48000008  b 0x82e58124
	pc = 0x82E58124; continue 'dispatch;
	// 82E58120: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E58124: 89790000  lbz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58128: 997E0018  stb r11, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82E5812C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E58130: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58134: 419A0038  beq cr6, 0x82e5816c
	if ctx.cr[6].eq {
	pc = 0x82E5816C; continue 'dispatch;
	}
	// 82E58138: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5813C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58140: 4E800421  bctrl
	ctx.lr = 0x82E58144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58144: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E58148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5814C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E58150: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58154: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58158: 4E800421  bctrl
	ctx.lr = 0x82E5815C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5815C: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82E58160: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E58164: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E58168: 4BE512E4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82E5816C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E58170: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E58174: 4BF2824D  bl 0x82d803c0
	ctx.lr = 0x82E58178;
	sub_82D803C0(ctx, base);
	// 82E58178: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5817C: 4BF28245  bl 0x82d803c0
	ctx.lr = 0x82E58180;
	sub_82D803C0(ctx, base);
	// 82E58180: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E58184: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E58188: 4BE512C4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82E5818C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E58190: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E58194: 4BE512B8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E58198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E58198 size=416
    let mut pc: u32 = 0x82E58198;
    'dispatch: loop {
        match pc {
            0x82E58198 => {
    //   block [0x82E58198..0x82E58240)
	// 82E58198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5819C: 4BE51269  bl 0x82ca9404
	ctx.lr = 0x82E581A0;
	sub_82CA93D0(ctx, base);
	// 82E581A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E581A4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82E581A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E581AC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E581B0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E581B4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E581B8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E581BC: 419A0014  beq cr6, 0x82e581d0
	if ctx.cr[6].eq {
	pc = 0x82E581D0; continue 'dispatch;
	}
	// 82E581C0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E581C4: 409A0168  bne cr6, 0x82e5832c
	if !ctx.cr[6].eq {
	pc = 0x82E5832C; continue 'dispatch;
	}
	// 82E581C8: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82E581CC: 48000008  b 0x82e581d4
	pc = 0x82E581D4; continue 'dispatch;
	// 82E581D0: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E581D4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E581D8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E581DC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82E581E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E581E4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E581E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E581EC: 806B16DC  lwz r3, 0x16dc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5852 as u32) ) } as u64;
	// 82E581F0: 4BFFF581  bl 0x82e57770
	ctx.lr = 0x82E581F4;
	sub_82E57770(ctx, base);
	// 82E581F4: 7F032000  cmpw cr6, r3, r4
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82E581F8: 409A0134  bne cr6, 0x82e5832c
	if !ctx.cr[6].eq {
	pc = 0x82E5832C; continue 'dispatch;
	}
	// 82E581FC: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82E58200: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E58204: 41990128  bgt cr6, 0x82e5832c
	if ctx.cr[6].gt {
	pc = 0x82E5832C; continue 'dispatch;
	}
	// 82E58208: 3D8082E6  lis r12, -0x7d1a
	ctx.r[12].s64 = -2098855936;
	// 82E5820C: 398C8220  addi r12, r12, -0x7de0
	ctx.r[12].s64 = ctx.r[12].s64 + -32224;
	// 82E58210: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E58214: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E58218: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E5821C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82E58240; continue 'dispatch;
		},
		1 => {
	pc = 0x82E58240; continue 'dispatch;
		},
		2 => {
	pc = 0x82E58240; continue 'dispatch;
		},
		3 => {
	pc = 0x82E58240; continue 'dispatch;
		},
		4 => {
	pc = 0x82E5826C; continue 'dispatch;
		},
		5 => {
	pc = 0x82E582E8; continue 'dispatch;
		},
		6 => {
	pc = 0x82E582E8; continue 'dispatch;
		},
		7 => {
	pc = 0x82E582E8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E58220: 82E58240  lwz r23, -0x7dc0(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32192 as u32) ) } as u64;
	// 82E58224: 82E58240  lwz r23, -0x7dc0(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32192 as u32) ) } as u64;
	// 82E58228: 82E58240  lwz r23, -0x7dc0(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32192 as u32) ) } as u64;
	// 82E5822C: 82E58240  lwz r23, -0x7dc0(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32192 as u32) ) } as u64;
	// 82E58230: 82E5826C  lwz r23, -0x7d94(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32148 as u32) ) } as u64;
	// 82E58234: 82E582E8  lwz r23, -0x7d18(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32024 as u32) ) } as u64;
	// 82E58238: 82E582E8  lwz r23, -0x7d18(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32024 as u32) ) } as u64;
	// 82E5823C: 82E582E8  lwz r23, -0x7d18(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-32024 as u32) ) } as u64;
            }
            0x82E58240 => {
    //   block [0x82E58240..0x82E5826C)
	// 82E58240: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 82E58244: 389D008C  addi r4, r29, 0x8c
	ctx.r[4].s64 = ctx.r[29].s64 + 140;
	// 82E58248: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E5824C: 4BF00AE5  bl 0x82d58d30
	ctx.lr = 0x82E58250;
	sub_82D58D30(ctx, base);
	// 82E58250: 897D0076  lbz r11, 0x76(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(118 as u32) ) } as u64;
	// 82E58254: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E58258: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82E5825C: 897D0077  lbz r11, 0x77(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(119 as u32) ) } as u64;
	// 82E58260: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82E58264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E58268: 4BE511EC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E5826C => {
    //   block [0x82E5826C..0x82E582E8)
	// 82E5826C: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 82E58270: 389D008C  addi r4, r29, 0x8c
	ctx.r[4].s64 = ctx.r[29].s64 + 140;
	// 82E58274: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E58278: 4BF00AB9  bl 0x82d58d30
	ctx.lr = 0x82E5827C;
	sub_82D58D30(ctx, base);
	// 82E5827C: 897D0076  lbz r11, 0x76(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(118 as u32) ) } as u64;
	// 82E58280: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82E58284: 897D0077  lbz r11, 0x77(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(119 as u32) ) } as u64;
	// 82E58288: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82E5828C: 897E000B  lbz r11, 0xb(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(11 as u32) ) } as u64;
	// 82E58290: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E58294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58298: 409A0044  bne cr6, 0x82e582dc
	if !ctx.cr[6].eq {
	pc = 0x82E582DC; continue 'dispatch;
	}
	// 82E5829C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E582A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E582A4: 4BFB0B7D  bl 0x82e08e20
	ctx.lr = 0x82E582A8;
	sub_82E08E20(ctx, base);
	// 82E582A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E582AC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E582B0: 419A0018  beq cr6, 0x82e582c8
	if ctx.cr[6].eq {
	pc = 0x82E582C8; continue 'dispatch;
	}
	// 82E582B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E582B8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E582BC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E582C0: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E582C4: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E582C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E582CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E582D0: 4BFFFAA9  bl 0x82e57d78
	ctx.lr = 0x82E582D4;
	sub_82E57D78(ctx, base);
	// 82E582D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E582D8: 4BE5117C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E582DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E582E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E582E4: 4BE51170  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E582E8 => {
    //   block [0x82E582E8..0x82E58338)
	// 82E582E8: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 82E582EC: 389D008C  addi r4, r29, 0x8c
	ctx.r[4].s64 = ctx.r[29].s64 + 140;
	// 82E582F0: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E582F4: 4BF00A3D  bl 0x82d58d30
	ctx.lr = 0x82E582F8;
	sub_82D58D30(ctx, base);
	// 82E582F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E582FC: 419A0018  beq cr6, 0x82e58314
	if ctx.cr[6].eq {
	pc = 0x82E58314; continue 'dispatch;
	}
	// 82E58300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E58304: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E58308: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5830C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E58310: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E58314: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E58318: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5831C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E58320: 4BFFFA59  bl 0x82e57d78
	ctx.lr = 0x82E58324;
	sub_82E57D78(ctx, base);
	// 82E58324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E58328: 4BE5112C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E5832C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E58330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E58334: 4BE51120  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E58338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E58338 size=720
    let mut pc: u32 = 0x82E58338;
    'dispatch: loop {
        match pc {
            0x82E58338 => {
    //   block [0x82E58338..0x82E58608)
	// 82E58338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5833C: 4BE51095  bl 0x82ca93d0
	ctx.lr = 0x82E58340;
	sub_82CA93D0(ctx, base);
	// 82E58340: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E58344: 7C902378  mr r16, r4
	ctx.r[16].u64 = ctx.r[4].u64;
	// 82E58348: 90610114  stw r3, 0x114(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), ctx.r[3].u32 ) };
	// 82E5834C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E58350: 90A10124  stw r5, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[5].u32 ) };
	// 82E58354: 3BF00028  addi r31, r16, 0x28
	ctx.r[31].s64 = ctx.r[16].s64 + 40;
	// 82E58358: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82E5835C: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82E58360: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82E58364: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58368: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E5836C: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82E58370: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E58374: 40990038  ble cr6, 0x82e583ac
	if !ctx.cr[6].gt {
	pc = 0x82E583AC; continue 'dispatch;
	}
	// 82E58378: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5837C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E58380: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E58384: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E58388: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5838C: 4BEFCEBD  bl 0x82d55248
	ctx.lr = 0x82E58390;
	sub_82D55248(ctx, base);
	// 82E58390: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E58394: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E58398: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5839C: 554A0042  rlwinm r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E583A0: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82E583A4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E583A8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E583AC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E583B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E583B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E583B8: 40990024  ble cr6, 0x82e583dc
	if !ctx.cr[6].gt {
	pc = 0x82E583DC; continue 'dispatch;
	}
	// 82E583BC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E583C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E583C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E583C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E583CC: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E583D0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E583D4: 409AFFE8  bne cr6, 0x82e583bc
	if !ctx.cr[6].eq {
	pc = 0x82E583BC; continue 'dispatch;
	}
	// 82E583D8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E583DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E583E0: 80D00038  lwz r6, 0x38(r16)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E583E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E583E8: 80B00034  lwz r5, 0x34(r16)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E583EC: 4BFD521D  bl 0x82e2d608
	ctx.lr = 0x82E583F0;
	sub_82E2D608(ctx, base);
	// 82E583F0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E583F4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E583F8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E583FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E58400: 409901D8  ble cr6, 0x82e585d8
	if !ctx.cr[6].gt {
	pc = 0x82E585D8; continue 'dispatch;
	}
	// 82E58404: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E58408: 7FCFF378  mr r15, r30
	ctx.r[15].u64 = ctx.r[30].u64;
	// 82E5840C: 3AEB6860  addi r23, r11, 0x6860
	ctx.r[23].s64 = ctx.r[11].s64 + 26720;
	// 82E58410: 3A800120  li r20, 0x120
	ctx.r[20].s64 = 288;
	// 82E58414: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82E58418: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82E5841C: 3AA00016  li r21, 0x16
	ctx.r[21].s64 = 22;
	// 82E58420: 3AC00003  li r22, 3
	ctx.r[22].s64 = 3;
	// 82E58424: 3A200103  li r17, 0x103
	ctx.r[17].s64 = 259;
	// 82E58428: 7D6F202E  lwzx r11, r15, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E5842C: 7FD2F378  mr r18, r30
	ctx.r[18].u64 = ctx.r[30].u64;
	// 82E58430: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E58434: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E58438: 40990184  ble cr6, 0x82e585bc
	if !ctx.cr[6].gt {
	pc = 0x82E585BC; continue 'dispatch;
	}
	// 82E5843C: 7FCEF378  mr r14, r30
	ctx.r[14].u64 = ctx.r[30].u64;
	// 82E58440: 7D6F202E  lwzx r11, r15, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E58444: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 82E58448: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E5844C: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E58450: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58454: 7D6A702E  lwzx r11, r10, r14
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82E58458: 39CE0004  addi r14, r14, 4
	ctx.r[14].s64 = ctx.r[14].s64 + 4;
	// 82E5845C: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 82E58460: 409A0014  bne cr6, 0x82e58474
	if !ctx.cr[6].eq {
	pc = 0x82E58474; continue 'dispatch;
	}
	// 82E58464: 7D4F202E  lwzx r10, r15, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E58468: 814A005C  lwz r10, 0x5c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E5846C: 7E6A5A14  add r19, r10, r11
	ctx.r[19].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E58470: 48000008  b 0x82e58478
	pc = 0x82E58478; continue 'dispatch;
	// 82E58474: 3A6B0200  addi r19, r11, 0x200
	ctx.r[19].s64 = ctx.r[11].s64 + 512;
	// 82E58478: 7F0B9840  cmplw cr6, r11, r19
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[19].u32, &mut ctx.xer);
	// 82E5847C: 40980130  bge cr6, 0x82e585ac
	if !ctx.cr[6].lt {
	pc = 0x82E585AC; continue 'dispatch;
	}
	// 82E58480: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58484: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	// 82E58488: 38A0000D  li r5, 0xd
	ctx.r[5].s64 = 13;
	// 82E5848C: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82E58490: 38800120  li r4, 0x120
	ctx.r[4].s64 = 288;
	// 82E58494: 4BEFCDB5  bl 0x82d55248
	ctx.lr = 0x82E58498;
	sub_82D55248(ctx, base);
	// 82E58498: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5849C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82E584A0: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82E584A4: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82E584A8: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
	// 82E584AC: B29F0004  sth r20, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[20].u16 ) };
	// 82E584B0: B37F0006  sth r27, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[27].u16 ) };
	// 82E584B4: 92FF0000  stw r23, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82E584B8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E584BC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E584C0: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E584C4: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82E584C8: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E584CC: B2AA0000  sth r21, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u16 ) };
	// 82E584D0: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E584D4: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82E584D8: B2CB0002  sth r22, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[22].u16 ) };
	// 82E584DC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E584E0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E584E4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E584E8: 4200FFF8  bdnz 0x82e584e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E584E0; continue 'dispatch;
	}
	// 82E584EC: 397F00F4  addi r11, r31, 0xf4
	ctx.r[11].s64 = ctx.r[31].s64 + 244;
	// 82E584F0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E584F4: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E584F8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E584FC: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82E58500: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E58504: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82E58508: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E5850C: 939F0064  stw r28, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E58510: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82E58514: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E58518: 939F0070  stw r28, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 82E5851C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E58520: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E58524: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E58528: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E5852C: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E58530: 938B0014  stw r28, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82E58534: 923F010C  stw r17, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[17].u32 ) };
	// 82E58538: 937F0110  stw r27, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[27].u32 ) };
	// 82E5853C: 80610114  lwz r3, 0x114(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 82E58540: 80B0006C  lwz r5, 0x6c(r16)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E58544: 4BFFF9B5  bl 0x82e57ef8
	ctx.lr = 0x82E58548;
	sub_82E57EF8(ctx, base);
	// 82E58548: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E5854C: 409A0010  bne cr6, 0x82e5855c
	if !ctx.cr[6].eq {
	pc = 0x82E5855C; continue 'dispatch;
	}
	// 82E58550: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E58554: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82E58558: 4BFFDC79  bl 0x82e561d0
	ctx.lr = 0x82E5855C;
	sub_82E561D0(ctx, base);
	// 82E5855C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58560: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58564: 419A0034  beq cr6, 0x82e58598
	if ctx.cr[6].eq {
	pc = 0x82E58598; continue 'dispatch;
	}
	// 82E58568: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E5856C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E58570: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E58574: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E58578: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E5857C: 409A001C  bne cr6, 0x82e58598
	if !ctx.cr[6].eq {
	pc = 0x82E58598; continue 'dispatch;
	}
	// 82E58580: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58584: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E58588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5858C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58594: 4E800421  bctrl
	ctx.lr = 0x82E58598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58598: 89780003  lbz r11, 3(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(3 as u32) ) } as u64;
	// 82E5859C: 7F0BC214  add r24, r11, r24
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82E585A0: 7F189840  cmplw cr6, r24, r19
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[19].u32, &mut ctx.xer);
	// 82E585A4: 4198FEE4  blt cr6, 0x82e58488
	if ctx.cr[6].lt {
	pc = 0x82E58488; continue 'dispatch;
	}
	// 82E585A8: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E585AC: 7D6F202E  lwzx r11, r15, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E585B0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E585B4: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E585B8: 4198FE88  blt cr6, 0x82e58440
	if ctx.cr[6].lt {
	pc = 0x82E58440; continue 'dispatch;
	}
	// 82E585BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E585C0: 39EF0004  addi r15, r15, 4
	ctx.r[15].s64 = ctx.r[15].s64 + 4;
	// 82E585C4: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E585C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E585CC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E585D0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E585D4: 4198FE54  blt cr6, 0x82e58428
	if ctx.cr[6].lt {
	pc = 0x82E58428; continue 'dispatch;
	}
	// 82E585D8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E585DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E585E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E585E4: 409A001C  bne cr6, 0x82e58600
	if !ctx.cr[6].eq {
	pc = 0x82E58600; continue 'dispatch;
	}
	// 82E585E8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E585EC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E585F0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E585F4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E585F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E585FC: 4BEFCCCD  bl 0x82d552c8
	ctx.lr = 0x82E58600;
	sub_82D552C8(ctx, base);
	// 82E58600: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82E58604: 4BE50E1C  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E58608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E58608 size=544
    let mut pc: u32 = 0x82E58608;
    'dispatch: loop {
        match pc {
            0x82E58608 => {
    //   block [0x82E58608..0x82E58828)
	// 82E58608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5860C: 4BE50DC5  bl 0x82ca93d0
	ctx.lr = 0x82E58610;
	sub_82CA93D0(ctx, base);
	// 82E58610: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E58614: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E58618: 90C1012C  stw r6, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[6].u32 ) };
	// 82E5861C: 7C711B78  mr r17, r3
	ctx.r[17].u64 = ctx.r[3].u64;
	// 82E58620: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E58624: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E58628: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5862C: 820B0008  lwz r16, 8(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58630: 4B40E911  bl 0x82266f40
	ctx.lr = 0x82E58634;
	sub_82266F40(ctx, base);
	// 82E58634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E58638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5863C: 4B40D59D  bl 0x82265bd8
	ctx.lr = 0x82E58640;
	sub_82265BD8(ctx, base);
	// 82E58640: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E58644: 409901D4  ble cr6, 0x82e58818
	if !ctx.cr[6].gt {
	pc = 0x82E58818; continue 'dispatch;
	}
	// 82E58648: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5864C: 7FCFF378  mr r15, r30
	ctx.r[15].u64 = ctx.r[30].u64;
	// 82E58650: 7FEEFB78  mr r14, r31
	ctx.r[14].u64 = ctx.r[31].u64;
	// 82E58654: 3B2B6860  addi r25, r11, 0x6860
	ctx.r[25].s64 = ctx.r[11].s64 + 26720;
	// 82E58658: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5865C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82E58660: 3AC00120  li r22, 0x120
	ctx.r[22].s64 = 288;
	// 82E58664: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82E58668: 3AE00016  li r23, 0x16
	ctx.r[23].s64 = 22;
	// 82E5866C: 3B000003  li r24, 3
	ctx.r[24].s64 = 3;
	// 82E58670: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82E58674: 3A600103  li r19, 0x103
	ctx.r[19].s64 = 259;
	// 82E58678: 816F0000  lwz r11, 0(r15)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5867C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E58680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E58684: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82E58688: 4B40D579  bl 0x82265c00
	ctx.lr = 0x82E5868C;
	sub_82265C00(ctx, base);
	// 82E5868C: 816F0000  lwz r11, 0(r15)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58690: 7FD2F378  mr r18, r30
	ctx.r[18].u64 = ctx.r[30].u64;
	// 82E58694: 3AAB005C  addi r21, r11, 0x5c
	ctx.r[21].s64 = ctx.r[11].s64 + 92;
	// 82E58698: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5869C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E586A0: 40990168  ble cr6, 0x82e58808
	if !ctx.cr[6].gt {
	pc = 0x82E58808; continue 'dispatch;
	}
	// 82E586A4: 7FD4F378  mr r20, r30
	ctx.r[20].u64 = ctx.r[30].u64;
	// 82E586A8: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E586AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E586B0: 7F545A14  add r26, r20, r11
	ctx.r[26].u64 = ctx.r[20].u64 + ctx.r[11].u64;
	// 82E586B4: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E586B8: 4BF07F79  bl 0x82d60630
	ctx.lr = 0x82E586BC;
	sub_82D60630(ctx, base);
	// 82E586BC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E586C0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E586C4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82E586C8: 40990008  ble cr6, 0x82e586d0
	if !ctx.cr[6].gt {
	pc = 0x82E586D0; continue 'dispatch;
	}
	// 82E586CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E586D0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E586D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E586D8: 409A011C  bne cr6, 0x82e587f4
	if !ctx.cr[6].eq {
	pc = 0x82E587F4; continue 'dispatch;
	}
	// 82E586DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E586E0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E586E4: 38A0000D  li r5, 0xd
	ctx.r[5].s64 = 13;
	// 82E586E8: 38800120  li r4, 0x120
	ctx.r[4].s64 = 288;
	// 82E586EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E586F0: 4BEFCB59  bl 0x82d55248
	ctx.lr = 0x82E586F4;
	sub_82D55248(ctx, base);
	// 82E586F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E586F8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82E586FC: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82E58700: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82E58704: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
	// 82E58708: B2DF0004  sth r22, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[22].u16 ) };
	// 82E5870C: B37F0006  sth r27, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[27].u16 ) };
	// 82E58710: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E58714: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E58718: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E5871C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E58720: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82E58724: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E58728: B2EA0000  sth r23, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u16 ) };
	// 82E5872C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E58730: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82E58734: B30B0002  sth r24, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[24].u16 ) };
	// 82E58738: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E5873C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E58740: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E58744: 4200FFF8  bdnz 0x82e5873c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E5873C; continue 'dispatch;
	}
	// 82E58748: 397F00F4  addi r11, r31, 0xf4
	ctx.r[11].s64 = ctx.r[31].s64 + 244;
	// 82E5874C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E58750: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E58754: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E58758: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82E5875C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 82E58760: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82E58764: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E58768: 939F0064  stw r28, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E5876C: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82E58770: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E58774: 939F0070  stw r28, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 82E58778: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E5877C: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E58780: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E58784: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E58788: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E5878C: 938B0014  stw r28, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82E58790: 927F010C  stw r19, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[19].u32 ) };
	// 82E58794: 937F0110  stw r27, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[27].u32 ) };
	// 82E58798: 80B0006C  lwz r5, 0x6c(r16)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5879C: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E587A0: 4BFFF759  bl 0x82e57ef8
	ctx.lr = 0x82E587A4;
	sub_82E57EF8(ctx, base);
	// 82E587A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E587A8: 409A0010  bne cr6, 0x82e587b8
	if !ctx.cr[6].eq {
	pc = 0x82E587B8; continue 'dispatch;
	}
	// 82E587AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E587B0: 8061012C  lwz r3, 0x12c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E587B4: 4BFFDA1D  bl 0x82e561d0
	ctx.lr = 0x82E587B8;
	sub_82E561D0(ctx, base);
	// 82E587B8: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E587BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E587C0: 419A0034  beq cr6, 0x82e587f4
	if ctx.cr[6].eq {
	pc = 0x82E587F4; continue 'dispatch;
	}
	// 82E587C4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E587C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E587CC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E587D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E587D4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E587D8: 409A001C  bne cr6, 0x82e587f4
	if !ctx.cr[6].eq {
	pc = 0x82E587F4; continue 'dispatch;
	}
	// 82E587DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E587E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E587E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E587E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E587EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E587F0: 4E800421  bctrl
	ctx.lr = 0x82E587F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E587F4: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E587F8: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 82E587FC: 3A940008  addi r20, r20, 8
	ctx.r[20].s64 = ctx.r[20].s64 + 8;
	// 82E58800: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58804: 4198FEA4  blt cr6, 0x82e586a8
	if ctx.cr[6].lt {
	pc = 0x82E586A8; continue 'dispatch;
	}
	// 82E58808: 39CEFFFF  addi r14, r14, -1
	ctx.r[14].s64 = ctx.r[14].s64 + -1;
	// 82E5880C: 39EF0004  addi r15, r15, 4
	ctx.r[15].s64 = ctx.r[15].s64 + 4;
	// 82E58810: 2B0E0000  cmplwi cr6, r14, 0
	ctx.cr[6].compare_u32(ctx.r[14].u32, 0 as u32, &mut ctx.xer);
	// 82E58814: 409AFE64  bne cr6, 0x82e58678
	if !ctx.cr[6].eq {
	pc = 0x82E58678; continue 'dispatch;
	}
	// 82E58818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5881C: 4B40E6ED  bl 0x82266f08
	ctx.lr = 0x82E58820;
	sub_82266F08(ctx, base);
	// 82E58820: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82E58824: 4BE50BFC  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E58828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E58828 size=244
    let mut pc: u32 = 0x82E58828;
    'dispatch: loop {
        match pc {
            0x82E58828 => {
    //   block [0x82E58828..0x82E5891C)
	// 82E58828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5882C: 4BE50BC9  bl 0x82ca93f4
	ctx.lr = 0x82E58830;
	sub_82CA93D0(ctx, base);
	// 82E58830: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E58834: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58838: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E5883C: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82E58840: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58844: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E58848: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E5884C: 409A0014  bne cr6, 0x82e58860
	if !ctx.cr[6].eq {
	pc = 0x82E58860; continue 'dispatch;
	}
	// 82E58850: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58854: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E58858: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E5885C: 48000008  b 0x82e58864
	pc = 0x82E58864; continue 'dispatch;
	// 82E58860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E58864: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58868: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E5886C: 409900A8  ble cr6, 0x82e58914
	if !ctx.cr[6].gt {
	pc = 0x82E58914; continue 'dispatch;
	}
	// 82E58870: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58874: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82E58878: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5887C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82E58880: 3B600120  li r27, 0x120
	ctx.r[27].s64 = 288;
	// 82E58884: 38A0000D  li r5, 0xd
	ctx.r[5].s64 = 13;
	// 82E58888: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E5888C: 38800120  li r4, 0x120
	ctx.r[4].s64 = 288;
	// 82E58890: 4BEFC9B9  bl 0x82d55248
	ctx.lr = 0x82E58894;
	sub_82D55248(ctx, base);
	// 82E58894: B3630004  sth r27, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82E58898: 48000929  bl 0x82e591c0
	ctx.lr = 0x82E5889C;
	sub_82E591C0(ctx, base);
	// 82E5889C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E588A0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E588A4: 80BA006C  lwz r5, 0x6c(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E588A8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E588AC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E588B0: 4BFFF649  bl 0x82e57ef8
	ctx.lr = 0x82E588B4;
	sub_82E57EF8(ctx, base);
	// 82E588B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E588B8: 409A0010  bne cr6, 0x82e588c8
	if !ctx.cr[6].eq {
	pc = 0x82E588C8; continue 'dispatch;
	}
	// 82E588BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E588C0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E588C4: 4BFFD90D  bl 0x82e561d0
	ctx.lr = 0x82E588C8;
	sub_82E561D0(ctx, base);
	// 82E588C8: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E588CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E588D0: 419A0034  beq cr6, 0x82e58904
	if ctx.cr[6].eq {
	pc = 0x82E58904; continue 'dispatch;
	}
	// 82E588D4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E588D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E588DC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E588E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E588E4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E588E8: 409A001C  bne cr6, 0x82e58904
	if !ctx.cr[6].eq {
	pc = 0x82E58904; continue 'dispatch;
	}
	// 82E588EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E588F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E588F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E588F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E588FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58900: 4E800421  bctrl
	ctx.lr = 0x82E58904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58904: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 82E58908: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5890C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E58910: 409AFF74  bne cr6, 0x82e58884
	if !ctx.cr[6].eq {
	pc = 0x82E58884; continue 'dispatch;
	}
	// 82E58914: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E58918: 4BE50B2C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E58920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E58920 size=1844
    let mut pc: u32 = 0x82E58920;
    'dispatch: loop {
        match pc {
            0x82E58920 => {
    //   block [0x82E58920..0x82E59054)
	// 82E58920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E58924: 4BE50AB5  bl 0x82ca93d8
	ctx.lr = 0x82E58928;
	sub_82CA93D0(ctx, base);
	// 82E58928: DBC1FF68  stfd f30, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[30].u64 ) };
	// 82E5892C: DBE1FF70  stfd f31, -0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82E58930: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82E58934: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82E58938: E981D000  ld r12, -0x3000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-12288 as u32) ) };
	// 82E5893C: 9421CDF0  stwu r1, -0x3210(r1)
	ea = ctx.r[1].u32.wrapping_add(-12816 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E58940: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82E58944: 7CD13378  mr r17, r6
	ctx.r[17].u64 = ctx.r[6].u64;
	// 82E58948: 7D134378  mr r19, r8
	ctx.r[19].u64 = ctx.r[8].u64;
	// 82E5894C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E58950: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82E58954: 809500B8  lwz r4, 0xb8(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E58958: 7CF03B78  mr r16, r7
	ctx.r[16].u64 = ctx.r[7].u64;
	// 82E5895C: 80B100B8  lwz r5, 0xb8(r17)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E58960: 83F30008  lwz r31, 8(r19)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58964: 7F042840  cmplw cr6, r4, r5
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82E58968: 409A000C  bne cr6, 0x82e58974
	if !ctx.cr[6].eq {
	pc = 0x82E58974; continue 'dispatch;
	}
	// 82E5896C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E58970: 48000038  b 0x82e589a8
	pc = 0x82E589A8; continue 'dispatch;
	// 82E58974: 897500D8  lbz r11, 0xd8(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[21].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E58978: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E5897C: 409A000C  bne cr6, 0x82e58988
	if !ctx.cr[6].eq {
	pc = 0x82E58988; continue 'dispatch;
	}
	// 82E58980: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82E58984: 48000024  b 0x82e589a8
	pc = 0x82E589A8; continue 'dispatch;
	// 82E58988: 897100D8  lbz r11, 0xd8(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[17].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E5898C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E58990: 409A000C  bne cr6, 0x82e5899c
	if !ctx.cr[6].eq {
	pc = 0x82E5899C; continue 'dispatch;
	}
	// 82E58994: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E58998: 48000010  b 0x82e589a8
	pc = 0x82E589A8; continue 'dispatch;
	// 82E5899C: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82E589A0: 4BF42D31  bl 0x82d9b6d0
	ctx.lr = 0x82E589A4;
	sub_82D9B6D0(ctx, base);
	// 82E589A4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E589A8: 897D0019  lbz r11, 0x19(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E589AC: 7E679B78  mr r7, r19
	ctx.r[7].u64 = ctx.r[19].u64;
	// 82E589B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E589B4: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82E589B8: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82E589BC: 4BFFF015  bl 0x82e579d0
	ctx.lr = 0x82E589C0;
	sub_82E579D0(ctx, base);
	// 82E589C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E589C4: 409A067C  bne cr6, 0x82e59040
	if !ctx.cr[6].eq {
	pc = 0x82E59040; continue 'dispatch;
	}
	// 82E589C8: 81730010  lwz r11, 0x10(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E589CC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E589D0: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E589D4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E589D8: 409A0014  bne cr6, 0x82e589ec
	if !ctx.cr[6].eq {
	pc = 0x82E589EC; continue 'dispatch;
	}
	// 82E589DC: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E589E0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E589E4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E589E8: 48000008  b 0x82e589f0
	pc = 0x82E589F0; continue 'dispatch;
	// 82E589EC: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82E589F0: 7F155840  cmplw cr6, r21, r11
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E589F4: 419A0074  beq cr6, 0x82e58a68
	if ctx.cr[6].eq {
	pc = 0x82E58A68; continue 'dispatch;
	}
	// 82E589F8: 81530014  lwz r10, 0x14(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E589FC: 81730010  lwz r11, 0x10(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58A00: 91530010  stw r10, 0x10(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E58A04: 91730014  stw r11, 0x14(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E58A08: A1530006  lhz r10, 6(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E58A0C: A1730004  lhz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58A10: B1530004  sth r10, 4(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82E58A14: B1730006  sth r11, 6(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E58A18: 81730008  lwz r11, 8(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58A1C: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 82E58A20: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E58A24: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E58A28: 912B0014  stw r9, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82E58A2C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82E58A30: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58A34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E58A38: 419A0030  beq cr6, 0x82e58a68
	if ctx.cr[6].eq {
	pc = 0x82E58A68; continue 'dispatch;
	}
	// 82E58A3C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E58A40: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58A44: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58A48: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E58A4C: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E58A50: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58A54: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E58A58: 894A001A  lbz r10, 0x1a(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(26 as u32) ) } as u64;
	// 82E58A5C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E58A60: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E58A64: 994B001A  stb r10, 0x1a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[10].u8 ) };
	// 82E58A68: 897D0019  lbz r11, 0x19(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E58A6C: 7E679B78  mr r7, r19
	ctx.r[7].u64 = ctx.r[19].u64;
	// 82E58A70: 7E068378  mr r6, r16
	ctx.r[6].u64 = ctx.r[16].u64;
	// 82E58A74: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82E58A78: 38BD00F4  addi r5, r29, 0xf4
	ctx.r[5].s64 = ctx.r[29].s64 + 244;
	// 82E58A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E58A80: 4BFFF719  bl 0x82e58198
	ctx.lr = 0x82E58A84;
	sub_82E58198(ctx, base);
	// 82E58A84: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E58A88: 409A05B8  bne cr6, 0x82e59040
	if !ctx.cr[6].eq {
	pc = 0x82E59040; continue 'dispatch;
	}
	// 82E58A8C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E58A90: 839D006C  lwz r28, 0x6c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E58A94: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58A98: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E58A9C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E58AA0: 40980024  bge cr6, 0x82e58ac4
	if !ctx.cr[6].lt {
	pc = 0x82E58AC4; continue 'dispatch;
	}
	// 82E58AA4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58AA8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58AAC: 41980008  blt cr6, 0x82e58ab4
	if ctx.cr[6].lt {
	pc = 0x82E58AB4; continue 'dispatch;
	}
	// 82E58AB0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E58AB4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E58AB8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E58ABC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E58AC0: 4BEFE451  bl 0x82d56f10
	ctx.lr = 0x82E58AC4;
	sub_82D56F10(ctx, base);
	// 82E58AC4: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E58AC8: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82E58ACC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E58AD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E58AD4: 40990024  ble cr6, 0x82e58af8
	if !ctx.cr[6].gt {
	pc = 0x82E58AF8; continue 'dispatch;
	}
	// 82E58AD8: 815D0068  lwz r10, 0x68(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E58ADC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58AE0: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E58AE4: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82E58AE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E58AEC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E58AF0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E58AF4: 4198FFE4  blt cr6, 0x82e58ad8
	if ctx.cr[6].lt {
	pc = 0x82E58AD8; continue 'dispatch;
	}
	// 82E58AF8: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58AFC: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82E58B00: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58B04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58B08: 409A005C  bne cr6, 0x82e58b64
	if !ctx.cr[6].eq {
	pc = 0x82E58B64; continue 'dispatch;
	}
	// 82E58B0C: 81730010  lwz r11, 0x10(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58B10: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E58B14: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E58B18: 409A0014  bne cr6, 0x82e58b2c
	if !ctx.cr[6].eq {
	pc = 0x82E58B2C; continue 'dispatch;
	}
	// 82E58B1C: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58B20: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E58B24: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E58B28: 48000008  b 0x82e58b30
	pc = 0x82E58B30; continue 'dispatch;
	// 82E58B2C: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82E58B30: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58B34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E58B38: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 82E58B3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E58B40: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E58B44: 9A9E008C  stb r20, 0x8c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[20].u8 ) };
	// 82E58B48: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E58B4C: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E58B50: 4BF3C6C9  bl 0x82d95218
	ctx.lr = 0x82E58B54;
	sub_82D95218(ctx, base);
	// 82E58B54: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E58B58: 9B1E008C  stb r24, 0x8c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[24].u8 ) };
	// 82E58B5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E58B60: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E58B64: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58B68: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58B6C: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82E58B70: 7D670E70  srawi r7, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E58B74: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82E58B78: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82E58B7C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E58B80: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58B84: 1CE70070  mulli r7, r7, 0x70
	ctx.r[7].s64 = ctx.r[7].s64 * 112;
	// 82E58B88: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E58B8C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58B90: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82E58B94: 394B0080  addi r10, r11, 0x80
	ctx.r[10].s64 = ctx.r[11].s64 + 128;
	// 82E58B98: 41980010  blt cr6, 0x82e58ba8
	if ctx.cr[6].lt {
	pc = 0x82E58BA8; continue 'dispatch;
	}
	// 82E58B9C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82E58BA0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E58BA4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E58BA8: 3B9F0044  addi r28, r31, 0x44
	ctx.r[28].s64 = ctx.r[31].s64 + 68;
	// 82E58BAC: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E58BB0: 80FC0028  lwz r7, 0x28(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58BB4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82E58BB8: 419A0064  beq cr6, 0x82e58c1c
	if ctx.cr[6].eq {
	pc = 0x82E58C1C; continue 'dispatch;
	}
	// 82E58BBC: 88EB001A  lbz r7, 0x1a(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82E58BC0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82E58BC4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E58BC8: 7D67582E  lwzx r11, r7, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E58BCC: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E58BD0: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E58BD4: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E58BD8: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E58BDC: 7CEA3850  subf r7, r10, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82E58BE0: 7CC93050  subf r6, r9, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[9].s64;
	// 82E58BE4: 7CA82850  subf r5, r8, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[8].s64;
	// 82E58BE8: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82E58BEC: 90CB0010  stw r6, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82E58BF0: 90AB0014  stw r5, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82E58BF4: 817C0028  lwz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58BF8: A0EB0016  lhz r7, 0x16(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 82E58BFC: A0CB0018  lhz r6, 0x18(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E58C00: A0AB0014  lhz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E58C04: 7D293850  subf r9, r9, r7
	ctx.r[9].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 82E58C08: 7D083050  subf r8, r8, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 82E58C0C: 7D4A2850  subf r10, r10, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82E58C10: B12B0016  sth r9, 0x16(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(22 as u32), ctx.r[9].u16 ) };
	// 82E58C14: B10B0018  sth r8, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[8].u16 ) };
	// 82E58C18: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82E58C1C: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58C20: 3AE00004  li r23, 4
	ctx.r[23].s64 = 4;
	// 82E58C24: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58C28: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82E58C2C: A0A40002  lhz r5, 2(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E58C30: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82E58C34: 4BEFC695  bl 0x82d552c8
	ctx.lr = 0x82E58C38;
	sub_82D552C8(ctx, base);
	// 82E58C38: A3DD0024  lhz r30, 0x24(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E58C3C: A0DD002C  lhz r6, 0x2c(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E58C40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E58C44: 88BD0029  lbz r5, 0x29(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(41 as u32) ) } as u64;
	// 82E58C48: 889D0028  lbz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58C4C: 4BF556B5  bl 0x82dae300
	ctx.lr = 0x82E58C50;
	sub_82DAE300(ctx, base);
	// 82E58C50: 397D0030  addi r11, r29, 0x30
	ctx.r[11].s64 = ctx.r[29].s64 + 48;
	// 82E58C54: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82E58C58: B3C30004  sth r30, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 82E58C5C: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58C60: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E58C64: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E58C68: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82E58C6C: E92B0008  ld r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E58C70: F92A0008  std r9, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 82E58C74: E92B0010  ld r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82E58C78: F92A0010  std r9, 0x10(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u64 ) };
	// 82E58C7C: E96B0018  ld r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82E58C80: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82E58C84: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E58C88: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58C8C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E58C90: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58C94: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E58C98: A16B0002  lhz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E58C9C: B16A0010  sth r11, 0x10(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82E58CA0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58CA4: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58CA8: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82E58CAC: 390B0002  addi r8, r11, 2
	ctx.r[8].s64 = ctx.r[11].s64 + 2;
	// 82E58CB0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E58CB4: 38EB0003  addi r7, r11, 3
	ctx.r[7].s64 = ctx.r[11].s64 + 3;
	// 82E58CB8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E58CBC: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58CC0: 1D290070  mulli r9, r9, 0x70
	ctx.r[9].s64 = ctx.r[9].s64 * 112;
	// 82E58CC4: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82E58CC8: 90E1006C  stw r7, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[7].u32 ) };
	// 82E58CCC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E58CD0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E58CD4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E58CD8: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 82E58CDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E58CE0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82E58CE4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E58CE8: 41980018  blt cr6, 0x82e58d00
	if ctx.cr[6].lt {
	pc = 0x82E58D00; continue 'dispatch;
	}
	// 82E58CEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E58CF0: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 82E58CF4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E58CF8: 39670001  addi r11, r7, 1
	ctx.r[11].s64 = ctx.r[7].s64 + 1;
	// 82E58CFC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E58D00: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E58D04: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E58D08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E58D0C: 894B001A  lbz r10, 0x1a(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82E58D10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E58D14: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E58D18: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E58D1C: 806B00B8  lwz r3, 0xb8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E58D20: 4BF40829  bl 0x82d99548
	ctx.lr = 0x82E58D24;
	sub_82D99548(ctx, base);
	// 82E58D24: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58D28: 809D0050  lwz r4, 0x50(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E58D2C: 894B000A  lbz r10, 0xa(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E58D30: 7CAAF1D6  mullw r5, r10, r30
	ctx.r[5].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82E58D34: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E58D38: 554A283E  rotlwi r10, r10, 5
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(5)) as u64;
	// 82E58D3C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E58D40: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82E58D44: 4BEFFFED  bl 0x82d58d30
	ctx.lr = 0x82E58D48;
	sub_82D58D30(ctx, base);
	// 82E58D48: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58D4C: 57C52834  slwi r5, r30, 5
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E58D50: 809D005C  lwz r4, 0x5c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E58D54: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82E58D58: 4BEFFFD9  bl 0x82d58d30
	ctx.lr = 0x82E58D5C;
	sub_82D58D30(ctx, base);
	// 82E58D5C: 89720001  lbz r11, 1(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E58D60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58D64: 419A0048  beq cr6, 0x82e58dac
	if ctx.cr[6].eq {
	pc = 0x82E58DAC; continue 'dispatch;
	}
	// 82E58D68: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58D6C: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82E58D70: A10B0004  lhz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58D74: 892B000A  lbz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E58D78: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E58D7C: A10B0006  lhz r8, 6(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E58D80: 5508283E  rotlwi r8, r8, 5
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(5)) as u64;
	// 82E58D84: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E58D88: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82E58D8C: 419A0020  beq cr6, 0x82e58dac
	if ctx.cr[6].eq {
	pc = 0x82E58DAC; continue 'dispatch;
	}
	// 82E58D90: 930B0008  stw r24, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82E58D94: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E58D98: 811F003C  lwz r8, 0x3c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E58D9C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E58DA0: A1080004  lhz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E58DA4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E58DA8: 4198FFE8  blt cr6, 0x82e58d90
	if ctx.cr[6].lt {
	pc = 0x82E58D90; continue 'dispatch;
	}
	// 82E58DAC: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E58DB0: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82E58DB4: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82E58DB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E58DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E58DC0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E58DC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58DC8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E58DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58DD0: 4E800421  bctrl
	ctx.lr = 0x82E58DD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58DD4: 89720008  lbz r11, 8(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58DD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58DDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E58DE0: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E58DE4: 419A0114  beq cr6, 0x82e58ef8
	if ctx.cr[6].eq {
	pc = 0x82E58EF8; continue 'dispatch;
	}
	// 82E58DE8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E58DEC: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E58DF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E58DF4: 40990104  ble cr6, 0x82e58ef8
	if !ctx.cr[6].gt {
	pc = 0x82E58EF8; continue 'dispatch;
	}
	// 82E58DF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E58DFC: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82E58E00: C3CB0C64  lfs f30, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E58E04: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E58E08: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82E58E0C: 8972000A  lbz r11, 0xa(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(10 as u32) ) } as u64;
	// 82E58E10: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82E58E14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58E18: 7FDA522E  lhzx r30, r26, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E58E1C: D3C13110  stfs f30, 0x3110(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12560 as u32), tmp.u32 ) };
	// 82E58E20: D3E13130  stfs f31, 0x3130(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12592 as u32), tmp.u32 ) };
	// 82E58E24: 930100E4  stw r24, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[24].u32 ) };
	// 82E58E28: D3E13134  stfs f31, 0x3134(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12596 as u32), tmp.u32 ) };
	// 82E58E2C: 419A000C  beq cr6, 0x82e58e38
	if ctx.cr[6].eq {
	pc = 0x82E58E38; continue 'dispatch;
	}
	// 82E58E30: 3B950010  addi r28, r21, 0x10
	ctx.r[28].s64 = ctx.r[21].s64 + 16;
	// 82E58E34: 3B710010  addi r27, r17, 0x10
	ctx.r[27].s64 = ctx.r[17].s64 + 16;
	// 82E58E38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58E3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E58E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E58E44: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58E4C: 4E800421  bctrl
	ctx.lr = 0x82E58E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58E50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58E54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E58E58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E58E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E58E60: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E58E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58E68: 4E800421  bctrl
	ctx.lr = 0x82E58E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58E6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E58E70: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 82E58E74: 80750008  lwz r3, 8(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58E78: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E58E7C: D3E1008C  stfs f31, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82E58E80: 93810070  stw r28, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 82E58E84: 93610074  stw r27, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 82E58E88: 92810078  stw r20, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[20].u32 ) };
	// 82E58E8C: 93010084  stw r24, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[24].u32 ) };
	// 82E58E90: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82E58E94: 93E10094  stw r31, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 82E58E98: 92010098  stw r16, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[16].u32 ) };
	// 82E58E9C: 9141009C  stw r10, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 82E58EA0: 93010090  stw r24, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[24].u32 ) };
	// 82E58EA4: B3C100A0  sth r30, 0xa0(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u16 ) };
	// 82E58EA8: B30100A2  sth r24, 0xa2(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(162 as u32), ctx.r[24].u16 ) };
	// 82E58EAC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E58EB0: 4BF3B429  bl 0x82d942d8
	ctx.lr = 0x82E58EB4;
	sub_82D942D8(ctx, base);
	// 82E58EB4: A17501F4  lhz r11, 0x1f4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(500 as u32) ) } as u64;
	// 82E58EB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58EBC: 419A0010  beq cr6, 0x82e58ecc
	if ctx.cr[6].eq {
	pc = 0x82E58ECC; continue 'dispatch;
	}
	// 82E58EC0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E58EC4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E58EC8: 4BF3A6D1  bl 0x82d93598
	ctx.lr = 0x82E58ECC;
	sub_82D93598(ctx, base);
	// 82E58ECC: A17101F4  lhz r11, 0x1f4(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[17].u32.wrapping_add(500 as u32) ) } as u64;
	// 82E58ED0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58ED4: 419A0010  beq cr6, 0x82e58ee4
	if ctx.cr[6].eq {
	pc = 0x82E58EE4; continue 'dispatch;
	}
	// 82E58ED8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E58EDC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 82E58EE0: 4BF3A6B9  bl 0x82d93598
	ctx.lr = 0x82E58EE4;
	sub_82D93598(ctx, base);
	// 82E58EE4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E58EE8: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E58EEC: 3B5A0002  addi r26, r26, 2
	ctx.r[26].s64 = ctx.r[26].s64 + 2;
	// 82E58EF0: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58EF4: 4198FF10  blt cr6, 0x82e58e04
	if ctx.cr[6].lt {
	pc = 0x82E58E04; continue 'dispatch;
	}
	// 82E58EF8: 89720009  lbz r11, 9(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(9 as u32) ) } as u64;
	// 82E58EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58F00: 419A00FC  beq cr6, 0x82e58ffc
	if ctx.cr[6].eq {
	pc = 0x82E58FFC; continue 'dispatch;
	}
	// 82E58F04: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E58F08: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82E58F0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E58F10: 409900EC  ble cr6, 0x82e58ffc
	if !ctx.cr[6].gt {
	pc = 0x82E58FFC; continue 'dispatch;
	}
	// 82E58F14: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82E58F18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E58F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E58F20: 7FDC5A2E  lhzx r30, r28, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E58F24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58F28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E58F2C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58F34: 4E800421  bctrl
	ctx.lr = 0x82E58F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58F38: 8963000F  lbz r11, 0xf(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 82E58F3C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82E58F40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58F44: 409A00A4  bne cr6, 0x82e58fe8
	if !ctx.cr[6].eq {
	pc = 0x82E58FE8; continue 'dispatch;
	}
	// 82E58F48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58F4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E58F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E58F54: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E58F58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58F5C: 4E800421  bctrl
	ctx.lr = 0x82E58F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58F60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E58F64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E58F68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E58F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E58F70: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E58F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E58F78: 4E800421  bctrl
	ctx.lr = 0x82E58F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E58F7C: 39550010  addi r10, r21, 0x10
	ctx.r[10].s64 = ctx.r[21].s64 + 16;
	// 82E58F80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E58F84: 80750008  lwz r3, 8(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E58F88: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E58F8C: D3E100C4  stfs f31, 0xc4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82E58F90: D3E100C8  stfs f31, 0xc8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82E58F94: 93A100C0  stw r29, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 82E58F98: 928100CC  stw r20, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[20].u32 ) };
	// 82E58F9C: 914100B0  stw r10, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 82E58FA0: 39510010  addi r10, r17, 0x10
	ctx.r[10].s64 = ctx.r[17].s64 + 16;
	// 82E58FA4: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 82E58FA8: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82E58FAC: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 82E58FB0: 914100D0  stw r10, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[10].u32 ) };
	// 82E58FB4: 4BF3B40D  bl 0x82d943c0
	ctx.lr = 0x82E58FB8;
	sub_82D943C0(ctx, base);
	// 82E58FB8: A17501F4  lhz r11, 0x1f4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(500 as u32) ) } as u64;
	// 82E58FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58FC0: 419A0010  beq cr6, 0x82e58fd0
	if ctx.cr[6].eq {
	pc = 0x82E58FD0; continue 'dispatch;
	}
	// 82E58FC4: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E58FC8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E58FCC: 4BF3A6AD  bl 0x82d93678
	ctx.lr = 0x82E58FD0;
	sub_82D93678(ctx, base);
	// 82E58FD0: A17101F4  lhz r11, 0x1f4(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[17].u32.wrapping_add(500 as u32) ) } as u64;
	// 82E58FD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E58FD8: 419A0010  beq cr6, 0x82e58fe8
	if ctx.cr[6].eq {
	pc = 0x82E58FE8; continue 'dispatch;
	}
	// 82E58FDC: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E58FE0: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 82E58FE4: 4BF3A695  bl 0x82d93678
	ctx.lr = 0x82E58FE8;
	sub_82D93678(ctx, base);
	// 82E58FE8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E58FEC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82E58FF0: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82E58FF4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E58FF8: 4198FF20  blt cr6, 0x82e58f18
	if ctx.cr[6].lt {
	pc = 0x82E58F18; continue 'dispatch;
	}
	// 82E58FFC: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82E59000: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82E59004: 4BFAD0F5  bl 0x82e060f8
	ctx.lr = 0x82E59008;
	sub_82E060F8(ctx, base);
	// 82E59008: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E5900C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E59010: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E59014: 409A0018  bne cr6, 0x82e5902c
	if !ctx.cr[6].eq {
	pc = 0x82E5902C; continue 'dispatch;
	}
	// 82E59018: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5901C: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82E59020: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82E59024: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E59028: 4BEFC2A1  bl 0x82d552c8
	ctx.lr = 0x82E5902C;
	sub_82D552C8(ctx, base);
	// 82E5902C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E59030: 38213210  addi r1, r1, 0x3210
	ctx.r[1].s64 = ctx.r[1].s64 + 12816;
	// 82E59034: CBC1FF68  lfd f30, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-152 as u32) ) };
	// 82E59038: CBE1FF70  lfd f31, -0x90(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82E5903C: 4BE503EC  b 0x82ca9428
	sub_82CA9420(ctx, base);
	return;
	// 82E59040: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E59044: 38213210  addi r1, r1, 0x3210
	ctx.r[1].s64 = ctx.r[1].s64 + 12816;
	// 82E59048: CBC1FF68  lfd f30, -0x98(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-152 as u32) ) };
	// 82E5904C: CBE1FF70  lfd f31, -0x90(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82E59050: 4BE503D8  b 0x82ca9428
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59058 size=48
    let mut pc: u32 = 0x82E59058;
    'dispatch: loop {
        match pc {
            0x82E59058 => {
    //   block [0x82E59058..0x82E59088)
	// 82E59058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59064: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59068: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E5906C: 396B73AC  addi r11, r11, 0x73ac
	ctx.r[11].s64 = ctx.r[11].s64 + 29612;
	// 82E59070: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E59074: 4800020D  bl 0x82e59280
	ctx.lr = 0x82E59078;
	sub_82E59280(ctx, base);
	// 82E59078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5907C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59088 size=80
    let mut pc: u32 = 0x82E59088;
    'dispatch: loop {
        match pc {
            0x82E59088 => {
    //   block [0x82E59088..0x82E590D8)
	// 82E59088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5908C: 4BE50381  bl 0x82ca940c
	ctx.lr = 0x82E59090;
	sub_82CA93D0(ctx, base);
	// 82E59090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59094: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E59098: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E5909C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E590A0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E590A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E590A8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E590AC: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E590B0: 48000029  bl 0x82e590d8
	ctx.lr = 0x82E590B4;
	sub_82E590D8(ctx, base);
	// 82E590B4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E590B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E590BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E590C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E590C4: 480001BD  bl 0x82e59280
	ctx.lr = 0x82E590C8;
	sub_82E59280(ctx, base);
	// 82E590C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E590CC: 4B40DE3D  bl 0x82266f08
	ctx.lr = 0x82E590D0;
	sub_82266F08(ctx, base);
	// 82E590D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E590D4: 4BE50388  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E590D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E590D8 size=120
    let mut pc: u32 = 0x82E590D8;
    'dispatch: loop {
        match pc {
            0x82E590D8 => {
    //   block [0x82E590D8..0x82E59150)
	// 82E590D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E590DC: 4BE5032D  bl 0x82ca9408
	ctx.lr = 0x82E590E0;
	sub_82CA93D0(ctx, base);
	// 82E590E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E590E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E590E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E590EC: 396B73B8  addi r11, r11, 0x73b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29624;
	// 82E590F0: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 82E590F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E590F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E590FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E59100: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59104: 4B40DE3D  bl 0x82266f40
	ctx.lr = 0x82E59108;
	sub_82266F40(ctx, base);
	// 82E59108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5910C: 4BF07475  bl 0x82d60580
	ctx.lr = 0x82E59110;
	sub_82D60580(ctx, base);
	// 82E59110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E59114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59118: 4B40CAC1  bl 0x82265bd8
	ctx.lr = 0x82E5911C;
	sub_82265BD8(ctx, base);
	// 82E5911C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E59120: 40990024  ble cr6, 0x82e59144
	if !ctx.cr[6].gt {
	pc = 0x82E59144; continue 'dispatch;
	}
	// 82E59124: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E59128: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5912C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59130: 4B40CAD1  bl 0x82265c00
	ctx.lr = 0x82E59134;
	sub_82265C00(ctx, base);
	// 82E59134: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E59138: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E5913C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E59140: 409AFFE4  bne cr6, 0x82e59124
	if !ctx.cr[6].eq {
	pc = 0x82E59124; continue 'dispatch;
	}
	// 82E59144: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E59148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5914C: 4BE5030C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59150 size=12
    let mut pc: u32 = 0x82E59150;
    'dispatch: loop {
        match pc {
            0x82E59150 => {
    //   block [0x82E59150..0x82E5915C)
	// 82E59150: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E59154: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82E59158: 4BF07530  b 0x82d60688
	sub_82D60688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59160 size=96
    let mut pc: u32 = 0x82E59160;
    'dispatch: loop {
        match pc {
            0x82E59160 => {
    //   block [0x82E59160..0x82E591C0)
	// 82E59160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5916C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5917C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E59180: 4B40DD89  bl 0x82266f08
	ctx.lr = 0x82E59184;
	sub_82266F08(ctx, base);
	// 82E59184: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59188: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5918C: 396B73A0  addi r11, r11, 0x73a0
	ctx.r[11].s64 = ctx.r[11].s64 + 29600;
	// 82E59190: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E59194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59198: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5919C: 419A000C  beq cr6, 0x82e591a8
	if ctx.cr[6].eq {
	pc = 0x82E591A8; continue 'dispatch;
	}
	// 82E591A0: 4B9EC611  bl 0x828457b0
	ctx.lr = 0x82E591A4;
	sub_828457B0(ctx, base);
	// 82E591A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E591A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E591AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E591B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E591B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E591B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E591BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E591C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E591C0 size=188
    let mut pc: u32 = 0x82E591C0;
    'dispatch: loop {
        match pc {
            0x82E591C0 => {
    //   block [0x82E591C0..0x82E5927C)
	// 82E591C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E591C4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82E591C8: 390B6860  addi r8, r11, 0x6860
	ctx.r[8].s64 = ctx.r[11].s64 + 26720;
	// 82E591CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E591D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E591D4: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82E591D8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E591DC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E591E0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E591E4: 39490010  addi r10, r9, 0x10
	ctx.r[10].s64 = ctx.r[9].s64 + 16;
	// 82E591E8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E591EC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E591F0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E591F4: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82E591F8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E591FC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E59200: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82E59204: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 82E59208: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82E5920C: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E59210: B0AA0002  sth r5, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 82E59214: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82E59218: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E5921C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59220: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E59224: 4200FFF8  bdnz 0x82e5921c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E5921C; continue 'dispatch;
	}
	// 82E59228: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E5922C: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E59230: 394300F4  addi r10, r3, 0xf4
	ctx.r[10].s64 = ctx.r[3].s64 + 244;
	// 82E59234: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E59238: 39000103  li r8, 0x103
	ctx.r[8].s64 = 259;
	// 82E5923C: 91230058  stw r9, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E59240: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E59244: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E59248: 91230064  stw r9, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82E5924C: 91630068  stw r11, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E59250: 9163006C  stw r11, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E59254: 91230070  stw r9, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82E59258: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5925C: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E59260: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E59264: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E59268: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E5926C: 912A0014  stw r9, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82E59270: 9103010C  stw r8, 0x10c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), ctx.r[8].u32 ) };
	// 82E59274: 90E30110  stw r7, 0x110(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), ctx.r[7].u32 ) };
	// 82E59278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59280 size=472
    let mut pc: u32 = 0x82E59280;
    'dispatch: loop {
        match pc {
            0x82E59280 => {
    //   block [0x82E59280..0x82E59458)
	// 82E59280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59284: 4BE5016D  bl 0x82ca93f0
	ctx.lr = 0x82E59288;
	sub_82CA93D0(ctx, base);
	// 82E59288: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5928C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E59290: 3B440044  addi r26, r4, 0x44
	ctx.r[26].s64 = ctx.r[4].s64 + 68;
	// 82E59294: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E59298: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82E5929C: 81790084  lwz r11, 0x84(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E592A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E592A4: 91790084  stw r11, 0x84(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E592A8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E592AC: 3AEBFFFF  addi r23, r11, -1
	ctx.r[23].s64 = ctx.r[11].s64 + -1;
	// 82E592B0: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82E592B4: 4198016C  blt cr6, 0x82e59420
	if ctx.cr[6].lt {
	pc = 0x82E59420; continue 'dispatch;
	}
	// 82E592B8: 56F8103A  slwi r24, r23, 2
	ctx.r[24].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82E592BC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E592C0: 7FEBC02E  lwzx r31, r11, r24
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E592C4: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E592C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E592CC: 419A0030  beq cr6, 0x82e592fc
	if ctx.cr[6].eq {
	pc = 0x82E592FC; continue 'dispatch;
	}
	// 82E592D0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E592D4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E592D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E592DC: 4E800421  bctrl
	ctx.lr = 0x82E592E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E592E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E592E4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E592E8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E592EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E592F0: 4E800421  bctrl
	ctx.lr = 0x82E592F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E592F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E592F8: 4800000C  b 0x82e59304
	pc = 0x82E59304; continue 'dispatch;
	// 82E592FC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E59300: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59304: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E59308: 419A0108  beq cr6, 0x82e59410
	if ctx.cr[6].eq {
	pc = 0x82E59410; continue 'dispatch;
	}
	// 82E5930C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E59310: 419A0100  beq cr6, 0x82e59410
	if ctx.cr[6].eq {
	pc = 0x82E59410; continue 'dispatch;
	}
	// 82E59314: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59318: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5931C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E59320: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59328: 4E800421  bctrl
	ctx.lr = 0x82E5932C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5932C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E59330: 409A0024  bne cr6, 0x82e59354
	if !ctx.cr[6].eq {
	pc = 0x82E59354; continue 'dispatch;
	}
	// 82E59334: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59338: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5933C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E59340: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59348: 4E800421  bctrl
	ctx.lr = 0x82E5934C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5934C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E59350: 419A00C0  beq cr6, 0x82e59410
	if ctx.cr[6].eq {
	pc = 0x82E59410; continue 'dispatch;
	}
	// 82E59354: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 82E59358: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82E5935C: 4BFACEE5  bl 0x82e06240
	ctx.lr = 0x82E59360;
	sub_82E06240(ctx, base);
	// 82E59360: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E59364: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E59368: 419A00A8  beq cr6, 0x82e59410
	if ctx.cr[6].eq {
	pc = 0x82E59410; continue 'dispatch;
	}
	// 82E5936C: 817F010C  lwz r11, 0x10c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 82E59370: 2B0B0103  cmplwi cr6, r11, 0x103
	ctx.cr[6].compare_u32(ctx.r[11].u32, 259 as u32, &mut ctx.xer);
	// 82E59374: 419A0010  beq cr6, 0x82e59384
	if ctx.cr[6].eq {
	pc = 0x82E59384; continue 'dispatch;
	}
	// 82E59378: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5937C: 8079006C  lwz r3, 0x6c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E59380: 4800C769  bl 0x82e65ae8
	ctx.lr = 0x82E59384;
	sub_82E65AE8(ctx, base);
	// 82E59384: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82E59388: 80F9006C  lwz r7, 0x6c(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5938C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E59390: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E59394: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E59398: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E5939C: 4BFFF585  bl 0x82e58920
	ctx.lr = 0x82E593A0;
	sub_82E58920(ctx, base);
	// 82E593A0: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E593A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E593A8: 419A0068  beq cr6, 0x82e59410
	if ctx.cr[6].eq {
	pc = 0x82E59410; continue 'dispatch;
	}
	// 82E593AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E593B0: 419A0060  beq cr6, 0x82e59410
	if ctx.cr[6].eq {
	pc = 0x82E59410; continue 'dispatch;
	}
	// 82E593B4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E593B8: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E593BC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E593C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E593C4: 419A0030  beq cr6, 0x82e593f4
	if ctx.cr[6].eq {
	pc = 0x82E593F4; continue 'dispatch;
	}
	// 82E593C8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E593CC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E593D0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E593D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E593D8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E593DC: 409A0018  bne cr6, 0x82e593f4
	if !ctx.cr[6].eq {
	pc = 0x82E593F4; continue 'dispatch;
	}
	// 82E593E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E593E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E593E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E593EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E593F0: 4E800421  bctrl
	ctx.lr = 0x82E593F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E593F4: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E593F8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E593FC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E59400: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E59404: 915A0004  stw r10, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E59408: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5940C: 7D58592E  stwx r10, r24, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E59410: 3AF7FFFF  addi r23, r23, -1
	ctx.r[23].s64 = ctx.r[23].s64 + -1;
	// 82E59414: 3B18FFFC  addi r24, r24, -4
	ctx.r[24].s64 = ctx.r[24].s64 + -4;
	// 82E59418: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82E5941C: 4098FEA0  bge cr6, 0x82e592bc
	if !ctx.cr[6].lt {
	pc = 0x82E592BC; continue 'dispatch;
	}
	// 82E59420: 81790084  lwz r11, 0x84(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E59424: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59428: 91790084  stw r11, 0x84(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E5942C: 40820024  bne 0x82e59450
	if !ctx.cr[0].eq {
	pc = 0x82E59450; continue 'dispatch;
	}
	// 82E59430: 81790080  lwz r11, 0x80(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E59434: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59438: 419A0018  beq cr6, 0x82e59450
	if ctx.cr[6].eq {
	pc = 0x82E59450; continue 'dispatch;
	}
	// 82E5943C: 8979008C  lbz r11, 0x8c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E59440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59444: 409A000C  bne cr6, 0x82e59450
	if !ctx.cr[6].eq {
	pc = 0x82E59450; continue 'dispatch;
	}
	// 82E59448: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E5944C: 4BF1E5FD  bl 0x82d77a48
	ctx.lr = 0x82E59450;
	sub_82D77A48(ctx, base);
	// 82E59450: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E59454: 4BE4FFEC  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59458 size=100
    let mut pc: u32 = 0x82E59458;
    'dispatch: loop {
        match pc {
            0x82E59458 => {
    //   block [0x82E59458..0x82E594BC)
	// 82E59458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5945C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E59464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5946C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59470: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E59474: 4800C90D  bl 0x82e65d80
	ctx.lr = 0x82E59478;
	sub_82E65D80(ctx, base);
	// 82E59478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5947C: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82E59480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59484: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E59488: 396B7430  addi r11, r11, 0x7430
	ctx.r[11].s64 = ctx.r[11].s64 + 29744;
	// 82E5948C: 394A7410  addi r10, r10, 0x7410
	ctx.r[10].s64 = ctx.r[10].s64 + 29712;
	// 82E59490: 39297400  addi r9, r9, 0x7400
	ctx.r[9].s64 = ctx.r[9].s64 + 29696;
	// 82E59494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59498: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5949C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E594A0: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E594A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E594A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E594AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E594B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E594B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E594B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E594C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E594C0 size=140
    let mut pc: u32 = 0x82E594C0;
    'dispatch: loop {
        match pc {
            0x82E594C0 => {
    //   block [0x82E594C0..0x82E5954C)
	// 82E594C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E594C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E594C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E594CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E594D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E594D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E594D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E594DC: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E594E0: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82E594E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E594E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E594EC: 4BEFBD5D  bl 0x82d55248
	ctx.lr = 0x82E594F0;
	sub_82D55248(ctx, base);
	// 82E594F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E594F4: 3960002C  li r11, 0x2c
	ctx.r[11].s64 = 44;
	// 82E594F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E594FC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E59500: 4800C881  bl 0x82e65d80
	ctx.lr = 0x82E59504;
	sub_82E65D80(ctx, base);
	// 82E59504: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5950C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E59510: 396B7430  addi r11, r11, 0x7430
	ctx.r[11].s64 = ctx.r[11].s64 + 29744;
	// 82E59514: 394A7410  addi r10, r10, 0x7410
	ctx.r[10].s64 = ctx.r[10].s64 + 29712;
	// 82E59518: 39297400  addi r9, r9, 0x7400
	ctx.r[9].s64 = ctx.r[9].s64 + 29696;
	// 82E5951C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E59520: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E59524: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59528: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5952C: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E59530: 991F0028  stb r8, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82E59534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E59538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5953C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E59544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E59548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59550 size=140
    let mut pc: u32 = 0x82E59550;
    'dispatch: loop {
        match pc {
            0x82E59550 => {
    //   block [0x82E59550..0x82E595DC)
	// 82E59550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5955C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59564: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59568: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5956C: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E59570: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82E59574: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E59578: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5957C: 4BEFBCCD  bl 0x82d55248
	ctx.lr = 0x82E59580;
	sub_82D55248(ctx, base);
	// 82E59580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59584: 3960002C  li r11, 0x2c
	ctx.r[11].s64 = 44;
	// 82E59588: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5958C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E59590: 4800C7F1  bl 0x82e65d80
	ctx.lr = 0x82E59594;
	sub_82E65D80(ctx, base);
	// 82E59594: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59598: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5959C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E595A0: 396B7430  addi r11, r11, 0x7430
	ctx.r[11].s64 = ctx.r[11].s64 + 29744;
	// 82E595A4: 394A7410  addi r10, r10, 0x7410
	ctx.r[10].s64 = ctx.r[10].s64 + 29712;
	// 82E595A8: 39297400  addi r9, r9, 0x7400
	ctx.r[9].s64 = ctx.r[9].s64 + 29696;
	// 82E595AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E595B0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E595B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E595B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E595BC: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E595C0: 991F0028  stb r8, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82E595C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E595C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E595CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E595D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E595D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E595D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E595E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E595E0 size=112
    let mut pc: u32 = 0x82E595E0;
    'dispatch: loop {
        match pc {
            0x82E595E0 => {
    //   block [0x82E595E0..0x82E59650)
	// 82E595E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E595E4: 4BE4FE25  bl 0x82ca9408
	ctx.lr = 0x82E595E8;
	sub_82CA93D0(ctx, base);
	// 82E595E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E595EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E595F0: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E595F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E595F8: 419A0050  beq cr6, 0x82e59648
	if ctx.cr[6].eq {
	pc = 0x82E59648; continue 'dispatch;
	}
	// 82E595FC: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59600: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E59604: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E59608: 40990040  ble cr6, 0x82e59648
	if !ctx.cr[6].gt {
	pc = 0x82E59648; continue 'dispatch;
	}
	// 82E5960C: 3BBC0018  addi r29, r28, 0x18
	ctx.r[29].s64 = ctx.r[28].s64 + 24;
	// 82E59610: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E59614: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59618: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5961C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E59620: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59624: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E59628: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E5962C: 4E800421  bctrl
	ctx.lr = 0x82E59630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59630: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59634: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E59638: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5963C: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59640: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E59644: 4198FFD0  blt cr6, 0x82e59614
	if ctx.cr[6].lt {
	pc = 0x82E59614; continue 'dispatch;
	}
	// 82E59648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5964C: 4BE4FE0C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59650 size=40
    let mut pc: u32 = 0x82E59650;
    'dispatch: loop {
        match pc {
            0x82E59650 => {
    //   block [0x82E59650..0x82E59678)
	// 82E59650: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59658: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5965C: 396B7430  addi r11, r11, 0x7430
	ctx.r[11].s64 = ctx.r[11].s64 + 29744;
	// 82E59660: 394A7410  addi r10, r10, 0x7410
	ctx.r[10].s64 = ctx.r[10].s64 + 29712;
	// 82E59664: 39297400  addi r9, r9, 0x7400
	ctx.r[9].s64 = ctx.r[9].s64 + 29696;
	// 82E59668: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5966C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E59670: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E59674: 4800C634  b 0x82e65ca8
	sub_82E65CA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59678 size=104
    let mut pc: u32 = 0x82E59678;
    'dispatch: loop {
        match pc {
            0x82E59678 => {
    //   block [0x82E59678..0x82E596E0)
	// 82E59678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59688: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 82E5968C: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E59690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59694: 38AB94C0  addi r5, r11, -0x6b40
	ctx.r[5].s64 = ctx.r[11].s64 + -27456;
	// 82E59698: 388A73C0  addi r4, r10, 0x73c0
	ctx.r[4].s64 = ctx.r[10].s64 + 29632;
	// 82E5969C: 807FB068  lwz r3, -0x4f98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E596A0: 4BFDF5F9  bl 0x82e38c98
	ctx.lr = 0x82E596A4;
	sub_82E38C98(ctx, base);
	// 82E596A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E596A8: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E596AC: 388A73E0  addi r4, r10, 0x73e0
	ctx.r[4].s64 = ctx.r[10].s64 + 29664;
	// 82E596B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 82E596B4: 38AB9550  addi r5, r11, -0x6ab0
	ctx.r[5].s64 = ctx.r[11].s64 + -27312;
	// 82E596B8: 906AB5AC  stw r3, -0x4a54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19028 as u32), ctx.r[3].u32 ) };
	// 82E596BC: 807FB068  lwz r3, -0x4f98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E596C0: 4BFDF5D9  bl 0x82e38c98
	ctx.lr = 0x82E596C4;
	sub_82E38C98(ctx, base);
	// 82E596C4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E596C8: 906BB5A8  stw r3, -0x4a58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19032 as u32), ctx.r[3].u32 ) };
	// 82E596CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E596D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E596D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E596D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E596DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E596E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E596E0 size=572
    let mut pc: u32 = 0x82E596E0;
    'dispatch: loop {
        match pc {
            0x82E596E0 => {
    //   block [0x82E596E0..0x82E5991C)
	// 82E596E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E596E4: 4BE4FD15  bl 0x82ca93f8
	ctx.lr = 0x82E596E8;
	sub_82CA93D0(ctx, base);
	// 82E596E8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E596EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E596F0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E596F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E596F8: 419A01DC  beq cr6, 0x82e598d4
	if ctx.cr[6].eq {
	pc = 0x82E598D4; continue 'dispatch;
	}
	// 82E596FC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E59700: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E59704: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 82E59708: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E5970C: 60849000  ori r4, r4, 0x9000
	ctx.r[4].u64 = ctx.r[4].u64 | 36864;
	// 82E59710: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E59714: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82E59718: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82E5971C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E59720: 4BEFD7F1  bl 0x82d56f10
	ctx.lr = 0x82E59724;
	sub_82D56F10(ctx, base);
	// 82E59724: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E59728: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E5972C: 3B0B3BA8  addi r24, r11, 0x3ba8
	ctx.r[24].s64 = ctx.r[11].s64 + 15272;
	// 82E59730: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 82E59734: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E59738: B3C10076  sth r30, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[30].u16 ) };
	// 82E5973C: 93010070  stw r24, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[24].u32 ) };
	// 82E59740: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E59744: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E59748: 93C10080  stw r30, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82E5974C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E59750: 4BEFEE49  bl 0x82d58598
	ctx.lr = 0x82E59754;
	sub_82D58598(ctx, base);
	// 82E59754: 897F0020  lbz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E59758: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5975C: 419A0010  beq cr6, 0x82e5976c
	if ctx.cr[6].eq {
	pc = 0x82E5976C; continue 'dispatch;
	}
	// 82E59760: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E59764: 3BAB6A98  addi r29, r11, 0x6a98
	ctx.r[29].s64 = ctx.r[11].s64 + 27288;
	// 82E59768: 4800000C  b 0x82e59774
	pc = 0x82E59774; continue 'dispatch;
	// 82E5976C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82E59770: 3BAB7D58  addi r29, r11, 0x7d58
	ctx.r[29].s64 = ctx.r[11].s64 + 32088;
	// 82E59774: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59778: 7F79DB78  mr r25, r27
	ctx.r[25].u64 = ctx.r[27].u64;
	// 82E5977C: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59780: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59784: 409900F4  ble cr6, 0x82e59878
	if !ctx.cr[6].gt {
	pc = 0x82E59878; continue 'dispatch;
	}
	// 82E59788: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 82E5978C: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 82E59790: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82E59794: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E59798: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5979C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E597A0: 40980024  bge cr6, 0x82e597c4
	if !ctx.cr[6].lt {
	pc = 0x82E597C4; continue 'dispatch;
	}
	// 82E597A4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E597A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E597AC: 41990008  bgt cr6, 0x82e597b4
	if ctx.cr[6].gt {
	pc = 0x82E597B4; continue 'dispatch;
	}
	// 82E597B0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82E597B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E597B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E597BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E597C0: 4BEFD751  bl 0x82d56f10
	ctx.lr = 0x82E597C4;
	sub_82D56F10(ctx, base);
	// 82E597C4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E597C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E597CC: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82E597D0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E597D4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E597D8: 88DA0000  lbz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E597DC: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82E597E0: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E597E4: 7C8BE02E  lwzx r4, r11, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E597E8: 4BFF2C99  bl 0x82e4c480
	ctx.lr = 0x82E597EC;
	sub_82E4C480(ctx, base);
	// 82E597EC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E597F0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E597F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E597F8: 41980068  blt cr6, 0x82e59860
	if ctx.cr[6].lt {
	pc = 0x82E59860; continue 'dispatch;
	}
	// 82E597FC: 388B0009  addi r4, r11, 9
	ctx.r[4].s64 = ctx.r[11].s64 + 9;
	// 82E59800: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59804: 4BF049C5  bl 0x82d5e1c8
	ctx.lr = 0x82E59808;
	sub_82D5E1C8(ctx, base);
	// 82E59808: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 82E5980C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59810: 4BF043D9  bl 0x82d5dbe8
	ctx.lr = 0x82E59814;
	sub_82D5DBE8(ctx, base);
	// 82E59814: 889D0000  lbz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59818: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5981C: 4BF043CD  bl 0x82d5dbe8
	ctx.lr = 0x82E59820;
	sub_82D5DBE8(ctx, base);
	// 82E59820: 889D0001  lbz r4, 1(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E59824: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59828: 4BF043C1  bl 0x82d5dbe8
	ctx.lr = 0x82E5982C;
	sub_82D5DBE8(ctx, base);
	// 82E5982C: 889D0002  lbz r4, 2(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E59830: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59834: 4BF043B5  bl 0x82d5dbe8
	ctx.lr = 0x82E59838;
	sub_82D5DBE8(ctx, base);
	// 82E59838: 889D0003  lbz r4, 3(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(3 as u32) ) } as u64;
	// 82E5983C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59840: 4BF043A9  bl 0x82d5dbe8
	ctx.lr = 0x82E59844;
	sub_82D5DBE8(ctx, base);
	// 82E59844: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E59848: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5984C: 4BF0494D  bl 0x82d5e198
	ctx.lr = 0x82E59850;
	sub_82D5E198(ctx, base);
	// 82E59850: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E59854: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E59858: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5985C: 4BF045AD  bl 0x82d5de08
	ctx.lr = 0x82E59860;
	sub_82D5DE08(ctx, base);
	// 82E59860: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59864: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E59868: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E5986C: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59870: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E59874: 4198FF20  blt cr6, 0x82e59794
	if ctx.cr[6].lt {
	pc = 0x82E59794; continue 'dispatch;
	}
	// 82E59878: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E5987C: 93010070  stw r24, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[24].u32 ) };
	// 82E59880: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59884: 409A0018  bne cr6, 0x82e5989c
	if !ctx.cr[6].eq {
	pc = 0x82E5989C; continue 'dispatch;
	}
	// 82E59888: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E5988C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E59890: 419A000C  beq cr6, 0x82e5989c
	if ctx.cr[6].eq {
	pc = 0x82E5989C; continue 'dispatch;
	}
	// 82E59894: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E59898: 4BFDE389  bl 0x82e37c20
	ctx.lr = 0x82E5989C;
	sub_82E37C20(ctx, base);
	// 82E5989C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E598A0: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E598A4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E598A8: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E598AC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E598B0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E598B4: 409A0020  bne cr6, 0x82e598d4
	if !ctx.cr[6].eq {
	pc = 0x82E598D4; continue 'dispatch;
	}
	// 82E598B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E598BC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E598C0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E598C4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E598C8: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E598CC: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E598D0: 4BEFB9F9  bl 0x82d552c8
	ctx.lr = 0x82E598D4;
	sub_82D552C8(ctx, base);
	// 82E598D4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E598D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E598DC: 419A0038  beq cr6, 0x82e59914
	if ctx.cr[6].eq {
	pc = 0x82E59914; continue 'dispatch;
	}
	// 82E598E0: 897F0020  lbz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E598E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E598E8: 419A0010  beq cr6, 0x82e598f8
	if ctx.cr[6].eq {
	pc = 0x82E598F8; continue 'dispatch;
	}
	// 82E598EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E598F0: 808BB5A8  lwz r4, -0x4a58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19032 as u32) ) } as u64;
	// 82E598F4: 4800000C  b 0x82e59900
	pc = 0x82E59900; continue 'dispatch;
	// 82E598F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E598FC: 808BB5AC  lwz r4, -0x4a54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19028 as u32) ) } as u64;
	// 82E59900: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E59904: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59908: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5990C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59910: 4E800421  bctrl
	ctx.lr = 0x82E59914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59914: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E59918: 4BE4FB30  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59920 size=24
    let mut pc: u32 = 0x82E59920;
    'dispatch: loop {
        match pc {
            0x82E59920 => {
    //   block [0x82E59920..0x82E59938)
	// 82E59920: 89630020  lbz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E59924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59928: 419A0010  beq cr6, 0x82e59938
	if ctx.cr[6].eq {
		sub_82E59938(ctx, base);
		return;
	}
	// 82E5992C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E59930: 806BB5A8  lwz r3, -0x4a58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19032 as u32) ) } as u64;
	// 82E59934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59938 size=12
    let mut pc: u32 = 0x82E59938;
    'dispatch: loop {
        match pc {
            0x82E59938 => {
    //   block [0x82E59938..0x82E59944)
	// 82E59938: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5993C: 806BB5AC  lwz r3, -0x4a54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19028 as u32) ) } as u64;
	// 82E59940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59948 size=8
    let mut pc: u32 = 0x82E59948;
    'dispatch: loop {
        match pc {
            0x82E59948 => {
    //   block [0x82E59948..0x82E59950)
	// 82E59948: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5994C: 4800000C  b 0x82e59958
	sub_82E59958(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59950 size=8
    let mut pc: u32 = 0x82E59950;
    'dispatch: loop {
        match pc {
            0x82E59950 => {
    //   block [0x82E59950..0x82E59958)
	// 82E59950: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E59954: 48000004  b 0x82e59958
	sub_82E59958(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59958 size=136
    let mut pc: u32 = 0x82E59958;
    'dispatch: loop {
        match pc {
            0x82E59958 => {
    //   block [0x82E59958..0x82E599E0)
	// 82E59958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5995C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59960: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E59964: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5996C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59974: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E59978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5997C: 396B7430  addi r11, r11, 0x7430
	ctx.r[11].s64 = ctx.r[11].s64 + 29744;
	// 82E59980: 394A7410  addi r10, r10, 0x7410
	ctx.r[10].s64 = ctx.r[10].s64 + 29712;
	// 82E59984: 39297400  addi r9, r9, 0x7400
	ctx.r[9].s64 = ctx.r[9].s64 + 29696;
	// 82E59988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5998C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59990: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E59994: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E59998: 4800C311  bl 0x82e65ca8
	ctx.lr = 0x82E5999C;
	sub_82E65CA8(ctx, base);
	// 82E5999C: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E599A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E599A4: 419A0020  beq cr6, 0x82e599c4
	if ctx.cr[6].eq {
	pc = 0x82E599C4; continue 'dispatch;
	}
	// 82E599A8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E599AC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E599B0: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E599B4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E599B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E599BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E599C0: 4BEFB909  bl 0x82d552c8
	ctx.lr = 0x82E599C4;
	sub_82D552C8(ctx, base);
	// 82E599C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E599C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E599CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E599D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E599D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E599D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E599DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E599E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E599E0 size=100
    let mut pc: u32 = 0x82E599E0;
    'dispatch: loop {
        match pc {
            0x82E599E0 => {
    //   block [0x82E599E0..0x82E59A44)
	// 82E599E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E599E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E599E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E599EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E599F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E599F4: 4800C38D  bl 0x82e65d80
	ctx.lr = 0x82E599F8;
	sub_82E65D80(ctx, base);
	// 82E599F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E599FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59A00: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E59A04: 396B7480  addi r11, r11, 0x7480
	ctx.r[11].s64 = ctx.r[11].s64 + 29824;
	// 82E59A08: 394A7460  addi r10, r10, 0x7460
	ctx.r[10].s64 = ctx.r[10].s64 + 29792;
	// 82E59A0C: 39297450  addi r9, r9, 0x7450
	ctx.r[9].s64 = ctx.r[9].s64 + 29776;
	// 82E59A10: 3C800007  lis r4, 7
	ctx.r[4].s64 = 458752;
	// 82E59A14: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82E59A18: 6084A120  ori r4, r4, 0xa120
	ctx.r[4].u64 = ctx.r[4].u64 | 41248;
	// 82E59A1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59A20: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E59A24: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E59A28: 48279609  bl 0x830d3030
	ctx.lr = 0x82E59A2C;
	sub_830D3030(ctx, base);
	// 82E59A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E59A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E59A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59A48 size=96
    let mut pc: u32 = 0x82E59A48;
    'dispatch: loop {
        match pc {
            0x82E59A48 => {
    //   block [0x82E59A48..0x82E59AA8)
	// 82E59A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59A58: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59A5C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E59A60: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E59A64: 38800054  li r4, 0x54
	ctx.r[4].s64 = 84;
	// 82E59A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59A6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E59A70: 4BEFB7D9  bl 0x82d55248
	ctx.lr = 0x82E59A74;
	sub_82D55248(ctx, base);
	// 82E59A74: 39600054  li r11, 0x54
	ctx.r[11].s64 = 84;
	// 82E59A78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E59A7C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E59A80: 4BFFFF61  bl 0x82e599e0
	ctx.lr = 0x82E59A84;
	sub_82E599E0(ctx, base);
	// 82E59A84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E59A88: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E59A8C: 409A0008  bne cr6, 0x82e59a94
	if !ctx.cr[6].eq {
	pc = 0x82E59A94; continue 'dispatch;
	}
	// 82E59A90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E59A94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E59A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59AA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E59AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59AA8 size=112
    let mut pc: u32 = 0x82E59AA8;
    'dispatch: loop {
        match pc {
            0x82E59AA8 => {
    //   block [0x82E59AA8..0x82E59B18)
	// 82E59AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59AAC: 4BE4F95D  bl 0x82ca9408
	ctx.lr = 0x82E59AB0;
	sub_82CA93D0(ctx, base);
	// 82E59AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59AB4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E59AB8: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59ABC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59AC0: 419A0050  beq cr6, 0x82e59b10
	if ctx.cr[6].eq {
	pc = 0x82E59B10; continue 'dispatch;
	}
	// 82E59AC4: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59AC8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E59ACC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E59AD0: 40990040  ble cr6, 0x82e59b10
	if !ctx.cr[6].gt {
	pc = 0x82E59B10; continue 'dispatch;
	}
	// 82E59AD4: 3BBC0018  addi r29, r28, 0x18
	ctx.r[29].s64 = ctx.r[28].s64 + 24;
	// 82E59AD8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E59ADC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59AE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E59AE4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E59AE8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59AEC: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E59AF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E59AF4: 4E800421  bctrl
	ctx.lr = 0x82E59AF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59AF8: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59AFC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E59B00: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E59B04: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59B08: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E59B0C: 4198FFD0  blt cr6, 0x82e59adc
	if ctx.cr[6].lt {
	pc = 0x82E59ADC; continue 'dispatch;
	}
	// 82E59B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E59B14: 4BE4F944  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59B18 size=92
    let mut pc: u32 = 0x82E59B18;
    'dispatch: loop {
        match pc {
            0x82E59B18 => {
    //   block [0x82E59B18..0x82E59B74)
	// 82E59B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59B20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59B24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59B28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59B2C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59B30: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E59B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59B38: 396B7480  addi r11, r11, 0x7480
	ctx.r[11].s64 = ctx.r[11].s64 + 29824;
	// 82E59B3C: 394A7460  addi r10, r10, 0x7460
	ctx.r[10].s64 = ctx.r[10].s64 + 29792;
	// 82E59B40: 39297450  addi r9, r9, 0x7450
	ctx.r[9].s64 = ctx.r[9].s64 + 29776;
	// 82E59B44: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82E59B48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59B4C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E59B50: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E59B54: 4827958D  bl 0x830d30e0
	ctx.lr = 0x82E59B58;
	sub_830D30E0(ctx, base);
	// 82E59B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59B5C: 4800C14D  bl 0x82e65ca8
	ctx.lr = 0x82E59B60;
	sub_82E65CA8(ctx, base);
	// 82E59B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E59B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59B6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E59B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59B78 size=224
    let mut pc: u32 = 0x82E59B78;
    'dispatch: loop {
        match pc {
            0x82E59B78 => {
    //   block [0x82E59B78..0x82E59C58)
	// 82E59B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59B7C: 4BE4F885  bl 0x82ca9400
	ctx.lr = 0x82E59B80;
	sub_82CA93D0(ctx, base);
	// 82E59B80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59B84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E59B88: 3B5D0020  addi r26, r29, 0x20
	ctx.r[26].s64 = ctx.r[29].s64 + 32;
	// 82E59B8C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E59B90: 48279469  bl 0x830d2ff8
	ctx.lr = 0x82E59B94;
	sub_830D2FF8(ctx, base);
	// 82E59B94: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82E59B98: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E59B9C: 48279415  bl 0x830d2fb0
	ctx.lr = 0x82E59BA0;
	sub_830D2FB0(ctx, base);
	// 82E59BA0: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59BA4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E59BA8: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59BAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E59BB0: 40990050  ble cr6, 0x82e59c00
	if !ctx.cr[6].gt {
	pc = 0x82E59C00; continue 'dispatch;
	}
	// 82E59BB4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82E59BB8: 3B9D0028  addi r28, r29, 0x28
	ctx.r[28].s64 = ctx.r[29].s64 + 40;
	// 82E59BBC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E59BC0: 3B6A9C30  addi r27, r10, -0x63d0
	ctx.r[27].s64 = ctx.r[10].s64 + -25552;
	// 82E59BC4: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59BC8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E59BCC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E59BD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E59BD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E59BD8: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59BDC: 7CCBF82E  lwzx r6, r11, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E59BE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E59BE4: 4E800421  bctrl
	ctx.lr = 0x82E59BE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59BE8: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59BEC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E59BF0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E59BF4: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59BF8: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E59BFC: 4198FFC8  blt cr6, 0x82e59bc4
	if ctx.cr[6].lt {
	pc = 0x82E59BC4; continue 'dispatch;
	}
	// 82E59C00: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E59C04: 482793ED  bl 0x830d2ff0
	ctx.lr = 0x82E59C08;
	sub_830D2FF0(ctx, base);
	// 82E59C08: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E59C0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59C10: 40990040  ble cr6, 0x82e59c50
	if !ctx.cr[6].gt {
	pc = 0x82E59C50; continue 'dispatch;
	}
	// 82E59C14: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E59C18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59C1C: 419A0034  beq cr6, 0x82e59c50
	if ctx.cr[6].eq {
	pc = 0x82E59C50; continue 'dispatch;
	}
	// 82E59C20: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E59C24: 80BD0034  lwz r5, 0x34(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E59C28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59C2C: 4099000C  ble cr6, 0x82e59c38
	if !ctx.cr[6].gt {
	pc = 0x82E59C38; continue 'dispatch;
	}
	// 82E59C30: 809A0010  lwz r4, 0x10(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E59C34: 48000008  b 0x82e59c3c
	pc = 0x82E59C3C; continue 'dispatch;
	// 82E59C38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E59C3C: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E59C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59C44: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E59C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59C4C: 4E800421  bctrl
	ctx.lr = 0x82E59C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59C50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E59C54: 4BE4F7FC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59C58 size=64
    let mut pc: u32 = 0x82E59C58;
    'dispatch: loop {
        match pc {
            0x82E59C58 => {
    //   block [0x82E59C58..0x82E59C98)
	// 82E59C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59C64: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E59C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59C6C: 38AB9A48  addi r5, r11, -0x65b8
	ctx.r[5].s64 = ctx.r[11].s64 + -26040;
	// 82E59C70: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E59C74: 388A743C  addi r4, r10, 0x743c
	ctx.r[4].s64 = ctx.r[10].s64 + 29756;
	// 82E59C78: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E59C7C: 4BFDF01D  bl 0x82e38c98
	ctx.lr = 0x82E59C80;
	sub_82E38C98(ctx, base);
	// 82E59C80: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E59C84: 906BB5B0  stw r3, -0x4a50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19024 as u32), ctx.r[3].u32 ) };
	// 82E59C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E59C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59C98 size=12
    let mut pc: u32 = 0x82E59C98;
    'dispatch: loop {
        match pc {
            0x82E59C98 => {
    //   block [0x82E59C98..0x82E59CA4)
	// 82E59C98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E59C9C: 806BB5B0  lwz r3, -0x4a50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19024 as u32) ) } as u64;
	// 82E59CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59CA8 size=8
    let mut pc: u32 = 0x82E59CA8;
    'dispatch: loop {
        match pc {
            0x82E59CA8 => {
    //   block [0x82E59CA8..0x82E59CB0)
	// 82E59CA8: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E59CAC: 4800000C  b 0x82e59cb8
	sub_82E59CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59CB0 size=8
    let mut pc: u32 = 0x82E59CB0;
    'dispatch: loop {
        match pc {
            0x82E59CB0 => {
    //   block [0x82E59CB0..0x82E59CB8)
	// 82E59CB0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E59CB4: 48000004  b 0x82e59cb8
	sub_82E59CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59CB8 size=148
    let mut pc: u32 = 0x82E59CB8;
    'dispatch: loop {
        match pc {
            0x82E59CB8 => {
    //   block [0x82E59CB8..0x82E59D4C)
	// 82E59CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59CC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E59CC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59CC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59CCC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59CD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E59CD4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E59CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59CDC: 396B7480  addi r11, r11, 0x7480
	ctx.r[11].s64 = ctx.r[11].s64 + 29824;
	// 82E59CE0: 394A7460  addi r10, r10, 0x7460
	ctx.r[10].s64 = ctx.r[10].s64 + 29792;
	// 82E59CE4: 39297450  addi r9, r9, 0x7450
	ctx.r[9].s64 = ctx.r[9].s64 + 29776;
	// 82E59CE8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82E59CEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E59CF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59CF4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E59CF8: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E59CFC: 482793E5  bl 0x830d30e0
	ctx.lr = 0x82E59D00;
	sub_830D30E0(ctx, base);
	// 82E59D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59D04: 4800BFA5  bl 0x82e65ca8
	ctx.lr = 0x82E59D08;
	sub_82E65CA8(ctx, base);
	// 82E59D08: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E59D0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59D10: 419A0020  beq cr6, 0x82e59d30
	if ctx.cr[6].eq {
	pc = 0x82E59D30; continue 'dispatch;
	}
	// 82E59D14: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59D18: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E59D1C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E59D20: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59D24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E59D28: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E59D2C: 4BEFB59D  bl 0x82d552c8
	ctx.lr = 0x82E59D30;
	sub_82D552C8(ctx, base);
	// 82E59D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59D34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E59D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59D40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E59D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E59D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59D50 size=72
    let mut pc: u32 = 0x82E59D50;
    'dispatch: loop {
        match pc {
            0x82E59D50 => {
    //   block [0x82E59D50..0x82E59D98)
	// 82E59D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59D60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59D64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59D68: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E59D6C: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E59D70: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E59D74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E59D78: 419A000C  beq cr6, 0x82e59d84
	if ctx.cr[6].eq {
	pc = 0x82E59D84; continue 'dispatch;
	}
	// 82E59D7C: 4B9EBA35  bl 0x828457b0
	ctx.lr = 0x82E59D80;
	sub_828457B0(ctx, base);
	// 82E59D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59D84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E59D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E59D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E59D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E59D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59D98 size=76
    let mut pc: u32 = 0x82E59D98;
    'dispatch: loop {
        match pc {
            0x82E59D98 => {
    //   block [0x82E59D98..0x82E59DE4)
	// 82E59D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59D9C: 4BE4F671  bl 0x82ca940c
	ctx.lr = 0x82E59DA0;
	sub_82CA93D0(ctx, base);
	// 82E59DA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59DA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59DA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E59DAC: 37DFFFE0  addic. r30, r31, -0x20
	ctx.xer.ca = (ctx.r[31].u32 > (!(-32 as u32)));
	ctx.r[30].s64 = ctx.r[31].s64 + -32;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E59DB0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E59DB4: 40820008  bne 0x82e59dbc
	if !ctx.cr[0].eq {
	pc = 0x82E59DBC; continue 'dispatch;
	}
	// 82E59DB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E59DBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E59DC0: 4BF1E309  bl 0x82d780c8
	ctx.lr = 0x82E59DC4;
	sub_82D780C8(ctx, base);
	// 82E59DC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E59DC8: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82E59DCC: 409A0008  bne cr6, 0x82e59dd4
	if !ctx.cr[6].eq {
	pc = 0x82E59DD4; continue 'dispatch;
	}
	// 82E59DD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E59DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E59DD8: 4BF1E4A1  bl 0x82d78278
	ctx.lr = 0x82E59DDC;
	sub_82D78278(ctx, base);
	// 82E59DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E59DE0: 4BE4F67C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59DE8 size=12
    let mut pc: u32 = 0x82E59DE8;
    'dispatch: loop {
        match pc {
            0x82E59DE8 => {
    //   block [0x82E59DE8..0x82E59DF4)
	// 82E59DE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E59DEC: 386B74A0  addi r3, r11, 0x74a0
	ctx.r[3].s64 = ctx.r[11].s64 + 29856;
	// 82E59DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59DF8 size=112
    let mut pc: u32 = 0x82E59DF8;
    'dispatch: loop {
        match pc {
            0x82E59DF8 => {
    //   block [0x82E59DF8..0x82E59E68)
	// 82E59DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59DFC: 4BE4F60D  bl 0x82ca9408
	ctx.lr = 0x82E59E00;
	sub_82CA93D0(ctx, base);
	// 82E59E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59E04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E59E08: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59E0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59E10: 419A0050  beq cr6, 0x82e59e60
	if ctx.cr[6].eq {
	pc = 0x82E59E60; continue 'dispatch;
	}
	// 82E59E14: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59E18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E59E1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E59E20: 40990040  ble cr6, 0x82e59e60
	if !ctx.cr[6].gt {
	pc = 0x82E59E60; continue 'dispatch;
	}
	// 82E59E24: 3BBC0018  addi r29, r28, 0x18
	ctx.r[29].s64 = ctx.r[28].s64 + 24;
	// 82E59E28: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E59E2C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59E30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E59E34: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E59E38: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59E3C: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E59E40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E59E44: 4E800421  bctrl
	ctx.lr = 0x82E59E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59E48: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E59E4C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E59E50: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E59E54: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E59E58: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E59E5C: 4198FFD0  blt cr6, 0x82e59e2c
	if ctx.cr[6].lt {
	pc = 0x82E59E2C; continue 'dispatch;
	}
	// 82E59E60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E59E64: 4BE4F5F4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59E68 size=224
    let mut pc: u32 = 0x82E59E68;
    'dispatch: loop {
        match pc {
            0x82E59E68 => {
    //   block [0x82E59E68..0x82E59F48)
	// 82E59E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59E6C: 4BE4F599  bl 0x82ca9404
	ctx.lr = 0x82E59E70;
	sub_82CA93D0(ctx, base);
	// 82E59E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59E74: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E59E78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E59E7C: 37DCFFE0  addic. r30, r28, -0x20
	ctx.xer.ca = (ctx.r[28].u32 > (!(-32 as u32)));
	ctx.r[30].s64 = ctx.r[28].s64 + -32;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E59E80: 389C0008  addi r4, r28, 8
	ctx.r[4].s64 = ctx.r[28].s64 + 8;
	// 82E59E84: 40820008  bne 0x82e59e8c
	if !ctx.cr[0].eq {
	pc = 0x82E59E8C; continue 'dispatch;
	}
	// 82E59E88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E59E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59E90: 4BF1F6E9  bl 0x82d79578
	ctx.lr = 0x82E59E94;
	sub_82D79578(ctx, base);
	// 82E59E94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E59E98: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 82E59E9C: 409A0008  bne cr6, 0x82e59ea4
	if !ctx.cr[6].eq {
	pc = 0x82E59EA4; continue 'dispatch;
	}
	// 82E59EA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E59EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59EA8: 4BF1F971  bl 0x82d79818
	ctx.lr = 0x82E59EAC;
	sub_82D79818(ctx, base);
	// 82E59EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E59EB0: 4BF1F011  bl 0x82d78ec0
	ctx.lr = 0x82E59EB4;
	sub_82D78EC0(ctx, base);
	// 82E59EB4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E59EB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E59EBC: 3BBB0020  addi r29, r27, 0x20
	ctx.r[29].s64 = ctx.r[27].s64 + 32;
	// 82E59EC0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59EC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59EC8: 4099003C  ble cr6, 0x82e59f04
	if !ctx.cr[6].gt {
	pc = 0x82E59F04; continue 'dispatch;
	}
	// 82E59ECC: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 82E59ED0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E59ED4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59ED8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E59EDC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59EE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59EE4: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E59EE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59EEC: 4E800421  bctrl
	ctx.lr = 0x82E59EF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59EF0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59EF4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E59EF8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E59EFC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E59F00: 4198FFD4  blt cr6, 0x82e59ed4
	if ctx.cr[6].lt {
	pc = 0x82E59ED4; continue 'dispatch;
	}
	// 82E59F04: A17B0004  lhz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59F08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59F0C: 419A0034  beq cr6, 0x82e59f40
	if ctx.cr[6].eq {
	pc = 0x82E59F40; continue 'dispatch;
	}
	// 82E59F10: A17B0006  lhz r11, 6(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E59F14: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E59F18: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E59F1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59F20: B17B0006  sth r11, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E59F24: 409A001C  bne cr6, 0x82e59f40
	if !ctx.cr[6].eq {
	pc = 0x82E59F40; continue 'dispatch;
	}
	// 82E59F28: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59F2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E59F30: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E59F34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59F38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59F3C: 4E800421  bctrl
	ctx.lr = 0x82E59F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E59F40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E59F44: 4BE4F510  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59F48 size=24
    let mut pc: u32 = 0x82E59F48;
    'dispatch: loop {
        match pc {
            0x82E59F48 => {
    //   block [0x82E59F48..0x82E59F60)
	// 82E59F48: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E59F4C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E59F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E59F54: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59F58: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E59F5C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59F60 size=36
    let mut pc: u32 = 0x82E59F60;
    'dispatch: loop {
        match pc {
            0x82E59F60 => {
    //   block [0x82E59F60..0x82E59F84)
	// 82E59F60: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E59F64: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59F68: 7F071840  cmplw cr6, r7, r3
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82E59F6C: 419A0018  beq cr6, 0x82e59f84
	if ctx.cr[6].eq {
		sub_82E59F84(ctx, base);
		return;
	}
	// 82E59F70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E59F74: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E59F78: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E59F7C: 4198FFE8  blt cr6, 0x82e59f64
	if ctx.cr[6].lt {
	pc = 0x82E59F64; continue 'dispatch;
	}
	// 82E59F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59F84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59F84 size=8
    let mut pc: u32 = 0x82E59F84;
    'dispatch: loop {
        match pc {
            0x82E59F84 => {
    //   block [0x82E59F84..0x82E59F8C)
	// 82E59F84: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E59F88: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59F8C size=44
    let mut pc: u32 = 0x82E59F8C;
    'dispatch: loop {
        match pc {
            0x82E59F8C => {
    //   block [0x82E59F8C..0x82E59FB8)
	// 82E59F8C: 8149000C  lwz r10, 0xc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E59F90: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E59F94: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E59F98: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E59F9C: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E59FA0: 9149000C  stw r10, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E59FA4: 7D47582E  lwzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E59FA8: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E59FAC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E59FB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E59FB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59FB8 size=24
    let mut pc: u32 = 0x82E59FB8;
    'dispatch: loop {
        match pc {
            0x82E59FB8 => {
    //   block [0x82E59FB8..0x82E59FD0)
	// 82E59FB8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E59FBC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E59FC0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E59FC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E59FC8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E59FCC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E59FD0 size=20
    let mut pc: u32 = 0x82E59FD0;
    'dispatch: loop {
        match pc {
            0x82E59FD0 => {
    //   block [0x82E59FD0..0x82E59FE4)
	// 82E59FD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59FD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E59FD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E59FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E59FE0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E59FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E59FE8 size=152
    let mut pc: u32 = 0x82E59FE8;
    'dispatch: loop {
        match pc {
            0x82E59FE8 => {
    //   block [0x82E59FE8..0x82E5A080)
	// 82E59FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E59FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E59FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E59FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E59FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E59FFC: 4800BD85  bl 0x82e65d80
	ctx.lr = 0x82E5A000;
	sub_82E65D80(ctx, base);
	// 82E5A000: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5A004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5A008: 396B7228  addi r11, r11, 0x7228
	ctx.r[11].s64 = ctx.r[11].s64 + 29224;
	// 82E5A00C: 394A7494  addi r10, r10, 0x7494
	ctx.r[10].s64 = ctx.r[10].s64 + 29844;
	// 82E5A010: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5A014: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5A018: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5A01C: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5A020: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5A024: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E5A028: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82E5A02C: 39297508  addi r9, r9, 0x7508
	ctx.r[9].s64 = ctx.r[9].s64 + 29960;
	// 82E5A030: 390874E8  addi r8, r8, 0x74e8
	ctx.r[8].s64 = ctx.r[8].s64 + 29928;
	// 82E5A034: 38E774D8  addi r7, r7, 0x74d8
	ctx.r[7].s64 = ctx.r[7].s64 + 29912;
	// 82E5A038: 38C674C8  addi r6, r6, 0x74c8
	ctx.r[6].s64 = ctx.r[6].s64 + 29896;
	// 82E5A03C: 38A574B8  addi r5, r5, 0x74b8
	ctx.r[5].s64 = ctx.r[5].s64 + 29880;
	// 82E5A040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5A044: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5A048: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E5A04C: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E5A050: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82E5A054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5A058: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82E5A05C: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82E5A060: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E5A064: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E5A068: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E5A06C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5A070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5A074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5A078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5A07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5A080 size=284
    let mut pc: u32 = 0x82E5A080;
    'dispatch: loop {
        match pc {
            0x82E5A080 => {
    //   block [0x82E5A080..0x82E5A19C)
	// 82E5A080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A084: 4BE4F37D  bl 0x82ca9400
	ctx.lr = 0x82E5A088;
	sub_82CA93D0(ctx, base);
	// 82E5A088: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5A08C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5A090: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5A094: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5A098: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5A09C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5A0A0: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5A0A4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5A0A8: 394A7508  addi r10, r10, 0x7508
	ctx.r[10].s64 = ctx.r[10].s64 + 29960;
	// 82E5A0AC: 392974E8  addi r9, r9, 0x74e8
	ctx.r[9].s64 = ctx.r[9].s64 + 29928;
	// 82E5A0B0: 390874D8  addi r8, r8, 0x74d8
	ctx.r[8].s64 = ctx.r[8].s64 + 29912;
	// 82E5A0B4: 38E774C8  addi r7, r7, 0x74c8
	ctx.r[7].s64 = ctx.r[7].s64 + 29896;
	// 82E5A0B8: 38C674B8  addi r6, r6, 0x74b8
	ctx.r[6].s64 = ctx.r[6].s64 + 29880;
	// 82E5A0BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5A0C0: 3B5F0020  addi r26, r31, 0x20
	ctx.r[26].s64 = ctx.r[31].s64 + 32;
	// 82E5A0C4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5A0C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5A0CC: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E5A0D0: 90FF0028  stw r7, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82E5A0D4: 90DF002C  stw r6, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82E5A0D8: 419A0070  beq cr6, 0x82e5a148
	if ctx.cr[6].eq {
	pc = 0x82E5A148; continue 'dispatch;
	}
	// 82E5A0DC: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5A0E0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E5A0E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5A0E8: 40990060  ble cr6, 0x82e5a148
	if !ctx.cr[6].gt {
	pc = 0x82E5A148; continue 'dispatch;
	}
	// 82E5A0EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5A0F0: 3B7AFFE0  addi r27, r26, -0x20
	ctx.r[27].s64 = ctx.r[26].s64 + -32;
	// 82E5A0F4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5A0F8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E5A0FC: 389A0008  addi r4, r26, 8
	ctx.r[4].s64 = ctx.r[26].s64 + 8;
	// 82E5A100: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5A104: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5A108: 409A0008  bne cr6, 0x82e5a110
	if !ctx.cr[6].eq {
	pc = 0x82E5A110; continue 'dispatch;
	}
	// 82E5A10C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5A110: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5A114: 4BF1DFB5  bl 0x82d780c8
	ctx.lr = 0x82E5A118;
	sub_82D780C8(ctx, base);
	// 82E5A118: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E5A11C: 389A000C  addi r4, r26, 0xc
	ctx.r[4].s64 = ctx.r[26].s64 + 12;
	// 82E5A120: 409A0008  bne cr6, 0x82e5a128
	if !ctx.cr[6].eq {
	pc = 0x82E5A128; continue 'dispatch;
	}
	// 82E5A124: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5A128: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5A12C: 4BF1E14D  bl 0x82d78278
	ctx.lr = 0x82E5A130;
	sub_82D78278(ctx, base);
	// 82E5A130: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5A134: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E5A138: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5A13C: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5A140: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5A144: 4198FFB0  blt cr6, 0x82e5a0f4
	if ctx.cr[6].lt {
	pc = 0x82E5A0F4; continue 'dispatch;
	}
	// 82E5A148: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5A14C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5A150: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5A154: 409A0020  bne cr6, 0x82e5a174
	if !ctx.cr[6].eq {
	pc = 0x82E5A174; continue 'dispatch;
	}
	// 82E5A158: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A15C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5A160: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5A164: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5A168: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5A16C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5A170: 4BEFB159  bl 0x82d552c8
	ctx.lr = 0x82E5A174;
	sub_82D552C8(ctx, base);
	// 82E5A174: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5A178: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5A17C: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5A180: 394A7228  addi r10, r10, 0x7228
	ctx.r[10].s64 = ctx.r[10].s64 + 29224;
	// 82E5A184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5A188: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E5A18C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82E5A190: 4800BB19  bl 0x82e65ca8
	ctx.lr = 0x82E5A194;
	sub_82E65CA8(ctx, base);
	// 82E5A194: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5A198: 4BE4F2B8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5A1A0 size=256
    let mut pc: u32 = 0x82E5A1A0;
    'dispatch: loop {
        match pc {
            0x82E5A1A0 => {
    //   block [0x82E5A1A0..0x82E5A2A0)
	// 82E5A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A1A4: 4BE4F265  bl 0x82ca9408
	ctx.lr = 0x82E5A1A8;
	sub_82CA93D0(ctx, base);
	// 82E5A1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5A1AC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E5A1B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E5A1B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E5A1B8: 83EB7BF4  lwz r31, 0x7bf4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31732 as u32) ) } as u64;
	// 82E5A1BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5A1C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A1C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5A1C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5A1CC: 4E800421  bctrl
	ctx.lr = 0x82E5A1D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5A1D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5A1D4: 388B74A0  addi r4, r11, 0x74a0
	ctx.r[4].s64 = ctx.r[11].s64 + 29856;
	// 82E5A1D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A1DC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5A1E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5A1E4: 4E800421  bctrl
	ctx.lr = 0x82E5A1E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5A1E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A1EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E5A1F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5A1F4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5A1F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5A1FC: 4E800421  bctrl
	ctx.lr = 0x82E5A200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5A200: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A204: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5A208: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5A20C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5A210: 4E800421  bctrl
	ctx.lr = 0x82E5A214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5A214: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E5A218: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5A21C: 419A007C  beq cr6, 0x82e5a298
	if ctx.cr[6].eq {
	pc = 0x82E5A298; continue 'dispatch;
	}
	// 82E5A220: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E5A224: 419A0074  beq cr6, 0x82e5a298
	if ctx.cr[6].eq {
	pc = 0x82E5A298; continue 'dispatch;
	}
	// 82E5A228: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5A22C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5A230: 4BEFB561  bl 0x82d55790
	ctx.lr = 0x82E5A234;
	sub_82D55790(ctx, base);
	// 82E5A234: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5A23C: 419A005C  beq cr6, 0x82e5a298
	if ctx.cr[6].eq {
	pc = 0x82E5A298; continue 'dispatch;
	}
	// 82E5A240: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5A248: 419A0010  beq cr6, 0x82e5a258
	if ctx.cr[6].eq {
	pc = 0x82E5A258; continue 'dispatch;
	}
	// 82E5A24C: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E5A250: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5A254: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E5A258: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 82E5A25C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5A260: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A264: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5A268: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5A26C: 409A0010  bne cr6, 0x82e5a27c
	if !ctx.cr[6].eq {
	pc = 0x82E5A27C; continue 'dispatch;
	}
	// 82E5A270: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5A274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5A278: 4BEFCD21  bl 0x82d56f98
	ctx.lr = 0x82E5A27C;
	sub_82D56F98(ctx, base);
	// 82E5A27C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A280: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A284: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5A288: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82E5A28C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A290: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5A294: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5A298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5A29C: 4BE4F1BC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5A2A0 size=96
    let mut pc: u32 = 0x82E5A2A0;
    'dispatch: loop {
        match pc {
            0x82E5A2A0 => {
    //   block [0x82E5A2A0..0x82E5A300)
	// 82E5A2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5A2A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5A2AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5A2B0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A2B4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5A2B8: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5A2BC: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82E5A2C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5A2C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5A2C8: 4BEFAF81  bl 0x82d55248
	ctx.lr = 0x82E5A2CC;
	sub_82D55248(ctx, base);
	// 82E5A2CC: 3960003C  li r11, 0x3c
	ctx.r[11].s64 = 60;
	// 82E5A2D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5A2D4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5A2D8: 4BFFFD11  bl 0x82e59fe8
	ctx.lr = 0x82E5A2DC;
	sub_82E59FE8(ctx, base);
	// 82E5A2DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5A2E0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5A2E4: 409A0008  bne cr6, 0x82e5a2ec
	if !ctx.cr[6].eq {
	pc = 0x82E5A2EC; continue 'dispatch;
	}
	// 82E5A2E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5A2EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5A2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5A2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5A2F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5A2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5A300 size=1396
    let mut pc: u32 = 0x82E5A300;
    'dispatch: loop {
        match pc {
            0x82E5A300 => {
    //   block [0x82E5A300..0x82E5A874)
	// 82E5A300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A304: 4BE4F0CD  bl 0x82ca93d0
	ctx.lr = 0x82E5A308;
	sub_82CA93D0(ctx, base);
	// 82E5A308: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82E5A30C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82E5A310: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5A878 size=64
    let mut pc: u32 = 0x82E5A878;
    'dispatch: loop {
        match pc {
            0x82E5A878 => {
    //   block [0x82E5A878..0x82E5A8B8)
	// 82E5A878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5A880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5A884: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5A888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5A88C: 38ABA2A0  addi r5, r11, -0x5d60
	ctx.r[5].s64 = ctx.r[11].s64 + -23904;
	// 82E5A890: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5A894: 388A44A0  addi r4, r10, 0x44a0
	ctx.r[4].s64 = ctx.r[10].s64 + 17568;
	// 82E5A898: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5A89C: 4BFDE3FD  bl 0x82e38c98
	ctx.lr = 0x82E5A8A0;
	sub_82E38C98(ctx, base);
	// 82E5A8A0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5A8A4: 906BB5B4  stw r3, -0x4a4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19020 as u32), ctx.r[3].u32 ) };
	// 82E5A8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5A8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5A8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5A8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5A8B8 size=12
    let mut pc: u32 = 0x82E5A8B8;
    'dispatch: loop {
        match pc {
            0x82E5A8B8 => {
    //   block [0x82E5A8B8..0x82E5A8C4)
	// 82E5A8B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5A8BC: 806BB5B4  lwz r3, -0x4a4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19020 as u32) ) } as u64;
	// 82E5A8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5A8C8 size=8
    let mut pc: u32 = 0x82E5A8C8;
    'dispatch: loop {
        match pc {
            0x82E5A8C8 => {
    //   block [0x82E5A8C8..0x82E5A8D0)
	// 82E5A8C8: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5A8CC: 4800001C  b 0x82e5a8e8
	sub_82E5A8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5A8D0 size=8
    let mut pc: u32 = 0x82E5A8D0;
    'dispatch: loop {
        match pc {
            0x82E5A8D0 => {
    //   block [0x82E5A8D0..0x82E5A8D8)
	// 82E5A8D0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5A8D4: 48000014  b 0x82e5a8e8
	sub_82E5A8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5A8D8 size=8
    let mut pc: u32 = 0x82E5A8D8;
    'dispatch: loop {
        match pc {
            0x82E5A8D8 => {
    //   block [0x82E5A8D8..0x82E5A8E0)
	// 82E5A8D8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82E5A8DC: 4800000C  b 0x82e5a8e8
	sub_82E5A8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5A8E0 size=8
    let mut pc: u32 = 0x82E5A8E0;
    'dispatch: loop {
        match pc {
            0x82E5A8E0 => {
    //   block [0x82E5A8E0..0x82E5A8E8)
	// 82E5A8E0: 3863FFD4  addi r3, r3, -0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + -44;
	// 82E5A8E4: 48000004  b 0x82e5a8e8
	sub_82E5A8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5A8E8 size=100
    let mut pc: u32 = 0x82E5A8E8;
    'dispatch: loop {
        match pc {
            0x82E5A8E8 => {
    //   block [0x82E5A8E8..0x82E5A94C)
	// 82E5A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5A8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5A8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5A8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5A8FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5A900: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5A904: 4BFFF77D  bl 0x82e5a080
	ctx.lr = 0x82E5A908;
	sub_82E5A080(ctx, base);
	// 82E5A908: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5A90C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5A910: 419A0020  beq cr6, 0x82e5a930
	if ctx.cr[6].eq {
	pc = 0x82E5A930; continue 'dispatch;
	}
	// 82E5A914: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A918: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5A91C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5A920: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A924: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5A928: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5A92C: 4BEFA99D  bl 0x82d552c8
	ctx.lr = 0x82E5A930;
	sub_82D552C8(ctx, base);
	// 82E5A930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5A934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5A938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5A93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5A940: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5A944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5A948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5A950 size=476
    let mut pc: u32 = 0x82E5A950;
    'dispatch: loop {
        match pc {
            0x82E5A950 => {
    //   block [0x82E5A950..0x82E5AB2C)
	// 82E5A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5A954: 4BE4EA91  bl 0x82ca93e4
	ctx.lr = 0x82E5A958;
	sub_82CA93D0(ctx, base);
	// 82E5A958: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5A95C: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A960: 3A800008  li r20, 8
	ctx.r[20].s64 = 8;
	// 82E5A964: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E5A968: 7D54982E  lwzx r10, r20, r19
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82E5A96C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A970: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5A974: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5A978: 40980020  bge cr6, 0x82e5a998
	if !ctx.cr[6].lt {
	pc = 0x82E5A998; continue 'dispatch;
	}
	// 82E5A97C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5A980: 39297544  addi r9, r9, 0x7544
	ctx.r[9].s64 = ctx.r[9].s64 + 30020;
	// 82E5A984: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5A988: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5A98C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E5A990: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5A994: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5A998: 3AC40028  addi r22, r4, 0x28
	ctx.r[22].s64 = ctx.r[4].s64 + 40;
	// 82E5A99C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82E5A9A0: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A9A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5A9A8: 4099014C  ble cr6, 0x82e5aaf4
	if !ctx.cr[6].gt {
	pc = 0x82E5AAF4; continue 'dispatch;
	}
	// 82E5A9AC: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82E5A9B0: 3F408334  lis r26, -0x7ccc
	ctx.r[26].s64 = -2093744128;
	// 82E5A9B4: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82E5A9B8: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A9BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5A9C0: 7D6BB82E  lwzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E5A9C4: 3B6B004C  addi r27, r11, 0x4c
	ctx.r[27].s64 = ctx.r[11].s64 + 76;
	// 82E5A9C8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5A9CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5A9D0: 40990110  ble cr6, 0x82e5aae0
	if !ctx.cr[6].gt {
	pc = 0x82E5AAE0; continue 'dispatch;
	}
	// 82E5A9D4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E5A9D8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5A9DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5A9E0: 7FDC582E  lwzx r30, r28, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5A9E4: 3BFE0120  addi r31, r30, 0x120
	ctx.r[31].s64 = ctx.r[30].s64 + 288;
	// 82E5A9E8: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82E5A9EC: 4BEFCD95  bl 0x82d57780
	ctx.lr = 0x82E5A9F0;
	sub_82D57780(ctx, base);
	// 82E5A9F0: 397F0040  addi r11, r31, 0x40
	ctx.r[11].s64 = ctx.r[31].s64 + 64;
	// 82E5A9F4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5AB30 size=60
    let mut pc: u32 = 0x82E5AB30;
    'dispatch: loop {
        match pc {
            0x82E5AB30 => {
    //   block [0x82E5AB30..0x82E5AB6C)
	// 82E5AB30: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5AB34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5AB38: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5AB3C: 40990028  ble cr6, 0x82e5ab64
	if !ctx.cr[6].gt {
	pc = 0x82E5AB64; continue 'dispatch;
	}
	// 82E5AB40: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5AB44: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AB48: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AB4C: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E5AB50: 419A001C  beq cr6, 0x82e5ab6c
	if ctx.cr[6].eq {
		sub_82E5AB6C(ctx, base);
		return;
	}
	// 82E5AB54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E5AB58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E5AB5C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5AB60: 4198FFE4  blt cr6, 0x82e5ab44
	if ctx.cr[6].lt {
	pc = 0x82E5AB44; continue 'dispatch;
	}
	// 82E5AB64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E5AB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AB6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5AB6C size=8
    let mut pc: u32 = 0x82E5AB6C;
    'dispatch: loop {
        match pc {
            0x82E5AB6C => {
    //   block [0x82E5AB6C..0x82E5AB74)
	// 82E5AB6C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E5AB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AB78 size=272
    let mut pc: u32 = 0x82E5AB78;
    'dispatch: loop {
        match pc {
            0x82E5AB78 => {
    //   block [0x82E5AB78..0x82E5AC88)
	// 82E5AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AB7C: 4BE4E88D  bl 0x82ca9408
	ctx.lr = 0x82E5AB80;
	sub_82CA93D0(ctx, base);
	// 82E5AB80: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5AB84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5AB88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5AB8C: 3BFE0120  addi r31, r30, 0x120
	ctx.r[31].s64 = ctx.r[30].s64 + 288;
	// 82E5AB90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5AB94: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82E5AB98: 4BEFCBE9  bl 0x82d57780
	ctx.lr = 0x82E5AB9C;
	sub_82D57780(ctx, base);
	// 82E5AB9C: 397F0040  addi r11, r31, 0x40
	ctx.r[11].s64 = ctx.r[31].s64 + 64;
	// 82E5ABA0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AC88 size=152
    let mut pc: u32 = 0x82E5AC88;
    'dispatch: loop {
        match pc {
            0x82E5AC88 => {
    //   block [0x82E5AC88..0x82E5AD20)
	// 82E5AC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5AC90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5AC94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5AC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5AC9C: 4800B0E5  bl 0x82e65d80
	ctx.lr = 0x82E5ACA0;
	sub_82E65D80(ctx, base);
	// 82E5ACA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5ACA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5ACA8: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E5ACAC: 394A7494  addi r10, r10, 0x7494
	ctx.r[10].s64 = ctx.r[10].s64 + 29844;
	// 82E5ACB0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5ACB4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5ACB8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5ACBC: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5ACC0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5ACC4: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E5ACC8: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82E5ACCC: 392975C4  addi r9, r9, 0x75c4
	ctx.r[9].s64 = ctx.r[9].s64 + 30148;
	// 82E5ACD0: 390875A4  addi r8, r8, 0x75a4
	ctx.r[8].s64 = ctx.r[8].s64 + 30116;
	// 82E5ACD4: 38E77594  addi r7, r7, 0x7594
	ctx.r[7].s64 = ctx.r[7].s64 + 30100;
	// 82E5ACD8: 38C6757C  addi r6, r6, 0x757c
	ctx.r[6].s64 = ctx.r[6].s64 + 30076;
	// 82E5ACDC: 38A5756C  addi r5, r5, 0x756c
	ctx.r[5].s64 = ctx.r[5].s64 + 30060;
	// 82E5ACE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5ACE4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5ACE8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E5ACEC: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E5ACF0: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82E5ACF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5ACF8: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82E5ACFC: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82E5AD00: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E5AD04: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E5AD08: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E5AD0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5AD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5AD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5AD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5AD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AD20 size=388
    let mut pc: u32 = 0x82E5AD20;
    'dispatch: loop {
        match pc {
            0x82E5AD20 => {
    //   block [0x82E5AD20..0x82E5AEA4)
	// 82E5AD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AD24: 4BE4E6E1  bl 0x82ca9404
	ctx.lr = 0x82E5AD28;
	sub_82CA93D0(ctx, base);
	// 82E5AD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5AD2C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E5AD30: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E5AD34: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5AD38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5AD3C: 419A0160  beq cr6, 0x82e5ae9c
	if ctx.cr[6].eq {
	pc = 0x82E5AE9C; continue 'dispatch;
	}
	// 82E5AD40: 813C000C  lwz r9, 0xc(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5AD44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5AD48: 811B0008  lwz r8, 8(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AD4C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5AD50: 4099014C  ble cr6, 0x82e5ae9c
	if !ctx.cr[6].gt {
	pc = 0x82E5AE9C; continue 'dispatch;
	}
	// 82E5AD54: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AD58: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AD5C: 80E70000  lwz r7, 0(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AD60: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E5AD64: 419A001C  beq cr6, 0x82e5ad80
	if ctx.cr[6].eq {
	pc = 0x82E5AD80; continue 'dispatch;
	}
	// 82E5AD68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5AD6C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5AD70: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5AD74: 4198FFE4  blt cr6, 0x82e5ad58
	if ctx.cr[6].lt {
	pc = 0x82E5AD58; continue 'dispatch;
	}
	// 82E5AD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5AD7C: 4BE4E6D8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E5AD80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5AD84: 41980118  blt cr6, 0x82e5ae9c
	if ctx.cr[6].lt {
	pc = 0x82E5AE9C; continue 'dispatch;
	}
	// 82E5AD88: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AD8C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5AD90: 807CFFF0  lwz r3, -0x10(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E5AD94: 3FA08334  lis r29, -0x7ccc
	ctx.r[29].s64 = -2093744128;
	// 82E5AD98: 3BDB0011  addi r30, r27, 0x11
	ctx.r[30].s64 = ctx.r[27].s64 + 17;
	// 82E5AD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E5ADA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5ADA4: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5ADA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5ADAC: 80BDB5B8  lwz r5, -0x4a48(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19016 as u32) ) } as u64;
	// 82E5ADB0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5ADB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5ADB8: 4E800421  bctrl
	ctx.lr = 0x82E5ADBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5ADBC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5ADC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5ADC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5ADC8: 40990050  ble cr6, 0x82e5ae18
	if !ctx.cr[6].gt {
	pc = 0x82E5AE18; continue 'dispatch;
	}
	// 82E5ADCC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5ADD0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5ADD4: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E5ADD8: 419A0018  beq cr6, 0x82e5adf0
	if ctx.cr[6].eq {
	pc = 0x82E5ADF0; continue 'dispatch;
	}
	// 82E5ADDC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5ADE0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E5ADE4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E5ADE8: 4198FFE8  blt cr6, 0x82e5add0
	if ctx.cr[6].lt {
	pc = 0x82E5ADD0; continue 'dispatch;
	}
	// 82E5ADEC: 4800002C  b 0x82e5ae18
	pc = 0x82E5AE18; continue 'dispatch;
	// 82E5ADF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5ADF4: 41980024  blt cr6, 0x82e5ae18
	if ctx.cr[6].lt {
	pc = 0x82E5AE18; continue 'dispatch;
	}
	// 82E5ADF8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5ADFC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5AE00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5AE04: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5AE08: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E5AE0C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5AE10: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5AE14: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5AE18: 807CFFF0  lwz r3, -0x10(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E5AE1C: 3BDB0012  addi r30, r27, 0x12
	ctx.r[30].s64 = ctx.r[27].s64 + 18;
	// 82E5AE20: 80BDB5B8  lwz r5, -0x4a48(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19016 as u32) ) } as u64;
	// 82E5AE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E5AE28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5AE2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AE30: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5AE34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5AE38: 4E800421  bctrl
	ctx.lr = 0x82E5AE3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5AE3C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5AE44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5AE48: 40990054  ble cr6, 0x82e5ae9c
	if !ctx.cr[6].gt {
	pc = 0x82E5AE9C; continue 'dispatch;
	}
	// 82E5AE4C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5AE50: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AE54: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E5AE58: 419A001C  beq cr6, 0x82e5ae74
	if ctx.cr[6].eq {
	pc = 0x82E5AE74; continue 'dispatch;
	}
	// 82E5AE5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5AE60: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E5AE64: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E5AE68: 4198FFE8  blt cr6, 0x82e5ae50
	if ctx.cr[6].lt {
	pc = 0x82E5AE50; continue 'dispatch;
	}
	// 82E5AE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5AE70: 4BE4E5E4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E5AE74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5AE78: 41980024  blt cr6, 0x82e5ae9c
	if ctx.cr[6].lt {
	pc = 0x82E5AE9C; continue 'dispatch;
	}
	// 82E5AE7C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AE80: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5AE84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5AE88: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5AE8C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E5AE90: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5AE94: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5AE98: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5AE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5AEA0: 4BE4E5B4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AEA8 size=172
    let mut pc: u32 = 0x82E5AEA8;
    'dispatch: loop {
        match pc {
            0x82E5AEA8 => {
    //   block [0x82E5AEA8..0x82E5AF54)
	// 82E5AEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AEAC: 4BE4E555  bl 0x82ca9400
	ctx.lr = 0x82E5AEB0;
	sub_82CA93D0(ctx, base);
	// 82E5AEB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5AEB4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E5AEB8: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5AEBC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E5AEC0: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82E5AEC4: 817C0030  lwz r11, 0x30(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5AEC8: 7FAB502E  lwzx r29, r11, r10
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5AECC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AED0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5AED4: 40990044  ble cr6, 0x82e5af18
	if !ctx.cr[6].gt {
	pc = 0x82E5AF18; continue 'dispatch;
	}
	// 82E5AED8: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82E5AEDC: 3F608334  lis r27, -0x7ccc
	ctx.r[27].s64 = -2093744128;
	// 82E5AEE0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5AEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E5AEE8: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5AEEC: 80BBB5B8  lwz r5, -0x4a48(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-19016 as u32) ) } as u64;
	// 82E5AEF0: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5AEF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AEF8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5AEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5AF00: 4E800421  bctrl
	ctx.lr = 0x82E5AF04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5AF04: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AF08: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E5AF0C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5AF10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5AF14: 4198FFCC  blt cr6, 0x82e5aee0
	if ctx.cr[6].lt {
	pc = 0x82E5AEE0; continue 'dispatch;
	}
	// 82E5AF18: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 82E5AF1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5AF20: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5AF24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5AF28: 40980020  bge cr6, 0x82e5af48
	if !ctx.cr[6].lt {
	pc = 0x82E5AF48; continue 'dispatch;
	}
	// 82E5AF2C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E5AF30: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E5AF34: 41990008  bgt cr6, 0x82e5af3c
	if ctx.cr[6].gt {
	pc = 0x82E5AF3C; continue 'dispatch;
	}
	// 82E5AF38: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5AF3C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E5AF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5AF44: 4BEFBFCD  bl 0x82d56f10
	ctx.lr = 0x82E5AF48;
	sub_82D56F10(ctx, base);
	// 82E5AF48: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82E5AF4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5AF50: 4BE4E500  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AF58 size=96
    let mut pc: u32 = 0x82E5AF58;
    'dispatch: loop {
        match pc {
            0x82E5AF58 => {
    //   block [0x82E5AF58..0x82E5AFB8)
	// 82E5AF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5AF60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5AF64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5AF68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5AF6C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5AF70: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5AF74: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82E5AF78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5AF7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5AF80: 4BEFA2C9  bl 0x82d55248
	ctx.lr = 0x82E5AF84;
	sub_82D55248(ctx, base);
	// 82E5AF84: 3960003C  li r11, 0x3c
	ctx.r[11].s64 = 60;
	// 82E5AF88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5AF8C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5AF90: 4BFFFCF9  bl 0x82e5ac88
	ctx.lr = 0x82E5AF94;
	sub_82E5AC88(ctx, base);
	// 82E5AF94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5AF98: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5AF9C: 409A0008  bne cr6, 0x82e5afa4
	if !ctx.cr[6].eq {
	pc = 0x82E5AFA4; continue 'dispatch;
	}
	// 82E5AFA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5AFA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5AFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5AFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5AFB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5AFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AFB8 size=64
    let mut pc: u32 = 0x82E5AFB8;
    'dispatch: loop {
        match pc {
            0x82E5AFB8 => {
    //   block [0x82E5AFB8..0x82E5AFF8)
	// 82E5AFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5AFC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5AFC4: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5AFC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5AFCC: 38ABAF58  addi r5, r11, -0x50a8
	ctx.r[5].s64 = ctx.r[11].s64 + -20648;
	// 82E5AFD0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5AFD4: 388A7530  addi r4, r10, 0x7530
	ctx.r[4].s64 = ctx.r[10].s64 + 30000;
	// 82E5AFD8: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5AFDC: 4BFDDCBD  bl 0x82e38c98
	ctx.lr = 0x82E5AFE0;
	sub_82E38C98(ctx, base);
	// 82E5AFE0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5AFE4: 906BB5B8  stw r3, -0x4a48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19016 as u32), ctx.r[3].u32 ) };
	// 82E5AFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5AFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5AFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5AFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5AFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5AFF8 size=580
    let mut pc: u32 = 0x82E5AFF8;
    'dispatch: loop {
        match pc {
            0x82E5AFF8 => {
    //   block [0x82E5AFF8..0x82E5B23C)
	// 82E5AFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5AFFC: 4BE4E3F5  bl 0x82ca93f0
	ctx.lr = 0x82E5B000;
	sub_82CA93D0(ctx, base);
	// 82E5B000: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B004: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82E5B008: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82E5B00C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E5B010: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82E5B014: 38970028  addi r4, r23, 0x28
	ctx.r[4].s64 = ctx.r[23].s64 + 40;
	// 82E5B018: 409A0008  bne cr6, 0x82e5b020
	if !ctx.cr[6].eq {
	pc = 0x82E5B020; continue 'dispatch;
	}
	// 82E5B01C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E5B020: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E5B024: 4BF1E635  bl 0x82d79658
	ctx.lr = 0x82E5B028;
	sub_82D79658(ctx, base);
	// 82E5B028: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82E5B02C: 3897002C  addi r4, r23, 0x2c
	ctx.r[4].s64 = ctx.r[23].s64 + 44;
	// 82E5B030: 409A0008  bne cr6, 0x82e5b038
	if !ctx.cr[6].eq {
	pc = 0x82E5B038; continue 'dispatch;
	}
	// 82E5B034: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E5B038: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E5B03C: 4BF1E7DD  bl 0x82d79818
	ctx.lr = 0x82E5B040;
	sub_82D79818(ctx, base);
	// 82E5B040: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B044: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5B048: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B04C: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E5B050: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5B054: 419A001C  beq cr6, 0x82e5b070
	if ctx.cr[6].eq {
	pc = 0x82E5B070; continue 'dispatch;
	}
	// 82E5B058: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E5B05C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5B060: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82E5B064: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B068: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E5B06C: 48000010  b 0x82e5b07c
	pc = 0x82E5B07C; continue 'dispatch;
	// 82E5B070: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E5B074: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E5B078: 4BEF9FD9  bl 0x82d55050
	ctx.lr = 0x82E5B07C;
	sub_82D55050(ctx, base);
	// 82E5B07C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5B080: 419A001C  beq cr6, 0x82e5b09c
	if ctx.cr[6].eq {
	pc = 0x82E5B09C; continue 'dispatch;
	}
	// 82E5B084: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E5B088: 93030004  stw r24, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82E5B08C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E5B090: 93030008  stw r24, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82E5B094: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E5B098: 48000008  b 0x82e5b0a0
	pc = 0x82E5B0A0; continue 'dispatch;
	// 82E5B09C: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5B0A0: 3BF70030  addi r31, r23, 0x30
	ctx.r[31].s64 = ctx.r[23].s64 + 48;
	// 82E5B0A4: 92DE0000  stw r22, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82E5B0A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5B0AC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B0B0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5B0B4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B0B8: 409A0010  bne cr6, 0x82e5b0c8
	if !ctx.cr[6].eq {
	pc = 0x82E5B0C8; continue 'dispatch;
	}
	// 82E5B0BC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5B0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5B0C4: 4BEFBED5  bl 0x82d56f98
	ctx.lr = 0x82E5B0C8;
	sub_82D56F98(ctx, base);
	// 82E5B0C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B0CC: 3B760028  addi r27, r22, 0x28
	ctx.r[27].s64 = ctx.r[22].s64 + 40;
	// 82E5B0D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B0D4: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E5B0D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5B0DC: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E5B0E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B0E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5B0E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5B0EC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B0F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B0F4: 40990070  ble cr6, 0x82e5b164
	if !ctx.cr[6].gt {
	pc = 0x82E5B164; continue 'dispatch;
	}
	// 82E5B0F8: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82E5B0FC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B100: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5B104: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E5B108: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5B10C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B110: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B114: 4099003C  ble cr6, 0x82e5b150
	if !ctx.cr[6].gt {
	pc = 0x82E5B150; continue 'dispatch;
	}
	// 82E5B118: 3B970028  addi r28, r23, 0x28
	ctx.r[28].s64 = ctx.r[23].s64 + 40;
	// 82E5B11C: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82E5B120: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B124: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5B128: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B12C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B130: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5B134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5B138: 4E800421  bctrl
	ctx.lr = 0x82E5B13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5B13C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B140: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5B144: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5B148: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B14C: 4198FFD4  blt cr6, 0x82e5b120
	if ctx.cr[6].lt {
	pc = 0x82E5B120; continue 'dispatch;
	}
	// 82E5B150: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B154: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5B158: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82E5B15C: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B160: 4198FF9C  blt cr6, 0x82e5b0fc
	if ctx.cr[6].lt {
	pc = 0x82E5B0FC; continue 'dispatch;
	}
	// 82E5B164: 3B560034  addi r26, r22, 0x34
	ctx.r[26].s64 = ctx.r[22].s64 + 52;
	// 82E5B168: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E5B16C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B170: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B174: 40990070  ble cr6, 0x82e5b1e4
	if !ctx.cr[6].gt {
	pc = 0x82E5B1E4; continue 'dispatch;
	}
	// 82E5B178: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82E5B17C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B180: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5B184: 7D7B582E  lwzx r11, r27, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B188: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5B18C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B190: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B194: 4099003C  ble cr6, 0x82e5b1d0
	if !ctx.cr[6].gt {
	pc = 0x82E5B1D0; continue 'dispatch;
	}
	// 82E5B198: 3B970028  addi r28, r23, 0x28
	ctx.r[28].s64 = ctx.r[23].s64 + 40;
	// 82E5B19C: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82E5B1A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B1A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5B1A8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B1AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B1B0: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5B1B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5B1B8: 4E800421  bctrl
	ctx.lr = 0x82E5B1BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5B1BC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B1C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5B1C4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5B1C8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B1CC: 4198FFD4  blt cr6, 0x82e5b1a0
	if ctx.cr[6].lt {
	pc = 0x82E5B1A0; continue 'dispatch;
	}
	// 82E5B1D0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B1D4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5B1D8: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E5B1DC: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B1E0: 4198FF9C  blt cr6, 0x82e5b17c
	if ctx.cr[6].lt {
	pc = 0x82E5B17C; continue 'dispatch;
	}
	// 82E5B1E4: 81760020  lwz r11, 0x20(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E5B1E8: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5B1EC: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5B1F0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B1F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B1F8: 4099003C  ble cr6, 0x82e5b234
	if !ctx.cr[6].gt {
	pc = 0x82E5B234; continue 'dispatch;
	}
	// 82E5B1FC: 3B970028  addi r28, r23, 0x28
	ctx.r[28].s64 = ctx.r[23].s64 + 40;
	// 82E5B200: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82E5B204: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B208: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5B20C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B210: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B214: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5B218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5B21C: 4E800421  bctrl
	ctx.lr = 0x82E5B220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5B220: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B224: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5B228: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5B22C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B230: 4198FFD4  blt cr6, 0x82e5b204
	if ctx.cr[6].lt {
	pc = 0x82E5B204; continue 'dispatch;
	}
	// 82E5B234: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E5B238: 4BE4E208  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5B240 size=992
    let mut pc: u32 = 0x82E5B240;
    'dispatch: loop {
        match pc {
            0x82E5B240 => {
    //   block [0x82E5B240..0x82E5B620)
	// 82E5B240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5B244: 4BE4E1B1  bl 0x82ca93f4
	ctx.lr = 0x82E5B248;
	sub_82CA93D0(ctx, base);
	// 82E5B248: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B24C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E5B250: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E5B254: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5B258: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B25C: 419A03BC  beq cr6, 0x82e5b618
	if ctx.cr[6].eq {
	pc = 0x82E5B618; continue 'dispatch;
	}
	// 82E5B260: 813B000C  lwz r9, 0xc(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5B264: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82E5B268: 811A0008  lwz r8, 8(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5B26C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82E5B270: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5B274: 409903A4  ble cr6, 0x82e5b618
	if !ctx.cr[6].gt {
	pc = 0x82E5B618; continue 'dispatch;
	}
	// 82E5B278: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5B27C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B280: 80E70000  lwz r7, 0(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B284: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E5B288: 419A001C  beq cr6, 0x82e5b2a4
	if ctx.cr[6].eq {
	pc = 0x82E5B2A4; continue 'dispatch;
	}
	// 82E5B28C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5B290: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5B294: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5B298: 4198FFE4  blt cr6, 0x82e5b27c
	if ctx.cr[6].lt {
	pc = 0x82E5B27C; continue 'dispatch;
	}
	// 82E5B29C: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 82E5B2A0: 4BE4E1A4  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82E5B2A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B2A8: 41980370  blt cr6, 0x82e5b618
	if ctx.cr[6].lt {
	pc = 0x82E5B618; continue 'dispatch;
	}
	// 82E5B2AC: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5B2B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5B2B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5B2B8: 7F0B502E  lwzx r24, r11, r10
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5B2BC: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E5B2C0: 92E10050  stw r23, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 82E5B2C4: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82E5B2C8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E5B2CC: 4800AC5D  bl 0x82e65f28
	ctx.lr = 0x82E5B2D0;
	sub_82E65F28(ctx, base);
	// 82E5B2D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E5B2D4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E5B2D8: 4800AC61  bl 0x82e65f38
	ctx.lr = 0x82E5B2DC;
	sub_82E65F38(ctx, base);
	// 82E5B2DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E5B2E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E5B2E4: 809A0010  lwz r4, 0x10(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5B2E8: 4800C661  bl 0x82e67948
	ctx.lr = 0x82E5B2EC;
	sub_82E67948(ctx, base);
	// 82E5B2EC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5B2F0: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 82E5B2F4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82E5B2F8: 419800B4  blt cr6, 0x82e5b3ac
	if ctx.cr[6].lt {
	pc = 0x82E5B3AC; continue 'dispatch;
	}
	// 82E5B2FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5B300: 573F103A  slwi r31, r25, 2
	ctx.r[31].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E5B304: 3BCB7610  addi r30, r11, 0x7610
	ctx.r[30].s64 = ctx.r[11].s64 + 30224;
	// 82E5B308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5B30C: 3F808333  lis r28, -0x7ccd
	ctx.r[28].s64 = -2093809664;
	// 82E5B310: 3BAB75D0  addi r29, r11, 0x75d0
	ctx.r[29].s64 = ctx.r[11].s64 + 30160;
	// 82E5B314: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5B318: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B31C: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5B320: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82E5B324: 409A0078  bne cr6, 0x82e5b39c
	if !ctx.cr[6].eq {
	pc = 0x82E5B39C; continue 'dispatch;
	}
	// 82E5B328: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5B32C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B330: 409A006C  bne cr6, 0x82e5b39c
	if !ctx.cr[6].eq {
	pc = 0x82E5B39C; continue 'dispatch;
	}
	// 82E5B334: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E5B338: 38810120  addi r4, r1, 0x120
	ctx.r[4].s64 = ctx.r[1].s64 + 288;
	// 82E5B33C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E5B340: 4BEFC6A9  bl 0x82d579e8
	ctx.lr = 0x82E5B344;
	sub_82D579E8(ctx, base);
	// 82E5B344: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E5B348: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5B34C: 4BEFCAA5  bl 0x82d57df0
	ctx.lr = 0x82E5B350;
	sub_82D57DF0(ctx, base);
	// 82E5B350: 807C7630  lwz r3, 0x7630(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E5B354: 390000A8  li r8, 0xa8
	ctx.r[8].s64 = 168;
	// 82E5B358: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E5B35C: 38C10120  addi r6, r1, 0x120
	ctx.r[6].s64 = ctx.r[1].s64 + 288;
	// 82E5B360: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E5B364: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B368: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5B36C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5B370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5B374: 4E800421  bctrl
	ctx.lr = 0x82E5B378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5B378: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E5B37C: 4BEFD0B5  bl 0x82d58430
	ctx.lr = 0x82E5B380;
	sub_82D58430(ctx, base);
	// 82E5B380: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5B384: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E5B388: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5B38C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E5B390: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5B394: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B398: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5B39C: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 82E5B3A0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E5B3A4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82E5B3A8: 4098FF6C  bge cr6, 0x82e5b314
	if !ctx.cr[6].lt {
	pc = 0x82E5B314; continue 'dispatch;
	}
	// 82E5B3AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E5B3B0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5B3B4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E5B3B8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5B3BC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E5B3C0: 409901E0  ble cr6, 0x82e5b5a0
	if !ctx.cr[6].gt {
	pc = 0x82E5B5A0; continue 'dispatch;
	}
	// 82E5B3C4: 897A00D8  lbz r11, 0xd8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E5B3C8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E5B3CC: 419A01D4  beq cr6, 0x82e5b5a0
	if ctx.cr[6].eq {
	pc = 0x82E5B5A0; continue 'dispatch;
	}
	// 82E5B3D0: 3BF80004  addi r31, r24, 4
	ctx.r[31].s64 = ctx.r[24].s64 + 4;
	// 82E5B3D4: 3B9A0011  addi r28, r26, 0x11
	ctx.r[28].s64 = ctx.r[26].s64 + 17;
	// 82E5B3D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5B3DC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B3E0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5B3E4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5B3E8: 409A0010  bne cr6, 0x82e5b3f8
	if !ctx.cr[6].eq {
	pc = 0x82E5B3F8; continue 'dispatch;
	}
	// 82E5B3EC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5B3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5B3F4: 4BEFBBA5  bl 0x82d56f98
	ctx.lr = 0x82E5B3F8;
	sub_82D56F98(ctx, base);
	// 82E5B3F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B3FC: 3BDA0120  addi r30, r26, 0x120
	ctx.r[30].s64 = ctx.r[26].s64 + 288;
	// 82E5B400: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B404: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82E5B408: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5B40C: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82E5B410: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E5B414: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B418: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5B41C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5B420: 4BEFC361  bl 0x82d57780
	ctx.lr = 0x82E5B424;
	sub_82D57780(ctx, base);
	// 82E5B424: 397E0040  addi r11, r30, 0x40
	ctx.r[11].s64 = ctx.r[30].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5B620 size=100
    let mut pc: u32 = 0x82E5B620;
    'dispatch: loop {
        match pc {
            0x82E5B620 => {
    //   block [0x82E5B620..0x82E5B684)
	// 82E5B620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5B624: 4BE4DDE5  bl 0x82ca9408
	ctx.lr = 0x82E5B628;
	sub_82CA93D0(ctx, base);
	// 82E5B628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B62C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5B630: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5B634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5B638: 419A0044  beq cr6, 0x82e5b67c
	if ctx.cr[6].eq {
	pc = 0x82E5B67C; continue 'dispatch;
	}
	// 82E5B63C: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5B640: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5B644: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5B648: 40990034  ble cr6, 0x82e5b67c
	if !ctx.cr[6].gt {
	pc = 0x82E5B67C; continue 'dispatch;
	}
	// 82E5B64C: 3B9DFFF8  addi r28, r29, -8
	ctx.r[28].s64 = ctx.r[29].s64 + -8;
	// 82E5B650: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5B654: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5B658: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5B65C: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5B660: 4BFFF999  bl 0x82e5aff8
	ctx.lr = 0x82E5B664;
	sub_82E5AFF8(ctx, base);
	// 82E5B664: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5B668: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5B66C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5B670: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5B674: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E5B678: 4198FFDC  blt cr6, 0x82e5b654
	if ctx.cr[6].lt {
	pc = 0x82E5B654; continue 'dispatch;
	}
	// 82E5B67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5B680: 4BE4DDD8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B688 size=8
    let mut pc: u32 = 0x82E5B688;
    'dispatch: loop {
        match pc {
            0x82E5B688 => {
    //   block [0x82E5B688..0x82E5B690)
	// 82E5B688: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5B68C: 4BFFF96C  b 0x82e5aff8
	sub_82E5AFF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5B690 size=148
    let mut pc: u32 = 0x82E5B690;
    'dispatch: loop {
        match pc {
            0x82E5B690 => {
    //   block [0x82E5B690..0x82E5B724)
	// 82E5B690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5B694: 4BE4DD79  bl 0x82ca940c
	ctx.lr = 0x82E5B698;
	sub_82CA93D0(ctx, base);
	// 82E5B698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B69C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5B6A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E5B6A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5B6A8: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82E5B6AC: 409A0008  bne cr6, 0x82e5b6b4
	if !ctx.cr[6].eq {
	pc = 0x82E5B6B4; continue 'dispatch;
	}
	// 82E5B6B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5B6B4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B6B8: 57BE103A  slwi r30, r29, 2
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E5B6BC: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5B6C0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B6C4: 4BF1CA95  bl 0x82d78158
	ctx.lr = 0x82E5B6C8;
	sub_82D78158(ctx, base);
	// 82E5B6C8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B6CC: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 82E5B6D0: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5B6D4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B6D8: 4BF1CBA1  bl 0x82d78278
	ctx.lr = 0x82E5B6DC;
	sub_82D78278(ctx, base);
	// 82E5B6DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5B6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5B6E4: 4BFFF7C5  bl 0x82e5aea8
	ctx.lr = 0x82E5B6E8;
	sub_82E5AEA8(ctx, base);
	// 82E5B6E8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B6EC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5B6F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5B6F4: 419A000C  beq cr6, 0x82e5b700
	if ctx.cr[6].eq {
	pc = 0x82E5B700; continue 'dispatch;
	}
	// 82E5B6F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5B6FC: 48002235  bl 0x82e5d930
	ctx.lr = 0x82E5B700;
	sub_82E5D930(ctx, base);
	// 82E5B700: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5B704: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B708: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5B70C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5B710: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82E5B714: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B718: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82E5B71C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5B720: 4BE4DD3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5B728 size=308
    let mut pc: u32 = 0x82E5B728;
    'dispatch: loop {
        match pc {
            0x82E5B728 => {
    //   block [0x82E5B728..0x82E5B85C)
	// 82E5B728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5B72C: 4BE4DCD9  bl 0x82ca9404
	ctx.lr = 0x82E5B730;
	sub_82CA93D0(ctx, base);
	// 82E5B730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B734: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5B738: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5B73C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5B740: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5B744: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5B748: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5B74C: 394A75C4  addi r10, r10, 0x75c4
	ctx.r[10].s64 = ctx.r[10].s64 + 30148;
	// 82E5B750: 392975A4  addi r9, r9, 0x75a4
	ctx.r[9].s64 = ctx.r[9].s64 + 30116;
	// 82E5B754: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82E5B758: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5B75C: 3908757C  addi r8, r8, 0x757c
	ctx.r[8].s64 = ctx.r[8].s64 + 30076;
	// 82E5B760: 396B7594  addi r11, r11, 0x7594
	ctx.r[11].s64 = ctx.r[11].s64 + 30100;
	// 82E5B764: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5B768: 38E7756C  addi r7, r7, 0x756c
	ctx.r[7].s64 = ctx.r[7].s64 + 30060;
	// 82E5B76C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5B770: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82E5B774: 3B7F002C  addi r27, r31, 0x2c
	ctx.r[27].s64 = ctx.r[31].s64 + 44;
	// 82E5B778: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 82E5B77C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5B780: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E5B784: 90FF002C  stw r7, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82E5B788: 41980080  blt cr6, 0x82e5b808
	if ctx.cr[6].lt {
	pc = 0x82E5B808; continue 'dispatch;
	}
	// 82E5B78C: 57DD103A  slwi r29, r30, 2
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E5B790: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B794: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5B798: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5B79C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B7A0: 4BF1C9B9  bl 0x82d78158
	ctx.lr = 0x82E5B7A4;
	sub_82D78158(ctx, base);
	// 82E5B7A4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B7A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E5B7AC: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5B7B0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B7B4: 4BF1CAC5  bl 0x82d78278
	ctx.lr = 0x82E5B7B8;
	sub_82D78278(ctx, base);
	// 82E5B7B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5B7BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5B7C0: 4BFFF6E9  bl 0x82e5aea8
	ctx.lr = 0x82E5B7C4;
	sub_82E5AEA8(ctx, base);
	// 82E5B7C4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B7C8: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5B7CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5B7D0: 419A000C  beq cr6, 0x82e5b7dc
	if ctx.cr[6].eq {
	pc = 0x82E5B7DC; continue 'dispatch;
	}
	// 82E5B7D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5B7D8: 48002159  bl 0x82e5d930
	ctx.lr = 0x82E5B7DC;
	sub_82E5D930(ctx, base);
	// 82E5B7DC: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5B7E0: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5B7E4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B7E8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5B7EC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5B7F0: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5B7F4: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82E5B7F8: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B7FC: 7D5D592E  stwx r10, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5B800: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82E5B804: 4098FF8C  bge cr6, 0x82e5b790
	if !ctx.cr[6].lt {
	pc = 0x82E5B790; continue 'dispatch;
	}
	// 82E5B808: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5B80C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5B810: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5B814: 409A0020  bne cr6, 0x82e5b834
	if !ctx.cr[6].eq {
	pc = 0x82E5B834; continue 'dispatch;
	}
	// 82E5B818: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B81C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5B820: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5B824: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B828: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5B82C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5B830: 4BEF9A99  bl 0x82d552c8
	ctx.lr = 0x82E5B834;
	sub_82D552C8(ctx, base);
	// 82E5B834: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5B838: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5B83C: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5B840: 394A5D40  addi r10, r10, 0x5d40
	ctx.r[10].s64 = ctx.r[10].s64 + 23872;
	// 82E5B844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5B848: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5B84C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5B850: 4800A459  bl 0x82e65ca8
	ctx.lr = 0x82E5B854;
	sub_82E65CA8(ctx, base);
	// 82E5B854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5B858: 4BE4DBFC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B860 size=20
    let mut pc: u32 = 0x82E5B860;
    'dispatch: loop {
        match pc {
            0x82E5B860 => {
    //   block [0x82E5B860..0x82E5B874)
	// 82E5B860: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5B864: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5B868: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5B86C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5B870: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B874 size=40
    let mut pc: u32 = 0x82E5B874;
    'dispatch: loop {
        match pc {
            0x82E5B874 => {
    //   block [0x82E5B874..0x82E5B89C)
	// 82E5B874: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5B878: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B87C: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B880: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E5B884: 419A0018  beq cr6, 0x82e5b89c
	if ctx.cr[6].eq {
		sub_82E5B89C(ctx, base);
		return;
	}
	// 82E5B888: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5B88C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5B890: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5B894: 4198FFE4  blt cr6, 0x82e5b878
	if ctx.cr[6].lt {
	pc = 0x82E5B878; continue 'dispatch;
	}
	// 82E5B898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B89C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B89C size=8
    let mut pc: u32 = 0x82E5B89C;
    'dispatch: loop {
        match pc {
            0x82E5B89C => {
    //   block [0x82E5B89C..0x82E5B8A4)
	// 82E5B89C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5B8A0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B8A4 size=8
    let mut pc: u32 = 0x82E5B8A4;
    'dispatch: loop {
        match pc {
            0x82E5B8A4 => {
    //   block [0x82E5B8A4..0x82E5B8AC)
	// 82E5B8A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5B8A8: 4BFFFDE8  b 0x82e5b690
	sub_82E5B690(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B8B0 size=12
    let mut pc: u32 = 0x82E5B8B0;
    'dispatch: loop {
        match pc {
            0x82E5B8B0 => {
    //   block [0x82E5B8B0..0x82E5B8BC)
	// 82E5B8B0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5B8B4: 806BB5B8  lwz r3, -0x4a48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19016 as u32) ) } as u64;
	// 82E5B8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B8C0 size=8
    let mut pc: u32 = 0x82E5B8C0;
    'dispatch: loop {
        match pc {
            0x82E5B8C0 => {
    //   block [0x82E5B8C0..0x82E5B8C8)
	// 82E5B8C0: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5B8C4: 4800001C  b 0x82e5b8e0
	sub_82E5B8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B8C8 size=8
    let mut pc: u32 = 0x82E5B8C8;
    'dispatch: loop {
        match pc {
            0x82E5B8C8 => {
    //   block [0x82E5B8C8..0x82E5B8D0)
	// 82E5B8C8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5B8CC: 48000014  b 0x82e5b8e0
	sub_82E5B8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B8D0 size=8
    let mut pc: u32 = 0x82E5B8D0;
    'dispatch: loop {
        match pc {
            0x82E5B8D0 => {
    //   block [0x82E5B8D0..0x82E5B8D8)
	// 82E5B8D0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82E5B8D4: 4800000C  b 0x82e5b8e0
	sub_82E5B8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B8D8 size=8
    let mut pc: u32 = 0x82E5B8D8;
    'dispatch: loop {
        match pc {
            0x82E5B8D8 => {
    //   block [0x82E5B8D8..0x82E5B8E0)
	// 82E5B8D8: 3863FFD4  addi r3, r3, -0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + -44;
	// 82E5B8DC: 48000004  b 0x82e5b8e0
	sub_82E5B8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5B8E0 size=100
    let mut pc: u32 = 0x82E5B8E0;
    'dispatch: loop {
        match pc {
            0x82E5B8E0 => {
    //   block [0x82E5B8E0..0x82E5B944)
	// 82E5B8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5B8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5B8E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5B8EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5B8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B8F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5B8F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5B8FC: 4BFFFE2D  bl 0x82e5b728
	ctx.lr = 0x82E5B900;
	sub_82E5B728(ctx, base);
	// 82E5B900: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5B904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5B908: 419A0020  beq cr6, 0x82e5b928
	if ctx.cr[6].eq {
	pc = 0x82E5B928; continue 'dispatch;
	}
	// 82E5B90C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5B910: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5B914: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5B918: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5B91C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5B920: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5B924: 4BEF99A5  bl 0x82d552c8
	ctx.lr = 0x82E5B928;
	sub_82D552C8(ctx, base);
	// 82E5B928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5B92C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5B930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5B934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5B938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5B93C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5B940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B948 size=28
    let mut pc: u32 = 0x82E5B948;
    'dispatch: loop {
        match pc {
            0x82E5B948 => {
    //   block [0x82E5B948..0x82E5B964)
	// 82E5B948: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E5B94C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E5B950: 354BFFE0  addic. r10, r11, -0x20
	ctx.xer.ca = (ctx.r[11].u32 > (!(-32 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -32;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5B954: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82E5B958: 40820008  bne 0x82e5b960
	if !ctx.cr[0].eq {
	pc = 0x82E5B960; continue 'dispatch;
	}
	// 82E5B95C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5B960: 4BF1DEB8  b 0x82d79818
	sub_82D79818(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5B968 size=28
    let mut pc: u32 = 0x82E5B968;
    'dispatch: loop {
        match pc {
            0x82E5B968 => {
    //   block [0x82E5B968..0x82E5B984)
	// 82E5B968: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E5B96C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E5B970: 354BFFE0  addic. r10, r11, -0x20
	ctx.xer.ca = (ctx.r[11].u32 > (!(-32 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -32;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5B974: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82E5B978: 40820008  bne 0x82e5b980
	if !ctx.cr[0].eq {
	pc = 0x82E5B980; continue 'dispatch;
	}
	// 82E5B97C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5B980: 4BF1C8F8  b 0x82d78278
	sub_82D78278(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5B988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5B988 size=216
    let mut pc: u32 = 0x82E5B988;
    'dispatch: loop {
        match pc {
            0x82E5B988 => {
    //   block [0x82E5B988..0x82E5BA60)
	// 82E5B988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5B98C: 4BE4DA7D  bl 0x82ca9408
	ctx.lr = 0x82E5B990;
	sub_82CA93D0(ctx, base);
	// 82E5B990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5B994: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5B998: 4800A3E9  bl 0x82e65d80
	ctx.lr = 0x82E5B99C;
	sub_82E65D80(ctx, base);
	// 82E5B99C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5B9A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5B9A4: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5B9A8: 394A76AC  addi r10, r10, 0x76ac
	ctx.r[10].s64 = ctx.r[10].s64 + 30380;
	// 82E5B9AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5B9B0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5B9B4: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5B9B8: 3929768C  addi r9, r9, 0x768c
	ctx.r[9].s64 = ctx.r[9].s64 + 30348;
	// 82E5B9BC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5B9C0: 3908767C  addi r8, r8, 0x767c
	ctx.r[8].s64 = ctx.r[8].s64 + 30332;
	// 82E5B9C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5B9C8: 38E7766C  addi r7, r7, 0x766c
	ctx.r[7].s64 = ctx.r[7].s64 + 30316;
	// 82E5B9CC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E5B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5B9D4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E5B9D8: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5B9DC: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E5B9E0: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82E5B9E4: 90FF0028  stw r7, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82E5B9E8: 98DF002C  stb r6, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u8 ) };
	// 82E5B9EC: 98DF002D  stb r6, 0x2d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(45 as u32), ctx.r[6].u8 ) };
	// 82E5B9F0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E5B9F4: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E5B9F8: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E5B9FC: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E5BA00: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E5BA04: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82E5BA08: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5BA0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5BA10: 419A0044  beq cr6, 0x82e5ba54
	if ctx.cr[6].eq {
	pc = 0x82E5BA54; continue 'dispatch;
	}
	// 82E5BA14: 814A0070  lwz r10, 0x70(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5BA18: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E5BA1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5BA20: 40990034  ble cr6, 0x82e5ba54
	if !ctx.cr[6].gt {
	pc = 0x82E5BA54; continue 'dispatch;
	}
	// 82E5BA24: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E5BA28: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5BA2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5BA30: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5BA34: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5BA38: 4BF1DDE1  bl 0x82d79818
	ctx.lr = 0x82E5BA3C;
	sub_82D79818(ctx, base);
	// 82E5BA3C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5BA40: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E5BA44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5BA48: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5BA4C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5BA50: 4198FFD8  blt cr6, 0x82e5ba28
	if ctx.cr[6].lt {
	pc = 0x82E5BA28; continue 'dispatch;
	}
	// 82E5BA54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5BA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5BA5C: 4BE4D9FC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5BA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5BA60 size=380
    let mut pc: u32 = 0x82E5BA60;
    'dispatch: loop {
        match pc {
            0x82E5BA60 => {
    //   block [0x82E5BA60..0x82E5BBDC)
	// 82E5BA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5BA64: 4BE4D9A5  bl 0x82ca9408
	ctx.lr = 0x82E5BA68;
	sub_82CA93D0(ctx, base);
	// 82E5BA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5BA6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5BA70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5BA74: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5BA78: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5BA7C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5BA80: 394A76AC  addi r10, r10, 0x76ac
	ctx.r[10].s64 = ctx.r[10].s64 + 30380;
	// 82E5BA84: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5BA88: 3929768C  addi r9, r9, 0x768c
	ctx.r[9].s64 = ctx.r[9].s64 + 30348;
	// 82E5BA8C: 3908767C  addi r8, r8, 0x767c
	ctx.r[8].s64 = ctx.r[8].s64 + 30332;
	// 82E5BA90: 38E7766C  addi r7, r7, 0x766c
	ctx.r[7].s64 = ctx.r[7].s64 + 30316;
	// 82E5BA94: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82E5BA98: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5BA9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5BAA0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5BAA4: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E5BAA8: 90FF0028  stw r7, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82E5BAAC: 419A0044  beq cr6, 0x82e5baf0
	if ctx.cr[6].eq {
	pc = 0x82E5BAF0; continue 'dispatch;
	}
	// 82E5BAB0: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5BAB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5BAB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5BABC: 40990034  ble cr6, 0x82e5baf0
	if !ctx.cr[6].gt {
	pc = 0x82E5BAF0; continue 'dispatch;
	}
	// 82E5BAC0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5BAC4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5BAC8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5BACC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5BAD0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5BAD4: 4BF1C7A5  bl 0x82d78278
	ctx.lr = 0x82E5BAD8;
	sub_82D78278(ctx, base);
	// 82E5BAD8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5BADC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E5BAE0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5BAE4: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5BAE8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5BAEC: 4198FFD8  blt cr6, 0x82e5bac4
	if ctx.cr[6].lt {
	pc = 0x82E5BAC4; continue 'dispatch;
	}
	// 82E5BAF0: 83DF0040  lwz r30, 0x40(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E5BAF4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5BAF8: 40990034  ble cr6, 0x82e5bb2c
	if !ctx.cr[6].gt {
	pc = 0x82E5BB2C; continue 'dispatch;
	}
	// 82E5BAFC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5BB00: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E5BB04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5BB08: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E5BB0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BB10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BB14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5BB18: 4E800421  bctrl
	ctx.lr = 0x82E5BB1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5BB1C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5BB20: 3BBD0080  addi r29, r29, 0x80
	ctx.r[29].s64 = ctx.r[29].s64 + 128;
	// 82E5BB24: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5BB28: 409AFFD8  bne cr6, 0x82e5bb00
	if !ctx.cr[6].eq {
	pc = 0x82E5BB00; continue 'dispatch;
	}
	// 82E5BB2C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E5BB30: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5BB34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5BB38: 409A0020  bne cr6, 0x82e5bb58
	if !ctx.cr[6].eq {
	pc = 0x82E5BB58; continue 'dispatch;
	}
	// 82E5BB3C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BB40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5BB44: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5BB48: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E5BB4C: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5BB50: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5BB54: 4BEF9775  bl 0x82d552c8
	ctx.lr = 0x82E5BB58;
	sub_82D552C8(ctx, base);
	// 82E5BB58: 83DF0034  lwz r30, 0x34(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5BB5C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5BB60: 40990034  ble cr6, 0x82e5bb94
	if !ctx.cr[6].gt {
	pc = 0x82E5BB94; continue 'dispatch;
	}
	// 82E5BB64: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5BB68: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5BB6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5BB70: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E5BB74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BB78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5BB80: 4E800421  bctrl
	ctx.lr = 0x82E5BB84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5BB84: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5BB88: 3BBD0080  addi r29, r29, 0x80
	ctx.r[29].s64 = ctx.r[29].s64 + 128;
	// 82E5BB8C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5BB90: 409AFFD8  bne cr6, 0x82e5bb68
	if !ctx.cr[6].eq {
	pc = 0x82E5BB68; continue 'dispatch;
	}
	// 82E5BB94: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5BB98: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5BB9C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5BBA0: 409A0020  bne cr6, 0x82e5bbc0
	if !ctx.cr[6].eq {
	pc = 0x82E5BBC0; continue 'dispatch;
	}
	// 82E5BBA4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BBA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5BBAC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5BBB0: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5BBB4: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5BBB8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5BBBC: 4BEF970D  bl 0x82d552c8
	ctx.lr = 0x82E5BBC0;
	sub_82D552C8(ctx, base);
	// 82E5BBC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5BBC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5BBC8: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5BBCC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5BBD0: 4800A0D9  bl 0x82e65ca8
	ctx.lr = 0x82E5BBD4;
	sub_82E65CA8(ctx, base);
	// 82E5BBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5BBD8: 4BE4D880  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5BBE0 size=96
    let mut pc: u32 = 0x82E5BBE0;
    'dispatch: loop {
        match pc {
            0x82E5BBE0 => {
    //   block [0x82E5BBE0..0x82E5BC40)
	// 82E5BBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5BBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5BBE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5BBEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5BBF0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BBF4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5BBF8: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5BBFC: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 82E5BC00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5BC04: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5BC08: 4BEF9641  bl 0x82d55248
	ctx.lr = 0x82E5BC0C;
	sub_82D55248(ctx, base);
	// 82E5BC0C: 39600048  li r11, 0x48
	ctx.r[11].s64 = 72;
	// 82E5BC10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5BC14: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5BC18: 4BFFFD71  bl 0x82e5b988
	ctx.lr = 0x82E5BC1C;
	sub_82E5B988(ctx, base);
	// 82E5BC1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5BC20: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5BC24: 409A0008  bne cr6, 0x82e5bc2c
	if !ctx.cr[6].eq {
	pc = 0x82E5BC2C; continue 'dispatch;
	}
	// 82E5BC28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5BC2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5BC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5BC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5BC38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5BC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5BC40 size=64
    let mut pc: u32 = 0x82E5BC40;
    'dispatch: loop {
        match pc {
            0x82E5BC40 => {
    //   block [0x82E5BC40..0x82E5BC80)
	// 82E5BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5BC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5BC4C: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5BC50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5BC54: 38ABBBE0  addi r5, r11, -0x4420
	ctx.r[5].s64 = ctx.r[11].s64 + -17440;
	// 82E5BC58: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5BC5C: 388A7654  addi r4, r10, 0x7654
	ctx.r[4].s64 = ctx.r[10].s64 + 30292;
	// 82E5BC60: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5BC64: 4BFDD035  bl 0x82e38c98
	ctx.lr = 0x82E5BC68;
	sub_82E38C98(ctx, base);
	// 82E5BC68: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5BC6C: 906BB5C4  stw r3, -0x4a3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19004 as u32), ctx.r[3].u32 ) };
	// 82E5BC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5BC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5BC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5BC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E5BC80 size=2264
    let mut pc: u32 = 0x82E5BC80;
    'dispatch: loop {
        match pc {
            0x82E5BC80 => {
    //   block [0x82E5BC80..0x82E5C558)
	// 82E5BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5BC84: 4BE4D74D  bl 0x82ca93d0
	ctx.lr = 0x82E5BC88;
	sub_82CA93D0(ctx, base);
	// 82E5BC88: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82E5BC8C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82E5BC90: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5BC94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BC98: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82E5BC9C: 906101D4  stw r3, 0x1d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(468 as u32), ctx.r[3].u32 ) };
	// 82E5BCA0: 908101DC  stw r4, 0x1dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(476 as u32), ctx.r[4].u32 ) };
	// 82E5BCA4: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5BCA8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E5BCAC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5BCB0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5BCB4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5BCB8: 40980020  bge cr6, 0x82e5bcd8
	if !ctx.cr[6].lt {
	pc = 0x82E5BCD8; continue 'dispatch;
	}
	// 82E5BCBC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5BCC0: 392976B8  addi r9, r9, 0x76b8
	ctx.r[9].s64 = ctx.r[9].s64 + 30392;
	// 82E5BCC4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5BCC8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5BCCC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E5BCD0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5BCD4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5BCD8: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5BCDC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E5BCE0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E5BCE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5BCE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5BCEC: 39C00001  li r14, 1
	ctx.r[14].s64 = 1;
	// 82E5BCF0: 3AAB5BAC  addi r21, r11, 0x5bac
	ctx.r[21].s64 = ctx.r[11].s64 + 23468;
	// 82E5BCF4: C3C90C64  lfs f30, 0xc64(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3172 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E5BCF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5BCFC: C3EA0BE4  lfs f31, 0xbe4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E5BD00: 39E0FFF0  li r15, -0x10
	ctx.r[15].s64 = -16;
	// 82E5BD04: 3A000010  li r16, 0x10
	ctx.r[16].s64 = 16;
	// 82E5BD08: 3AC0FFE0  li r22, -0x20
	ctx.r[22].s64 = -32;
	// 82E5BD0C: 3A20FFD0  li r17, -0x30
	ctx.r[17].s64 = -48;
	// 82E5BD10: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82E5BD14: 3A400020  li r18, 0x20
	ctx.r[18].s64 = 32;
	// 82E5BD18: 3A600040  li r19, 0x40
	ctx.r[19].s64 = 64;
	// 82E5BD1C: 3A8B5C1C  addi r20, r11, 0x5c1c
	ctx.r[20].s64 = ctx.r[11].s64 + 23580;
	// 82E5BD20: 419A0440  beq cr6, 0x82e5c160
	if ctx.cr[6].eq {
	pc = 0x82E5C160; continue 'dispatch;
	}
	// 82E5BD24: 816101DC  lwz r11, 0x1dc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(476 as u32) ) } as u64;
	// 82E5BD28: 3B0B0028  addi r24, r11, 0x28
	ctx.r[24].s64 = ctx.r[11].s64 + 40;
	// 82E5BD2C: 816101D4  lwz r11, 0x1d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(468 as u32) ) } as u64;
	// 82E5BD30: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5BD34: 83580004  lwz r26, 4(r24)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5BD38: 7F1A5000  cmpw cr6, r26, r10
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E5BD3C: 409901C8  ble cr6, 0x82e5bf04
	if !ctx.cr[6].gt {
	pc = 0x82E5BF04; continue 'dispatch;
	}
	// 82E5BD40: 3B6B0014  addi r27, r11, 0x14
	ctx.r[27].s64 = ctx.r[11].s64 + 20;
	// 82E5BD44: 83BB0004  lwz r29, 4(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5BD48: 7F1AE800  cmpw cr6, r26, r29
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E5BD4C: 4098003C  bge cr6, 0x82e5bd88
	if !ctx.cr[6].lt {
	pc = 0x82E5BD88; continue 'dispatch;
	}
	// 82E5BD50: 575E3830  slwi r30, r26, 7
	ctx.r[30].u32 = ctx.r[26].u32.wrapping_shl(7);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E5BD54: 7FFAE850  subf r31, r26, r29
	ctx.r[31].s64 = ctx.r[29].s64 - ctx.r[26].s64;
	// 82E5BD58: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BD5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5BD60: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E5BD64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BD68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BD6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5BD70: 4E800421  bctrl
	ctx.lr = 0x82E5BD74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5BD74: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E5BD78: 3BDE0080  addi r30, r30, 0x80
	ctx.r[30].s64 = ctx.r[30].s64 + 128;
	// 82E5BD7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5BD80: 409AFFD8  bne cr6, 0x82e5bd58
	if !ctx.cr[6].eq {
	pc = 0x82E5BD58; continue 'dispatch;
	}
	// 82E5BD84: 4800017C  b 0x82e5bf00
	pc = 0x82E5BF00; continue 'dispatch;
	// 82E5BD88: 833B0008  lwz r25, 8(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5BD8C: 572B00BE  clrlwi r11, r25, 2
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5BD90: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5BD94: 40990134  ble cr6, 0x82e5bec8
	if !ctx.cr[6].gt {
	pc = 0x82E5BEC8; continue 'dispatch;
	}
	// 82E5BD98: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5BD9C: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5BDA0: 41980008  blt cr6, 0x82e5bda8
	if ctx.cr[6].lt {
	pc = 0x82E5BDA8; continue 'dispatch;
	}
	// 82E5BDA4: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82E5BDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5BDAC: 839B0000  lwz r28, 0(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BDB0: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E5BDB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5BDB8: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5BDBC: 915B0004  stw r10, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E5BDC0: 913B0008  stw r9, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5BDC4: 4099001C  ble cr6, 0x82e5bde0
	if !ctx.cr[6].gt {
	pc = 0x82E5BDE0; continue 'dispatch;
	}
	// 82E5BDC8: 40980008  bge cr6, 0x82e5bdd0
	if !ctx.cr[6].lt {
	pc = 0x82E5BDD0; continue 'dispatch;
	}
	// 82E5BDCC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E5BDD0: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 82E5BDD4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5BDD8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E5BDDC: 4BEFB135  bl 0x82d56f10
	ctx.lr = 0x82E5BDE0;
	sub_82D56F10(ctx, base);
	// 82E5BDE0: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5BDE4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E5BDE8: 4099007C  ble cr6, 0x82e5be64
	if !ctx.cr[6].gt {
	pc = 0x82E5BE64; continue 'dispatch;
	}
	// 82E5BDEC: 39690040  addi r11, r9, 0x40
	ctx.r[11].s64 = ctx.r[9].s64 + 64;
	// 82E5BDF0: 395C0030  addi r10, r28, 0x30
	ctx.r[10].s64 = ctx.r[28].s64 + 48;
	// 82E5BDF4: 7D3C4850  subf r9, r28, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[28].s64;
	// 82E5BDF8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82E5BDFC: 34EBFFC0  addic. r7, r11, -0x40
	ctx.xer.ca = (ctx.r[11].u32 > (!(-64 as u32)));
	ctx.r[7].s64 = ctx.r[11].s64 + -64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E5BE00: 41820050  beq 0x82e5be50
	if ctx.cr[0].eq {
	pc = 0x82E5BE50; continue 'dispatch;
	}
	// 82E5BE04: 92ABFFC0  stw r21, -0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-64 as u32), ctx.r[21].u32 ) };
	// 82E5BE08: B1CBFFC6  sth r14, -0x3a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-58 as u32), ctx.r[14].u16 ) };
	// 82E5BE0C: 80EAFFD8  lwz r7, -0x28(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-40 as u32) ) } as u64;
	// 82E5BE10: 90EBFFC8  stw r7, -0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-56 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C558 size=12
    let mut pc: u32 = 0x82E5C558;
    'dispatch: loop {
        match pc {
            0x82E5C558 => {
    //   block [0x82E5C558..0x82E5C564)
	// 82E5C558: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5C55C: 806BB5C4  lwz r3, -0x4a3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19004 as u32) ) } as u64;
	// 82E5C560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C568 size=8
    let mut pc: u32 = 0x82E5C568;
    'dispatch: loop {
        match pc {
            0x82E5C568 => {
    //   block [0x82E5C568..0x82E5C570)
	// 82E5C568: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5C56C: 48000014  b 0x82e5c580
	sub_82E5C580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C570 size=8
    let mut pc: u32 = 0x82E5C570;
    'dispatch: loop {
        match pc {
            0x82E5C570 => {
    //   block [0x82E5C570..0x82E5C578)
	// 82E5C570: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5C574: 4800000C  b 0x82e5c580
	sub_82E5C580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C578 size=8
    let mut pc: u32 = 0x82E5C578;
    'dispatch: loop {
        match pc {
            0x82E5C578 => {
    //   block [0x82E5C578..0x82E5C580)
	// 82E5C578: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82E5C57C: 48000004  b 0x82e5c580
	sub_82E5C580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5C580 size=100
    let mut pc: u32 = 0x82E5C580;
    'dispatch: loop {
        match pc {
            0x82E5C580 => {
    //   block [0x82E5C580..0x82E5C5E4)
	// 82E5C580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5C584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5C588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5C58C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5C590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5C594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5C598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5C59C: 4BFFF4C5  bl 0x82e5ba60
	ctx.lr = 0x82E5C5A0;
	sub_82E5BA60(ctx, base);
	// 82E5C5A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5C5A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5C5A8: 419A0020  beq cr6, 0x82e5c5c8
	if ctx.cr[6].eq {
	pc = 0x82E5C5C8; continue 'dispatch;
	}
	// 82E5C5AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C5B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5C5B4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5C5B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C5BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5C5C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5C5C4: 4BEF8D05  bl 0x82d552c8
	ctx.lr = 0x82E5C5C8;
	sub_82D552C8(ctx, base);
	// 82E5C5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5C5CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5C5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5C5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5C5D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5C5DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5C5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C5E8 size=8
    let mut pc: u32 = 0x82E5C5E8;
    'dispatch: loop {
        match pc {
            0x82E5C5E8 => {
    //   block [0x82E5C5E8..0x82E5C5F0)
	// 82E5C5E8: 98830061  stb r4, 0x61(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(97 as u32), ctx.r[4].u8 ) };
	// 82E5C5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C5F0 size=8
    let mut pc: u32 = 0x82E5C5F0;
    'dispatch: loop {
        match pc {
            0x82E5C5F0 => {
    //   block [0x82E5C5F0..0x82E5C5F8)
	// 82E5C5F0: 98830065  stb r4, 0x65(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(101 as u32), ctx.r[4].u8 ) };
	// 82E5C5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C5F8 size=8
    let mut pc: u32 = 0x82E5C5F8;
    'dispatch: loop {
        match pc {
            0x82E5C5F8 => {
    //   block [0x82E5C5F8..0x82E5C600)
	// 82E5C5F8: 98830062  stb r4, 0x62(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(98 as u32), ctx.r[4].u8 ) };
	// 82E5C5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C600 size=8
    let mut pc: u32 = 0x82E5C600;
    'dispatch: loop {
        match pc {
            0x82E5C600 => {
    //   block [0x82E5C600..0x82E5C608)
	// 82E5C600: 98830063  stb r4, 0x63(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(99 as u32), ctx.r[4].u8 ) };
	// 82E5C604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C608 size=8
    let mut pc: u32 = 0x82E5C608;
    'dispatch: loop {
        match pc {
            0x82E5C608 => {
    //   block [0x82E5C608..0x82E5C610)
	// 82E5C608: 98830064  stb r4, 0x64(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[4].u8 ) };
	// 82E5C60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C610 size=24
    let mut pc: u32 = 0x82E5C610;
    'dispatch: loop {
        match pc {
            0x82E5C610 => {
    //   block [0x82E5C610..0x82E5C628)
	// 82E5C610: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C614: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E5C618: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5C61C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5C620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5C624: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C628 size=8
    let mut pc: u32 = 0x82E5C628;
    'dispatch: loop {
        match pc {
            0x82E5C628 => {
    //   block [0x82E5C628..0x82E5C630)
	// 82E5C628: 38630054  addi r3, r3, 0x54
	ctx.r[3].s64 = ctx.r[3].s64 + 84;
	// 82E5C62C: 4BF03F54  b 0x82d60580
	sub_82D60580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E5C630 size=272
    let mut pc: u32 = 0x82E5C630;
    'dispatch: loop {
        match pc {
            0x82E5C630 => {
    //   block [0x82E5C630..0x82E5C740)
	// 82E5C630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5C634: 4BE4CDBD  bl 0x82ca93f0
	ctx.lr = 0x82E5C638;
	sub_82CA93D0(ctx, base);
	// 82E5C638: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82E5C63C: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5C640: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E5C644: C3E30068  lfs f31, 0x68(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E5C648: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E5C64C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E5C650: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E5C654: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82E5C658: 4199000C  bgt cr6, 0x82e5c664
	if ctx.cr[6].gt {
	pc = 0x82E5C664; continue 'dispatch;
	}
	// 82E5C65C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5C660: C3EB0010  lfs f31, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E5C664: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C668: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E5C66C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E5C670: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5C674: 4E800421  bctrl
	ctx.lr = 0x82E5C678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5C678: 3AFF0028  addi r23, r31, 0x28
	ctx.r[23].s64 = ctx.r[31].s64 + 40;
	// 82E5C67C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82E5C680: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C684: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5C688: 40990098  ble cr6, 0x82e5c720
	if !ctx.cr[6].gt {
	pc = 0x82E5C720; continue 'dispatch;
	}
	// 82E5C68C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E5C690: 3F208334  lis r25, -0x7ccc
	ctx.r[25].s64 = -2093744128;
	// 82E5C694: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C698: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5C69C: 7D6BC02E  lwzx r11, r11, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E5C6A0: 3B8B004C  addi r28, r11, 0x4c
	ctx.r[28].s64 = ctx.r[11].s64 + 76;
	// 82E5C6A4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C6A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5C6AC: 40990060  ble cr6, 0x82e5c70c
	if !ctx.cr[6].gt {
	pc = 0x82E5C70C; continue 'dispatch;
	}
	// 82E5C6B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5C6B4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C6B8: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5C6BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5C6C0: 480011B1  bl 0x82e5d870
	ctx.lr = 0x82E5C6C4;
	sub_82E5D870(ctx, base);
	// 82E5C6C4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E5C6C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E5C6CC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E5C6D0: 387D00D0  addi r3, r29, 0xd0
	ctx.r[3].s64 = ctx.r[29].s64 + 208;
	// 82E5C6D4: 4BF2E37D  bl 0x82d8aa50
	ctx.lr = 0x82E5C6D8;
	sub_82D8AA50(ctx, base);
	// 82E5C6D8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C6DC: 80D9B5C8  lwz r6, -0x4a38(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-19000 as u32) ) } as u64;
	// 82E5C6E0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E5C6E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5C6E8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E5C6EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5C6F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5C6F4: 4E800421  bctrl
	ctx.lr = 0x82E5C6F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5C6F8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C6FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E5C700: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5C704: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5C708: 4198FFAC  blt cr6, 0x82e5c6b4
	if ctx.cr[6].lt {
	pc = 0x82E5C6B4; continue 'dispatch;
	}
	// 82E5C70C: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C710: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82E5C714: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 82E5C718: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5C71C: 4198FF78  blt cr6, 0x82e5c694
	if ctx.cr[6].lt {
	pc = 0x82E5C694; continue 'dispatch;
	}
	// 82E5C720: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C724: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E5C728: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E5C72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5C730: 4E800421  bctrl
	ctx.lr = 0x82E5C734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5C734: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82E5C738: CBE1FFA0  lfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82E5C73C: 4BE4CD04  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5C740 size=192
    let mut pc: u32 = 0x82E5C740;
    'dispatch: loop {
        match pc {
            0x82E5C740 => {
    //   block [0x82E5C740..0x82E5C800)
	// 82E5C740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5C744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5C748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5C74C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5C750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5C754: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C758: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82E5C75C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E5C760: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E5C764: 7D3FF02E  lwzx r9, r31, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5C768: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C76C: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5C770: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E5C774: 40980020  bge cr6, 0x82e5c794
	if !ctx.cr[6].lt {
	pc = 0x82E5C794; continue 'dispatch;
	}
	// 82E5C778: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5C77C: 390876D4  addi r8, r8, 0x76d4
	ctx.r[8].s64 = ctx.r[8].s64 + 30420;
	// 82E5C780: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E5C784: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82E5C788: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82E5C78C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5C790: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E5C794: 894B0034  lbz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5C798: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5C79C: 419A001C  beq cr6, 0x82e5c7b8
	if ctx.cr[6].eq {
	pc = 0x82E5C7B8; continue 'dispatch;
	}
	// 82E5C7A0: 814BFFD4  lwz r10, -0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-44 as u32) ) } as u64;
	// 82E5C7A4: 386BFFD4  addi r3, r11, -0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + -44;
	// 82E5C7A8: 808BFFEC  lwz r4, -0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82E5C7AC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5C7B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5C7B4: 4E800421  bctrl
	ctx.lr = 0x82E5C7B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5C7B8: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5C7BC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5C7C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C7C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5C7C8: 40980020  bge cr6, 0x82e5c7e8
	if !ctx.cr[6].lt {
	pc = 0x82E5C7E8; continue 'dispatch;
	}
	// 82E5C7CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82E5C7D0: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82E5C7D4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5C7D8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5C7DC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E5C7E0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5C7E4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5C7E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5C7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5C7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5C7F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5C7F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5C7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C800 size=60
    let mut pc: u32 = 0x82E5C800;
    'dispatch: loop {
        match pc {
            0x82E5C800 => {
    //   block [0x82E5C800..0x82E5C83C)
	// 82E5C800: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5C804: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5C808: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5C80C: 40990028  ble cr6, 0x82e5c834
	if !ctx.cr[6].gt {
	pc = 0x82E5C834; continue 'dispatch;
	}
	// 82E5C810: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5C814: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C818: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C81C: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E5C820: 419A001C  beq cr6, 0x82e5c83c
	if ctx.cr[6].eq {
		sub_82E5C83C(ctx, base);
		return;
	}
	// 82E5C824: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E5C828: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E5C82C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5C830: 4198FFE4  blt cr6, 0x82e5c814
	if ctx.cr[6].lt {
	pc = 0x82E5C814; continue 'dispatch;
	}
	// 82E5C834: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E5C838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C83C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5C83C size=8
    let mut pc: u32 = 0x82E5C83C;
    'dispatch: loop {
        match pc {
            0x82E5C83C => {
    //   block [0x82E5C83C..0x82E5C844)
	// 82E5C83C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E5C840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5C848 size=72
    let mut pc: u32 = 0x82E5C848;
    'dispatch: loop {
        match pc {
            0x82E5C848 => {
    //   block [0x82E5C848..0x82E5C890)
	// 82E5C848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5C84C: 4BE4CBC1  bl 0x82ca940c
	ctx.lr = 0x82E5C850;
	sub_82CA93D0(ctx, base);
	// 82E5C850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5C854: 83C3FFEC  lwz r30, -0x14(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82E5C858: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E5C85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5C860: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C864: 4800100D  bl 0x82e5d870
	ctx.lr = 0x82E5C868;
	sub_82E5D870(ctx, base);
	// 82E5C868: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5C86C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5C870: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E5C874: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 82E5C878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5C87C: 80CBB5C8  lwz r6, -0x4a38(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19000 as u32) ) } as u64;
	// 82E5C880: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E5C884: 4E800421  bctrl
	ctx.lr = 0x82E5C888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5C888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5C88C: 4BE4CBD0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E5C890 size=336
    let mut pc: u32 = 0x82E5C890;
    'dispatch: loop {
        match pc {
            0x82E5C890 => {
    //   block [0x82E5C890..0x82E5C9E0)
	// 82E5C890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5C894: 4BE4CB69  bl 0x82ca93fc
	ctx.lr = 0x82E5C898;
	sub_82CA93D0(ctx, base);
	// 82E5C898: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5C89C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5C8A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5C8A4: 480094DD  bl 0x82e65d80
	ctx.lr = 0x82E5C8A8;
	sub_82E65D80(ctx, base);
	// 82E5C8A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5C8AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5C8B0: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E5C8B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5C8B8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5C8BC: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5C8C0: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5C8C4: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E5C8C8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5C8CC: 394A7494  addi r10, r10, 0x7494
	ctx.r[10].s64 = ctx.r[10].s64 + 29844;
	// 82E5C8D0: 39297764  addi r9, r9, 0x7764
	ctx.r[9].s64 = ctx.r[9].s64 + 30564;
	// 82E5C8D4: 39087744  addi r8, r8, 0x7744
	ctx.r[8].s64 = ctx.r[8].s64 + 30532;
	// 82E5C8D8: 38E77734  addi r7, r7, 0x7734
	ctx.r[7].s64 = ctx.r[7].s64 + 30516;
	// 82E5C8DC: 38C6771C  addi r6, r6, 0x771c
	ctx.r[6].s64 = ctx.r[6].s64 + 30492;
	// 82E5C8E0: 38A5770C  addi r5, r5, 0x770c
	ctx.r[5].s64 = ctx.r[5].s64 + 30476;
	// 82E5C8E4: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82E5C8E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5C8EC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5C8F0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E5C8F4: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E5C8F8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82E5C8FC: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82E5C900: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82E5C904: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82E5C908: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82E5C90C: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82E5C910: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E5C914: 4B40A62D  bl 0x82266f40
	ctx.lr = 0x82E5C918;
	sub_82266F40(ctx, base);
	// 82E5C918: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82E5C91C: 4B40A625  bl 0x82266f40
	ctx.lr = 0x82E5C920;
	sub_82266F40(ctx, base);
	// 82E5C920: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82E5C924: 4B40A61D  bl 0x82266f40
	ctx.lr = 0x82E5C928;
	sub_82266F40(ctx, base);
	// 82E5C928: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5C92C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E5C930: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82E5C934: 997F0060  stb r11, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82E5C938: 9BBF0061  stb r29, 0x61(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(97 as u32), ctx.r[29].u8 ) };
	// 82E5C93C: C00A0EE0  lfs f0, 0xee0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E5C940: 9BBF0062  stb r29, 0x62(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(98 as u32), ctx.r[29].u8 ) };
	// 82E5C944: 997F0063  stb r11, 0x63(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(99 as u32), ctx.r[11].u8 ) };
	// 82E5C948: 997F0064  stb r11, 0x64(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u8 ) };
	// 82E5C94C: 997F0065  stb r11, 0x65(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(101 as u32), ctx.r[11].u8 ) };
	// 82E5C950: D01F0068  stfs f0, 0x68(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E5C954: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C958: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E5C95C: 40990078  ble cr6, 0x82e5c9d4
	if !ctx.cr[6].gt {
	pc = 0x82E5C9D4; continue 'dispatch;
	}
	// 82E5C960: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5C964: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82E5C968: 3B2B76EC  addi r25, r11, 0x76ec
	ctx.r[25].s64 = ctx.r[11].s64 + 30444;
	// 82E5C96C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C970: 7C7A582E  lwzx r3, r26, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5C974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C978: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5C97C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5C980: 4E800421  bctrl
	ctx.lr = 0x82E5C984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5C984: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5C988: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E5C98C: 4BEFC04D  bl 0x82d589d8
	ctx.lr = 0x82E5C990;
	sub_82D589D8(ctx, base);
	// 82E5C990: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E5C994: 419A0020  beq cr6, 0x82e5c9b4
	if ctx.cr[6].eq {
	pc = 0x82E5C9B4; continue 'dispatch;
	}
	// 82E5C998: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82E5C99C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82E5C9A0: 7F1BE000  cmpw cr6, r27, r28
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E5C9A4: 4198FFC8  blt cr6, 0x82e5c96c
	if ctx.cr[6].lt {
	pc = 0x82E5C96C; continue 'dispatch;
	}
	// 82E5C9A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5C9AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5C9B0: 4BE4CA9C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82E5C9B4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5C9B8: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5C9BC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5C9C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5C9C4: 419A0008  beq cr6, 0x82e5c9cc
	if ctx.cr[6].eq {
	pc = 0x82E5C9CC; continue 'dispatch;
	}
	// 82E5C9C8: 3BABFFF8  addi r29, r11, -8
	ctx.r[29].s64 = ctx.r[11].s64 + -8;
	// 82E5C9CC: 897D0058  lbz r11, 0x58(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E5C9D0: 997F0060  stb r11, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82E5C9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5C9D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5C9DC: 4BE4CA70  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5C9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5C9E0 size=444
    let mut pc: u32 = 0x82E5C9E0;
    'dispatch: loop {
        match pc {
            0x82E5C9E0 => {
    //   block [0x82E5C9E0..0x82E5CB9C)
	// 82E5C9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5C9E4: 4BE4CA21  bl 0x82ca9404
	ctx.lr = 0x82E5C9E8;
	sub_82CA93D0(ctx, base);
	// 82E5C9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5C9EC: 81240078  lwz r9, 0x78(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E5C9F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5C9F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5C9F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5C9FC: 40990028  ble cr6, 0x82e5ca24
	if !ctx.cr[6].gt {
	pc = 0x82E5CA24; continue 'dispatch;
	}
	// 82E5CA00: 81040074  lwz r8, 0x74(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E5CA04: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82E5CA08: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CA0C: 2B071135  cmplwi cr6, r7, 0x1135
	ctx.cr[6].compare_u32(ctx.r[7].u32, 4405 as u32, &mut ctx.xer);
	// 82E5CA10: 419A0070  beq cr6, 0x82e5ca80
	if ctx.cr[6].eq {
	pc = 0x82E5CA80; continue 'dispatch;
	}
	// 82E5CA14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5CA18: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E5CA1C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5CA20: 4198FFE8  blt cr6, 0x82e5ca08
	if ctx.cr[6].lt {
	pc = 0x82E5CA08; continue 'dispatch;
	}
	// 82E5CA24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5CA28: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E5CA2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5CA30: 409A0010  bne cr6, 0x82e5ca40
	if !ctx.cr[6].eq {
	pc = 0x82E5CA40; continue 'dispatch;
	}
	// 82E5CA34: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5CA38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CA3C: 419A0158  beq cr6, 0x82e5cb94
	if ctx.cr[6].eq {
	pc = 0x82E5CB94; continue 'dispatch;
	}
	// 82E5CA40: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5CA44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5CA48: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CA4C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5CA50: 40990144  ble cr6, 0x82e5cb94
	if !ctx.cr[6].gt {
	pc = 0x82E5CB94; continue 'dispatch;
	}
	// 82E5CA54: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CA58: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CA5C: 80E70000  lwz r7, 0(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CA60: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E5CA64: 419A002C  beq cr6, 0x82e5ca90
	if ctx.cr[6].eq {
	pc = 0x82E5CA90; continue 'dispatch;
	}
	// 82E5CA68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5CA6C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5CA70: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5CA74: 4198FFE4  blt cr6, 0x82e5ca58
	if ctx.cr[6].lt {
	pc = 0x82E5CA58; continue 'dispatch;
	}
	// 82E5CA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5CA7C: 4BE4C9D8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E5CA80: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5CA84: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E5CA88: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E5CA8C: 4BFFFF9C  b 0x82e5ca28
	pc = 0x82E5CA28; continue 'dispatch;
	// 82E5CA90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CA94: 41980100  blt cr6, 0x82e5cb94
	if ctx.cr[6].lt {
	pc = 0x82E5CB94; continue 'dispatch;
	}
	// 82E5CA98: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CA9C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5CAA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5CAA4: 7F6B502E  lwzx r27, r11, r10
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5CAA8: 409A0008  bne cr6, 0x82e5cab0
	if !ctx.cr[6].eq {
	pc = 0x82E5CAB0; continue 'dispatch;
	}
	// 82E5CAAC: 83E40010  lwz r31, 0x10(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5CAB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E5CAB4: 48000DBD  bl 0x82e5d870
	ctx.lr = 0x82E5CAB8;
	sub_82E5D870(ctx, base);
	// 82E5CAB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 82E5CABC: 817DFFF0  lwz r11, -0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E5CAC0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E5CAC4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E5CAC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E5CACC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5CAD0: 80AAB5C8  lwz r5, -0x4a38(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19000 as u32) ) } as u64;
	// 82E5CAD4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CAD8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5CADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5CAE0: 4E800421  bctrl
	ctx.lr = 0x82E5CAE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5CAE4: 897D0039  lbz r11, 0x39(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(57 as u32) ) } as u64;
	// 82E5CAE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5CAEC: 419A0048  beq cr6, 0x82e5cb34
	if ctx.cr[6].eq {
	pc = 0x82E5CB34; continue 'dispatch;
	}
	// 82E5CAF0: 3BDD0020  addi r30, r29, 0x20
	ctx.r[30].s64 = ctx.r[29].s64 + 32;
	// 82E5CAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E5CAF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5CAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5CB00: 4BF03B89  bl 0x82d60688
	ctx.lr = 0x82E5CB04;
	sub_82D60688(ctx, base);
	// 82E5CB04: 38A3FFFF  addi r5, r3, -1
	ctx.r[5].s64 = ctx.r[3].s64 + -1;
	// 82E5CB08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5CB0C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E5CB10: 409A001C  bne cr6, 0x82e5cb2c
	if !ctx.cr[6].eq {
	pc = 0x82E5CB2C; continue 'dispatch;
	}
	// 82E5CB14: 387D0014  addi r3, r29, 0x14
	ctx.r[3].s64 = ctx.r[29].s64 + 20;
	// 82E5CB18: 4BF04151  bl 0x82d60c68
	ctx.lr = 0x82E5CB1C;
	sub_82D60C68(ctx, base);
	// 82E5CB1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5CB20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5CB24: 4BF04145  bl 0x82d60c68
	ctx.lr = 0x82E5CB28;
	sub_82D60C68(ctx, base);
	// 82E5CB28: 4800000C  b 0x82e5cb34
	pc = 0x82E5CB34; continue 'dispatch;
	// 82E5CB2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5CB30: 4B4090D1  bl 0x82265c00
	ctx.lr = 0x82E5CB34;
	sub_82265C00(ctx, base);
	// 82E5CB34: 813B0008  lwz r9, 8(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5CB3C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5CB40: 40990054  ble cr6, 0x82e5cb94
	if !ctx.cr[6].gt {
	pc = 0x82E5CB94; continue 'dispatch;
	}
	// 82E5CB44: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CB48: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CB4C: 7F08E040  cmplw cr6, r8, r28
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E5CB50: 419A001C  beq cr6, 0x82e5cb6c
	if ctx.cr[6].eq {
	pc = 0x82E5CB6C; continue 'dispatch;
	}
	// 82E5CB54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5CB58: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5CB5C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5CB60: 4198FFE8  blt cr6, 0x82e5cb48
	if ctx.cr[6].lt {
	pc = 0x82E5CB48; continue 'dispatch;
	}
	// 82E5CB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5CB68: 4BE4C8EC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E5CB6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CB70: 41980024  blt cr6, 0x82e5cb94
	if ctx.cr[6].lt {
	pc = 0x82E5CB94; continue 'dispatch;
	}
	// 82E5CB74: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CB78: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5CB7C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CB80: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5CB84: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E5CB88: 915B0008  stw r10, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5CB8C: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5CB90: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5CB94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5CB98: 4BE4C8BC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5CBA0 size=176
    let mut pc: u32 = 0x82E5CBA0;
    'dispatch: loop {
        match pc {
            0x82E5CBA0 => {
    //   block [0x82E5CBA0..0x82E5CC50)
	// 82E5CBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5CBA4: 4BE4C85D  bl 0x82ca9400
	ctx.lr = 0x82E5CBA8;
	sub_82CA93D0(ctx, base);
	// 82E5CBA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5CBAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E5CBB0: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5CBB4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E5CBB8: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82E5CBBC: 817C0030  lwz r11, 0x30(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5CBC0: 7FAB502E  lwzx r29, r11, r10
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5CBC4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CBC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CBCC: 40990048  ble cr6, 0x82e5cc14
	if !ctx.cr[6].gt {
	pc = 0x82E5CC14; continue 'dispatch;
	}
	// 82E5CBD0: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82E5CBD4: 3F608334  lis r27, -0x7ccc
	ctx.r[27].s64 = -2093744128;
	// 82E5CBD8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CBDC: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5CBE0: 80BBB5C8  lwz r5, -0x4a38(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-19000 as u32) ) } as u64;
	// 82E5CBE4: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5CBE8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CBEC: 5524003E  slwi r4, r9, 0
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E5CBF0: 80C90000  lwz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CBF4: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5CBF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5CBFC: 4E800421  bctrl
	ctx.lr = 0x82E5CC00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5CC00: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CC04: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5CC08: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5CC0C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CC10: 4198FFC8  blt cr6, 0x82e5cbd8
	if ctx.cr[6].lt {
	pc = 0x82E5CBD8; continue 'dispatch;
	}
	// 82E5CC14: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 82E5CC18: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CC1C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5CC20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CC24: 40980020  bge cr6, 0x82e5cc44
	if !ctx.cr[6].lt {
	pc = 0x82E5CC44; continue 'dispatch;
	}
	// 82E5CC28: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E5CC2C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E5CC30: 41990008  bgt cr6, 0x82e5cc38
	if ctx.cr[6].gt {
	pc = 0x82E5CC38; continue 'dispatch;
	}
	// 82E5CC34: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5CC38: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E5CC3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5CC40: 4BEFA2D1  bl 0x82d56f10
	ctx.lr = 0x82E5CC44;
	sub_82D56F10(ctx, base);
	// 82E5CC44: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82E5CC48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5CC4C: 4BE4C804  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5CC50 size=96
    let mut pc: u32 = 0x82E5CC50;
    'dispatch: loop {
        match pc {
            0x82E5CC50 => {
    //   block [0x82E5CC50..0x82E5CCB0)
	// 82E5CC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5CC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5CC58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5CC5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5CC60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CC64: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5CC68: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5CC6C: 3880006C  li r4, 0x6c
	ctx.r[4].s64 = 108;
	// 82E5CC70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5CC74: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5CC78: 4BEF85D1  bl 0x82d55248
	ctx.lr = 0x82E5CC7C;
	sub_82D55248(ctx, base);
	// 82E5CC7C: 3960006C  li r11, 0x6c
	ctx.r[11].s64 = 108;
	// 82E5CC80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5CC84: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5CC88: 4BFFFC09  bl 0x82e5c890
	ctx.lr = 0x82E5CC8C;
	sub_82E5C890(ctx, base);
	// 82E5CC8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5CC90: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5CC94: 409A0008  bne cr6, 0x82e5cc9c
	if !ctx.cr[6].eq {
	pc = 0x82E5CC9C; continue 'dispatch;
	}
	// 82E5CC98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5CC9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5CCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5CCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5CCA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5CCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5CCB0 size=64
    let mut pc: u32 = 0x82E5CCB0;
    'dispatch: loop {
        match pc {
            0x82E5CCB0 => {
    //   block [0x82E5CCB0..0x82E5CCF0)
	// 82E5CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5CCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5CCBC: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5CCC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5CCC4: 38ABCC50  addi r5, r11, -0x33b0
	ctx.r[5].s64 = ctx.r[11].s64 + -13232;
	// 82E5CCC8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5CCCC: 388A5820  addi r4, r10, 0x5820
	ctx.r[4].s64 = ctx.r[10].s64 + 22560;
	// 82E5CCD0: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5CCD4: 4BFDBFC5  bl 0x82e38c98
	ctx.lr = 0x82E5CCD8;
	sub_82E38C98(ctx, base);
	// 82E5CCD8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5CCDC: 906BB5C8  stw r3, -0x4a38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19000 as u32), ctx.r[3].u32 ) };
	// 82E5CCE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5CCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5CCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5CCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5CCF0 size=588
    let mut pc: u32 = 0x82E5CCF0;
    'dispatch: loop {
        match pc {
            0x82E5CCF0 => {
    //   block [0x82E5CCF0..0x82E5CF3C)
	// 82E5CCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5CCF4: 4BE4C6FD  bl 0x82ca93f0
	ctx.lr = 0x82E5CCF8;
	sub_82CA93D0(ctx, base);
	// 82E5CCF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5CCFC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82E5CD00: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82E5CD04: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82E5CD08: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82E5CD0C: 38970028  addi r4, r23, 0x28
	ctx.r[4].s64 = ctx.r[23].s64 + 40;
	// 82E5CD10: 409A0008  bne cr6, 0x82e5cd18
	if !ctx.cr[6].eq {
	pc = 0x82E5CD18; continue 'dispatch;
	}
	// 82E5CD14: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E5CD18: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E5CD1C: 4BF1C93D  bl 0x82d79658
	ctx.lr = 0x82E5CD20;
	sub_82D79658(ctx, base);
	// 82E5CD20: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82E5CD24: 3897002C  addi r4, r23, 0x2c
	ctx.r[4].s64 = ctx.r[23].s64 + 44;
	// 82E5CD28: 409A0008  bne cr6, 0x82e5cd30
	if !ctx.cr[6].eq {
	pc = 0x82E5CD30; continue 'dispatch;
	}
	// 82E5CD2C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E5CD30: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E5CD34: 4BF1CAE5  bl 0x82d79818
	ctx.lr = 0x82E5CD38;
	sub_82D79818(ctx, base);
	// 82E5CD38: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CD3C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5CD40: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5CD44: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E5CD48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5CD4C: 419A001C  beq cr6, 0x82e5cd68
	if ctx.cr[6].eq {
	pc = 0x82E5CD68; continue 'dispatch;
	}
	// 82E5CD50: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E5CD54: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5CD58: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82E5CD5C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CD60: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E5CD64: 48000010  b 0x82e5cd74
	pc = 0x82E5CD74; continue 'dispatch;
	// 82E5CD68: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E5CD6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E5CD70: 4BEF82E1  bl 0x82d55050
	ctx.lr = 0x82E5CD74;
	sub_82D55050(ctx, base);
	// 82E5CD74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5CD78: 419A001C  beq cr6, 0x82e5cd94
	if ctx.cr[6].eq {
	pc = 0x82E5CD94; continue 'dispatch;
	}
	// 82E5CD7C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E5CD80: 93030004  stw r24, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82E5CD84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E5CD88: 93030008  stw r24, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82E5CD8C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E5CD90: 48000008  b 0x82e5cd98
	pc = 0x82E5CD98; continue 'dispatch;
	// 82E5CD94: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5CD98: 3BF70030  addi r31, r23, 0x30
	ctx.r[31].s64 = ctx.r[23].s64 + 48;
	// 82E5CD9C: 92DE0000  stw r22, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82E5CDA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5CDA4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CDA8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5CDAC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CDB0: 409A0010  bne cr6, 0x82e5cdc0
	if !ctx.cr[6].eq {
	pc = 0x82E5CDC0; continue 'dispatch;
	}
	// 82E5CDB4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5CDB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5CDBC: 4BEFA1DD  bl 0x82d56f98
	ctx.lr = 0x82E5CDC0;
	sub_82D56F98(ctx, base);
	// 82E5CDC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CDC4: 3B760028  addi r27, r22, 0x28
	ctx.r[27].s64 = ctx.r[22].s64 + 40;
	// 82E5CDC8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CDCC: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E5CDD0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5CDD4: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E5CDD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CDDC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5CDE0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5CDE4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CDE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CDEC: 40990070  ble cr6, 0x82e5ce5c
	if !ctx.cr[6].gt {
	pc = 0x82E5CE5C; continue 'dispatch;
	}
	// 82E5CDF0: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82E5CDF4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CDF8: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5CDFC: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E5CE00: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5CE04: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CE08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CE0C: 4099003C  ble cr6, 0x82e5ce48
	if !ctx.cr[6].gt {
	pc = 0x82E5CE48; continue 'dispatch;
	}
	// 82E5CE10: 3B970028  addi r28, r23, 0x28
	ctx.r[28].s64 = ctx.r[23].s64 + 40;
	// 82E5CE14: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82E5CE18: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CE1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5CE20: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CE24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CE28: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5CE2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5CE30: 4E800421  bctrl
	ctx.lr = 0x82E5CE34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5CE34: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CE38: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5CE3C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5CE40: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CE44: 4198FFD4  blt cr6, 0x82e5ce18
	if ctx.cr[6].lt {
	pc = 0x82E5CE18; continue 'dispatch;
	}
	// 82E5CE48: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CE4C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5CE50: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82E5CE54: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CE58: 4198FF9C  blt cr6, 0x82e5cdf4
	if ctx.cr[6].lt {
	pc = 0x82E5CDF4; continue 'dispatch;
	}
	// 82E5CE5C: 3B560034  addi r26, r22, 0x34
	ctx.r[26].s64 = ctx.r[22].s64 + 52;
	// 82E5CE60: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82E5CE64: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CE68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CE6C: 40990070  ble cr6, 0x82e5cedc
	if !ctx.cr[6].gt {
	pc = 0x82E5CEDC; continue 'dispatch;
	}
	// 82E5CE70: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82E5CE74: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CE78: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5CE7C: 7D7B582E  lwzx r11, r27, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5CE80: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5CE84: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CE88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CE8C: 4099003C  ble cr6, 0x82e5cec8
	if !ctx.cr[6].gt {
	pc = 0x82E5CEC8; continue 'dispatch;
	}
	// 82E5CE90: 3B970028  addi r28, r23, 0x28
	ctx.r[28].s64 = ctx.r[23].s64 + 40;
	// 82E5CE94: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82E5CE98: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CE9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5CEA0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CEA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CEA8: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5CEAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5CEB0: 4E800421  bctrl
	ctx.lr = 0x82E5CEB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5CEB4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CEB8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5CEBC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5CEC0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CEC4: 4198FFD4  blt cr6, 0x82e5ce98
	if ctx.cr[6].lt {
	pc = 0x82E5CE98; continue 'dispatch;
	}
	// 82E5CEC8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CECC: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5CED0: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E5CED4: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CED8: 4198FF9C  blt cr6, 0x82e5ce74
	if ctx.cr[6].lt {
	pc = 0x82E5CE74; continue 'dispatch;
	}
	// 82E5CEDC: 81760020  lwz r11, 0x20(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E5CEE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CEE4: 419A0050  beq cr6, 0x82e5cf34
	if ctx.cr[6].eq {
	pc = 0x82E5CF34; continue 'dispatch;
	}
	// 82E5CEE8: 3B8B004C  addi r28, r11, 0x4c
	ctx.r[28].s64 = ctx.r[11].s64 + 76;
	// 82E5CEEC: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82E5CEF0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CEF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5CEF8: 4099003C  ble cr6, 0x82e5cf34
	if !ctx.cr[6].gt {
	pc = 0x82E5CF34; continue 'dispatch;
	}
	// 82E5CEFC: 3BB70028  addi r29, r23, 0x28
	ctx.r[29].s64 = ctx.r[23].s64 + 40;
	// 82E5CF00: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82E5CF04: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CF08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5CF0C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CF10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CF14: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5CF18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5CF1C: 4E800421  bctrl
	ctx.lr = 0x82E5CF20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5CF20: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5CF24: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5CF28: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5CF2C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5CF30: 4198FFD4  blt cr6, 0x82e5cf04
	if ctx.cr[6].lt {
	pc = 0x82E5CF04; continue 'dispatch;
	}
	// 82E5CF34: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E5CF38: 4BE4C508  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5CF40 size=100
    let mut pc: u32 = 0x82E5CF40;
    'dispatch: loop {
        match pc {
            0x82E5CF40 => {
    //   block [0x82E5CF40..0x82E5CFA4)
	// 82E5CF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5CF44: 4BE4C4C5  bl 0x82ca9408
	ctx.lr = 0x82E5CF48;
	sub_82CA93D0(ctx, base);
	// 82E5CF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5CF4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5CF50: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5CF54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5CF58: 419A0044  beq cr6, 0x82e5cf9c
	if ctx.cr[6].eq {
	pc = 0x82E5CF9C; continue 'dispatch;
	}
	// 82E5CF5C: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5CF60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5CF64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5CF68: 40990034  ble cr6, 0x82e5cf9c
	if !ctx.cr[6].gt {
	pc = 0x82E5CF9C; continue 'dispatch;
	}
	// 82E5CF6C: 3B9DFFF8  addi r28, r29, -8
	ctx.r[28].s64 = ctx.r[29].s64 + -8;
	// 82E5CF70: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5CF74: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5CF78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5CF7C: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5CF80: 4BFFFD71  bl 0x82e5ccf0
	ctx.lr = 0x82E5CF84;
	sub_82E5CCF0(ctx, base);
	// 82E5CF84: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5CF88: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5CF8C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5CF90: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5CF94: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E5CF98: 4198FFDC  blt cr6, 0x82e5cf74
	if ctx.cr[6].lt {
	pc = 0x82E5CF74; continue 'dispatch;
	}
	// 82E5CF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5CFA0: 4BE4C4B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5CFA8 size=8
    let mut pc: u32 = 0x82E5CFA8;
    'dispatch: loop {
        match pc {
            0x82E5CFA8 => {
    //   block [0x82E5CFA8..0x82E5CFB0)
	// 82E5CFA8: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5CFAC: 4BFFFD44  b 0x82e5ccf0
	sub_82E5CCF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5CFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E5CFB0 size=1672
    let mut pc: u32 = 0x82E5CFB0;
    'dispatch: loop {
        match pc {
            0x82E5CFB0 => {
    //   block [0x82E5CFB0..0x82E5D638)
	// 82E5CFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5CFB4: 4BE4C42D  bl 0x82ca93e0
	ctx.lr = 0x82E5CFB8;
	sub_82CA93D0(ctx, base);
	// 82E5CFB8: 9421FCB0  stwu r1, -0x350(r1)
	ea = ctx.r[1].u32.wrapping_add(-848 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5CFBC: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 82E5CFC0: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82E5CFC4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E5CFC8: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82E5CFCC: 81340078  lwz r9, 0x78(r20)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E5CFD0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5CFD4: 40990028  ble cr6, 0x82e5cffc
	if !ctx.cr[6].gt {
	pc = 0x82E5CFFC; continue 'dispatch;
	}
	// 82E5CFD8: 81140074  lwz r8, 0x74(r20)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E5CFDC: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82E5CFE0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5CFE4: 2B071135  cmplwi cr6, r7, 0x1135
	ctx.cr[6].compare_u32(ctx.r[7].u32, 4405 as u32, &mut ctx.xer);
	// 82E5CFE8: 419A00FC  beq cr6, 0x82e5d0e4
	if ctx.cr[6].eq {
	pc = 0x82E5D0E4; continue 'dispatch;
	}
	// 82E5CFEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5CFF0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E5CFF4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5CFF8: 4198FFE8  blt cr6, 0x82e5cfe0
	if ctx.cr[6].lt {
	pc = 0x82E5CFE0; continue 'dispatch;
	}
	// 82E5CFFC: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82E5D000: 5576003E  slwi r22, r11, 0
	ctx.r[22].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 82E5D004: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82E5D008: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5D00C: 40990024  ble cr6, 0x82e5d030
	if !ctx.cr[6].gt {
	pc = 0x82E5D030; continue 'dispatch;
	}
	// 82E5D010: 81540074  lwz r10, 0x74(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E5D014: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D018: 2B081135  cmplwi cr6, r8, 0x1135
	ctx.cr[6].compare_u32(ctx.r[8].u32, 4405 as u32, &mut ctx.xer);
	// 82E5D01C: 419A00D8  beq cr6, 0x82e5d0f4
	if ctx.cr[6].eq {
	pc = 0x82E5D0F4; continue 'dispatch;
	}
	// 82E5D020: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5D024: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E5D028: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5D02C: 4198FFE8  blt cr6, 0x82e5d014
	if ctx.cr[6].lt {
	pc = 0x82E5D014; continue 'dispatch;
	}
	// 82E5D030: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82E5D034: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E5D038: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5D03C: 419A000C  beq cr6, 0x82e5d048
	if ctx.cr[6].eq {
	pc = 0x82E5D048; continue 'dispatch;
	}
	// 82E5D040: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82E5D044: 419A05EC  beq cr6, 0x82e5d630
	if ctx.cr[6].eq {
	pc = 0x82E5D630; continue 'dispatch;
	}
	// 82E5D048: 83F40010  lwz r31, 0x10(r20)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5D04C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E5D050: 409A000C  bne cr6, 0x82e5d05c
	if !ctx.cr[6].eq {
	pc = 0x82E5D05C; continue 'dispatch;
	}
	// 82E5D054: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82E5D058: 419A05D8  beq cr6, 0x82e5d630
	if ctx.cr[6].eq {
	pc = 0x82E5D630; continue 'dispatch;
	}
	// 82E5D05C: 8178FFE8  lwz r11, -0x18(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82E5D060: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D064: 409A0014  bne cr6, 0x82e5d078
	if !ctx.cr[6].eq {
	pc = 0x82E5D078; continue 'dispatch;
	}
	// 82E5D068: 8178FFEC  lwz r11, -0x14(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82E5D06C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D070: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5D074: 419A0008  beq cr6, 0x82e5d07c
	if ctx.cr[6].eq {
	pc = 0x82E5D07C; continue 'dispatch;
	}
	// 82E5D078: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82E5D07C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E5D080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D084: 419A0020  beq cr6, 0x82e5d0a4
	if ctx.cr[6].eq {
	pc = 0x82E5D0A4; continue 'dispatch;
	}
	// 82E5D088: 38A01234  li r5, 0x1234
	ctx.r[5].s64 = 4660;
	// 82E5D08C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82E5D090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5D094: 4BFECF2D  bl 0x82e49fc0
	ctx.lr = 0x82E5D098;
	sub_82E49FC0(ctx, base);
	// 82E5D098: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E5D09C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D0A0: 409A0590  bne cr6, 0x82e5d630
	if !ctx.cr[6].eq {
	pc = 0x82E5D630; continue 'dispatch;
	}
	// 82E5D0A4: 8138000C  lwz r9, 0xc(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5D0A8: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82E5D0AC: 81140008  lwz r8, 8(r20)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5D0B0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5D0B4: 4099057C  ble cr6, 0x82e5d630
	if !ctx.cr[6].gt {
	pc = 0x82E5D630; continue 'dispatch;
	}
	// 82E5D0B8: 81580008  lwz r10, 8(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5D0BC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D0C0: 80E70000  lwz r7, 0(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D0C4: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E5D0C8: 419A0034  beq cr6, 0x82e5d0fc
	if ctx.cr[6].eq {
	pc = 0x82E5D0FC; continue 'dispatch;
	}
	// 82E5D0CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5D0D0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5D0D4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5D0D8: 4198FFE4  blt cr6, 0x82e5d0bc
	if ctx.cr[6].lt {
	pc = 0x82E5D0BC; continue 'dispatch;
	}
	// 82E5D0DC: 38210350  addi r1, r1, 0x350
	ctx.r[1].s64 = ctx.r[1].s64 + 848;
	// 82E5D0E0: 4BE4C350  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
	// 82E5D0E4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5D0E8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E5D0EC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E5D0F0: 4BFFFF10  b 0x82e5d000
	pc = 0x82E5D000; continue 'dispatch;
	// 82E5D0F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5D0F8: 4BFFFF3C  b 0x82e5d034
	pc = 0x82E5D034; continue 'dispatch;
	// 82E5D0FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5D100: 41980530  blt cr6, 0x82e5d630
	if ctx.cr[6].lt {
	pc = 0x82E5D630; continue 'dispatch;
	}
	// 82E5D104: 81580008  lwz r10, 8(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5D108: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5D10C: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 82E5D110: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82E5D114: 7EAB502E  lwzx r21, r11, r10
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5D118: 409A0008  bne cr6, 0x82e5d120
	if !ctx.cr[6].eq {
	pc = 0x82E5D120; continue 'dispatch;
	}
	// 82E5D11C: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82E5D120: 8978003A  lbz r11, 0x3a(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(58 as u32) ) } as u64;
	// 82E5D124: 7E579378  mr r23, r18
	ctx.r[23].u64 = ctx.r[18].u64;
	// 82E5D128: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D12C: 419A003C  beq cr6, 0x82e5d168
	if ctx.cr[6].eq {
	pc = 0x82E5D168; continue 'dispatch;
	}
	// 82E5D130: 3BF8002C  addi r31, r24, 0x2c
	ctx.r[31].s64 = ctx.r[24].s64 + 44;
	// 82E5D134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E5D138: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5D13C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5D140: 4BF03549  bl 0x82d60688
	ctx.lr = 0x82E5D144;
	sub_82D60688(ctx, base);
	// 82E5D144: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 82E5D148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5D14C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82E5D150: 557707FE  clrlwi r23, r11, 0x1f
	ctx.r[23].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82E5D154: 409A0014  bne cr6, 0x82e5d168
	if !ctx.cr[6].eq {
	pc = 0x82E5D168; continue 'dispatch;
	}
	// 82E5D158: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E5D15C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5D160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5D164: 4B408A9D  bl 0x82265c00
	ctx.lr = 0x82E5D168;
	sub_82265C00(ctx, base);
	// 82E5D168: 89780039  lbz r11, 0x39(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(57 as u32) ) } as u64;
	// 82E5D16C: 7E599378  mr r25, r18
	ctx.r[25].u64 = ctx.r[18].u64;
	// 82E5D170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D174: 419A0018  beq cr6, 0x82e5d18c
	if ctx.cr[6].eq {
	pc = 0x82E5D18C; continue 'dispatch;
	}
	// 82E5D178: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E5D17C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5D180: 38780014  addi r3, r24, 0x14
	ctx.r[3].s64 = ctx.r[24].s64 + 20;
	// 82E5D184: 4BF03505  bl 0x82d60688
	ctx.lr = 0x82E5D188;
	sub_82D60688(ctx, base);
	// 82E5D188: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E5D18C: 3961006C  addi r11, r1, 0x6c
	ctx.r[11].s64 = ctx.r[1].s64 + 108;
	// 82E5D190: 92410064  stw r18, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[18].u32 ) };
	// 82E5D194: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 82E5D198: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82E5D19C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E5D1A0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E5D1A4: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82E5D1A8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E5D1AC: 409A0264  bne cr6, 0x82e5d410
	if !ctx.cr[6].eq {
	pc = 0x82E5D410; continue 'dispatch;
	}
	// 82E5D1B0: 56EB063E  clrlwi r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	// 82E5D1B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D1B8: 409A0258  bne cr6, 0x82e5d410
	if !ctx.cr[6].eq {
	pc = 0x82E5D410; continue 'dispatch;
	}
	// 82E5D1BC: 8978003B  lbz r11, 0x3b(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(59 as u32) ) } as u64;
	// 82E5D1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D1C4: 419A024C  beq cr6, 0x82e5d410
	if ctx.cr[6].eq {
	pc = 0x82E5D410; continue 'dispatch;
	}
	// 82E5D1C8: 8978003C  lbz r11, 0x3c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E5D1CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D1D0: 419A0240  beq cr6, 0x82e5d410
	if ctx.cr[6].eq {
	pc = 0x82E5D410; continue 'dispatch;
	}
	// 82E5D1D4: 38A01136  li r5, 0x1136
	ctx.r[5].s64 = 4406;
	// 82E5D1D8: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82E5D1DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5D1E0: 4BFECDE1  bl 0x82e49fc0
	ctx.lr = 0x82E5D1E4;
	sub_82E49FC0(ctx, base);
	// 82E5D1E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E5D1E8: 38A01131  li r5, 0x1131
	ctx.r[5].s64 = 4401;
	// 82E5D1EC: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82E5D1F0: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82E5D1F4: EBEB0000  ld r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E5D1F8: 4BFECDC9  bl 0x82e49fc0
	ctx.lr = 0x82E5D1FC;
	sub_82E49FC0(ctx, base);
	// 82E5D1FC: EBC30000  ld r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82E5D200: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5D204: 409A0154  bne cr6, 0x82e5d358
	if !ctx.cr[6].eq {
	pc = 0x82E5D358; continue 'dispatch;
	}
	// 82E5D208: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5D20C: 409A014C  bne cr6, 0x82e5d358
	if !ctx.cr[6].eq {
	pc = 0x82E5D358; continue 'dispatch;
	}
	// 82E5D210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5D214: 48008D15  bl 0x82e65f28
	ctx.lr = 0x82E5D218;
	sub_82E65F28(ctx, base);
	// 82E5D218: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5D21C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E5D220: 48008D19  bl 0x82e65f38
	ctx.lr = 0x82E5D224;
	sub_82E65F38(ctx, base);
	// 82E5D224: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E5D228: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5D22C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E5D230: 4800A719  bl 0x82e67948
	ctx.lr = 0x82E5D234;
	sub_82E67948(ctx, base);
	// 82E5D234: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E5D238: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82E5D23C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E5D240: 419800B4  blt cr6, 0x82e5d2f4
	if ctx.cr[6].lt {
	pc = 0x82E5D2F4; continue 'dispatch;
	}
	// 82E5D244: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5D248: 577F103A  slwi r31, r27, 2
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E5D24C: 3BCB7778  addi r30, r11, 0x7778
	ctx.r[30].s64 = ctx.r[11].s64 + 30584;
	// 82E5D250: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5D254: 3F808333  lis r28, -0x7ccd
	ctx.r[28].s64 = -2093809664;
	// 82E5D258: 3BAB75D0  addi r29, r11, 0x75d0
	ctx.r[29].s64 = ctx.r[11].s64 + 30160;
	// 82E5D25C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E5D260: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5D264: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5D268: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82E5D26C: 409A0078  bne cr6, 0x82e5d2e4
	if !ctx.cr[6].eq {
	pc = 0x82E5D2E4; continue 'dispatch;
	}
	// 82E5D270: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5D274: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5D278: 409A006C  bne cr6, 0x82e5d2e4
	if !ctx.cr[6].eq {
	pc = 0x82E5D2E4; continue 'dispatch;
	}
	// 82E5D27C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E5D280: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82E5D284: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E5D288: 4BEFA761  bl 0x82d579e8
	ctx.lr = 0x82E5D28C;
	sub_82D579E8(ctx, base);
	// 82E5D28C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E5D290: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5D294: 4BEFAB5D  bl 0x82d57df0
	ctx.lr = 0x82E5D298;
	sub_82D57DF0(ctx, base);
	// 82E5D298: 807C7630  lwz r3, 0x7630(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E5D29C: 3900010F  li r8, 0x10f
	ctx.r[8].s64 = 271;
	// 82E5D2A0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E5D2A4: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82E5D2A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E5D2AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D2B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5D2B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5D2B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5D2BC: 4E800421  bctrl
	ctx.lr = 0x82E5D2C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5D2C0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E5D2C4: 4BEFB16D  bl 0x82d58430
	ctx.lr = 0x82E5D2C8;
	sub_82D58430(ctx, base);
	// 82E5D2C8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E5D2CC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E5D2D0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5D2D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E5D2D8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E5D2DC: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5D2E0: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5D2E4: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82E5D2E8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E5D2EC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E5D2F0: 4098FF6C  bge cr6, 0x82e5d25c
	if !ctx.cr[6].lt {
	pc = 0x82E5D25C; continue 'dispatch;
	}
	// 82E5D2F4: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82E5D2F8: 419A0050  beq cr6, 0x82e5d348
	if ctx.cr[6].eq {
	pc = 0x82E5D348; continue 'dispatch;
	}
	// 82E5D2FC: A1760004  lhz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5D300: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D304: 419A0034  beq cr6, 0x82e5d338
	if ctx.cr[6].eq {
	pc = 0x82E5D338; continue 'dispatch;
	}
	// 82E5D308: A1760006  lhz r11, 6(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[22].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E5D30C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E5D310: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E5D314: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5D318: B1760006  sth r11, 6(r22)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[22].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E5D31C: 409A001C  bne cr6, 0x82e5d338
	if !ctx.cr[6].eq {
	pc = 0x82E5D338; continue 'dispatch;
	}
	// 82E5D320: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D324: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5D328: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E5D32C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5D334: 4E800421  bctrl
	ctx.lr = 0x82E5D338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5D338: 38A01135  li r5, 0x1135
	ctx.r[5].s64 = 4405;
	// 82E5D33C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82E5D340: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82E5D344: 4BF23185  bl 0x82d804c8
	ctx.lr = 0x82E5D348;
	sub_82D804C8(ctx, base);
	// 82E5D348: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E5D34C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E5D350: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82E5D354: 480000BC  b 0x82e5d410
	pc = 0x82E5D410; continue 'dispatch;
	// 82E5D358: 7FEB0034  cntlzw r11, r31
	ctx.r[11].u64 = if ctx.r[31].u32 == 0 { 32 } else { ctx.r[31].u32.leading_zeros() as u64 };
	// 82E5D35C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5D360: 5573DFFE  rlwinm r19, r11, 0x1b, 0x1f, 0x1f
	ctx.r[19].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E5D364: 419A0014  beq cr6, 0x82e5d378
	if ctx.cr[6].eq {
	pc = 0x82E5D378; continue 'dispatch;
	}
	// 82E5D368: 38A01131  li r5, 0x1131
	ctx.r[5].s64 = 4401;
	// 82E5D36C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82E5D370: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82E5D374: 4BF23155  bl 0x82d804c8
	ctx.lr = 0x82E5D378;
	sub_82D804C8(ctx, base);
	// 82E5D378: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D37C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5D380: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E5D384: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E5D388: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5D38C: 4BEF7EBD  bl 0x82d55248
	ctx.lr = 0x82E5D390;
	sub_82D55248(ctx, base);
	// 82E5D390: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E5D394: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5D398: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5D39C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5D3A0: 409A0008  bne cr6, 0x82e5d3a8
	if !ctx.cr[6].eq {
	pc = 0x82E5D3A8; continue 'dispatch;
	}
	// 82E5D3A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5D3A8: 4BFE6D41  bl 0x82e440e8
	ctx.lr = 0x82E5D3AC;
	sub_82E440E8(ctx, base);
	// 82E5D3AC: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82E5D3B0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82E5D3B4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5D638 size=148
    let mut pc: u32 = 0x82E5D638;
    'dispatch: loop {
        match pc {
            0x82E5D638 => {
    //   block [0x82E5D638..0x82E5D6CC)
	// 82E5D638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5D63C: 4BE4BDD1  bl 0x82ca940c
	ctx.lr = 0x82E5D640;
	sub_82CA93D0(ctx, base);
	// 82E5D640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5D644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5D648: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E5D64C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5D650: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82E5D654: 409A0008  bne cr6, 0x82e5d65c
	if !ctx.cr[6].eq {
	pc = 0x82E5D65C; continue 'dispatch;
	}
	// 82E5D658: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5D65C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D660: 57BE103A  slwi r30, r29, 2
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E5D664: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5D668: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D66C: 4BF1AAED  bl 0x82d78158
	ctx.lr = 0x82E5D670;
	sub_82D78158(ctx, base);
	// 82E5D670: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D674: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 82E5D678: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5D67C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D680: 4BF1ABF9  bl 0x82d78278
	ctx.lr = 0x82E5D684;
	sub_82D78278(ctx, base);
	// 82E5D684: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5D688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5D68C: 4BFFF515  bl 0x82e5cba0
	ctx.lr = 0x82E5D690;
	sub_82E5CBA0(ctx, base);
	// 82E5D690: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D694: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5D698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5D69C: 419A000C  beq cr6, 0x82e5d6a8
	if ctx.cr[6].eq {
	pc = 0x82E5D6A8; continue 'dispatch;
	}
	// 82E5D6A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5D6A4: 4800028D  bl 0x82e5d930
	ctx.lr = 0x82E5D6A8;
	sub_82E5D930(ctx, base);
	// 82E5D6A8: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5D6AC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D6B0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5D6B4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5D6B8: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82E5D6BC: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5D6C0: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82E5D6C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5D6C8: 4BE4BD94  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5D6D0 size=332
    let mut pc: u32 = 0x82E5D6D0;
    'dispatch: loop {
        match pc {
            0x82E5D6D0 => {
    //   block [0x82E5D6D0..0x82E5D81C)
	// 82E5D6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5D6D4: 4BE4BD31  bl 0x82ca9404
	ctx.lr = 0x82E5D6D8;
	sub_82CA93D0(ctx, base);
	// 82E5D6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5D6DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5D6E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5D6E4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5D6E8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5D6EC: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5D6F0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5D6F4: 394A7764  addi r10, r10, 0x7764
	ctx.r[10].s64 = ctx.r[10].s64 + 30564;
	// 82E5D6F8: 39297744  addi r9, r9, 0x7744
	ctx.r[9].s64 = ctx.r[9].s64 + 30532;
	// 82E5D6FC: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82E5D700: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5D704: 3908771C  addi r8, r8, 0x771c
	ctx.r[8].s64 = ctx.r[8].s64 + 30492;
	// 82E5D708: 396B7734  addi r11, r11, 0x7734
	ctx.r[11].s64 = ctx.r[11].s64 + 30516;
	// 82E5D70C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5D710: 38E7770C  addi r7, r7, 0x770c
	ctx.r[7].s64 = ctx.r[7].s64 + 30476;
	// 82E5D714: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5D718: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82E5D71C: 3B7F002C  addi r27, r31, 0x2c
	ctx.r[27].s64 = ctx.r[31].s64 + 44;
	// 82E5D720: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 82E5D724: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5D728: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E5D72C: 90FF002C  stw r7, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82E5D730: 41980080  blt cr6, 0x82e5d7b0
	if ctx.cr[6].lt {
	pc = 0x82E5D7B0; continue 'dispatch;
	}
	// 82E5D734: 57DD103A  slwi r29, r30, 2
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E5D738: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D73C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5D740: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5D744: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D748: 4BF1AA11  bl 0x82d78158
	ctx.lr = 0x82E5D74C;
	sub_82D78158(ctx, base);
	// 82E5D74C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D750: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E5D754: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5D758: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D75C: 4BF1AB1D  bl 0x82d78278
	ctx.lr = 0x82E5D760;
	sub_82D78278(ctx, base);
	// 82E5D760: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5D764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5D768: 4BFFF439  bl 0x82e5cba0
	ctx.lr = 0x82E5D76C;
	sub_82E5CBA0(ctx, base);
	// 82E5D76C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D770: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5D774: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5D778: 419A000C  beq cr6, 0x82e5d784
	if ctx.cr[6].eq {
	pc = 0x82E5D784; continue 'dispatch;
	}
	// 82E5D77C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5D780: 480001B1  bl 0x82e5d930
	ctx.lr = 0x82E5D784;
	sub_82E5D930(ctx, base);
	// 82E5D784: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5D788: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5D78C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D790: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5D794: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5D798: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5D79C: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82E5D7A0: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5D7A4: 7D5D592E  stwx r10, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5D7A8: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82E5D7AC: 4098FF8C  bge cr6, 0x82e5d738
	if !ctx.cr[6].lt {
	pc = 0x82E5D738; continue 'dispatch;
	}
	// 82E5D7B0: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82E5D7B4: 4B409755  bl 0x82266f08
	ctx.lr = 0x82E5D7B8;
	sub_82266F08(ctx, base);
	// 82E5D7B8: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82E5D7BC: 4B40974D  bl 0x82266f08
	ctx.lr = 0x82E5D7C0;
	sub_82266F08(ctx, base);
	// 82E5D7C0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82E5D7C4: 4B409745  bl 0x82266f08
	ctx.lr = 0x82E5D7C8;
	sub_82266F08(ctx, base);
	// 82E5D7C8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5D7CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5D7D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5D7D4: 409A0020  bne cr6, 0x82e5d7f4
	if !ctx.cr[6].eq {
	pc = 0x82E5D7F4; continue 'dispatch;
	}
	// 82E5D7D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D7DC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5D7E0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5D7E4: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D7E8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5D7EC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5D7F0: 4BEF7AD9  bl 0x82d552c8
	ctx.lr = 0x82E5D7F4;
	sub_82D552C8(ctx, base);
	// 82E5D7F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5D7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5D7FC: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5D800: 394A5D40  addi r10, r10, 0x5d40
	ctx.r[10].s64 = ctx.r[10].s64 + 23872;
	// 82E5D804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5D808: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5D80C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5D810: 48008499  bl 0x82e65ca8
	ctx.lr = 0x82E5D814;
	sub_82E65CA8(ctx, base);
	// 82E5D814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5D818: 4BE4BC3C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D820 size=20
    let mut pc: u32 = 0x82E5D820;
    'dispatch: loop {
        match pc {
            0x82E5D820 => {
    //   block [0x82E5D820..0x82E5D834)
	// 82E5D820: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5D824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5D828: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5D82C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5D830: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D834(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D834 size=40
    let mut pc: u32 = 0x82E5D834;
    'dispatch: loop {
        match pc {
            0x82E5D834 => {
    //   block [0x82E5D834..0x82E5D85C)
	// 82E5D834: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5D838: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D83C: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D840: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E5D844: 419A0018  beq cr6, 0x82e5d85c
	if ctx.cr[6].eq {
		sub_82E5D85C(ctx, base);
		return;
	}
	// 82E5D848: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5D84C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5D850: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5D854: 4198FFE4  blt cr6, 0x82e5d838
	if ctx.cr[6].lt {
	pc = 0x82E5D838; continue 'dispatch;
	}
	// 82E5D858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D85C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D85C size=8
    let mut pc: u32 = 0x82E5D85C;
    'dispatch: loop {
        match pc {
            0x82E5D85C => {
    //   block [0x82E5D85C..0x82E5D864)
	// 82E5D85C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5D860: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D864 size=8
    let mut pc: u32 = 0x82E5D864;
    'dispatch: loop {
        match pc {
            0x82E5D864 => {
    //   block [0x82E5D864..0x82E5D86C)
	// 82E5D864: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5D868: 4BFFFDD0  b 0x82e5d638
	sub_82E5D638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5D870 size=140
    let mut pc: u32 = 0x82E5D870;
    'dispatch: loop {
        match pc {
            0x82E5D870 => {
    //   block [0x82E5D870..0x82E5D8FC)
	// 82E5D870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5D874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5D878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5D87C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5D880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5D884: 81240078  lwz r9, 0x78(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E5D888: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5D88C: 40990024  ble cr6, 0x82e5d8b0
	if !ctx.cr[6].gt {
	pc = 0x82E5D8B0; continue 'dispatch;
	}
	// 82E5D890: 81440074  lwz r10, 0x74(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E5D894: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D898: 2B081134  cmplwi cr6, r8, 0x1134
	ctx.cr[6].compare_u32(ctx.r[8].u32, 4404 as u32, &mut ctx.xer);
	// 82E5D89C: 419A0044  beq cr6, 0x82e5d8e0
	if ctx.cr[6].eq {
	pc = 0x82E5D8E0; continue 'dispatch;
	}
	// 82E5D8A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5D8A4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E5D8A8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5D8AC: 4198FFE8  blt cr6, 0x82e5d894
	if ctx.cr[6].lt {
	pc = 0x82E5D894; continue 'dispatch;
	}
	// 82E5D8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5D8B4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E5D8B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5D8BC: 419A002C  beq cr6, 0x82e5d8e8
	if ctx.cr[6].eq {
	pc = 0x82E5D8E8; continue 'dispatch;
	}
	// 82E5D8C0: 38A01134  li r5, 0x1134
	ctx.r[5].s64 = 4404;
	// 82E5D8C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5D8C8: 4BFEC6F9  bl 0x82e49fc0
	ctx.lr = 0x82E5D8CC;
	sub_82E49FC0(ctx, base);
	// 82E5D8CC: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82E5D8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5D8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5D8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5D8DC: 4E800020  blr
	return;
	// 82E5D8E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5D8E4: 4BFFFFD0  b 0x82e5d8b4
	pc = 0x82E5D8B4; continue 'dispatch;
	// 82E5D8E8: 38640010  addi r3, r4, 0x10
	ctx.r[3].s64 = ctx.r[4].s64 + 16;
	// 82E5D8EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5D8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5D8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5D8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D900 size=12
    let mut pc: u32 = 0x82E5D900;
    'dispatch: loop {
        match pc {
            0x82E5D900 => {
    //   block [0x82E5D900..0x82E5D90C)
	// 82E5D900: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5D904: 806BB5C8  lwz r3, -0x4a38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19000 as u32) ) } as u64;
	// 82E5D908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D910 size=8
    let mut pc: u32 = 0x82E5D910;
    'dispatch: loop {
        match pc {
            0x82E5D910 => {
    //   block [0x82E5D910..0x82E5D918)
	// 82E5D910: 3863FFD4  addi r3, r3, -0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + -44;
	// 82E5D914: 480000DC  b 0x82e5d9f0
	sub_82E5D9F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D918 size=8
    let mut pc: u32 = 0x82E5D918;
    'dispatch: loop {
        match pc {
            0x82E5D918 => {
    //   block [0x82E5D918..0x82E5D920)
	// 82E5D918: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5D91C: 480000D4  b 0x82e5d9f0
	sub_82E5D9F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D920 size=8
    let mut pc: u32 = 0x82E5D920;
    'dispatch: loop {
        match pc {
            0x82E5D920 => {
    //   block [0x82E5D920..0x82E5D928)
	// 82E5D920: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5D924: 480000CC  b 0x82e5d9f0
	sub_82E5D9F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5D928 size=8
    let mut pc: u32 = 0x82E5D928;
    'dispatch: loop {
        match pc {
            0x82E5D928 => {
    //   block [0x82E5D928..0x82E5D930)
	// 82E5D928: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82E5D92C: 480000C4  b 0x82e5d9f0
	sub_82E5D9F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5D930 size=192
    let mut pc: u32 = 0x82E5D930;
    'dispatch: loop {
        match pc {
            0x82E5D930 => {
    //   block [0x82E5D930..0x82E5D9F0)
	// 82E5D930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5D934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5D938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5D93C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5D940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5D944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5D948: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5D94C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5D950: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5D954: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5D958: 409A0020  bne cr6, 0x82e5d978
	if !ctx.cr[6].eq {
	pc = 0x82E5D978; continue 'dispatch;
	}
	// 82E5D95C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D960: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5D964: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5D968: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5D96C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5D970: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5D974: 4BEF7955  bl 0x82d552c8
	ctx.lr = 0x82E5D978;
	sub_82D552C8(ctx, base);
	// 82E5D978: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5D97C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5D980: 419A0054  beq cr6, 0x82e5d9d4
	if ctx.cr[6].eq {
	pc = 0x82E5D9D4; continue 'dispatch;
	}
	// 82E5D984: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5D988: 419A004C  beq cr6, 0x82e5d9d4
	if ctx.cr[6].eq {
	pc = 0x82E5D9D4; continue 'dispatch;
	}
	// 82E5D98C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5D990: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5D994: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5D998: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E5D99C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5D9A0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5D9A4: 41980018  blt cr6, 0x82e5d9bc
	if ctx.cr[6].lt {
	pc = 0x82E5D9BC; continue 'dispatch;
	}
	// 82E5D9A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E5D9AC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E5D9B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E5D9B4: 4BEF7775  bl 0x82d55128
	ctx.lr = 0x82E5D9B8;
	sub_82D55128(ctx, base);
	// 82E5D9B8: 4800001C  b 0x82e5d9d4
	pc = 0x82E5D9D4; continue 'dispatch;
	// 82E5D9BC: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E5D9C0: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E5D9C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E5D9C8: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82E5D9CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5D9D0: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82E5D9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5D9D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5D9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5D9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5D9E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5D9E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5D9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5D9F0 size=100
    let mut pc: u32 = 0x82E5D9F0;
    'dispatch: loop {
        match pc {
            0x82E5D9F0 => {
    //   block [0x82E5D9F0..0x82E5DA54)
	// 82E5D9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5D9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5D9F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5D9FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5DA00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DA04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5DA08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5DA0C: 4BFFFCC5  bl 0x82e5d6d0
	ctx.lr = 0x82E5DA10;
	sub_82E5D6D0(ctx, base);
	// 82E5DA10: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5DA14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5DA18: 419A0020  beq cr6, 0x82e5da38
	if ctx.cr[6].eq {
	pc = 0x82E5DA38; continue 'dispatch;
	}
	// 82E5DA1C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DA20: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5DA24: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5DA28: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DA2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5DA30: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5DA34: 4BEF7895  bl 0x82d552c8
	ctx.lr = 0x82E5DA38;
	sub_82D552C8(ctx, base);
	// 82E5DA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5DA3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5DA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5DA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5DA48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5DA4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5DA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DA58 size=152
    let mut pc: u32 = 0x82E5DA58;
    'dispatch: loop {
        match pc {
            0x82E5DA58 => {
    //   block [0x82E5DA58..0x82E5DAF0)
	// 82E5DA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DA5C: 4BE4B9AD  bl 0x82ca9408
	ctx.lr = 0x82E5DA60;
	sub_82CA93D0(ctx, base);
	// 82E5DA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DA64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5DA68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5DA6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5DA70: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82E5DA74: 409A0008  bne cr6, 0x82e5da7c
	if !ctx.cr[6].eq {
	pc = 0x82E5DA7C; continue 'dispatch;
	}
	// 82E5DA78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5DA7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5DA80: 4BF1A721  bl 0x82d781a0
	ctx.lr = 0x82E5DA84;
	sub_82D781A0(ctx, base);
	// 82E5DA84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5DA88: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 82E5DA8C: 409A0008  bne cr6, 0x82e5da94
	if !ctx.cr[6].eq {
	pc = 0x82E5DA94; continue 'dispatch;
	}
	// 82E5DA90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5DA94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5DA98: 4BF1A7E1  bl 0x82d78278
	ctx.lr = 0x82E5DA9C;
	sub_82D78278(ctx, base);
	// 82E5DA9C: 3BBE00E0  addi r29, r30, 0xe0
	ctx.r[29].s64 = ctx.r[30].s64 + 224;
	// 82E5DAA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5DAA4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DAA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5DAAC: 4099003C  ble cr6, 0x82e5dae8
	if !ctx.cr[6].gt {
	pc = 0x82E5DAE8; continue 'dispatch;
	}
	// 82E5DAB0: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82E5DAB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5DAB8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DABC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5DAC0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DAC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5DAC8: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5DACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5DAD0: 4E800421  bctrl
	ctx.lr = 0x82E5DAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5DAD4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DAD8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5DADC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5DAE0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5DAE4: 4198FFD4  blt cr6, 0x82e5dab8
	if ctx.cr[6].lt {
	pc = 0x82E5DAB8; continue 'dispatch;
	}
	// 82E5DAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5DAEC: 4BE4B96C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DAF0 size=152
    let mut pc: u32 = 0x82E5DAF0;
    'dispatch: loop {
        match pc {
            0x82E5DAF0 => {
    //   block [0x82E5DAF0..0x82E5DB88)
	// 82E5DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DAF4: 4BE4B915  bl 0x82ca9408
	ctx.lr = 0x82E5DAF8;
	sub_82CA93D0(ctx, base);
	// 82E5DAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DAFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5DB00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5DB04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5DB08: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82E5DB0C: 409A0008  bne cr6, 0x82e5db14
	if !ctx.cr[6].eq {
	pc = 0x82E5DB14; continue 'dispatch;
	}
	// 82E5DB10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5DB14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5DB18: 4BF1BBB1  bl 0x82d796c8
	ctx.lr = 0x82E5DB1C;
	sub_82D796C8(ctx, base);
	// 82E5DB1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5DB20: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 82E5DB24: 409A0008  bne cr6, 0x82e5db2c
	if !ctx.cr[6].eq {
	pc = 0x82E5DB2C; continue 'dispatch;
	}
	// 82E5DB28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5DB2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5DB30: 4BF1BCE9  bl 0x82d79818
	ctx.lr = 0x82E5DB34;
	sub_82D79818(ctx, base);
	// 82E5DB34: 3BBE00E0  addi r29, r30, 0xe0
	ctx.r[29].s64 = ctx.r[30].s64 + 224;
	// 82E5DB38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5DB3C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DB40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5DB44: 4099003C  ble cr6, 0x82e5db80
	if !ctx.cr[6].gt {
	pc = 0x82E5DB80; continue 'dispatch;
	}
	// 82E5DB48: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82E5DB4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5DB50: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DB54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5DB58: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DB5C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DB60: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5DB64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5DB68: 4E800421  bctrl
	ctx.lr = 0x82E5DB6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5DB6C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DB70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5DB74: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5DB78: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5DB7C: 4198FFD4  blt cr6, 0x82e5db50
	if ctx.cr[6].lt {
	pc = 0x82E5DB50; continue 'dispatch;
	}
	// 82E5DB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5DB84: 4BE4B8D4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5DB88 size=20
    let mut pc: u32 = 0x82E5DB88;
    'dispatch: loop {
        match pc {
            0x82E5DB88 => {
    //   block [0x82E5DB88..0x82E5DB9C)
	// 82E5DB88: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5DB8C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82E5DB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5DB94: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E5DB98: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DB9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5DB9C size=36
    let mut pc: u32 = 0x82E5DB9C;
    'dispatch: loop {
        match pc {
            0x82E5DB9C => {
    //   block [0x82E5DB9C..0x82E5DBC0)
	// 82E5DB9C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5DBA0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DBA4: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5DBA8: 419A0018  beq cr6, 0x82e5dbc0
	if ctx.cr[6].eq {
		sub_82E5DBC0(ctx, base);
		return;
	}
	// 82E5DBAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5DBB0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5DBB4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E5DBB8: 4198FFE8  blt cr6, 0x82e5dba0
	if ctx.cr[6].lt {
	pc = 0x82E5DBA0; continue 'dispatch;
	}
	// 82E5DBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5DBC0 size=8
    let mut pc: u32 = 0x82E5DBC0;
    'dispatch: loop {
        match pc {
            0x82E5DBC0 => {
    //   block [0x82E5DBC0..0x82E5DBC8)
	// 82E5DBC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5DBC4: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5DBC8 size=68
    let mut pc: u32 = 0x82E5DBC8;
    'dispatch: loop {
        match pc {
            0x82E5DBC8 => {
    //   block [0x82E5DBC8..0x82E5DC0C)
	// 82E5DBC8: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5DBCC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E5DBD0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5DBD4: 38890010  addi r4, r9, 0x10
	ctx.r[4].s64 = ctx.r[9].s64 + 16;
	// 82E5DBD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5DBDC: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E5DBE0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E5DBE4: 7D47582E  lwzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5DBE8: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5DBEC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5DBF0: 8063FFF0  lwz r3, -0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E5DBF4: 80C90010  lwz r6, 0x10(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5DBF8: 80ABB5D8  lwz r5, -0x4a28(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18984 as u32) ) } as u64;
	// 82E5DBFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DC00: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5DC04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5DC08: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DC10 size=100
    let mut pc: u32 = 0x82E5DC10;
    'dispatch: loop {
        match pc {
            0x82E5DC10 => {
    //   block [0x82E5DC10..0x82E5DC74)
	// 82E5DC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5DC18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DC1C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DC20: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5DC24: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82E5DC28: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82E5DC2C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5DC30: 4BEF7619  bl 0x82d55248
	ctx.lr = 0x82E5DC34;
	sub_82D55248(ctx, base);
	// 82E5DC34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5DC38: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82E5DC3C: 396B77B8  addi r11, r11, 0x77b8
	ctx.r[11].s64 = ctx.r[11].s64 + 30648;
	// 82E5DC40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E5DC44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5DC48: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82E5DC4C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82E5DC50: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5DC54: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82E5DC58: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5DC5C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E5DC60: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82E5DC64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5DC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5DC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5DC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DC78 size=152
    let mut pc: u32 = 0x82E5DC78;
    'dispatch: loop {
        match pc {
            0x82E5DC78 => {
    //   block [0x82E5DC78..0x82E5DD10)
	// 82E5DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5DC80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5DC84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DC88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5DC8C: 480080F5  bl 0x82e65d80
	ctx.lr = 0x82E5DC90;
	sub_82E65D80(ctx, base);
	// 82E5DC90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5DC94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5DC98: 396B5D58  addi r11, r11, 0x5d58
	ctx.r[11].s64 = ctx.r[11].s64 + 23896;
	// 82E5DC9C: 394A7494  addi r10, r10, 0x7494
	ctx.r[10].s64 = ctx.r[10].s64 + 29844;
	// 82E5DCA0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5DCA4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5DCA8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5DCAC: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5DCB0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5DCB4: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E5DCB8: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82E5DCBC: 39297820  addi r9, r9, 0x7820
	ctx.r[9].s64 = ctx.r[9].s64 + 30752;
	// 82E5DCC0: 39087800  addi r8, r8, 0x7800
	ctx.r[8].s64 = ctx.r[8].s64 + 30720;
	// 82E5DCC4: 38E777F0  addi r7, r7, 0x77f0
	ctx.r[7].s64 = ctx.r[7].s64 + 30704;
	// 82E5DCC8: 38C677D8  addi r6, r6, 0x77d8
	ctx.r[6].s64 = ctx.r[6].s64 + 30680;
	// 82E5DCCC: 38A577C8  addi r5, r5, 0x77c8
	ctx.r[5].s64 = ctx.r[5].s64 + 30664;
	// 82E5DCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5DCD4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5DCD8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E5DCDC: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E5DCE0: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82E5DCE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5DCE8: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82E5DCEC: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82E5DCF0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E5DCF4: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E5DCF8: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E5DCFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5DD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5DD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5DD08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5DD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DD10 size=100
    let mut pc: u32 = 0x82E5DD10;
    'dispatch: loop {
        match pc {
            0x82E5DD10 => {
    //   block [0x82E5DD10..0x82E5DD74)
	// 82E5DD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DD14: 4BE4B6F5  bl 0x82ca9408
	ctx.lr = 0x82E5DD18;
	sub_82CA93D0(ctx, base);
	// 82E5DD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DD1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5DD20: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5DD24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5DD28: 419A0044  beq cr6, 0x82e5dd6c
	if ctx.cr[6].eq {
	pc = 0x82E5DD6C; continue 'dispatch;
	}
	// 82E5DD2C: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5DD30: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5DD34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5DD38: 40990034  ble cr6, 0x82e5dd6c
	if !ctx.cr[6].gt {
	pc = 0x82E5DD6C; continue 'dispatch;
	}
	// 82E5DD3C: 3B9DFFF8  addi r28, r29, -8
	ctx.r[28].s64 = ctx.r[29].s64 + -8;
	// 82E5DD40: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5DD44: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5DD48: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5DD4C: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5DD50: 4BFFFDA1  bl 0x82e5daf0
	ctx.lr = 0x82E5DD54;
	sub_82E5DAF0(ctx, base);
	// 82E5DD54: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5DD58: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5DD5C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5DD60: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5DD64: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E5DD68: 4198FFDC  blt cr6, 0x82e5dd44
	if ctx.cr[6].lt {
	pc = 0x82E5DD44; continue 'dispatch;
	}
	// 82E5DD6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5DD70: 4BE4B6E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DD78 size=236
    let mut pc: u32 = 0x82E5DD78;
    'dispatch: loop {
        match pc {
            0x82E5DD78 => {
    //   block [0x82E5DD78..0x82E5DE64)
	// 82E5DD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DD7C: 4BE4B691  bl 0x82ca940c
	ctx.lr = 0x82E5DD80;
	sub_82CA93D0(ctx, base);
	// 82E5DD80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DD84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5DD88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5DD8C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5DD90: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5DD94: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5DD98: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5DD9C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5DDA0: 394A7820  addi r10, r10, 0x7820
	ctx.r[10].s64 = ctx.r[10].s64 + 30752;
	// 82E5DDA4: 39297800  addi r9, r9, 0x7800
	ctx.r[9].s64 = ctx.r[9].s64 + 30720;
	// 82E5DDA8: 390877F0  addi r8, r8, 0x77f0
	ctx.r[8].s64 = ctx.r[8].s64 + 30704;
	// 82E5DDAC: 38E777D8  addi r7, r7, 0x77d8
	ctx.r[7].s64 = ctx.r[7].s64 + 30680;
	// 82E5DDB0: 38C677C8  addi r6, r6, 0x77c8
	ctx.r[6].s64 = ctx.r[6].s64 + 30664;
	// 82E5DDB4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5DDB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5DDBC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5DDC0: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E5DDC4: 90FF0028  stw r7, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82E5DDC8: 90DF002C  stw r6, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82E5DDCC: 419A0044  beq cr6, 0x82e5de10
	if ctx.cr[6].eq {
	pc = 0x82E5DE10; continue 'dispatch;
	}
	// 82E5DDD0: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5DDD4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5DDD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5DDDC: 40990034  ble cr6, 0x82e5de10
	if !ctx.cr[6].gt {
	pc = 0x82E5DE10; continue 'dispatch;
	}
	// 82E5DDE0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5DDE4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5DDE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5DDEC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5DDF0: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5DDF4: 4BFFFC65  bl 0x82e5da58
	ctx.lr = 0x82E5DDF8;
	sub_82E5DA58(ctx, base);
	// 82E5DDF8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5DDFC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E5DE00: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5DE04: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5DE08: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5DE0C: 4198FFD8  blt cr6, 0x82e5dde4
	if ctx.cr[6].lt {
	pc = 0x82E5DDE4; continue 'dispatch;
	}
	// 82E5DE10: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5DE14: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5DE18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5DE1C: 409A0020  bne cr6, 0x82e5de3c
	if !ctx.cr[6].eq {
	pc = 0x82E5DE3C; continue 'dispatch;
	}
	// 82E5DE20: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DE24: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5DE28: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5DE2C: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5DE30: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5DE34: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5DE38: 4BEF7491  bl 0x82d552c8
	ctx.lr = 0x82E5DE3C;
	sub_82D552C8(ctx, base);
	// 82E5DE3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5DE40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5DE44: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5DE48: 394A5D58  addi r10, r10, 0x5d58
	ctx.r[10].s64 = ctx.r[10].s64 + 23896;
	// 82E5DE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5DE50: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E5DE54: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82E5DE58: 48007E51  bl 0x82e65ca8
	ctx.lr = 0x82E5DE5C;
	sub_82E65CA8(ctx, base);
	// 82E5DE5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5DE60: 4BE4B5FC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5DE68 size=8
    let mut pc: u32 = 0x82E5DE68;
    'dispatch: loop {
        match pc {
            0x82E5DE68 => {
    //   block [0x82E5DE68..0x82E5DE70)
	// 82E5DE68: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5DE6C: 4BFFFBEC  b 0x82e5da58
	sub_82E5DA58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5DE70 size=8
    let mut pc: u32 = 0x82E5DE70;
    'dispatch: loop {
        match pc {
            0x82E5DE70 => {
    //   block [0x82E5DE70..0x82E5DE78)
	// 82E5DE70: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5DE74: 4BFFFC7C  b 0x82e5daf0
	sub_82E5DAF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DE78 size=112
    let mut pc: u32 = 0x82E5DE78;
    'dispatch: loop {
        match pc {
            0x82E5DE78 => {
    //   block [0x82E5DE78..0x82E5DEE8)
	// 82E5DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5DE80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5DE84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5DE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DE8C: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 82E5DE90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5DE94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5DE98: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DE9C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5DEA0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5DEA4: 409A0010  bne cr6, 0x82e5deb4
	if !ctx.cr[6].eq {
	pc = 0x82E5DEB4; continue 'dispatch;
	}
	// 82E5DEA8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5DEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5DEB0: 4BEF90E9  bl 0x82d56f98
	ctx.lr = 0x82E5DEB4;
	sub_82D56F98(ctx, base);
	// 82E5DEB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DEB8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DEBC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5DEC0: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E5DEC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5DEC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5DECC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5DED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5DED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5DED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5DEDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5DEE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5DEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DEE8 size=96
    let mut pc: u32 = 0x82E5DEE8;
    'dispatch: loop {
        match pc {
            0x82E5DEE8 => {
    //   block [0x82E5DEE8..0x82E5DF48)
	// 82E5DEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5DEF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5DEF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DEF8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DEFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5DF00: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5DF04: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82E5DF08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5DF0C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5DF10: 4BEF7339  bl 0x82d55248
	ctx.lr = 0x82E5DF14;
	sub_82D55248(ctx, base);
	// 82E5DF14: 3960003C  li r11, 0x3c
	ctx.r[11].s64 = 60;
	// 82E5DF18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5DF1C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5DF20: 4BFFFD59  bl 0x82e5dc78
	ctx.lr = 0x82E5DF24;
	sub_82E5DC78(ctx, base);
	// 82E5DF24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5DF28: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5DF2C: 409A0008  bne cr6, 0x82e5df34
	if !ctx.cr[6].eq {
	pc = 0x82E5DF34; continue 'dispatch;
	}
	// 82E5DF30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5DF34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5DF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5DF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5DF40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5DF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5DF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5DF48 size=712
    let mut pc: u32 = 0x82E5DF48;
    'dispatch: loop {
        match pc {
            0x82E5DF48 => {
    //   block [0x82E5DF48..0x82E5E210)
	// 82E5DF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5DF4C: 4BE4B4A5  bl 0x82ca93f0
	ctx.lr = 0x82E5DF50;
	sub_82CA93D0(ctx, base);
	// 82E5DF50: 9421FD00  stwu r1, -0x300(r1)
	ea = ctx.r[1].u32.wrapping_add(-768 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5DF54: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82E5DF58: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E5DF5C: 3B190010  addi r24, r25, 0x10
	ctx.r[24].s64 = ctx.r[25].s64 + 16;
	// 82E5DF60: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E5DF64: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DF68: 82F90010  lwz r23, 0x10(r25)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5DF6C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E5DF70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5DF74: 4E800421  bctrl
	ctx.lr = 0x82E5DF78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5DF78: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82E5DF7C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82E5DF80: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82E5DF84: 92C10050  stw r22, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[22].u32 ) };
	// 82E5DF88: 92C10054  stw r22, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[22].u32 ) };
	// 82E5DF8C: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82E5DF90: 419A0010  beq cr6, 0x82e5dfa0
	if ctx.cr[6].eq {
	pc = 0x82E5DFA0; continue 'dispatch;
	}
	// 82E5DF94: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82E5DF98: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82E5DF9C: 409A0008  bne cr6, 0x82e5dfa4
	if !ctx.cr[6].eq {
	pc = 0x82E5DFA4; continue 'dispatch;
	}
	// 82E5DFA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5DFA4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E5DFA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5DFAC: 409A0050  bne cr6, 0x82e5dffc
	if !ctx.cr[6].eq {
	pc = 0x82E5DFFC; continue 'dispatch;
	}
	// 82E5DFB0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5DFB4: 814BB5D4  lwz r10, -0x4a2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18988 as u32) ) } as u64;
	// 82E5DFB8: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82E5DFBC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5DFC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5DFC4: 40990024  ble cr6, 0x82e5dfe8
	if !ctx.cr[6].gt {
	pc = 0x82E5DFE8; continue 'dispatch;
	}
	// 82E5DFC8: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5DFCC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5DFD0: 7F071800  cmpw cr6, r7, r3
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82E5DFD4: 419A0018  beq cr6, 0x82e5dfec
	if ctx.cr[6].eq {
	pc = 0x82E5DFEC; continue 'dispatch;
	}
	// 82E5DFD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5DFDC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5DFE0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5DFE4: 4198FFE8  blt cr6, 0x82e5dfcc
	if ctx.cr[6].lt {
	pc = 0x82E5DFCC; continue 'dispatch;
	}
	// 82E5DFE8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E5DFEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5DFF0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E5DFF4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E5DFF8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82E5DFFC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E5E000: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5E004: 419A01DC  beq cr6, 0x82e5e1e0
	if ctx.cr[6].eq {
	pc = 0x82E5E1E0; continue 'dispatch;
	}
	// 82E5E008: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82E5E00C: 419A01D4  beq cr6, 0x82e5e1e0
	if ctx.cr[6].eq {
	pc = 0x82E5E1E0; continue 'dispatch;
	}
	// 82E5E010: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E5E014: 48007F15  bl 0x82e65f28
	ctx.lr = 0x82E5E018;
	sub_82E65F28(ctx, base);
	// 82E5E018: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E5E01C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E5E020: 48007F19  bl 0x82e65f38
	ctx.lr = 0x82E5E024;
	sub_82E65F38(ctx, base);
	// 82E5E024: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E5E028: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82E5E02C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E5E030: 48009919  bl 0x82e67948
	ctx.lr = 0x82E5E034;
	sub_82E67948(ctx, base);
	// 82E5E034: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5E038: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82E5E03C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E5E040: 419800B4  blt cr6, 0x82e5e0f4
	if ctx.cr[6].lt {
	pc = 0x82E5E0F4; continue 'dispatch;
	}
	// 82E5E044: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5E048: 577F103A  slwi r31, r27, 2
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E5E04C: 3BCB782C  addi r30, r11, 0x782c
	ctx.r[30].s64 = ctx.r[11].s64 + 30764;
	// 82E5E050: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5E054: 3F808333  lis r28, -0x7ccd
	ctx.r[28].s64 = -2093809664;
	// 82E5E058: 3BAB75D0  addi r29, r11, 0x75d0
	ctx.r[29].s64 = ctx.r[11].s64 + 30160;
	// 82E5E05C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E060: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E064: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E068: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82E5E06C: 409A0078  bne cr6, 0x82e5e0e4
	if !ctx.cr[6].eq {
	pc = 0x82E5E0E4; continue 'dispatch;
	}
	// 82E5E070: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E074: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E078: 409A006C  bne cr6, 0x82e5e0e4
	if !ctx.cr[6].eq {
	pc = 0x82E5E0E4; continue 'dispatch;
	}
	// 82E5E07C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E5E080: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82E5E084: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5E088: 4BEF9961  bl 0x82d579e8
	ctx.lr = 0x82E5E08C;
	sub_82D579E8(ctx, base);
	// 82E5E08C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5E090: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5E094: 4BEF9D5D  bl 0x82d57df0
	ctx.lr = 0x82E5E098;
	sub_82D57DF0(ctx, base);
	// 82E5E098: 807C7630  lwz r3, 0x7630(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E5E09C: 39000092  li r8, 0x92
	ctx.r[8].s64 = 146;
	// 82E5E0A0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E5E0A4: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82E5E0A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E5E0AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E0B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5E0B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5E0B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E0BC: 4E800421  bctrl
	ctx.lr = 0x82E5E0C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E0C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5E0C4: 4BEFA36D  bl 0x82d58430
	ctx.lr = 0x82E5E0C8;
	sub_82D58430(ctx, base);
	// 82E5E0C8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5E0CC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E5E0D0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5E0D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E5E0D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E0DC: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E0E0: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5E0E4: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82E5E0E8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E5E0EC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E5E0F0: 4098FF6C  bge cr6, 0x82e5e05c
	if !ctx.cr[6].lt {
	pc = 0x82E5E05C; continue 'dispatch;
	}
	// 82E5E0F4: 3BFA0008  addi r31, r26, 8
	ctx.r[31].s64 = ctx.r[26].s64 + 8;
	// 82E5E0F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E0FC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E100: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5E104: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5E108: 409A0010  bne cr6, 0x82e5e118
	if !ctx.cr[6].eq {
	pc = 0x82E5E118; continue 'dispatch;
	}
	// 82E5E10C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5E110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5E114: 4BEF8E85  bl 0x82d56f98
	ctx.lr = 0x82E5E118;
	sub_82D56F98(ctx, base);
	// 82E5E118: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E11C: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 82E5E120: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E124: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82E5E128: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5E12C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5E130: 7F2B512E  stwx r25, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[25].u32) };
	// 82E5E134: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E138: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5E13C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5E140: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 82E5E144: 807AFFF0  lwz r3, -0x10(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E5E148: 80B80008  lwz r5, 8(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E14C: 80FFB5D8  lwz r7, -0x4a28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18984 as u32) ) } as u64;
	// 82E5E150: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E154: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E15C: 4E800421  bctrl
	ctx.lr = 0x82E5E160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E160: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E164: 807AFFF0  lwz r3, -0x10(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E5E168: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82E5E16C: 80DFB5D8  lwz r6, -0x4a28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18984 as u32) ) } as u64;
	// 82E5E170: 808BB5DC  lwz r4, -0x4a24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18980 as u32) ) } as u64;
	// 82E5E174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E178: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5E17C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E180: 4E800421  bctrl
	ctx.lr = 0x82E5E184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E184: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5E188: 7EDFB378  mr r31, r22
	ctx.r[31].u64 = ctx.r[22].u64;
	// 82E5E18C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E190: 40990040  ble cr6, 0x82e5e1d0
	if !ctx.cr[6].gt {
	pc = 0x82E5E1D0; continue 'dispatch;
	}
	// 82E5E194: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 82E5E198: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E19C: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E1A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5E1A4: 419A0018  beq cr6, 0x82e5e1bc
	if ctx.cr[6].eq {
	pc = 0x82E5E1BC; continue 'dispatch;
	}
	// 82E5E1A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E1AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5E1B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E1B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E1B8: 4E800421  bctrl
	ctx.lr = 0x82E5E1BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E1BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5E1C0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E5E1C4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5E1C8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5E1CC: 4198FFCC  blt cr6, 0x82e5e198
	if ctx.cr[6].lt {
	pc = 0x82E5E198; continue 'dispatch;
	}
	// 82E5E1D0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E5E1D4: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E5E1D8: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E5E1DC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E5E1E0: 550B0000  rlwinm r11, r8, 0, 0, 0
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5E1E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E1E8: 409A0020  bne cr6, 0x82e5e208
	if !ctx.cr[6].eq {
	pc = 0x82E5E208; continue 'dispatch;
	}
	// 82E5E1EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E1F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E1F4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5E1F8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E1FC: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5E200: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E204: 4BEF70C5  bl 0x82d552c8
	ctx.lr = 0x82E5E208;
	sub_82D552C8(ctx, base);
	// 82E5E208: 38210300  addi r1, r1, 0x300
	ctx.r[1].s64 = ctx.r[1].s64 + 768;
	// 82E5E20C: 4BE4B234  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E210 size=64
    let mut pc: u32 = 0x82E5E210;
    'dispatch: loop {
        match pc {
            0x82E5E210 => {
    //   block [0x82E5E210..0x82E5E250)
	// 82E5E210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E21C: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5E220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5E224: 38ABDEE8  addi r5, r11, -0x2118
	ctx.r[5].s64 = ctx.r[11].s64 + -8472;
	// 82E5E228: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E22C: 388A891C  addi r4, r10, -0x76e4
	ctx.r[4].s64 = ctx.r[10].s64 + -30436;
	// 82E5E230: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5E234: 4BFDAA65  bl 0x82e38c98
	ctx.lr = 0x82E5E238;
	sub_82E38C98(ctx, base);
	// 82E5E238: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E23C: 906BB5D8  stw r3, -0x4a28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18984 as u32), ctx.r[3].u32 ) };
	// 82E5E240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5E244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E250 size=748
    let mut pc: u32 = 0x82E5E250;
    'dispatch: loop {
        match pc {
            0x82E5E250 => {
    //   block [0x82E5E250..0x82E5E53C)
	// 82E5E250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E254: 4BE4B1A1  bl 0x82ca93f4
	ctx.lr = 0x82E5E258;
	sub_82CA93D0(ctx, base);
	// 82E5E258: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E25C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E260: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	// 82E5E264: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E5E268: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E5E26C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E270: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5E274: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5E278: 40980020  bge cr6, 0x82e5e298
	if !ctx.cr[6].lt {
	pc = 0x82E5E298; continue 'dispatch;
	}
	// 82E5E27C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5E280: 3929786C  addi r9, r9, 0x786c
	ctx.r[9].s64 = ctx.r[9].s64 + 30828;
	// 82E5E284: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5E288: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5E28C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E5E290: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5E294: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5E298: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E5E29C: 3B8400E0  addi r28, r4, 0xe0
	ctx.r[28].s64 = ctx.r[4].s64 + 224;
	// 82E5E2A0: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82E5E2A4: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82E5E2A8: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82E5E2AC: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E2B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E5E2B4: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82E5E2B8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5E2BC: 40980038  bge cr6, 0x82e5e2f4
	if !ctx.cr[6].lt {
	pc = 0x82E5E2F4; continue 'dispatch;
	}
	// 82E5E2C0: 57DF3830  slwi r31, r30, 7
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(7);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E5E2C4: 48000008  b 0x82e5e2cc
	pc = 0x82E5E2CC; continue 'dispatch;
	// 82E5E2C8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E2CC: 7C7F5A14  add r3, r31, r11
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E5E2D0: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E2D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5E2D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E2DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E2E0: 4E800421  bctrl
	ctx.lr = 0x82E5E2E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E2E4: 3BFF0080  addi r31, r31, 0x80
	ctx.r[31].s64 = ctx.r[31].s64 + 128;
	// 82E5E2E8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E5E2EC: 4198FFDC  blt cr6, 0x82e5e2c8
	if ctx.cr[6].lt {
	pc = 0x82E5E2C8; continue 'dispatch;
	}
	// 82E5E2F0: 48000068  b 0x82e5e358
	pc = 0x82E5E358; continue 'dispatch;
	// 82E5E2F4: 40990034  ble cr6, 0x82e5e328
	if !ctx.cr[6].gt {
	pc = 0x82E5E328; continue 'dispatch;
	}
	// 82E5E2F8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82E5E2FC: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82E5E300: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82E5E304: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5E308: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E5E30C: 40990018  ble cr6, 0x82e5e324
	if !ctx.cr[6].gt {
	pc = 0x82E5E324; continue 'dispatch;
	}
	// 82E5E310: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 82E5E314: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5E318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5E31C: 4BEF8BF5  bl 0x82d56f10
	ctx.lr = 0x82E5E320;
	sub_82D56F10(ctx, base);
	// 82E5E320: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E324: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82E5E328: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5E32C: 4099002C  ble cr6, 0x82e5e358
	if !ctx.cr[6].gt {
	pc = 0x82E5E358; continue 'dispatch;
	}
	// 82E5E330: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E5E334: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82E5E338: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5E33C: 419A000C  beq cr6, 0x82e5e348
	if ctx.cr[6].eq {
	pc = 0x82E5E348; continue 'dispatch;
	}
	// 82E5E340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5E344: 4BFE03E5  bl 0x82e3e728
	ctx.lr = 0x82E5E348;
	sub_82E3E728(ctx, base);
	// 82E5E348: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82E5E34C: 3BFF0080  addi r31, r31, 0x80
	ctx.r[31].s64 = ctx.r[31].s64 + 128;
	// 82E5E350: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E5E354: 409AFFE4  bne cr6, 0x82e5e338
	if !ctx.cr[6].eq {
	pc = 0x82E5E338; continue 'dispatch;
	}
	// 82E5E358: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E35C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E5E360: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82E5E364: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E5E368: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82E5E36C: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82E5E370: 4099001C  ble cr6, 0x82e5e38c
	if !ctx.cr[6].gt {
	pc = 0x82E5E38C; continue 'dispatch;
	}
	// 82E5E374: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E5E378: 41980008  blt cr6, 0x82e5e380
	if ctx.cr[6].lt {
	pc = 0x82E5E380; continue 'dispatch;
	}
	// 82E5E37C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5E380: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E5E384: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5E388: 4BEF8B89  bl 0x82d56f10
	ctx.lr = 0x82E5E38C;
	sub_82D56F10(ctx, base);
	// 82E5E38C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E390: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E5E394: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82E5E398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E39C: 40990064  ble cr6, 0x82e5e400
	if !ctx.cr[6].gt {
	pc = 0x82E5E400; continue 'dispatch;
	}
	// 82E5E3A0: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E5E3A4: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E5E3A8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E3AC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E5E3B0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E3B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E3B8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5E3BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E3C0: 4E800421  bctrl
	ctx.lr = 0x82E5E3C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E3C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E3C8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82E5E3CC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E5E3D0: 7C7F5A14  add r3, r31, r11
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E5E3D4: 4BFE0265  bl 0x82e3e638
	ctx.lr = 0x82E5E3D8;
	sub_82E3E638(ctx, base);
	// 82E5E3D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E3DC: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E5E3E0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E5E3E4: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E5E3E8: 3BFF0080  addi r31, r31, 0x80
	ctx.r[31].s64 = ctx.r[31].s64 + 128;
	// 82E5E3EC: 7D7E512E  stwx r11, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82E5E3F0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5E3F4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E3F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5E3FC: 4198FFAC  blt cr6, 0x82e5e3a8
	if ctx.cr[6].lt {
	pc = 0x82E5E3A8; continue 'dispatch;
	}
	// 82E5E400: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E404: 807AFFEC  lwz r3, -0x14(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82E5E408: 3FA08334  lis r29, -0x7ccc
	ctx.r[29].s64 = -2093744128;
	// 82E5E40C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E5E410: 80ABB5DC  lwz r5, -0x4a24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18980 as u32) ) } as u64;
	// 82E5E414: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E418: 80DDB5D8  lwz r6, -0x4a28(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-18984 as u32) ) } as u64;
	// 82E5E41C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E5E420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E424: 4E800421  bctrl
	ctx.lr = 0x82E5E428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E428: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E5E42C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5E430: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5E434: 409A001C  bne cr6, 0x82e5e450
	if !ctx.cr[6].eq {
	pc = 0x82E5E450; continue 'dispatch;
	}
	// 82E5E438: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E43C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E5E440: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5E444: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5E448: 7C78502E  lwzx r3, r24, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5E44C: 4BEF6E7D  bl 0x82d552c8
	ctx.lr = 0x82E5E450;
	sub_82D552C8(ctx, base);
	// 82E5E450: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E5E454: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E5E458: 40990034  ble cr6, 0x82e5e48c
	if !ctx.cr[6].gt {
	pc = 0x82E5E48C; continue 'dispatch;
	}
	// 82E5E45C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E5E460: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5E468: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E5E46C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E470: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E478: 4E800421  bctrl
	ctx.lr = 0x82E5E47C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E47C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E5E480: 3BDE0080  addi r30, r30, 0x80
	ctx.r[30].s64 = ctx.r[30].s64 + 128;
	// 82E5E484: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5E488: 409AFFD8  bne cr6, 0x82e5e460
	if !ctx.cr[6].eq {
	pc = 0x82E5E460; continue 'dispatch;
	}
	// 82E5E48C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E5E490: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5E494: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5E498: 409A001C  bne cr6, 0x82e5e4b4
	if !ctx.cr[6].eq {
	pc = 0x82E5E4B4; continue 'dispatch;
	}
	// 82E5E49C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E4A0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5E4A4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5E4A8: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5E4AC: 7C78502E  lwzx r3, r24, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5E4B0: 4BEF6E19  bl 0x82d552c8
	ctx.lr = 0x82E5E4B4;
	sub_82D552C8(ctx, base);
	// 82E5E4B4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E4B8: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E5E4BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E4C0: 40990044  ble cr6, 0x82e5e504
	if !ctx.cr[6].gt {
	pc = 0x82E5E504; continue 'dispatch;
	}
	// 82E5E4C4: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E5E4C8: 807AFFEC  lwz r3, -0x14(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82E5E4CC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E4D0: 80DDB5D8  lwz r6, -0x4a28(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-18984 as u32) ) } as u64;
	// 82E5E4D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E4D8: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E4DC: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82E5E4E0: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5E4E4: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5E4E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E5E4EC: 4E800421  bctrl
	ctx.lr = 0x82E5E4F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E4F0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E4F4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5E4F8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5E4FC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5E500: 4198FFC8  blt cr6, 0x82e5e4c8
	if ctx.cr[6].lt {
	pc = 0x82E5E4C8; continue 'dispatch;
	}
	// 82E5E504: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E5E508: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E50C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5E510: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5E514: 40980020  bge cr6, 0x82e5e534
	if !ctx.cr[6].lt {
	pc = 0x82E5E534; continue 'dispatch;
	}
	// 82E5E518: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82E5E51C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82E5E520: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5E524: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5E528: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E5E52C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5E530: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5E534: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E5E538: 4BE4AF0C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E540 size=12
    let mut pc: u32 = 0x82E5E540;
    'dispatch: loop {
        match pc {
            0x82E5E540 => {
    //   block [0x82E5E540..0x82E5E54C)
	// 82E5E540: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E544: 806BB5D8  lwz r3, -0x4a28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18984 as u32) ) } as u64;
	// 82E5E548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E550 size=8
    let mut pc: u32 = 0x82E5E550;
    'dispatch: loop {
        match pc {
            0x82E5E550 => {
    //   block [0x82E5E550..0x82E5E558)
	// 82E5E550: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5E554: 480000B4  b 0x82e5e608
	sub_82E5E608(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E558 size=8
    let mut pc: u32 = 0x82E5E558;
    'dispatch: loop {
        match pc {
            0x82E5E558 => {
    //   block [0x82E5E558..0x82E5E560)
	// 82E5E558: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5E55C: 480000AC  b 0x82e5e608
	sub_82E5E608(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E560 size=8
    let mut pc: u32 = 0x82E5E560;
    'dispatch: loop {
        match pc {
            0x82E5E560 => {
    //   block [0x82E5E560..0x82E5E568)
	// 82E5E560: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82E5E564: 480000A4  b 0x82e5e608
	sub_82E5E608(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E568 size=8
    let mut pc: u32 = 0x82E5E568;
    'dispatch: loop {
        match pc {
            0x82E5E568 => {
    //   block [0x82E5E568..0x82E5E570)
	// 82E5E568: 3863FFD4  addi r3, r3, -0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + -44;
	// 82E5E56C: 4800009C  b 0x82e5e608
	sub_82E5E608(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E570 size=152
    let mut pc: u32 = 0x82E5E570;
    'dispatch: loop {
        match pc {
            0x82E5E570 => {
    //   block [0x82E5E570..0x82E5E608)
	// 82E5E570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5E57C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5E588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5E58C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5E590: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5E594: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5E598: 409A0020  bne cr6, 0x82e5e5b8
	if !ctx.cr[6].eq {
	pc = 0x82E5E5B8; continue 'dispatch;
	}
	// 82E5E59C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E5A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5E5A4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5E5A8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E5AC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5E5B0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5E5B4: 4BEF6D15  bl 0x82d552c8
	ctx.lr = 0x82E5E5B8;
	sub_82D552C8(ctx, base);
	// 82E5E5B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E5E5BC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5E5C0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E5E5C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5E5C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5E5CC: 419A0020  beq cr6, 0x82e5e5ec
	if ctx.cr[6].eq {
	pc = 0x82E5E5EC; continue 'dispatch;
	}
	// 82E5E5D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E5D4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E5D8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82E5E5DC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E5E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5E5E4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E5E8: 4BEF6CE1  bl 0x82d552c8
	ctx.lr = 0x82E5E5EC;
	sub_82D552C8(ctx, base);
	// 82E5E5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5E5F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E5FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5E600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5E604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E608 size=100
    let mut pc: u32 = 0x82E5E608;
    'dispatch: loop {
        match pc {
            0x82E5E608 => {
    //   block [0x82E5E608..0x82E5E66C)
	// 82E5E608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5E614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E61C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5E620: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5E624: 4BFFF755  bl 0x82e5dd78
	ctx.lr = 0x82E5E628;
	sub_82E5DD78(ctx, base);
	// 82E5E628: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5E62C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5E630: 419A0020  beq cr6, 0x82e5e650
	if ctx.cr[6].eq {
	pc = 0x82E5E650; continue 'dispatch;
	}
	// 82E5E634: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E638: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E63C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5E640: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5E648: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E64C: 4BEF6C7D  bl 0x82d552c8
	ctx.lr = 0x82E5E650;
	sub_82D552C8(ctx, base);
	// 82E5E650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5E654: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5E658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E660: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5E664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5E668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E5E670 size=108
    let mut pc: u32 = 0x82E5E670;
    'dispatch: loop {
        match pc {
            0x82E5E670 => {
    //   block [0x82E5E670..0x82E5E6DC)
	// 82E5E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E67C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E680: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5E684: 480076FD  bl 0x82e65d80
	ctx.lr = 0x82E5E688;
	sub_82E65D80(ctx, base);
	// 82E5E688: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82E5E68C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5E690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5E694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E5E698: 396B78CC  addi r11, r11, 0x78cc
	ctx.r[11].s64 = ctx.r[11].s64 + 30924;
	// 82E5E69C: C0099404  lfs f0, -0x6bfc(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27644 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E5E6A0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5E6A4: 394A78AC  addi r10, r10, 0x78ac
	ctx.r[10].s64 = ctx.r[10].s64 + 30892;
	// 82E5E6A8: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E5E6AC: 3929789C  addi r9, r9, 0x789c
	ctx.r[9].s64 = ctx.r[9].s64 + 30876;
	// 82E5E6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5E6B4: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 82E5E6B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5E6BC: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82E5E6C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5E6C4: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E5E6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5E6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E6D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5E6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E6E0 size=24
    let mut pc: u32 = 0x82E5E6E0;
    'dispatch: loop {
        match pc {
            0x82E5E6E0 => {
    //   block [0x82E5E6E0..0x82E5E6F8)
	// 82E5E6E0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82E5E6E4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E5E6E8: 396BD438  addi r11, r11, -0x2bc8
	ctx.r[11].s64 = ctx.r[11].s64 + -11208;
	// 82E5E6EC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5E6F0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5E6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E6F8 size=12
    let mut pc: u32 = 0x82E5E6F8;
    'dispatch: loop {
        match pc {
            0x82E5E6F8 => {
    //   block [0x82E5E6F8..0x82E5E704)
	// 82E5E6F8: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E5E6FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5E700: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E704 size=12
    let mut pc: u32 = 0x82E5E704;
    'dispatch: loop {
        match pc {
            0x82E5E704 => {
    //   block [0x82E5E704..0x82E5E710)
	// 82E5E704: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E708: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E70C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E710 size=8
    let mut pc: u32 = 0x82E5E710;
    'dispatch: loop {
        match pc {
            0x82E5E710 => {
    //   block [0x82E5E710..0x82E5E718)
	// 82E5E710: 4BFF6550  b 0x82e54c60
	sub_82E54C60(ctx, base);
	return;
	// 82E5E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E718 size=168
    let mut pc: u32 = 0x82E5E718;
    'dispatch: loop {
        match pc {
            0x82E5E718 => {
    //   block [0x82E5E718..0x82E5E7C0)
	// 82E5E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5E72C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E5E730: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5E734: 419A0070  beq cr6, 0x82e5e7a4
	if ctx.cr[6].eq {
	pc = 0x82E5E7A4; continue 'dispatch;
	}
	// 82E5E738: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E73C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E740: 419A0020  beq cr6, 0x82e5e760
	if ctx.cr[6].eq {
	pc = 0x82E5E760; continue 'dispatch;
	}
	// 82E5E744: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5E748: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E5E74C: 4BF1FF7D  bl 0x82d7e6c8
	ctx.lr = 0x82E5E750;
	sub_82D7E6C8(ctx, base);
	// 82E5E750: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E5E754: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5E758: 4BF26D51  bl 0x82d854a8
	ctx.lr = 0x82E5E75C;
	sub_82D854A8(ctx, base);
	// 82E5E75C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E5E760: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5E764: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5E768: 419A0030  beq cr6, 0x82e5e798
	if ctx.cr[6].eq {
	pc = 0x82E5E798; continue 'dispatch;
	}
	// 82E5E76C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E5E770: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E5E774: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E5E778: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5E77C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E5E780: 409A0018  bne cr6, 0x82e5e798
	if !ctx.cr[6].eq {
	pc = 0x82E5E798; continue 'dispatch;
	}
	// 82E5E784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E788: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5E78C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5E794: 4E800421  bctrl
	ctx.lr = 0x82E5E798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5E798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5E79C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E5E7A0: 48000008  b 0x82e5e7a8
	pc = 0x82E5E7A8; continue 'dispatch;
	// 82E5E7A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5E7A8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5E7AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5E7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E7B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E7C0 size=96
    let mut pc: u32 = 0x82E5E7C0;
    'dispatch: loop {
        match pc {
            0x82E5E7C0 => {
    //   block [0x82E5E7C0..0x82E5E820)
	// 82E5E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E7C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E7CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E7D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E7D4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E7D8: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5E7DC: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 82E5E7E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5E7E4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E7E8: 4BEF6A61  bl 0x82d55248
	ctx.lr = 0x82E5E7EC;
	sub_82D55248(ctx, base);
	// 82E5E7EC: 39600034  li r11, 0x34
	ctx.r[11].s64 = 52;
	// 82E5E7F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5E7F4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5E7F8: 4BFFFE79  bl 0x82e5e670
	ctx.lr = 0x82E5E7FC;
	sub_82E5E670(ctx, base);
	// 82E5E7FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5E800: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5E804: 409A0008  bne cr6, 0x82e5e80c
	if !ctx.cr[6].eq {
	pc = 0x82E5E80C; continue 'dispatch;
	}
	// 82E5E808: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5E80C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5E810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5E81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E820 size=64
    let mut pc: u32 = 0x82E5E820;
    'dispatch: loop {
        match pc {
            0x82E5E820 => {
    //   block [0x82E5E820..0x82E5E860)
	// 82E5E820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E82C: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5E830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5E834: 38ABE7C0  addi r5, r11, -0x1840
	ctx.r[5].s64 = ctx.r[11].s64 + -6208;
	// 82E5E838: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E83C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82E5E840: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5E844: 4BFDA455  bl 0x82e38c98
	ctx.lr = 0x82E5E848;
	sub_82E38C98(ctx, base);
	// 82E5E848: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5E84C: 906BB5E0  stw r3, -0x4a20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18976 as u32), ctx.r[3].u32 ) };
	// 82E5E850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5E854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E860 size=12
    let mut pc: u32 = 0x82E5E860;
    'dispatch: loop {
        match pc {
            0x82E5E860 => {
    //   block [0x82E5E860..0x82E5E86C)
	// 82E5E860: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E864: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E5E868: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E86C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E86C size=8
    let mut pc: u32 = 0x82E5E86C;
    'dispatch: loop {
        match pc {
            0x82E5E86C => {
    //   block [0x82E5E86C..0x82E5E874)
	// 82E5E86C: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5E870: 4BFFFEA8  b 0x82e5e718
	sub_82E5E718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5E874 size=4
    let mut pc: u32 = 0x82E5E874;
    'dispatch: loop {
        match pc {
            0x82E5E874 => {
    //   block [0x82E5E874..0x82E5E878)
	// 82E5E874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E878 size=88
    let mut pc: u32 = 0x82E5E878;
    'dispatch: loop {
        match pc {
            0x82E5E878 => {
    //   block [0x82E5E878..0x82E5E8D0)
	// 82E5E878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E884: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E888: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5E88C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5E890: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5E894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5E898: 396B78CC  addi r11, r11, 0x78cc
	ctx.r[11].s64 = ctx.r[11].s64 + 30924;
	// 82E5E89C: 394A78AC  addi r10, r10, 0x78ac
	ctx.r[10].s64 = ctx.r[10].s64 + 30892;
	// 82E5E8A0: 3929789C  addi r9, r9, 0x789c
	ctx.r[9].s64 = ctx.r[9].s64 + 30876;
	// 82E5E8A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5E8A8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5E8AC: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E5E8B0: 4BFFFE69  bl 0x82e5e718
	ctx.lr = 0x82E5E8B4;
	sub_82E5E718(ctx, base);
	// 82E5E8B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5E8B8: 480073F1  bl 0x82e65ca8
	ctx.lr = 0x82E5E8BC;
	sub_82E65CA8(ctx, base);
	// 82E5E8BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5E8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5E8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5E8C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5E8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E5E8D0 size=276
    let mut pc: u32 = 0x82E5E8D0;
    'dispatch: loop {
        match pc {
            0x82E5E8D0 => {
    //   block [0x82E5E8D0..0x82E5E9E4)
	// 82E5E8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E8D4: 4BE4AB35  bl 0x82ca9408
	ctx.lr = 0x82E5E8D8;
	sub_82CA93D0(ctx, base);
	// 82E5E8D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E8DC: 78AB07A0  clrldi r11, r5, 0x3e
	ctx.r[11].u64 = ctx.r[5].u64 & 0x0000000000000003u64;
	// 82E5E8E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5E8E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E5E8E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E5E8EC: 2B2B0003  cmpldi cr6, r11, 3
	ctx.cr[6].compare_u64(ctx.r[11].u64, 3, &mut ctx.xer);
	// 82E5E8F0: 409A00D8  bne cr6, 0x82e5e9c8
	if !ctx.cr[6].eq {
	pc = 0x82E5E9C8; continue 'dispatch;
	}
	// 82E5E8F4: 78A50764  rldicr r5, r5, 0, 0x3d
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(0) & 0xFFFFFFFFFFFFFFFC;
	// 82E5E8F8: 54AB003E  slwi r11, r5, 0
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5E8FC: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5E900: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E5E904: 409A00B0  bne cr6, 0x82e5e9b4
	if !ctx.cr[6].eq {
	pc = 0x82E5E9B4; continue 'dispatch;
	}
	// 82E5E908: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5E90C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E5E910: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E5E914: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5E918: 419A009C  beq cr6, 0x82e5e9b4
	if ctx.cr[6].eq {
	pc = 0x82E5E9B4; continue 'dispatch;
	}
	// 82E5E91C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E920: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5E924: 897E00D8  lbz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E5E928: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E5E92C: 419A0088  beq cr6, 0x82e5e9b4
	if ctx.cr[6].eq {
	pc = 0x82E5E9B4; continue 'dispatch;
	}
	// 82E5E930: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5E934: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E5E938: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E5E93C: 409A0078  bne cr6, 0x82e5e9b4
	if !ctx.cr[6].eq {
	pc = 0x82E5E9B4; continue 'dispatch;
	}
	// 82E5E940: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E5E944: 389E00E0  addi r4, r30, 0xe0
	ctx.r[4].s64 = ctx.r[30].s64 + 224;
	// 82E5E948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5E94C: 4BEF7AA5  bl 0x82d563f0
	ctx.lr = 0x82E5E950;
	sub_82D563F0(ctx, base);
	// 82E5E950: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5E954: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5E958: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E5E95C: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E5E960: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5E964: 4BEF68E5  bl 0x82d55248
	ctx.lr = 0x82E5E968;
	sub_82D55248(ctx, base);
	// 82E5E968: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5E96C: 39000060  li r8, 0x60
	ctx.r[8].s64 = 96;
	// 82E5E970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5E974: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82E5E978: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E5E97C: C06B7384  lfs f3, 0x7384(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29572 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E5E980: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E5E984: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5E988: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82E5E98C: C04B0B24  lfs f2, 0xb24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E5E990: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E5E994: C02B0BFC  lfs f1, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E5E998: 4BFF6479  bl 0x82e54e10
	ctx.lr = 0x82E5E99C;
	sub_82E54E10(ctx, base);
	// 82E5E99C: C03F0030  lfs f1, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E5E9A0: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82E5E9A4: 4BFF637D  bl 0x82e54d20
	ctx.lr = 0x82E5E9A8;
	sub_82E54D20(ctx, base);
	// 82E5E9A8: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E5E9AC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E5E9B0: 4BF1E121  bl 0x82d7cad0
	ctx.lr = 0x82E5E9B4;
	sub_82D7CAD0(ctx, base);
	// 82E5E9B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5E9B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5E9BC: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E5E9C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5E9C4: 4BE4AA94  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E5E9C8: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 82E5E9CC: 419AFF2C  beq cr6, 0x82e5e8f8
	if ctx.cr[6].eq {
	pc = 0x82E5E8F8; continue 'dispatch;
	}
	// 82E5E9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5E9D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5E9D8: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E5E9DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5E9E0: 4BE4AA78  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5E9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5E9E8 size=260
    let mut pc: u32 = 0x82E5E9E8;
    'dispatch: loop {
        match pc {
            0x82E5E9E8 => {
    //   block [0x82E5E9E8..0x82E5EAEC)
	// 82E5E9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5E9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5E9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5E9F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5E9F8: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E5E9FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5EA00: 2F0B00B0  cmpwi cr6, r11, 0xb0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 176, &mut ctx.xer);
	// 82E5EA04: 419A0088  beq cr6, 0x82e5ea8c
	if ctx.cr[6].eq {
	pc = 0x82E5EA8C; continue 'dispatch;
	}
	// 82E5EA08: 2F0B00B1  cmpwi cr6, r11, 0xb1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 177, &mut ctx.xer);
	// 82E5EA0C: 419A0028  beq cr6, 0x82e5ea34
	if ctx.cr[6].eq {
	pc = 0x82E5EA34; continue 'dispatch;
	}
	// 82E5EA10: 2F0B00B2  cmpwi cr6, r11, 0xb2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 178, &mut ctx.xer);
	// 82E5EA14: 409A00C4  bne cr6, 0x82e5ead8
	if !ctx.cr[6].eq {
	pc = 0x82E5EAD8; continue 'dispatch;
	}
	// 82E5EA18: 387FFFF8  addi r3, r31, -8
	ctx.r[3].s64 = ctx.r[31].s64 + -8;
	// 82E5EA1C: 4BFFFCFD  bl 0x82e5e718
	ctx.lr = 0x82E5EA20;
	sub_82E5E718(ctx, base);
	// 82E5EA20: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5EA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5EA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5EA2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5EA30: 4E800020  blr
	return;
	// 82E5EA34: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E5EA38: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EA3C: 4BFDBEED  bl 0x82e3a928
	ctx.lr = 0x82E5EA40;
	sub_82E3A928(ctx, base);
	// 82E5EA40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5EA44: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EA48: 4826F601  bl 0x830ce048
	ctx.lr = 0x82E5EA4C;
	sub_830CE048(ctx, base);
	// 82E5EA4C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EA50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5EA54: 419A0084  beq cr6, 0x82e5ead8
	if ctx.cr[6].eq {
	pc = 0x82E5EAD8; continue 'dispatch;
	}
	// 82E5EA58: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5EA5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5EA60: 419A0078  beq cr6, 0x82e5ead8
	if ctx.cr[6].eq {
	pc = 0x82E5EAD8; continue 'dispatch;
	}
	// 82E5EA64: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EA68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EA6C: 419A006C  beq cr6, 0x82e5ead8
	if ctx.cr[6].eq {
	pc = 0x82E5EAD8; continue 'dispatch;
	}
	// 82E5EA70: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E5EA74: 4BFF61ED  bl 0x82e54c60
	ctx.lr = 0x82E5EA78;
	sub_82E54C60(ctx, base);
	// 82E5EA78: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5EA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5EA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5EA84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5EA88: 4E800020  blr
	return;
	// 82E5EA8C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E5EA90: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EA94: 4BFDBE95  bl 0x82e3a928
	ctx.lr = 0x82E5EA98;
	sub_82E3A928(ctx, base);
	// 82E5EA98: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E5EA9C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E5EAA0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EAA4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E5EAA8: 4826F449  bl 0x830cdef0
	ctx.lr = 0x82E5EAAC;
	sub_830CDEF0(ctx, base);
	// 82E5EAAC: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82E5EAB0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EAB4: 4826F595  bl 0x830ce048
	ctx.lr = 0x82E5EAB8;
	sub_830CE048(ctx, base);
	// 82E5EAB8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EABC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5EAC0: 419A0018  beq cr6, 0x82e5ead8
	if ctx.cr[6].eq {
	pc = 0x82E5EAD8; continue 'dispatch;
	}
	// 82E5EAC4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82E5EAC8: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E5EACC: 389FFFF8  addi r4, r31, -8
	ctx.r[4].s64 = ctx.r[31].s64 + -8;
	// 82E5EAD0: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82E5EAD4: 4BFFFDFD  bl 0x82e5e8d0
	ctx.lr = 0x82E5EAD8;
	sub_82E5E8D0(ctx, base);
	// 82E5EAD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5EADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5EAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5EAE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5EAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EAF0 size=12
    let mut pc: u32 = 0x82E5EAF0;
    'dispatch: loop {
        match pc {
            0x82E5EAF0 => {
    //   block [0x82E5EAF0..0x82E5EAFC)
	// 82E5EAF0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5EAF4: 806BB5E0  lwz r3, -0x4a20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18976 as u32) ) } as u64;
	// 82E5EAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EB00 size=8
    let mut pc: u32 = 0x82E5EB00;
    'dispatch: loop {
        match pc {
            0x82E5EB00 => {
    //   block [0x82E5EB00..0x82E5EB08)
	// 82E5EB00: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5EB04: 4800000C  b 0x82e5eb10
	sub_82E5EB10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EB08 size=8
    let mut pc: u32 = 0x82E5EB08;
    'dispatch: loop {
        match pc {
            0x82E5EB08 => {
    //   block [0x82E5EB08..0x82E5EB10)
	// 82E5EB08: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5EB0C: 48000004  b 0x82e5eb10
	sub_82E5EB10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5EB10 size=144
    let mut pc: u32 = 0x82E5EB10;
    'dispatch: loop {
        match pc {
            0x82E5EB10 => {
    //   block [0x82E5EB10..0x82E5EBA0)
	// 82E5EB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5EB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5EB18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5EB1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5EB20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5EB24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5EB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5EB2C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5EB30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5EB34: 396B78CC  addi r11, r11, 0x78cc
	ctx.r[11].s64 = ctx.r[11].s64 + 30924;
	// 82E5EB38: 394A78AC  addi r10, r10, 0x78ac
	ctx.r[10].s64 = ctx.r[10].s64 + 30892;
	// 82E5EB3C: 3929789C  addi r9, r9, 0x789c
	ctx.r[9].s64 = ctx.r[9].s64 + 30876;
	// 82E5EB40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5EB44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5EB48: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E5EB4C: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E5EB50: 4BFFFBC9  bl 0x82e5e718
	ctx.lr = 0x82E5EB54;
	sub_82E5E718(ctx, base);
	// 82E5EB54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5EB58: 48007151  bl 0x82e65ca8
	ctx.lr = 0x82E5EB5C;
	sub_82E65CA8(ctx, base);
	// 82E5EB5C: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5EB60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5EB64: 419A0020  beq cr6, 0x82e5eb84
	if ctx.cr[6].eq {
	pc = 0x82E5EB84; continue 'dispatch;
	}
	// 82E5EB68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EB6C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5EB70: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5EB74: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EB78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5EB7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5EB80: 4BEF6749  bl 0x82d552c8
	ctx.lr = 0x82E5EB84;
	sub_82D552C8(ctx, base);
	// 82E5EB84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5EB88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5EB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5EB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5EB94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5EB98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5EB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5EBA0 size=420
    let mut pc: u32 = 0x82E5EBA0;
    'dispatch: loop {
        match pc {
            0x82E5EBA0 => {
    //   block [0x82E5EBA0..0x82E5ED44)
	// 82E5EBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5EBA4: 4BE4A851  bl 0x82ca93f4
	ctx.lr = 0x82E5EBA8;
	sub_82CA93D0(ctx, base);
	// 82E5EBA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5EBAC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E5EBB0: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82E5EBB4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E5EBB8: 38980028  addi r4, r24, 0x28
	ctx.r[4].s64 = ctx.r[24].s64 + 40;
	// 82E5EBBC: 409A0008  bne cr6, 0x82e5ebc4
	if !ctx.cr[6].eq {
	pc = 0x82E5EBC4; continue 'dispatch;
	}
	// 82E5EBC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5EBC4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E5EBC8: 4BF19591  bl 0x82d78158
	ctx.lr = 0x82E5EBCC;
	sub_82D78158(ctx, base);
	// 82E5EBCC: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E5EBD0: 3898002C  addi r4, r24, 0x2c
	ctx.r[4].s64 = ctx.r[24].s64 + 44;
	// 82E5EBD4: 409A0008  bne cr6, 0x82e5ebdc
	if !ctx.cr[6].eq {
	pc = 0x82E5EBDC; continue 'dispatch;
	}
	// 82E5EBD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5EBDC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E5EBE0: 4BF19699  bl 0x82d78278
	ctx.lr = 0x82E5EBE4;
	sub_82D78278(ctx, base);
	// 82E5EBE4: 3B570028  addi r26, r23, 0x28
	ctx.r[26].s64 = ctx.r[23].s64 + 40;
	// 82E5EBE8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E5EBEC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EBF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EBF4: 40990070  ble cr6, 0x82e5ec64
	if !ctx.cr[6].gt {
	pc = 0x82E5EC64; continue 'dispatch;
	}
	// 82E5EBF8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E5EBFC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EC00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5EC04: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E5EC08: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5EC0C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EC10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EC14: 4099003C  ble cr6, 0x82e5ec50
	if !ctx.cr[6].gt {
	pc = 0x82E5EC50; continue 'dispatch;
	}
	// 82E5EC18: 3B980028  addi r28, r24, 0x28
	ctx.r[28].s64 = ctx.r[24].s64 + 40;
	// 82E5EC1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5EC20: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EC24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5EC28: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EC2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EC30: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5EC34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5EC38: 4E800421  bctrl
	ctx.lr = 0x82E5EC3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5EC3C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EC40: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5EC44: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5EC48: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EC4C: 4198FFD4  blt cr6, 0x82e5ec20
	if ctx.cr[6].lt {
	pc = 0x82E5EC20; continue 'dispatch;
	}
	// 82E5EC50: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EC54: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5EC58: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E5EC5C: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EC60: 4198FF9C  blt cr6, 0x82e5ebfc
	if ctx.cr[6].lt {
	pc = 0x82E5EBFC; continue 'dispatch;
	}
	// 82E5EC64: 3B570034  addi r26, r23, 0x34
	ctx.r[26].s64 = ctx.r[23].s64 + 52;
	// 82E5EC68: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E5EC6C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EC70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EC74: 40990070  ble cr6, 0x82e5ece4
	if !ctx.cr[6].gt {
	pc = 0x82E5ECE4; continue 'dispatch;
	}
	// 82E5EC78: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E5EC7C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EC80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5EC84: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E5EC88: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5EC8C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EC90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EC94: 4099003C  ble cr6, 0x82e5ecd0
	if !ctx.cr[6].gt {
	pc = 0x82E5ECD0; continue 'dispatch;
	}
	// 82E5EC98: 3B980028  addi r28, r24, 0x28
	ctx.r[28].s64 = ctx.r[24].s64 + 40;
	// 82E5EC9C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5ECA0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5ECA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5ECA8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5ECAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5ECB0: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5ECB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5ECB8: 4E800421  bctrl
	ctx.lr = 0x82E5ECBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5ECBC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5ECC0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5ECC4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5ECC8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5ECCC: 4198FFD4  blt cr6, 0x82e5eca0
	if ctx.cr[6].lt {
	pc = 0x82E5ECA0; continue 'dispatch;
	}
	// 82E5ECD0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5ECD4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5ECD8: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E5ECDC: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5ECE0: 4198FF9C  blt cr6, 0x82e5ec7c
	if ctx.cr[6].lt {
	pc = 0x82E5EC7C; continue 'dispatch;
	}
	// 82E5ECE4: 81770020  lwz r11, 0x20(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E5ECE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5ECEC: 419A0050  beq cr6, 0x82e5ed3c
	if ctx.cr[6].eq {
	pc = 0x82E5ED3C; continue 'dispatch;
	}
	// 82E5ECF0: 3B8B004C  addi r28, r11, 0x4c
	ctx.r[28].s64 = ctx.r[11].s64 + 76;
	// 82E5ECF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5ECF8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5ECFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5ED00: 4099003C  ble cr6, 0x82e5ed3c
	if !ctx.cr[6].gt {
	pc = 0x82E5ED3C; continue 'dispatch;
	}
	// 82E5ED04: 3BB80028  addi r29, r24, 0x28
	ctx.r[29].s64 = ctx.r[24].s64 + 40;
	// 82E5ED08: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5ED0C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5ED10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5ED14: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5ED18: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5ED1C: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5ED20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5ED24: 4E800421  bctrl
	ctx.lr = 0x82E5ED28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5ED28: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5ED2C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5ED30: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5ED34: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5ED38: 4198FFD4  blt cr6, 0x82e5ed0c
	if ctx.cr[6].lt {
	pc = 0x82E5ED0C; continue 'dispatch;
	}
	// 82E5ED3C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E5ED40: 4BE4A704  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5ED48 size=420
    let mut pc: u32 = 0x82E5ED48;
    'dispatch: loop {
        match pc {
            0x82E5ED48 => {
    //   block [0x82E5ED48..0x82E5EEEC)
	// 82E5ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5ED4C: 4BE4A6A9  bl 0x82ca93f4
	ctx.lr = 0x82E5ED50;
	sub_82CA93D0(ctx, base);
	// 82E5ED50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5ED54: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E5ED58: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82E5ED5C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E5ED60: 38980028  addi r4, r24, 0x28
	ctx.r[4].s64 = ctx.r[24].s64 + 40;
	// 82E5ED64: 409A0008  bne cr6, 0x82e5ed6c
	if !ctx.cr[6].eq {
	pc = 0x82E5ED6C; continue 'dispatch;
	}
	// 82E5ED68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5ED6C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E5ED70: 4BF1A8E9  bl 0x82d79658
	ctx.lr = 0x82E5ED74;
	sub_82D79658(ctx, base);
	// 82E5ED74: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E5ED78: 3898002C  addi r4, r24, 0x2c
	ctx.r[4].s64 = ctx.r[24].s64 + 44;
	// 82E5ED7C: 409A0008  bne cr6, 0x82e5ed84
	if !ctx.cr[6].eq {
	pc = 0x82E5ED84; continue 'dispatch;
	}
	// 82E5ED80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5ED84: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E5ED88: 4BF1AA91  bl 0x82d79818
	ctx.lr = 0x82E5ED8C;
	sub_82D79818(ctx, base);
	// 82E5ED8C: 3B570028  addi r26, r23, 0x28
	ctx.r[26].s64 = ctx.r[23].s64 + 40;
	// 82E5ED90: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E5ED94: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5ED98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5ED9C: 40990070  ble cr6, 0x82e5ee0c
	if !ctx.cr[6].gt {
	pc = 0x82E5EE0C; continue 'dispatch;
	}
	// 82E5EDA0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E5EDA4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EDA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5EDAC: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E5EDB0: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5EDB4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EDB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EDBC: 4099003C  ble cr6, 0x82e5edf8
	if !ctx.cr[6].gt {
	pc = 0x82E5EDF8; continue 'dispatch;
	}
	// 82E5EDC0: 3B980028  addi r28, r24, 0x28
	ctx.r[28].s64 = ctx.r[24].s64 + 40;
	// 82E5EDC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5EDC8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EDCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5EDD0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EDD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EDD8: 7C8AF82E  lwzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5EDDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5EDE0: 4E800421  bctrl
	ctx.lr = 0x82E5EDE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5EDE4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EDE8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5EDEC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5EDF0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EDF4: 4198FFD4  blt cr6, 0x82e5edc8
	if ctx.cr[6].lt {
	pc = 0x82E5EDC8; continue 'dispatch;
	}
	// 82E5EDF8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EDFC: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5EE00: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E5EE04: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EE08: 4198FF9C  blt cr6, 0x82e5eda4
	if ctx.cr[6].lt {
	pc = 0x82E5EDA4; continue 'dispatch;
	}
	// 82E5EE0C: 3B570034  addi r26, r23, 0x34
	ctx.r[26].s64 = ctx.r[23].s64 + 52;
	// 82E5EE10: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E5EE14: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EE18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EE1C: 40990070  ble cr6, 0x82e5ee8c
	if !ctx.cr[6].gt {
	pc = 0x82E5EE8C; continue 'dispatch;
	}
	// 82E5EE20: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E5EE24: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EE28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5EE2C: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E5EE30: 3BAB004C  addi r29, r11, 0x4c
	ctx.r[29].s64 = ctx.r[11].s64 + 76;
	// 82E5EE34: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EE38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EE3C: 4099003C  ble cr6, 0x82e5ee78
	if !ctx.cr[6].gt {
	pc = 0x82E5EE78; continue 'dispatch;
	}
	// 82E5EE40: 3B980028  addi r28, r24, 0x28
	ctx.r[28].s64 = ctx.r[24].s64 + 40;
	// 82E5EE44: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5EE48: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EE4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5EE50: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EE54: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EE58: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5EE5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5EE60: 4E800421  bctrl
	ctx.lr = 0x82E5EE64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5EE64: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EE68: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5EE6C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5EE70: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EE74: 4198FFD4  blt cr6, 0x82e5ee48
	if ctx.cr[6].lt {
	pc = 0x82E5EE48; continue 'dispatch;
	}
	// 82E5EE78: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EE7C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E5EE80: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E5EE84: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EE88: 4198FF9C  blt cr6, 0x82e5ee24
	if ctx.cr[6].lt {
	pc = 0x82E5EE24; continue 'dispatch;
	}
	// 82E5EE8C: 81770020  lwz r11, 0x20(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E5EE90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EE94: 419A0050  beq cr6, 0x82e5eee4
	if ctx.cr[6].eq {
	pc = 0x82E5EEE4; continue 'dispatch;
	}
	// 82E5EE98: 3B8B004C  addi r28, r11, 0x4c
	ctx.r[28].s64 = ctx.r[11].s64 + 76;
	// 82E5EE9C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5EEA0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EEA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EEA8: 4099003C  ble cr6, 0x82e5eee4
	if !ctx.cr[6].gt {
	pc = 0x82E5EEE4; continue 'dispatch;
	}
	// 82E5EEAC: 3BB80028  addi r29, r24, 0x28
	ctx.r[29].s64 = ctx.r[24].s64 + 40;
	// 82E5EEB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5EEB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EEB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5EEBC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EEC0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EEC4: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5EEC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5EECC: 4E800421  bctrl
	ctx.lr = 0x82E5EED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5EED0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EED4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E5EED8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E5EEDC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EEE0: 4198FFD4  blt cr6, 0x82e5eeb4
	if ctx.cr[6].lt {
	pc = 0x82E5EEB4; continue 'dispatch;
	}
	// 82E5EEE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E5EEE8: 4BE4A55C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EEF0 size=16
    let mut pc: u32 = 0x82E5EEF0;
    'dispatch: loop {
        match pc {
            0x82E5EEF0 => {
    //   block [0x82E5EEF0..0x82E5EF00)
	// 82E5EEF0: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5EEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5EEF8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E5EEFC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EF00 size=36
    let mut pc: u32 = 0x82E5EF00;
    'dispatch: loop {
        match pc {
            0x82E5EF00 => {
    //   block [0x82E5EF00..0x82E5EF24)
	// 82E5EF00: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EF04: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5EF08: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E5EF0C: 419A0018  beq cr6, 0x82e5ef24
	if ctx.cr[6].eq {
		sub_82E5EF24(ctx, base);
		return;
	}
	// 82E5EF10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5EF14: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5EF18: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E5EF1C: 4198FFE8  blt cr6, 0x82e5ef04
	if ctx.cr[6].lt {
	pc = 0x82E5EF04; continue 'dispatch;
	}
	// 82E5EF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EF24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EF24 size=8
    let mut pc: u32 = 0x82E5EF24;
    'dispatch: loop {
        match pc {
            0x82E5EF24 => {
    //   block [0x82E5EF24..0x82E5EF2C)
	// 82E5EF24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5EF28: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EF2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EF2C size=36
    let mut pc: u32 = 0x82E5EF2C;
    'dispatch: loop {
        match pc {
            0x82E5EF2C => {
    //   block [0x82E5EF2C..0x82E5EF50)
	// 82E5EF2C: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5EF30: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E5EF34: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EF38: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E5EF3C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E5EF40: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E5EF44: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5EF48: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E5EF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5EF50 size=92
    let mut pc: u32 = 0x82E5EF50;
    'dispatch: loop {
        match pc {
            0x82E5EF50 => {
    //   block [0x82E5EF50..0x82E5EFAC)
	// 82E5EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5EF54: 4BE4A4B5  bl 0x82ca9408
	ctx.lr = 0x82E5EF58;
	sub_82CA93D0(ctx, base);
	// 82E5EF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5EF5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E5EF60: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5EF64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5EF68: 419A003C  beq cr6, 0x82e5efa4
	if ctx.cr[6].eq {
	pc = 0x82E5EFA4; continue 'dispatch;
	}
	// 82E5EF6C: 83EB0070  lwz r31, 0x70(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5EF70: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E5EF74: 40990030  ble cr6, 0x82e5efa4
	if !ctx.cr[6].gt {
	pc = 0x82E5EFA4; continue 'dispatch;
	}
	// 82E5EF78: 3B9DFFF8  addi r28, r29, -8
	ctx.r[28].s64 = ctx.r[29].s64 + -8;
	// 82E5EF7C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5EF80: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5EF84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E5EF88: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5EF8C: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E5EF90: 4BFFFDB9  bl 0x82e5ed48
	ctx.lr = 0x82E5EF94;
	sub_82E5ED48(ctx, base);
	// 82E5EF94: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E5EF98: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5EF9C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5EFA0: 409AFFE0  bne cr6, 0x82e5ef80
	if !ctx.cr[6].eq {
	pc = 0x82E5EF80; continue 'dispatch;
	}
	// 82E5EFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5EFA8: 4BE4A4B0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EFB0 size=8
    let mut pc: u32 = 0x82E5EFB0;
    'dispatch: loop {
        match pc {
            0x82E5EFB0 => {
    //   block [0x82E5EFB0..0x82E5EFB8)
	// 82E5EFB0: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5EFB4: 4BFFFBEC  b 0x82e5eba0
	sub_82E5EBA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5EFB8 size=8
    let mut pc: u32 = 0x82E5EFB8;
    'dispatch: loop {
        match pc {
            0x82E5EFB8 => {
    //   block [0x82E5EFB8..0x82E5EFC0)
	// 82E5EFB8: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5EFBC: 4BFFFD8C  b 0x82e5ed48
	sub_82E5ED48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5EFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5EFC0 size=112
    let mut pc: u32 = 0x82E5EFC0;
    'dispatch: loop {
        match pc {
            0x82E5EFC0 => {
    //   block [0x82E5EFC0..0x82E5F030)
	// 82E5EFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5EFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5EFC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5EFCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5EFD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5EFD4: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 82E5EFD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5EFDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5EFE0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5EFE4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5EFE8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5EFEC: 409A0010  bne cr6, 0x82e5effc
	if !ctx.cr[6].eq {
	pc = 0x82E5EFFC; continue 'dispatch;
	}
	// 82E5EFF0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E5EFF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5EFF8: 4BEF7FA1  bl 0x82d56f98
	ctx.lr = 0x82E5EFFC;
	sub_82D56F98(ctx, base);
	// 82E5EFFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5F000: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F004: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5F008: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E5F00C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5F010: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5F014: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5F018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5F01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5F020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5F024: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5F028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5F02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5F030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5F030 size=164
    let mut pc: u32 = 0x82E5F030;
    'dispatch: loop {
        match pc {
            0x82E5F030 => {
    //   block [0x82E5F030..0x82E5F0D4)
	// 82E5F030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5F034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5F038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5F03C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5F040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5F044: 48006D3D  bl 0x82e65d80
	ctx.lr = 0x82E5F048;
	sub_82E65D80(ctx, base);
	// 82E5F048: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5F04C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5F050: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E5F054: 394A7494  addi r10, r10, 0x7494
	ctx.r[10].s64 = ctx.r[10].s64 + 29844;
	// 82E5F058: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5F05C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5F060: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5F064: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5F068: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E5F06C: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E5F070: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82E5F074: 39297944  addi r9, r9, 0x7944
	ctx.r[9].s64 = ctx.r[9].s64 + 31044;
	// 82E5F078: 39087924  addi r8, r8, 0x7924
	ctx.r[8].s64 = ctx.r[8].s64 + 31012;
	// 82E5F07C: 38E77914  addi r7, r7, 0x7914
	ctx.r[7].s64 = ctx.r[7].s64 + 30996;
	// 82E5F080: 38C678FC  addi r6, r6, 0x78fc
	ctx.r[6].s64 = ctx.r[6].s64 + 30972;
	// 82E5F084: 38A578EC  addi r5, r5, 0x78ec
	ctx.r[5].s64 = ctx.r[5].s64 + 30956;
	// 82E5F088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5F08C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5F090: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E5F094: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E5F098: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82E5F09C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5F0A0: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82E5F0A4: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82E5F0A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E5F0AC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E5F0B0: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E5F0B4: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E5F0B8: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E5F0BC: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82E5F0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5F0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5F0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5F0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5F0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5F0D8 size=332
    let mut pc: u32 = 0x82E5F0D8;
    'dispatch: loop {
        match pc {
            0x82E5F0D8 => {
    //   block [0x82E5F0D8..0x82E5F224)
	// 82E5F0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5F0DC: 4BE4A331  bl 0x82ca940c
	ctx.lr = 0x82E5F0E0;
	sub_82CA93D0(ctx, base);
	// 82E5F0E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5F0E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5F0E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5F0EC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5F0F0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E5F0F4: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E5F0F8: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E5F0FC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5F100: 394A7944  addi r10, r10, 0x7944
	ctx.r[10].s64 = ctx.r[10].s64 + 31044;
	// 82E5F104: 39297924  addi r9, r9, 0x7924
	ctx.r[9].s64 = ctx.r[9].s64 + 31012;
	// 82E5F108: 39087914  addi r8, r8, 0x7914
	ctx.r[8].s64 = ctx.r[8].s64 + 30996;
	// 82E5F10C: 38E778FC  addi r7, r7, 0x78fc
	ctx.r[7].s64 = ctx.r[7].s64 + 30972;
	// 82E5F110: 38C678EC  addi r6, r6, 0x78ec
	ctx.r[6].s64 = ctx.r[6].s64 + 30956;
	// 82E5F114: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5F118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5F11C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5F120: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E5F124: 90FF0028  stw r7, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82E5F128: 90DF002C  stw r6, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82E5F12C: 419A0038  beq cr6, 0x82e5f164
	if ctx.cr[6].eq {
	pc = 0x82E5F164; continue 'dispatch;
	}
	// 82E5F130: 83CB0070  lwz r30, 0x70(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5F134: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5F138: 4099002C  ble cr6, 0x82e5f164
	if !ctx.cr[6].gt {
	pc = 0x82E5F164; continue 'dispatch;
	}
	// 82E5F13C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5F140: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5F144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5F148: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5F14C: 7C8BE82E  lwzx r4, r11, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5F150: 4BFFFA51  bl 0x82e5eba0
	ctx.lr = 0x82E5F154;
	sub_82E5EBA0(ctx, base);
	// 82E5F154: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5F158: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E5F15C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5F160: 409AFFE0  bne cr6, 0x82e5f140
	if !ctx.cr[6].eq {
	pc = 0x82E5F140; continue 'dispatch;
	}
	// 82E5F164: 83DF0040  lwz r30, 0x40(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E5F168: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E5F16C: 40990034  ble cr6, 0x82e5f1a0
	if !ctx.cr[6].gt {
	pc = 0x82E5F1A0; continue 'dispatch;
	}
	// 82E5F170: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5F174: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E5F178: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5F17C: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E5F180: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F184: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F188: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5F18C: 4E800421  bctrl
	ctx.lr = 0x82E5F190;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5F190: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5F194: 3BBD0070  addi r29, r29, 0x70
	ctx.r[29].s64 = ctx.r[29].s64 + 112;
	// 82E5F198: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5F19C: 409AFFD8  bne cr6, 0x82e5f174
	if !ctx.cr[6].eq {
	pc = 0x82E5F174; continue 'dispatch;
	}
	// 82E5F1A0: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E5F1A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5F1A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5F1AC: 409A0024  bne cr6, 0x82e5f1d0
	if !ctx.cr[6].eq {
	pc = 0x82E5F1D0; continue 'dispatch;
	}
	// 82E5F1B0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F1B4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5F1B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5F1BC: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E5F1C0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5F1C4: 1CAB0070  mulli r5, r11, 0x70
	ctx.r[5].s64 = ctx.r[11].s64 * 112;
	// 82E5F1C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5F1CC: 4BEF60FD  bl 0x82d552c8
	ctx.lr = 0x82E5F1D0;
	sub_82D552C8(ctx, base);
	// 82E5F1D0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E5F1D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5F1D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5F1DC: 409A0020  bne cr6, 0x82e5f1fc
	if !ctx.cr[6].eq {
	pc = 0x82E5F1FC; continue 'dispatch;
	}
	// 82E5F1E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F1E4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5F1E8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5F1EC: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5F1F0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5F1F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5F1F8: 4BEF60D1  bl 0x82d552c8
	ctx.lr = 0x82E5F1FC;
	sub_82D552C8(ctx, base);
	// 82E5F1FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5F200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5F204: 396B7494  addi r11, r11, 0x7494
	ctx.r[11].s64 = ctx.r[11].s64 + 29844;
	// 82E5F208: 394A5D40  addi r10, r10, 0x5d40
	ctx.r[10].s64 = ctx.r[10].s64 + 23872;
	// 82E5F20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5F210: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E5F214: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82E5F218: 48006A91  bl 0x82e65ca8
	ctx.lr = 0x82E5F21C;
	sub_82E65CA8(ctx, base);
	// 82E5F21C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5F220: 4BE4A23C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5F228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5F228 size=96
    let mut pc: u32 = 0x82E5F228;
    'dispatch: loop {
        match pc {
            0x82E5F228 => {
    //   block [0x82E5F228..0x82E5F288)
	// 82E5F228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5F22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5F230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5F234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5F238: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F23C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5F240: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E5F244: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 82E5F248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5F24C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5F250: 4BEF5FF9  bl 0x82d55248
	ctx.lr = 0x82E5F254;
	sub_82D55248(ctx, base);
	// 82E5F254: 39600048  li r11, 0x48
	ctx.r[11].s64 = 72;
	// 82E5F258: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5F25C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E5F260: 4BFFFDD1  bl 0x82e5f030
	ctx.lr = 0x82E5F264;
	sub_82E5F030(ctx, base);
	// 82E5F264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5F268: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E5F26C: 409A0008  bne cr6, 0x82e5f274
	if !ctx.cr[6].eq {
	pc = 0x82E5F274; continue 'dispatch;
	}
	// 82E5F270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5F274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5F278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5F27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5F280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5F284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5F288 size=64
    let mut pc: u32 = 0x82E5F288;
    'dispatch: loop {
        match pc {
            0x82E5F288 => {
    //   block [0x82E5F288..0x82E5F2C8)
	// 82E5F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5F290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5F294: 3D6082E6  lis r11, -0x7d1a
	ctx.r[11].s64 = -2098855936;
	// 82E5F298: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E5F29C: 38ABF228  addi r5, r11, -0xdd8
	ctx.r[5].s64 = ctx.r[11].s64 + -3544;
	// 82E5F2A0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5F2A4: 388A78D8  addi r4, r10, 0x78d8
	ctx.r[4].s64 = ctx.r[10].s64 + 30936;
	// 82E5F2A8: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E5F2AC: 4BFD99ED  bl 0x82e38c98
	ctx.lr = 0x82E5F2B0;
	sub_82E38C98(ctx, base);
	// 82E5F2B0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5F2B4: 906BB5E4  stw r3, -0x4a1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18972 as u32), ctx.r[3].u32 ) };
	// 82E5F2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5F2C8 size=1928
    let mut pc: u32 = 0x82E5F2C8;
    'dispatch: loop {
        match pc {
            0x82E5F2C8 => {
    //   block [0x82E5F2C8..0x82E5FA50)
	// 82E5F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5F2CC: 4BE4A105  bl 0x82ca93d0
	ctx.lr = 0x82E5F2D0;
	sub_82CA93D0(ctx, base);
	// 82E5F2D0: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82E5F2D4: 4BE4E9FD  bl 0x82cadcd0
	ctx.lr = 0x82E5F2D8;
	sub_82CADCA0(ctx, base);
	// 82E5F2D8: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5F2DC: 824D0000  lwz r18, 0(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F2E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82E5F2E4: 7C6E1B78  mr r14, r3
	ctx.r[14].u64 = ctx.r[3].u64;
	// 82E5F2E8: 7D52402E  lwzx r10, r18, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E5F2EC: 92410050  stw r18, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[18].u32 ) };
	// 82E5F2F0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5F2F4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5F2F8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5F2FC: 40980020  bge cr6, 0x82e5f31c
	if !ctx.cr[6].lt {
	pc = 0x82E5F31C; continue 'dispatch;
	}
	// 82E5F300: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5F304: 3929796C  addi r9, r9, 0x796c
	ctx.r[9].s64 = ctx.r[9].s64 + 31084;
	// 82E5F308: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5F30C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5F310: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82E5F314: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5F318: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E5F31C: 7D52402E  lwzx r10, r18, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E5F320: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5F324: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5F328: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E5F32C: 40980020  bge cr6, 0x82e5f34c
	if !ctx.cr[6].lt {
	pc = 0x82E5F34C; continue 'dispatch;
	}
	// 82E5F330: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E5F334: 39297960  addi r9, r9, 0x7960
	ctx.r[9].s64 = ctx.r[9].s64 + 31072;
	// 82E5F338: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5F33C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E5F340: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82E5F344: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E5F348: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E5F34C: 3B2E0010  addi r25, r14, 0x10
	ctx.r[25].s64 = ctx.r[14].s64 + 16;
	// 82E5F350: 838E0008  lwz r28, 8(r14)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5F354: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5F358: 39E00001  li r15, 1
	ctx.r[15].s64 = 1;
	// 82E5F35C: 3A8B5BAC  addi r20, r11, 0x5bac
	ctx.r[20].s64 = ctx.r[11].s64 + 23468;
	// 82E5F360: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E5F364: 83B90004  lwz r29, 4(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5F368: 3A00FFF0  li r16, -0x10
	ctx.r[16].s64 = -16;
	// 82E5F36C: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82E5F370: 3AA0FFE0  li r21, -0x20
	ctx.r[21].s64 = -32;
	// 82E5F374: 3A20FFD0  li r17, -0x30
	ctx.r[17].s64 = -48;
	// 82E5F378: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82E5F37C: 3B000020  li r24, 0x20
	ctx.r[24].s64 = 32;
	// 82E5F380: 7F1CE800  cmpw cr6, r28, r29
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E5F384: 3A6B5C34  addi r19, r11, 0x5c34
	ctx.r[19].s64 = ctx.r[11].s64 + 23604;
	// 82E5F388: 4098003C  bge cr6, 0x82e5f3c4
	if !ctx.cr[6].lt {
	pc = 0x82E5F3C4; continue 'dispatch;
	}
	// 82E5F38C: 1FDC0070  mulli r30, r28, 0x70
	ctx.r[30].s64 = ctx.r[28].s64 * 112;
	// 82E5F390: 7FFCE850  subf r31, r28, r29
	ctx.r[31].s64 = ctx.r[29].s64 - ctx.r[28].s64;
	// 82E5F394: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F398: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5F39C: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E5F3A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F3A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F3A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5F3AC: 4E800421  bctrl
	ctx.lr = 0x82E5F3B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5F3B0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E5F3B4: 3BDE0070  addi r30, r30, 0x70
	ctx.r[30].s64 = ctx.r[30].s64 + 112;
	// 82E5F3B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5F3BC: 409AFFD8  bne cr6, 0x82e5f394
	if !ctx.cr[6].eq {
	pc = 0x82E5F394; continue 'dispatch;
	}
	// 82E5F3C0: 48000174  b 0x82e5f534
	pc = 0x82E5F534; continue 'dispatch;
	// 82E5F3C4: 83590008  lwz r26, 8(r25)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5F3C8: 574B00BE  clrlwi r11, r26, 2
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5F3CC: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5F3D0: 4099012C  ble cr6, 0x82e5f4fc
	if !ctx.cr[6].gt {
	pc = 0x82E5F4FC; continue 'dispatch;
	}
	// 82E5F3D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5F3D8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5F3DC: 41980008  blt cr6, 0x82e5f3e4
	if ctx.cr[6].lt {
	pc = 0x82E5F3E4; continue 'dispatch;
	}
	// 82E5F3E0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E5F3E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5F3E8: 83790000  lwz r27, 0(r25)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F3EC: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E5F3F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5F3F4: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5F3F8: 91590004  stw r10, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E5F3FC: 91390008  stw r9, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E5F400: 4099001C  ble cr6, 0x82e5f41c
	if !ctx.cr[6].gt {
	pc = 0x82E5F41C; continue 'dispatch;
	}
	// 82E5F404: 40980008  bge cr6, 0x82e5f40c
	if !ctx.cr[6].lt {
	pc = 0x82E5F40C; continue 'dispatch;
	}
	// 82E5F408: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E5F40C: 38A00070  li r5, 0x70
	ctx.r[5].s64 = 112;
	// 82E5F410: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5F414: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E5F418: 4BEF7AF9  bl 0x82d56f10
	ctx.lr = 0x82E5F41C;
	sub_82D56F10(ctx, base);
	// 82E5F41C: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5F420: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E5F424: 40990074  ble cr6, 0x82e5f498
	if !ctx.cr[6].gt {
	pc = 0x82E5F498; continue 'dispatch;
	}
	// 82E5F428: 39690040  addi r11, r9, 0x40
	ctx.r[11].s64 = ctx.r[9].s64 + 64;
	// 82E5F42C: 395B0030  addi r10, r27, 0x30
	ctx.r[10].s64 = ctx.r[27].s64 + 48;
	// 82E5F430: 7D3B4850  subf r9, r27, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[27].s64;
	// 82E5F434: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82E5F438: 34EBFFC0  addic. r7, r11, -0x40
	ctx.xer.ca = (ctx.r[11].u32 > (!(-64 as u32)));
	ctx.r[7].s64 = ctx.r[11].s64 + -64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E5F43C: 41820048  beq 0x82e5f484
	if ctx.cr[0].eq {
	pc = 0x82E5F484; continue 'dispatch;
	}
	// 82E5F440: 928BFFC0  stw r20, -0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-64 as u32), ctx.r[20].u32 ) };
	// 82E5F444: B1EBFFC6  sth r15, -0x3a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-58 as u32), ctx.r[15].u16 ) };
	// 82E5F448: 80EAFFD8  lwz r7, -0x28(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-40 as u32) ) } as u64;
	// 82E5F44C: 90EBFFC8  stw r7, -0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-56 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5FA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5FA50 size=12
    let mut pc: u32 = 0x82E5FA50;
    'dispatch: loop {
        match pc {
            0x82E5FA50 => {
    //   block [0x82E5FA50..0x82E5FA5C)
	// 82E5FA50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E5FA54: 806BB5E4  lwz r3, -0x4a1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18972 as u32) ) } as u64;
	// 82E5FA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5FA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5FA60 size=8
    let mut pc: u32 = 0x82E5FA60;
    'dispatch: loop {
        match pc {
            0x82E5FA60 => {
    //   block [0x82E5FA60..0x82E5FA68)
	// 82E5FA60: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5FA64: 4800001C  b 0x82e5fa80
	sub_82E5FA80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5FA68 size=8
    let mut pc: u32 = 0x82E5FA68;
    'dispatch: loop {
        match pc {
            0x82E5FA68 => {
    //   block [0x82E5FA68..0x82E5FA70)
	// 82E5FA68: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82E5FA6C: 48000014  b 0x82e5fa80
	sub_82E5FA80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5FA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5FA70 size=8
    let mut pc: u32 = 0x82E5FA70;
    'dispatch: loop {
        match pc {
            0x82E5FA70 => {
    //   block [0x82E5FA70..0x82E5FA78)
	// 82E5FA70: 3863FFD4  addi r3, r3, -0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + -44;
	// 82E5FA74: 4800000C  b 0x82e5fa80
	sub_82E5FA80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5FA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5FA78 size=8
    let mut pc: u32 = 0x82E5FA78;
    'dispatch: loop {
        match pc {
            0x82E5FA78 => {
    //   block [0x82E5FA78..0x82E5FA80)
	// 82E5FA78: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E5FA7C: 48000004  b 0x82e5fa80
	sub_82E5FA80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5FA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E5FA80 size=100
    let mut pc: u32 = 0x82E5FA80;
    'dispatch: loop {
        match pc {
            0x82E5FA80 => {
    //   block [0x82E5FA80..0x82E5FAE4)
	// 82E5FA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5FA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E5FA88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5FA8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5FA90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5FA94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5FA98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5FA9C: 4BFFF63D  bl 0x82e5f0d8
	ctx.lr = 0x82E5FAA0;
	sub_82E5F0D8(ctx, base);
	// 82E5FAA0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E5FAA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5FAA8: 419A0020  beq cr6, 0x82e5fac8
	if ctx.cr[6].eq {
	pc = 0x82E5FAC8; continue 'dispatch;
	}
	// 82E5FAAC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5FAB0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E5FAB4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E5FAB8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5FABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5FAC0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5FAC4: 4BEF5805  bl 0x82d552c8
	ctx.lr = 0x82E5FAC8;
	sub_82D552C8(ctx, base);
	// 82E5FAC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5FACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5FAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5FAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5FAD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5FADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5FAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


