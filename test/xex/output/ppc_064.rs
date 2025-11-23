pub fn sub_82529810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529810 size=4
    let mut pc: u32 = 0x82529810;
    'dispatch: loop {
        match pc {
            0x82529810 => {
    //   block [0x82529810..0x82529814)
	// 82529810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529818 size=40
    let mut pc: u32 = 0x82529818;
    'dispatch: loop {
        match pc {
            0x82529818 => {
    //   block [0x82529818..0x82529840)
	// 82529818: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252981C: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 82529820: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82529824: 5469043E  clrlwi r9, r3, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82529828: 7D4A5830  slw r10, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8252982C: 7CAB5830  slw r11, r5, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82529830: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82529834: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82529838: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8252983C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529840 size=308
    let mut pc: u32 = 0x82529840;
    'dispatch: loop {
        match pc {
            0x82529840 => {
    //   block [0x82529840..0x82529974)
	// 82529840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82529844: 4800B869  bl 0x825350ac
	ctx.lr = 0x82529848;
	sub_82535080(ctx, base);
	// 82529848: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 8252984C: 39250001  addi r9, r5, 1
	ctx.r[9].s64 = ctx.r[5].s64 + 1;
	// 82529850: 616B5556  ori r11, r11, 0x5556
	ctx.r[11].u64 = ctx.r[11].u64 | 21846;
	// 82529854: 39050002  addi r8, r5, 2
	ctx.r[8].s64 = ctx.r[5].s64 + 2;
	// 82529858: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8252985C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82529860: 7D655896  mulhw r11, r5, r11
	ctx.r[11].s64 = ((ctx.r[5].s32 as i64 * ctx.r[11].s32 as i64) >> 32);
	// 82529864: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82529868: 7C89F896  mulhw r4, r9, r31
	ctx.r[4].s64 = ((ctx.r[9].s32 as i64 * ctx.r[31].s32 as i64) >> 32);
	// 8252986C: 7FE8F096  mulhw r31, r8, r30
	ctx.r[31].s64 = ((ctx.r[8].s32 as i64 * ctx.r[30].s32 as i64) >> 32);
	// 82529870: 557E0FFE  srwi r30, r11, 0x1f
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82529874: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82529878: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 8252987C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82529880: 549E0FFE  srwi r30, r4, 0x1f
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shr(31);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82529884: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 82529888: 7C84F214  add r4, r4, r30
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[30].u64;
	// 8252988C: 57FE0FFE  srwi r30, r31, 0x1f
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shr(31);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82529890: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82529894: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82529898: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8252989C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825298A0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825298A4: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 825298A8: 7D6B2850  subf r11, r11, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 825298AC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529978 size=108
    let mut pc: u32 = 0x82529978;
    'dispatch: loop {
        match pc {
            0x82529978 => {
    //   block [0x82529978..0x825299E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825299E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825299E8 size=352
    let mut pc: u32 = 0x825299E8;
    'dispatch: loop {
        match pc {
            0x825299E8 => {
    //   block [0x825299E8..0x82529B48)
	// 825299E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825299EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825299F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825299F4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82529B48 size=148
    let mut pc: u32 = 0x82529B48;
    'dispatch: loop {
        match pc {
            0x82529B48 => {
    //   block [0x82529B48..0x82529B74)
	// 82529B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82529B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82529B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82529B54: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82529B58: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82529B5C: 4BFFFE8D  bl 0x825299e8
	ctx.lr = 0x82529B60;
	sub_825299E8(ctx, base);
	// 82529B60: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82529B64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82529B68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82529B6C: C18A2930  lfs f12, 0x2930(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10544 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82529B70: C1AB4598  lfs f13, 0x4598(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17816 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	pc = 0x82529B74; continue 'dispatch;
            }
            0x82529B74 => {
    //   block [0x82529B74..0x82529BB0)
	// 82529B74: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82529B78: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82529B7C: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82529B80: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82529B84: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82529B88: EC006378  fmsubs f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82529B8C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82529B90: 40990020  ble cr6, 0x82529bb0
	if !ctx.cr[6].gt {
	pc = 0x82529BB0; continue 'dispatch;
	}
	// 82529B94: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82529B98: 2F03001F  cmpwi cr6, r3, 0x1f
	ctx.cr[6].compare_i32(ctx.r[3].s32, 31, &mut ctx.xer);
	// 82529B9C: 4198FFD8  blt cr6, 0x82529b74
	if ctx.cr[6].lt {
	pc = 0x82529B74; continue 'dispatch;
	}
	// 82529BA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82529BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82529BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82529BAC: 4E800020  blr
	return;
            }
            0x82529BB0 => {
    //   block [0x82529BB0..0x82529BCC)
	// 82529BB0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82529BB4: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82529BB8: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82529BBC: 40990010  ble cr6, 0x82529bcc
	if !ctx.cr[6].gt {
	pc = 0x82529BCC; continue 'dispatch;
	}
	// 82529BC0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82529BC4: 419A0008  beq cr6, 0x82529bcc
	if ctx.cr[6].eq {
	pc = 0x82529BCC; continue 'dispatch;
	}
	// 82529BC8: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	pc = 0x82529BCC; continue 'dispatch;
            }
            0x82529BCC => {
    //   block [0x82529BCC..0x82529BDC)
	// 82529BCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82529BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82529BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82529BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82529BE0 size=524
    let mut pc: u32 = 0x82529BE0;
    'dispatch: loop {
        match pc {
            0x82529BE0 => {
    //   block [0x82529BE0..0x82529DEC)
	// 82529BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82529BE4: 4800B4B5  bl 0x82535098
	ctx.lr = 0x82529BE8;
	sub_82535080(ctx, base);
	// 82529BE8: 9421FA90  stwu r1, -0x570(r1)
	ea = ctx.r[1].u32.wrapping_add(-1392 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82529BEC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82529BF0: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82529BF4: 390100FC  addi r8, r1, 0xfc
	ctx.r[8].s64 = ctx.r[1].s64 + 252;
	// 82529BF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82529BFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82529C00: C1AA1FF8  lfs f13, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82529C04: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82529C08: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82529C0C: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 82529C10: 910100F0  stw r8, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[8].u32 ) };
	// 82529C14: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82529C18: C00B4910  lfs f0, 0x4910(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18704 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82529C1C: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 82529C20: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82529C24: 61080080  ori r8, r8, 0x80
	ctx.r[8].u64 = ctx.r[8].u64 | 128;
	// 82529C28: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82529C2C: 61695556  ori r9, r11, 0x5556
	ctx.r[9].u64 = ctx.r[11].u64 | 21846;
	// 82529C30: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82529C34: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82529C38: C00A20AC  lfs f0, 0x20ac(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8364 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82529C3C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529C40: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82529C44: 3B450020  addi r26, r5, 0x20
	ctx.r[26].s64 = ctx.r[5].s64 + 32;
	// 82529C48: 910100F8  stw r8, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[8].u32 ) };
	// 82529C4C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82529C50: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82529C54: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82529C58: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82529C5C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82529C60: 810A002C  lwz r8, 0x2c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82529C64: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82529C68: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82529C6C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82529C70: 38A100F0  addi r5, r1, 0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + 240;
	// 82529C74: 92A100F4  stw r21, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[21].u32 ) };
	// 82529C78: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82529DF0 size=188
    let mut pc: u32 = 0x82529DF0;
    'dispatch: loop {
        match pc {
            0x82529DF0 => {
    //   block [0x82529DF0..0x82529E6C)
	// 82529DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82529DF4: 4800B2C1  bl 0x825350b4
	ctx.lr = 0x82529DF8;
	sub_82535080(ctx, base);
	// 82529DF8: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82529DFC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82529E00: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82529E04: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82529E08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82529E0C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529E10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82529E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529E18: 4E800421  bctrl
	ctx.lr = 0x82529E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82529E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529E20: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82529E24: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82529E28: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82529E2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529E30: 4E800421  bctrl
	ctx.lr = 0x82529E34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82529E34: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82529E38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82529E3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82529E40: 38DE0040  addi r6, r30, 0x40
	ctx.r[6].s64 = ctx.r[30].s64 + 64;
	// 82529E44: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82529E48: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82529E4C: C02BE098  lfs f1, -0x1f68(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82529E50: 480002F1  bl 0x8252a140
	ctx.lr = 0x82529E54;
	sub_8252A140(ctx, base);
	// 82529E54: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82529E5C: 419A0010  beq cr6, 0x82529e6c
	if ctx.cr[6].eq {
	pc = 0x82529E6C; continue 'dispatch;
	}
	// 82529E60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82529E64: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82529E68: 4800B29C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82529E6C => {
    //   block [0x82529E6C..0x82529E74)
	// 82529E6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82529E70: B3E10052  sth r31, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[31].u16 ) };
	pc = 0x82529E74; continue 'dispatch;
            }
            0x82529E74 => {
    //   block [0x82529E74..0x82529EAC)
	// 82529E74: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82529E78: 39010052  addi r8, r1, 0x52
	ctx.r[8].s64 = ctx.r[1].s64 + 82;
	// 82529E7C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82529E80: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82529E84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82529E88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82529E8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82529E90: 4BFFFD51  bl 0x82529be0
	ctx.lr = 0x82529E94;
	sub_82529BE0(ctx, base);
	// 82529E94: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82529E98: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 82529E9C: 4198FFD8  blt cr6, 0x82529e74
	if ctx.cr[6].lt {
	pc = 0x82529E74; continue 'dispatch;
	}
	// 82529EA0: A0610052  lhz r3, 0x52(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 82529EA4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82529EA8: 4800B25C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82529EB0 size=312
    let mut pc: u32 = 0x82529EB0;
    'dispatch: loop {
        match pc {
            0x82529EB0 => {
    //   block [0x82529EB0..0x82529F58)
	// 82529EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82529EB4: 4800B1F5  bl 0x825350a8
	ctx.lr = 0x82529EB8;
	sub_82535080(ctx, base);
	// 82529EB8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82529EBC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82529EC0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82529EC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82529EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82529ECC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529ED0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82529ED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529ED8: 4E800421  bctrl
	ctx.lr = 0x82529EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82529EDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529EE0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82529EE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82529EE8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82529EEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529EF0: 4E800421  bctrl
	ctx.lr = 0x82529EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82529EF4: 3F808293  lis r28, -0x7d6d
	ctx.r[28].s64 = -2104295424;
	// 82529EF8: 3D60FE98  lis r11, -0x168
	ctx.r[11].s64 = -23592960;
	// 82529EFC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82529F00: 617A751E  ori r26, r11, 0x751e
	ctx.r[26].u64 = ctx.r[11].u64 | 29982;
	// 82529F04: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82529F08: 809C9190  lwz r4, -0x6e70(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82529F0C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82529F10: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82529F14: B3210052  sth r25, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[25].u16 ) };
	// 82529F18: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529F1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82529F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529F24: 4E800421  bctrl
	ctx.lr = 0x82529F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82529F28: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82529F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82529F30: 419A0028  beq cr6, 0x82529f58
	if ctx.cr[6].eq {
	pc = 0x82529F58; continue 'dispatch;
	}
	// 82529F34: 807C9190  lwz r3, -0x6e70(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82529F38: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82529F3C: 9B210050  stb r25, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 82529F40: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82529F44: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529F48: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529F4C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82529F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529F54: 4E800421  bctrl
	ctx.lr = 0x82529F58;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82529F58 => {
    //   block [0x82529F58..0x82529F5C)
	// 82529F58: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	pc = 0x82529F5C; continue 'dispatch;
            }
            0x82529F5C => {
    //   block [0x82529F5C..0x82529FC8)
	// 82529F5C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82529F60: 39010052  addi r8, r1, 0x52
	ctx.r[8].s64 = ctx.r[1].s64 + 82;
	// 82529F64: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82529F68: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82529F6C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82529F70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82529F74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82529F78: 4BFFFC69  bl 0x82529be0
	ctx.lr = 0x82529F7C;
	sub_82529BE0(ctx, base);
	// 82529F7C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529F80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82529F84: 419A0054  beq cr6, 0x82529fd8
	if ctx.cr[6].eq {
	pc = 0x82529FD8; continue 'dispatch;
	}
	// 82529F88: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82529F8C: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 82529F90: 4198FFCC  blt cr6, 0x82529f5c
	if ctx.cr[6].lt {
	pc = 0x82529F5C; continue 'dispatch;
	}
	// 82529F94: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82529F98: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82529F9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82529FA0: 419A0028  beq cr6, 0x82529fc8
	if ctx.cr[6].eq {
	pc = 0x82529FC8; continue 'dispatch;
	}
	// 82529FA4: 807C9190  lwz r3, -0x6e70(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82529FA8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82529FAC: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82529FB0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82529FB4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529FB8: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529FBC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82529FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529FC4: 4E800421  bctrl
	ctx.lr = 0x82529FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82529FC8 => {
    //   block [0x82529FC8..0x82529FD8)
	// 82529FC8: 9BF80000  stb r31, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 82529FCC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82529FD0: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82529FD4: 4800B124  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82529FD8 => {
    //   block [0x82529FD8..0x82529FE8)
	// 82529FD8: 9B380000  stb r25, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82529FDC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82529FE0: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82529FE4: 4800B114  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529FE8 size=112
    let mut pc: u32 = 0x82529FE8;
    'dispatch: loop {
        match pc {
            0x82529FE8 => {
    //   block [0x82529FE8..0x8252A058)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252A058 size=232
    let mut pc: u32 = 0x8252A058;
    'dispatch: loop {
        match pc {
            0x8252A058 => {
    //   block [0x8252A058..0x8252A078)
	// 8252A058: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252A05C: C1AB4910  lfs f13, 0x4910(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252A060: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252A064: 40990014  ble cr6, 0x8252a078
	if !ctx.cr[6].gt {
	pc = 0x8252A078; continue 'dispatch;
	}
	// 8252A068: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252A06C: C1AB20AC  lfs f13, 0x20ac(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8364 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252A070: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252A074: 4198FFD8  blt cr6, 0x8252a04c
	if ctx.cr[6].lt {
		sub_82529FE8(ctx, base);
		return;
	}
	pc = 0x8252A078; continue 'dispatch;
            }
            0x8252A078 => {
    //   block [0x8252A078..0x8252A140)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A140 size=224
    let mut pc: u32 = 0x8252A140;
    'dispatch: loop {
        match pc {
            0x8252A140 => {
    //   block [0x8252A140..0x8252A220)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A220 size=8
    let mut pc: u32 = 0x8252A220;
    'dispatch: loop {
        match pc {
            0x8252A220 => {
    //   block [0x8252A220..0x8252A228)
	// 8252A220: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252A224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A228 size=20
    let mut pc: u32 = 0x8252A228;
    'dispatch: loop {
        match pc {
            0x8252A228 => {
    //   block [0x8252A228..0x8252A23C)
	// 8252A228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252A22C: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8252A230: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8252A234: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8252A238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A240 size=8
    let mut pc: u32 = 0x8252A240;
    'dispatch: loop {
        match pc {
            0x8252A240 => {
    //   block [0x8252A240..0x8252A248)
	// 8252A240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8252A244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A248 size=28
    let mut pc: u32 = 0x8252A248;
    'dispatch: loop {
        match pc {
            0x8252A248 => {
    //   block [0x8252A248..0x8252A264)
	// 8252A248: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252A24C: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 8252A250: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8252A254: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8252A258: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 8252A25C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8252A260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A268 size=36
    let mut pc: u32 = 0x8252A268;
    'dispatch: loop {
        match pc {
            0x8252A268 => {
    //   block [0x8252A268..0x8252A28C)
	// 8252A268: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252A26C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252A270: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252A274: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8252A278: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8252A27C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A280: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252A284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A288: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A290 size=16
    let mut pc: u32 = 0x8252A290;
    'dispatch: loop {
        match pc {
            0x8252A290 => {
    //   block [0x8252A290..0x8252A2A0)
	// 8252A290: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A2A0 size=80
    let mut pc: u32 = 0x8252A2A0;
    'dispatch: loop {
        match pc {
            0x8252A2A0 => {
    //   block [0x8252A2A0..0x8252A2F0)
	// 8252A2A0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8252A2A4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8252A2A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8252A2AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8252A2B0: 810A001C  lwz r8, 0x1c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252A2B4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8252A2B8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8252A2BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8252A2C0: 80EA0018  lwz r7, 0x18(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252A2C4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252A2F0 size=172
    let mut pc: u32 = 0x8252A2F0;
    'dispatch: loop {
        match pc {
            0x8252A2F0 => {
    //   block [0x8252A2F0..0x8252A394)
	// 8252A2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A2F4: 4800ADB9  bl 0x825350ac
	ctx.lr = 0x8252A2F8;
	sub_82535080(ctx, base);
	// 8252A2F8: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A2FC: 3BA5FFFF  addi r29, r5, -1
	ctx.r[29].s64 = ctx.r[5].s64 + -1;
	// 8252A300: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8252A304: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8252A308: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8252A30C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8252A310: 41980084  blt cr6, 0x8252a394
	if ctx.cr[6].lt {
	pc = 0x8252A394; continue 'dispatch;
	}
	// 8252A314: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 8252A318: 3B200003  li r25, 3
	ctx.r[25].s64 = 3;
	// 8252A31C: 617A5556  ori r26, r11, 0x5556
	ctx.r[26].u64 = ctx.r[11].u64 | 21846;
	// 8252A320: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252A324: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8252A328: A3FC0000  lhz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A32C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8252A330: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A334: 7D7FCBD6  divw r11, r31, r25
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[25].s32;
	// 8252A338: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252A33C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A340: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252A344: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252A348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A34C: 4E800421  bctrl
	ctx.lr = 0x8252A350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A350: 7D7FD096  mulhw r11, r31, r26
	ctx.r[11].s64 = ((ctx.r[31].s32 as i64 * ctx.r[26].s32 as i64) >> 32);
	// 8252A354: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252A358: 67E93F00  oris r9, r31, 0x3f00
	ctx.r[9].u64 = ctx.r[31].u64 | 1056964608;
	// 8252A35C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252A360: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8252A364: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252A368: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 8252A36C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252A370: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8252A374: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8252A378: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8252A37C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            0x8252A394 => {
    //   block [0x8252A394..0x8252A39C)
	// 8252A394: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 8252A398: 4800AD64  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252A3A0 size=68
    let mut pc: u32 = 0x8252A3A0;
    'dispatch: loop {
        match pc {
            0x8252A3A0 => {
    //   block [0x8252A3A0..0x8252A3E4)
	// 8252A3A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252A3A4: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252A3A8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252A3AC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252A3B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8252A3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8252A3B8: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 8252A3BC: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8252A3C0: 394A5B84  addi r10, r10, 0x5b84
	ctx.r[10].s64 = ctx.r[10].s64 + 23428;
	// 8252A3C4: 39295B60  addi r9, r9, 0x5b60
	ctx.r[9].s64 = ctx.r[9].s64 + 23392;
	// 8252A3C8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8252A3CC: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8252A3D0: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8252A3D4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8252A3D8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252A3DC: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8252A3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252A3E8 size=244
    let mut pc: u32 = 0x8252A3E8;
    'dispatch: loop {
        match pc {
            0x8252A3E8 => {
    //   block [0x8252A3E8..0x8252A4DC)
	// 8252A3E8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8252A3EC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8252A3F0: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 8252A3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8252A3F8: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252A3FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252A400: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252A4E0 size=240
    let mut pc: u32 = 0x8252A4E0;
    'dispatch: loop {
        match pc {
            0x8252A4E0 => {
    //   block [0x8252A4E0..0x8252A5BC)
	// 8252A4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A4E4: 4800ABC1  bl 0x825350a4
	ctx.lr = 0x8252A4E8;
	sub_82535080(ctx, base);
	// 8252A4E8: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8252A4EC: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A4F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252A4F4: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252A4F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8252A4FC: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8252A500: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8252A504: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8252A508: C3EB0DA0  lfs f31, 0xda0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3488 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8252A50C: 3AE30028  addi r23, r3, 0x28
	ctx.r[23].s64 = ctx.r[3].s64 + 40;
	// 8252A510: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252A514: 409900A8  ble cr6, 0x8252a5bc
	if !ctx.cr[6].gt {
	pc = 0x8252A5BC; continue 'dispatch;
	}
	// 8252A518: 3B630024  addi r27, r3, 0x24
	ctx.r[27].s64 = ctx.r[3].s64 + 36;
	// 8252A51C: 3B430020  addi r26, r3, 0x20
	ctx.r[26].s64 = ctx.r[3].s64 + 32;
	// 8252A520: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8252A524: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8252A528: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A52C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8252A530: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A534: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8252A538: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252A53C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A540: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252A544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A548: 4E800421  bctrl
	ctx.lr = 0x8252A54C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A54C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A550: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8252A554: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252A558: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8252A55C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A560: 4E800421  bctrl
	ctx.lr = 0x8252A564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A564: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            0x8252A5BC => {
    //   block [0x8252A5BC..0x8252A5D0)
	// 8252A5BC: 672B3F00  oris r11, r25, 0x3f00
	ctx.r[11].u64 = ctx.r[25].u64 | 1056964608;
	// 8252A5C0: 9178000C  stw r11, 0xc(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8252A5C4: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 8252A5C8: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8252A5CC: 4800AB28  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252A5D0 size=416
    let mut pc: u32 = 0x8252A5D0;
    'dispatch: loop {
        match pc {
            0x8252A5D0 => {
    //   block [0x8252A5D0..0x8252A624)
	// 8252A5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A5D4: 4800AAD1  bl 0x825350a4
	ctx.lr = 0x8252A5D8;
	sub_82535080(ctx, base);
	// 8252A5D8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A5DC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A5E0: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8252A5E4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8252A5E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252A5EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8252A5F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8252A5F4: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8252A5F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252A5FC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252A600: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252A604: 40980020  bge cr6, 0x8252a624
	if !ctx.cr[6].lt {
	pc = 0x8252A624; continue 'dispatch;
	}
	// 8252A608: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252A60C: 39295BC4  addi r9, r9, 0x5bc4
	ctx.r[9].s64 = ctx.r[9].s64 + 23492;
	// 8252A610: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252A614: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252A618: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8252A61C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252A620: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252A624; continue 'dispatch;
            }
            0x8252A624 => {
    //   block [0x8252A624..0x8252A648)
	// 8252A624: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 8252A628: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 8252A62C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8252A630: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252A634: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8252A638: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252A63C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252A640: 409900C0  ble cr6, 0x8252a700
	if !ctx.cr[6].gt {
	pc = 0x8252A700; continue 'dispatch;
	}
	// 8252A644: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x8252A648; continue 'dispatch;
            }
            0x8252A648 => {
    //   block [0x8252A648..0x8252A668)
	// 8252A648: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A64C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252A650: 419A0048  beq cr6, 0x8252a698
	if ctx.cr[6].eq {
	pc = 0x8252A698; continue 'dispatch;
	}
	// 8252A654: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252A658: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8252A65C: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 8252A660: 409A0008  bne cr6, 0x8252a668
	if !ctx.cr[6].eq {
	pc = 0x8252A668; continue 'dispatch;
	}
	// 8252A664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	pc = 0x8252A668; continue 'dispatch;
            }
            0x8252A668 => {
    //   block [0x8252A668..0x8252A698)
	// 8252A668: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A66C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252A670: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A674: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8252A678: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8252A67C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A680: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A688: 4E800421  bctrl
	ctx.lr = 0x8252A68C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A68C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252A694: 419A0058  beq cr6, 0x8252a6ec
	if ctx.cr[6].eq {
	pc = 0x8252A6EC; continue 'dispatch;
	}
            }
            0x8252A698 => {
    //   block [0x8252A698..0x8252A6EC)
	// 8252A698: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252A69C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8252A6A0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A6A4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8252A6A8: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8252A6AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A6B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252A6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A6B8: 4E800421  bctrl
	ctx.lr = 0x8252A6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A6BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8252A6C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8252A6C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252A6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252A6CC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A6D0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252A6D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A6D8: 4E800421  bctrl
	ctx.lr = 0x8252A6DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A6DC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8252A6E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252A6E4: 419A0008  beq cr6, 0x8252a6ec
	if ctx.cr[6].eq {
	pc = 0x8252A6EC; continue 'dispatch;
	}
	// 8252A6E8: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
            }
            0x8252A6EC => {
    //   block [0x8252A6EC..0x8252A700)
	// 8252A6EC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252A6F0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8252A6F4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8252A6F8: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252A6FC: 4198FF4C  blt cr6, 0x8252a648
	if ctx.cr[6].lt {
	pc = 0x8252A648; continue 'dispatch;
	}
	pc = 0x8252A700; continue 'dispatch;
            }
            0x8252A700 => {
    //   block [0x8252A700..0x8252A720)
	// 8252A700: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 8252A704: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 8252A708: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252A70C: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8252A710: 419A0010  beq cr6, 0x8252a720
	if ctx.cr[6].eq {
	pc = 0x8252A720; continue 'dispatch;
	}
	// 8252A714: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8252A718: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252A71C: 7F4BE12E  stwx r26, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	pc = 0x8252A720; continue 'dispatch;
            }
            0x8252A720 => {
    //   block [0x8252A720..0x8252A750)
	// 8252A720: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8252A724: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252A728: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252A72C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252A730: 40980020  bge cr6, 0x8252a750
	if !ctx.cr[6].lt {
	pc = 0x8252A750; continue 'dispatch;
	}
	// 8252A734: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252A738: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8252A73C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252A740: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252A744: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252A748: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252A74C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252A750; continue 'dispatch;
            }
            0x8252A750 => {
    //   block [0x8252A750..0x8252A770)
	// 8252A750: 397A0001  addi r11, r26, 1
	ctx.r[11].s64 = ctx.r[26].s64 + 1;
	// 8252A754: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8252A758: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8252A75C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8252A760: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8252A764: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8252A768: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8252A76C: 4800A988  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252A770 size=344
    let mut pc: u32 = 0x8252A770;
    'dispatch: loop {
        match pc {
            0x8252A770 => {
    //   block [0x8252A770..0x8252A7C4)
	// 8252A770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A774: 4800A935  bl 0x825350a8
	ctx.lr = 0x8252A778;
	sub_82535080(ctx, base);
	// 8252A778: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A77C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A780: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8252A784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252A788: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8252A78C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8252A790: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8252A794: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8252A798: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252A79C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252A7A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252A7A4: 40980020  bge cr6, 0x8252a7c4
	if !ctx.cr[6].lt {
	pc = 0x8252A7C4; continue 'dispatch;
	}
	// 8252A7A8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252A7AC: 39295BC4  addi r9, r9, 0x5bc4
	ctx.r[9].s64 = ctx.r[9].s64 + 23492;
	// 8252A7B0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252A7B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252A7B8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8252A7BC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252A7C0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252A7C4; continue 'dispatch;
            }
            0x8252A7C4 => {
    //   block [0x8252A7C4..0x8252A7D8)
	// 8252A7C4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252A7C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8252A7CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252A7D0: 409900C0  ble cr6, 0x8252a890
	if !ctx.cr[6].gt {
	pc = 0x8252A890; continue 'dispatch;
	}
	// 8252A7D4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x8252A7D8; continue 'dispatch;
            }
            0x8252A7D8 => {
    //   block [0x8252A7D8..0x8252A7F8)
	// 8252A7D8: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A7DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252A7E0: 419A0048  beq cr6, 0x8252a828
	if ctx.cr[6].eq {
	pc = 0x8252A828; continue 'dispatch;
	}
	// 8252A7E4: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252A7E8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8252A7EC: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 8252A7F0: 409A0008  bne cr6, 0x8252a7f8
	if !ctx.cr[6].eq {
	pc = 0x8252A7F8; continue 'dispatch;
	}
	// 8252A7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	pc = 0x8252A7F8; continue 'dispatch;
            }
            0x8252A7F8 => {
    //   block [0x8252A7F8..0x8252A828)
	// 8252A7F8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A7FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252A800: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A804: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252A808: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8252A80C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A810: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A818: 4E800421  bctrl
	ctx.lr = 0x8252A81C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A81C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A820: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252A824: 419A0058  beq cr6, 0x8252a87c
	if ctx.cr[6].eq {
	pc = 0x8252A87C; continue 'dispatch;
	}
            }
            0x8252A828 => {
    //   block [0x8252A828..0x8252A87C)
	// 8252A828: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252A82C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8252A830: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252A834: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8252A838: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8252A83C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A840: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252A844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A848: 4E800421  bctrl
	ctx.lr = 0x8252A84C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252A84C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252A850: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 8252A854: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8252A858: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 8252A85C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8252A860: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8252A864: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252A868: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8252A86C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A870: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252A874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252A878: 4E800421  bctrl
	ctx.lr = 0x8252A87C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252A87C => {
    //   block [0x8252A87C..0x8252A890)
	// 8252A87C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252A880: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8252A884: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8252A888: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252A88C: 4198FF4C  blt cr6, 0x8252a7d8
	if ctx.cr[6].lt {
	pc = 0x8252A7D8; continue 'dispatch;
	}
	pc = 0x8252A890; continue 'dispatch;
            }
            0x8252A890 => {
    //   block [0x8252A890..0x8252A8C0)
	// 8252A890: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8252A894: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252A898: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252A89C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252A8A0: 40980020  bge cr6, 0x8252a8c0
	if !ctx.cr[6].lt {
	pc = 0x8252A8C0; continue 'dispatch;
	}
	// 8252A8A4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252A8A8: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8252A8AC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252A8B0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252A8B4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252A8B8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252A8BC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252A8C0; continue 'dispatch;
            }
            0x8252A8C0 => {
    //   block [0x8252A8C0..0x8252A8C8)
	// 8252A8C0: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 8252A8C4: 4800A834  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252A8C8 size=8
    let mut pc: u32 = 0x8252A8C8;
    'dispatch: loop {
        match pc {
            0x8252A8C8 => {
    //   block [0x8252A8C8..0x8252A8D0)
	// 8252A8C8: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 8252A8CC: 48000004  b 0x8252a8d0
	sub_8252A8D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252A8D0 size=124
    let mut pc: u32 = 0x8252A8D0;
    'dispatch: loop {
        match pc {
            0x8252A8D0 => {
    //   block [0x8252A8D0..0x8252A8F4)
	// 8252A8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252A8D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252A8DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A8E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252A8E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8252A8E8: 393F0014  addi r9, r31, 0x14
	ctx.r[9].s64 = ctx.r[31].s64 + 20;
	// 8252A8EC: 409A0008  bne cr6, 0x8252a8f4
	if !ctx.cr[6].eq {
	pc = 0x8252A8F4; continue 'dispatch;
	}
	// 8252A8F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x8252A8F4; continue 'dispatch;
            }
            0x8252A8F4 => {
    //   block [0x8252A8F4..0x8252A934)
	// 8252A8F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252A8F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8252A8FC: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8252A900: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 8252A904: 548807FE  clrlwi r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8252A908: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8252A90C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252A910: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252A914: 419A0020  beq cr6, 0x8252a934
	if ctx.cr[6].eq {
	pc = 0x8252A934; continue 'dispatch;
	}
	// 8252A918: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252A91C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252A920: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 8252A924: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252A928: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252A92C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252A930: 4BF39789  bl 0x824640b8
	ctx.lr = 0x8252A934;
	sub_824640B8(ctx, base);
	pc = 0x8252A934; continue 'dispatch;
            }
            0x8252A934 => {
    //   block [0x8252A934..0x8252A94C)
	// 8252A934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252A938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252A93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252A940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252A944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252A948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252A950 size=148
    let mut pc: u32 = 0x8252A950;
    'dispatch: loop {
        match pc {
            0x8252A950 => {
    //   block [0x8252A950..0x8252A9E4)
	// 8252A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252A958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252A95C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A960: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252A964: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252A968: 394A08F0  addi r10, r10, 0x8f0
	ctx.r[10].s64 = ctx.r[10].s64 + 2288;
	// 8252A96C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252A970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252A974: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252A9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252A9E8 size=152
    let mut pc: u32 = 0x8252A9E8;
    'dispatch: loop {
        match pc {
            0x8252A9E8 => {
    //   block [0x8252A9E8..0x8252AA80)
	// 8252A9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252A9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252A9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252A9F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252A9F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252A9FC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252AA00: 394A08F0  addi r10, r10, 0x8f0
	ctx.r[10].s64 = ctx.r[10].s64 + 2288;
	// 8252AA04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252AA08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252AA0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252AA80 size=84
    let mut pc: u32 = 0x8252AA80;
    'dispatch: loop {
        match pc {
            0x8252AA80 => {
    //   block [0x8252AA80..0x8252AAD4)
	// 8252AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252AA88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252AA8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252AA90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252AA94: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252AA98: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8252AA9C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8252AAA0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AAA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AAA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252AAAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252AAB0: 4E800421  bctrl
	ctx.lr = 0x8252AAB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252AAB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AAB8: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252AABC: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8252AAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252AAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252AAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252AACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252AAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252AAD8 size=4
    let mut pc: u32 = 0x8252AAD8;
    'dispatch: loop {
        match pc {
            0x8252AAD8 => {
    //   block [0x8252AAD8..0x8252AADC)
	// 8252AAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252AAE0 size=28
    let mut pc: u32 = 0x8252AAE0;
    'dispatch: loop {
        match pc {
            0x8252AAE0 => {
    //   block [0x8252AAE0..0x8252AAFC)
	// 8252AAE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252AAE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252AAE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AAEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252AAF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AAF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252AAF8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AAFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252AAFC size=4
    let mut pc: u32 = 0x8252AAFC;
    'dispatch: loop {
        match pc {
            0x8252AAFC => {
    //   block [0x8252AAFC..0x8252AB00)
	// 8252AAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252AB00 size=32
    let mut pc: u32 = 0x8252AB00;
    'dispatch: loop {
        match pc {
            0x8252AB00 => {
    //   block [0x8252AB00..0x8252AB20)
	// 8252AB00: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8252AB04: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252AB20 size=20
    let mut pc: u32 = 0x8252AB20;
    'dispatch: loop {
        match pc {
            0x8252AB20 => {
    //   block [0x8252AB20..0x8252AB34)
	// 8252AB20: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252AB24: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8252AB28: 409A000C  bne cr6, 0x8252ab34
	if !ctx.cr[6].eq {
		sub_8252AB34(ctx, base);
		return;
	}
	// 8252AB2C: D0430018  stfs f2, 0x18(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8252AB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252AB34 size=32
    let mut pc: u32 = 0x8252AB34;
    'dispatch: loop {
        match pc {
            0x8252AB34 => {
    //   block [0x8252AB34..0x8252AB54)
	// 8252AB34: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252AB38: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8252AB3C: C00A2074  lfs f0, 0x2074(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252AB40: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252AB58 size=240
    let mut pc: u32 = 0x8252AB58;
    'dispatch: loop {
        match pc {
            0x8252AB58 => {
    //   block [0x8252AB58..0x8252ABC4)
	// 8252AB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252AB5C: 4800A559  bl 0x825350b4
	ctx.lr = 0x8252AB60;
	sub_82535080(ctx, base);
	// 8252AB60: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252AB64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252AB68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252AB6C: 396B5BD8  addi r11, r11, 0x5bd8
	ctx.r[11].s64 = ctx.r[11].s64 + 23512;
	// 8252AB70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252AB74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252AB78: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8252AB7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252AB80: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8252AB84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252AB88: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252AB8C: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AB90: 83BC0000  lwz r29, 0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AB94: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AB98: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AB9C: 4807E5BD  bl 0x825a9158
	ctx.lr = 0x8252ABA0;
	sub_825A9158(ctx, base);
	// 8252ABA0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252ABA4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8252ABA8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252ABAC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8252ABB0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8252ABB4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8252ABB8: 409A000C  bne cr6, 0x8252abc4
	if !ctx.cr[6].eq {
	pc = 0x8252ABC4; continue 'dispatch;
	}
	// 8252ABBC: 4809A275  bl 0x825c4e30
	ctx.lr = 0x8252ABC0;
	sub_825C4E30(ctx, base);
	// 8252ABC0: 48000008  b 0x8252abc8
	pc = 0x8252ABC8; continue 'dispatch;
            }
            0x8252ABC4 => {
    //   block [0x8252ABC4..0x8252ABC8)
	// 8252ABC4: 4809A085  bl 0x825c4c48
	ctx.lr = 0x8252ABC8;
	sub_825C4C48(ctx, base);
	pc = 0x8252ABC8; continue 'dispatch;
            }
            0x8252ABC8 => {
    //   block [0x8252ABC8..0x8252AC48)
	// 8252ABC8: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 8252ABCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252AC48 size=164
    let mut pc: u32 = 0x8252AC48;
    'dispatch: loop {
        match pc {
            0x8252AC48 => {
    //   block [0x8252AC48..0x8252ACEC)
	// 8252AC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252AC4C: 4800A469  bl 0x825350b4
	ctx.lr = 0x8252AC50;
	sub_82535080(ctx, base);
	// 8252AC50: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252AC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252AC58: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AC5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8252AC60: 80A40008  lwz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AC64: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8252AC68: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8252AC6C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AC70: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AC74: 4807E4E5  bl 0x825a9158
	ctx.lr = 0x8252AC78;
	sub_825A9158(ctx, base);
	// 8252AC78: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8252AC7C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8252AC80: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8252AC84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252AC88: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8252AC8C: 4809CCA5  bl 0x825c7930
	ctx.lr = 0x8252AC90;
	sub_825C7930(ctx, base);
	// 8252AC90: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8252AC94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AC98: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8252AC9C: C1A1005C  lfs f13, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252ACF0 size=556
    let mut pc: u32 = 0x8252ACF0;
    'dispatch: loop {
        match pc {
            0x8252ACF0 => {
    //   block [0x8252ACF0..0x8252AD50)
	// 8252ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252ACF4: 4800A3AD  bl 0x825350a0
	ctx.lr = 0x8252ACF8;
	sub_82535080(ctx, base);
	// 8252ACF8: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252ACFC: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AD00: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 8252AD04: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8252AD08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8252AD0C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8252AD10: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8252AD14: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 8252AD18: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252AD1C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252AD20: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252AD24: 4098002C  bge cr6, 0x8252ad50
	if !ctx.cr[6].lt {
	pc = 0x8252AD50; continue 'dispatch;
	}
	// 8252AD28: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252AD2C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8252AD30: 39295C24  addi r9, r9, 0x5c24
	ctx.r[9].s64 = ctx.r[9].s64 + 23588;
	// 8252AD34: 39085C1C  addi r8, r8, 0x5c1c
	ctx.r[8].s64 = ctx.r[8].s64 + 23580;
	// 8252AD38: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252AD3C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8252AD40: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252AD44: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 8252AD48: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252AD4C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252AD50; continue 'dispatch;
            }
            0x8252AD50 => {
    //   block [0x8252AD50..0x8252AD80)
	// 8252AD50: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 8252AD54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252AD58: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252AD5C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252AD60: 40980020  bge cr6, 0x8252ad80
	if !ctx.cr[6].lt {
	pc = 0x8252AD80; continue 'dispatch;
	}
	// 8252AD64: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252AD68: 39295C10  addi r9, r9, 0x5c10
	ctx.r[9].s64 = ctx.r[9].s64 + 23568;
	// 8252AD6C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252AD70: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252AD74: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252AD78: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252AD7C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252AD80; continue 'dispatch;
            }
            0x8252AD80 => {
    //   block [0x8252AD80..0x8252AE48)
	// 8252AD80: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 8252AD84: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AD88: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AD8C: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AD90: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AD94: 4807E3C5  bl 0x825a9158
	ctx.lr = 0x8252AD98;
	sub_825A9158(ctx, base);
	// 8252AD98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8252AD9C: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 8252ADA0: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8252ADA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252ADA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252ADAC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8252ADB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252ADB4: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 8252ADB8: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252ADBC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8252ADC0: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 8252ADC4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8252ADC8: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252ADCC: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 8252ADD0: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8252ADD4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8252ADD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8252ADDC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252ADE0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252ADE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252ADE8: 4E800421  bctrl
	ctx.lr = 0x8252ADEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252ADEC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8252ADF0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252ADF4: 38C10140  addi r6, r1, 0x140
	ctx.r[6].s64 = ctx.r[1].s64 + 320;
	// 8252ADF8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252ADFC: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8252AE00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252AE04: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8252AE08: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252AE0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252AE10: 4E800421  bctrl
	ctx.lr = 0x8252AE14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252AE14: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8252AE18: 38C101E0  addi r6, r1, 0x1e0
	ctx.r[6].s64 = ctx.r[1].s64 + 480;
	// 8252AE1C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252AE20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252AE24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8252AE28: 4809CB09  bl 0x825c7930
	ctx.lr = 0x8252AE2C;
	sub_825C7930(ctx, base);
	// 8252AE2C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8252AE30: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8252AE34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252AE38: 419A0010  beq cr6, 0x8252ae48
	if ctx.cr[6].eq {
	pc = 0x8252AE48; continue 'dispatch;
	}
	// 8252AE3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252AE40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8252AE44: 4809B47D  bl 0x825c62c0
	ctx.lr = 0x8252AE48;
	sub_825C62C0(ctx, base);
            }
            0x8252AE48 => {
    //   block [0x8252AE48..0x8252AEB8)
	// 8252AE48: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8252AE4C: 409A006C  bne cr6, 0x8252aeb8
	if !ctx.cr[6].eq {
	pc = 0x8252AEB8; continue 'dispatch;
	}
	// 8252AE50: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8252AE54: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AE58: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252AE5C: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252AE60: C1A1005C  lfs f13, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252AE64: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252AE68: C1BD0010  lfs f13, 0x10(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	pc = 0x8252AEB8; continue 'dispatch;
            }
            0x8252AEB8 => {
    //   block [0x8252AEB8..0x8252AF1C)
	// 8252AEB8: 397B0020  addi r11, r27, 0x20
	ctx.r[11].s64 = ctx.r[27].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252AF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252AF20 size=420
    let mut pc: u32 = 0x8252AF20;
    'dispatch: loop {
        match pc {
            0x8252AF20 => {
    //   block [0x8252AF20..0x8252AF70)
	// 8252AF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252AF24: 4800A189  bl 0x825350ac
	ctx.lr = 0x8252AF28;
	sub_82535080(ctx, base);
	// 8252AF28: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252AF2C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AF30: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 8252AF34: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8252AF38: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8252AF3C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8252AF40: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8252AF44: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252AF48: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252AF4C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252AF50: 40980020  bge cr6, 0x8252af70
	if !ctx.cr[6].lt {
	pc = 0x8252AF70; continue 'dispatch;
	}
	// 8252AF54: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252AF58: 39295600  addi r9, r9, 0x5600
	ctx.r[9].s64 = ctx.r[9].s64 + 22016;
	// 8252AF5C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252AF60: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252AF64: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252AF68: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252AF6C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252AF70; continue 'dispatch;
            }
            0x8252AF70 => {
    //   block [0x8252AF70..0x8252AFAC)
	// 8252AF70: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8252AF74: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AF78: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AF7C: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AF80: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252AF84: 4807E1D5  bl 0x825a9158
	ctx.lr = 0x8252AF88;
	sub_825A9158(ctx, base);
	// 8252AF88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252AF8C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8252AF90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8252AF94: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8252AF98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252AF9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252AFA0: 409A000C  bne cr6, 0x8252afac
	if !ctx.cr[6].eq {
	pc = 0x8252AFAC; continue 'dispatch;
	}
	// 8252AFA4: 48099E8D  bl 0x825c4e30
	ctx.lr = 0x8252AFA8;
	sub_825C4E30(ctx, base);
	// 8252AFA8: 48000008  b 0x8252afb0
	pc = 0x8252AFB0; continue 'dispatch;
            }
            0x8252AFAC => {
    //   block [0x8252AFAC..0x8252AFB0)
	// 8252AFAC: 48099C9D  bl 0x825c4c48
	ctx.lr = 0x8252AFB0;
	sub_825C4C48(ctx, base);
	pc = 0x8252AFB0; continue 'dispatch;
            }
            0x8252AFB0 => {
    //   block [0x8252AFB0..0x8252B074)
	// 8252AFB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252AFB4: 8961005A  lbz r11, 0x5a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8252AFB8: 88A10058  lbz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8252AFBC: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 8252AFC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252AFC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252AFC8: 914100C0  stw r10, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 8252AFCC: 89410059  lbz r10, 0x59(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 8252AFD0: 90A100B0  stw r5, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 8252AFD4: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 8252AFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252AFDC: 914100C4  stw r10, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8252AFE0: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252AFE4: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8252AFE8: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 8252AFEC: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 8252AFF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252AFF4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252AFF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252AFFC: 4E800421  bctrl
	ctx.lr = 0x8252B000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B000: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B004: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 8252B008: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8252B00C: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8252B010: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 8252B014: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252B018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252B01C: 81290034  lwz r9, 0x34(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252B020: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252B024: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8252B028: 4E800421  bctrl
	ctx.lr = 0x8252B02C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B02C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8252B030: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8252B034: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8252B038: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252B03C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8252B040: 4809C8F1  bl 0x825c7930
	ctx.lr = 0x8252B044;
	sub_825C7930(ctx, base);
	// 8252B044: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8252B048: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252B04C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252B050: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252B054: 40980020  bge cr6, 0x8252b074
	if !ctx.cr[6].lt {
	pc = 0x8252B074; continue 'dispatch;
	}
	// 8252B058: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252B05C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8252B060: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252B064: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252B068: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8252B06C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252B070: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x8252B074 => {
    //   block [0x8252B074..0x8252B0A0)
	// 8252B074: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8252B078: 409A0028  bne cr6, 0x8252b0a0
	if !ctx.cr[6].eq {
	pc = 0x8252B0A0; continue 'dispatch;
	}
	// 8252B07C: C1A1006C  lfs f13, 0x6c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252B080: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252B084: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252B088: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252B08C: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252B090: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252B094: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252B098: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8252B09C: 40980020  bge cr6, 0x8252b0bc
	if !ctx.cr[6].lt {
	pc = 0x8252B0BC; continue 'dispatch;
	}
	pc = 0x8252B0A0; continue 'dispatch;
            }
            0x8252B0A0 => {
    //   block [0x8252B0A0..0x8252B0BC)
	// 8252B0A0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B0A4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8252B0A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8252B0AC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8252B0B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252B0B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252B0B8: 4E800421  bctrl
	ctx.lr = 0x8252B0BC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252B0BC => {
    //   block [0x8252B0BC..0x8252B0C4)
	// 8252B0BC: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 8252B0C0: 4800A03C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252B0C8 size=740
    let mut pc: u32 = 0x8252B0C8;
    'dispatch: loop {
        match pc {
            0x8252B0C8 => {
    //   block [0x8252B0C8..0x8252B11C)
	// 8252B0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B0CC: 48009FD9  bl 0x825350a4
	ctx.lr = 0x8252B0D0;
	sub_82535080(ctx, base);
	// 8252B0D0: 9421FCF0  stwu r1, -0x310(r1)
	ea = ctx.r[1].u32.wrapping_add(-784 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B0D4: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B0D8: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8252B0DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8252B0E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8252B0E4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8252B0E8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8252B0EC: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8252B0F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252B0F4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252B0F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252B0FC: 40980020  bge cr6, 0x8252b11c
	if !ctx.cr[6].lt {
	pc = 0x8252B11C; continue 'dispatch;
	}
	// 8252B100: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252B104: 39295600  addi r9, r9, 0x5600
	ctx.r[9].s64 = ctx.r[9].s64 + 22016;
	// 8252B108: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252B10C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252B110: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8252B114: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252B118: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252B11C; continue 'dispatch;
            }
            0x8252B11C => {
    //   block [0x8252B11C..0x8252B158)
	// 8252B11C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8252B120: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B124: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B128: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B12C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B130: 4807E029  bl 0x825a9158
	ctx.lr = 0x8252B134;
	sub_825A9158(ctx, base);
	// 8252B134: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252B138: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 8252B13C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8252B140: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8252B144: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252B148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252B14C: 409A000C  bne cr6, 0x8252b158
	if !ctx.cr[6].eq {
	pc = 0x8252B158; continue 'dispatch;
	}
	// 8252B150: 48099CE1  bl 0x825c4e30
	ctx.lr = 0x8252B154;
	sub_825C4E30(ctx, base);
	// 8252B154: 48000008  b 0x8252b15c
	pc = 0x8252B15C; continue 'dispatch;
            }
            0x8252B158 => {
    //   block [0x8252B158..0x8252B15C)
	// 8252B158: 48099AF1  bl 0x825c4c48
	ctx.lr = 0x8252B15C;
	sub_825C4C48(ctx, base);
	pc = 0x8252B15C; continue 'dispatch;
            }
            0x8252B15C => {
    //   block [0x8252B15C..0x8252B2E4)
	// 8252B15C: 89410059  lbz r10, 0x59(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 8252B160: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8252B164: 8961005A  lbz r11, 0x5a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8252B168: 38C10160  addi r6, r1, 0x160
	ctx.r[6].s64 = ctx.r[1].s64 + 352;
	// 8252B16C: 88A10058  lbz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8252B170: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252B174: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B178: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B17C: 91410144  stw r10, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 8252B180: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252B184: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8252B188: 93610150  stw r27, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[27].u32 ) };
	// 8252B18C: 90A10140  stw r5, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[5].u32 ) };
	// 8252B190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252B194: 93610154  stw r27, 0x154(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), ctx.r[27].u32 ) };
	// 8252B198: 91410148  stw r10, 0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[10].u32 ) };
	// 8252B19C: 9161014C  stw r11, 0x14c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[11].u32 ) };
	// 8252B1A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B1A4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252B1A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252B1AC: 4E800421  bctrl
	ctx.lr = 0x8252B1B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B1B0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B1B4: 81610140  lwz r11, 0x140(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 8252B1B8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8252B1BC: 80A10144  lwz r5, 0x144(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 8252B1C0: 38C10220  addi r6, r1, 0x220
	ctx.r[6].s64 = ctx.r[1].s64 + 544;
	// 8252B1C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252B1C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252B1CC: 81290034  lwz r9, 0x34(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252B1D0: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252B1D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8252B1D8: 4E800421  bctrl
	ctx.lr = 0x8252B1DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B1DC: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 8252B1E0: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 8252B1E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8252B1E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252B1EC: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8252B1F0: 4809C741  bl 0x825c7930
	ctx.lr = 0x8252B1F4;
	sub_825C7930(ctx, base);
	// 8252B1F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8252B1F8: 409A00EC  bne cr6, 0x8252b2e4
	if !ctx.cr[6].eq {
	pc = 0x8252B2E4; continue 'dispatch;
	}
	// 8252B1FC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8252B200: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 8252B204: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8252B208: 4809B209  bl 0x825c6410
	ctx.lr = 0x8252B20C;
	sub_825C6410(ctx, base);
	// 8252B20C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8252B210: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8252B214: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B218: 48080989  bl 0x825abba0
	ctx.lr = 0x8252B21C;
	sub_825ABBA0(ctx, base);
	// 8252B21C: C1A10090  lfs f13, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252B220: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252B224: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252B228: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252B22C: C19A0004  lfs f12, 4(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252B230: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252B234: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8252B238: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8252B23C: 409800A8  bge cr6, 0x8252b2e4
	if !ctx.cr[6].lt {
	pc = 0x8252B2E4; continue 'dispatch;
	}
	// 8252B240: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8252B244: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252B248: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8252B24C: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252B250: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8252B254: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252B258: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8252B25C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
            }
            0x8252B2E4 => {
    //   block [0x8252B2E4..0x8252B3AC)
	// 8252B2E4: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252B3B0 size=512
    let mut pc: u32 = 0x8252B3B0;
    'dispatch: loop {
        match pc {
            0x8252B3B0 => {
    //   block [0x8252B3B0..0x8252B490)
	// 8252B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B3B4: 48009CF5  bl 0x825350a8
	ctx.lr = 0x8252B3B8;
	sub_82535080(ctx, base);
	// 8252B3B8: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B3BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8252B3C0: 83860000  lwz r28, 0(r6)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B3C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8252B3C8: 80A60008  lwz r5, 8(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B3CC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8252B3D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252B3D4: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8252B3D8: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B3DC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8252B3E0: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B3E4: 4807DD75  bl 0x825a9158
	ctx.lr = 0x8252B3E8;
	sub_825A9158(ctx, base);
	// 8252B3E8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8252B3EC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8252B3F0: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 8252B3F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252B3F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252B3FC: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 8252B400: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 8252B404: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B408: 932100C0  stw r25, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[25].u32 ) };
	// 8252B40C: 932100C4  stw r25, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[25].u32 ) };
	// 8252B410: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 8252B414: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252B418: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8252B41C: 90A100B0  stw r5, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 8252B420: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 8252B424: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 8252B428: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B42C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252B430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252B434: 4E800421  bctrl
	ctx.lr = 0x8252B438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B438: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 8252B43C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B440: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 8252B444: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252B448: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8252B44C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8252B450: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8252B454: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252B458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252B45C: 4E800421  bctrl
	ctx.lr = 0x8252B460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B460: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8252B464: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8252B468: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8252B46C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252B470: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8252B474: 4809C4BD  bl 0x825c7930
	ctx.lr = 0x8252B478;
	sub_825C7930(ctx, base);
	// 8252B478: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 8252B47C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252B480: 419A0010  beq cr6, 0x8252b490
	if ctx.cr[6].eq {
	pc = 0x8252B490; continue 'dispatch;
	}
	// 8252B484: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252B488: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8252B48C: 4809AE35  bl 0x825c62c0
	ctx.lr = 0x8252B490;
	sub_825C62C0(ctx, base);
            }
            0x8252B490 => {
    //   block [0x8252B490..0x8252B5B0)
	// 8252B490: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252B494: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8252B498: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8252B49C: 4809AF75  bl 0x825c6410
	ctx.lr = 0x8252B4A0;
	sub_825C6410(ctx, base);
	// 8252B4A0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252B5B0 size=292
    let mut pc: u32 = 0x8252B5B0;
    'dispatch: loop {
        match pc {
            0x8252B5B0 => {
    //   block [0x8252B5B0..0x8252B608)
	// 8252B5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B5B4: 48009AF9  bl 0x825350ac
	ctx.lr = 0x8252B5B8;
	sub_82535080(ctx, base);
	// 8252B5B8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B5BC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B5C0: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 8252B5C4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8252B5C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252B5CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8252B5D0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8252B5D4: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8252B5D8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8252B5DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252B5E0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252B5E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252B5E8: 40980020  bge cr6, 0x8252b608
	if !ctx.cr[6].lt {
	pc = 0x8252B608; continue 'dispatch;
	}
	// 8252B5EC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252B5F0: 39295600  addi r9, r9, 0x5600
	ctx.r[9].s64 = ctx.r[9].s64 + 22016;
	// 8252B5F4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252B5F8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252B5FC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8252B600: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252B604: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252B608; continue 'dispatch;
            }
            0x8252B608 => {
    //   block [0x8252B608..0x8252B69C)
	// 8252B608: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8252B60C: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B610: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B614: 4807DB45  bl 0x825a9158
	ctx.lr = 0x8252B618;
	sub_825A9158(ctx, base);
	// 8252B618: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B61C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8252B620: C01A0004  lfs f0, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252B624: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8252B628: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8252B62C: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 8252B630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252B634: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8252B638: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B63C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8252B640: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B644: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8252B648: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 8252B64C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8252B650: 4809D169  bl 0x825c87b8
	ctx.lr = 0x8252B654;
	sub_825C87B8(ctx, base);
	// 8252B654: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8252B658: 409A0044  bne cr6, 0x8252b69c
	if !ctx.cr[6].eq {
	pc = 0x8252B69C; continue 'dispatch;
	}
	// 8252B65C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8252B660: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 8252B664: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B668: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8252B66C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	pc = 0x8252B69C; continue 'dispatch;
            }
            0x8252B69C => {
    //   block [0x8252B69C..0x8252B6D4)
	// 8252B69C: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8252B6A0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252B6A4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252B6A8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252B6AC: 40980020  bge cr6, 0x8252b6cc
	if !ctx.cr[6].lt {
	pc = 0x8252B6CC; continue 'dispatch;
	}
	// 8252B6B0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252B6B4: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8252B6B8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252B6BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252B6C0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252B6C4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252B6C8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8252B6CC: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 8252B6D0: 48009A2C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252B6D8 size=80
    let mut pc: u32 = 0x8252B6D8;
    'dispatch: loop {
        match pc {
            0x8252B6D8 => {
    //   block [0x8252B6D8..0x8252B728)
	// 8252B6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252B6E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252B6E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B6E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252B6EC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8252B6F0: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8252B6F4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B6F8: 48097321  bl 0x825c2a18
	ctx.lr = 0x8252B6FC;
	sub_825C2A18(ctx, base);
	// 8252B6FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B700: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252B704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252B708: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B70C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252B710: 4E800421  bctrl
	ctx.lr = 0x8252B714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252B714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252B718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252B71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252B720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252B724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B728 size=56
    let mut pc: u32 = 0x8252B728;
    'dispatch: loop {
        match pc {
            0x8252B728 => {
    //   block [0x8252B728..0x8252B740)
	// 8252B728: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 8252B72C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252B730: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8252B734: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8252B738: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8252B73C: 39630036  addi r11, r3, 0x36
	ctx.r[11].s64 = ctx.r[3].s64 + 54;
	pc = 0x8252B740; continue 'dispatch;
            }
            0x8252B740 => {
    //   block [0x8252B740..0x8252B760)
	// 8252B740: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B744: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8252B748: 419A0018  beq cr6, 0x8252b760
	if ctx.cr[6].eq {
		sub_8252B760(ctx, base);
		return;
	}
	// 8252B74C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8252B750: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8252B754: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252B758: 4198FFE8  blt cr6, 0x8252b740
	if ctx.cr[6].lt {
	pc = 0x8252B740; continue 'dispatch;
	}
	// 8252B75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B760 size=12
    let mut pc: u32 = 0x8252B760;
    'dispatch: loop {
        match pc {
            0x8252B760 => {
    //   block [0x8252B760..0x8252B76C)
	// 8252B760: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8252B764: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 8252B768: 48098200  b 0x825c3968
	sub_825C3968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B770 size=52
    let mut pc: u32 = 0x8252B770;
    'dispatch: loop {
        match pc {
            0x8252B770 => {
    //   block [0x8252B770..0x8252B784)
	// 8252B770: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 8252B774: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252B778: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8252B77C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8252B780: 39430036  addi r10, r3, 0x36
	ctx.r[10].s64 = ctx.r[3].s64 + 54;
	pc = 0x8252B784; continue 'dispatch;
            }
            0x8252B784 => {
    //   block [0x8252B784..0x8252B7A4)
	// 8252B784: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B788: 2B08FFFF  cmplwi cr6, r8, 0xffff
	ctx.cr[6].compare_u32(ctx.r[8].u32, 65535 as u32, &mut ctx.xer);
	// 8252B78C: 419A0018  beq cr6, 0x8252b7a4
	if ctx.cr[6].eq {
		sub_8252B7A4(ctx, base);
		return;
	}
	// 8252B790: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252B794: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8252B798: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252B79C: 4198FFE8  blt cr6, 0x8252b784
	if ctx.cr[6].lt {
	pc = 0x8252B784; continue 'dispatch;
	}
	// 8252B7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B7A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B7A4 size=16
    let mut pc: u32 = 0x8252B7A4;
    'dispatch: loop {
        match pc {
            0x8252B7A4 => {
    //   block [0x8252B7A4..0x8252B7B4)
	// 8252B7A4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252B7A8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8252B7AC: B08B0036  sth r4, 0x36(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(54 as u32), ctx.r[4].u16 ) };
	// 8252B7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B7B8 size=60
    let mut pc: u32 = 0x8252B7B8;
    'dispatch: loop {
        match pc {
            0x8252B7B8 => {
    //   block [0x8252B7B8..0x8252B7D4)
	// 8252B7B8: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 8252B7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8252B7C0: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8252B7C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8252B7C8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8252B7CC: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8252B7D0: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	pc = 0x8252B7D4; continue 'dispatch;
            }
            0x8252B7D4 => {
    //   block [0x8252B7D4..0x8252B7F4)
	// 8252B7D4: A0CB0002  lhz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8252B7D8: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8252B7DC: 419A0018  beq cr6, 0x8252b7f4
	if ctx.cr[6].eq {
		sub_8252B7F4(ctx, base);
		return;
	}
	// 8252B7E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8252B7E4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8252B7E8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252B7EC: 4198FFE8  blt cr6, 0x8252b7d4
	if ctx.cr[6].lt {
	pc = 0x8252B7D4; continue 'dispatch;
	}
	// 8252B7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B7F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B7F4 size=12
    let mut pc: u32 = 0x8252B7F4;
    'dispatch: loop {
        match pc {
            0x8252B7F4 => {
    //   block [0x8252B7F4..0x8252B800)
	// 8252B7F4: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 8252B7F8: 98EB0001  stb r7, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 8252B7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252B800 size=68
    let mut pc: u32 = 0x8252B800;
    'dispatch: loop {
        match pc {
            0x8252B800 => {
    //   block [0x8252B800..0x8252B844)
	// 8252B800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252B808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252B80C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B810: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252B814: 4BFFF345  bl 0x8252ab58
	ctx.lr = 0x8252B818;
	sub_8252AB58(ctx, base);
	// 8252B818: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252B81C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252B820: 396B5C44  addi r11, r11, 0x5c44
	ctx.r[11].s64 = ctx.r[11].s64 + 23620;
	// 8252B824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252B828: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252B82C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8252B830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252B834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252B838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252B83C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252B840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252B848 size=156
    let mut pc: u32 = 0x8252B848;
    'dispatch: loop {
        match pc {
            0x8252B848 => {
    //   block [0x8252B848..0x8252B8BC)
	// 8252B848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B84C: 4800986D  bl 0x825350b8
	ctx.lr = 0x8252B850;
	sub_82535080(ctx, base);
	// 8252B850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B854: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252B858: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252B85C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8252B860: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8252B864: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8252B868: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8252B86C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252B870: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8252B874: 419A0048  beq cr6, 0x8252b8bc
	if ctx.cr[6].eq {
	pc = 0x8252B8BC; continue 'dispatch;
	}
	// 8252B878: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 8252B87C: 4BF387BD  bl 0x82464038
	ctx.lr = 0x8252B880;
	sub_82464038(ctx, base);
	// 8252B880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252B884: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 8252B888: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8252B88C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252B890: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252B894: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8252B898: 4BFFF2C1  bl 0x8252ab58
	ctx.lr = 0x8252B89C;
	sub_8252AB58(ctx, base);
	// 8252B89C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252B8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252B8A4: 396B5C44  addi r11, r11, 0x5c44
	ctx.r[11].s64 = ctx.r[11].s64 + 23620;
	// 8252B8A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252B8AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252B8B0: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8252B8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252B8B8: 48009850  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8252B8BC => {
    //   block [0x8252B8BC..0x8252B8E4)
	// 8252B8BC: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8252B8C0: 4BF38779  bl 0x82464038
	ctx.lr = 0x8252B8C4;
	sub_82464038(ctx, base);
	// 8252B8C4: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8252B8C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8252B8CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252B8D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252B8D4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8252B8D8: 4BFFF281  bl 0x8252ab58
	ctx.lr = 0x8252B8DC;
	sub_8252AB58(ctx, base);
	// 8252B8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252B8E0: 48009828  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252B8E8 size=64
    let mut pc: u32 = 0x8252B8E8;
    'dispatch: loop {
        match pc {
            0x8252B8E8 => {
    //   block [0x8252B8E8..0x8252B928)
	// 8252B8E8: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8252B8EC: 3D008253  lis r8, -0x7dad
	ctx.r[8].s64 = -2108489728;
	// 8252B8F0: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 8252B8F4: 3D408253  lis r10, -0x7dad
	ctx.r[10].s64 = -2108489728;
	// 8252B8F8: 38EBBEF8  addi r7, r11, -0x4108
	ctx.r[7].s64 = ctx.r[11].s64 + -16648;
	// 8252B8FC: 3908B848  addi r8, r8, -0x47b8
	ctx.r[8].s64 = ctx.r[8].s64 + -18360;
	// 8252B900: 3929AF20  addi r9, r9, -0x50e0
	ctx.r[9].s64 = ctx.r[9].s64 + -20704;
	// 8252B904: 394AB0C8  addi r10, r10, -0x4f38
	ctx.r[10].s64 = ctx.r[10].s64 + -20280;
	// 8252B908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252B90C: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8252B910: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8252B914: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252B918: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8252B91C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8252B920: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8252B924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252B928 size=104
    let mut pc: u32 = 0x8252B928;
    'dispatch: loop {
        match pc {
            0x8252B928 => {
    //   block [0x8252B928..0x8252B990)
	// 8252B928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252B930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B934: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8252B938: 3D008253  lis r8, -0x7dad
	ctx.r[8].s64 = -2108489728;
	// 8252B93C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8252B940: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 8252B944: 3D408253  lis r10, -0x7dad
	ctx.r[10].s64 = -2108489728;
	// 8252B948: 3908B848  addi r8, r8, -0x47b8
	ctx.r[8].s64 = ctx.r[8].s64 + -18360;
	// 8252B94C: 3929AF20  addi r9, r9, -0x50e0
	ctx.r[9].s64 = ctx.r[9].s64 + -20704;
	// 8252B950: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8252B954: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252B958: 394AB0C8  addi r10, r10, -0x4f38
	ctx.r[10].s64 = ctx.r[10].s64 + -20280;
	// 8252B95C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8252B960: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252B964: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8252B968: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252B96C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8252B970: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8252B974: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252B978: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 8252B97C: 4BFD440D  bl 0x824ffd88
	ctx.lr = 0x8252B980;
	sub_824FFD88(ctx, base);
	// 8252B980: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252B984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252B988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252B98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252B990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252B990 size=124
    let mut pc: u32 = 0x8252B990;
    'dispatch: loop {
        match pc {
            0x8252B990 => {
    //   block [0x8252B990..0x8252BA0C)
	// 8252B990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252B994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252B998: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252B99C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252B9A0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252B9A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252B9A8: 90810070  stw r4, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 8252B9AC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8252B9B0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B9B4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8252B9B8: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 8252B9BC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8252B9C0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B9C4: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252B9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8252B9CC: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8252B9D0: 4807D789  bl 0x825a9158
	ctx.lr = 0x8252B9D4;
	sub_825A9158(ctx, base);
	// 8252B9D4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8252B9D8: 391F0020  addi r8, r31, 0x20
	ctx.r[8].s64 = ctx.r[31].s64 + 32;
	// 8252B9DC: 38FF0030  addi r7, r31, 0x30
	ctx.r[7].s64 = ctx.r[31].s64 + 48;
	// 8252B9E0: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 8252B9E4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8252B9E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252B9EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8252B9F0: 480A51D9  bl 0x825d0bc8
	ctx.lr = 0x8252B9F4;
	sub_825D0BC8(ctx, base);
	// 8252B9F4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 8252B9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252B9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252BA00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252BA04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252BA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252BA10 size=456
    let mut pc: u32 = 0x8252BA10;
    'dispatch: loop {
        match pc {
            0x8252BA10 => {
    //   block [0x8252BA10..0x8252BA64)
	// 8252BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252BA14: 48009699  bl 0x825350ac
	ctx.lr = 0x8252BA18;
	sub_82535080(ctx, base);
	// 8252BA18: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252BA1C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252BA20: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8252BA24: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252BA28: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252BA2C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252BA30: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252BA34: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252BA38: 4098002C  bge cr6, 0x8252ba64
	if !ctx.cr[6].lt {
	pc = 0x8252BA64; continue 'dispatch;
	}
	// 8252BA3C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252BA40: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8252BA44: 39295C7C  addi r9, r9, 0x5c7c
	ctx.r[9].s64 = ctx.r[9].s64 + 23676;
	// 8252BA48: 39085064  addi r8, r8, 0x5064
	ctx.r[8].s64 = ctx.r[8].s64 + 20580;
	// 8252BA4C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252BA50: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8252BA54: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252BA58: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 8252BA5C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252BA60: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252BA64; continue 'dispatch;
            }
            0x8252BA64 => {
    //   block [0x8252BA64..0x8252BB64)
	// 8252BA64: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252BA68: 3B630018  addi r27, r3, 0x18
	ctx.r[27].s64 = ctx.r[3].s64 + 24;
	// 8252BA6C: C1A60050  lfs f13, 0x50(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252BA70: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252BA74: 419A00F0  beq cr6, 0x8252bb64
	if ctx.cr[6].eq {
	pc = 0x8252BB64; continue 'dispatch;
	}
	// 8252BA78: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252BA7C: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252BA80: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252BA84: C1860004  lfs f12, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252BA88: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
	// 8252BA8C: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 8252BA90: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 8252BA94: C0060058  lfs f0, 0x58(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252BA98: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 8252BA9C: C1AB00A0  lfs f13, 0xa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252BAA0: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 8252BAA4: C18B009C  lfs f12, 0x9c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252BAA8: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 8252BAAC: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	pc = 0x8252BB64; continue 'dispatch;
            }
            0x8252BB64 => {
    //   block [0x8252BB64..0x8252BBD8)
	// 8252BB64: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252BB68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252BB6C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252BB70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252BB74: 40980020  bge cr6, 0x8252bb94
	if !ctx.cr[6].lt {
	pc = 0x8252BB94; continue 'dispatch;
	}
	// 8252BB78: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252BB7C: 39295050  addi r9, r9, 0x5050
	ctx.r[9].s64 = ctx.r[9].s64 + 20560;
	// 8252BB80: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252BB84: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252BB88: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252BB8C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252BB90: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8252BB94: C0060054  lfs f0, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252BB98: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252BB9C: 4BFFFDF5  bl 0x8252b990
	ctx.lr = 0x8252BBA0;
	sub_8252B990(ctx, base);
	// 8252BBA0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252BBA4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252BBA8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252BBAC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252BBB0: 40980020  bge cr6, 0x8252bbd0
	if !ctx.cr[6].lt {
	pc = 0x8252BBD0; continue 'dispatch;
	}
	// 8252BBB4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252BBB8: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 8252BBBC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252BBC0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252BBC4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252BBC8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252BBCC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8252BBD0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8252BBD4: 48009528  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252BBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252BBD8 size=796
    let mut pc: u32 = 0x8252BBD8;
    'dispatch: loop {
        match pc {
            0x8252BBD8 => {
    //   block [0x8252BBD8..0x8252BEF4)
	// 8252BBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252BBDC: 480094C5  bl 0x825350a0
	ctx.lr = 0x8252BBE0;
	sub_82535080(ctx, base);
	// 8252BBE0: DBC1FF98  stfd f30, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 8252BBE4: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 8252BBE8: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252BEF8 size=828
    let mut pc: u32 = 0x8252BEF8;
    'dispatch: loop {
        match pc {
            0x8252BEF8 => {
    //   block [0x8252BEF8..0x8252C234)
	// 8252BEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252BEFC: 480091A5  bl 0x825350a0
	ctx.lr = 0x8252BF00;
	sub_82535080(ctx, base);
	// 8252BF00: DBA1FF90  stfd f29, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[29].u64 ) };
	// 8252BF04: DBC1FF98  stfd f30, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 8252BF08: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 8252BF0C: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C238 size=12
    let mut pc: u32 = 0x8252C238;
    'dispatch: loop {
        match pc {
            0x8252C238 => {
    //   block [0x8252C238..0x8252C244)
	// 8252C238: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8252C23C: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8252C240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252C248 size=72
    let mut pc: u32 = 0x8252C248;
    'dispatch: loop {
        match pc {
            0x8252C248 => {
    //   block [0x8252C248..0x8252C264)
	// 8252C248: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252C24C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C250: 419A0014  beq cr6, 0x8252c264
	if ctx.cr[6].eq {
	pc = 0x8252C264; continue 'dispatch;
	}
	// 8252C254: C004001C  lfs f0, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252C258: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252C25C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252C260: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
	pc = 0x8252C264; continue 'dispatch;
            }
            0x8252C264 => {
    //   block [0x8252C264..0x8252C290)
	// 8252C264: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8252C268: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 8252C26C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252C270: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C290 size=64
    let mut pc: u32 = 0x8252C290;
    'dispatch: loop {
        match pc {
            0x8252C290 => {
    //   block [0x8252C290..0x8252C2D0)
	// 8252C290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252C294: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252C298: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8252C29C: 99430022  stb r10, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[10].u8 ) };
	// 8252C2A0: 89430021  lbz r10, 0x21(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 8252C2A4: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 8252C2A8: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8252C2AC: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8252C2B0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C2B4: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8252C2B8: A14A0002  lhz r10, 2(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 8252C2BC: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 8252C2C0: 89630021  lbz r11, 0x21(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 8252C2C4: 396B00FF  addi r11, r11, 0xff
	ctx.r[11].s64 = ctx.r[11].s64 + 255;
	// 8252C2C8: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8252C2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C2D0 size=52
    let mut pc: u32 = 0x8252C2D0;
    'dispatch: loop {
        match pc {
            0x8252C2D0 => {
    //   block [0x8252C2D0..0x8252C304)
	// 8252C2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252C2D4: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 8252C2D8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252C2DC: B163000A  sth r11, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 8252C2E0: B163000E  sth r11, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 8252C2E4: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 8252C2E8: B1630016  sth r11, 0x16(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u16 ) };
	// 8252C2EC: B163001A  sth r11, 0x1a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 8252C2F0: B163001E  sth r11, 0x1e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u16 ) };
	// 8252C2F4: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8252C2F8: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8252C2FC: 99630022  stb r11, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[11].u8 ) };
	// 8252C300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C308 size=128
    let mut pc: u32 = 0x8252C308;
    'dispatch: loop {
        match pc {
            0x8252C308 => {
    //   block [0x8252C308..0x8252C37C)
	// 8252C308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C30C: 48008DB1  bl 0x825350bc
	ctx.lr = 0x8252C310;
	sub_82535080(ctx, base);
	// 8252C310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C318: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8252C31C: 8BBF0021  lbz r29, 0x21(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 8252C320: 2B1D0008  cmplwi cr6, r29, 8
	ctx.cr[6].compare_u32(ctx.r[29].u32, 8 as u32, &mut ctx.xer);
	// 8252C324: 41990058  bgt cr6, 0x8252c37c
	if ctx.cr[6].gt {
	pc = 0x8252C37C; continue 'dispatch;
	}
	// 8252C328: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8252C32C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252C330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252C334: 48000055  bl 0x8252c388
	ctx.lr = 0x8252C338;
	sub_8252C388(ctx, base);
	// 8252C338: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C33C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C340: 409A003C  bne cr6, 0x8252c37c
	if !ctx.cr[6].eq {
	pc = 0x8252C37C; continue 'dispatch;
	}
	// 8252C344: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 8252C348: 40980034  bge cr6, 0x8252c37c
	if !ctx.cr[6].lt {
	pc = 0x8252C37C; continue 'dispatch;
	}
	// 8252C34C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252C350: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C354: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252C358: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8252C35C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8252C360: A15E0002  lhz r10, 2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 8252C364: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 8252C368: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 8252C36C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252C370: 997F0021  stb r11, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8252C374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252C378: 48008D94  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8252C37C => {
    //   block [0x8252C37C..0x8252C388)
	// 8252C37C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8252C380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252C384: 48008D88  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C388 size=84
    let mut pc: u32 = 0x8252C388;
    'dispatch: loop {
        match pc {
            0x8252C388 => {
    //   block [0x8252C388..0x8252C3A4)
	// 8252C388: 89640021  lbz r11, 0x21(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(33 as u32) ) } as u64;
	// 8252C38C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 8252C390: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C394: 4198003C  blt cr6, 0x8252c3d0
	if ctx.cr[6].lt {
	pc = 0x8252C3D0; continue 'dispatch;
	}
	// 8252C398: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252C39C: 89250000  lbz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C3A0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	pc = 0x8252C3A4; continue 'dispatch;
            }
            0x8252C3A4 => {
    //   block [0x8252C3A4..0x8252C3C0)
	// 8252C3A4: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C3A8: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252C3AC: 409A0014  bne cr6, 0x8252c3c0
	if !ctx.cr[6].eq {
	pc = 0x8252C3C0; continue 'dispatch;
	}
	// 8252C3B0: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8252C3B4: 88E50001  lbz r7, 1(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 8252C3B8: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8252C3BC: 419A0020  beq cr6, 0x8252c3dc
	if ctx.cr[6].eq {
		sub_8252C3DC(ctx, base);
		return;
	}
	pc = 0x8252C3C0; continue 'dispatch;
            }
            0x8252C3C0 => {
    //   block [0x8252C3C0..0x8252C3D0)
	// 8252C3C0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8252C3C4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8252C3C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C3CC: 4098FFD8  bge cr6, 0x8252c3a4
	if !ctx.cr[6].lt {
	pc = 0x8252C3A4; continue 'dispatch;
	}
	pc = 0x8252C3D0; continue 'dispatch;
            }
            0x8252C3D0 => {
    //   block [0x8252C3D0..0x8252C3DC)
	// 8252C3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252C3D4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8252C3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C3DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C3DC size=12
    let mut pc: u32 = 0x8252C3DC;
    'dispatch: loop {
        match pc {
            0x8252C3DC => {
    //   block [0x8252C3DC..0x8252C3E8)
	// 8252C3DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8252C3E0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8252C3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C3E8 size=4
    let mut pc: u32 = 0x8252C3E8;
    'dispatch: loop {
        match pc {
            0x8252C3E8 => {
    //   block [0x8252C3E8..0x8252C3EC)
	// 8252C3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C3F0 size=20
    let mut pc: u32 = 0x8252C3F0;
    'dispatch: loop {
        match pc {
            0x8252C3F0 => {
    //   block [0x8252C3F0..0x8252C404)
	// 8252C3F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C3F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C3F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C3FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C400: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C408 size=32
    let mut pc: u32 = 0x8252C408;
    'dispatch: loop {
        match pc {
            0x8252C408 => {
    //   block [0x8252C408..0x8252C428)
	// 8252C408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252C40C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C410: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C414: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252C418: 396B5F94  addi r11, r11, 0x5f94
	ctx.r[11].s64 = ctx.r[11].s64 + 24468;
	// 8252C41C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252C420: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C428 size=12
    let mut pc: u32 = 0x8252C428;
    'dispatch: loop {
        match pc {
            0x8252C428 => {
    //   block [0x8252C428..0x8252C434)
	// 8252C428: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C42C: 386B5F94  addi r3, r11, 0x5f94
	ctx.r[3].s64 = ctx.r[11].s64 + 24468;
	// 8252C430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C438 size=152
    let mut pc: u32 = 0x8252C438;
    'dispatch: loop {
        match pc {
            0x8252C438 => {
    //   block [0x8252C438..0x8252C478)
	// 8252C438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C44C: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8252C450: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252C454: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C458: 409A0020  bne cr6, 0x8252c478
	if !ctx.cr[6].eq {
	pc = 0x8252C478; continue 'dispatch;
	}
	// 8252C45C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C460: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252C464: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252C468: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8252C46C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252C470: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252C474: 4BF37C45  bl 0x824640b8
	ctx.lr = 0x8252C478;
	sub_824640B8(ctx, base);
	pc = 0x8252C478; continue 'dispatch;
            }
            0x8252C478 => {
    //   block [0x8252C478..0x8252C4B0)
	// 8252C478: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8252C47C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252C480: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C484: 409A002C  bne cr6, 0x8252c4b0
	if !ctx.cr[6].eq {
	pc = 0x8252C4B0; continue 'dispatch;
	}
	// 8252C488: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C48C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252C490: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252C494: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8252C498: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252C49C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252C4A0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252C4A4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252C4A8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252C4AC: 4BF37C0D  bl 0x824640b8
	ctx.lr = 0x8252C4B0;
	sub_824640B8(ctx, base);
	pc = 0x8252C4B0; continue 'dispatch;
            }
            0x8252C4B0 => {
    //   block [0x8252C4B0..0x8252C4D0)
	// 8252C4B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252C4B4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252C4B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C4BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252C4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C4C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C4D0 size=100
    let mut pc: u32 = 0x8252C4D0;
    'dispatch: loop {
        match pc {
            0x8252C4D0 => {
    //   block [0x8252C4D0..0x8252C518)
	// 8252C4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C4D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252C4DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C4E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C4E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252C4EC: 4BFFFF4D  bl 0x8252c438
	ctx.lr = 0x8252C4F0;
	sub_8252C438(ctx, base);
	// 8252C4F0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252C4F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C4F8: 419A0020  beq cr6, 0x8252c518
	if ctx.cr[6].eq {
	pc = 0x8252C518; continue 'dispatch;
	}
	// 8252C4FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C500: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252C504: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252C508: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252C50C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252C510: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252C514: 4BF37BA5  bl 0x824640b8
	ctx.lr = 0x8252C518;
	sub_824640B8(ctx, base);
	pc = 0x8252C518; continue 'dispatch;
            }
            0x8252C518 => {
    //   block [0x8252C518..0x8252C534)
	// 8252C518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252C51C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252C520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C528: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252C52C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C538 size=20
    let mut pc: u32 = 0x8252C538;
    'dispatch: loop {
        match pc {
            0x8252C538 => {
    //   block [0x8252C538..0x8252C54C)
	// 8252C538: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C53C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C540: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C548: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C550 size=20
    let mut pc: u32 = 0x8252C550;
    'dispatch: loop {
        match pc {
            0x8252C550 => {
    //   block [0x8252C550..0x8252C564)
	// 8252C550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C554: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C558: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C55C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C560: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C568 size=56
    let mut pc: u32 = 0x8252C568;
    'dispatch: loop {
        match pc {
            0x8252C568 => {
    //   block [0x8252C568..0x8252C5A0)
	// 8252C568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252C56C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C570: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C574: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252C578: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252C57C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8252C580: 396B0CE4  addi r11, r11, 0xce4
	ctx.r[11].s64 = ctx.r[11].s64 + 3300;
	// 8252C584: 394A601C  addi r10, r10, 0x601c
	ctx.r[10].s64 = ctx.r[10].s64 + 24604;
	// 8252C588: 3929600C  addi r9, r9, 0x600c
	ctx.r[9].s64 = ctx.r[9].s64 + 24588;
	// 8252C58C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8252C590: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8252C594: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252C598: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8252C59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C5A0 size=12
    let mut pc: u32 = 0x8252C5A0;
    'dispatch: loop {
        match pc {
            0x8252C5A0 => {
    //   block [0x8252C5A0..0x8252C5AC)
	// 8252C5A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C5A4: 386B601C  addi r3, r11, 0x601c
	ctx.r[3].s64 = ctx.r[11].s64 + 24604;
	// 8252C5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C5B0 size=40
    let mut pc: u32 = 0x8252C5B0;
    'dispatch: loop {
        match pc {
            0x8252C5B0 => {
    //   block [0x8252C5B0..0x8252C5D8)
	// 8252C5B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8252C5B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C5B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C5BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252C5C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8252C5C4: 394A602C  addi r10, r10, 0x602c
	ctx.r[10].s64 = ctx.r[10].s64 + 24620;
	// 8252C5C8: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 8252C5CC: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8252C5D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252C5D4: 48000F2C  b 0x8252d500
	sub_8252D500(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C5D8 size=4
    let mut pc: u32 = 0x8252C5D8;
    'dispatch: loop {
        match pc {
            0x8252C5D8 => {
    //   block [0x8252C5D8..0x8252C5DC)
	// 8252C5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C5E0 size=60
    let mut pc: u32 = 0x8252C5E0;
    'dispatch: loop {
        match pc {
            0x8252C5E0 => {
    //   block [0x8252C5E0..0x8252C61C)
	// 8252C5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C5E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C5EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C5F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252C5F4: 396B602C  addi r11, r11, 0x602c
	ctx.r[11].s64 = ctx.r[11].s64 + 24620;
	// 8252C5F8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8252C5FC: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8252C600: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8252C604: 48000EFD  bl 0x8252d500
	ctx.lr = 0x8252C608;
	sub_8252D500(ctx, base);
	// 8252C608: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8252C60C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8252C610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C620 size=100
    let mut pc: u32 = 0x8252C620;
    'dispatch: loop {
        match pc {
            0x8252C620 => {
    //   block [0x8252C620..0x8252C668)
	// 8252C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252C62C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C638: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252C63C: 48000EFD  bl 0x8252d538
	ctx.lr = 0x8252C640;
	sub_8252D538(ctx, base);
	// 8252C640: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252C644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C648: 419A0020  beq cr6, 0x8252c668
	if ctx.cr[6].eq {
	pc = 0x8252C668; continue 'dispatch;
	}
	// 8252C64C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C650: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252C654: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252C658: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252C65C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252C660: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252C664: 4BF37A55  bl 0x824640b8
	ctx.lr = 0x8252C668;
	sub_824640B8(ctx, base);
	pc = 0x8252C668; continue 'dispatch;
            }
            0x8252C668 => {
    //   block [0x8252C668..0x8252C684)
	// 8252C668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252C66C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252C670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C678: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252C67C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C688 size=8
    let mut pc: u32 = 0x8252C688;
    'dispatch: loop {
        match pc {
            0x8252C688 => {
    //   block [0x8252C688..0x8252C690)
	// 8252C688: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 8252C68C: 4BFFFF94  b 0x8252c620
	sub_8252C620(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C690 size=100
    let mut pc: u32 = 0x8252C690;
    'dispatch: loop {
        match pc {
            0x8252C690 => {
    //   block [0x8252C690..0x8252C6D8)
	// 8252C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252C69C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C6A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C6A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C6A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252C6AC: 480010C5  bl 0x8252d770
	ctx.lr = 0x8252C6B0;
	sub_8252D770(ctx, base);
	// 8252C6B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252C6B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C6B8: 419A0020  beq cr6, 0x8252c6d8
	if ctx.cr[6].eq {
	pc = 0x8252C6D8; continue 'dispatch;
	}
	// 8252C6BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C6C0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252C6C4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252C6C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252C6CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252C6D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252C6D4: 4BF379E5  bl 0x824640b8
	ctx.lr = 0x8252C6D8;
	sub_824640B8(ctx, base);
	pc = 0x8252C6D8; continue 'dispatch;
            }
            0x8252C6D8 => {
    //   block [0x8252C6D8..0x8252C6F4)
	// 8252C6D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252C6DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252C6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C6E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252C6EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C6F8 size=4
    let mut pc: u32 = 0x8252C6F8;
    'dispatch: loop {
        match pc {
            0x8252C6F8 => {
    //   block [0x8252C6F8..0x8252C6FC)
	// 8252C6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C700 size=4
    let mut pc: u32 = 0x8252C700;
    'dispatch: loop {
        match pc {
            0x8252C700 => {
    //   block [0x8252C700..0x8252C704)
	// 8252C700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C708 size=4
    let mut pc: u32 = 0x8252C708;
    'dispatch: loop {
        match pc {
            0x8252C708 => {
    //   block [0x8252C708..0x8252C70C)
	// 8252C708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C710 size=4
    let mut pc: u32 = 0x8252C710;
    'dispatch: loop {
        match pc {
            0x8252C710 => {
    //   block [0x8252C710..0x8252C714)
	// 8252C710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C718 size=4
    let mut pc: u32 = 0x8252C718;
    'dispatch: loop {
        match pc {
            0x8252C718 => {
    //   block [0x8252C718..0x8252C71C)
	// 8252C718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C720 size=96
    let mut pc: u32 = 0x8252C720;
    'dispatch: loop {
        match pc {
            0x8252C720 => {
    //   block [0x8252C720..0x8252C768)
	// 8252C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C728: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C72C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C730: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252C734: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C738: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252C73C: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8252C740: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252C744: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C748: 419A0020  beq cr6, 0x8252c768
	if ctx.cr[6].eq {
	pc = 0x8252C768; continue 'dispatch;
	}
	// 8252C74C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C750: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252C754: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252C758: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252C75C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252C760: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252C764: 4BF37955  bl 0x824640b8
	ctx.lr = 0x8252C768;
	sub_824640B8(ctx, base);
	pc = 0x8252C768; continue 'dispatch;
            }
            0x8252C768 => {
    //   block [0x8252C768..0x8252C780)
	// 8252C768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252C76C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252C770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C780 size=32
    let mut pc: u32 = 0x8252C780;
    'dispatch: loop {
        match pc {
            0x8252C780 => {
    //   block [0x8252C780..0x8252C7A0)
	// 8252C780: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252C784: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C788: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C78C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252C790: 396B6344  addi r11, r11, 0x6344
	ctx.r[11].s64 = ctx.r[11].s64 + 25412;
	// 8252C794: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252C798: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C7A0 size=20
    let mut pc: u32 = 0x8252C7A0;
    'dispatch: loop {
        match pc {
            0x8252C7A0 => {
    //   block [0x8252C7A0..0x8252C7B4)
	// 8252C7A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C7A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C7A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C7AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C7B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C7B8 size=12
    let mut pc: u32 = 0x8252C7B8;
    'dispatch: loop {
        match pc {
            0x8252C7B8 => {
    //   block [0x8252C7B8..0x8252C7C4)
	// 8252C7B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C7BC: 386B6344  addi r3, r11, 0x6344
	ctx.r[3].s64 = ctx.r[11].s64 + 25412;
	// 8252C7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C7C8 size=32
    let mut pc: u32 = 0x8252C7C8;
    'dispatch: loop {
        match pc {
            0x8252C7C8 => {
    //   block [0x8252C7C8..0x8252C7E8)
	// 8252C7C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252C7CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C7D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C7D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252C7D8: 396B6358  addi r11, r11, 0x6358
	ctx.r[11].s64 = ctx.r[11].s64 + 25432;
	// 8252C7DC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252C7E0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C7E8 size=20
    let mut pc: u32 = 0x8252C7E8;
    'dispatch: loop {
        match pc {
            0x8252C7E8 => {
    //   block [0x8252C7E8..0x8252C7FC)
	// 8252C7E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C7EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C7F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C7F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C7F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C800 size=12
    let mut pc: u32 = 0x8252C800;
    'dispatch: loop {
        match pc {
            0x8252C800 => {
    //   block [0x8252C800..0x8252C80C)
	// 8252C800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C804: 386B6358  addi r3, r11, 0x6358
	ctx.r[3].s64 = ctx.r[11].s64 + 25432;
	// 8252C808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C810 size=4
    let mut pc: u32 = 0x8252C810;
    'dispatch: loop {
        match pc {
            0x8252C810 => {
    //   block [0x8252C810..0x8252C814)
	// 8252C810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C818 size=20
    let mut pc: u32 = 0x8252C818;
    'dispatch: loop {
        match pc {
            0x8252C818 => {
    //   block [0x8252C818..0x8252C82C)
	// 8252C818: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C81C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C820: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C824: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C828: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C830 size=32
    let mut pc: u32 = 0x8252C830;
    'dispatch: loop {
        match pc {
            0x8252C830 => {
    //   block [0x8252C830..0x8252C850)
	// 8252C830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252C834: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C838: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C83C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252C840: 396B6484  addi r11, r11, 0x6484
	ctx.r[11].s64 = ctx.r[11].s64 + 25732;
	// 8252C844: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252C848: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C850 size=12
    let mut pc: u32 = 0x8252C850;
    'dispatch: loop {
        match pc {
            0x8252C850 => {
    //   block [0x8252C850..0x8252C85C)
	// 8252C850: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C854: 386B6484  addi r3, r11, 0x6484
	ctx.r[3].s64 = ctx.r[11].s64 + 25732;
	// 8252C858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C860 size=140
    let mut pc: u32 = 0x8252C860;
    'dispatch: loop {
        match pc {
            0x8252C860 => {
    //   block [0x8252C860..0x8252C8A0)
	// 8252C860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C86C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C874: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8252C878: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252C87C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C880: 409A0020  bne cr6, 0x8252c8a0
	if !ctx.cr[6].eq {
	pc = 0x8252C8A0; continue 'dispatch;
	}
	// 8252C884: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C888: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252C88C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252C890: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252C894: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252C898: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252C89C: 4BF3781D  bl 0x824640b8
	ctx.lr = 0x8252C8A0;
	sub_824640B8(ctx, base);
	pc = 0x8252C8A0; continue 'dispatch;
            }
            0x8252C8A0 => {
    //   block [0x8252C8A0..0x8252C8CC)
	// 8252C8A0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252C8A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252C8A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C8AC: 409A0020  bne cr6, 0x8252c8cc
	if !ctx.cr[6].eq {
	pc = 0x8252C8CC; continue 'dispatch;
	}
	// 8252C8B0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C8B4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252C8B8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252C8BC: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252C8C0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252C8C4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252C8C8: 4BF377F1  bl 0x824640b8
	ctx.lr = 0x8252C8CC;
	sub_824640B8(ctx, base);
	pc = 0x8252C8CC; continue 'dispatch;
            }
            0x8252C8CC => {
    //   block [0x8252C8CC..0x8252C8EC)
	// 8252C8CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252C8D0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252C8D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252C8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C8E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C8F0 size=100
    let mut pc: u32 = 0x8252C8F0;
    'dispatch: loop {
        match pc {
            0x8252C8F0 => {
    //   block [0x8252C8F0..0x8252C938)
	// 8252C8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C8F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252C8FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C908: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252C90C: 4BFFFF55  bl 0x8252c860
	ctx.lr = 0x8252C910;
	sub_8252C860(ctx, base);
	// 8252C910: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252C914: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252C918: 419A0020  beq cr6, 0x8252c938
	if ctx.cr[6].eq {
	pc = 0x8252C938; continue 'dispatch;
	}
	// 8252C91C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C920: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252C924: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252C928: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252C92C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252C930: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252C934: 4BF37785  bl 0x824640b8
	ctx.lr = 0x8252C938;
	sub_824640B8(ctx, base);
	pc = 0x8252C938; continue 'dispatch;
            }
            0x8252C938 => {
    //   block [0x8252C938..0x8252C954)
	// 8252C938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252C93C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252C940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252C944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252C948: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252C94C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252C950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C958 size=4
    let mut pc: u32 = 0x8252C958;
    'dispatch: loop {
        match pc {
            0x8252C958 => {
    //   block [0x8252C958..0x8252C95C)
	// 8252C958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C960 size=20
    let mut pc: u32 = 0x8252C960;
    'dispatch: loop {
        match pc {
            0x8252C960 => {
    //   block [0x8252C960..0x8252C974)
	// 8252C960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252C968: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252C970: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C978 size=32
    let mut pc: u32 = 0x8252C978;
    'dispatch: loop {
        match pc {
            0x8252C978 => {
    //   block [0x8252C978..0x8252C998)
	// 8252C978: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252C97C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252C980: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252C988: 396B650C  addi r11, r11, 0x650c
	ctx.r[11].s64 = ctx.r[11].s64 + 25868;
	// 8252C98C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252C990: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252C994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252C998 size=12
    let mut pc: u32 = 0x8252C998;
    'dispatch: loop {
        match pc {
            0x8252C998 => {
    //   block [0x8252C998..0x8252C9A4)
	// 8252C998: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252C99C: 386B650C  addi r3, r11, 0x650c
	ctx.r[3].s64 = ctx.r[11].s64 + 25868;
	// 8252C9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252C9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252C9A8 size=164
    let mut pc: u32 = 0x8252C9A8;
    'dispatch: loop {
        match pc {
            0x8252C9A8 => {
    //   block [0x8252C9A8..0x8252C9F4)
	// 8252C9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252C9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252C9B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252C9B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252C9B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252C9BC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252C9C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252C9C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252C9C8: 409A002C  bne cr6, 0x8252c9f4
	if !ctx.cr[6].eq {
	pc = 0x8252C9F4; continue 'dispatch;
	}
	// 8252C9CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252C9D0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252C9D4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252C9D8: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252C9DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252C9E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252C9E4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252C9E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252C9EC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252C9F0: 4BF376C9  bl 0x824640b8
	ctx.lr = 0x8252C9F4;
	sub_824640B8(ctx, base);
	pc = 0x8252C9F4; continue 'dispatch;
            }
            0x8252C9F4 => {
    //   block [0x8252C9F4..0x8252CA2C)
	// 8252C9F4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252C9F8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252C9FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252CA00: 409A002C  bne cr6, 0x8252ca2c
	if !ctx.cr[6].eq {
	pc = 0x8252CA2C; continue 'dispatch;
	}
	// 8252CA04: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CA08: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252CA0C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252CA10: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252CA14: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252CA18: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252CA1C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252CA20: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252CA24: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252CA28: 4BF37691  bl 0x824640b8
	ctx.lr = 0x8252CA2C;
	sub_824640B8(ctx, base);
	pc = 0x8252CA2C; continue 'dispatch;
            }
            0x8252CA2C => {
    //   block [0x8252CA2C..0x8252CA4C)
	// 8252CA2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252CA30: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252CA34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CA44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252CA50 size=100
    let mut pc: u32 = 0x8252CA50;
    'dispatch: loop {
        match pc {
            0x8252CA50 => {
    //   block [0x8252CA50..0x8252CA98)
	// 8252CA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252CA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252CA58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252CA5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252CA60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252CA64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252CA68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252CA6C: 4BFFFF3D  bl 0x8252c9a8
	ctx.lr = 0x8252CA70;
	sub_8252C9A8(ctx, base);
	// 8252CA70: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252CA74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252CA78: 419A0020  beq cr6, 0x8252ca98
	if ctx.cr[6].eq {
	pc = 0x8252CA98; continue 'dispatch;
	}
	// 8252CA7C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CA80: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252CA84: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252CA88: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252CA8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252CA90: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252CA94: 4BF37625  bl 0x824640b8
	ctx.lr = 0x8252CA98;
	sub_824640B8(ctx, base);
	pc = 0x8252CA98; continue 'dispatch;
            }
            0x8252CA98 => {
    //   block [0x8252CA98..0x8252CAB4)
	// 8252CA98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252CA9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252CAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CAA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252CAAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CAB8 size=4
    let mut pc: u32 = 0x8252CAB8;
    'dispatch: loop {
        match pc {
            0x8252CAB8 => {
    //   block [0x8252CAB8..0x8252CABC)
	// 8252CAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CAC0 size=20
    let mut pc: u32 = 0x8252CAC0;
    'dispatch: loop {
        match pc {
            0x8252CAC0 => {
    //   block [0x8252CAC0..0x8252CAD4)
	// 8252CAC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CAC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CAC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CAD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CAD8 size=32
    let mut pc: u32 = 0x8252CAD8;
    'dispatch: loop {
        match pc {
            0x8252CAD8 => {
    //   block [0x8252CAD8..0x8252CAF8)
	// 8252CAD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CADC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CAE0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CAE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CAE8: 396B659C  addi r11, r11, 0x659c
	ctx.r[11].s64 = ctx.r[11].s64 + 26012;
	// 8252CAEC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CAF0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CAF8 size=12
    let mut pc: u32 = 0x8252CAF8;
    'dispatch: loop {
        match pc {
            0x8252CAF8 => {
    //   block [0x8252CAF8..0x8252CB04)
	// 8252CAF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CAFC: 386B659C  addi r3, r11, 0x659c
	ctx.r[3].s64 = ctx.r[11].s64 + 26012;
	// 8252CB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252CB08 size=164
    let mut pc: u32 = 0x8252CB08;
    'dispatch: loop {
        match pc {
            0x8252CB08 => {
    //   block [0x8252CB08..0x8252CB5C)
	// 8252CB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252CB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252CB10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252CB14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252CB18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252CB1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252CB20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252CB24: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252CB28: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252CB2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252CB30: 409A002C  bne cr6, 0x8252cb5c
	if !ctx.cr[6].eq {
	pc = 0x8252CB5C; continue 'dispatch;
	}
	// 8252CB34: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CB38: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252CB3C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252CB40: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252CB44: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252CB48: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252CB4C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252CB50: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252CB54: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252CB58: 4BF37561  bl 0x824640b8
	ctx.lr = 0x8252CB5C;
	sub_824640B8(ctx, base);
	pc = 0x8252CB5C; continue 'dispatch;
            }
            0x8252CB5C => {
    //   block [0x8252CB5C..0x8252CB90)
	// 8252CB5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252CB60: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252CB64: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252CB68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252CB6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CB70: 419A0020  beq cr6, 0x8252cb90
	if ctx.cr[6].eq {
	pc = 0x8252CB90; continue 'dispatch;
	}
	// 8252CB74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CB78: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252CB7C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252CB80: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252CB84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252CB88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252CB8C: 4BF3752D  bl 0x824640b8
	ctx.lr = 0x8252CB90;
	sub_824640B8(ctx, base);
	pc = 0x8252CB90; continue 'dispatch;
            }
            0x8252CB90 => {
    //   block [0x8252CB90..0x8252CBAC)
	// 8252CB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252CB94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252CB98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CB9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CBA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252CBA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CBB0 size=4
    let mut pc: u32 = 0x8252CBB0;
    'dispatch: loop {
        match pc {
            0x8252CBB0 => {
    //   block [0x8252CBB0..0x8252CBB4)
	// 8252CBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CBB8 size=4
    let mut pc: u32 = 0x8252CBB8;
    'dispatch: loop {
        match pc {
            0x8252CBB8 => {
    //   block [0x8252CBB8..0x8252CBBC)
	// 8252CBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CBC0 size=20
    let mut pc: u32 = 0x8252CBC0;
    'dispatch: loop {
        match pc {
            0x8252CBC0 => {
    //   block [0x8252CBC0..0x8252CBD4)
	// 8252CBC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CBC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CBC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CBCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CBD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CBD8 size=20
    let mut pc: u32 = 0x8252CBD8;
    'dispatch: loop {
        match pc {
            0x8252CBD8 => {
    //   block [0x8252CBD8..0x8252CBEC)
	// 8252CBD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CBDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CBE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CBE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CBE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CBF0 size=32
    let mut pc: u32 = 0x8252CBF0;
    'dispatch: loop {
        match pc {
            0x8252CBF0 => {
    //   block [0x8252CBF0..0x8252CC10)
	// 8252CBF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CBF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CBF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CBFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CC00: 396B6674  addi r11, r11, 0x6674
	ctx.r[11].s64 = ctx.r[11].s64 + 26228;
	// 8252CC04: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CC08: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CC10 size=12
    let mut pc: u32 = 0x8252CC10;
    'dispatch: loop {
        match pc {
            0x8252CC10 => {
    //   block [0x8252CC10..0x8252CC1C)
	// 8252CC10: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CC14: 386B6674  addi r3, r11, 0x6674
	ctx.r[3].s64 = ctx.r[11].s64 + 26228;
	// 8252CC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CC20 size=32
    let mut pc: u32 = 0x8252CC20;
    'dispatch: loop {
        match pc {
            0x8252CC20 => {
    //   block [0x8252CC20..0x8252CC40)
	// 8252CC20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CC24: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CC28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CC2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CC30: 396B6684  addi r11, r11, 0x6684
	ctx.r[11].s64 = ctx.r[11].s64 + 26244;
	// 8252CC34: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CC38: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CC40 size=12
    let mut pc: u32 = 0x8252CC40;
    'dispatch: loop {
        match pc {
            0x8252CC40 => {
    //   block [0x8252CC40..0x8252CC4C)
	// 8252CC40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CC44: 386B6684  addi r3, r11, 0x6684
	ctx.r[3].s64 = ctx.r[11].s64 + 26244;
	// 8252CC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252CC50 size=164
    let mut pc: u32 = 0x8252CC50;
    'dispatch: loop {
        match pc {
            0x8252CC50 => {
    //   block [0x8252CC50..0x8252CCA4)
	// 8252CC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252CC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252CC58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252CC5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252CC60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252CC64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CC68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252CC6C: 396B6674  addi r11, r11, 0x6674
	ctx.r[11].s64 = ctx.r[11].s64 + 26228;
	// 8252CC70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252CC74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CC78: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252CC7C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252CC80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252CC84: 409A0020  bne cr6, 0x8252cca4
	if !ctx.cr[6].eq {
	pc = 0x8252CCA4; continue 'dispatch;
	}
	// 8252CC88: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CC8C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252CC90: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252CC94: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252CC98: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252CC9C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252CCA0: 4BF37419  bl 0x824640b8
	ctx.lr = 0x8252CCA4;
	sub_824640B8(ctx, base);
	pc = 0x8252CCA4; continue 'dispatch;
            }
            0x8252CCA4 => {
    //   block [0x8252CCA4..0x8252CCD8)
	// 8252CCA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252CCA8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252CCAC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252CCB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252CCB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CCB8: 419A0020  beq cr6, 0x8252ccd8
	if ctx.cr[6].eq {
	pc = 0x8252CCD8; continue 'dispatch;
	}
	// 8252CCBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CCC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252CCC4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252CCC8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252CCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252CCD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252CCD4: 4BF373E5  bl 0x824640b8
	ctx.lr = 0x8252CCD8;
	sub_824640B8(ctx, base);
	pc = 0x8252CCD8; continue 'dispatch;
            }
            0x8252CCD8 => {
    //   block [0x8252CCD8..0x8252CCF4)
	// 8252CCD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252CCDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252CCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CCE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252CCEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252CCF8 size=100
    let mut pc: u32 = 0x8252CCF8;
    'dispatch: loop {
        match pc {
            0x8252CCF8 => {
    //   block [0x8252CCF8..0x8252CD40)
	// 8252CCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252CCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252CD00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252CD04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252CD08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252CD0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252CD10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252CD14: 48002305  bl 0x8252f018
	ctx.lr = 0x8252CD18;
	sub_8252F018(ctx, base);
	// 8252CD18: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252CD1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252CD20: 419A0020  beq cr6, 0x8252cd40
	if ctx.cr[6].eq {
	pc = 0x8252CD40; continue 'dispatch;
	}
	// 8252CD24: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CD28: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252CD2C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252CD30: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252CD34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252CD38: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252CD3C: 4BF3737D  bl 0x824640b8
	ctx.lr = 0x8252CD40;
	sub_824640B8(ctx, base);
	pc = 0x8252CD40; continue 'dispatch;
            }
            0x8252CD40 => {
    //   block [0x8252CD40..0x8252CD5C)
	// 8252CD40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252CD44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252CD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CD50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252CD54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CD60 size=20
    let mut pc: u32 = 0x8252CD60;
    'dispatch: loop {
        match pc {
            0x8252CD60 => {
    //   block [0x8252CD60..0x8252CD74)
	// 8252CD60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CD64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CD68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CD6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CD70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CD78 size=32
    let mut pc: u32 = 0x8252CD78;
    'dispatch: loop {
        match pc {
            0x8252CD78 => {
    //   block [0x8252CD78..0x8252CD98)
	// 8252CD78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CD7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CD80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CD84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CD88: 396B678C  addi r11, r11, 0x678c
	ctx.r[11].s64 = ctx.r[11].s64 + 26508;
	// 8252CD8C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CD90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CD98 size=12
    let mut pc: u32 = 0x8252CD98;
    'dispatch: loop {
        match pc {
            0x8252CD98 => {
    //   block [0x8252CD98..0x8252CDA4)
	// 8252CD98: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CD9C: 386B678C  addi r3, r11, 0x678c
	ctx.r[3].s64 = ctx.r[11].s64 + 26508;
	// 8252CDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CDA8 size=4
    let mut pc: u32 = 0x8252CDA8;
    'dispatch: loop {
        match pc {
            0x8252CDA8 => {
    //   block [0x8252CDA8..0x8252CDAC)
	// 8252CDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CDB0 size=20
    let mut pc: u32 = 0x8252CDB0;
    'dispatch: loop {
        match pc {
            0x8252CDB0 => {
    //   block [0x8252CDB0..0x8252CDC4)
	// 8252CDB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CDB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CDB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CDBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CDC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CDC8 size=32
    let mut pc: u32 = 0x8252CDC8;
    'dispatch: loop {
        match pc {
            0x8252CDC8 => {
    //   block [0x8252CDC8..0x8252CDE8)
	// 8252CDC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CDCC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CDD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CDD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CDD8: 396B6B94  addi r11, r11, 0x6b94
	ctx.r[11].s64 = ctx.r[11].s64 + 27540;
	// 8252CDDC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CDE0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CDE8 size=12
    let mut pc: u32 = 0x8252CDE8;
    'dispatch: loop {
        match pc {
            0x8252CDE8 => {
    //   block [0x8252CDE8..0x8252CDF4)
	// 8252CDE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CDEC: 386B6B94  addi r3, r11, 0x6b94
	ctx.r[3].s64 = ctx.r[11].s64 + 27540;
	// 8252CDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252CDF8 size=100
    let mut pc: u32 = 0x8252CDF8;
    'dispatch: loop {
        match pc {
            0x8252CDF8 => {
    //   block [0x8252CDF8..0x8252CE40)
	// 8252CDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252CDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252CE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252CE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252CE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252CE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252CE10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252CE14: 48002A45  bl 0x8252f858
	ctx.lr = 0x8252CE18;
	sub_8252F858(ctx, base);
	// 8252CE18: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252CE1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252CE20: 419A0020  beq cr6, 0x8252ce40
	if ctx.cr[6].eq {
	pc = 0x8252CE40; continue 'dispatch;
	}
	// 8252CE24: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CE28: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252CE2C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252CE30: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252CE34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252CE38: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252CE3C: 4BF3727D  bl 0x824640b8
	ctx.lr = 0x8252CE40;
	sub_824640B8(ctx, base);
	pc = 0x8252CE40; continue 'dispatch;
            }
            0x8252CE40 => {
    //   block [0x8252CE40..0x8252CE5C)
	// 8252CE40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252CE44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252CE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CE50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252CE54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CE60 size=20
    let mut pc: u32 = 0x8252CE60;
    'dispatch: loop {
        match pc {
            0x8252CE60 => {
    //   block [0x8252CE60..0x8252CE74)
	// 8252CE60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CE64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CE68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CE70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CE78 size=32
    let mut pc: u32 = 0x8252CE78;
    'dispatch: loop {
        match pc {
            0x8252CE78 => {
    //   block [0x8252CE78..0x8252CE98)
	// 8252CE78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CE7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CE80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CE84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CE88: 396B6C1C  addi r11, r11, 0x6c1c
	ctx.r[11].s64 = ctx.r[11].s64 + 27676;
	// 8252CE8C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CE90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CE98 size=12
    let mut pc: u32 = 0x8252CE98;
    'dispatch: loop {
        match pc {
            0x8252CE98 => {
    //   block [0x8252CE98..0x8252CEA4)
	// 8252CE98: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CE9C: 386B6C1C  addi r3, r11, 0x6c1c
	ctx.r[3].s64 = ctx.r[11].s64 + 27676;
	// 8252CEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CEA8 size=20
    let mut pc: u32 = 0x8252CEA8;
    'dispatch: loop {
        match pc {
            0x8252CEA8 => {
    //   block [0x8252CEA8..0x8252CEBC)
	// 8252CEA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CEAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CEB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CEB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CEB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CEC0 size=32
    let mut pc: u32 = 0x8252CEC0;
    'dispatch: loop {
        match pc {
            0x8252CEC0 => {
    //   block [0x8252CEC0..0x8252CEE0)
	// 8252CEC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CEC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CEC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CECC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CED0: 396B6C7C  addi r11, r11, 0x6c7c
	ctx.r[11].s64 = ctx.r[11].s64 + 27772;
	// 8252CED4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CED8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CEE0 size=12
    let mut pc: u32 = 0x8252CEE0;
    'dispatch: loop {
        match pc {
            0x8252CEE0 => {
    //   block [0x8252CEE0..0x8252CEEC)
	// 8252CEE0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CEE4: 386B6C7C  addi r3, r11, 0x6c7c
	ctx.r[3].s64 = ctx.r[11].s64 + 27772;
	// 8252CEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252CEF0 size=152
    let mut pc: u32 = 0x8252CEF0;
    'dispatch: loop {
        match pc {
            0x8252CEF0 => {
    //   block [0x8252CEF0..0x8252CF38)
	// 8252CEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252CEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252CEF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252CEFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252CF00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252CF04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252CF08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252CF0C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252CF10: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252CF14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252CF18: 409A0020  bne cr6, 0x8252cf38
	if !ctx.cr[6].eq {
	pc = 0x8252CF38; continue 'dispatch;
	}
	// 8252CF1C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CF20: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252CF24: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252CF28: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252CF2C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252CF30: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252CF34: 4BF37185  bl 0x824640b8
	ctx.lr = 0x8252CF38;
	sub_824640B8(ctx, base);
	pc = 0x8252CF38; continue 'dispatch;
            }
            0x8252CF38 => {
    //   block [0x8252CF38..0x8252CF6C)
	// 8252CF38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252CF3C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8252CF40: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252CF44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252CF48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CF4C: 419A0020  beq cr6, 0x8252cf6c
	if ctx.cr[6].eq {
	pc = 0x8252CF6C; continue 'dispatch;
	}
	// 8252CF50: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CF54: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252CF58: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8252CF5C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252CF60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252CF64: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252CF68: 4BF37151  bl 0x824640b8
	ctx.lr = 0x8252CF6C;
	sub_824640B8(ctx, base);
	pc = 0x8252CF6C; continue 'dispatch;
            }
            0x8252CF6C => {
    //   block [0x8252CF6C..0x8252CF88)
	// 8252CF6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252CF70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252CF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252CF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252CF7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252CF80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252CF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CF88 size=20
    let mut pc: u32 = 0x8252CF88;
    'dispatch: loop {
        match pc {
            0x8252CF88 => {
    //   block [0x8252CF88..0x8252CF9C)
	// 8252CF88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252CF90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252CF94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252CF98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CFA0 size=32
    let mut pc: u32 = 0x8252CFA0;
    'dispatch: loop {
        match pc {
            0x8252CFA0 => {
    //   block [0x8252CFA0..0x8252CFC0)
	// 8252CFA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252CFA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252CFA8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CFAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252CFB0: 396B6D24  addi r11, r11, 0x6d24
	ctx.r[11].s64 = ctx.r[11].s64 + 27940;
	// 8252CFB4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252CFB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252CFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CFC0 size=12
    let mut pc: u32 = 0x8252CFC0;
    'dispatch: loop {
        match pc {
            0x8252CFC0 => {
    //   block [0x8252CFC0..0x8252CFCC)
	// 8252CFC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252CFC4: 386B6D24  addi r3, r11, 0x6d24
	ctx.r[3].s64 = ctx.r[11].s64 + 27940;
	// 8252CFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252CFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252CFD0 size=232
    let mut pc: u32 = 0x8252CFD0;
    'dispatch: loop {
        match pc {
            0x8252CFD0 => {
    //   block [0x8252CFD0..0x8252D0B8)
	// 8252CFD0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8252CFD4: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 8252CFD8: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 8252CFDC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8252CFE0: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 8252CFE4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252D0B8 size=896
    let mut pc: u32 = 0x8252D0B8;
    'dispatch: loop {
        match pc {
            0x8252D0B8 => {
    //   block [0x8252D0B8..0x8252D108)
	// 8252D0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D0BC: 48007FE1  bl 0x8253509c
	ctx.lr = 0x8252D0C0;
	sub_82535080(ctx, base);
	// 8252D0C0: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 8252D0C4: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D0C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252D0CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8252D0D0: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8252D0D4: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8252D0D8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8252D0DC: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 8252D0E0: 997F0190  stb r11, 0x190(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u8 ) };
	// 8252D0E4: 3B1F0020  addi r24, r31, 0x20
	ctx.r[24].s64 = ctx.r[31].s64 + 32;
	// 8252D0E8: 897F0020  lbz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252D0EC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252D0F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252D0F4: 4099003C  ble cr6, 0x8252d130
	if !ctx.cr[6].gt {
	pc = 0x8252D130; continue 'dispatch;
	}
	// 8252D0F8: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8252D0FC: 89380000  lbz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D100: 394B0024  addi r10, r11, 0x24
	ctx.r[10].s64 = ctx.r[11].s64 + 36;
	// 8252D104: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	pc = 0x8252D108; continue 'dispatch;
            }
            0x8252D108 => {
    //   block [0x8252D108..0x8252D120)
	// 8252D108: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D10C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252D110: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252D114: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8252D118: 40990008  ble cr6, 0x8252d120
	if !ctx.cr[6].gt {
	pc = 0x8252D120; continue 'dispatch;
	}
	// 8252D11C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x8252D120; continue 'dispatch;
            }
            0x8252D120 => {
    //   block [0x8252D120..0x8252D130)
	// 8252D120: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8252D124: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 8252D128: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8252D12C: 409AFFDC  bne cr6, 0x8252d108
	if !ctx.cr[6].eq {
	pc = 0x8252D108; continue 'dispatch;
	}
	pc = 0x8252D130; continue 'dispatch;
            }
            0x8252D130 => {
    //   block [0x8252D130..0x8252D160)
	// 8252D130: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8252D134: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252D138: 4099003C  ble cr6, 0x8252d174
	if !ctx.cr[6].gt {
	pc = 0x8252D174; continue 'dispatch;
	}
	// 8252D13C: 3BBF0098  addi r29, r31, 0x98
	ctx.r[29].s64 = ctx.r[31].s64 + 152;
	// 8252D140: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252D144: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252D148: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8252D14C: 40980024  bge cr6, 0x8252d170
	if !ctx.cr[6].lt {
	pc = 0x8252D170; continue 'dispatch;
	}
	// 8252D150: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252D154: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252D158: 41980008  blt cr6, 0x8252d160
	if ctx.cr[6].lt {
	pc = 0x8252D160; continue 'dispatch;
	}
	// 8252D15C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8252D160; continue 'dispatch;
            }
            0x8252D160 => {
    //   block [0x8252D160..0x8252D170)
	// 8252D160: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252D164: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8252D168: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252D16C: 4BF4115D  bl 0x8246e2c8
	ctx.lr = 0x8252D170;
	sub_8246E2C8(ctx, base);
	pc = 0x8252D170; continue 'dispatch;
            }
            0x8252D170 => {
    //   block [0x8252D170..0x8252D174)
	// 8252D170: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	pc = 0x8252D174; continue 'dispatch;
            }
            0x8252D174 => {
    //   block [0x8252D174..0x8252D184)
	// 8252D174: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8252D178: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 8252D17C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252D180: 4099001C  ble cr6, 0x8252d19c
	if !ctx.cr[6].gt {
	pc = 0x8252D19C; continue 'dispatch;
	}
	pc = 0x8252D184; continue 'dispatch;
            }
            0x8252D184 => {
    //   block [0x8252D184..0x8252D19C)
	// 8252D184: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8252D188: 7ECA59AE  stbx r22, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[22].u8) };
	// 8252D18C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252D190: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8252D194: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8252D198: 4198FFEC  blt cr6, 0x8252d184
	if ctx.cr[6].lt {
	pc = 0x8252D184; continue 'dispatch;
	}
	pc = 0x8252D19C; continue 'dispatch;
            }
            0x8252D19C => {
    //   block [0x8252D19C..0x8252D1B4)
	// 8252D19C: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D1A0: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 8252D1A4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252D1A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252D1AC: 40990040  ble cr6, 0x8252d1ec
	if !ctx.cr[6].gt {
	pc = 0x8252D1EC; continue 'dispatch;
	}
	// 8252D1B0: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	pc = 0x8252D1B4; continue 'dispatch;
            }
            0x8252D1B4 => {
    //   block [0x8252D1B4..0x8252D1EC)
	// 8252D1B4: 813F008C  lwz r9, 0x8c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8252D1B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8252D1BC: 811F0098  lwz r8, 0x98(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8252D1C0: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8252D1C4: 396B0028  addi r11, r11, 0x28
	ctx.r[11].s64 = ctx.r[11].s64 + 40;
	// 8252D1C8: 89290024  lbz r9, 0x24(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252D1CC: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8252D1D0: 7CE940AE  lbzx r7, r9, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8252D1D4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8252D1D8: 7CE941AE  stbx r7, r9, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u8) };
	// 8252D1DC: 89380000  lbz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D1E0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8252D1E4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252D1E8: 4198FFCC  blt cr6, 0x8252d1b4
	if ctx.cr[6].lt {
	pc = 0x8252D1B4; continue 'dispatch;
	}
	pc = 0x8252D1EC; continue 'dispatch;
            }
            0x8252D1EC => {
    //   block [0x8252D1EC..0x8252D438)
	// 8252D1EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D1F0: C1BF0074  lfs f13, 0x74(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252D1F4: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 8252D1F8: C01F0068  lfs f0, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252D1FC: 3B6B0E30  addi r27, r11, 0xe30
	ctx.r[27].s64 = ctx.r[11].s64 + 3632;
	// 8252D200: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8252D204: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8252D208: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8252D20C: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 8252D210: C1BF0078  lfs f13, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252D214: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
	// 8252D218: C01F0060  lfs f0, 0x60(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252D438 size=188
    let mut pc: u32 = 0x8252D438;
    'dispatch: loop {
        match pc {
            0x8252D438 => {
    //   block [0x8252D438..0x8252D4C8)
	// 8252D438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252D440: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252D444: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252D448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D44C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252D450: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8252D454: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252D458: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8252D45C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252D460: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252D468: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252D46C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252D470: 4E800421  bctrl
	ctx.lr = 0x8252D474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252D474: 815E0094  lwz r10, 0x94(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8252D478: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8252D47C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252D480: 409A0048  bne cr6, 0x8252d4c8
	if !ctx.cr[6].eq {
	pc = 0x8252D4C8; continue 'dispatch;
	}
	// 8252D484: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D488: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252D48C: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8252D490: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8252D494: 80DE008C  lwz r6, 0x8c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 8252D498: 38896D3C  addi r4, r9, 0x6d3c
	ctx.r[4].s64 = ctx.r[9].s64 + 27964;
	// 8252D49C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8252D4A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252D4A4: 83C80008  lwz r30, 8(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252D4A8: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252D4AC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8252D4B0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8252D4B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252D4B8: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252D4BC: 55671838  slwi r7, r11, 3
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8252D4C0: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 8252D4C4: 4E800421  bctrl
	ctx.lr = 0x8252D4C8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252D4C8 => {
    //   block [0x8252D4C8..0x8252D4F4)
	// 8252D4C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252D4D0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252D4D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252D4D8: 4E800421  bctrl
	ctx.lr = 0x8252D4DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252D4DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252D4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252D4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252D4E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252D4EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252D4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D4F8 size=4
    let mut pc: u32 = 0x8252D4F8;
    'dispatch: loop {
        match pc {
            0x8252D4F8 => {
    //   block [0x8252D4F8..0x8252D4FC)
	// 8252D4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D500 size=56
    let mut pc: u32 = 0x8252D500;
    'dispatch: loop {
        match pc {
            0x8252D500 => {
    //   block [0x8252D500..0x8252D538)
	// 8252D500: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D504: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252D508: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252D50C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8252D510: 396B0CE4  addi r11, r11, 0xce4
	ctx.r[11].s64 = ctx.r[11].s64 + 3300;
	// 8252D514: 394A601C  addi r10, r10, 0x601c
	ctx.r[10].s64 = ctx.r[10].s64 + 24604;
	// 8252D518: 3929600C  addi r9, r9, 0x600c
	ctx.r[9].s64 = ctx.r[9].s64 + 24588;
	// 8252D51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8252D520: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8252D524: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8252D528: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252D52C: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8252D530: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8252D534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D538 size=40
    let mut pc: u32 = 0x8252D538;
    'dispatch: loop {
        match pc {
            0x8252D538 => {
    //   block [0x8252D538..0x8252D560)
	// 8252D538: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D53C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252D540: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252D544: 396B600C  addi r11, r11, 0x600c
	ctx.r[11].s64 = ctx.r[11].s64 + 24588;
	// 8252D548: 394A0CE4  addi r10, r10, 0xce4
	ctx.r[10].s64 = ctx.r[10].s64 + 3300;
	// 8252D54C: 39296DD0  addi r9, r9, 0x6dd0
	ctx.r[9].s64 = ctx.r[9].s64 + 28112;
	// 8252D550: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8252D554: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8252D558: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252D55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D560 size=28
    let mut pc: u32 = 0x8252D560;
    'dispatch: loop {
        match pc {
            0x8252D560 => {
    //   block [0x8252D560..0x8252D57C)
	// 8252D560: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252D564: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252D568: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8252D56C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8252D570: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8252D574: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8252D578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D580 size=4
    let mut pc: u32 = 0x8252D580;
    'dispatch: loop {
        match pc {
            0x8252D580 => {
    //   block [0x8252D580..0x8252D584)
	// 8252D580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D588 size=84
    let mut pc: u32 = 0x8252D588;
    'dispatch: loop {
        match pc {
            0x8252D588 => {
    //   block [0x8252D588..0x8252D5DC)
	// 8252D588: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D58C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252D590: 396B602C  addi r11, r11, 0x602c
	ctx.r[11].s64 = ctx.r[11].s64 + 24620;
	// 8252D594: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8252D598: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252D59C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8252D5A0: 394A0CE4  addi r10, r10, 0xce4
	ctx.r[10].s64 = ctx.r[10].s64 + 3300;
	// 8252D5A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252D5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252D5AC: 3929601C  addi r9, r9, 0x601c
	ctx.r[9].s64 = ctx.r[9].s64 + 24604;
	// 8252D5B0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 8252D5B4: 3908600C  addi r8, r8, 0x600c
	ctx.r[8].s64 = ctx.r[8].s64 + 24588;
	// 8252D5B8: B0E3001A  sth r7, 0x1a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[7].u16 ) };
	// 8252D5BC: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8252D5C0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8252D5C4: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8252D5C8: 9103001C  stw r8, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8252D5CC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8252D5D0: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8252D5D4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8252D5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252D5E0 size=184
    let mut pc: u32 = 0x8252D5E0;
    'dispatch: loop {
        match pc {
            0x8252D5E0 => {
    //   block [0x8252D5E0..0x8252D698)
	// 8252D5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252D5E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252D5EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252D5F0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D5F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252D5F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252D5FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8252D600: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D604: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252D608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252D60C: 4E800421  bctrl
	ctx.lr = 0x8252D610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252D610: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8252D614: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D618: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252D61C: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 8252D620: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252D698 size=216
    let mut pc: u32 = 0x8252D698;
    'dispatch: loop {
        match pc {
            0x8252D698 => {
    //   block [0x8252D698..0x8252D744)
	// 8252D698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D69C: 48007A19  bl 0x825350b4
	ctx.lr = 0x8252D6A0;
	sub_82535080(ctx, base);
	// 8252D6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D6A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D6A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252D6AC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8252D6B0: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8252D6B4: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8252D6B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8252D6BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252D6C0: 4BF36979  bl 0x82464038
	ctx.lr = 0x8252D6C4;
	sub_82464038(ctx, base);
	// 8252D6C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D6C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252D6CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252D6D0: 396B602C  addi r11, r11, 0x602c
	ctx.r[11].s64 = ctx.r[11].s64 + 24620;
	// 8252D6D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8252D6D8: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8252D6DC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252D6E0: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8252D6E4: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 8252D6E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252D6EC: 394A0CE4  addi r10, r10, 0xce4
	ctx.r[10].s64 = ctx.r[10].s64 + 3300;
	// 8252D6F0: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 8252D6F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8252D6F8: B0DF0004  sth r6, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 8252D6FC: 3929601C  addi r9, r9, 0x601c
	ctx.r[9].s64 = ctx.r[9].s64 + 24604;
	// 8252D700: 3908600C  addi r8, r8, 0x600c
	ctx.r[8].s64 = ctx.r[8].s64 + 24588;
	// 8252D704: B0FE0006  sth r7, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 8252D708: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8252D70C: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8252D710: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252D714: 911E0008  stw r8, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8252D718: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8252D71C: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 8252D720: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8252D724: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D728: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D72C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8252D730: 4BF6E301  bl 0x8249ba30
	ctx.lr = 0x8252D734;
	sub_8249BA30(ctx, base);
	// 8252D734: 357C0014  addic. r11, r28, 0x14
	ctx.xer.ca = (ctx.r[28].u32 > (!(20 as u32)));
	ctx.r[11].s64 = ctx.r[28].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252D738: 389C001C  addi r4, r28, 0x1c
	ctx.r[4].s64 = ctx.r[28].s64 + 28;
	// 8252D73C: 40820008  bne 0x8252d744
	if !ctx.cr[0].eq {
	pc = 0x8252D744; continue 'dispatch;
	}
	// 8252D740: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8252D744; continue 'dispatch;
            }
            0x8252D744 => {
    //   block [0x8252D744..0x8252D75C)
	// 8252D744: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252D748: 4BF7C189  bl 0x824a98d0
	ctx.lr = 0x8252D74C;
	sub_824A98D0(ctx, base);
	// 8252D74C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8252D750: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 8252D754: 409A0008  bne cr6, 0x8252d75c
	if !ctx.cr[6].eq {
	pc = 0x8252D75C; continue 'dispatch;
	}
	// 8252D758: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8252D75C; continue 'dispatch;
            }
            0x8252D75C => {
    //   block [0x8252D75C..0x8252D770)
	// 8252D75C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252D760: 4BF7C259  bl 0x824a99b8
	ctx.lr = 0x8252D764;
	sub_824A99B8(ctx, base);
	// 8252D764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252D768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252D76C: 48007998  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252D770 size=160
    let mut pc: u32 = 0x8252D770;
    'dispatch: loop {
        match pc {
            0x8252D770 => {
    //   block [0x8252D770..0x8252D7C4)
	// 8252D770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252D778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252D77C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252D784: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D788: 396B602C  addi r11, r11, 0x602c
	ctx.r[11].s64 = ctx.r[11].s64 + 24620;
	// 8252D78C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252D790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252D794: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252D798: 419A0038  beq cr6, 0x8252d7d0
	if ctx.cr[6].eq {
	pc = 0x8252D7D0; continue 'dispatch;
	}
	// 8252D79C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252D7A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252D7A4: 419A0024  beq cr6, 0x8252d7c8
	if ctx.cr[6].eq {
	pc = 0x8252D7C8; continue 'dispatch;
	}
	// 8252D7A8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252D7AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252D7B0: 419A0018  beq cr6, 0x8252d7c8
	if ctx.cr[6].eq {
	pc = 0x8252D7C8; continue 'dispatch;
	}
	// 8252D7B4: 357F0014  addic. r11, r31, 0x14
	ctx.xer.ca = (ctx.r[31].u32 > (!(20 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252D7B8: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 8252D7BC: 40820008  bne 0x8252d7c4
	if !ctx.cr[0].eq {
	pc = 0x8252D7C4; continue 'dispatch;
	}
	// 8252D7C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	pc = 0x8252D7C4; continue 'dispatch;
            }
            0x8252D7C4 => {
    //   block [0x8252D7C4..0x8252D7C8)
	// 8252D7C4: 4BF7C10D  bl 0x824a98d0
	ctx.lr = 0x8252D7C8;
	sub_824A98D0(ctx, base);
	pc = 0x8252D7C8; continue 'dispatch;
            }
            0x8252D7C8 => {
    //   block [0x8252D7C8..0x8252D7D0)
	// 8252D7C8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252D7CC: 4BF6E285  bl 0x8249ba50
	ctx.lr = 0x8252D7D0;
	sub_8249BA50(ctx, base);
	pc = 0x8252D7D0; continue 'dispatch;
            }
            0x8252D7D0 => {
    //   block [0x8252D7D0..0x8252D810)
	// 8252D7D0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252D7D4: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 8252D7D8: 392A600C  addi r9, r10, 0x600c
	ctx.r[9].s64 = ctx.r[10].s64 + 24588;
	// 8252D7DC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252D7E0: 390A0CE4  addi r8, r10, 0xce4
	ctx.r[8].s64 = ctx.r[10].s64 + 3300;
	// 8252D7E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8252D7E8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8252D7EC: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 8252D7F0: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8252D7F4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252D7F8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252D7FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252D800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252D804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252D808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252D80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252D810 size=184
    let mut pc: u32 = 0x8252D810;
    'dispatch: loop {
        match pc {
            0x8252D810 => {
    //   block [0x8252D810..0x8252D8C8)
	// 8252D810: 81240048  lwz r9, 0x48(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252D814: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 8252D818: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 8252D81C: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 8252D820: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252D8C8 size=96
    let mut pc: u32 = 0x8252D8C8;
    'dispatch: loop {
        match pc {
            0x8252D8C8 => {
    //   block [0x8252D8C8..0x8252D928)
	// 8252D8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252D8D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D8D4: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252D8D8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252D8DC: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 8252D8E0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252D8E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8252D8E8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8252D8EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252D8F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8252D8F4: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252D928 size=72
    let mut pc: u32 = 0x8252D928;
    'dispatch: loop {
        match pc {
            0x8252D928 => {
    //   block [0x8252D928..0x8252D970)
	// 8252D928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252D930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252D934: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252D93C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8252D940: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D944: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252D948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252D94C: 4E800421  bctrl
	ctx.lr = 0x8252D950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252D950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252D954: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252D958: 4BF8CE99  bl 0x824ba7f0
	ctx.lr = 0x8252D95C;
	sub_824BA7F0(ctx, base);
	// 8252D95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252D960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252D964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252D968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252D96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252D970 size=852
    let mut pc: u32 = 0x8252D970;
    'dispatch: loop {
        match pc {
            0x8252D970 => {
    //   block [0x8252D970..0x8252DC50)
	// 8252D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252D974: 4800770D  bl 0x82535080
	ctx.lr = 0x8252D978;
	sub_82535080(ctx, base);
	// 8252D978: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 8252D97C: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 8252D980: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 8252D984: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252D988: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8252D98C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8252D990: 7CCE3378  mr r14, r6
	ctx.r[14].u64 = ctx.r[6].u64;
	// 8252D994: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8252D998: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252D99C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252D9A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252D9A4: 4E800421  bctrl
	ctx.lr = 0x8252D9A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252D9A8: 8179001C  lwz r11, 0x1c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252D9AC: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8252D9B0: 3AF9001C  addi r23, r25, 0x1c
	ctx.r[23].s64 = ctx.r[25].s64 + 28;
	// 8252D9B4: 7ED2B378  mr r18, r22
	ctx.r[18].u64 = ctx.r[22].u64;
	// 8252D9B8: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252D9BC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252D9C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252D9C4: 409902EC  ble cr6, 0x8252dcb0
	if !ctx.cr[6].gt {
	pc = 0x8252DCB0; continue 'dispatch;
	}
	// 8252D9C8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252D9CC: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8252D9D0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8252D9D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8252D9D8: 396BFA60  addi r11, r11, -0x5a0
	ctx.r[11].s64 = ctx.r[11].s64 + -1440;
	// 8252D9DC: 3A190048  addi r16, r25, 0x48
	ctx.r[16].s64 = ctx.r[25].s64 + 72;
	// 8252D9E0: C3A82074  lfs f29, 0x2074(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8308 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8252D9E4: 39F90034  addi r15, r25, 0x34
	ctx.r[15].s64 = ctx.r[25].s64 + 52;
	// 8252D9E8: C3C91FF8  lfs f30, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8252D9EC: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 8252D9F0: C3EA1850  lfs f31, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8252D9F4: 3A20FFFF  li r17, -1
	ctx.r[17].s64 = -1;
	// 8252D9F8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8252D9FC: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 8252DA00: 3AA00060  li r21, 0x60
	ctx.r[21].s64 = 96;
	// 8252DA04: 3A6000D0  li r19, 0xd0
	ctx.r[19].s64 = 208;
	// 8252DA08: 57C9083C  slwi r9, r30, 1
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8252DA0C: 81500000  lwz r10, 0(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DA10: 57CB3032  slwi r11, r30, 6
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252DA14: 811A0000  lwz r8, 0(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DA18: 7D3E4A14  add r9, r30, r9
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 8252DA1C: D3E10080  stfs f31, 0x80(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8252DA20: 7FEB7214  add r31, r11, r14
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[14].u64;
	// 8252DA24: 92210084  stw r17, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[17].u32 ) };
	// 8252DA28: 552B3032  slwi r11, r9, 6
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252DA2C: 92C100B0  stw r22, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[22].u32 ) };
	// 8252DA30: 92810090  stw r20, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[20].u32 ) };
	// 8252DA34: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8252DA38: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252DA3C: 81280024  lwz r9, 0x24(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252DA40: 92C100C0  stw r22, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[22].u32 ) };
	// 8252DA44: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8252DA48: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8252DA4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252DA50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8252DA54: 4E800421  bctrl
	ctx.lr = 0x8252DA58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252DA58: 814F0000  lwz r10, 0(r15)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DA5C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252DA60: 81170000  lwz r8, 0(r23)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DA64: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8252DA68: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8252DA6C: 7D3E4A14  add r9, r30, r9
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 8252DA70: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252DA74: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252DA78: 80E8008C  lwz r7, 0x8c(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(140 as u32) ) } as u64;
	// 8252DA7C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252DA80: 810100C0  lwz r8, 0xc0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 8252DA84: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252DA88: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8252DA8C: C1AB0020  lfs f13, 0x20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DA90: 7C0A3C2E  lfsx f0, r10, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DA94: 419A01BC  beq cr6, 0x8252dc50
	if ctx.cr[6].eq {
	pc = 0x8252DC50; continue 'dispatch;
	}
	// 8252DA98: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8252DA9C: 3B1F0010  addi r24, r31, 0x10
	ctx.r[24].s64 = ctx.r[31].s64 + 16;
	// 8252DAA0: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
            }
            0x8252DC50 => {
    //   block [0x8252DC50..0x8252DCB0)
	// 8252DC50: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8252DC54: D1BF002C  stfs f13, 0x2c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8252DC58: D3DF0030  stfs f30, 0x30(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8252DC5C: 92DF0024  stw r22, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[22].u32 ) };
	// 8252DC60: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x8252DCB0; continue 'dispatch;
            }
            0x8252DCB0 => {
    //   block [0x8252DCB0..0x8252DCC4)
	// 8252DCB0: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8252DCB4: CBA1FF50  lfd f29, -0xb0(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-176 as u32) ) };
	// 8252DCB8: CBC1FF58  lfd f30, -0xa8(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	// 8252DCBC: CBE1FF60  lfd f31, -0xa0(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	// 8252DCC0: 48007410  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252DCC8 size=112
    let mut pc: u32 = 0x8252DCC8;
    'dispatch: loop {
        match pc {
            0x8252DCC8 => {
    //   block [0x8252DCC8..0x8252DD04)
	// 8252DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252DCD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252DCD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252DCD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252DCDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252DCE0: 83C30010  lwz r30, 0x10(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252DCE4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252DCE8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252DCEC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252DCF0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252DCF4: 409A0010  bne cr6, 0x8252dd04
	if !ctx.cr[6].eq {
	pc = 0x8252DD04; continue 'dispatch;
	}
	// 8252DCF8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8252DCFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252DD00: 4BF40651  bl 0x8246e350
	ctx.lr = 0x8252DD04;
	sub_8246E350(ctx, base);
	pc = 0x8252DD04; continue 'dispatch;
            }
            0x8252DD04 => {
    //   block [0x8252DD04..0x8252DD38)
	// 8252DD04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252DD08: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DD0C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252DD10: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 8252DD14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252DD18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252DD1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8252DD20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252DD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252DD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252DD2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252DD30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252DD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DD38 size=76
    let mut pc: u32 = 0x8252DD38;
    'dispatch: loop {
        match pc {
            0x8252DD38 => {
    //   block [0x8252DD38..0x8252DD60)
	// 8252DD38: 896500CC  lbz r11, 0xcc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(204 as u32) ) } as u64;
	// 8252DD3C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252DD40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252DD44: 419A001C  beq cr6, 0x8252dd60
	if ctx.cr[6].eq {
	pc = 0x8252DD60; continue 'dispatch;
	}
	// 8252DD48: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252DD4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252DD50: 419A0010  beq cr6, 0x8252dd60
	if ctx.cr[6].eq {
	pc = 0x8252DD60; continue 'dispatch;
	}
	// 8252DD54: C006000C  lfs f0, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DD58: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252DD5C: 48000008  b 0x8252dd64
	pc = 0x8252DD64; continue 'dispatch;
            }
            0x8252DD60 => {
    //   block [0x8252DD60..0x8252DD64)
	// 8252DD60: C006000C  lfs f0, 0xc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x8252DD64; continue 'dispatch;
            }
            0x8252DD64 => {
    //   block [0x8252DD64..0x8252DD84)
	// 8252DD64: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252DD68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252DD6C: 409A0018  bne cr6, 0x8252dd84
	if !ctx.cr[6].eq {
		sub_8252DD84(ctx, base);
		return;
	}
	// 8252DD70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252DD74: 419A0010  beq cr6, 0x8252dd84
	if ctx.cr[6].eq {
		sub_8252DD84(ctx, base);
		return;
	}
	// 8252DD78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8252DD7C: C02B1850  lfs f1, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252DD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DD84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DD84 size=32
    let mut pc: u32 = 0x8252DD84;
    'dispatch: loop {
        match pc {
            0x8252DD84 => {
    //   block [0x8252DD84..0x8252DDA4)
	// 8252DD84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252DD88: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252DD8C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8252DD90: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 8252DD94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252DD98: C1AB2074  lfs f13, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DD9C: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8252DDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DDA8 size=48
    let mut pc: u32 = 0x8252DDA8;
    'dispatch: loop {
        match pc {
            0x8252DDA8 => {
    //   block [0x8252DDA8..0x8252DDD8)
	// 8252DDA8: 896500CC  lbz r11, 0xcc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(204 as u32) ) } as u64;
	// 8252DDAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252DDB0: 419A0028  beq cr6, 0x8252ddd8
	if ctx.cr[6].eq {
		sub_8252DDD8(ctx, base);
		return;
	}
	// 8252DDB4: 89630014  lbz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252DDB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252DDBC: 419A001C  beq cr6, 0x8252ddd8
	if ctx.cr[6].eq {
		sub_8252DDD8(ctx, base);
		return;
	}
	// 8252DDC0: C006000C  lfs f0, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DDC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252DDC8: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252DDCC: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DDD0: FC20682E  fsel f1, f0, f0, f13
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8252DDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DDD8 size=20
    let mut pc: u32 = 0x8252DDD8;
    'dispatch: loop {
        match pc {
            0x8252DDD8 => {
    //   block [0x8252DDD8..0x8252DDEC)
	// 8252DDD8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252DDDC: C006000C  lfs f0, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DDE0: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DDE4: FC20682E  fsel f1, f0, f0, f13
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8252DDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252DDF0 size=204
    let mut pc: u32 = 0x8252DDF0;
    'dispatch: loop {
        match pc {
            0x8252DDF0 => {
    //   block [0x8252DDF0..0x8252DEBC)
	// 8252DDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252DDF4: 480072C5  bl 0x825350b8
	ctx.lr = 0x8252DDF8;
	sub_82535080(ctx, base);
	// 8252DDF8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8252DDFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252DE00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8252DE04: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8252DE08: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8252DE0C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8252DE10: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8252DE14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DE18: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252DE1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252DE20: 4E800421  bctrl
	ctx.lr = 0x8252DE24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252DE24: D03F0000  stfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252DE28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DE2C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8252DE30: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8252DE34: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8252DE38: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8252DE3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252DE40: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252DE44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252DE48: 4E800421  bctrl
	ctx.lr = 0x8252DE4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252DE4C: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8252DE50: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DE54: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8252DE58: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8252DE5C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8252DE60: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8252DE64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252DE68: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252DE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252DE70: 4E800421  bctrl
	ctx.lr = 0x8252DE74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252DE74: D03F0008  stfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252DE78: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252DE7C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8252DE80: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8252DE84: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8252DE88: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8252DE8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252DE90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252DE94: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 8252DE98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DE9C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252DEA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252DEA4: 4E800421  bctrl
	ctx.lr = 0x8252DEA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252DEA8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252DEAC: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 8252DEB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8252DEB4: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8252DEB8: 48007250  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DEC0 size=52
    let mut pc: u32 = 0x8252DEC0;
    'dispatch: loop {
        match pc {
            0x8252DEC0 => {
    //   block [0x8252DEC0..0x8252DEF4)
	// 8252DEC0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252DEC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252DEC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8252DECC: 396B6358  addi r11, r11, 0x6358
	ctx.r[11].s64 = ctx.r[11].s64 + 25432;
	// 8252DED0: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252DED8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252DEDC: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8252DEE0: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252DEE4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252DEE8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8252DEEC: 99230014  stb r9, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 8252DEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DEF8 size=32
    let mut pc: u32 = 0x8252DEF8;
    'dispatch: loop {
        match pc {
            0x8252DEF8 => {
    //   block [0x8252DEF8..0x8252DF18)
	// 8252DEF8: C0060008  lfs f0, 8(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DEFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252DF00: FD400210  fabs f10, f0
	ctx.f[10].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 8252DF04: C1230010  lfs f9, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8252DF08: FF0A4800  fcmpu cr6, f10, f9
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[9].f64);
	// 8252DF0C: 4098000C  bge cr6, 0x8252df18
	if !ctx.cr[6].lt {
		sub_8252DF18(ctx, base);
		return;
	}
	// 8252DF10: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252DF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DF18 size=68
    let mut pc: u32 = 0x8252DF18;
    'dispatch: loop {
        match pc {
            0x8252DF18 => {
    //   block [0x8252DF18..0x8252DF34)
	// 8252DF18: C1AB1FF8  lfs f13, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DF1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8252DF20: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252DF24: C18B1850  lfs f12, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252DF28: 4099000C  ble cr6, 0x8252df34
	if !ctx.cr[6].gt {
	pc = 0x8252DF34; continue 'dispatch;
	}
	// 8252DF2C: FD606090  fmr f11, f12
	ctx.f[11].f64 = ctx.f[12].f64;
	// 8252DF30: 4800000C  b 0x8252df3c
	pc = 0x8252DF3C; continue 'dispatch;
            }
            0x8252DF34 => {
    //   block [0x8252DF34..0x8252DF3C)
	// 8252DF34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252DF38: C16B2074  lfs f11, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	pc = 0x8252DF3C; continue 'dispatch;
            }
            0x8252DF3C => {
    //   block [0x8252DF3C..0x8252DF5C)
	// 8252DF3C: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DF40: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 8252DF44: 40980018  bge cr6, 0x8252df5c
	if !ctx.cr[6].lt {
		sub_8252DF5C(ctx, base);
		return;
	}
	// 8252DF48: EC0A4828  fsubs f0, f10, f9
	ctx.f[0].f64 = (((ctx.f[10].f64 - ctx.f[9].f64) as f32) as f64);
	// 8252DF4C: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DF50: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8252DF54: EC2002F2  fmuls f1, f0, f11
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 8252DF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DF5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252DF5C size=52
    let mut pc: u32 = 0x8252DF5C;
    'dispatch: loop {
        match pc {
            0x8252DF5C => {
    //   block [0x8252DF5C..0x8252DF90)
	// 8252DF5C: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252DF60: C1A30010  lfs f13, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252DF64: ED4A0028  fsubs f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252DF68: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252DF6C: C123000C  lfs f9, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8252DF70: ED0C6828  fsubs f8, f12, f13
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252DF74: EDA90032  fmuls f13, f9, f0
	ctx.f[13].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252DF78: EC080028  fsubs f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252DF7C: ED8C6828  fsubs f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252DF80: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8252DF84: EC006ABA  fmadds f0, f0, f10, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64);
	// 8252DF88: EC2002F2  fmuls f1, f0, f11
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 8252DF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252DF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252DF90 size=548
    let mut pc: u32 = 0x8252DF90;
    'dispatch: loop {
        match pc {
            0x8252DF90 => {
    //   block [0x8252DF90..0x8252DFB0)
	// 8252DF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252DF94: 48007119  bl 0x825350ac
	ctx.lr = 0x8252DF98;
	sub_82535080(ctx, base);
	// 8252DF98: 89640014  lbz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252DF9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252DFA0: 409A0010  bne cr6, 0x8252dfb0
	if !ctx.cr[6].eq {
	pc = 0x8252DFB0; continue 'dispatch;
	}
	// 8252DFA4: 89670011  lbz r11, 0x11(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(17 as u32) ) } as u64;
	// 8252DFA8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8252DFAC: 48007150  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            0x8252DFB0 => {
    //   block [0x8252DFB0..0x8252E1B4)
	// 8252DFB0: 81260018  lwz r9, 0x18(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252DFB4: 3961FF90  addi r11, r1, -0x70
	ctx.r[11].s64 = ctx.r[1].s64 + -112;
	// 8252DFB8: 3901FF80  addi r8, r1, -0x80
	ctx.r[8].s64 = ctx.r[1].s64 + -128;
	// 8252DFBC: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	// 8252DFC0: 38A1FF80  addi r5, r1, -0x80
	ctx.r[5].s64 = ctx.r[1].s64 + -128;
	// 8252DFC4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8252DFC8: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8252DFCC: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8252DFD0: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8252DFD4: F88B0000  std r4, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 8252DFD8: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8252DFDC: 3961FF90  addi r11, r1, -0x70
	ctx.r[11].s64 = ctx.r[1].s64 + -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252E1B8 size=140
    let mut pc: u32 = 0x8252E1B8;
    'dispatch: loop {
        match pc {
            0x8252E1B8 => {
    //   block [0x8252E1B8..0x8252E244)
	// 8252E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252E1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252E1C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252E1C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252E1C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E1CC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252E1D0: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8252E1D4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8252E1D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252E1DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252E1E0: 4BF35E59  bl 0x82464038
	ctx.lr = 0x8252E1E4;
	sub_82464038(ctx, base);
	// 8252E1E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252E1E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8252E1EC: 396B6344  addi r11, r11, 0x6344
	ctx.r[11].s64 = ctx.r[11].s64 + 25412;
	// 8252E1F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8252E1F4: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8252E1F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252E1FC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8252E200: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252E204: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8252E208: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252E20C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252E210: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E214: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252E218: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E21C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8252E220: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252E224: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8252E228: 897F0011  lbz r11, 0x11(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(17 as u32) ) } as u64;
	// 8252E22C: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8252E230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252E234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252E238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252E23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252E240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E248 size=64
    let mut pc: u32 = 0x8252E248;
    'dispatch: loop {
        match pc {
            0x8252E248 => {
    //   block [0x8252E248..0x8252E288)
	// 8252E248: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252E24C: C00B6D4C  lfs f0, 0x6d4c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27980 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E250: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E254: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E258: C00B278C  lfs f0, 0x278c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E25C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E260: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E264: EDA01024  fdivs f13, f0, f2
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[2].f64) as f32) as f64;
	// 8252E268: C00B2054  lfs f0, 0x2054(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E26C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E270: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E274: C00B2200  lfs f0, 0x2200(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8704 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E278: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E27C: EC030024  fdivs f0, f3, f0
	ctx.f[0].f64 = ((ctx.f[3].f64 / ctx.f[0].f64) as f32) as f64;
	// 8252E280: EC202024  fdivs f1, f0, f4
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[4].f64) as f32) as f64;
	// 8252E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252E288 size=100
    let mut pc: u32 = 0x8252E288;
    'dispatch: loop {
        match pc {
            0x8252E288 => {
    //   block [0x8252E288..0x8252E2C0)
	// 8252E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252E290: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8252E294: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252E298: 8965000F  lbz r11, 0xf(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(15 as u32) ) } as u64;
	// 8252E29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252E2A0: 419A0020  beq cr6, 0x8252e2c0
	if ctx.cr[6].eq {
	pc = 0x8252E2C0; continue 'dispatch;
	}
	// 8252E2A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E2A8: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252E2AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252E2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252E2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252E2B8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252E2BC: 4E800020  blr
	return;
            }
            0x8252E2C0 => {
    //   block [0x8252E2C0..0x8252E2EC)
	// 8252E2C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E2C4: C3E400B4  lfs f31, 0xb4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(180 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8252E2C8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E2CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252E2D0: 4E800421  bctrl
	ctx.lr = 0x8252E2D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252E2D4: EC2107F2  fmuls f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 8252E2D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252E2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252E2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252E2E4: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252E2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252E2F0 size=20
    let mut pc: u32 = 0x8252E2F0;
    'dispatch: loop {
        match pc {
            0x8252E2F0 => {
    //   block [0x8252E2F0..0x8252E304)
	// 8252E2F0: 896500B0  lbz r11, 0xb0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(176 as u32) ) } as u64;
	// 8252E2F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252E2F8: 409A000C  bne cr6, 0x8252e304
	if !ctx.cr[6].eq {
		sub_8252E304(ctx, base);
		return;
	}
	// 8252E2FC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8252E300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E304(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252E304 size=32
    let mut pc: u32 = 0x8252E304;
    'dispatch: loop {
        match pc {
            0x8252E304 => {
    //   block [0x8252E304..0x8252E31C)
	// 8252E304: 8966000E  lbz r11, 0xe(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as u64;
	// 8252E308: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252E30C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252E310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252E314: 41990008  bgt cr6, 0x8252e31c
	if ctx.cr[6].gt {
	pc = 0x8252E31C; continue 'dispatch;
	}
	// 8252E318: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x8252E31C; continue 'dispatch;
            }
            0x8252E31C => {
    //   block [0x8252E31C..0x8252E324)
	// 8252E31C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8252E320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E328 size=32
    let mut pc: u32 = 0x8252E328;
    'dispatch: loop {
        match pc {
            0x8252E328 => {
    //   block [0x8252E328..0x8252E348)
	// 8252E328: 8965000D  lbz r11, 0xd(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(13 as u32) ) } as u64;
	// 8252E32C: C1A30010  lfs f13, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E330: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252E334: 419A0014  beq cr6, 0x8252e348
	if ctx.cr[6].eq {
		sub_8252E348(ctx, base);
		return;
	}
	// 8252E338: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E33C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252E340: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E348 size=28
    let mut pc: u32 = 0x8252E348;
    'dispatch: loop {
        match pc {
            0x8252E348 => {
    //   block [0x8252E348..0x8252E364)
	// 8252E348: 8965000E  lbz r11, 0xe(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(14 as u32) ) } as u64;
	// 8252E34C: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E350: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252E354: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252E358: 7C0B542E  lfsx f0, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E35C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252E368 size=240
    let mut pc: u32 = 0x8252E368;
    'dispatch: loop {
        match pc {
            0x8252E368 => {
    //   block [0x8252E368..0x8252E400)
	// 8252E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252E36C: 48006D51  bl 0x825350bc
	ctx.lr = 0x8252E370;
	sub_82535080(ctx, base);
	// 8252E370: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8252E374: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252E378: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252E37C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8252E380: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8252E384: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8252E388: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252E38C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252E390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E394: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252E398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252E39C: 4E800421  bctrl
	ctx.lr = 0x8252E3A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252E3A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8252E3A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8252E3A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252E3AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252E3B0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E3B4: 997E000D  stb r11, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 8252E3B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E3BC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252E3C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252E3C4: 4E800421  bctrl
	ctx.lr = 0x8252E3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252E3C8: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8252E3CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E3D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8252E3D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252E3D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252E3DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252E3E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252E3E4: 4E800421  bctrl
	ctx.lr = 0x8252E3E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252E3E8: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252E3EC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8252E3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252E3F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252E3F8: 40990034  ble cr6, 0x8252e42c
	if !ctx.cr[6].gt {
	pc = 0x8252E42C; continue 'dispatch;
	}
	// 8252E3FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
            }
            0x8252E400 => {
    //   block [0x8252E400..0x8252E42C)
	// 8252E400: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252E404: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E408: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E40C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8252E410: 7DA95C2E  lfsx f13, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E414: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E418: 7C085D2E  stfsx f0, r8, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 8252E41C: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8252E420: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8252E424: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252E428: 4198FFD8  blt cr6, 0x8252e400
	if ctx.cr[6].lt {
	pc = 0x8252E400; continue 'dispatch;
	}
	pc = 0x8252E42C; continue 'dispatch;
            }
            0x8252E42C => {
    //   block [0x8252E42C..0x8252E458)
	// 8252E42C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E430: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8252E434: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252E438: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8252E43C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252E440: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E444: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252E448: 4E800421  bctrl
	ctx.lr = 0x8252E44C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252E44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252E450: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8252E454: 48006CB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252E458 size=296
    let mut pc: u32 = 0x8252E458;
    'dispatch: loop {
        match pc {
            0x8252E458 => {
    //   block [0x8252E458..0x8252E4B0)
	// 8252E458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252E45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252E460: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8252E464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252E468: 8144001C  lwz r10, 0x1c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E46C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E470: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E474: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8252E478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252E47C: 7D470774  extsb r7, r10
	ctx.r[7].s64 = ctx.r[10].s8 as i64;
	// 8252E480: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252E484: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 8252E488: C00A6D50  lfs f0, 0x6d50(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27984 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E48C: 41980078  blt cr6, 0x8252e504
	if ctx.cr[6].lt {
	pc = 0x8252E504; continue 'dispatch;
	}
	// 8252E490: 3947FFFC  addi r10, r7, -4
	ctx.r[10].s64 = ctx.r[7].s64 + -4;
	// 8252E494: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252E498: 80C40048  lwz r6, 0x48(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252E49C: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252E4A0: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 8252E4A4: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 8252E4A8: 39460160  addi r10, r6, 0x160
	ctx.r[10].s64 = ctx.r[6].s64 + 352;
	// 8252E4AC: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x8252E4B0; continue 'dispatch;
            }
            0x8252E4B0 => {
    //   block [0x8252E4B0..0x8252E504)
	// 8252E4B0: C1AAFF40  lfs f13, -0xc0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E4B4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8252E4B8: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E4BC: C189FFF8  lfs f12, -8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252E4C0: C16A0000  lfs f11, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8252E4C4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8252E4C8: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E4CC: C149FFFC  lfs f10, -4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252E4D0: C12A00C0  lfs f9, 0xc0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8252E4D4: ED290032  fmuls f9, f9, f0
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E4D8: C1090000  lfs f8, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8252E4DC: C0EA0180  lfs f7, 0x180(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(384 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8252E4E0: 394A0300  addi r10, r10, 0x300
	ctx.r[10].s64 = ctx.r[10].s64 + 768;
	// 8252E4E4: ECE70032  fmuls f7, f7, f0
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E4E8: C0C90004  lfs f6, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8252E4EC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 8252E4F0: EDADFB3A  fmadds f13, f13, f12, f31
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 + ctx.f[31].f64) as f32) as f64);
	// 8252E4F4: EDAB6ABA  fmadds f13, f11, f10, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64);
	// 8252E4F8: EDA96A3A  fmadds f13, f9, f8, f13
	ctx.f[13].f64 = (((ctx.f[9].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 8252E4FC: EFE769BA  fmadds f31, f7, f6, f13
	ctx.f[31].f64 = (((ctx.f[7].f64 * ctx.f[6].f64 + ctx.f[13].f64) as f32) as f64);
	// 8252E500: 409AFFB0  bne cr6, 0x8252e4b0
	if !ctx.cr[6].eq {
	pc = 0x8252E4B0; continue 'dispatch;
	}
	pc = 0x8252E504; continue 'dispatch;
            }
            0x8252E504 => {
    //   block [0x8252E504..0x8252E534)
	// 8252E504: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8252E508: 40980050  bge cr6, 0x8252e558
	if !ctx.cr[6].lt {
	pc = 0x8252E558; continue 'dispatch;
	}
	// 8252E50C: 7D4B3850  subf r10, r11, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 8252E510: 81040048  lwz r8, 0x48(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252E514: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8252E518: 80C30028  lwz r6, 0x28(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252E51C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8252E520: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8252E524: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 8252E528: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252E52C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8252E530: 396B00A0  addi r11, r11, 0xa0
	ctx.r[11].s64 = ctx.r[11].s64 + 160;
	pc = 0x8252E534; continue 'dispatch;
            }
            0x8252E534 => {
    //   block [0x8252E534..0x8252E558)
	// 8252E534: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E538: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8252E53C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E540: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252E544: 396B00C0  addi r11, r11, 0xc0
	ctx.r[11].s64 = ctx.r[11].s64 + 192;
	// 8252E548: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8252E54C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252E550: EFEDFB3A  fmadds f31, f13, f12, f31
	ctx.f[31].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 + ctx.f[31].f64) as f32) as f64);
	// 8252E554: 409AFFE0  bne cr6, 0x8252e534
	if !ctx.cr[6].eq {
	pc = 0x8252E534; continue 'dispatch;
	}
	pc = 0x8252E558; continue 'dispatch;
            }
            0x8252E558 => {
    //   block [0x8252E558..0x8252E580)
	// 8252E558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E55C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252E564: 4E800421  bctrl
	ctx.lr = 0x8252E568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252E568: EC2107F2  fmuls f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 8252E56C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252E570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252E574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252E578: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252E57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E580 size=176
    let mut pc: u32 = 0x8252E580;
    'dispatch: loop {
        match pc {
            0x8252E580 => {
    //   block [0x8252E580..0x8252E5B0)
	// 8252E580: C0060010  lfs f0, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E584: 8966000F  lbz r11, 0xf(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(15 as u32) ) } as u64;
	// 8252E588: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8252E58C: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252E590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252E594: 419A001C  beq cr6, 0x8252e5b0
	if ctx.cr[6].eq {
	pc = 0x8252E5B0; continue 'dispatch;
	}
	// 8252E598: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E59C: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E5A0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252E5A4: 4199000C  bgt cr6, 0x8252e5b0
	if ctx.cr[6].gt {
	pc = 0x8252E5B0; continue 'dispatch;
	}
	// 8252E5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252E5AC: 9966000F  stb r11, 0xf(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	pc = 0x8252E5B0; continue 'dispatch;
            }
            0x8252E5B0 => {
    //   block [0x8252E5B0..0x8252E5F4)
	// 8252E5B0: 8966000D  lbz r11, 0xd(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(13 as u32) ) } as u64;
	// 8252E5B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252E5B8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8252E5BC: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E5C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252E5C4: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E5C8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252E5CC: 40980028  bge cr6, 0x8252e5f4
	if !ctx.cr[6].lt {
	pc = 0x8252E5F4; continue 'dispatch;
	}
	// 8252E5D0: 8966000E  lbz r11, 0xe(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as u64;
	// 8252E5D4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252E5D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252E5DC: 40990018  ble cr6, 0x8252e5f4
	if !ctx.cr[6].gt {
	pc = 0x8252E5F4; continue 'dispatch;
	}
	// 8252E5E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252E5E4: 9966000E  stb r11, 0xe(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 8252E5E8: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E5EC: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252E5F0: 9946000F  stb r10, 0xf(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	pc = 0x8252E5F4; continue 'dispatch;
            }
            0x8252E5F4 => {
    //   block [0x8252E5F4..0x8252E630)
	// 8252E5F4: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E5F8: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E5FC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252E600: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8252E604: 8966000E  lbz r11, 0xe(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as u64;
	// 8252E608: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E60C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252E610: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252E614: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252E618: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
	// 8252E61C: 9966000E  stb r11, 0xe(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 8252E620: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E624: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252E628: 9946000F  stb r10, 0xf(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 8252E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E630 size=84
    let mut pc: u32 = 0x8252E630;
    'dispatch: loop {
        match pc {
            0x8252E630 => {
    //   block [0x8252E630..0x8252E684)
	// 8252E630: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E634: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8252E638: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8252E63C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E640: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252E644: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8252E648: 394B6484  addi r10, r11, 0x6484
	ctx.r[10].s64 = ctx.r[11].s64 + 25732;
	// 8252E64C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252E650: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8252E654: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8252E658: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8252E65C: 91230024  stw r9, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8252E660: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8252E664: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8252E668: 91230030  stw r9, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 8252E66C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252E670: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8252E674: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252E678: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8252E67C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8252E680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252E688 size=240
    let mut pc: u32 = 0x8252E688;
    'dispatch: loop {
        match pc {
            0x8252E688 => {
    //   block [0x8252E688..0x8252E6DC)
	// 8252E688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252E68C: 48006A2D  bl 0x825350b8
	ctx.lr = 0x8252E690;
	sub_82535080(ctx, base);
	// 8252E690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252E694: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8252E698: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8252E69C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8252E6A0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8252E6A4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252E6A8: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 8252E6AC: 4BF6CE8D  bl 0x8249b538
	ctx.lr = 0x8252E6B0;
	sub_8249B538(ctx, base);
	// 8252E6B0: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8252E6B8: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E6BC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252E6C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252E6C4: 409900AC  ble cr6, 0x8252e770
	if !ctx.cr[6].gt {
	pc = 0x8252E770; continue 'dispatch;
	}
	// 8252E6C8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8252E6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8252E6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252E6D4: 397E002C  addi r11, r30, 0x2c
	ctx.r[11].s64 = ctx.r[30].s64 + 44;
	// 8252E6D8: C1681FF8  lfs f11, 0x1ff8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	pc = 0x8252E6DC; continue 'dispatch;
            }
            0x8252E6DC => {
    //   block [0x8252E6DC..0x8252E734)
	// 8252E6DC: 810BFFF8  lwz r8, -8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252E6E0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8252E6E4: 419A0064  beq cr6, 0x8252e748
	if ctx.cr[6].eq {
	pc = 0x8252E748; continue 'dispatch;
	}
	// 8252E6E8: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E6EC: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E6F0: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252E6F4: 811C0014  lwz r8, 0x14(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252E6F8: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 8252E6FC: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E700: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8252E704: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8252E708: C1470020  lfs f10, 0x20(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252E70C: EDAA6828  fsubs f13, f10, f13
	ctx.f[13].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252E710: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252E714: C1880000  lfs f12, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252E718: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252E71C: 40980018  bge cr6, 0x8252e734
	if !ctx.cr[6].lt {
	pc = 0x8252E734; continue 'dispatch;
	}
	// 8252E720: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E724: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8252E728: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8252E72C: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252E730: 4800001C  b 0x8252e74c
	pc = 0x8252E74C; continue 'dispatch;
            }
            0x8252E734 => {
    //   block [0x8252E734..0x8252E748)
	// 8252E734: C1A80008  lfs f13, 8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E738: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8252E73C: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8252E740: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252E744: 48000008  b 0x8252e74c
	pc = 0x8252E74C; continue 'dispatch;
            }
            0x8252E748 => {
    //   block [0x8252E748..0x8252E74C)
	// 8252E748: D17D0000  stfs f11, 0(r29)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	pc = 0x8252E74C; continue 'dispatch;
            }
            0x8252E74C => {
    //   block [0x8252E74C..0x8252E770)
	// 8252E74C: 89050020  lbz r8, 0x20(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E750: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8252E754: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 8252E758: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 8252E75C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 8252E760: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 8252E764: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8252E768: 7F064000  cmpw cr6, r6, r8
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8252E76C: 4198FF70  blt cr6, 0x8252e6dc
	if ctx.cr[6].lt {
	pc = 0x8252E6DC; continue 'dispatch;
	}
	pc = 0x8252E770; continue 'dispatch;
            }
            0x8252E770 => {
    //   block [0x8252E770..0x8252E778)
	// 8252E770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252E774: 48006994  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E778 size=440
    let mut pc: u32 = 0x8252E778;
    'dispatch: loop {
        match pc {
            0x8252E778 => {
    //   block [0x8252E778..0x8252E7DC)
	// 8252E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252E77C: 48006939  bl 0x825350b4
	ctx.lr = 0x8252E780;
	sub_82535080(ctx, base);
	// 8252E780: 8165001C  lwz r11, 0x1c(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E784: C1860004  lfs f12, 4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252E788: 8926000C  lbz r9, 0xc(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252E78C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8252E790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252E794: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8252E798: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E79C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252E7A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252E7A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E7A8: C14B1FF8  lfs f10, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252E7AC: 409900FC  ble cr6, 0x8252e8a8
	if !ctx.cr[6].gt {
	pc = 0x8252E8A8; continue 'dispatch;
	}
	// 8252E7B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8252E7B4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8252E7B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252E7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8252E7C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8252E7C4: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E7C8: 7D2B0034  cntlzw r11, r9
	ctx.r[11].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8252E7CC: ED600824  fdivs f11, f0, f1
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 8252E7D0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8252E7D4: 697F0001  xori r31, r11, 1
	ctx.r[31].u64 = ctx.r[11].u64 ^ 1;
	// 8252E7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8252E7DC; continue 'dispatch;
            }
            0x8252E7DC => {
    //   block [0x8252E7DC..0x8252E81C)
	// 8252E7DC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E7E0: 8387000C  lwz r28, 0xc(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252E7E4: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8252E7E8: 89290008  lbz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E7EC: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8252E7F0: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8252E7F4: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8252E7F8: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 8252E7FC: 7D29F838  and r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[31].u64;
	// 8252E800: 7D3C51AE  stbx r9, r28, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 8252E804: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E808: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8252E80C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E810: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8252E814: 40990008  ble cr6, 0x8252e81c
	if !ctx.cr[6].gt {
	pc = 0x8252E81C; continue 'dispatch;
	}
	// 8252E818: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	pc = 0x8252E81C; continue 'dispatch;
            }
            0x8252E81C => {
    //   block [0x8252E81C..0x8252E874)
	// 8252E81C: 8385001C  lwz r28, 0x1c(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E820: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E824: 81250048  lwz r9, 0x48(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252E828: EDAC0032  fmuls f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E82C: 7F664A14  add r27, r6, r9
	ctx.r[27].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 8252E830: 813C008C  lwz r9, 0x8c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(140 as u32) ) } as u64;
	// 8252E834: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8252E838: C13B00A0  lfs f9, 0xa0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(160 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8252E83C: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E840: C1090004  lfs f8, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8252E844: ED290032  fmuls f9, f9, f0
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E848: ED0802F2  fmuls f8, f8, f11
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[11].f64) as f32) as f64);
	// 8252E84C: ED290232  fmuls f9, f9, f8
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[8].f64) as f32) as f64);
	// 8252E850: FD204850  fneg f9, f9
	ctx.f[9].u64 = ctx.f[9].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252E854: EC090032  fmuls f0, f9, f0
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252E858: FD200210  fabs f9, f0
	ctx.f[9].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 8252E85C: FF096800  fcmpu cr6, f9, f13
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[13].f64);
	// 8252E860: 40990018  ble cr6, 0x8252e878
	if !ctx.cr[6].gt {
	pc = 0x8252E878; continue 'dispatch;
	}
	// 8252E864: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 8252E868: 4099000C  ble cr6, 0x8252e874
	if !ctx.cr[6].gt {
	pc = 0x8252E874; continue 'dispatch;
	}
	// 8252E86C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8252E870: 48000008  b 0x8252e878
	pc = 0x8252E878; continue 'dispatch;
            }
            0x8252E874 => {
    //   block [0x8252E874..0x8252E878)
	// 8252E874: FC006850  fneg f0, f13
	ctx.f[0].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x8252E878; continue 'dispatch;
            }
            0x8252E878 => {
    //   block [0x8252E878..0x8252E8A8)
	// 8252E878: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252E87C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8252E880: 8385001C  lwz r28, 0x1c(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E884: 39080028  addi r8, r8, 0x28
	ctx.r[8].s64 = ctx.r[8].s64 + 40;
	// 8252E888: 38C600C0  addi r6, r6, 0xc0
	ctx.r[6].s64 = ctx.r[6].s64 + 192;
	// 8252E88C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8252E890: 7C044D2E  stfsx f0, r4, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 8252E894: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 8252E898: 893C0020  lbz r9, 0x20(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E89C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8252E8A0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252E8A4: 4198FF38  blt cr6, 0x8252e7dc
	if ctx.cr[6].lt {
	pc = 0x8252E7DC; continue 'dispatch;
	}
	pc = 0x8252E8A8; continue 'dispatch;
            }
            0x8252E8A8 => {
    //   block [0x8252E8A8..0x8252E8E0)
	// 8252E8A8: 7FCB0774  extsb r11, r30
	ctx.r[11].s64 = ctx.r[30].s8 as i64;
	// 8252E8AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252E8B0: 419A0078  beq cr6, 0x8252e928
	if ctx.cr[6].eq {
	pc = 0x8252E928; continue 'dispatch;
	}
	// 8252E8B4: C0070018  lfs f0, 0x18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E8B8: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252E8BC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8252E8C0: 4198005C  blt cr6, 0x8252e91c
	if ctx.cr[6].lt {
	pc = 0x8252E91C; continue 'dispatch;
	}
	// 8252E8C4: 8145001C  lwz r10, 0x1c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252E8CC: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E8D0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8252E8D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252E8D8: 40990054  ble cr6, 0x8252e92c
	if !ctx.cr[6].gt {
	pc = 0x8252E92C; continue 'dispatch;
	}
	// 8252E8DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x8252E8E0; continue 'dispatch;
            }
            0x8252E8E0 => {
    //   block [0x8252E8E0..0x8252E8FC)
	// 8252E8E0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E8E4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8252E8E8: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E8EC: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8252E8F0: 4099000C  ble cr6, 0x8252e8fc
	if !ctx.cr[6].gt {
	pc = 0x8252E8FC; continue 'dispatch;
	}
	// 8252E8F4: 8127000C  lwz r9, 0xc(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252E8F8: 7FA959AE  stbx r29, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	pc = 0x8252E8FC; continue 'dispatch;
            }
            0x8252E8FC => {
    //   block [0x8252E8FC..0x8252E91C)
	// 8252E8FC: 8125001C  lwz r9, 0x1c(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252E900: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252E904: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 8252E908: 89290020  lbz r9, 0x20(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252E90C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8252E910: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8252E914: 4198FFCC  blt cr6, 0x8252e8e0
	if ctx.cr[6].lt {
	pc = 0x8252E8E0; continue 'dispatch;
	}
	// 8252E918: 480067EC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8252E91C => {
    //   block [0x8252E91C..0x8252E928)
	// 8252E91C: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8252E920: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8252E924: 480067E0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8252E928 => {
    //   block [0x8252E928..0x8252E92C)
	// 8252E928: D1470018  stfs f10, 0x18(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	pc = 0x8252E92C; continue 'dispatch;
            }
            0x8252E92C => {
    //   block [0x8252E92C..0x8252E930)
	// 8252E92C: 480067D8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E930 size=56
    let mut pc: u32 = 0x8252E930;
    'dispatch: loop {
        match pc {
            0x8252E930 => {
    //   block [0x8252E930..0x8252E968)
	// 8252E930: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252E934: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8252E938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252E93C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 8252E940: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252E944: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252E948: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8252E94C: 396B659C  addi r11, r11, 0x659c
	ctx.r[11].s64 = ctx.r[11].s64 + 26012;
	// 8252E950: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252E954: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8252E958: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8252E95C: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8252E960: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8252E964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252E968 size=32
    let mut pc: u32 = 0x8252E968;
    'dispatch: loop {
        match pc {
            0x8252E968 => {
    //   block [0x8252E968..0x8252E988)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252E988 size=60
    let mut pc: u32 = 0x8252E988;
    'dispatch: loop {
        match pc {
            0x8252E988 => {
    //   block [0x8252E988..0x8252E9C4)
	// 8252E988: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E98C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252E990: 7D0B2214  add r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8252E994: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252E998: 0CC90000  twi 6, r9, 0
	// 8252E99C: 7CE84BD6  divw r7, r8, r9
	ctx.r[7].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8252E9A0: 550B083E  rotlwi r11, r8, 1
	ctx.r[11].u64 = ((ctx.r[8].u32).rotate_left(1)) as u64;
	// 8252E9A4: 7CE749D6  mullw r7, r7, r9
	ctx.r[7].s32 = ((ctx.r[7].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 8252E9A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252E9AC: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8252E9B0: 7D295878  andc r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[11].u64;
	// 8252E9B4: 550B2834  slwi r11, r8, 5
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252E9B8: 0CA9FFFF  twi 5, r9, -1
	// 8252E9BC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252E9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252E9C8 size=8
    let mut pc: u32 = 0x8252E9C8;
    'dispatch: loop {
        match pc {
            0x8252E9C8 => {
    //   block [0x8252E9C8..0x8252E9D0)
	// 8252E9C8: C023001C  lfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252E9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252E9D0 size=136
    let mut pc: u32 = 0x8252E9D0;
    'dispatch: loop {
        match pc {
            0x8252E9D0 => {
    //   block [0x8252E9D0..0x8252EA50)
	// 8252E9D0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8252E9D4: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252E9D8: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252E9DC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8252E9E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252E9E4: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8252E9E8: 80A9000C  lwz r5, 0xc(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252E9EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8252E9F0: 40990060  ble cr6, 0x8252ea50
	if !ctx.cr[6].gt {
	pc = 0x8252EA50; continue 'dispatch;
	}
	// 8252E9F4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8252E9F8: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252E9FC: 8089000C  lwz r4, 0xc(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252EA00: 7C685A14  add r3, r8, r11
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8252EA04: 80E90010  lwz r7, 0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252EA08: 0CC40000  twi 6, r4, 0
	// 8252EA0C: 7FE323D6  divw r31, r3, r4
	ctx.r[31].s32 = ctx.r[3].s32 / ctx.r[4].s32;
	// 8252EA10: 5468083E  rotlwi r8, r3, 1
	ctx.r[8].u64 = ((ctx.r[3].u32).rotate_left(1)) as u64;
	// 8252EA14: 7FFF21D6  mullw r31, r31, r4
	ctx.r[31].s32 = ((ctx.r[31].s32 as i64 * ctx.r[4].s32 as i64) as i32);
	ctx.r[31].s64 = ctx.r[31].s32 as i64;
	// 8252EA18: 7C7F1850  subf r3, r31, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8252EA1C: 3BE8FFFF  addi r31, r8, -1
	ctx.r[31].s64 = ctx.r[8].s64 + -1;
	// 8252EA20: 54682834  slwi r8, r3, 5
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252EA24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8252EA28: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 8252EA2C: 7C84F878  andc r4, r4, r31
	ctx.r[4].u64 = ctx.r[4].u64 & !ctx.r[31].u64;
	// 8252EA30: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8252EA34: 0CA4FFFF  twi 5, r4, -1
	pc = 0x8252EA50; continue 'dispatch;
            }
            0x8252EA50 => {
    //   block [0x8252EA50..0x8252EA58)
	// 8252EA50: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8252EA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252EA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252EA90 size=188
    let mut pc: u32 = 0x8252EA90;
    'dispatch: loop {
        match pc {
            0x8252EA90 => {
    //   block [0x8252EA90..0x8252EB4C)
	// 8252EA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252EA94: 48006625  bl 0x825350b8
	ctx.lr = 0x8252EA98;
	sub_82535080(ctx, base);
	// 8252EA98: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252EB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252EB50 size=196
    let mut pc: u32 = 0x8252EB50;
    'dispatch: loop {
        match pc {
            0x8252EB50 => {
    //   block [0x8252EB50..0x8252EBA0)
	// 8252EB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252EB54: 48006561  bl 0x825350b4
	ctx.lr = 0x8252EB58;
	sub_82535080(ctx, base);
	// 8252EB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252EB5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8252EB60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252EB64: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8252EB68: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8252EB6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8252EB70: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252EB74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252EB78: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252EB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252EB80: 4E800421  bctrl
	ctx.lr = 0x8252EB84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252EB84: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252EB88: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8252EB8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252EB90: 40990068  ble cr6, 0x8252ebf8
	if !ctx.cr[6].gt {
	pc = 0x8252EBF8; continue 'dispatch;
	}
	// 8252EB94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252EB98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8252EB9C: 3B6B6D54  addi r27, r11, 0x6d54
	ctx.r[27].s64 = ctx.r[11].s64 + 27988;
            }
            0x8252EBA0 => {
    //   block [0x8252EBA0..0x8252EBE4)
	// 8252EBA0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252EBA4: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8252EBA8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252EBAC: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8252EBB0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8252EBB4: 409A0030  bne cr6, 0x8252ebe4
	if !ctx.cr[6].eq {
	pc = 0x8252EBE4; continue 'dispatch;
	}
	// 8252EBB8: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252EBBC: 55482834  slwi r8, r10, 5
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252EBC0: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252EBC4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8252EBC8: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252EBCC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8252EBD0: 54E72834  slwi r7, r7, 5
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8252EBD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252EBD8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252EBDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252EBE0: 4E800421  bctrl
	ctx.lr = 0x8252EBE4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252EBE4 => {
    //   block [0x8252EBE4..0x8252EBF8)
	// 8252EBE4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252EBE8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8252EBEC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8252EBF0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252EBF4: 4198FFAC  blt cr6, 0x8252eba0
	if ctx.cr[6].lt {
	pc = 0x8252EBA0; continue 'dispatch;
	}
	pc = 0x8252EBF8; continue 'dispatch;
            }
            0x8252EBF8 => {
    //   block [0x8252EBF8..0x8252EC14)
	// 8252EBF8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252EBFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252EC00: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252EC04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252EC08: 4E800421  bctrl
	ctx.lr = 0x8252EC0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252EC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252EC10: 480064F4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252EC18 size=180
    let mut pc: u32 = 0x8252EC18;
    'dispatch: loop {
        match pc {
            0x8252EC18 => {
    //   block [0x8252EC18..0x8252EC74)
	// 8252EC18: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252EC1C: C1A4001C  lfs f13, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252EC20: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252EC24: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8252EC28: 409A004C  bne cr6, 0x8252ec74
	if !ctx.cr[6].eq {
	pc = 0x8252EC74; continue 'dispatch;
	}
	// 8252EC2C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252EC30: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252EC34: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252EC38: 0CCB0000  twi 6, r11, 0
	// 8252EC3C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8252EC40: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 8252EC44: 7CE85BD6  divw r7, r8, r11
	ctx.r[7].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 8252EC48: 550A083E  rotlwi r10, r8, 1
	ctx.r[10].u64 = ((ctx.r[8].u32).rotate_left(1)) as u64;
	// 8252EC4C: 7CE759D6  mullw r7, r7, r11
	ctx.r[7].s32 = ((ctx.r[7].s32 as i64 * ctx.r[11].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 8252EC50: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8252EC54: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 8252EC58: 550A2834  slwi r10, r8, 5
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252EC5C: 7D6B3878  andc r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[7].u64;
	// 8252EC60: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8252EC64: 0CABFFFF  twi 5, r11, -1
	// 8252EC68: C1AA001C  lfs f13, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252EC6C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8252EC70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x8252EC74; continue 'dispatch;
            }
            0x8252EC74 => {
    //   block [0x8252EC74..0x8252ECCC)
	// 8252EC74: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252EC78: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252EC7C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252ECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252ECD0 size=552
    let mut pc: u32 = 0x8252ECD0;
    'dispatch: loop {
        match pc {
            0x8252ECD0 => {
    //   block [0x8252ECD0..0x8252EEF8)
	// 8252ECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252ECD4: 480063D5  bl 0x825350a8
	ctx.lr = 0x8252ECD8;
	sub_82535080(ctx, base);
	// 8252ECD8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252EEF8 size=284
    let mut pc: u32 = 0x8252EEF8;
    'dispatch: loop {
        match pc {
            0x8252EEF8 => {
    //   block [0x8252EEF8..0x8252EF70)
	// 8252EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252EEFC: 48006199  bl 0x82535094
	ctx.lr = 0x8252EF00;
	sub_82535080(ctx, base);
	// 8252EF00: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252EF04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252EF08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8252EF0C: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 8252EF10: 3B5D0010  addi r26, r29, 0x10
	ctx.r[26].s64 = ctx.r[29].s64 + 16;
	// 8252EF14: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8252EF18: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252EF1C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252EF20: 3E808000  lis r20, -0x8000
	ctx.r[20].s64 = -2147483648;
	// 8252EF24: 396B6684  addi r11, r11, 0x6684
	ctx.r[11].s64 = ctx.r[11].s64 + 26244;
	// 8252EF28: B27D0006  sth r19, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[19].u16 ) };
	// 8252EF2C: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 8252EF30: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252EF34: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8252EF38: 93DA0004  stw r30, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8252EF3C: 929A0008  stw r20, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[20].u32 ) };
	// 8252EF40: D01D0008  stfs f0, 8(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252EF44: D01D000C  stfs f0, 0xc(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8252EF48: 89640020  lbz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252EF4C: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252EF50: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 8252EF54: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8252EF58: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8252EF5C: 40980024  bge cr6, 0x8252ef80
	if !ctx.cr[6].lt {
	pc = 0x8252EF80; continue 'dispatch;
	}
	// 8252EF60: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252EF64: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252EF68: 41980008  blt cr6, 0x8252ef70
	if ctx.cr[6].lt {
	pc = 0x8252EF70; continue 'dispatch;
	}
	// 8252EF6C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8252EF70; continue 'dispatch;
            }
            0x8252EF70 => {
    //   block [0x8252EF70..0x8252EF80)
	// 8252EF70: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8252EF74: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8252EF78: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8252EF7C: 4BF3F34D  bl 0x8246e2c8
	ctx.lr = 0x8252EF80;
	sub_8246E2C8(ctx, base);
	pc = 0x8252EF80; continue 'dispatch;
            }
            0x8252EF80 => {
    //   block [0x8252EF80..0x8252EFAC)
	// 8252EF80: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8252EF84: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 8252EF88: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252EF8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252EF90: 40990078  ble cr6, 0x8252f008
	if !ctx.cr[6].gt {
	pc = 0x8252F008; continue 'dispatch;
	}
	// 8252EF94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252EF98: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252EF9C: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 8252EFA0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8252EFA4: 3B2B6674  addi r25, r11, 0x6674
	ctx.r[25].s64 = ctx.r[11].s64 + 26228;
	// 8252EFA8: 3AC0001C  li r22, 0x1c
	ctx.r[22].s64 = 28;
	pc = 0x8252EFAC; continue 'dispatch;
            }
            0x8252EFAC => {
    //   block [0x8252EFAC..0x8252F008)
	// 8252EFAC: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8252EFB0: 7C78B82E  lwzx r3, r24, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8252EFB4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8252EFB8: 4BF35081  bl 0x82464038
	ctx.lr = 0x8252EFBC;
	sub_82464038(ctx, base);
	// 8252EFBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252EFC0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8252EFC4: B2DF0004  sth r22, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[22].u16 ) };
	// 8252EFC8: B27F0006  sth r19, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[19].u16 ) };
	// 8252EFCC: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8252EFD0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8252EFD4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8252EFD8: 929F0018  stw r20, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[20].u32 ) };
	// 8252EFDC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8252EFE0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8252EFE4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8252EFE8: 4BFFFAA9  bl 0x8252ea90
	ctx.lr = 0x8252EFEC;
	sub_8252EA90(ctx, base);
	// 8252EFEC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252EFF0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8252EFF4: 7FEBE12E  stwx r31, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[31].u32) };
	// 8252EFF8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8252EFFC: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252F000: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252F004: 4198FFA8  blt cr6, 0x8252efac
	if ctx.cr[6].lt {
	pc = 0x8252EFAC; continue 'dispatch;
	}
	pc = 0x8252F008; continue 'dispatch;
            }
            0x8252F008 => {
    //   block [0x8252F008..0x8252F014)
	// 8252F008: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8252F00C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8252F010: 480060D4  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252F018 size=196
    let mut pc: u32 = 0x8252F018;
    'dispatch: loop {
        match pc {
            0x8252F018 => {
    //   block [0x8252F018..0x8252F048)
	// 8252F018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252F01C: 480060A1  bl 0x825350bc
	ctx.lr = 0x8252F020;
	sub_82535080(ctx, base);
	// 8252F020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252F024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252F028: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252F02C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8252F030: 396B6684  addi r11, r11, 0x6684
	ctx.r[11].s64 = ctx.r[11].s64 + 26244;
	// 8252F034: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252F038: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252F03C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252F040: 4099005C  ble cr6, 0x8252f09c
	if !ctx.cr[6].gt {
	pc = 0x8252F09C; continue 'dispatch;
	}
	// 8252F044: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x8252F048; continue 'dispatch;
            }
            0x8252F048 => {
    //   block [0x8252F048..0x8252F088)
	// 8252F048: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252F04C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8252F050: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F054: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F058: 419A0030  beq cr6, 0x8252f088
	if ctx.cr[6].eq {
	pc = 0x8252F088; continue 'dispatch;
	}
	// 8252F05C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F060: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F064: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F06C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F070: 409A0018  bne cr6, 0x8252f088
	if !ctx.cr[6].eq {
	pc = 0x8252F088; continue 'dispatch;
	}
	// 8252F074: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F078: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F07C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F084: 4E800421  bctrl
	ctx.lr = 0x8252F088;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F088 => {
    //   block [0x8252F088..0x8252F09C)
	// 8252F088: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252F08C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8252F090: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8252F094: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8252F098: 4198FFB0  blt cr6, 0x8252f048
	if ctx.cr[6].lt {
	pc = 0x8252F048; continue 'dispatch;
	}
	pc = 0x8252F09C; continue 'dispatch;
            }
            0x8252F09C => {
    //   block [0x8252F09C..0x8252F0C8)
	// 8252F09C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252F0A0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252F0A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252F0A8: 409A0020  bne cr6, 0x8252f0c8
	if !ctx.cr[6].eq {
	pc = 0x8252F0C8; continue 'dispatch;
	}
	// 8252F0AC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F0B0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252F0B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252F0B8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252F0BC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252F0C0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252F0C4: 4BF34FF5  bl 0x824640b8
	ctx.lr = 0x8252F0C8;
	sub_824640B8(ctx, base);
	pc = 0x8252F0C8; continue 'dispatch;
            }
            0x8252F0C8 => {
    //   block [0x8252F0C8..0x8252F0DC)
	// 8252F0C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8252F0CC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252F0D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252F0D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252F0D8: 48006034  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252F0E0 size=228
    let mut pc: u32 = 0x8252F0E0;
    'dispatch: loop {
        match pc {
            0x8252F0E0 => {
    //   block [0x8252F0E0..0x8252F11C)
	// 8252F0E0: C1A70000  lfs f13, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F0E4: D1A80004  stfs f13, 4(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8252F0E8: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252F0EC: C0E60000  lfs f7, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8252F0F0: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 8252F0F4: 40980044  bge cr6, 0x8252f138
	if !ctx.cr[6].lt {
	pc = 0x8252F138; continue 'dispatch;
	}
	// 8252F0F8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252F0FC: C163002C  lfs f11, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8252F100: ED6B01F2  fmuls f11, f11, f7
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[7].f64) as f32) as f64);
	// 8252F104: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252F108: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252F10C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8252F110: 4098000C  bge cr6, 0x8252f11c
	if !ctx.cr[6].lt {
	pc = 0x8252F11C; continue 'dispatch;
	}
	// 8252F114: EC0C582A  fadds f0, f12, f11
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 8252F118: 4800001C  b 0x8252f134
	pc = 0x8252F134; continue 'dispatch;
            }
            0x8252F11C => {
    //   block [0x8252F11C..0x8252F134)
	// 8252F11C: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252F120: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252F124: EC0C0028  fsubs f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8252F128: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 8252F12C: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8252F130: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	pc = 0x8252F134; continue 'dispatch;
            }
            0x8252F134 => {
    //   block [0x8252F134..0x8252F138)
	// 8252F134: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	pc = 0x8252F138; continue 'dispatch;
            }
            0x8252F138 => {
    //   block [0x8252F138..0x8252F1C4)
	// 8252F138: C1880004  lfs f12, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252F13C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252F140: C123000C  lfs f9, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8252F144: EC0C4828  fsubs f0, f12, f9
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[9].f64) as f32) as f64);
	// 8252F148: C16B1FF8  lfs f11, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8252F14C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8252F150: 40980074  bge cr6, 0x8252f1c4
	if !ctx.cr[6].lt {
		sub_8252F1C4(ctx, base);
		return;
	}
	// 8252F154: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252F158: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8252F15C: ED2C4828  fsubs f9, f12, f9
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[9].f64) as f32) as f64);
	// 8252F160: C1070000  lfs f8, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8252F164: C1430018  lfs f10, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252F168: FF086000  fcmpu cr6, f8, f12
	ctx.cr[6].compare_f64(ctx.f[8].f64, ctx.f[12].f64);
	// 8252F16C: C1630024  lfs f11, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8252F170: C0A30020  lfs f5, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8252F174: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F178: ECA55828  fsubs f5, f5, f11
	ctx.f[5].f64 = (((ctx.f[5].f64 - ctx.f[11].f64) as f32) as f64);
	// 8252F17C: ECCA6828  fsubs f6, f10, f13
	ctx.f[6].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252F180: C1430014  lfs f10, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252F184: ED8D4824  fdivs f12, f13, f9
	ctx.f[12].f64 = ((ctx.f[13].f64 / ctx.f[9].f64) as f32) as f64;
	// 8252F188: ED260332  fmuls f9, f6, f12
	ctx.f[9].f64 = (((ctx.f[6].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F18C: ECC50332  fmuls f6, f5, f12
	ctx.f[6].f64 = (((ctx.f[5].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F190: ED890332  fmuls f12, f9, f12
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F194: ED66583A  fmadds f11, f6, f0, f11
	ctx.f[11].f64 = (((ctx.f[6].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 8252F198: ED2C0032  fmuls f9, f12, f0
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252F19C: ED8B02B2  fmuls f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8252F1A0: EC09683A  fmadds f0, f9, f0, f13
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8252F1A4: EC0002B2  fmuls f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 8252F1A8: 40980090  bge cr6, 0x8252f238
	if !ctx.cr[6].lt {
		sub_8252F224(ctx, base);
		return;
	}
	// 8252F1AC: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F1B0: EDA86824  fdivs f13, f8, f13
	ctx.f[13].f64 = ((ctx.f[8].f64 / ctx.f[13].f64) as f32) as f64;
	// 8252F1B4: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F1B8: EC0061F8  fmsubs f0, f0, f7, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 8252F1BC: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252F1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F1C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252F1C4 size=96
    let mut pc: u32 = 0x8252F1C4;
    'dispatch: loop {
        match pc {
            0x8252F1C4 => {
    //   block [0x8252F1C4..0x8252F224)
	// 8252F1C4: C1A30010  lfs f13, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F1C8: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8252F1CC: 40980058  bge cr6, 0x8252f224
	if !ctx.cr[6].lt {
		sub_8252F224(ctx, base);
		return;
	}
	// 8252F1D0: ED8D4828  fsubs f12, f13, f9
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 8252F1D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8252F1D8: C143001C  lfs f10, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252F1DC: C1630024  lfs f11, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8252F1E0: C1030028  lfs f8, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8252F1E4: ED085828  fsubs f8, f8, f11
	ctx.f[8].f64 = (((ctx.f[8].f64 - ctx.f[11].f64) as f32) as f64);
	// 8252F1E8: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F1EC: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 8252F1F0: C1430014  lfs f10, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252F1F4: ED8D6024  fdivs f12, f13, f12
	ctx.f[12].f64 = ((ctx.f[13].f64 / ctx.f[12].f64) as f32) as f64;
	// 8252F1F8: ED290332  fmuls f9, f9, f12
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F1FC: ED080332  fmuls f8, f8, f12
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F200: ED890332  fmuls f12, f9, f12
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F204: ED68583A  fmadds f11, f8, f0, f11
	ctx.f[11].f64 = (((ctx.f[8].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 8252F208: ED2C0032  fmuls f9, f12, f0
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8252F20C: ED8B02B2  fmuls f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8252F210: EC09683A  fmadds f0, f9, f0, f13
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8252F214: EC0002B2  fmuls f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 8252F218: EC0061F8  fmsubs f0, f0, f7, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 8252F21C: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252F220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F224(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252F224 size=32
    let mut pc: u32 = 0x8252F224;
    'dispatch: loop {
        match pc {
            0x8252F224 => {
    //   block [0x8252F224..0x8252F244)
	// 8252F224: D1A80004  stfs f13, 4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8252F228: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 8252F22C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F230: C1830014  lfs f12, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252F234: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 8252F238: EC0061F8  fmsubs f0, f0, f7, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 8252F23C: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252F240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252F248 size=120
    let mut pc: u32 = 0x8252F248;
    'dispatch: loop {
        match pc {
            0x8252F248 => {
    //   block [0x8252F248..0x8252F2C0)
	// 8252F248: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252F24C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252F250: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8252F254: 396B678C  addi r11, r11, 0x678c
	ctx.r[11].s64 = ctx.r[11].s64 + 26508;
	// 8252F258: C00A204C  lfs f0, 0x204c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252F25C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252F260: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252F264: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8252F268: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252F26C: C18A2898  lfs f12, 0x2898(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10392 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8252F270: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252F274: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8252F278: C16A2C04  lfs f11, 0x2c04(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(11268 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8252F27C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252F280: D1630010  stfs f11, 0x10(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252F284: C14A266C  lfs f10, 0x266c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9836 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8252F288: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8252F28C: D1430014  stfs f10, 0x14(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8252F290: C1AA1850  lfs f13, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F294: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252F298: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8252F29C: D1A3001C  stfs f13, 0x1c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8252F2A0: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252F2A4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252F2A8: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8252F2AC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8252F2B0: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8252F2B4: C1AA6D5C  lfs f13, 0x6d5c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27996 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8252F2B8: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8252F2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252F2C0 size=4
    let mut pc: u32 = 0x8252F2C0;
    'dispatch: loop {
        match pc {
            0x8252F2C0 => {
    //   block [0x8252F2C0..0x8252F2C4)
	// 8252F2C0: 4807F6B8  b 0x825ae978
	sub_825AE978(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8252F2C8 size=8
    let mut pc: u32 = 0x8252F2C8;
    'dispatch: loop {
        match pc {
            0x8252F2C8 => {
    //   block [0x8252F2C8..0x8252F2D0)
	// 8252F2C8: C02300B8  lfs f1, 0xb8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252F2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252F2D0 size=20
    let mut pc: u32 = 0x8252F2D0;
    'dispatch: loop {
        match pc {
            0x8252F2D0 => {
    //   block [0x8252F2D0..0x8252F2E4)
	// 8252F2D0: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8252F2D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F2D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252F2DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F2E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252F2E8 size=256
    let mut pc: u32 = 0x8252F2E8;
    'dispatch: loop {
        match pc {
            0x8252F2E8 => {
    //   block [0x8252F2E8..0x8252F3E8)
	// 8252F2E8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8252F2EC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8252F2F0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8252F2F4: 394A9F80  addi r10, r10, -0x6080
	ctx.r[10].s64 = ctx.r[10].s64 + -24704;
	// 8252F2F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8252F2FC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8252F300: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 8252F304: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252F3E8 size=116
    let mut pc: u32 = 0x8252F3E8;
    'dispatch: loop {
        match pc {
            0x8252F3E8 => {
    //   block [0x8252F3E8..0x8252F45C)
	// 8252F3E8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8252F3EC: 39450030  addi r10, r5, 0x30
	ctx.r[10].s64 = ctx.r[5].s64 + 48;
	// 8252F3F0: 39250010  addi r9, r5, 0x10
	ctx.r[9].s64 = ctx.r[5].s64 + 16;
	// 8252F3F4: 39050020  addi r8, r5, 0x20
	ctx.r[8].s64 = ctx.r[5].s64 + 32;
	// 8252F3F8: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 8252F3FC: 98C50000  stb r6, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 8252F400: 38E50040  addi r7, r5, 0x40
	ctx.r[7].s64 = ctx.r[5].s64 + 64;
	// 8252F404: 98C50001  stb r6, 1(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(1 as u32), ctx.r[6].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252F460 size=236
    let mut pc: u32 = 0x8252F460;
    'dispatch: loop {
        match pc {
            0x8252F460 => {
    //   block [0x8252F460..0x8252F544)
	// 8252F460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252F464: 48005C3D  bl 0x825350a0
	ctx.lr = 0x8252F468;
	sub_82535080(ctx, base);
	// 8252F468: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252F46C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252F470: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8252F474: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8252F478: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252F47C: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252F480: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8252F484: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F488: 409900BC  ble cr6, 0x8252f544
	if !ctx.cr[6].gt {
	pc = 0x8252F544; continue 'dispatch;
	}
	// 8252F48C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8252F490: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8252F494: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8252F498: 3AC00040  li r22, 0x40
	ctx.r[22].s64 = 64;
	// 8252F49C: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252F4A0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8252F4A4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252F4A8: 7FABE214  add r29, r11, r28
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8252F4AC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252F4B0: 3B5D0060  addi r26, r29, 0x60
	ctx.r[26].s64 = ctx.r[29].s64 + 96;
	// 8252F4B4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8252F4B8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8252F4BC: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 8252F4C0: 4807C6E1  bl 0x825abba0
	ctx.lr = 0x8252F4C4;
	sub_825ABBA0(ctx, base);
	// 8252F4C4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252F4C8: 3B3D0030  addi r25, r29, 0x30
	ctx.r[25].s64 = ctx.r[29].s64 + 48;
	// 8252F4CC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8252F4D0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8252F4D4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252F4D8: 7CABF214  add r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8252F4DC: 4807C62D  bl 0x825abb08
	ctx.lr = 0x8252F4E0;
	sub_825ABB08(ctx, base);
	// 8252F4E0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252F4E4: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252F4E8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	pc = 0x8252F544; continue 'dispatch;
            }
            0x8252F544 => {
    //   block [0x8252F544..0x8252F54C)
	// 8252F544: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8252F548: 48005BA8  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252F550 size=376
    let mut pc: u32 = 0x8252F550;
    'dispatch: loop {
        match pc {
            0x8252F550 => {
    //   block [0x8252F550..0x8252F6C8)
	// 8252F550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252F554: 48005B5D  bl 0x825350b0
	ctx.lr = 0x8252F558;
	sub_82535080(ctx, base);
	// 8252F558: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252F55C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8252F560: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252F564: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8252F568: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252F56C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8252F570: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8252F574: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8252F578: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252F57C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8252F580: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252F584: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8252F588: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 8252F58C: C01F00A4  lfs f0, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252F590: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8252F594: 48078EB5  bl 0x825a8448
	ctx.lr = 0x8252F598;
	sub_825A8448(ctx, base);
	// 8252F598: 397D0150  addi r11, r29, 0x150
	ctx.r[11].s64 = ctx.r[29].s64 + 336;
	// 8252F59C: 395F0090  addi r10, r31, 0x90
	ctx.r[10].s64 = ctx.r[31].s64 + 144;
	// 8252F5A0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8252F5A4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8252F5A8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252F6C8 size=76
    let mut pc: u32 = 0x8252F6C8;
    'dispatch: loop {
        match pc {
            0x8252F6C8 => {
    //   block [0x8252F6C8..0x8252F714)
	// 8252F6C8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252F6CC: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 8252F6D0: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 8252F6D4: 396B01A0  addi r11, r11, 0x1a0
	ctx.r[11].s64 = ctx.r[11].s64 + 416;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252F718 size=76
    let mut pc: u32 = 0x8252F718;
    'dispatch: loop {
        match pc {
            0x8252F718 => {
    //   block [0x8252F718..0x8252F764)
	// 8252F718: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252F71C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 8252F720: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 8252F724: 396B01A0  addi r11, r11, 0x1a0
	ctx.r[11].s64 = ctx.r[11].s64 + 416;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8252F768 size=240
    let mut pc: u32 = 0x8252F768;
    'dispatch: loop {
        match pc {
            0x8252F768 => {
    //   block [0x8252F768..0x8252F858)
	// 8252F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252F770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252F774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252F778: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8252F77C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252F780: 4807F281  bl 0x825aea00
	ctx.lr = 0x8252F784;
	sub_825AEA00(ctx, base);
	// 8252F784: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8252F788: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8252F78C: 397F0054  addi r11, r31, 0x54
	ctx.r[11].s64 = ctx.r[31].s64 + 84;
	// 8252F790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252F794: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252F798: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252F79C: 390A6B94  addi r8, r10, 0x6b94
	ctx.r[8].s64 = ctx.r[10].s64 + 27540;
	// 8252F7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8252F7A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8252F7A8: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8252F7AC: 915F004C  stw r10, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8252F7B0: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8252F7B4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8252F7B8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8252F7BC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8252F7C0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8252F7C4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8252F7C8: D00B0014  stfs f0, 0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8252F7CC: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8252F7D0: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8252F7D4: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8252F7D8: D00B0024  stfs f0, 0x24(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8252F7DC: D00B0028  stfs f0, 0x28(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8252F7E0: D00B002C  stfs f0, 0x2c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8252F7E4: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8252F7E8: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8252F7EC: D00B0038  stfs f0, 0x38(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8252F7F0: D00B003C  stfs f0, 0x3c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8252F7F4: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8252F7F8: D00B0044  stfs f0, 0x44(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8252F7FC: 915F00A0  stw r10, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 8252F800: 915F00A4  stw r10, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 8252F804: 913F00A8  stw r9, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[9].u32 ) };
	// 8252F808: 915F00C0  stw r10, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 8252F80C: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8252F810: 913F00C8  stw r9, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 8252F814: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8252F818: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8252F81C: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8252F820: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8252F824: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8252F828: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8252F82C: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8252F830: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8252F834: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 8252F838: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8252F83C: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 8252F840: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 8252F844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252F848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252F84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252F850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252F854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252F858 size=932
    let mut pc: u32 = 0x8252F858;
    'dispatch: loop {
        match pc {
            0x8252F858 => {
    //   block [0x8252F858..0x8252F8B4)
	// 8252F858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252F85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8252F860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252F864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252F868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252F86C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252F870: 396B6B94  addi r11, r11, 0x6b94
	ctx.r[11].s64 = ctx.r[11].s64 + 27540;
	// 8252F874: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252F878: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252F87C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F884: 419A0030  beq cr6, 0x8252f8b4
	if ctx.cr[6].eq {
	pc = 0x8252F8B4; continue 'dispatch;
	}
	// 8252F888: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F88C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F890: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F894: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F898: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F89C: 409A0018  bne cr6, 0x8252f8b4
	if !ctx.cr[6].eq {
	pc = 0x8252F8B4; continue 'dispatch;
	}
	// 8252F8A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F8A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F8A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F8AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F8B0: 4E800421  bctrl
	ctx.lr = 0x8252F8B4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F8B4 => {
    //   block [0x8252F8B4..0x8252F8F0)
	// 8252F8B4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252F8B8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F8BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F8C0: 419A0030  beq cr6, 0x8252f8f0
	if ctx.cr[6].eq {
	pc = 0x8252F8F0; continue 'dispatch;
	}
	// 8252F8C4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F8C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F8CC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F8D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F8D4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F8D8: 409A0018  bne cr6, 0x8252f8f0
	if !ctx.cr[6].eq {
	pc = 0x8252F8F0; continue 'dispatch;
	}
	// 8252F8DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F8E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F8E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F8E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F8EC: 4E800421  bctrl
	ctx.lr = 0x8252F8F0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F8F0 => {
    //   block [0x8252F8F0..0x8252F92C)
	// 8252F8F0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8252F8F4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F8F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F8FC: 419A0030  beq cr6, 0x8252f92c
	if ctx.cr[6].eq {
	pc = 0x8252F92C; continue 'dispatch;
	}
	// 8252F900: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F904: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F908: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F90C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F910: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F914: 409A0018  bne cr6, 0x8252f92c
	if !ctx.cr[6].eq {
	pc = 0x8252F92C; continue 'dispatch;
	}
	// 8252F918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F91C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F920: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F928: 4E800421  bctrl
	ctx.lr = 0x8252F92C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F92C => {
    //   block [0x8252F92C..0x8252F968)
	// 8252F92C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8252F930: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F934: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F938: 419A0030  beq cr6, 0x8252f968
	if ctx.cr[6].eq {
	pc = 0x8252F968; continue 'dispatch;
	}
	// 8252F93C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F940: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F944: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F948: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F94C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F950: 409A0018  bne cr6, 0x8252f968
	if !ctx.cr[6].eq {
	pc = 0x8252F968; continue 'dispatch;
	}
	// 8252F954: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F958: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F95C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F964: 4E800421  bctrl
	ctx.lr = 0x8252F968;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F968 => {
    //   block [0x8252F968..0x8252F9A4)
	// 8252F968: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8252F96C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F970: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F974: 419A0030  beq cr6, 0x8252f9a4
	if ctx.cr[6].eq {
	pc = 0x8252F9A4; continue 'dispatch;
	}
	// 8252F978: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F97C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F980: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F984: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F988: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F98C: 409A0018  bne cr6, 0x8252f9a4
	if !ctx.cr[6].eq {
	pc = 0x8252F9A4; continue 'dispatch;
	}
	// 8252F990: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F994: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F998: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F99C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F9A0: 4E800421  bctrl
	ctx.lr = 0x8252F9A4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F9A4 => {
    //   block [0x8252F9A4..0x8252F9E0)
	// 8252F9A4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8252F9A8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F9B0: 419A0030  beq cr6, 0x8252f9e0
	if ctx.cr[6].eq {
	pc = 0x8252F9E0; continue 'dispatch;
	}
	// 8252F9B4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F9B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F9BC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F9C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252F9C4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252F9C8: 409A0018  bne cr6, 0x8252f9e0
	if !ctx.cr[6].eq {
	pc = 0x8252F9E0; continue 'dispatch;
	}
	// 8252F9CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F9D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252F9D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252F9D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252F9DC: 4E800421  bctrl
	ctx.lr = 0x8252F9E0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252F9E0 => {
    //   block [0x8252F9E0..0x8252FA1C)
	// 8252F9E0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252F9E4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252F9E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252F9EC: 419A0030  beq cr6, 0x8252fa1c
	if ctx.cr[6].eq {
	pc = 0x8252FA1C; continue 'dispatch;
	}
	// 8252F9F0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252F9F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252F9F8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252F9FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FA00: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252FA04: 409A0018  bne cr6, 0x8252fa1c
	if !ctx.cr[6].eq {
	pc = 0x8252FA1C; continue 'dispatch;
	}
	// 8252FA08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FA0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252FA10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FA14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FA18: 4E800421  bctrl
	ctx.lr = 0x8252FA1C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FA1C => {
    //   block [0x8252FA1C..0x8252FA58)
	// 8252FA1C: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8252FA20: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252FA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252FA28: 419A0030  beq cr6, 0x8252fa58
	if ctx.cr[6].eq {
	pc = 0x8252FA58; continue 'dispatch;
	}
	// 8252FA2C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252FA30: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252FA34: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252FA38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FA3C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252FA40: 409A0018  bne cr6, 0x8252fa58
	if !ctx.cr[6].eq {
	pc = 0x8252FA58; continue 'dispatch;
	}
	// 8252FA44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FA48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252FA4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FA50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FA54: 4E800421  bctrl
	ctx.lr = 0x8252FA58;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FA58 => {
    //   block [0x8252FA58..0x8252FA94)
	// 8252FA58: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8252FA5C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252FA60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252FA64: 419A0030  beq cr6, 0x8252fa94
	if ctx.cr[6].eq {
	pc = 0x8252FA94; continue 'dispatch;
	}
	// 8252FA68: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252FA6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252FA70: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252FA74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FA78: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252FA7C: 409A0018  bne cr6, 0x8252fa94
	if !ctx.cr[6].eq {
	pc = 0x8252FA94; continue 'dispatch;
	}
	// 8252FA80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FA84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252FA88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FA8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FA90: 4E800421  bctrl
	ctx.lr = 0x8252FA94;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FA94 => {
    //   block [0x8252FA94..0x8252FAD0)
	// 8252FA94: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8252FA98: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252FA9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252FAA0: 419A0030  beq cr6, 0x8252fad0
	if ctx.cr[6].eq {
	pc = 0x8252FAD0; continue 'dispatch;
	}
	// 8252FAA4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252FAA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252FAAC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252FAB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FAB4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252FAB8: 409A0018  bne cr6, 0x8252fad0
	if !ctx.cr[6].eq {
	pc = 0x8252FAD0; continue 'dispatch;
	}
	// 8252FABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FAC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252FAC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FAC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FACC: 4E800421  bctrl
	ctx.lr = 0x8252FAD0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FAD0 => {
    //   block [0x8252FAD0..0x8252FB0C)
	// 8252FAD0: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8252FAD4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252FAD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252FADC: 419A0030  beq cr6, 0x8252fb0c
	if ctx.cr[6].eq {
	pc = 0x8252FB0C; continue 'dispatch;
	}
	// 8252FAE0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252FAE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252FAE8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252FAEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FAF0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252FAF4: 409A0018  bne cr6, 0x8252fb0c
	if !ctx.cr[6].eq {
	pc = 0x8252FB0C; continue 'dispatch;
	}
	// 8252FAF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FAFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252FB00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FB08: 4E800421  bctrl
	ctx.lr = 0x8252FB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FB0C => {
    //   block [0x8252FB0C..0x8252FB50)
	// 8252FB0C: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8252FB10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8252FB14: 419A003C  beq cr6, 0x8252fb50
	if ctx.cr[6].eq {
	pc = 0x8252FB50; continue 'dispatch;
	}
	// 8252FB18: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252FB1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252FB20: 419A0030  beq cr6, 0x8252fb50
	if ctx.cr[6].eq {
	pc = 0x8252FB50; continue 'dispatch;
	}
	// 8252FB24: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8252FB28: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8252FB2C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8252FB30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FB34: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8252FB38: 409A0018  bne cr6, 0x8252fb50
	if !ctx.cr[6].eq {
	pc = 0x8252FB50; continue 'dispatch;
	}
	// 8252FB3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FB40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252FB44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FB48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FB4C: 4E800421  bctrl
	ctx.lr = 0x8252FB50;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FB50 => {
    //   block [0x8252FB50..0x8252FB7C)
	// 8252FB50: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 8252FB54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252FB58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252FB5C: 409A0020  bne cr6, 0x8252fb7c
	if !ctx.cr[6].eq {
	pc = 0x8252FB7C; continue 'dispatch;
	}
	// 8252FB60: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FB64: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252FB68: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252FB6C: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 8252FB70: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252FB74: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252FB78: 4BF34541  bl 0x824640b8
	ctx.lr = 0x8252FB7C;
	sub_824640B8(ctx, base);
	pc = 0x8252FB7C; continue 'dispatch;
            }
            0x8252FB7C => {
    //   block [0x8252FB7C..0x8252FBA8)
	// 8252FB7C: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8252FB80: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252FB84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252FB88: 409A0020  bne cr6, 0x8252fba8
	if !ctx.cr[6].eq {
	pc = 0x8252FBA8; continue 'dispatch;
	}
	// 8252FB8C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FB90: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252FB94: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252FB98: 809F00A0  lwz r4, 0xa0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8252FB9C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252FBA0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252FBA4: 4BF34515  bl 0x824640b8
	ctx.lr = 0x8252FBA8;
	sub_824640B8(ctx, base);
	pc = 0x8252FBA8; continue 'dispatch;
            }
            0x8252FBA8 => {
    //   block [0x8252FBA8..0x8252FBE0)
	// 8252FBA8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8252FBAC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252FBB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252FBB4: 409A002C  bne cr6, 0x8252fbe0
	if !ctx.cr[6].eq {
	pc = 0x8252FBE0; continue 'dispatch;
	}
	// 8252FBB8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FBBC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8252FBC0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252FBC4: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252FBC8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8252FBCC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8252FBD0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252FBD4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252FBD8: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8252FBDC: 4BF344DD  bl 0x824640b8
	ctx.lr = 0x8252FBE0;
	sub_824640B8(ctx, base);
	pc = 0x8252FBE0; continue 'dispatch;
            }
            0x8252FBE0 => {
    //   block [0x8252FBE0..0x8252FBFC)
	// 8252FBE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252FBE4: 4807EE85  bl 0x825aea68
	ctx.lr = 0x8252FBE8;
	sub_825AEA68(ctx, base);
	// 8252FBE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8252FBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252FBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252FBF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8252FC00 size=752
    let mut pc: u32 = 0x8252FC00;
    'dispatch: loop {
        match pc {
            0x8252FC00 => {
    //   block [0x8252FC00..0x8252FEF0)
	// 8252FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252FC04: 480054B1  bl 0x825350b4
	ctx.lr = 0x8252FC08;
	sub_82535080(ctx, base);
	// 8252FC08: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8252FC0C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8252FEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8252FEF0 size=656
    let mut pc: u32 = 0x8252FEF0;
    'dispatch: loop {
        match pc {
            0x8252FEF0 => {
    //   block [0x8252FEF0..0x8252FF80)
	// 8252FEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252FEF4: 480051C5  bl 0x825350b8
	ctx.lr = 0x8252FEF8;
	sub_82535080(ctx, base);
	// 8252FEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252FEFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252FF00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252FF04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8252FF08: 388B6DE4  addi r4, r11, 0x6de4
	ctx.r[4].s64 = ctx.r[11].s64 + 28132;
	// 8252FF0C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8252FF10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FF14: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8252FF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252FF1C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252FF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FF24: 4E800421  bctrl
	ctx.lr = 0x8252FF28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252FF28: 815E0050  lwz r10, 0x50(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 8252FF2C: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8252FF30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8252FF34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252FF38: 3BAB6DD8  addi r29, r11, 0x6dd8
	ctx.r[29].s64 = ctx.r[11].s64 + 28120;
	// 8252FF3C: 409A0044  bne cr6, 0x8252ff80
	if !ctx.cr[6].eq {
	pc = 0x8252FF80; continue 'dispatch;
	}
	// 8252FF40: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FF44: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8252FF48: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8252FF4C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8252FF50: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252FF54: 80DE0048  lwz r6, 0x48(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8252FF58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252FF5C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8252FF60: 83890008  lwz r28, 8(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252FF64: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8252FF68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252FF6C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8252FF70: 55483032  slwi r8, r10, 6
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252FF74: 55673032  slwi r7, r11, 6
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8252FF78: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 8252FF7C: 4E800421  bctrl
	ctx.lr = 0x8252FF80;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FF80 => {
    //   block [0x8252FF80..0x8252FFBC)
	// 8252FF80: 817E00C8  lwz r11, 0xc8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 8252FF84: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252FF88: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252FF8C: 409A0030  bne cr6, 0x8252ffbc
	if !ctx.cr[6].eq {
	pc = 0x8252FFBC; continue 'dispatch;
	}
	// 8252FF90: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FF94: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8252FF98: 813E00C4  lwz r9, 0xc4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8252FF9C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8252FFA0: 80DE00C0  lwz r6, 0xc0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8252FFA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252FFA8: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8252FFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252FFB0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252FFB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FFB8: 4E800421  bctrl
	ctx.lr = 0x8252FFBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FFBC => {
    //   block [0x8252FFBC..0x8252FFF4)
	// 8252FFBC: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 8252FFC0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8252FFC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252FFC8: 409A002C  bne cr6, 0x8252fff4
	if !ctx.cr[6].eq {
	pc = 0x8252FFF4; continue 'dispatch;
	}
	// 8252FFCC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FFD0: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8252FFD4: 80FE00A4  lwz r7, 0xa4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) } as u64;
	// 8252FFD8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8252FFDC: 80DE00A0  lwz r6, 0xa0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) } as u64;
	// 8252FFE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252FFE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252FFE8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252FFEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252FFF0: 4E800421  bctrl
	ctx.lr = 0x8252FFF4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252FFF4 => {
    //   block [0x8252FFF4..0x82530140)
	// 8252FFF4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252FFF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252FFFC: 80DE001C  lwz r6, 0x1c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82530000: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82530004: 388B6DD0  addi r4, r11, 0x6dd0
	ctx.r[4].s64 = ctx.r[11].s64 + 28112;
	// 82530008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253000C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82530010: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82530014: 4E800421  bctrl
	ctx.lr = 0x82530018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82530018: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253001C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82530020: 80DE0020  lwz r6, 0x20(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82530024: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82530028: 388B6DC4  addi r4, r11, 0x6dc4
	ctx.r[4].s64 = ctx.r[11].s64 + 28100;
	// 8253002C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82530030: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82530034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82530038: 4E800421  bctrl
	ctx.lr = 0x8253003C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253003C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82530040: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82530044: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82530048: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8253004C: 388B6DB8  addi r4, r11, 0x6db8
	ctx.r[4].s64 = ctx.r[11].s64 + 28088;
	// 82530050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82530054: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82530058: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8253005C: 4E800421  bctrl
	ctx.lr = 0x82530060;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82530060: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82530064: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82530068: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8253006C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82530070: 388B6DB0  addi r4, r11, 0x6db0
	ctx.r[4].s64 = ctx.r[11].s64 + 28080;
	// 82530074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82530078: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253007C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82530080: 4E800421  bctrl
	ctx.lr = 0x82530084;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82530084: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82530088: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8253008C: 80DE002C  lwz r6, 0x2c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82530090: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82530094: 388B6DA0  addi r4, r11, 0x6da0
	ctx.r[4].s64 = ctx.r[11].s64 + 28064;
	// 82530098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253009C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825300A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825300A4: 4E800421  bctrl
	ctx.lr = 0x825300A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825300A8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825300AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825300B0: 80DE0030  lwz r6, 0x30(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 825300B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825300B8: 388B6D98  addi r4, r11, 0x6d98
	ctx.r[4].s64 = ctx.r[11].s64 + 28056;
	// 825300BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825300C0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825300C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825300C8: 4E800421  bctrl
	ctx.lr = 0x825300CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825300CC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825300D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825300D4: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 825300D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825300DC: 388B6D8C  addi r4, r11, 0x6d8c
	ctx.r[4].s64 = ctx.r[11].s64 + 28044;
	// 825300E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825300E4: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825300E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825300EC: 4E800421  bctrl
	ctx.lr = 0x825300F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825300F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825300F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825300F8: 80DE0038  lwz r6, 0x38(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 825300FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82530100: 388B6D7C  addi r4, r11, 0x6d7c
	ctx.r[4].s64 = ctx.r[11].s64 + 28028;
	// 82530104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82530108: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253010C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82530110: 4E800421  bctrl
	ctx.lr = 0x82530114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82530114: 80DE0040  lwz r6, 0x40(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82530118: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8253011C: 419A0024  beq cr6, 0x82530140
	if ctx.cr[6].eq {
	pc = 0x82530140; continue 'dispatch;
	}
	// 82530120: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82530124: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82530128: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8253012C: 388B6D70  addi r4, r11, 0x6d70
	ctx.r[4].s64 = ctx.r[11].s64 + 28016;
	// 82530130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82530134: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82530138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8253013C: 4E800421  bctrl
	ctx.lr = 0x82530140;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82530140 => {
    //   block [0x82530140..0x82530180)
	// 82530140: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82530144: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82530148: 80DE0044  lwz r6, 0x44(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8253014C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82530150: 388B6D64  addi r4, r11, 0x6d64
	ctx.r[4].s64 = ctx.r[11].s64 + 28004;
	// 82530154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82530158: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253015C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82530160: 4E800421  bctrl
	ctx.lr = 0x82530164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82530164: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82530168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253016C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82530170: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82530174: 4E800421  bctrl
	ctx.lr = 0x82530178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82530178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8253017C: 48004F8C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82530180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82530180 size=5904
    let mut pc: u32 = 0x82530180;
    'dispatch: loop {
        match pc {
            0x82530180 => {
    //   block [0x82530180..0x82531890)
	// 82530180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82530184: 48004EFD  bl 0x82535080
	ctx.lr = 0x82530188;
	sub_82535080(ctx, base);
	// 82530188: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 8253018C: 48005E55  bl 0x82535fe0
	ctx.lr = 0x82530190;
	sub_82535FB0(ctx, base);
	// 82530190: 3980FF20  li r12, -0xe0
	ctx.r[12].s64 = -224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82531890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82531890 size=460
    let mut pc: u32 = 0x82531890;
    'dispatch: loop {
        match pc {
            0x82531890 => {
    //   block [0x82531890..0x82531948)
	// 82531890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82531894: 48003825  bl 0x825350b8
	ctx.lr = 0x82531898;
	sub_82535080(ctx, base);
	// 82531898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253189C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825318A0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825318A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825318A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825318AC: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 825318B0: 388000D4  li r4, 0xd4
	ctx.r[4].s64 = 212;
	// 825318B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825318B8: 83C90000  lwz r30, 0(r9)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825318BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825318C0: 4BF32779  bl 0x82464038
	ctx.lr = 0x825318C4;
	sub_82464038(ctx, base);
	// 825318C4: 396000D4  li r11, 0xd4
	ctx.r[11].s64 = 212;
	// 825318C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825318CC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825318D0: 4BFFDE99  bl 0x8252f768
	ctx.lr = 0x825318D4;
	sub_8252F768(ctx, base);
	// 825318D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825318D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825318DC: 48000185  bl 0x82531a60
	ctx.lr = 0x825318E0;
	sub_82531A60(ctx, base);
	// 825318E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825318E4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 825318E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825318EC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825318F0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825318F4: 807C003C  lwz r3, 0x3c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 825318F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825318FC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82531900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82531904: 4E800421  bctrl
	ctx.lr = 0x82531908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82531908: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8253190C: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82531910: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82531914: 807C009C  lwz r3, 0x9c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(156 as u32) ) } as u64;
	// 82531918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253191C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82531920: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82531924: 4E800421  bctrl
	ctx.lr = 0x82531928;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82531928: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8253192C: 907F009C  stw r3, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[3].u32 ) };
	// 82531930: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531934: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82531938: 419A0010  beq cr6, 0x82531948
	if ctx.cr[6].eq {
	pc = 0x82531948; continue 'dispatch;
	}
	// 8253193C: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531940: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82531944: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
            }
            0x82531948 => {
    //   block [0x82531948..0x82531964)
	// 82531948: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8253194C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531950: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82531954: 419A0010  beq cr6, 0x82531964
	if ctx.cr[6].eq {
	pc = 0x82531964; continue 'dispatch;
	}
	// 82531958: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8253195C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82531960: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82531964; continue 'dispatch;
            }
            0x82531964 => {
    //   block [0x82531964..0x82531980)
	// 82531964: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82531968: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253196C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82531970: 419A0010  beq cr6, 0x82531980
	if ctx.cr[6].eq {
	pc = 0x82531980; continue 'dispatch;
	}
	// 82531974: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531978: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253197C: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82531980; continue 'dispatch;
            }
            0x82531980 => {
    //   block [0x82531980..0x8253199C)
	// 82531980: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82531984: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531988: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253198C: 419A0010  beq cr6, 0x8253199c
	if ctx.cr[6].eq {
	pc = 0x8253199C; continue 'dispatch;
	}
	// 82531990: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531994: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82531998: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x8253199C; continue 'dispatch;
            }
            0x8253199C => {
    //   block [0x8253199C..0x825319B8)
	// 8253199C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 825319A0: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825319A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825319A8: 419A0010  beq cr6, 0x825319b8
	if ctx.cr[6].eq {
	pc = 0x825319B8; continue 'dispatch;
	}
	// 825319AC: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 825319B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825319B4: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x825319B8; continue 'dispatch;
            }
            0x825319B8 => {
    //   block [0x825319B8..0x825319D4)
	// 825319B8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 825319BC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825319C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825319C4: 419A0010  beq cr6, 0x825319d4
	if ctx.cr[6].eq {
	pc = 0x825319D4; continue 'dispatch;
	}
	// 825319C8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 825319CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825319D0: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x825319D4; continue 'dispatch;
            }
            0x825319D4 => {
    //   block [0x825319D4..0x825319F0)
	// 825319D4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 825319D8: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825319DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825319E0: 419A0010  beq cr6, 0x825319f0
	if ctx.cr[6].eq {
	pc = 0x825319F0; continue 'dispatch;
	}
	// 825319E4: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 825319E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825319EC: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x825319F0; continue 'dispatch;
            }
            0x825319F0 => {
    //   block [0x825319F0..0x82531A0C)
	// 825319F0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825319F4: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825319F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825319FC: 419A0010  beq cr6, 0x82531a0c
	if ctx.cr[6].eq {
	pc = 0x82531A0C; continue 'dispatch;
	}
	// 82531A00: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531A04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82531A08: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82531A0C; continue 'dispatch;
            }
            0x82531A0C => {
    //   block [0x82531A0C..0x82531A28)
	// 82531A0C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82531A10: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531A14: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82531A18: 419A0010  beq cr6, 0x82531a28
	if ctx.cr[6].eq {
	pc = 0x82531A28; continue 'dispatch;
	}
	// 82531A1C: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531A20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82531A24: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82531A28; continue 'dispatch;
            }
            0x82531A28 => {
    //   block [0x82531A28..0x82531A54)
	// 82531A28: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82531A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82531A30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82531A34: 419A0020  beq cr6, 0x82531a54
	if ctx.cr[6].eq {
	pc = 0x82531A54; continue 'dispatch;
	}
	// 82531A38: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82531A3C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531A40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82531A44: 419A0010  beq cr6, 0x82531a54
	if ctx.cr[6].eq {
	pc = 0x82531A54; continue 'dispatch;
	}
	// 82531A48: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531A4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82531A50: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82531A54; continue 'dispatch;
            }
            0x82531A54 => {
    //   block [0x82531A54..0x82531A5C)
	// 82531A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82531A58: 480036B0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82531A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82531A60 size=2068
    let mut pc: u32 = 0x82531A60;
    'dispatch: loop {
        match pc {
            0x82531A60 => {
    //   block [0x82531A60..0x82531B58)
	// 82531A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82531A64: 4800361D  bl 0x82535080
	ctx.lr = 0x82531A68;
	sub_82535080(ctx, base);
	// 82531A68: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82531A6C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82531A70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82531A74: 3BDC0048  addi r30, r28, 0x48
	ctx.r[30].s64 = ctx.r[28].s64 + 72;
	// 82531A78: 3BFD0048  addi r31, r29, 0x48
	ctx.r[31].s64 = ctx.r[29].s64 + 72;
	// 82531A7C: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531A80: 93A10114  stw r29, 0x114(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), ctx.r[29].u32 ) };
	// 82531A84: 9381011C  stw r28, 0x11c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), ctx.r[28].u32 ) };
	// 82531A88: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82531A8C: A17C0006  lhz r11, 6(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(6 as u32) ) } as u64;
	// 82531A90: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82531A94: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82531A98: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82531A9C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82531AA0: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82531AA4: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82531AA8: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82531AAC: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82531AB0: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82531AB4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82531AB8: 917D0018  stw r11, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82531ABC: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82531AC0: 917D001C  stw r11, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82531AC4: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82531AC8: 917D0020  stw r11, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82531ACC: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82531AD0: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82531AD4: 817C0028  lwz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82531AD8: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82531ADC: 817C002C  lwz r11, 0x2c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82531AE0: 917D002C  stw r11, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82531AE4: 817C0030  lwz r11, 0x30(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82531AE8: 917D0030  stw r11, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82531AEC: 817C0034  lwz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 82531AF0: 917D0034  stw r11, 0x34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82531AF4: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82531AF8: 917D0038  stw r11, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82531AFC: 817C003C  lwz r11, 0x3c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 82531B00: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82531B04: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82531B08: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82531B0C: 817C0044  lwz r11, 0x44(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) } as u64;
	// 82531B10: 917D0044  stw r11, 0x44(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82531B14: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82531B18: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531B1C: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82531B20: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82531B24: 40980070  bge cr6, 0x82531b94
	if !ctx.cr[6].lt {
	pc = 0x82531B94; continue 'dispatch;
	}
	// 82531B28: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82531B2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82531B30: 409A0028  bne cr6, 0x82531b58
	if !ctx.cr[6].eq {
	pc = 0x82531B58; continue 'dispatch;
	}
	// 82531B34: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82531B38: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82531B3C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82531B40: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82531B44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82531B48: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82531B4C: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82531B50: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82531B54: 4BF32565  bl 0x824640b8
	ctx.lr = 0x82531B58;
	sub_824640B8(ctx, base);
	pc = 0x82531B58; continue 'dispatch;
            }
            0x82531B58 => {
    //   block [0x82531B58..0x82531B94)
	// 82531B58: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82531B5C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82531B60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531B64: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82531B68: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82531B6C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82531B70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82531B74: 55643032  slwi r4, r11, 6
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82531B78: 4BF324C1  bl 0x82464038
	ctx.lr = 0x82531B7C;
	sub_82464038(ctx, base);
	// 82531B7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82531B80: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82531B84: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531B88: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82531B8C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82531B90: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82531B94; continue 'dispatch;
            }
            0x82531B94 => {
    //   block [0x82531B94..0x82531F48)
	// 82531B94: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82531B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82531B9C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82531BA0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82531BA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82531BA8: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82531BAC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82531BB0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82531BB4: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82531BB8: 41980390  blt cr6, 0x82531f48
	if ctx.cr[6].lt {
	pc = 0x82531F48; continue 'dispatch;
	}
	// 82531BBC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82531BC0: 394700E4  addi r10, r7, 0xe4
	ctx.r[10].s64 = ctx.r[7].s64 + 228;
	// 82531BC4: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82531BC8: 396800D0  addi r11, r8, 0xd0
	ctx.r[11].s64 = ctx.r[8].s64 + 208;
	// 82531BCC: 7D083850  subf r8, r8, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82531BD0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82531BD4: 39E00110  li r15, 0x110
	ctx.r[15].s64 = 272;
	// 82531BD8: 3A00010C  li r16, 0x10c
	ctx.r[16].s64 = 268;
	// 82531BDC: 3A200120  li r17, 0x120
	ctx.r[17].s64 = 288;
	// 82531BE0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82531BE4: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82531BE8: 3A40011C  li r18, 0x11c
	ctx.r[18].s64 = 284;
	// 82531BEC: 3A600130  li r19, 0x130
	ctx.r[19].s64 = 304;
	// 82531BF0: 3A80012C  li r20, 0x12c
	ctx.r[20].s64 = 300;
	// 82531BF4: 3AA00140  li r21, 0x140
	ctx.r[21].s64 = 320;
	// 82531BF8: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82531BFC: 3AC0015C  li r22, 0x15c
	ctx.r[22].s64 = 348;
	// 82531C00: 3AE00170  li r23, 0x170
	ctx.r[23].s64 = 368;
	// 82531C04: 3B00016C  li r24, 0x16c
	ctx.r[24].s64 = 364;
	// 82531C08: 3B200180  li r25, 0x180
	ctx.r[25].s64 = 384;
	// 82531C0C: 3B40018C  li r26, 0x18c
	ctx.r[26].s64 = 396;
	// 82531C10: 3B6001A0  li r27, 0x1a0
	ctx.r[27].s64 = 416;
	// 82531C14: 3B80019C  li r28, 0x19c
	ctx.r[28].s64 = 412;
	// 82531C18: 3BA001B0  li r29, 0x1b0
	ctx.r[29].s64 = 432;
	// 82531C1C: 3BC001BC  li r30, 0x1bc
	ctx.r[30].s64 = 444;
	// 82531C20: 3BE001D0  li r31, 0x1d0
	ctx.r[31].s64 = 464;
	// 82531C24: 386001CC  li r3, 0x1cc
	ctx.r[3].s64 = 460;
	// 82531C28: 388001E0  li r4, 0x1e0
	ctx.r[4].s64 = 480;
	// 82531C2C: 38A001DC  li r5, 0x1dc
	ctx.r[5].s64 = 476;
	// 82531C30: 38C001F0  li r6, 0x1f0
	ctx.r[6].s64 = 496;
	// 82531C34: 38E001EC  li r7, 0x1ec
	ctx.r[7].s64 = 492;
	// 82531C38: 39000200  li r8, 0x200
	ctx.r[8].s64 = 512;
	// 82531C3C: 39C0FF1C  li r14, -0xe4
	ctx.r[14].s64 = -228;
	pc = 0x82531F48; continue 'dispatch;
            }
            0x82531F48 => {
    //   block [0x82531F48..0x82532274)
	// 82531F48: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82531F4C: 40980104  bge cr6, 0x82532050
	if !ctx.cr[6].lt {
	pc = 0x82532050; continue 'dispatch;
	}
	// 82531F50: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82531F54: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82531F58: 7CC83850  subf r6, r8, r7
	ctx.r[6].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82531F5C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82531F60: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82531F64: 7D4B3A14  add r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82531F68: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82531F6C: 394A0024  addi r10, r10, 0x24
	ctx.r[10].s64 = ctx.r[10].s64 + 36;
	// 82531F70: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82531F74: 3900FFDC  li r8, -0x24
	ctx.r[8].s64 = -36;
	// 82531F78: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82531F7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825322B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825322B0 size=204
    let mut pc: u32 = 0x825322B0;
    'dispatch: loop {
        match pc {
            0x825322B0 => {
    //   block [0x825322B0..0x8253237C)
	// 825322B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825322B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825322B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825322BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825322C0: 83E50018  lwz r31, 0x18(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 825322C4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825322C8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825322CC: 395F01B0  addi r10, r31, 0x1b0
	ctx.r[10].s64 = ctx.r[31].s64 + 432;
	// 825322D0: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 825322D4: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 825322D8: F90B0000  std r8, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 825322DC: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 825322E0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82532380 size=140
    let mut pc: u32 = 0x82532380;
    'dispatch: loop {
        match pc {
            0x82532380 => {
    //   block [0x82532380..0x8253240C)
	// 82532380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532384: 48002D39  bl 0x825350bc
	ctx.lr = 0x82532388;
	sub_82535080(ctx, base);
	// 82532388: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253238C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82532390: C0060008  lfs f0, 8(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82532394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82532398: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8253239C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825323A0: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 825323A4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825323A8: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825323AC: 38AA0040  addi r5, r10, 0x40
	ctx.r[5].s64 = ctx.r[10].s64 + 64;
	// 825323B0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825323B4: 388B00E0  addi r4, r11, 0xe0
	ctx.r[4].s64 = ctx.r[11].s64 + 224;
	// 825323B8: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825323BC: 480797E5  bl 0x825abba0
	ctx.lr = 0x825323C0;
	sub_825ABBA0(ctx, base);
	// 825323C0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 825323C4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825323C8: 394001A0  li r10, 0x1a0
	ctx.r[10].s64 = 416;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82532410 size=152
    let mut pc: u32 = 0x82532410;
    'dispatch: loop {
        match pc {
            0x82532410 => {
    //   block [0x82532410..0x82532458)
	// 82532410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8253241C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82532420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82532428: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8253242C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82532430: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82532434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82532438: 4E800421  bctrl
	ctx.lr = 0x8253243C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253243C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82532440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82532444: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82532448: 40990048  ble cr6, 0x82532490
	if !ctx.cr[6].gt {
	pc = 0x82532490; continue 'dispatch;
	}
	// 8253244C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82532450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82532454: C0091FF8  lfs f0, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            0x82532458 => {
    //   block [0x82532458..0x82532478)
	// 82532458: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253245C: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82532460: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82532464: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82532468: 419A0010  beq cr6, 0x82532478
	if ctx.cr[6].eq {
	pc = 0x82532478; continue 'dispatch;
	}
	// 8253246C: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82532470: 7DA9552E  stfsx f13, r9, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82532474: 48000008  b 0x8253247c
	pc = 0x8253247C; continue 'dispatch;
            }
            0x82532478 => {
    //   block [0x82532478..0x8253247C)
	// 82532478: 7C09552E  stfsx f0, r9, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	pc = 0x8253247C; continue 'dispatch;
            }
            0x8253247C => {
    //   block [0x8253247C..0x82532490)
	// 8253247C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82532480: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82532484: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82532488: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8253248C: 4198FFCC  blt cr6, 0x82532458
	if ctx.cr[6].lt {
	pc = 0x82532458; continue 'dispatch;
	}
	pc = 0x82532490; continue 'dispatch;
            }
            0x82532490 => {
    //   block [0x82532490..0x825324A8)
	// 82532490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82532494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253249C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825324A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825324A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825324A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825324A8 size=60
    let mut pc: u32 = 0x825324A8;
    'dispatch: loop {
        match pc {
            0x825324A8 => {
    //   block [0x825324A8..0x825324E4)
	// 825324A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825324AC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825324B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825324B4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 825324B8: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825324BC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825324C0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825324C4: 396B6C7C  addi r11, r11, 0x6c7c
	ctx.r[11].s64 = ctx.r[11].s64 + 27772;
	// 825324C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825324CC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825324D0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 825324D4: 91030018  stw r8, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 825324D8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825324DC: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825324E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825324E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825324E8 size=44
    let mut pc: u32 = 0x825324E8;
    'dispatch: loop {
        match pc {
            0x825324E8 => {
    //   block [0x825324E8..0x82532514)
	// 825324E8: C1A3000C  lfs f13, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825324EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825324F0: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825324F4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825324F8: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825324FC: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82532500: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82532504: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82532508: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8253250C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82532510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82532518 size=64
    let mut pc: u32 = 0x82532518;
    'dispatch: loop {
        match pc {
            0x82532518 => {
    //   block [0x82532518..0x82532558)
	// 82532518: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8253251C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82532520: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82532524: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82532528: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8253252C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82532530: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82532534: 394A6D24  addi r10, r10, 0x6d24
	ctx.r[10].s64 = ctx.r[10].s64 + 27940;
	// 82532538: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8253253C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82532540: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82532544: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82532558 size=48
    let mut pc: u32 = 0x82532558;
    'dispatch: loop {
        match pc {
            0x82532558 => {
    //   block [0x82532558..0x82532588)
	// 82532558: FD600A10  fabs f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 8253255C: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82532560: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82532564: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82532568: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8253256C: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82532570: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82532574: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82532578: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8253257C: C00B2048  lfs f0, 0x2048(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8264 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82532580: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82532584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532588 size=340
    let mut pc: u32 = 0x82532588;
    'dispatch: loop {
        match pc {
            0x82532588 => {
    //   block [0x82532588..0x825326DC)
	// 82532588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253258C: 48002B2D  bl 0x825350b8
	ctx.lr = 0x82532590;
	sub_82535080(ctx, base);
	// 82532590: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82532594: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825326E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825326E0 size=168
    let mut pc: u32 = 0x825326E0;
    'dispatch: loop {
        match pc {
            0x825326E0 => {
    //   block [0x825326E0..0x82532720)
	// 825326E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825326E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825326E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825326EC: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 825326F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825326F4: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 825326F8: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 825326FC: 409A0024  bne cr6, 0x82532720
	if !ctx.cr[6].eq {
	pc = 0x82532720; continue 'dispatch;
	}
	// 82532700: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82532704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82532708: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8253270C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82532710: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 82532714: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82532718: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8253271C: 48000008  b 0x82532724
	pc = 0x82532724; continue 'dispatch;
            }
            0x82532720 => {
    //   block [0x82532720..0x82532724)
	// 82532720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	pc = 0x82532724; continue 'dispatch;
            }
            0x82532724 => {
    //   block [0x82532724..0x82532760)
	// 82532724: 3C800007  lis r4, 7
	ctx.r[4].s64 = 458752;
	// 82532728: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8253272C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82532730: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82532734: 6084001A  ori r4, r4, 0x1a
	ctx.r[4].u64 = ctx.r[4].u64 | 26;
	// 82532738: 386000FA  li r3, 0xfa
	ctx.r[3].s64 = 250;
	// 8253273C: 481DB591  bl 0x8270dccc
	ctx.lr = 0x82532740;
	// extern call 0x8270DCCC → crate::xboxkrnl::XMsgStartIORequestEx
	crate::xboxkrnl::XMsgStartIORequestEx(ctx, base);
	// 82532740: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82532744: 4180001C  blt 0x82532760
	if ctx.cr[0].lt {
	pc = 0x82532760; continue 'dispatch;
	}
	// 82532748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253274C: 4BE8DAE5  bl 0x823c0230
	ctx.lr = 0x82532750;
	sub_823C0230(ctx, base);
	// 82532750: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82532754: 4180000C  blt 0x82532760
	if ctx.cr[0].lt {
	pc = 0x82532760; continue 'dispatch;
	}
	// 82532758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253275C: 4800001C  b 0x82532778
	pc = 0x82532778; continue 'dispatch;
            }
            0x82532760 => {
    //   block [0x82532760..0x82532778)
	// 82532760: 546B00DE  rlwinm r11, r3, 0, 3, 0xf
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82532764: 3D400007  lis r10, 7
	ctx.r[10].s64 = 458752;
	// 82532768: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8253276C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82532770: 419A0008  beq cr6, 0x82532778
	if ctx.cr[6].eq {
	pc = 0x82532778; continue 'dispatch;
	}
	// 82532774: 3860065B  li r3, 0x65b
	ctx.r[3].s64 = 1627;
	pc = 0x82532778; continue 'dispatch;
            }
            0x82532778 => {
    //   block [0x82532778..0x82532788)
	// 82532778: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253277C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532788 size=116
    let mut pc: u32 = 0x82532788;
    'dispatch: loop {
        match pc {
            0x82532788 => {
    //   block [0x82532788..0x825327D4)
	// 82532788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253278C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532794: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82532798: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253279C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 825327A0: 3C800007  lis r4, 7
	ctx.r[4].s64 = 458752;
	// 825327A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825327A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825327AC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825327B0: 6084001B  ori r4, r4, 0x1b
	ctx.r[4].u64 = ctx.r[4].u64 | 27;
	// 825327B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825327B8: 386000FA  li r3, 0xfa
	ctx.r[3].s64 = 250;
	// 825327BC: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 825327C0: 481DB4BD  bl 0x8270dc7c
	ctx.lr = 0x825327C4;
	// extern call 0x8270DC7C → crate::xboxkrnl::XMsgInProcessCall
	crate::xboxkrnl::XMsgInProcessCall(ctx, base);
	// 825327C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825327C8: 4180000C  blt 0x825327d4
	if ctx.cr[0].lt {
	pc = 0x825327D4; continue 'dispatch;
	}
	// 825327CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825327D0: 4800001C  b 0x825327ec
	pc = 0x825327EC; continue 'dispatch;
            }
            0x825327D4 => {
    //   block [0x825327D4..0x825327EC)
	// 825327D4: 546B00DE  rlwinm r11, r3, 0, 3, 0xf
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 825327D8: 3D400007  lis r10, 7
	ctx.r[10].s64 = 458752;
	// 825327DC: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 825327E0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825327E4: 419A0008  beq cr6, 0x825327ec
	if ctx.cr[6].eq {
	pc = 0x825327EC; continue 'dispatch;
	}
	// 825327E8: 3860065B  li r3, 0x65b
	ctx.r[3].s64 = 1627;
	pc = 0x825327EC; continue 'dispatch;
            }
            0x825327EC => {
    //   block [0x825327EC..0x825327FC)
	// 825327EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825327F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825327F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825327F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532800 size=68
    let mut pc: u32 = 0x82532800;
    'dispatch: loop {
        match pc {
            0x82532800 => {
    //   block [0x82532800..0x8253282C)
	// 82532800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253280C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532810: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82532814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82532818: 396B6DF8  addi r11, r11, 0x6df8
	ctx.r[11].s64 = ctx.r[11].s64 + 28152;
	// 8253281C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82532820: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82532824: 41820008  beq 0x8253282c
	if ctx.cr[0].eq {
	pc = 0x8253282C; continue 'dispatch;
	}
	// 82532828: 48000391  bl 0x82532bb8
	ctx.lr = 0x8253282C;
	sub_82532BB8(ctx, base);
	pc = 0x8253282C; continue 'dispatch;
            }
            0x8253282C => {
    //   block [0x8253282C..0x82532844)
	// 8253282C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82532830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82532834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253283C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82532840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532848 size=216
    let mut pc: u32 = 0x82532848;
    'dispatch: loop {
        match pc {
            0x82532848 => {
    //   block [0x82532848..0x82532894)
	// 82532848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253284C: 4800286D  bl 0x825350b8
	ctx.lr = 0x82532850;
	sub_82535080(ctx, base);
	// 82532850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532854: 481DA879  bl 0x8270d0cc
	ctx.lr = 0x82532858;
	// extern call 0x8270D0CC → crate::xam::XamGetSystemVersion
	crate::xam::XamGetSystemVersion(ctx, base);
	// 82532858: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 8253285C: 546B022E  rlwinm r11, r3, 0, 8, 0x17
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82532860: 614AA100  ori r10, r10, 0xa100
	ctx.r[10].u64 = ctx.r[10].u64 | 41216;
	// 82532864: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82532868: 40980098  bge cr6, 0x82532900
	if !ctx.cr[6].lt {
	pc = 0x82532900; continue 'dispatch;
	}
	// 8253286C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532870: 3B8BE660  addi r28, r11, -0x19a0
	ctx.r[28].s64 = ctx.r[11].s64 + -6560;
	// 82532874: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82532878: 481DA9E5  bl 0x8270d25c
	ctx.lr = 0x8253287C;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8253287C: 3FA0829A  lis r29, -0x7d66
	ctx.r[29].s64 = -2103836672;
	// 82532880: 817D2D24  lwz r11, 0x2d24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(11556 as u32) ) } as u64;
	// 82532884: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82532888: 409A000C  bne cr6, 0x82532894
	if !ctx.cr[6].eq {
	pc = 0x82532894; continue 'dispatch;
	}
	// 8253288C: 3BC0065B  li r30, 0x65b
	ctx.r[30].s64 = 1627;
	// 82532890: 48000064  b 0x825328f4
	pc = 0x825328F4; continue 'dispatch;
            }
            0x82532894 => {
    //   block [0x82532894..0x825328E8)
	// 82532894: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82532898: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253289C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825328A0: 4BFFFEE9  bl 0x82532788
	ctx.lr = 0x825328A4;
	sub_82532788(ctx, base);
	// 825328A4: 3FE0829A  lis r31, -0x7d66
	ctx.r[31].s64 = -2103836672;
	// 825328A8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825328AC: 4082003C  bne 0x825328e8
	if !ctx.cr[0].eq {
	pc = 0x825328E8; continue 'dispatch;
	}
	// 825328B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825328B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825328B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825328BC: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825328C0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825328C4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825328C8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825328CC: 917F2D28  stw r11, 0x2d28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11560 as u32), ctx.r[11].u32 ) };
	// 825328D0: 4BFFFE11  bl 0x825326e0
	ctx.lr = 0x825328D4;
	sub_825326E0(ctx, base);
	// 825328D4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825328D8: 40820010  bne 0x825328e8
	if !ctx.cr[0].eq {
	pc = 0x825328E8; continue 'dispatch;
	}
	// 825328DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825328E0: 917D2D24  stw r11, 0x2d24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(11556 as u32), ctx.r[11].u32 ) };
	// 825328E4: 48000010  b 0x825328f4
	pc = 0x825328F4; continue 'dispatch;
            }
            0x825328E8 => {
    //   block [0x825328E8..0x825328F4)
	// 825328E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825328EC: 917D2D24  stw r11, 0x2d24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(11556 as u32), ctx.r[11].u32 ) };
	// 825328F0: 917F2D28  stw r11, 0x2d28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11560 as u32), ctx.r[11].u32 ) };
	pc = 0x825328F4; continue 'dispatch;
            }
            0x825328F4 => {
    //   block [0x825328F4..0x82532900)
	// 825328F4: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 825328F8: 481DA975  bl 0x8270d26c
	ctx.lr = 0x825328FC;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825328FC: 48000018  b 0x82532914
	pc = 0x82532914; continue 'dispatch;
            }
            0x82532900 => {
    //   block [0x82532900..0x82532914)
	// 82532900: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82532904: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82532908: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8253290C: 4BFFFDD5  bl 0x825326e0
	ctx.lr = 0x82532910;
	sub_825326E0(ctx, base);
	// 82532910: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	pc = 0x82532914; continue 'dispatch;
            }
            0x82532914 => {
    //   block [0x82532914..0x82532920)
	// 82532914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82532918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8253291C: 480027EC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532920 size=172
    let mut pc: u32 = 0x82532920;
    'dispatch: loop {
        match pc {
            0x82532920 => {
    //   block [0x82532920..0x8253296C)
	// 82532920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532924: 48002795  bl 0x825350b8
	ctx.lr = 0x82532928;
	sub_82535080(ctx, base);
	// 82532928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253292C: 481DA7A1  bl 0x8270d0cc
	ctx.lr = 0x82532930;
	// extern call 0x8270D0CC → crate::xam::XamGetSystemVersion
	crate::xam::XamGetSystemVersion(ctx, base);
	// 82532930: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 82532934: 546B022E  rlwinm r11, r3, 0, 8, 0x17
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82532938: 614AA100  ori r10, r10, 0xa100
	ctx.r[10].u64 = ctx.r[10].u64 | 41216;
	// 8253293C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82532940: 4098006C  bge cr6, 0x825329ac
	if !ctx.cr[6].lt {
	pc = 0x825329AC; continue 'dispatch;
	}
	// 82532944: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532948: 3B8BE660  addi r28, r11, -0x19a0
	ctx.r[28].s64 = ctx.r[11].s64 + -6560;
	// 8253294C: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82532950: 481DA90D  bl 0x8270d25c
	ctx.lr = 0x82532954;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82532954: 3FE0829A  lis r31, -0x7d66
	ctx.r[31].s64 = -2103836672;
	// 82532958: 817F2D24  lwz r11, 0x2d24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11556 as u32) ) } as u64;
	// 8253295C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82532960: 409A000C  bne cr6, 0x8253296c
	if !ctx.cr[6].eq {
	pc = 0x8253296C; continue 'dispatch;
	}
	// 82532964: 3BA0065B  li r29, 0x65b
	ctx.r[29].s64 = 1627;
	// 82532968: 48000038  b 0x825329a0
	pc = 0x825329A0; continue 'dispatch;
            }
            0x8253296C => {
    //   block [0x8253296C..0x8253298C)
	// 8253296C: 3FC0829A  lis r30, -0x7d66
	ctx.r[30].s64 = -2103836672;
	// 82532970: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82532974: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82532978: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253297C: 817E2D28  lwz r11, 0x2d28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(11560 as u32) ) } as u64;
	// 82532980: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82532984: 419A0008  beq cr6, 0x8253298c
	if ctx.cr[6].eq {
	pc = 0x8253298C; continue 'dispatch;
	}
	// 82532988: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	pc = 0x8253298C; continue 'dispatch;
            }
            0x8253298C => {
    //   block [0x8253298C..0x825329A0)
	// 8253298C: 4BFFFD55  bl 0x825326e0
	ctx.lr = 0x82532990;
	sub_825326E0(ctx, base);
	// 82532990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82532994: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82532998: 917E2D28  stw r11, 0x2d28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(11560 as u32), ctx.r[11].u32 ) };
	// 8253299C: 917F2D24  stw r11, 0x2d24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11556 as u32), ctx.r[11].u32 ) };
	pc = 0x825329A0; continue 'dispatch;
            }
            0x825329A0 => {
    //   block [0x825329A0..0x825329AC)
	// 825329A0: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 825329A4: 481DA8C9  bl 0x8270d26c
	ctx.lr = 0x825329A8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 825329A8: 48000018  b 0x825329c0
	pc = 0x825329C0; continue 'dispatch;
            }
            0x825329AC => {
    //   block [0x825329AC..0x825329C0)
	// 825329AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825329B0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825329B4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825329B8: 4BFFFD29  bl 0x825326e0
	ctx.lr = 0x825329BC;
	sub_825326E0(ctx, base);
	// 825329BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	pc = 0x825329C0; continue 'dispatch;
            }
            0x825329C0 => {
    //   block [0x825329C0..0x825329CC)
	// 825329C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825329C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825329C8: 48002740  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825329D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825329D0 size=84
    let mut pc: u32 = 0x825329D0;
    'dispatch: loop {
        match pc {
            0x825329D0 => {
    //   block [0x825329D0..0x82532A08)
	// 825329D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825329D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825329D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825329DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825329E0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825329E4: 48006FD5  bl 0x825399b8
	ctx.lr = 0x825329E8;
	sub_825399B8(ctx, base);
	// 825329E8: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 825329EC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825329F0: 916A3DF0  stw r11, 0x3df0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15856 as u32), ctx.r[11].u32 ) };
	// 825329F4: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 825329F8: 916A3DEC  stw r11, 0x3dec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15852 as u32), ctx.r[11].u32 ) };
	// 825329FC: 4082000C  bne 0x82532a08
	if !ctx.cr[0].eq {
	pc = 0x82532A08; continue 'dispatch;
	}
	// 82532A00: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82532A04: 48000010  b 0x82532a14
	pc = 0x82532A14; continue 'dispatch;
            }
            0x82532A08 => {
    //   block [0x82532A08..0x82532A14)
	// 82532A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82532A0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82532A10: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82532A14; continue 'dispatch;
            }
            0x82532A14 => {
    //   block [0x82532A14..0x82532A24)
	// 82532A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82532A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532A30 size=264
    let mut pc: u32 = 0x82532A30;
    'dispatch: loop {
        match pc {
            0x82532A30 => {
    //   block [0x82532A30..0x82532A98)
	// 82532A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532A34: 48002671  bl 0x825350a4
	ctx.lr = 0x82532A38;
	sub_82535080(ctx, base);
	// 82532A38: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82532A3C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532A40: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82532A44: 48002E5D  bl 0x825358a0
	ctx.lr = 0x82532A48;
	sub_825358A0(ctx, base);
	// 82532A48: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82532A4C: 3F208313  lis r25, -0x7ced
	ctx.r[25].s64 = -2095906816;
	// 82532A50: 83993DF0  lwz r28, 0x3df0(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(15856 as u32) ) } as u64;
	// 82532A54: 3F008313  lis r24, -0x7ced
	ctx.r[24].s64 = -2095906816;
	// 82532A58: 83D83DEC  lwz r30, 0x3dec(r24)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(15852 as u32) ) } as u64;
	// 82532A5C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82532A60: 41980094  blt cr6, 0x82532af4
	if ctx.cr[6].lt {
	pc = 0x82532AF4; continue 'dispatch;
	}
	// 82532A64: 7F5CF050  subf r26, r28, r30
	ctx.r[26].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 82532A68: 3B7A0004  addi r27, r26, 4
	ctx.r[27].s64 = ctx.r[26].s64 + 4;
	// 82532A6C: 2B1B0004  cmplwi cr6, r27, 4
	ctx.cr[6].compare_u32(ctx.r[27].u32, 4 as u32, &mut ctx.xer);
	// 82532A70: 41980084  blt cr6, 0x82532af4
	if ctx.cr[6].lt {
	pc = 0x82532AF4; continue 'dispatch;
	}
	// 82532A74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82532A78: 48006DF9  bl 0x82539870
	ctx.lr = 0x82532A7C;
	sub_82539870(ctx, base);
	// 82532A7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82532A80: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82532A84: 4098005C  bge cr6, 0x82532ae0
	if !ctx.cr[6].lt {
	pc = 0x82532AE0; continue 'dispatch;
	}
	// 82532A88: 2B1D0800  cmplwi cr6, r29, 0x800
	ctx.cr[6].compare_u32(ctx.r[29].u32, 2048 as u32, &mut ctx.xer);
	// 82532A8C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82532A90: 41980008  blt cr6, 0x82532a98
	if ctx.cr[6].lt {
	pc = 0x82532A98; continue 'dispatch;
	}
	// 82532A94: 39600800  li r11, 0x800
	ctx.r[11].s64 = 2048;
	pc = 0x82532A98; continue 'dispatch;
            }
            0x82532A98 => {
    //   block [0x82532A98..0x82532AB4)
	// 82532A98: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82532A9C: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82532AA0: 41980014  blt cr6, 0x82532ab4
	if ctx.cr[6].lt {
	pc = 0x82532AB4; continue 'dispatch;
	}
	// 82532AA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82532AA8: 48006C21  bl 0x825396c8
	ctx.lr = 0x82532AAC;
	sub_825396C8(ctx, base);
	// 82532AAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82532AB0: 40820020  bne 0x82532ad0
	if !ctx.cr[0].eq {
	pc = 0x82532AD0; continue 'dispatch;
	}
	pc = 0x82532AB4; continue 'dispatch;
            }
            0x82532AB4 => {
    //   block [0x82532AB4..0x82532AD0)
	// 82532AB4: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 82532AB8: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82532ABC: 41980038  blt cr6, 0x82532af4
	if ctx.cr[6].lt {
	pc = 0x82532AF4; continue 'dispatch;
	}
	// 82532AC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82532AC4: 48006C05  bl 0x825396c8
	ctx.lr = 0x82532AC8;
	sub_825396C8(ctx, base);
	// 82532AC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82532ACC: 41820028  beq 0x82532af4
	if ctx.cr[0].eq {
	pc = 0x82532AF4; continue 'dispatch;
	}
	pc = 0x82532AD0; continue 'dispatch;
            }
            0x82532AD0 => {
    //   block [0x82532AD0..0x82532AE0)
	// 82532AD0: 90793DF0  stw r3, 0x3df0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(15856 as u32), ctx.r[3].u32 ) };
	// 82532AD4: 7F4B1670  srawi r11, r26, 2
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[26].s32 >> 2) as i64;
	// 82532AD8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82532ADC: 7FCB1A14  add r30, r11, r3
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	pc = 0x82532AE0; continue 'dispatch;
            }
            0x82532AE0 => {
    //   block [0x82532AE0..0x82532AF4)
	// 82532AE0: 92FE0000  stw r23, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82532AE4: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82532AE8: 91783DEC  stw r11, 0x3dec(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(15852 as u32), ctx.r[11].u32 ) };
	// 82532AEC: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 82532AF0: 4800000C  b 0x82532afc
	pc = 0x82532AFC; continue 'dispatch;
            }
            0x82532AF4 => {
    //   block [0x82532AF4..0x82532AFC)
	// 82532AF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82532AF8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	pc = 0x82532AFC; continue 'dispatch;
            }
            0x82532AFC => {
    //   block [0x82532AFC..0x82532B14)
	// 82532AFC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82532B00: 399F00B0  addi r12, r31, 0xb0
	ctx.r[12].s64 = ctx.r[31].s64 + 176;
	// 82532B04: 48000011  bl 0x82532b14
	ctx.lr = 0x82532B08;
	sub_82532A30(ctx, base);
	// 82532B08: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82532B0C: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82532B10: 480025E4  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            0x82532B14 => {
    //   block [0x82532B14..0x82532B38)
	// 82532B14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532B18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532B1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532B20: 48002D89  bl 0x825358a8
	ctx.lr = 0x82532B24;
	sub_825358A8(ctx, base);
	// 82532B24: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82532B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532B38 size=44
    let mut pc: u32 = 0x82532B38;
    'dispatch: loop {
        match pc {
            0x82532B38 => {
    //   block [0x82532B38..0x82532B64)
	// 82532B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532B44: 4BFFFEED  bl 0x82532a30
	ctx.lr = 0x82532B48;
	sub_82532A30(ctx, base);
	// 82532B48: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82532B4C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82532B50: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 82532B54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82532B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532B68 size=76
    let mut pc: u32 = 0x82532B68;
    'dispatch: loop {
        match pc {
            0x82532B68 => {
    //   block [0x82532B68..0x82532B8C)
	// 82532B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532B74: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82532B78: 816B2D6C  lwz r11, 0x2d6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11628 as u32) ) } as u64;
	// 82532B7C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82532B80: 4182000C  beq 0x82532b8c
	if ctx.cr[0].eq {
	pc = 0x82532B8C; continue 'dispatch;
	}
	// 82532B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82532B88: 4E800421  bctrl
	ctx.lr = 0x82532B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82532B8C => {
    //   block [0x82532B8C..0x82532BB4)
	// 82532B8C: 38600019  li r3, 0x19
	ctx.r[3].s64 = 25;
	// 82532B90: 48006E91  bl 0x82539a20
	ctx.lr = 0x82532B94;
	sub_82539A20(ctx, base);
	// 82532B94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82532B98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82532B9C: 48003D35  bl 0x825368d0
	ctx.lr = 0x82532BA0;
	sub_825368D0(ctx, base);
	// 82532BA0: 48003C81  bl 0x82536820
	ctx.lr = 0x82532BA4;
	sub_82536820(ctx, base);
	// 82532BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82532BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532BB8 size=8
    let mut pc: u32 = 0x82532BB8;
    'dispatch: loop {
        match pc {
            0x82532BB8 => {
    //   block [0x82532BB8..0x82532BC0)
	// 82532BB8: 48001180  b 0x82533d38
	sub_82533D38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532BC0 size=92
    let mut pc: u32 = 0x82532BC0;
    'dispatch: loop {
        match pc {
            0x82532BC0 => {
    //   block [0x82532BC0..0x82532BCC)
	// 82532BC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82532BC4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82532BC8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82532BCC; continue 'dispatch;
            }
            0x82532BCC => {
    //   block [0x82532BCC..0x82532BEC)
	// 82532BCC: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82532BD0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82532BD4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82532BD8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82532BDC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82532BE0: 4182000C  beq 0x82532bec
	if ctx.cr[0].eq {
	pc = 0x82532BEC; continue 'dispatch;
	}
	// 82532BE4: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82532BE8: 4082FFE4  bne 0x82532bcc
	if !ctx.cr[0].eq {
	pc = 0x82532BCC; continue 'dispatch;
	}
	pc = 0x82532BEC; continue 'dispatch;
            }
            0x82532BEC => {
    //   block [0x82532BEC..0x82532C0C)
	// 82532BEC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82532BF0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82532BF4: 3545FFFF  addic. r10, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82532BF8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82532BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82532C00: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82532C04: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82532C08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82532C0C; continue 'dispatch;
            }
            0x82532C0C => {
    //   block [0x82532C0C..0x82532C1C)
	// 82532C0C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82532C10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82532C14: 4200FFF8  bdnz 0x82532c0c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82532C0C; continue 'dispatch;
	}
	// 82532C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532C20 size=236
    let mut pc: u32 = 0x82532C20;
    'dispatch: loop {
        match pc {
            0x82532C20 => {
    //   block [0x82532C20..0x82532C54)
	// 82532C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532C28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82532C2C: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82532C30: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82532C34: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82532C38: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82532C3C: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82532C40: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82532C44: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532C48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82532C4C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82532C50: 409A0034  bne cr6, 0x82532c84
	if !ctx.cr[6].eq {
	pc = 0x82532C84; continue 'dispatch;
	}
	pc = 0x82532C54; continue 'dispatch;
            }
            0x82532C54 => {
    //   block [0x82532C54..0x82532C84)
	// 82532C54: 48007CE5  bl 0x8253a938
	ctx.lr = 0x82532C58;
	sub_8253A938(ctx, base);
	// 82532C58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82532C5C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82532C60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82532C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82532C68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82532C6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82532C70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82532C74: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82532C78: 48007B89  bl 0x8253a800
	ctx.lr = 0x82532C7C;
	sub_8253A800(ctx, base);
	// 82532C7C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82532C80: 48000078  b 0x82532cf8
	pc = 0x82532CF8; continue 'dispatch;
            }
            0x82532C84 => {
    //   block [0x82532C84..0x82532CE8)
	// 82532C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82532C88: 419AFFCC  beq cr6, 0x82532c54
	if ctx.cr[6].eq {
	pc = 0x82532C54; continue 'dispatch;
	}
	// 82532C8C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82532C90: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82532C94: 392100B0  addi r9, r1, 0xb0
	ctx.r[9].s64 = ctx.r[1].s64 + 176;
	// 82532C98: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82532C9C: 3D007FFF  lis r8, 0x7fff
	ctx.r[8].s64 = 2147418112;
	// 82532CA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82532CA4: 6108FFFF  ori r8, r8, 0xffff
	ctx.r[8].u64 = ctx.r[8].u64 | 65535;
	// 82532CA8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82532CAC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82532CB0: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 82532CB4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82532CB8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82532CBC: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82532CC0: 48006FB9  bl 0x82539c78
	ctx.lr = 0x82532CC4;
	sub_82539C78(ctx, base);
	// 82532CC4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82532CC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82532CCC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82532CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82532CD4: 41800014  blt 0x82532ce8
	if ctx.cr[0].lt {
	pc = 0x82532CE8; continue 'dispatch;
	}
	// 82532CD8: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82532CDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82532CE0: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82532CE4: 48000010  b 0x82532cf4
	pc = 0x82532CF4; continue 'dispatch;
            }
            0x82532CE8 => {
    //   block [0x82532CE8..0x82532CF4)
	// 82532CE8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82532CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82532CF0: 48006DB9  bl 0x82539aa8
	ctx.lr = 0x82532CF4;
	sub_82539AA8(ctx, base);
	pc = 0x82532CF4; continue 'dispatch;
            }
            0x82532CF4 => {
    //   block [0x82532CF4..0x82532CF8)
	// 82532CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82532CF8; continue 'dispatch;
            }
            0x82532CF8 => {
    //   block [0x82532CF8..0x82532D0C)
	// 82532CF8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82532CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82532D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532D10 size=72
    let mut pc: u32 = 0x82532D10;
    'dispatch: loop {
        match pc {
            0x82532D10 => {
    //   block [0x82532D10..0x82532D58)
	// 82532D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532D18: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82532D1C: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82532D20: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82532D24: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82532D28: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82532D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532D30: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82532D34: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82532D38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82532D3C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82532D40: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82532D44: 48004D35  bl 0x82537a78
	ctx.lr = 0x82532D48;
	sub_82537A78(ctx, base);
	// 82532D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82532D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82532D58 size=192
    let mut pc: u32 = 0x82532D58;
    'dispatch: loop {
        match pc {
            0x82532D58 => {
    //   block [0x82532D58..0x82532D74)
	// 82532D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82532D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82532D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82532D64: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82532D68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82532D6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82532D70: 409A0034  bne cr6, 0x82532da4
	if !ctx.cr[6].eq {
	pc = 0x82532DA4; continue 'dispatch;
	}
	pc = 0x82532D74; continue 'dispatch;
            }
            0x82532D74 => {
    //   block [0x82532D74..0x82532DA4)
	// 82532D74: 48007BC5  bl 0x8253a938
	ctx.lr = 0x82532D78;
	sub_8253A938(ctx, base);
	// 82532D78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82532D7C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82532D80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82532D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82532D88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82532D8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82532D90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82532D94: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82532D98: 48007A69  bl 0x8253a800
	ctx.lr = 0x82532D9C;
	sub_8253A800(ctx, base);
	// 82532D9C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82532DA0: 48000064  b 0x82532e04
	pc = 0x82532E04; continue 'dispatch;
            }
            0x82532DA4 => {
    //   block [0x82532DA4..0x82532DF4)
	// 82532DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82532DA8: 419AFFCC  beq cr6, 0x82532d74
	if ctx.cr[6].eq {
	pc = 0x82532D74; continue 'dispatch;
	}
	// 82532DAC: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82532DB0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82532DB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82532DB8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82532DBC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82532DC0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82532DC4: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 82532DC8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82532DCC: 48006EAD  bl 0x82539c78
	ctx.lr = 0x82532DD0;
	sub_82539C78(ctx, base);
	// 82532DD0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82532DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82532DD8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82532DDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82532DE0: 41800014  blt 0x82532df4
	if ctx.cr[0].lt {
	pc = 0x82532DF4; continue 'dispatch;
	}
	// 82532DE4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82532DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82532DEC: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82532DF0: 48000010  b 0x82532e00
	pc = 0x82532E00; continue 'dispatch;
            }
            0x82532DF4 => {
    //   block [0x82532DF4..0x82532E00)
	// 82532DF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82532DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82532DFC: 48006CAD  bl 0x82539aa8
	ctx.lr = 0x82532E00;
	sub_82539AA8(ctx, base);
	pc = 0x82532E00; continue 'dispatch;
            }
            0x82532E00 => {
    //   block [0x82532E00..0x82532E04)
	// 82532E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82532E04; continue 'dispatch;
            }
            0x82532E04 => {
    //   block [0x82532E04..0x82532E18)
	// 82532E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82532E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82532E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82532E10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82532E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532E18 size=12
    let mut pc: u32 = 0x82532E18;
    'dispatch: loop {
        match pc {
            0x82532E18 => {
    //   block [0x82532E18..0x82532E24)
	// 82532E18: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82532E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82532E20: 4BFFFF38  b 0x82532d58
	sub_82532D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532E28 size=28
    let mut pc: u32 = 0x82532E28;
    'dispatch: loop {
        match pc {
            0x82532E28 => {
    //   block [0x82532E28..0x82532E44)
	// 82532E28: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532E2C: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82532E30: 396BEDA0  addi r11, r11, -0x1260
	ctx.r[11].s64 = ctx.r[11].s64 + -4704;
	// 82532E34: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82532E38: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82532E3C: 5563077A  rlwinm r3, r11, 0, 0x1d, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82532E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532E48 size=28
    let mut pc: u32 = 0x82532E48;
    'dispatch: loop {
        match pc {
            0x82532E48 => {
    //   block [0x82532E48..0x82532E64)
	// 82532E48: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532E4C: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82532E50: 396BEDA0  addi r11, r11, -0x1260
	ctx.r[11].s64 = ctx.r[11].s64 + -4704;
	// 82532E54: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82532E58: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82532E5C: 55630630  rlwinm r3, r11, 0, 0x18, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82532E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532E68 size=28
    let mut pc: u32 = 0x82532E68;
    'dispatch: loop {
        match pc {
            0x82532E68 => {
    //   block [0x82532E68..0x82532E84)
	// 82532E68: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532E6C: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82532E70: 396BEDA0  addi r11, r11, -0x1260
	ctx.r[11].s64 = ctx.r[11].s64 + -4704;
	// 82532E74: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82532E78: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82532E7C: 55630738  rlwinm r3, r11, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82532E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532E88 size=28
    let mut pc: u32 = 0x82532E88;
    'dispatch: loop {
        match pc {
            0x82532E88 => {
    //   block [0x82532E88..0x82532EA4)
	// 82532E88: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532E8C: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82532E90: 396BEDA0  addi r11, r11, -0x1260
	ctx.r[11].s64 = ctx.r[11].s64 + -4704;
	// 82532E94: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82532E98: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82532E9C: 71630107  andi. r3, r11, 0x107
	ctx.r[3].u64 = ctx.r[11].u64 & 263;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82532EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82532EA8 size=72
    let mut pc: u32 = 0x82532EA8;
    'dispatch: loop {
        match pc {
            0x82532EA8 => {
    //   block [0x82532EA8..0x82532EF0)
	// 82532EA8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82532EAC: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 82532EB0: D8410018  stfd f2, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[2].u64 ) };
	// 82532EB4: 396BE680  addi r11, r11, -0x1980
	ctx.r[11].s64 = ctx.r[11].s64 + -6528;
	// 82532EB8: C00B00A8  lfs f0, 0xa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82532EBC: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82532EC0: 409A0040  bne cr6, 0x82532f00
	if !ctx.cr[6].eq {
		sub_82532EF8(ctx, base);
		return;
	}
	// 82532EC4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82532EC8: 409A0030  bne cr6, 0x82532ef8
	if !ctx.cr[6].eq {
		sub_82532EF8(ctx, base);
		return;
	}
	// 82532ECC: 81410018  lwz r10, 0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 82532ED0: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82532ED4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82532ED8: 81410010  lwz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82532EDC: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82532EE0: 41820010  beq 0x82532ef0
	if ctx.cr[0].eq {
		sub_82532EF0(ctx, base);
		return;
	}
	// 82532EE4: C80B0010  lfd f0, 0x10(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82532EE8: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82532EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532EF0 size=8
    let mut pc: u32 = 0x82532EF0;
    'dispatch: loop {
        match pc {
            0x82532EF0 => {
    //   block [0x82532EF0..0x82532EF8)
	// 82532EF0: C82B0010  lfd f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82532EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82532EF8 size=224
    let mut pc: u32 = 0x82532EF8;
    'dispatch: loop {
        match pc {
            0x82532EF8 => {
    //   block [0x82532EF8..0x82532F24)
	// 82532EF8: C80B0008  lfd f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82532EFC: 480000BC  b 0x82532fb8
	pc = 0x82532FB8; continue 'dispatch;
	// 82532F00: FD801210  fabs f12, f2
	ctx.f[12].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 82532F04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82532F08: FDA00A10  fabs f13, f1
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82532F0C: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 82532F10: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82532F14: 40990010  ble cr6, 0x82532f24
	if !ctx.cr[6].gt {
	pc = 0x82532F24; continue 'dispatch;
	}
	// 82532F18: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82532F1C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82532F20: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
            }
            0x82532F24 => {
    //   block [0x82532F24..0x82532F4C)
	// 82532F24: FDAD0024  fdiv f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 82532F28: C80B0018  lfd f0, 0x18(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82532F2C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82532F30: 4099001C  ble cr6, 0x82532f4c
	if !ctx.cr[6].gt {
	pc = 0x82532F4C; continue 'dispatch;
	}
	// 82532F34: C80B0028  lfd f0, 0x28(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82532F38: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82532F3C: C18B00B0  lfs f12, 0xb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82532F40: FD60682A  fadd f11, f0, f13
	ctx.f[11].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 82532F44: FC006378  fmsub f0, f0, f13, f12
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64;
	// 82532F48: FDA05824  fdiv f13, f0, f11
	ctx.f[13].f64 = ctx.f[0].f64 / ctx.f[11].f64;
	pc = 0x82532F4C; continue 'dispatch;
            }
            0x82532F4C => {
    //   block [0x82532F4C..0x82532FA8)
	// 82532F4C: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82532F50: C96B0070  lfd f11, 0x70(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82532F54: C98B0050  lfd f12, 0x50(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82532F58: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82532F5C: FD4B002A  fadd f10, f11, f0
	ctx.f[10].f64 = ctx.f[11].f64 + ctx.f[0].f64;
	// 82532F60: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 82532F64: FD6C583A  fmadd f11, f12, f0, f11
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82532F68: C98B0068  lfd f12, 0x68(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 82532F6C: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82532F70: C98B0040  lfd f12, 0x40(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82532F74: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82532F78: C98B0060  lfd f12, 0x60(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 82532F7C: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82532F80: C98B0038  lfd f12, 0x38(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82532F84: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82532F88: C98B0058  lfd f12, 0x58(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 82532F8C: FD8A603A  fmadd f12, f10, f0, f12
	ctx.f[12].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82532F90: FC0B0032  fmul f0, f11, f0
	ctx.f[0].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 82532F94: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 82532F98: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 82532F9C: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 82532FA0: 40990008  ble cr6, 0x82532fa8
	if !ctx.cr[6].gt {
	pc = 0x82532FA8; continue 'dispatch;
	}
	// 82532FA4: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x82532FA8; continue 'dispatch;
            }
            0x82532FA8 => {
    //   block [0x82532FA8..0x82532FB8)
	// 82532FA8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82532FAC: 392B0080  addi r9, r11, 0x80
	ctx.r[9].s64 = ctx.r[11].s64 + 128;
	// 82532FB0: 7DAA4CAE  lfdx f13, r10, r9
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82532FB4: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	pc = 0x82532FB8; continue 'dispatch;
            }
            0x82532FB8 => {
    //   block [0x82532FB8..0x82532FD8)
	// 82532FB8: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82532FBC: 81610010  lwz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82532FC0: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 82532FC4: FC22682E  fsel f1, f2, f0, f13
	ctx.f[1].f64 = if ctx.f[2].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82532FC8: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82532FCC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82532FD0: FC200850  fneg f1, f1
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82532FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532FD8 size=32
    let mut pc: u32 = 0x82532FD8;
    'dispatch: loop {
        match pc {
            0x82532FD8 => {
    //   block [0x82532FD8..0x82532FF8)
	// 82532FD8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82532FDC: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 82532FE0: C90B2000  lfd f8, 0x2000(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8192 as u32) ) };
	// 82532FE4: FF014000  fcmpu cr6, f1, f8
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[8].f64);
	// 82532FE8: 409A0010  bne cr6, 0x82532ff8
	if !ctx.cr[6].eq {
		sub_82532FF8(ctx, base);
		return;
	}
	// 82532FEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82532FF0: C82B2008  lfd f1, 0x2008(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 82532FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82532FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82532FF8 size=48
    let mut pc: u32 = 0x82532FF8;
    'dispatch: loop {
        match pc {
            0x82532FF8 => {
    //   block [0x82532FF8..0x82533028)
	// 82532FF8: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82532FFC: 556A0476  rlwinm r10, r11, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82533000: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 82533004: 409A0024  bne cr6, 0x82533028
	if !ctx.cr[6].eq {
		sub_82533028(ctx, base);
		return;
	}
	// 82533008: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8253300C: C80B6E88  lfd f0, 0x6e88(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(28296 as u32) ) };
	// 82533010: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82533014: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 82533018: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253301C: C80BEE90  lfd f0, -0x1170(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4464 as u32) ) };
	// 82533020: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82533024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82533028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82533028 size=36
    let mut pc: u32 = 0x82533028;
    'dispatch: loop {
        match pc {
            0x82533028 => {
    //   block [0x82533028..0x8253304C)
	// 82533028: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8253302C: C8092008  lfd f0, 0x2008(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8200 as u32) ) };
	// 82533030: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82533034: 41990018  bgt cr6, 0x8253304c
	if ctx.cr[6].gt {
		sub_8253304C(ctx, base);
		return;
	}
	// 82533038: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8253303C: 409AFFDC  bne cr6, 0x82533018
	if !ctx.cr[6].eq {
		sub_82532FF8(ctx, base);
		return;
	}
	// 82533040: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82533044: C80BEE88  lfd f0, -0x1178(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4472 as u32) ) };
	// 82533048: 4BFFFFD8  b 0x82533020
	sub_82532FF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253304C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253304C size=264
    let mut pc: u32 = 0x8253304C;
    'dispatch: loop {
        match pc {
            0x8253304C => {
    //   block [0x8253304C..0x8253307C)
	// 8253304C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82533050: C8096E80  lfd f0, 0x6e80(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(28288 as u32) ) };
	// 82533054: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82533058: 40980024  bge cr6, 0x8253307c
	if !ctx.cr[6].lt {
	pc = 0x8253307C; continue 'dispatch;
	}
	// 8253305C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82533060: C80B6E78  lfd f0, 0x6e78(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(28280 as u32) ) };
	// 82533064: FC210032  fmul f1, f1, f0
	ctx.f[1].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 82533068: D8210010  stfd f1, 0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 8253306C: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82533070: 556AE57E  rlwinm r10, r11, 0x1c, 0x15, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82533074: 394AFBCD  addi r10, r10, -0x433
	ctx.r[10].s64 = ctx.r[10].s64 + -1075;
	// 82533078: 4800000C  b 0x82533084
	pc = 0x82533084; continue 'dispatch;
            }
            0x8253307C => {
    //   block [0x8253307C..0x82533084)
	// 8253307C: 554AE53E  rlwinm r10, r10, 0x1c, 0x14, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82533080: 394AFC02  addi r10, r10, -0x3fe
	ctx.r[10].s64 = ctx.r[10].s64 + -1022;
	pc = 0x82533084; continue 'dispatch;
            }
            0x82533084 => {
    //   block [0x82533084..0x825330C8)
	// 82533084: 716B800F  andi. r11, r11, 0x800f
	ctx.r[11].u64 = ctx.r[11].u64 & 32783;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82533088: D821FFF0  stfd f1, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[1].u64 ) };
	// 8253308C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82533090: 616B3FE0  ori r11, r11, 0x3fe0
	ctx.r[11].u64 = ctx.r[11].u64 | 16352;
	// 82533094: B161FFF0  sth r11, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u16 ) };
	// 82533098: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8253309C: 396B6E00  addi r11, r11, 0x6e00
	ctx.r[11].s64 = ctx.r[11].s64 + 28160;
	// 825330A0: C9AB0000  lfd f13, 0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825330A4: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825330A8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825330AC: C9A920E0  lfd f13, 0x20e0(r9)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8416 as u32) ) };
	// 825330B0: 40990018  ble cr6, 0x825330c8
	if !ctx.cr[6].gt {
	pc = 0x825330C8; continue 'dispatch;
	}
	// 825330B4: FD806828  fsub f12, f0, f13
	ctx.f[12].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 825330B8: FD60402A  fadd f11, f0, f8
	ctx.f[11].f64 = ctx.f[0].f64 + ctx.f[8].f64;
	// 825330BC: FC0C6828  fsub f0, f12, f13
	ctx.f[0].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 825330C0: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 825330C4: 48000014  b 0x825330d8
	pc = 0x825330D8; continue 'dispatch;
            }
            0x825330C8 => {
    //   block [0x825330C8..0x825330D8)
	// 825330C8: FC006828  fsub f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 825330CC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 825330D0: FD80402A  fadd f12, f0, f8
	ctx.f[12].f64 = ctx.f[0].f64 + ctx.f[8].f64;
	// 825330D4: FDAC0372  fmul f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	pc = 0x825330D8; continue 'dispatch;
            }
            0x825330D8 => {
    //   block [0x825330D8..0x82533154)
	// 825330D8: 7D4907B4  extsw r9, r10
	ctx.r[9].s64 = ctx.r[10].s32 as i64;
	// 825330DC: FDA06824  fdiv f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 825330E0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825330E4: C94B0008  lfd f10, 8(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 825330E8: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 825330EC: C96A6E70  lfd f11, 0x6e70(r10)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28272 as u32) ) };
	// 825330F0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825330F4: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 825330F8: C92A6E68  lfd f9, 0x6e68(r10)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28264 as u32) ) };
	// 825330FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82533100: FCE04828  fsub f7, f0, f9
	ctx.f[7].f64 = ctx.f[0].f64 - ctx.f[9].f64;
	// 82533104: C92A6E60  lfd f9, 0x6e60(r10)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28256 as u32) ) };
	// 82533108: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253310C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82533110: FCCC0272  fmul f6, f12, f9
	ctx.f[6].f64 = ctx.f[12].f64 * ctx.f[9].f64;
	// 82533114: C92B0028  lfd f9, 0x28(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82533118: FD204AFC  fnmsub f9, f0, f11, f9
	ctx.f[9].f64 = -(ctx.f[0].f64 * ctx.f[11].f64 - ctx.f[9].f64);
	// 8253311C: C96B0040  lfd f11, 0x40(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82533120: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82533124: FCE7583A  fmadd f7, f7, f0, f11
	ctx.f[7].f64 = ctx.f[7].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82533128: C96B6E58  lfd f11, 0x6e58(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(28248 as u32) ) };
	// 8253312C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82533130: FD295838  fmsub f9, f9, f0, f11
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 - ctx.f[11].f64;
	// 82533134: C96B6E50  lfd f11, 0x6e50(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(28240 as u32) ) };
	// 82533138: FD675838  fmsub f11, f7, f0, f11
	ctx.f[11].f64 = ctx.f[7].f64 * ctx.f[0].f64 - ctx.f[11].f64;
	// 8253313C: FC090032  fmul f0, f9, f0
	ctx.f[0].f64 = ctx.f[9].f64 * ctx.f[0].f64;
	// 82533140: FC005824  fdiv f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[11].f64;
	// 82533144: FC00402A  fadd f0, f0, f8
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[8].f64;
	// 82533148: FC003378  fmsub f0, f0, f13, f6
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[6].f64;
	// 8253314C: FC2C02BA  fmadd f1, f12, f10, f0
	ctx.f[1].f64 = ctx.f[12].f64 * ctx.f[10].f64 + ctx.f[0].f64;
	// 82533150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82533158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82533158 size=44
    let mut pc: u32 = 0x82533158;
    'dispatch: loop {
        match pc {
            0x82533158 => {
    //   block [0x82533158..0x82533184)
	// 82533158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253315C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82533160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82533164: 4BFFFE75  bl 0x82532fd8
	ctx.lr = 0x82533168;
	sub_82532FD8(ctx, base);
	// 82533168: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8253316C: C80B6E18  lfd f0, 0x6e18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(28184 as u32) ) };
	// 82533170: FC210032  fmul f1, f1, f0
	ctx.f[1].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 82533174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82533178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253317C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82533180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82533190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82533190 size=40
    let mut pc: u32 = 0x82533190;
    'dispatch: loop {
        match pc {
            0x82533190 => {
    //   block [0x82533190..0x825331B8)
	// 82533190: 7CA02B79  or. r0, r5, r5
	ctx.r[0].u64 = ctx.r[5].u64 | ctx.r[5].u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82533194: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82533198: 4081003C  ble 0x825331d4
	if !ctx.cr[0].gt {
		crate::recompiler::externs::call(ctx, base, 0x825331D4);
		return;
	}
	// 8253319C: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825331A0: 88E40000  lbz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825331A4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 825331A8: 2C880000  cmpwi cr1, r8, 0
	ctx.cr[1].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825331AC: 7C674011  subfc. r3, r7, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[7].u32;
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825331B0: 40060008  bdnzf 4*cr1+eq, 0x825331b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x825331B8; continue 'dispatch;
	}
	// 825331B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825331E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825331E0 size=48
    let mut pc: u32 = 0x825331E0;
    'dispatch: loop {
        match pc {
            0x825331E0 => {
    //   block [0x825331E0..0x82533210)
	// 825331E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825331E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825331E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825331EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825331F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825331F4: 48002AD5  bl 0x82535cc8
	ctx.lr = 0x825331F8;
	sub_82535CC8(ctx, base);
	// 825331F8: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 825331FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82533200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82533204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82533208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82533210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82533210 size=68
    let mut pc: u32 = 0x82533210;
    'dispatch: loop {
        match pc {
            0x82533210 => {
    //   block [0x82533210..0x82533254)
	// 82533210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82533214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82533218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253321C: 48002AAD  bl 0x82535cc8
	ctx.lr = 0x82533220;
	sub_82535CC8(ctx, base);
	// 82533220: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82533224: 3D400003  lis r10, 3
	ctx.r[10].s64 = 196608;
	// 82533228: 614A43FD  ori r10, r10, 0x43fd
	ctx.r[10].u64 = ctx.r[10].u64 | 17405;
	// 8253322C: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82533230: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s32 = ((ctx.r[9].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82533234: 3D4A0027  addis r10, r10, 0x27
	ctx.r[10].s64 = ctx.r[10].s64 + 2555904;
	// 82533238: 394A9EC3  addi r10, r10, -0x613d
	ctx.r[10].s64 = ctx.r[10].s64 + -24893;
	// 8253323C: 5543847E  rlwinm r3, r10, 0x10, 0x11, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82533240: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82533244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82533248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253324C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82533250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82533258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82533258 size=276
    let mut pc: u32 = 0x82533258;
    'dispatch: loop {
        match pc {
            0x82533258 => {
    //   block [0x82533258..0x82533294)
	// 82533258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253325C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82533260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82533264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82533268: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 8253326C: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82533270: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82533274: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82533278: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 8253327C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82533280: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82533284: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82533288: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253328C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82533290: 409A0034  bne cr6, 0x825332c4
	if !ctx.cr[6].eq {
	pc = 0x825332C4; continue 'dispatch;
	}
	pc = 0x82533294; continue 'dispatch;
            }
            0x82533294 => {
    //   block [0x82533294..0x825332C4)
	// 82533294: 480076A5  bl 0x8253a938
	ctx.lr = 0x82533298;
	sub_8253A938(ctx, base);
	// 82533298: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253329C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 825332A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825332A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825332A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825332AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825332B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825332B4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825332B8: 48007549  bl 0x8253a800
	ctx.lr = 0x825332BC;
	sub_8253A800(ctx, base);
	// 825332BC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825332C0: 48000094  b 0x82533354
	pc = 0x82533354; continue 'dispatch;
            }
            0x825332C4 => {
    //   block [0x825332C4..0x825332D4)
	// 825332C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825332C8: 419A000C  beq cr6, 0x825332d4
	if ctx.cr[6].eq {
	pc = 0x825332D4; continue 'dispatch;
	}
	// 825332CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825332D0: 419AFFC4  beq cr6, 0x82533294
	if ctx.cr[6].eq {
	pc = 0x82533294; continue 'dispatch;
	}
	pc = 0x825332D4; continue 'dispatch;
            }
            0x825332D4 => {
    //   block [0x825332D4..0x825332F8)
	// 825332D4: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 825332D8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825332DC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 825332E0: 390100C8  addi r8, r1, 0xc8
	ctx.r[8].s64 = ctx.r[1].s64 + 200;
	// 825332E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825332E8: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 825332EC: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825332F0: 41990008  bgt cr6, 0x825332f8
	if ctx.cr[6].gt {
	pc = 0x825332F8; continue 'dispatch;
	}
	// 825332F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	pc = 0x825332F8; continue 'dispatch;
            }
            0x825332F8 => {
    //   block [0x825332F8..0x82533344)
	// 825332F8: 39600042  li r11, 0x42
	ctx.r[11].s64 = 66;
	// 825332FC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82533300: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82533304: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82533308: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8253330C: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82533310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82533314: 48006965  bl 0x82539c78
	ctx.lr = 0x82533318;
	sub_82539C78(ctx, base);
	// 82533318: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8253331C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82533320: 419A0030  beq cr6, 0x82533350
	if ctx.cr[6].eq {
	pc = 0x82533350; continue 'dispatch;
	}
	// 82533324: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82533328: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253332C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82533330: 41800014  blt 0x82533344
	if ctx.cr[0].lt {
	pc = 0x82533344; continue 'dispatch;
	}
	// 82533334: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82533338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253333C: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82533340: 48000010  b 0x82533350
	pc = 0x82533350; continue 'dispatch;
            }
            0x82533344 => {
    //   block [0x82533344..0x82533350)
	// 82533344: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82533348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253334C: 4800675D  bl 0x82539aa8
	ctx.lr = 0x82533350;
	sub_82539AA8(ctx, base);
	pc = 0x82533350; continue 'dispatch;
            }
            0x82533350 => {
    //   block [0x82533350..0x82533354)
	// 82533350: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82533354; continue 'dispatch;
            }
            0x82533354 => {
    //   block [0x82533354..0x8253336C)
	// 82533354: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82533358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253335C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82533360: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82533364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82533368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82533370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82533370 size=60
    let mut pc: u32 = 0x82533370;
    'dispatch: loop {
        match pc {
            0x82533370 => {
    //   block [0x82533370..0x8253337C)
	// 82533370: 88C30000  lbz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82533374: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82533378: 4182001C  beq 0x82533394
	if ctx.cr[0].eq {
	pc = 0x82533394; continue 'dispatch;
	}
	pc = 0x8253337C; continue 'dispatch;
            }
            0x8253337C => {
    //   block [0x8253337C..0x82533394)
	// 8253337C: 2C860000  cmpwi cr1, r6, 0
	ctx.cr[1].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82533380: 7C062000  cmpw r6, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 82533384: 41860020  beq cr1, 0x825333a4
	if ctx.cr[1].eq {
	pc = 0x825333A4; continue 'dispatch;
	}
	// 82533388: 41820020  beq 0x825333a8
	if ctx.cr[0].eq {
	pc = 0x825333A8; continue 'dispatch;
	}
	// 8253338C: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 82533390: 4BFFFFEC  b 0x8253337c
	pc = 0x8253337C; continue 'dispatch;
            }
            0x82533394 => {
    //   block [0x82533394..0x825333A4)
	// 82533394: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82533398: 41820010  beq 0x825333a8
	if ctx.cr[0].eq {
	pc = 0x825333A8; continue 'dispatch;
	}
	// 8253339C: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 825333A0: 4BFFFFF4  b 0x82533394
	pc = 0x82533394; continue 'dispatch;
            }
            0x825333A4 => {
    //   block [0x825333A4..0x825333A8)
	// 825333A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x825333A8; continue 'dispatch;
            }
            0x825333A8 => {
    //   block [0x825333A8..0x825333AC)
	// 825333A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825333B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825333B0 size=228
    let mut pc: u32 = 0x825333B0;
    'dispatch: loop {
        match pc {
            0x825333B0 => {
    //   block [0x825333B0..0x825333D0)
	// 825333B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825333B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825333B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825333BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825333C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825333C4: 419A000C  beq cr6, 0x825333d0
	if ctx.cr[6].eq {
	pc = 0x825333D0; continue 'dispatch;
	}
	// 825333C8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825333CC: 409A0034  bne cr6, 0x82533400
	if !ctx.cr[6].eq {
	pc = 0x82533400; continue 'dispatch;
	}
	pc = 0x825333D0; continue 'dispatch;
            }
            0x825333D0 => {
    //   block [0x825333D0..0x82533400)
	// 825333D0: 48007569  bl 0x8253a938
	ctx.lr = 0x825333D4;
	sub_8253A938(ctx, base);
	// 825333D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825333D8: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 825333DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825333E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825333E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825333E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825333EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825333F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825333F4: 4800740D  bl 0x8253a800
	ctx.lr = 0x825333F8;
	sub_8253A800(ctx, base);
	// 825333F8: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 825333FC: 48000084  b 0x82533480
	pc = 0x82533480; continue 'dispatch;
            }
            0x82533400 => {
    //   block [0x82533400..0x82533418)
	// 82533400: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82533404: 409A0038  bne cr6, 0x8253343c
	if !ctx.cr[6].eq {
	pc = 0x8253343C; continue 'dispatch;
	}
	// 82533408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253340C: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82533410: 48007529  bl 0x8253a938
	ctx.lr = 0x82533414;
	sub_8253A938(ctx, base);
	// 82533414: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	pc = 0x82533418; continue 'dispatch;
            }
            0x82533418 => {
    //   block [0x82533418..0x8253343C)
	// 82533418: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8253341C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82533420: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82533424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82533428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253342C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82533430: 480073D1  bl 0x8253a800
	ctx.lr = 0x82533434;
	sub_8253A800(ctx, base);
	// 82533434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82533438: 48000048  b 0x82533480
	pc = 0x82533480; continue 'dispatch;
            }
            0x8253343C => {
    //   block [0x8253343C..0x82533440)
	// 8253343C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82533440; continue 'dispatch;
            }
            0x82533440 => {
    //   block [0x82533440..0x82533460)
	// 82533440: A1450000  lhz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82533444: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 82533448: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253344C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82533450: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82533454: 4182000C  beq 0x82533460
	if ctx.cr[0].eq {
	pc = 0x82533460; continue 'dispatch;
	}
	// 82533458: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8253345C: 4082FFE4  bne 0x82533440
	if !ctx.cr[0].eq {
	pc = 0x82533440; continue 'dispatch;
	}
	pc = 0x82533460; continue 'dispatch;
            }
            0x82533460 => {
    //   block [0x82533460..0x8253347C)
	// 82533460: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82533464: 409A0018  bne cr6, 0x8253347c
	if !ctx.cr[6].eq {
	pc = 0x8253347C; continue 'dispatch;
	}
	// 82533468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253346C: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82533470: 480074C9  bl 0x8253a938
	ctx.lr = 0x82533474;
	sub_8253A938(ctx, base);
	// 82533474: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 82533478: 4BFFFFA0  b 0x82533418
	pc = 0x82533418; continue 'dispatch;
            }
            0x8253347C => {
    //   block [0x8253347C..0x82533480)
	// 8253347C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82533480; continue 'dispatch;
            }
            0x82533480 => {
    //   block [0x82533480..0x82533494)
	// 82533480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82533484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82533488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253348C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82533490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


