pub fn sub_82FA9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FA9DF8 size=244
    let mut pc: u32 = 0x82FA9DF8;
    'dispatch: loop {
        match pc {
            0x82FA9DF8 => {
    //   block [0x82FA9DF8..0x82FA9EEC)
	// 82FA9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FA9DFC: 481FE36D  bl 0x831a8168
	ctx.lr = 0x82FA9E00;
	sub_831A8130(ctx, base);
	// 82FA9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FA9E04: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FA9E08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FA9E0C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82FA9E10: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FA9E14: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FA9E18: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82FA9E1C: 4BF3075D  bl 0x82eda578
	ctx.lr = 0x82FA9E20;
	sub_82EDA578(ctx, base);
	// 82FA9E20: 809C001C  lwz r4, 0x1c(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FA9E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FA9E28: 89640020  lbz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FA9E2C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82FA9E30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FA9E34: 409900B0  ble cr6, 0x82fa9ee4
	if !ctx.cr[6].gt {
	pc = 0x82FA9EE4; continue 'dispatch;
	}
	// 82FA9E38: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82FA9E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FA9E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FA9E44: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FA9E48: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 82FA9E4C: C16808A4  lfs f11, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FA9E50: 810BFFF8  lwz r8, -8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FA9E54: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FA9E58: 419A0064  beq cr6, 0x82fa9ebc
	if ctx.cr[6].eq {
	pc = 0x82FA9EBC; continue 'dispatch;
	}
	// 82FA9E5C: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FA9E60: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FA9E64: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FA9E68: 80FD0014  lwz r7, 0x14(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FA9E6C: 7C685214  add r3, r8, r10
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82FA9E70: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FA9E74: 7D074A14  add r8, r7, r9
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82FA9E78: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FA9E7C: 7D474C2E  lfsx f10, r7, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FA9E80: C1230020  lfs f9, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FA9E84: ED096828  fsubs f8, f9, f13
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FA9E88: ECE80332  fmuls f7, f8, f12
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[12].f64) as f32) as f64);
	// 82FA9E8C: ED8702B2  fmuls f12, f7, f10
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[10].f64) as f32) as f64);
	// 82FA9E90: 40980018  bge cr6, 0x82fa9ea8
	if !ctx.cr[6].lt {
	pc = 0x82FA9EA8; continue 'dispatch;
	}
	// 82FA9E94: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FA9E98: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82FA9E9C: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82FA9EA0: D1A60000  stfs f13, 0(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FA9EA4: 4800001C  b 0x82fa9ec0
	pc = 0x82FA9EC0; continue 'dispatch;
	// 82FA9EA8: C1A80008  lfs f13, 8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FA9EAC: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82FA9EB0: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82FA9EB4: D1A60000  stfs f13, 0(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FA9EB8: 48000008  b 0x82fa9ec0
	pc = 0x82FA9EC0; continue 'dispatch;
	// 82FA9EBC: D1660000  stfs f11, 0(r6)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FA9EC0: 89040020  lbz r8, 0x20(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FA9EC4: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82FA9EC8: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 82FA9ECC: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82FA9ED0: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82FA9ED4: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 82FA9ED8: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82FA9EDC: 7F053800  cmpw cr6, r5, r7
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82FA9EE0: 4198FF70  blt cr6, 0x82fa9e50
	if ctx.cr[6].lt {
	pc = 0x82FA9E50; continue 'dispatch;
	}
	// 82FA9EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FA9EE8: 481FE2D0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FA9EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FA9EF0 size=440
    let mut pc: u32 = 0x82FA9EF0;
    'dispatch: loop {
        match pc {
            0x82FA9EF0 => {
    //   block [0x82FA9EF0..0x82FAA0A8)
	// 82FA9EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FA9EF4: 481FE271  bl 0x831a8164
	ctx.lr = 0x82FA9EF8;
	sub_831A8130(ctx, base);
	// 82FA9EF8: 8145001C  lwz r10, 0x1c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FA9EFC: C1860004  lfs f12, 4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FA9F00: 8966000C  lbz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FA9F04: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82FA9F08: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82FA9F0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FA9F10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FA9F14: 88CA0020  lbz r6, 0x20(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FA9F18: C14808A4  lfs f10, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FA9F1C: 7CC40774  extsb r4, r6
	ctx.r[4].s64 = ctx.r[6].s8 as i64;
	// 82FA9F20: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FA9F24: 409900FC  ble cr6, 0x82faa020
	if !ctx.cr[6].gt {
	pc = 0x82FAA020; continue 'dispatch;
	}
	// 82FA9F28: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82FA9F2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FA9F30: 7D680034  cntlzw r8, r11
	ctx.r[8].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FA9F34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FA9F38: 5506DFFE  rlwinm r6, r8, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82FA9F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FA9F40: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FA9F44: 68DF0001  xori r31, r6, 1
	ctx.r[31].u64 = ctx.r[6].u64 ^ 1;
	// 82FA9F48: ED600824  fdivs f11, f0, f1
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82FA9F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FA9F50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FA9F54: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FA9F58: 8387000C  lwz r28, 0xc(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FA9F5C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FA9F60: 894A0008  lbz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FA9F64: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82FA9F68: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FA9F6C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FA9F70: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FA9F74: 7D4AF838  and r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[31].u64;
	// 82FA9F78: 7D5C49AE  stbx r10, r28, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82FA9F7C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FA9F80: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FA9F84: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FA9F88: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82FA9F8C: 40990008  ble cr6, 0x82fa9f94
	if !ctx.cr[6].gt {
	pc = 0x82FA9F94; continue 'dispatch;
	}
	// 82FA9F90: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82FA9F94: 8385001C  lwz r28, 0x1c(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FA9F98: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FA9F9C: 81450048  lwz r10, 0x48(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FA9FA0: EDAC0032  fmuls f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FA9FA4: 7F665214  add r27, r6, r10
	ctx.r[27].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 82FA9FA8: 815C008C  lwz r10, 0x8c(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FA9FAC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82FA9FB0: C13B00A0  lfs f9, 0xa0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(160 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FA9FB4: C10A0000  lfs f8, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FA9FB8: C0EA0004  lfs f7, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FA9FBC: ECC90232  fmuls f6, f9, f8
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[8].f64) as f32) as f64);
	// 82FA9FC0: ECA702F2  fmuls f5, f7, f11
	ctx.f[5].f64 = (((ctx.f[7].f64 * ctx.f[11].f64) as f32) as f64);
	// 82FA9FC4: EC860172  fmuls f4, f6, f5
	ctx.f[4].f64 = (((ctx.f[6].f64 * ctx.f[5].f64) as f32) as f64);
	// 82FA9FC8: FC602050  fneg f3, f4
	ctx.f[3].u64 = ctx.f[4].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FA9FCC: EC030232  fmuls f0, f3, f8
	ctx.f[0].f64 = (((ctx.f[3].f64 * ctx.f[8].f64) as f32) as f64);
	// 82FA9FD0: FC400210  fabs f2, f0
	ctx.f[2].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82FA9FD4: FF026800  fcmpu cr6, f2, f13
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[13].f64);
	// 82FA9FD8: 40990018  ble cr6, 0x82fa9ff0
	if !ctx.cr[6].gt {
	pc = 0x82FA9FF0; continue 'dispatch;
	}
	// 82FA9FDC: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FA9FE0: 4099000C  ble cr6, 0x82fa9fec
	if !ctx.cr[6].gt {
	pc = 0x82FA9FEC; continue 'dispatch;
	}
	// 82FA9FE4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82FA9FE8: 48000008  b 0x82fa9ff0
	pc = 0x82FA9FF0; continue 'dispatch;
	// 82FA9FEC: FC006850  fneg f0, f13
	ctx.f[0].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FA9FF0: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FA9FF4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FA9FF8: 8385001C  lwz r28, 0x1c(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FA9FFC: 39080028  addi r8, r8, 0x28
	ctx.r[8].s64 = ctx.r[8].s64 + 40;
	// 82FAA000: 38C600C0  addi r6, r6, 0xc0
	ctx.r[6].s64 = ctx.r[6].s64 + 192;
	// 82FAA004: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FAA008: 7C04552E  stfsx f0, r4, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82FAA00C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82FAA010: 895C0020  lbz r10, 0x20(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAA014: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82FAA018: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FAA01C: 4198FF38  blt cr6, 0x82fa9f54
	if ctx.cr[6].lt {
	pc = 0x82FA9F54; continue 'dispatch;
	}
	// 82FAA020: 7FCB0774  extsb r11, r30
	ctx.r[11].s64 = ctx.r[30].s8 as i64;
	// 82FAA024: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAA028: 419A0078  beq cr6, 0x82faa0a0
	if ctx.cr[6].eq {
	pc = 0x82FAA0A0; continue 'dispatch;
	}
	// 82FAA02C: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAA030: C0070018  lfs f0, 0x18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAA034: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FAA038: 4198005C  blt cr6, 0x82faa094
	if ctx.cr[6].lt {
	pc = 0x82FAA094; continue 'dispatch;
	}
	// 82FAA03C: 8145001C  lwz r10, 0x1c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAA040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAA044: 892A0020  lbz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAA048: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 82FAA04C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAA050: 40990054  ble cr6, 0x82faa0a4
	if !ctx.cr[6].gt {
	pc = 0x82FAA0A4; continue 'dispatch;
	}
	// 82FAA054: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAA058: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA05C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FAA060: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAA064: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82FAA068: 4099000C  ble cr6, 0x82faa074
	if !ctx.cr[6].gt {
	pc = 0x82FAA074; continue 'dispatch;
	}
	// 82FAA06C: 8127000C  lwz r9, 0xc(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAA070: 7FA959AE  stbx r29, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	// 82FAA074: 8125001C  lwz r9, 0x1c(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAA078: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAA07C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82FAA080: 89090020  lbz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAA084: 7D060774  extsb r6, r8
	ctx.r[6].s64 = ctx.r[8].s8 as i64;
	// 82FAA088: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82FAA08C: 4198FFCC  blt cr6, 0x82faa058
	if ctx.cr[6].lt {
	pc = 0x82FAA058; continue 'dispatch;
	}
	// 82FAA090: 481FE124  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FAA094: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82FAA098: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82FAA09C: 481FE118  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FAA0A0: D1470018  stfs f10, 0x18(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82FAA0A4: 481FE110  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAA0A8 size=56
    let mut pc: u32 = 0x82FAA0A8;
    'dispatch: loop {
        match pc {
            0x82FAA0A8 => {
    //   block [0x82FAA0A8..0x82FAA0E0)
	// 82FAA0A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAA0AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FAA0B0: 392B355C  addi r9, r11, 0x355c
	ctx.r[9].s64 = ctx.r[11].s64 + 13660;
	// 82FAA0B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FAA0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAA0BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAA0C0: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82FAA0C4: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82FAA0C8: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAA0CC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FAA0D0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FAA0D4: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82FAA0D8: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82FAA0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA0E0 size=236
    let mut pc: u32 = 0x82FAA0E0;
    'dispatch: loop {
        match pc {
            0x82FAA0E0 => {
    //   block [0x82FAA0E0..0x82FAA1CC)
	// 82FAA0E0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82FAA0E4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FAA0E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FAA0EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAA0F0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FAA0F4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FAA0F8: 392B3AAC  addi r9, r11, 0x3aac
	ctx.r[9].s64 = ctx.r[11].s64 + 15020;
	// 82FAA0FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAA100: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82FAA104: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAA108: 91430094  stw r10, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82FAA10C: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 82FAA110: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82FAA114: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82FAA118: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82FAA11C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82FAA120: 90E300A0  stw r7, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[7].u32 ) };
	// 82FAA124: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FAA128: 91630098  stw r11, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82FAA12C: 3C808200  lis r4, -0x7e00
	ctx.r[4].s64 = -2113929216;
	// 82FAA130: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAA1D0 size=888
    let mut pc: u32 = 0x82FAA1D0;
    'dispatch: loop {
        match pc {
            0x82FAA1D0 => {
    //   block [0x82FAA1D0..0x82FAA548)
	// 82FAA1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAA1D4: 481FDF71  bl 0x831a8144
	ctx.lr = 0x82FAA1D8;
	sub_831A8130(ctx, base);
	// 82FAA1D8: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82FAA1DC: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAA1E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAA1E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAA1E8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FAA1EC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82FAA1F0: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82FAA1F4: 997F0190  stb r11, 0x190(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u8 ) };
	// 82FAA1F8: 3B3F0020  addi r25, r31, 0x20
	ctx.r[25].s64 = ctx.r[31].s64 + 32;
	// 82FAA1FC: 895F0020  lbz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAA200: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	// 82FAA204: 7EFEBB78  mr r30, r23
	ctx.r[30].u64 = ctx.r[23].u64;
	// 82FAA208: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAA20C: 40990038  ble cr6, 0x82faa244
	if !ctx.cr[6].gt {
	pc = 0x82FAA244; continue 'dispatch;
	}
	// 82FAA210: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FAA214: 89390000  lbz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA218: 394B0024  addi r10, r11, 0x24
	ctx.r[10].s64 = ctx.r[11].s64 + 36;
	// 82FAA21C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82FAA220: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA224: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82FAA228: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAA22C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FAA230: 40990008  ble cr6, 0x82faa238
	if !ctx.cr[6].gt {
	pc = 0x82FAA238; continue 'dispatch;
	}
	// 82FAA234: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82FAA238: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAA23C: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 82FAA240: 4082FFE0  bne 0x82faa220
	if !ctx.cr[0].eq {
	pc = 0x82FAA220; continue 'dispatch;
	}
	// 82FAA244: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FAA248: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAA24C: 4099003C  ble cr6, 0x82faa288
	if !ctx.cr[6].gt {
	pc = 0x82FAA288; continue 'dispatch;
	}
	// 82FAA250: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FAA254: 3BBF0098  addi r29, r31, 0x98
	ctx.r[29].s64 = ctx.r[31].s64 + 152;
	// 82FAA258: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAA25C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FAA260: 40980024  bge cr6, 0x82faa284
	if !ctx.cr[6].lt {
	pc = 0x82FAA284; continue 'dispatch;
	}
	// 82FAA264: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAA268: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAA26C: 41980008  blt cr6, 0x82faa274
	if ctx.cr[6].lt {
	pc = 0x82FAA274; continue 'dispatch;
	}
	// 82FAA270: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FAA274: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FAA278: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAA27C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FAA280: 4BEFC579  bl 0x82ea67f8
	ctx.lr = 0x82FAA284;
	sub_82EA67F8(ctx, base);
	// 82FAA284: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FAA288: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FAA28C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82FAA290: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAA294: 4099001C  ble cr6, 0x82faa2b0
	if !ctx.cr[6].gt {
	pc = 0x82FAA2B0; continue 'dispatch;
	}
	// 82FAA298: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FAA29C: 7EEA59AE  stbx r23, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[23].u8) };
	// 82FAA2A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAA2A4: 813F009C  lwz r9, 0x9c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FAA2A8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAA2AC: 4198FFEC  blt cr6, 0x82faa298
	if ctx.cr[6].lt {
	pc = 0x82FAA298; continue 'dispatch;
	}
	// 82FAA2B0: 89790000  lbz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA2B4: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82FAA2B8: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82FAA2BC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAA2C0: 40990040  ble cr6, 0x82faa300
	if !ctx.cr[6].gt {
	pc = 0x82FAA300; continue 'dispatch;
	}
	// 82FAA2C4: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82FAA2C8: 811F008C  lwz r8, 0x8c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FAA2CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FAA2D0: 813F0098  lwz r9, 0x98(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FAA2D4: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82FAA2D8: 396B0028  addi r11, r11, 0x28
	ctx.r[11].s64 = ctx.r[11].s64 + 40;
	// 82FAA2DC: 88E80024  lbz r7, 0x24(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FAA2E0: 7CE80774  extsb r8, r7
	ctx.r[8].s64 = ctx.r[7].s8 as i64;
	// 82FAA2E4: 7CE848AE  lbzx r7, r8, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAA2E8: 38A70001  addi r5, r7, 1
	ctx.r[5].s64 = ctx.r[7].s64 + 1;
	// 82FAA2EC: 7CA849AE  stbx r5, r8, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[5].u8) };
	// 82FAA2F0: 88790000  lbz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA2F4: 7C690774  extsb r9, r3
	ctx.r[9].s64 = ctx.r[3].s8 as i64;
	// 82FAA2F8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAA2FC: 4198FFCC  blt cr6, 0x82faa2c8
	if ctx.cr[6].lt {
	pc = 0x82FAA2C8; continue 'dispatch;
	}
	// 82FAA300: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82FAA304: C01F0060  lfs f0, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAA308: C1BF0078  lfs f13, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAA30C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82FAA310: C17F0064  lfs f11, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAA314: ED806824  fdivs f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82FAA318: C15F007C  lfs f10, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAA31C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAA320: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82FAA324: ED2B5024  fdivs f9, f11, f10
	ctx.f[9].f64 = ((ctx.f[11].f64 / ctx.f[10].f64) as f32) as f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAA548 size=188
    let mut pc: u32 = 0x82FAA548;
    'dispatch: loop {
        match pc {
            0x82FAA548 => {
    //   block [0x82FAA548..0x82FAA604)
	// 82FAA548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAA54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAA550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAA554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAA558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAA55C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FAA560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FAA564: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FAA568: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FAA56C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAA570: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAA578: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAA57C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAA580: 4E800421  bctrl
	ctx.lr = 0x82FAA584;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAA584: 815E0094  lwz r10, 0x94(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FAA588: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAA58C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAA590: 409A0048  bne cr6, 0x82faa5d8
	if !ctx.cr[6].eq {
	pc = 0x82FAA5D8; continue 'dispatch;
	}
	// 82FAA594: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FAA598: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAA59C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA5A0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAA5A4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAA5A8: 80DE008C  lwz r6, 0x8c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FAA5AC: 7CA95214  add r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82FAA5B0: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82FAA5B4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAA5B8: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA5BC: 54A81838  slwi r8, r5, 3
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAA5C0: 54671838  slwi r7, r3, 3
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FAA5C4: 388B45BC  addi r4, r11, 0x45bc
	ctx.r[4].s64 = ctx.r[11].s64 + 17852;
	// 82FAA5C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FAA5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAA5D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAA5D4: 4E800421  bctrl
	ctx.lr = 0x82FAA5D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAA5D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAA5E0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAA5E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAA5E8: 4E800421  bctrl
	ctx.lr = 0x82FAA5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAA5EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAA5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAA5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAA5F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAA5FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAA600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA608 size=32
    let mut pc: u32 = 0x82FAA608;
    'dispatch: loop {
        match pc {
            0x82FAA608 => {
    //   block [0x82FAA608..0x82FAA628)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA628 size=60
    let mut pc: u32 = 0x82FAA628;
    'dispatch: loop {
        match pc {
            0x82FAA628 => {
    //   block [0x82FAA628..0x82FAA664)
	// 82FAA628: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA62C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAA630: 7D0B2214  add r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FAA634: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAA638: 0CC90000  twi 6, r9, 0
	// 82FAA63C: 7CE84BD6  divw r7, r8, r9
	ctx.r[7].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 82FAA640: 550B083E  rotlwi r11, r8, 1
	ctx.r[11].u64 = ((ctx.r[8].u32).rotate_left(1)) as u64;
	// 82FAA644: 7CC749D6  mullw r6, r7, r9
	ctx.r[6].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82FAA648: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82FAA64C: 7C864050  subf r4, r6, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 82FAA650: 7D232878  andc r3, r9, r5
	ctx.r[3].u64 = ctx.r[9].u64 & !ctx.r[5].u64;
	// 82FAA654: 548B2834  slwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAA658: 0CA3FFFF  twi 5, r3, -1
	// 82FAA65C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FAA660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAA668 size=8
    let mut pc: u32 = 0x82FAA668;
    'dispatch: loop {
        match pc {
            0x82FAA668 => {
    //   block [0x82FAA668..0x82FAA670)
	// 82FAA668: C023001C  lfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82FAA66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA670 size=136
    let mut pc: u32 = 0x82FAA670;
    'dispatch: loop {
        match pc {
            0x82FAA670 => {
    //   block [0x82FAA670..0x82FAA6F8)
	// 82FAA670: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FAA674: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAA678: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAA67C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82FAA680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAA684: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAA688: 80A9000C  lwz r5, 0xc(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAA68C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FAA690: 40990060  ble cr6, 0x82faa6f0
	if !ctx.cr[6].gt {
	pc = 0x82FAA6F0; continue 'dispatch;
	}
	// 82FAA694: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82FAA698: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA69C: 8089000C  lwz r4, 0xc(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAA6A0: 7C685A14  add r3, r8, r11
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82FAA6A4: 80E90010  lwz r7, 0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAA6A8: 0CC40000  twi 6, r4, 0
	// 82FAA6AC: 7FE323D6  divw r31, r3, r4
	ctx.r[31].s32 = ctx.r[3].s32 / ctx.r[4].s32;
	// 82FAA6B0: 5468083E  rotlwi r8, r3, 1
	ctx.r[8].u64 = ((ctx.r[3].u32).rotate_left(1)) as u64;
	// 82FAA6B4: 7FFF21D6  mullw r31, r31, r4
	ctx.r[31].s64 = (ctx.r[31].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82FAA6B8: 7C7F1850  subf r3, r31, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82FAA6BC: 3BE8FFFF  addi r31, r8, -1
	ctx.r[31].s64 = ctx.r[8].s64 + -1;
	// 82FAA6C0: 54682834  slwi r8, r3, 5
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAA6C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAA6C8: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82FAA6CC: 7C84F878  andc r4, r4, r31
	ctx.r[4].u64 = ctx.r[4].u64 & !ctx.r[31].u64;
	// 82FAA6D0: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82FAA6D4: 0CA4FFFF  twi 5, r4, -1
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA6F8 size=56
    let mut pc: u32 = 0x82FAA6F8;
    'dispatch: loop {
        match pc {
            0x82FAA6F8 => {
    //   block [0x82FAA6F8..0x82FAA730)
	// 82FAA6F8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAA6FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FAA700: 392B3C34  addi r9, r11, 0x3c34
	ctx.r[9].s64 = ctx.r[11].s64 + 15412;
	// 82FAA704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAA708: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FAA70C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82FAA710: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAA714: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FAA718: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FAA71C: 91030018  stw r8, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82FAA720: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FAA724: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FAA728: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FAA72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA730 size=188
    let mut pc: u32 = 0x82FAA730;
    'dispatch: loop {
        match pc {
            0x82FAA730 => {
    //   block [0x82FAA730..0x82FAA7EC)
	// 82FAA730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAA734: 481FDA35  bl 0x831a8168
	ctx.lr = 0x82FAA738;
	sub_831A8130(ctx, base);
	// 82FAA738: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAA7F0 size=196
    let mut pc: u32 = 0x82FAA7F0;
    'dispatch: loop {
        match pc {
            0x82FAA7F0 => {
    //   block [0x82FAA7F0..0x82FAA8B4)
	// 82FAA7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAA7F4: 481FD971  bl 0x831a8164
	ctx.lr = 0x82FAA7F8;
	sub_831A8130(ctx, base);
	// 82FAA7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAA7FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FAA800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAA804: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FAA808: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82FAA80C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAA810: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FAA818: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAA81C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAA820: 4E800421  bctrl
	ctx.lr = 0x82FAA824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAA824: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAA828: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FAA82C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAA830: 40990068  ble cr6, 0x82faa898
	if !ctx.cr[6].gt {
	pc = 0x82FAA898; continue 'dispatch;
	}
	// 82FAA834: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAA838: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FAA83C: 3B6B45C8  addi r27, r11, 0x45c8
	ctx.r[27].s64 = ctx.r[11].s64 + 17864;
	// 82FAA840: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAA844: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FAA848: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAA84C: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAA850: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAA854: 409A0030  bne cr6, 0x82faa884
	if !ctx.cr[6].eq {
	pc = 0x82FAA884; continue 'dispatch;
	}
	// 82FAA858: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA85C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FAA860: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAA864: 55482834  slwi r8, r10, 5
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAA868: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAA86C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FAA870: 54E72834  slwi r7, r7, 5
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FAA874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FAA878: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA87C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FAA880: 4E800421  bctrl
	ctx.lr = 0x82FAA884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAA884: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAA888: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FAA88C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FAA890: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAA894: 4198FFAC  blt cr6, 0x82faa840
	if ctx.cr[6].lt {
	pc = 0x82FAA840; continue 'dispatch;
	}
	// 82FAA898: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAA89C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FAA8A0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAA8A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAA8A8: 4E800421  bctrl
	ctx.lr = 0x82FAA8AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAA8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FAA8B0: 481FD904  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAA8B8 size=92
    let mut pc: u32 = 0x82FAA8B8;
    'dispatch: loop {
        match pc {
            0x82FAA8B8 => {
    //   block [0x82FAA8B8..0x82FAA914)
	// 82FAA8B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAA8BC: C1A4001C  lfs f13, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAA8C0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAA8C4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FAA8C8: 409A004C  bne cr6, 0x82faa914
	if !ctx.cr[6].eq {
		sub_82FAA914(ctx, base);
		return;
	}
	// 82FAA8CC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAA8D0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA8D4: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAA8D8: 0CCB0000  twi 6, r11, 0
	// 82FAA8DC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FAA8E0: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82FAA8E4: 7CE85BD6  divw r7, r8, r11
	ctx.r[7].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 82FAA8E8: 550A083E  rotlwi r10, r8, 1
	ctx.r[10].u64 = ((ctx.r[8].u32).rotate_left(1)) as u64;
	// 82FAA8EC: 7CC759D6  mullw r6, r7, r11
	ctx.r[6].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82FAA8F0: 7CA64050  subf r5, r6, r8
	ctx.r[5].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 82FAA8F4: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82FAA8F8: 54AA2834  slwi r10, r5, 5
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAA8FC: 7D674078  andc r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 & !ctx.r[8].u64;
	// 82FAA900: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FAA904: 0CA7FFFF  twi 5, r7, -1
	// 82FAA908: C1A6001C  lfs f13, 0x1c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAA90C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FAA910: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA914 size=88
    let mut pc: u32 = 0x82FAA914;
    'dispatch: loop {
        match pc {
            0x82FAA914 => {
    //   block [0x82FAA914..0x82FAA96C)
	// 82FAA914: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAA918: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82FAA91C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAA970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAA970 size=552
    let mut pc: u32 = 0x82FAA970;
    'dispatch: loop {
        match pc {
            0x82FAA970 => {
    //   block [0x82FAA970..0x82FAAB98)
	// 82FAA970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAA974: 481FD7DD  bl 0x831a8150
	ctx.lr = 0x82FAA978;
	sub_831A8130(ctx, base);
	// 82FAA978: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAAB98 size=284
    let mut pc: u32 = 0x82FAAB98;
    'dispatch: loop {
        match pc {
            0x82FAAB98 => {
    //   block [0x82FAAB98..0x82FAACB4)
	// 82FAAB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAAB9C: 481FD5A9  bl 0x831a8144
	ctx.lr = 0x82FAABA0;
	sub_831A8130(ctx, base);
	// 82FAABA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAABA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FAABA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAABAC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FAABB0: 392B3C44  addi r9, r11, 0x3c44
	ctx.r[9].s64 = ctx.r[11].s64 + 15428;
	// 82FAABB4: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82FAABB8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FAABBC: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAABC0: 3EA08000  lis r21, -0x8000
	ctx.r[21].s64 = -2147483648;
	// 82FAABC4: B29C0006  sth r20, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[20].u16 ) };
	// 82FAABC8: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAABCC: 93FC0010  stw r31, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82FAABD0: 93FC0014  stw r31, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82FAABD4: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82FAABD8: 92BC0018  stw r21, 0x18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[21].u32 ) };
	// 82FAABDC: D01C0008  stfs f0, 8(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAABE0: D01C000C  stfs f0, 0xc(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FAABE4: 88E40020  lbz r7, 0x20(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAABE8: 811C0018  lwz r8, 0x18(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAABEC: 550B00BE  clrlwi r11, r8, 2
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAABF0: 7CFE0774  extsb r30, r7
	ctx.r[30].s64 = ctx.r[7].s8 as i64;
	// 82FAABF4: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82FAABF8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FAABFC: 40980024  bge cr6, 0x82faac20
	if !ctx.cr[6].lt {
	pc = 0x82FAAC20; continue 'dispatch;
	}
	// 82FAAC00: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAAC04: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAAC08: 41980008  blt cr6, 0x82faac10
	if ctx.cr[6].lt {
	pc = 0x82FAAC10; continue 'dispatch;
	}
	// 82FAAC0C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FAAC10: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FAAC14: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAAC18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FAAC1C: 4BEFBBDD  bl 0x82ea67f8
	ctx.lr = 0x82FAAC20;
	sub_82EA67F8(ctx, base);
	// 82FAAC20: 93DB0004  stw r30, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FAAC24: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82FAAC28: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAAC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAAC30: 40990078  ble cr6, 0x82faaca8
	if !ctx.cr[6].gt {
	pc = 0x82FAACA8; continue 'dispatch;
	}
	// 82FAAC34: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAAC38: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAC3C: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 82FAAC40: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82FAAC44: 3AE0001C  li r23, 0x1c
	ctx.r[23].s64 = 28;
	// 82FAAC48: 3B4B3C34  addi r26, r11, 0x3c34
	ctx.r[26].s64 = ctx.r[11].s64 + 15412;
	// 82FAAC4C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82FAAC50: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82FAAC54: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FAAC58: 4BEF5AD9  bl 0x82ea0730
	ctx.lr = 0x82FAAC5C;
	sub_82EA0730(ctx, base);
	// 82FAAC5C: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82FAAC60: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FAAC64: B2F30004  sth r23, 4(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(4 as u32), ctx.r[23].u16 ) };
	// 82FAAC68: B2930006  sth r20, 6(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[20].u16 ) };
	// 82FAAC6C: 93530000  stw r26, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82FAAC70: 93F30010  stw r31, 0x10(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82FAAC74: 93F30014  stw r31, 0x14(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82FAAC78: 92B30018  stw r21, 0x18(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(24 as u32), ctx.r[21].u32 ) };
	// 82FAAC7C: 93F3000C  stw r31, 0xc(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82FAAC80: 93F30008  stw r31, 8(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82FAAC84: 93F30014  stw r31, 0x14(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82FAAC88: 4BFFFAA9  bl 0x82faa730
	ctx.lr = 0x82FAAC8C;
	sub_82FAA730(ctx, base);
	// 82FAAC8C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAC90: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FAAC94: 7E6BF12E  stwx r19, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[19].u32) };
	// 82FAAC98: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FAAC9C: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAACA0: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FAACA4: 4198FFA8  blt cr6, 0x82faac4c
	if ctx.cr[6].lt {
	pc = 0x82FAAC4C; continue 'dispatch;
	}
	// 82FAACA8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FAACAC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FAACB0: 481FD4E4  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAACB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAACB8 size=196
    let mut pc: u32 = 0x82FAACB8;
    'dispatch: loop {
        match pc {
            0x82FAACB8 => {
    //   block [0x82FAACB8..0x82FAAD7C)
	// 82FAACB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAACBC: 481FD4B1  bl 0x831a816c
	ctx.lr = 0x82FAACC0;
	sub_831A8130(ctx, base);
	// 82FAACC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAACC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAACC8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAACCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FAACD0: 392B3C44  addi r9, r11, 0x3c44
	ctx.r[9].s64 = ctx.r[11].s64 + 15428;
	// 82FAACD4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAACD8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAACDC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAACE0: 4099005C  ble cr6, 0x82faad3c
	if !ctx.cr[6].gt {
	pc = 0x82FAAD3C; continue 'dispatch;
	}
	// 82FAACE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FAACE8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAACEC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FAACF0: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAACF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FAACF8: 419A0030  beq cr6, 0x82faad28
	if ctx.cr[6].eq {
	pc = 0x82FAAD28; continue 'dispatch;
	}
	// 82FAACFC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FAAD00: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82FAAD04: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82FAAD08: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82FAAD0C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAAD10: 409A0018  bne cr6, 0x82faad28
	if !ctx.cr[6].eq {
	pc = 0x82FAAD28; continue 'dispatch;
	}
	// 82FAAD14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAD18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FAAD1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAD20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAAD24: 4E800421  bctrl
	ctx.lr = 0x82FAAD28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAAD28: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAAD2C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FAAD30: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FAAD34: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAAD38: 4198FFB0  blt cr6, 0x82faace8
	if ctx.cr[6].lt {
	pc = 0x82FAACE8; continue 'dispatch;
	}
	// 82FAAD3C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAAD40: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAAD44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAAD48: 409A0020  bne cr6, 0x82faad68
	if !ctx.cr[6].eq {
	pc = 0x82FAAD68; continue 'dispatch;
	}
	// 82FAAD4C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAD50: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FAAD54: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FAAD58: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAAD5C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FAAD60: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAAD64: 4BEF5A4D  bl 0x82ea07b0
	ctx.lr = 0x82FAAD68;
	sub_82EA07B0(ctx, base);
	// 82FAAD68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAAD6C: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82FAAD70: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FAAD74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAAD78: 481FD444  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAAD80 size=4
    let mut pc: u32 = 0x82FAAD80;
    'dispatch: loop {
        match pc {
            0x82FAAD80 => {
    //   block [0x82FAAD80..0x82FAAD84)
	// 82FAAD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAAD88 size=56
    let mut pc: u32 = 0x82FAAD88;
    'dispatch: loop {
        match pc {
            0x82FAAD88 => {
    //   block [0x82FAAD88..0x82FAADC0)
	// 82FAAD88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAAD8C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAAD90: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAAD94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FAAD98: 38EBAC74  addi r7, r11, -0x538c
	ctx.r[7].s64 = ctx.r[11].s64 + -21388;
	// 82FAAD9C: 38CA3D3C  addi r6, r10, 0x3d3c
	ctx.r[6].s64 = ctx.r[10].s64 + 15676;
	// 82FAADA0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82FAADA4: 38A93D2C  addi r5, r9, 0x3d2c
	ctx.r[5].s64 = ctx.r[9].s64 + 15660;
	// 82FAADA8: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FAADAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAADB0: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FAADB4: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82FAADB8: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82FAADBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAADC0 size=44
    let mut pc: u32 = 0x82FAADC0;
    'dispatch: loop {
        match pc {
            0x82FAADC0 => {
    //   block [0x82FAADC0..0x82FAADEC)
	// 82FAADC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAADC4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FAADC8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAADCC: 390B3D2C  addi r8, r11, 0x3d2c
	ctx.r[8].s64 = ctx.r[11].s64 + 15660;
	// 82FAADD0: 38EAAC74  addi r7, r10, -0x538c
	ctx.r[7].s64 = ctx.r[10].s64 + -21388;
	// 82FAADD4: 38C99EAC  addi r6, r9, -0x6154
	ctx.r[6].s64 = ctx.r[9].s64 + -24916;
	// 82FAADD8: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82FAADDC: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FAADE0: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82FAADE4: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FAADE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAADF0 size=16
    let mut pc: u32 = 0x82FAADF0;
    'dispatch: loop {
        match pc {
            0x82FAADF0 => {
    //   block [0x82FAADF0..0x82FAAE00)
	// 82FAADF0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAADF4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAADF8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FAADFC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAAE00 size=12
    let mut pc: u32 = 0x82FAAE00;
    'dispatch: loop {
        match pc {
            0x82FAAE00 => {
    //   block [0x82FAAE00..0x82FAAE0C)
	// 82FAAE00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAAE04: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FAAE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAAE10 size=4
    let mut pc: u32 = 0x82FAAE10;
    'dispatch: loop {
        match pc {
            0x82FAAE10 => {
    //   block [0x82FAAE10..0x82FAAE14)
	// 82FAAE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAAE18 size=88
    let mut pc: u32 = 0x82FAAE18;
    'dispatch: loop {
        match pc {
            0x82FAAE18 => {
    //   block [0x82FAAE18..0x82FAAE70)
	// 82FAAE18: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAAE1C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAAE20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FAAE24: 390B3D4C  addi r8, r11, 0x3d4c
	ctx.r[8].s64 = ctx.r[11].s64 + 15692;
	// 82FAAE28: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82FAAE2C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FAAE30: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82FAAE34: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FAAE38: 38A9AC74  addi r5, r9, -0x538c
	ctx.r[5].s64 = ctx.r[9].s64 + -21388;
	// 82FAAE3C: B143001A  sth r10, 0x1a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 82FAAE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAAE44: 38873D3C  addi r4, r7, 0x3d3c
	ctx.r[4].s64 = ctx.r[7].s64 + 15676;
	// 82FAAE48: 90A3001C  stw r5, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82FAAE4C: 39263D2C  addi r9, r6, 0x3d2c
	ctx.r[9].s64 = ctx.r[6].s64 + 15660;
	// 82FAAE50: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FAAE54: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82FAAE58: 39430014  addi r10, r3, 0x14
	ctx.r[10].s64 = ctx.r[3].s64 + 20;
	// 82FAAE5C: 9123001C  stw r9, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82FAAE60: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FAAE64: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82FAAE68: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FAAE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAAE70 size=184
    let mut pc: u32 = 0x82FAAE70;
    'dispatch: loop {
        match pc {
            0x82FAAE70 => {
    //   block [0x82FAAE70..0x82FAAF28)
	// 82FAAE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAAE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAAE78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAAE7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAAE80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAAE84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAAE88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FAAE8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FAAE90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAE94: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAAE98: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAAE9C: 4E800421  bctrl
	ctx.lr = 0x82FAAEA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAAEA0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82FAAEA4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82FAAEA8: 80ED0000  lwz r7, 0(r13)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAEAC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82FAAEB0: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82FAAEB4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAAF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAAF28 size=216
    let mut pc: u32 = 0x82FAAF28;
    'dispatch: loop {
        match pc {
            0x82FAAF28 => {
    //   block [0x82FAAF28..0x82FAB000)
	// 82FAAF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAAF2C: 481FD239  bl 0x831a8164
	ctx.lr = 0x82FAAF30;
	sub_831A8130(ctx, base);
	// 82FAAF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAAF34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAF38: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FAAF3C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FAAF40: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82FAAF44: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FAAF48: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FAAF4C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAAF50: 4BEF57E1  bl 0x82ea0730
	ctx.lr = 0x82FAAF54;
	sub_82EA0730(ctx, base);
	// 82FAAF54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAAF58: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAAF5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAAF60: 38E93D4C  addi r7, r9, 0x3d4c
	ctx.r[7].s64 = ctx.r[9].s64 + 15692;
	// 82FAAF64: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82FAAF68: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82FAAF6C: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82FAAF70: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82FAAF74: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FAAF78: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82FAAF7C: B0DF0004  sth r6, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82FAAF80: B17F001A  sth r11, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82FAAF84: 3868AC74  addi r3, r8, -0x538c
	ctx.r[3].s64 = ctx.r[8].s64 + -21388;
	// 82FAAF88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FAAF8C: 39653D3C  addi r11, r5, 0x3d3c
	ctx.r[11].s64 = ctx.r[5].s64 + 15676;
	// 82FAAF90: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FAAF94: 39443D2C  addi r10, r4, 0x3d2c
	ctx.r[10].s64 = ctx.r[4].s64 + 15660;
	// 82FAAF98: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82FAAF9C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FAAFA0: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82FAAFA4: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82FAAFA8: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FAAFAC: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82FAAFB0: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FAAFB4: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAFB8: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAAFBC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FAAFC0: 4BF2A129  bl 0x82ed50e8
	ctx.lr = 0x82FAAFC4;
	sub_82ED50E8(ctx, base);
	// 82FAAFC4: 351C0014  addic. r8, r28, 0x14
	ctx.xer.ca = (ctx.r[28].u32 > (!(20 as u32)));
	ctx.r[8].s64 = ctx.r[28].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAAFC8: 389C001C  addi r4, r28, 0x1c
	ctx.r[4].s64 = ctx.r[28].s64 + 28;
	// 82FAAFCC: 40820008  bne 0x82faafd4
	if !ctx.cr[0].eq {
	pc = 0x82FAAFD4; continue 'dispatch;
	}
	// 82FAAFD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FAAFD4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAAFD8: 4BF2A9F9  bl 0x82ed59d0
	ctx.lr = 0x82FAAFDC;
	sub_82ED59D0(ctx, base);
	// 82FAAFDC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FAAFE0: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82FAAFE4: 409A0008  bne cr6, 0x82faafec
	if !ctx.cr[6].eq {
	pc = 0x82FAAFEC; continue 'dispatch;
	}
	// 82FAAFE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FAAFEC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAAFF0: 4BF2AAC9  bl 0x82ed5ab8
	ctx.lr = 0x82FAAFF4;
	sub_82ED5AB8(ctx, base);
	// 82FAAFF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAAFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FAAFFC: 481FD1B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAB000 size=164
    let mut pc: u32 = 0x82FAB000;
    'dispatch: loop {
        match pc {
            0x82FAB000 => {
    //   block [0x82FAB000..0x82FAB0A4)
	// 82FAB000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAB008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAB00C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAB010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAB014: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAB018: 394B3D4C  addi r10, r11, 0x3d4c
	ctx.r[10].s64 = ctx.r[11].s64 + 15692;
	// 82FAB01C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB020: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FAB024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FAB028: 419A0038  beq cr6, 0x82fab060
	if ctx.cr[6].eq {
	pc = 0x82FAB060; continue 'dispatch;
	}
	// 82FAB02C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAB030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAB034: 419A0024  beq cr6, 0x82fab058
	if ctx.cr[6].eq {
	pc = 0x82FAB058; continue 'dispatch;
	}
	// 82FAB038: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAB03C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAB040: 419A0018  beq cr6, 0x82fab058
	if ctx.cr[6].eq {
	pc = 0x82FAB058; continue 'dispatch;
	}
	// 82FAB044: 357F0014  addic. r11, r31, 0x14
	ctx.xer.ca = (ctx.r[31].u32 > (!(20 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAB048: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82FAB04C: 40820008  bne 0x82fab054
	if !ctx.cr[0].eq {
	pc = 0x82FAB054; continue 'dispatch;
	}
	// 82FAB050: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAB054: 4BF2A97D  bl 0x82ed59d0
	ctx.lr = 0x82FAB058;
	sub_82ED59D0(ctx, base);
	// 82FAB058: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB05C: 4BF2A0AD  bl 0x82ed5108
	ctx.lr = 0x82FAB060;
	sub_82ED5108(ctx, base);
	// 82FAB060: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAB064: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FAB068: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAB06C: 390B3D2C  addi r8, r11, 0x3d2c
	ctx.r[8].s64 = ctx.r[11].s64 + 15660;
	// 82FAB070: 38EAAC74  addi r7, r10, -0x538c
	ctx.r[7].s64 = ctx.r[10].s64 + -21388;
	// 82FAB074: 38C99EAC  addi r6, r9, -0x6154
	ctx.r[6].s64 = ctx.r[9].s64 + -24916;
	// 82FAB078: 911F001C  stw r8, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82FAB07C: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82FAB080: 90FF001C  stw r7, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 82FAB084: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FAB088: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FAB08C: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FAB090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAB094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAB098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAB09C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAB0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAB0A8 size=76
    let mut pc: u32 = 0x82FAB0A8;
    'dispatch: loop {
        match pc {
            0x82FAB0A8 => {
    //   block [0x82FAB0A8..0x82FAB0F4)
	// 82FAB0A8: 80E40048  lwz r7, 0x48(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FAB0AC: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82FAB0B0: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82FAB0B4: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82FAB0B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB0F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAB0F4 size=108
    let mut pc: u32 = 0x82FAB0F4;
    'dispatch: loop {
        match pc {
            0x82FAB0F4 => {
    //   block [0x82FAB0F4..0x82FAB160)
	// 82FAB0F4: 392000C0  li r9, 0xc0
	ctx.r[9].s64 = 192;
	// 82FAB0F8: 81040048  lwz r8, 0x48(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAB160 size=92
    let mut pc: u32 = 0x82FAB160;
    'dispatch: loop {
        match pc {
            0x82FAB160 => {
    //   block [0x82FAB160..0x82FAB1BC)
	// 82FAB160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAB168: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAB16C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAB170: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82FAB174: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82FAB178: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB17C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82FAB180: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82FAB184: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAB1C0 size=72
    let mut pc: u32 = 0x82FAB1C0;
    'dispatch: loop {
        match pc {
            0x82FAB1C0 => {
    //   block [0x82FAB1C0..0x82FAB208)
	// 82FAB1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAB1C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAB1CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAB1D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAB1D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FAB1D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB1DC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAB1E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAB1E4: 4E800421  bctrl
	ctx.lr = 0x82FAB1E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB1E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FAB1EC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB1F0: 4BF2F9D9  bl 0x82edabc8
	ctx.lr = 0x82FAB1F4;
	sub_82EDABC8(ctx, base);
	// 82FAB1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FAB1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAB1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAB200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAB204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAB208 size=852
    let mut pc: u32 = 0x82FAB208;
    'dispatch: loop {
        match pc {
            0x82FAB208 => {
    //   block [0x82FAB208..0x82FAB55C)
	// 82FAB208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB20C: 481FCF25  bl 0x831a8130
	ctx.lr = 0x82FAB210;
	sub_831A8130(ctx, base);
	// 82FAB210: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82FAB214: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82FAB218: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82FAB21C: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAB220: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FAB224: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82FAB228: 7CCE3378  mr r14, r6
	ctx.r[14].u64 = ctx.r[6].u64;
	// 82FAB22C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FAB230: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB234: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FAB238: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAB23C: 4E800421  bctrl
	ctx.lr = 0x82FAB240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB240: 8139001C  lwz r9, 0x1c(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAB244: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82FAB248: 3AF9001C  addi r23, r25, 0x1c
	ctx.r[23].s64 = ctx.r[25].s64 + 28;
	// 82FAB24C: 7ED2B378  mr r18, r22
	ctx.r[18].u64 = ctx.r[22].u64;
	// 82FAB250: 89090020  lbz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAB254: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82FAB258: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FAB25C: 409902EC  ble cr6, 0x82fab548
	if !ctx.cr[6].gt {
	pc = 0x82FAB548; continue 'dispatch;
	}
	// 82FAB260: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FAB264: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FAB268: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82FAB26C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAB270: 3A190048  addi r16, r25, 0x48
	ctx.r[16].s64 = ctx.r[25].s64 + 72;
	// 82FAB274: 38EBC5B0  addi r7, r11, -0x3a50
	ctx.r[7].s64 = ctx.r[11].s64 + -14928;
	// 82FAB278: C3AA9534  lfs f29, -0x6acc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82FAB27C: C3C908A4  lfs f30, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82FAB280: 39F90034  addi r15, r25, 0x34
	ctx.r[15].s64 = ctx.r[25].s64 + 52;
	// 82FAB284: C3E808A8  lfs f31, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82FAB288: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 82FAB28C: 3A20FFFF  li r17, -1
	ctx.r[17].s64 = -1;
	// 82FAB290: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82FAB294: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 82FAB298: 3AA00060  li r21, 0x60
	ctx.r[21].s64 = 96;
	// 82FAB29C: 3A6000D0  li r19, 0xd0
	ctx.r[19].s64 = 208;
	// 82FAB2A0: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAB2A4: 81300000  lwz r9, 0(r16)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB2A8: 811A0000  lwz r8, 0(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB2AC: 57CA3032  slwi r10, r30, 6
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAB2B0: 7CFE5A14  add r7, r30, r11
	ctx.r[7].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82FAB2B4: D3E10080  stfs f31, 0x80(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82FAB2B8: 92210084  stw r17, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[17].u32 ) };
	// 82FAB2BC: 7FEA7214  add r31, r10, r14
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[14].u64;
	// 82FAB2C0: 54EB3032  slwi r11, r7, 6
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAB2C4: 92C100B0  stw r22, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[22].u32 ) };
	// 82FAB2C8: 92810090  stw r20, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[20].u32 ) };
	// 82FAB2CC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82FAB2D0: 7FAB4A14  add r29, r11, r9
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FAB2D4: 81680024  lwz r11, 0x24(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FAB2D8: 92C100C0  stw r22, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[22].u32 ) };
	// 82FAB2DC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FAB2E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FAB2E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FAB2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FAB2EC: 4E800421  bctrl
	ctx.lr = 0x82FAB2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB2F0: 80AF0000  lwz r5, 0(r15)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB2F4: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAB2F8: 80970000  lwz r4, 0(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB2FC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAB300: 810100C0  lwz r8, 0xc0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FAB304: 7D3E5A14  add r9, r30, r11
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82FAB308: 7CFE5214  add r7, r30, r10
	ctx.r[7].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 82FAB30C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAB310: 8064008C  lwz r3, 0x8c(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FAB314: 54E61838  slwi r6, r7, 3
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82FAB318: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FAB31C: 7C061C2E  lfsx f0, r6, r3
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB320: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAB324: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FAB328: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAB32C: 419A01BC  beq cr6, 0x82fab4e8
	if ctx.cr[6].eq {
	pc = 0x82FAB4E8; continue 'dispatch;
	}
	// 82FAB330: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82FAB334: 3B1F0010  addi r24, r31, 0x10
	ctx.r[24].s64 = ctx.r[31].s64 + 16;
	// 82FAB338: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82FAB33C: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAB560 size=112
    let mut pc: u32 = 0x82FAB560;
    'dispatch: loop {
        match pc {
            0x82FAB560 => {
    //   block [0x82FAB560..0x82FAB5D0)
	// 82FAB560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAB568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAB56C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAB570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAB574: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FAB578: 83C30010  lwz r30, 0x10(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB57C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAB580: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAB584: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAB588: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAB58C: 409A0010  bne cr6, 0x82fab59c
	if !ctx.cr[6].eq {
	pc = 0x82FAB59C; continue 'dispatch;
	}
	// 82FAB590: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FAB594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAB598: 4BEFB2E9  bl 0x82ea6880
	ctx.lr = 0x82FAB59C;
	sub_82EA6880(ctx, base);
	// 82FAB59C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAB5A0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB5A4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAB5A8: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FAB5AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAB5B0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82FAB5B4: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FAB5B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAB5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAB5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAB5C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAB5C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAB5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB5D0 size=40
    let mut pc: u32 = 0x82FAB5D0;
    'dispatch: loop {
        match pc {
            0x82FAB5D0 => {
    //   block [0x82FAB5D0..0x82FAB5F8)
	// 82FAB5D0: 896500CC  lbz r11, 0xcc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FAB5D4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82FAB5D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAB5DC: 419A001C  beq cr6, 0x82fab5f8
	if ctx.cr[6].eq {
		sub_82FAB5F8(ctx, base);
		return;
	}
	// 82FAB5E0: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAB5E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FAB5E8: 419A0010  beq cr6, 0x82fab5f8
	if ctx.cr[6].eq {
		sub_82FAB5F8(ctx, base);
		return;
	}
	// 82FAB5EC: C006000C  lfs f0, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB5F0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FAB5F4: 48000008  b 0x82fab5fc
	sub_82FAB5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB5F8 size=36
    let mut pc: u32 = 0x82FAB5F8;
    'dispatch: loop {
        match pc {
            0x82FAB5F8 => {
    //   block [0x82FAB5F8..0x82FAB61C)
	// 82FAB5F8: C006000C  lfs f0, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB5FC: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAB600: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FAB604: 409A0018  bne cr6, 0x82fab61c
	if !ctx.cr[6].eq {
		sub_82FAB61C(ctx, base);
		return;
	}
	// 82FAB608: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAB60C: 419A0010  beq cr6, 0x82fab61c
	if ctx.cr[6].eq {
		sub_82FAB61C(ctx, base);
		return;
	}
	// 82FAB610: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAB614: C02B08A8  lfs f1, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82FAB618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB61C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB61C size=16
    let mut pc: u32 = 0x82FAB61C;
    'dispatch: loop {
        match pc {
            0x82FAB61C => {
    //   block [0x82FAB61C..0x82FAB62C)
	// 82FAB61C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAB620: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82FAB624: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82FAB628: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB62C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB62C size=16
    let mut pc: u32 = 0x82FAB62C;
    'dispatch: loop {
        match pc {
            0x82FAB62C => {
    //   block [0x82FAB62C..0x82FAB63C)
	// 82FAB62C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAB630: C1AB9534  lfs f13, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAB634: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FAB638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB640 size=48
    let mut pc: u32 = 0x82FAB640;
    'dispatch: loop {
        match pc {
            0x82FAB640 => {
    //   block [0x82FAB640..0x82FAB670)
	// 82FAB640: 896500CC  lbz r11, 0xcc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FAB644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAB648: 419A0028  beq cr6, 0x82fab670
	if ctx.cr[6].eq {
		sub_82FAB670(ctx, base);
		return;
	}
	// 82FAB64C: 89630014  lbz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAB650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAB654: 419A001C  beq cr6, 0x82fab670
	if ctx.cr[6].eq {
		sub_82FAB670(ctx, base);
		return;
	}
	// 82FAB658: C006000C  lfs f0, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB65C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAB660: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FAB664: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB668: FC2D036E  fsel f1, f13, f13, f0
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82FAB66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB670 size=20
    let mut pc: u32 = 0x82FAB670;
    'dispatch: loop {
        match pc {
            0x82FAB670 => {
    //   block [0x82FAB670..0x82FAB684)
	// 82FAB670: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAB674: C1A6000C  lfs f13, 0xc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAB678: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB67C: FC2D036E  fsel f1, f13, f13, f0
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82FAB680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAB688 size=204
    let mut pc: u32 = 0x82FAB688;
    'dispatch: loop {
        match pc {
            0x82FAB688 => {
    //   block [0x82FAB688..0x82FAB754)
	// 82FAB688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB68C: 481FCADD  bl 0x831a8168
	ctx.lr = 0x82FAB690;
	sub_831A8130(ctx, base);
	// 82FAB690: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82FAB694: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAB698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAB69C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82FAB6A0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FAB6A4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FAB6A8: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FAB6AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB6B0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB6B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAB6B8: 4E800421  bctrl
	ctx.lr = 0x82FAB6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB6BC: D03C0000  stfs f1, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAB6C0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB6C4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FAB6C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FAB6CC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FAB6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAB6D4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FAB6D8: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAB6DC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82FAB6E0: 4E800421  bctrl
	ctx.lr = 0x82FAB6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB6E4: D03C0004  stfs f1, 4(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FAB6E8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB6EC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FAB6F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FAB6F4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FAB6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAB6FC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FAB700: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAB704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FAB708: 4E800421  bctrl
	ctx.lr = 0x82FAB70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB70C: D03C0008  stfs f1, 8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAB710: 895D0010  lbz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAB714: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82FAB718: 995C000C  stb r10, 0xc(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82FAB71C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FAB720: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB724: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FAB728: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FAB72C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FAB730: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAB734: 80A90018  lwz r5, 0x18(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAB738: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82FAB73C: 4E800421  bctrl
	ctx.lr = 0x82FAB740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAB740: 88830000  lbz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAB744: 989C000D  stb r4, 0xd(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(13 as u32), ctx.r[4].u8 ) };
	// 82FAB748: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FAB74C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82FAB750: 481FCA68  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB758 size=52
    let mut pc: u32 = 0x82FAB758;
    'dispatch: loop {
        match pc {
            0x82FAB758 => {
    //   block [0x82FAB758..0x82FAB78C)
	// 82FAB758: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAB75C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAB760: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FAB764: 390A3F10  addi r8, r10, 0x3f10
	ctx.r[8].s64 = ctx.r[10].s64 + 16144;
	// 82FAB768: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FAB76C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82FAB770: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB774: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FAB778: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAB77C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FAB780: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FAB784: 98E30014  stb r7, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u8 ) };
	// 82FAB788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB790 size=32
    let mut pc: u32 = 0x82FAB790;
    'dispatch: loop {
        match pc {
            0x82FAB790 => {
    //   block [0x82FAB790..0x82FAB7B0)
	// 82FAB790: C1A60008  lfs f13, 8(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAB794: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAB798: FD806A10  fabs f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82FAB79C: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAB7A0: FF0C5000  fcmpu cr6, f12, f10
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[10].f64);
	// 82FAB7A4: 4098000C  bge cr6, 0x82fab7b0
	if !ctx.cr[6].lt {
		sub_82FAB7B0(ctx, base);
		return;
	}
	// 82FAB7A8: C02B08A4  lfs f1, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82FAB7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB7B0 size=28
    let mut pc: u32 = 0x82FAB7B0;
    'dispatch: loop {
        match pc {
            0x82FAB7B0 => {
    //   block [0x82FAB7B0..0x82FAB7CC)
	// 82FAB7B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FAB7B4: C16B08A4  lfs f11, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAB7B8: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 82FAB7BC: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAB7C0: 4099000C  ble cr6, 0x82fab7cc
	if !ctx.cr[6].gt {
		sub_82FAB7CC(ctx, base);
		return;
	}
	// 82FAB7C4: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82FAB7C8: 4800000C  b 0x82fab7d4
	sub_82FAB7CC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB7CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB7CC size=40
    let mut pc: u32 = 0x82FAB7CC;
    'dispatch: loop {
        match pc {
            0x82FAB7CC => {
    //   block [0x82FAB7CC..0x82FAB7F4)
	// 82FAB7CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAB7D0: C1AB9534  lfs f13, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAB7D4: C1630008  lfs f11, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAB7D8: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82FAB7DC: 40980018  bge cr6, 0x82fab7f4
	if !ctx.cr[6].lt {
		sub_82FAB7F4(ctx, base);
		return;
	}
	// 82FAB7E0: EC0C5028  fsubs f0, f12, f10
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 82FAB7E4: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAB7E8: ED600332  fmuls f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82FAB7EC: EC2B0372  fmuls f1, f11, f13
	ctx.f[1].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FAB7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB7F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAB7F4 size=52
    let mut pc: u32 = 0x82FAB7F4;
    'dispatch: loop {
        match pc {
            0x82FAB7F4 => {
    //   block [0x82FAB7F4..0x82FAB828)
	// 82FAB7F4: C1630008  lfs f11, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAB7F8: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAB7FC: ED2C5828  fsubs f9, f12, f11
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82FAB800: ED0B5028  fsubs f8, f11, f10
	ctx.f[8].f64 = (((ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 82FAB804: C0E3000C  lfs f7, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FAB808: ECC05028  fsubs f6, f0, f10
	ctx.f[6].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82FAB80C: ECA70232  fmuls f5, f7, f8
	ctx.f[5].f64 = (((ctx.f[7].f64 * ctx.f[8].f64) as f32) as f64);
	// 82FAB810: EC864028  fsubs f4, f6, f8
	ctx.f[4].f64 = (((ctx.f[6].f64 - ctx.f[8].f64) as f32) as f64);
	// 82FAB814: EC602828  fsubs f3, f0, f5
	ctx.f[3].f64 = (((ctx.f[0].f64 - ctx.f[5].f64) as f32) as f64);
	// 82FAB818: EC432024  fdivs f2, f3, f4
	ctx.f[2].f64 = ((ctx.f[3].f64 / ctx.f[4].f64) as f32) as f64;
	// 82FAB81C: EC222A7A  fmadds f1, f2, f9, f5
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[9].f64 + ctx.f[5].f64) as f32) as f64);
	// 82FAB820: EC210372  fmuls f1, f1, f13
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FAB824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAB828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAB828 size=540
    let mut pc: u32 = 0x82FAB828;
    'dispatch: loop {
        match pc {
            0x82FAB828 => {
    //   block [0x82FAB828..0x82FABA44)
	// 82FAB828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAB82C: 481FC921  bl 0x831a814c
	ctx.lr = 0x82FAB830;
	sub_831A8130(ctx, base);
	// 82FAB830: 89640014  lbz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAB834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAB838: 409A0010  bne cr6, 0x82fab848
	if !ctx.cr[6].eq {
	pc = 0x82FAB848; continue 'dispatch;
	}
	// 82FAB83C: 89670011  lbz r11, 0x11(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(17 as u32) ) } as u64;
	// 82FAB840: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FAB844: 481FC958  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82FAB848: 81660018  lwz r11, 0x18(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAB84C: 3941FF70  addi r10, r1, -0x90
	ctx.r[10].s64 = ctx.r[1].s64 + -144;
	// 82FAB850: 3921FF70  addi r9, r1, -0x90
	ctx.r[9].s64 = ctx.r[1].s64 + -144;
	// 82FAB854: 3901FF70  addi r8, r1, -0x90
	ctx.r[8].s64 = ctx.r[1].s64 + -144;
	// 82FAB858: 38A1FF60  addi r5, r1, -0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + -160;
	// 82FAB85C: 3881FF60  addi r4, r1, -0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + -160;
	// 82FAB860: EBCB01A0  ld r30, 0x1a0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(416 as u32) ) };
	// 82FAB864: 3FA08213  lis r29, -0x7ded
	ctx.r[29].s64 = -2112684032;
	// 82FAB868: EB8B01A8  ld r28, 0x1a8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(424 as u32) ) };
	// 82FAB86C: 3BEB01A0  addi r31, r11, 0x1a0
	ctx.r[31].s64 = ctx.r[11].s64 + 416;
	// 82FAB870: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82FAB874: FBCA0000  std r30, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82FAB878: FB8A0008  std r28, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[28].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FABA48 size=140
    let mut pc: u32 = 0x82FABA48;
    'dispatch: loop {
        match pc {
            0x82FABA48 => {
    //   block [0x82FABA48..0x82FABAD4)
	// 82FABA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FABA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FABA50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FABA54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FABA58: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABA5C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FABA60: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82FABA64: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82FABA68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FABA6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FABA70: 4BEF4CC1  bl 0x82ea0730
	ctx.lr = 0x82FABA74;
	sub_82EA0730(ctx, base);
	// 82FABA74: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FABA78: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82FABA7C: 38E93EFC  addi r7, r9, 0x3efc
	ctx.r[7].s64 = ctx.r[9].s64 + 16124;
	// 82FABA80: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82FABA84: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82FABA88: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FABA8C: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82FABA90: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FABA94: B0A30004  sth r5, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 82FABA98: A09F0006  lhz r4, 6(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FABA9C: B0830006  sth r4, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 82FABAA0: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABAA4: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FABAA8: C1BF000C  lfs f13, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABAAC: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FABAB0: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FABAB4: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82FABAB8: 895F0011  lbz r10, 0x11(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(17 as u32) ) } as u64;
	// 82FABABC: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 82FABAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FABAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FABAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FABACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FABAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABAD8 size=52
    let mut pc: u32 = 0x82FABAD8;
    'dispatch: loop {
        match pc {
            0x82FABAD8 => {
    //   block [0x82FABAD8..0x82FABB0C)
	// 82FABAD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FABADC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FABAE0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FABAE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FABAE8: 38E94004  addi r7, r9, 0x4004
	ctx.r[7].s64 = ctx.r[9].s64 + 16388;
	// 82FABAEC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABAF0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82FABAF4: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABAF8: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FABAFC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FABB00: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FABB04: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FABB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FABB10 size=204
    let mut pc: u32 = 0x82FABB10;
    'dispatch: loop {
        match pc {
            0x82FABB10 => {
    //   block [0x82FABB10..0x82FABBDC)
	// 82FABB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FABB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FABB18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FABB1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FABB20: 83E50018  lwz r31, 0x18(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FABB24: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82FABB28: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82FABB2C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82FABB30: 391F01B0  addi r8, r31, 0x1b0
	ctx.r[8].s64 = ctx.r[31].s64 + 432;
	// 82FABB34: E8FF01B0  ld r7, 0x1b0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) };
	// 82FABB38: E8DF01B8  ld r6, 0x1b8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(440 as u32) ) };
	// 82FABB3C: F8EB0000  std r7, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82FABB40: F8CB0008  std r6, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABBE0 size=64
    let mut pc: u32 = 0x82FABBE0;
    'dispatch: loop {
        match pc {
            0x82FABBE0 => {
    //   block [0x82FABBE0..0x82FABC20)
	// 82FABBE0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FABBE4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FABBE8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FABBEC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82FABBF0: C00B45D8  lfs f0, 0x45d8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABBF4: ED610032  fmuls f11, f1, f0
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABBF8: C00A45D4  lfs f0, 0x45d4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(17876 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABBFC: C1A99584  lfs f13, -0x6a7c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABC00: C188340C  lfs f12, 0x340c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(13324 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FABC04: ED4B0032  fmuls f10, f11, f0
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABC08: ED2A1024  fdivs f9, f10, f2
	ctx.f[9].f64 = ((ctx.f[10].f64 / ctx.f[2].f64) as f32) as f64;
	// 82FABC0C: ED090372  fmuls f8, f9, f13
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FABC10: ECE80332  fmuls f7, f8, f12
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[12].f64) as f32) as f64);
	// 82FABC14: ECC33824  fdivs f6, f3, f7
	ctx.f[6].f64 = ((ctx.f[3].f64 / ctx.f[7].f64) as f32) as f64;
	// 82FABC18: EC262024  fdivs f1, f6, f4
	ctx.f[1].f64 = ((ctx.f[6].f64 / ctx.f[4].f64) as f32) as f64;
	// 82FABC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FABC20 size=100
    let mut pc: u32 = 0x82FABC20;
    'dispatch: loop {
        match pc {
            0x82FABC20 => {
    //   block [0x82FABC20..0x82FABC84)
	// 82FABC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FABC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FABC28: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82FABC2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FABC30: 8965000F  lbz r11, 0xf(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(15 as u32) ) } as u64;
	// 82FABC34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FABC38: 419A0020  beq cr6, 0x82fabc58
	if ctx.cr[6].eq {
	pc = 0x82FABC58; continue 'dispatch;
	}
	// 82FABC3C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FABC40: C02B08A4  lfs f1, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82FABC44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FABC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FABC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FABC50: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FABC54: 4E800020  blr
	return;
	// 82FABC58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABC5C: C3E400B4  lfs f31, 0xb4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(180 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82FABC60: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FABC64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FABC68: 4E800421  bctrl
	ctx.lr = 0x82FABC6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FABC6C: EC2107F2  fmuls f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 82FABC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FABC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FABC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FABC7C: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FABC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FABC88 size=20
    let mut pc: u32 = 0x82FABC88;
    'dispatch: loop {
        match pc {
            0x82FABC88 => {
    //   block [0x82FABC88..0x82FABC9C)
	// 82FABC88: 896500B0  lbz r11, 0xb0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FABC8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FABC90: 409A000C  bne cr6, 0x82fabc9c
	if !ctx.cr[6].eq {
		sub_82FABC9C(ctx, base);
		return;
	}
	// 82FABC94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FABC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABC9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FABC9C size=32
    let mut pc: u32 = 0x82FABC9C;
    'dispatch: loop {
        match pc {
            0x82FABC9C => {
    //   block [0x82FABC9C..0x82FABCBC)
	// 82FABC9C: 8966000E  lbz r11, 0xe(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as u64;
	// 82FABCA0: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82FABCA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FABCA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FABCAC: 41990008  bgt cr6, 0x82fabcb4
	if ctx.cr[6].gt {
	pc = 0x82FABCB4; continue 'dispatch;
	}
	// 82FABCB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FABCB4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FABCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABCC0 size=32
    let mut pc: u32 = 0x82FABCC0;
    'dispatch: loop {
        match pc {
            0x82FABCC0 => {
    //   block [0x82FABCC0..0x82FABCE0)
	// 82FABCC0: 8965000D  lbz r11, 0xd(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FABCC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FABCC8: 419A0018  beq cr6, 0x82fabce0
	if ctx.cr[6].eq {
		sub_82FABCE0(ctx, base);
		return;
	}
	// 82FABCCC: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABCD0: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FABCD4: C1830010  lfs f12, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FABCD8: EC2C0372  fmuls f1, f12, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FABCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABCE0 size=32
    let mut pc: u32 = 0x82FABCE0;
    'dispatch: loop {
        match pc {
            0x82FABCE0 => {
    //   block [0x82FABCE0..0x82FABD00)
	// 82FABCE0: 8965000E  lbz r11, 0xe(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(14 as u32) ) } as u64;
	// 82FABCE4: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABCE8: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FABCEC: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82FABCF0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FABCF4: 7DA8542E  lfsx f13, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABCF8: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FABCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FABD00 size=240
    let mut pc: u32 = 0x82FABD00;
    'dispatch: loop {
        match pc {
            0x82FABD00 => {
    //   block [0x82FABD00..0x82FABDF0)
	// 82FABD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FABD04: 481FC469  bl 0x831a816c
	ctx.lr = 0x82FABD08;
	sub_831A8130(ctx, base);
	// 82FABD08: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82FABD0C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FABD10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FABD14: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82FABD18: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FABD1C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FABD20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FABD24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FABD28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABD2C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FABD30: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FABD34: 4E800421  bctrl
	ctx.lr = 0x82FABD38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FABD38: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FABD3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FABD40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FABD44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FABD48: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABD4C: 991E000D  stb r8, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[8].u8 ) };
	// 82FABD50: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABD54: 80C70010  lwz r6, 0x10(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FABD58: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82FABD5C: 4E800421  bctrl
	ctx.lr = 0x82FABD60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FABD60: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FABD64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABD68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FABD6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FABD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FABD74: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FABD78: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FABD7C: 4E800421  bctrl
	ctx.lr = 0x82FABD80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FABD80: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FABD84: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FABD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FABD8C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FABD90: 40990034  ble cr6, 0x82fabdc4
	if !ctx.cr[6].gt {
	pc = 0x82FABDC4; continue 'dispatch;
	}
	// 82FABD94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FABD98: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FABD9C: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABDA0: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FABDA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FABDA8: 7DA95C2E  lfsx f13, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABDAC: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABDB0: 7D885D2E  stfsx f12, r8, r11
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82FABDB4: 80FF002C  lwz r7, 0x2c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FABDB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FABDBC: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82FABDC0: 4198FFD8  blt cr6, 0x82fabd98
	if ctx.cr[6].lt {
	pc = 0x82FABD98; continue 'dispatch;
	}
	// 82FABDC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABDC8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FABDCC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FABDD0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FABDD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FABDD8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FABDDC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FABDE0: 4E800421  bctrl
	ctx.lr = 0x82FABDE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FABDE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FABDE8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82FABDEC: 481FC3D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FABDF0 size=296
    let mut pc: u32 = 0x82FABDF0;
    'dispatch: loop {
        match pc {
            0x82FABDF0 => {
    //   block [0x82FABDF0..0x82FABF18)
	// 82FABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FABDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FABDF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FABDFC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82FABE00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FABE04: 8144001C  lwz r10, 0x1c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FABE08: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FABE0C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FABE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FABE14: 88EA0020  lbz r7, 0x20(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FABE18: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82FABE1C: C00845DC  lfs f0, 0x45dc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(17884 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABE20: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 82FABE24: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82FABE28: 41980074  blt cr6, 0x82fabe9c
	if ctx.cr[6].lt {
	pc = 0x82FABE9C; continue 'dispatch;
	}
	// 82FABE2C: 3947FFFC  addi r10, r7, -4
	ctx.r[10].s64 = ctx.r[7].s64 + -4;
	// 82FABE30: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FABE34: 80C40048  lwz r6, 0x48(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FABE38: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FABE3C: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82FABE40: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82FABE44: 39460160  addi r10, r6, 0x160
	ctx.r[10].s64 = ctx.r[6].s64 + 352;
	// 82FABE48: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FABE4C: C1AAFF40  lfs f13, -0xc0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABE50: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FABE54: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABE58: C169FFF8  lfs f11, -8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FABE5C: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FABE60: ED2A0032  fmuls f9, f10, f0
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABE64: C109FFFC  lfs f8, -4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FABE68: C0EA00C0  lfs f7, 0xc0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FABE6C: ECC70032  fmuls f6, f7, f0
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABE70: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82FABE74: C08A0180  lfs f4, 0x180(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(384 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82FABE78: 394A0300  addi r10, r10, 0x300
	ctx.r[10].s64 = ctx.r[10].s64 + 768;
	// 82FABE7C: EC640032  fmuls f3, f4, f0
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABE80: C0490004  lfs f2, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82FABE84: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82FABE88: EC2CFAFA  fmadds f1, f12, f11, f31
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[31].f64) as f32) as f64);
	// 82FABE8C: EDA90A3A  fmadds f13, f9, f8, f1
	ctx.f[13].f64 = (((ctx.f[9].f64 * ctx.f[8].f64 + ctx.f[1].f64) as f32) as f64);
	// 82FABE90: ED86697A  fmadds f12, f6, f5, f13
	ctx.f[12].f64 = (((ctx.f[6].f64 * ctx.f[5].f64 + ctx.f[13].f64) as f32) as f64);
	// 82FABE94: EFE360BA  fmadds f31, f3, f2, f12
	ctx.f[31].f64 = (((ctx.f[3].f64 * ctx.f[2].f64 + ctx.f[12].f64) as f32) as f64);
	// 82FABE98: 4082FFB4  bne 0x82fabe4c
	if !ctx.cr[0].eq {
	pc = 0x82FABE4C; continue 'dispatch;
	}
	// 82FABE9C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82FABEA0: 4098004C  bge cr6, 0x82fabeec
	if !ctx.cr[6].lt {
	pc = 0x82FABEEC; continue 'dispatch;
	}
	// 82FABEA4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FABEA8: 80C30028  lwz r6, 0x28(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FABEAC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FABEB0: 81040048  lwz r8, 0x48(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FABEB4: 7FEB4A14  add r31, r11, r9
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FABEB8: 7D2A3214  add r9, r10, r6
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82FABEBC: 57EA3032  slwi r10, r31, 6
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FABEC0: 7D6B3850  subf r11, r11, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 82FABEC4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82FABEC8: 394A00A0  addi r10, r10, 0xa0
	ctx.r[10].s64 = ctx.r[10].s64 + 160;
	// 82FABECC: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABED0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FABED4: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FABED8: C1690000  lfs f11, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FABEDC: 394A00C0  addi r10, r10, 0xc0
	ctx.r[10].s64 = ctx.r[10].s64 + 192;
	// 82FABEE0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82FABEE4: EFECFAFA  fmadds f31, f12, f11, f31
	ctx.f[31].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[31].f64) as f32) as f64);
	// 82FABEE8: 4082FFE4  bne 0x82fabecc
	if !ctx.cr[0].eq {
	pc = 0x82FABECC; continue 'dispatch;
	}
	// 82FABEEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FABEF0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FABEF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FABEF8: 4E800421  bctrl
	ctx.lr = 0x82FABEFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FABEFC: EC2107F2  fmuls f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 82FABF00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FABF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FABF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FABF0C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FABF10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FABF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABF18 size=60
    let mut pc: u32 = 0x82FABF18;
    'dispatch: loop {
        match pc {
            0x82FABF18 => {
    //   block [0x82FABF18..0x82FABF54)
	// 82FABF18: C0060010  lfs f0, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABF1C: 8966000F  lbz r11, 0xf(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(15 as u32) ) } as u64;
	// 82FABF20: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82FABF24: D1A60010  stfs f13, 0x10(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FABF28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FABF2C: 419A001C  beq cr6, 0x82fabf48
	if ctx.cr[6].eq {
	pc = 0x82FABF48; continue 'dispatch;
	}
	// 82FABF30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FABF34: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABF38: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FABF3C: 4199000C  bgt cr6, 0x82fabf48
	if ctx.cr[6].gt {
	pc = 0x82FABF48; continue 'dispatch;
	}
	// 82FABF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FABF44: 9966000F  stb r11, 0xf(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 82FABF48: 8966000D  lbz r11, 0xd(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FABF4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FABF50: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABF54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABF54 size=72
    let mut pc: u32 = 0x82FABF54;
    'dispatch: loop {
        match pc {
            0x82FABF54 => {
    //   block [0x82FABF54..0x82FABF9C)
	// 82FABF54: C0060000  lfs f0, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABF58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FABF5C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABF60: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FABF64: 40980028  bge cr6, 0x82fabf8c
	if !ctx.cr[6].lt {
	pc = 0x82FABF8C; continue 'dispatch;
	}
	// 82FABF68: 8966000E  lbz r11, 0xe(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as u64;
	// 82FABF6C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82FABF70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FABF74: 40990018  ble cr6, 0x82fabf8c
	if !ctx.cr[6].gt {
	pc = 0x82FABF8C; continue 'dispatch;
	}
	// 82FABF78: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FABF7C: 9966000E  stb r11, 0xe(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 82FABF80: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABF84: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FABF88: 9946000F  stb r10, 0xf(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82FABF8C: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABF90: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FABF94: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FABF98: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABF9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FABF9C size=24
    let mut pc: u32 = 0x82FABF9C;
    'dispatch: loop {
        match pc {
            0x82FABF9C => {
    //   block [0x82FABF9C..0x82FABFB4)
	// 82FABF9C: 8966000E  lbz r11, 0xe(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(14 as u32) ) } as u64;
	// 82FABFA0: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FABFA4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82FABFA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FABFAC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FABFB0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABFB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABFB4 size=20
    let mut pc: u32 = 0x82FABFB4;
    'dispatch: loop {
        match pc {
            0x82FABFB4 => {
    //   block [0x82FABFB4..0x82FABFC8)
	// 82FABFB4: 9966000E  stb r11, 0xe(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 82FABFB8: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABFBC: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FABFC0: 9946000F  stb r10, 0xf(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82FABFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FABFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FABFC8 size=84
    let mut pc: u32 = 0x82FABFC8;
    'dispatch: loop {
        match pc {
            0x82FABFC8 => {
    //   block [0x82FABFC8..0x82FAC01C)
	// 82FABFC8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FABFCC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FABFD0: 390B415C  addi r8, r11, 0x415c
	ctx.r[8].s64 = ctx.r[11].s64 + 16732;
	// 82FABFD4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82FABFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FABFDC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FABFE0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FABFE4: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82FABFE8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FABFEC: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FABFF0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FABFF4: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82FABFF8: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FABFFC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FAC000: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82FAC004: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAC008: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FAC00C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FAC010: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82FAC014: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82FAC018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAC020 size=140
    let mut pc: u32 = 0x82FAC020;
    'dispatch: loop {
        match pc {
            0x82FAC020 => {
    //   block [0x82FAC020..0x82FAC0AC)
	// 82FAC020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC024: 481FC149  bl 0x831a816c
	ctx.lr = 0x82FAC028;
	sub_831A8130(ctx, base);
	// 82FAC028: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC02C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FAC030: C0060008  lfs f0, 8(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC034: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82FAC038: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FAC03C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FAC040: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC044: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FAC048: D19F0000  stfs f12, 0(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAC04C: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAC050: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAC054: 38AB0040  addi r5, r11, 0x40
	ctx.r[5].s64 = ctx.r[11].s64 + 64;
	// 82FAC058: 388A00E0  addi r4, r10, 0xe0
	ctx.r[4].s64 = ctx.r[10].s64 + 224;
	// 82FAC05C: 4BEFAEA5  bl 0x82ea6f00
	ctx.lr = 0x82FAC060;
	sub_82EA6F00(ctx, base);
	// 82FAC060: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82FAC064: 811D0018  lwz r8, 0x18(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAC068: 394001A0  li r10, 0x1a0
	ctx.r[10].s64 = 416;
	// 82FAC06C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAC0B0 size=152
    let mut pc: u32 = 0x82FAC0B0;
    'dispatch: loop {
        match pc {
            0x82FAC0B0 => {
    //   block [0x82FAC0B0..0x82FAC148)
	// 82FAC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC0B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAC0BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAC0C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC0C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAC0C8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FAC0CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC0D0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAC0D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC0D8: 4E800421  bctrl
	ctx.lr = 0x82FAC0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAC0DC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAC0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAC0E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAC0E8: 40990048  ble cr6, 0x82fac130
	if !ctx.cr[6].gt {
	pc = 0x82FAC130; continue 'dispatch;
	}
	// 82FAC0EC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FAC0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAC0F4: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC0F8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAC0FC: 7D0950AE  lbzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAC100: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAC104: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FAC108: 419A0010  beq cr6, 0x82fac118
	if ctx.cr[6].eq {
	pc = 0x82FAC118; continue 'dispatch;
	}
	// 82FAC10C: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC110: 7DA95D2E  stfsx f13, r9, r11
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82FAC114: 48000008  b 0x82fac11c
	pc = 0x82FAC11C; continue 'dispatch;
	// 82FAC118: 7C095D2E  stfsx f0, r9, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82FAC11C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAC120: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FAC124: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FAC128: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAC12C: 4198FFCC  blt cr6, 0x82fac0f8
	if ctx.cr[6].lt {
	pc = 0x82FAC0F8; continue 'dispatch;
	}
	// 82FAC130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAC134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC13C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAC140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAC144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC148 size=60
    let mut pc: u32 = 0x82FAC148;
    'dispatch: loop {
        match pc {
            0x82FAC148 => {
    //   block [0x82FAC148..0x82FAC184)
	// 82FAC148: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAC14C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FAC150: 392B422C  addi r9, r11, 0x422c
	ctx.r[9].s64 = ctx.r[11].s64 + 16940;
	// 82FAC154: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FAC158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAC15C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAC160: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82FAC164: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82FAC168: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC16C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FAC170: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FAC174: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82FAC178: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAC17C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FAC180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC188 size=64
    let mut pc: u32 = 0x82FAC188;
    'dispatch: loop {
        match pc {
            0x82FAC188 => {
    //   block [0x82FAC188..0x82FAC1C8)
	// 82FAC188: C1A70000  lfs f13, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC18C: D1A80004  stfs f13, 4(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FAC190: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAC194: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82FAC198: C1260000  lfs f9, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FAC19C: 40980048  bge cr6, 0x82fac1e4
	if !ctx.cr[6].lt {
		sub_82FAC1C8(ctx, base);
		return;
	}
	// 82FAC1A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAC1A4: C003002C  lfs f0, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC1A8: ED600272  fmuls f11, f0, f9
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 82FAC1AC: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC1B0: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FAC1B4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FAC1B8: 40980010  bge cr6, 0x82fac1c8
	if !ctx.cr[6].lt {
		sub_82FAC1C8(ctx, base);
		return;
	}
	// 82FAC1BC: EC0C582A  fadds f0, f12, f11
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 82FAC1C0: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FAC1C4: 48000020  b 0x82fac1e4
	sub_82FAC1C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC1C8 size=168
    let mut pc: u32 = 0x82FAC1C8;
    'dispatch: loop {
        match pc {
            0x82FAC1C8 => {
    //   block [0x82FAC1C8..0x82FAC270)
	// 82FAC1C8: EDAD0028  fsubs f13, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82FAC1CC: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAC1D0: ED4C0028  fsubs f10, f12, f0
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82FAC1D4: ED0D02F2  fmuls f8, f13, f11
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82FAC1D8: ECE85024  fdivs f7, f8, f10
	ctx.f[7].f64 = ((ctx.f[8].f64 / ctx.f[10].f64) as f32) as f64;
	// 82FAC1DC: ECC7602A  fadds f6, f7, f12
	ctx.f[6].f64 = ((ctx.f[7].f64 + ctx.f[12].f64) as f32) as f64;
	// 82FAC1E0: D0C80004  stfs f6, 4(r8)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FAC1E4: C1680004  lfs f11, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAC1E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAC1EC: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC1F0: EC0B6828  fsubs f0, f11, f13
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FAC1F4: C14B08A4  lfs f10, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAC1F8: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FAC1FC: 40980074  bge cr6, 0x82fac270
	if !ctx.cr[6].lt {
		sub_82FAC270(ctx, base);
		return;
	}
	// 82FAC200: C1630008  lfs f11, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAC204: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAC208: ED4B6828  fsubs f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FAC20C: C1030018  lfs f8, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FAC210: C0E30024  lfs f7, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FAC214: C0C30020  lfs f6, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82FAC218: ECA63828  fsubs f5, f6, f7
	ctx.f[5].f64 = (((ctx.f[6].f64 - ctx.f[7].f64) as f32) as f64);
	// 82FAC21C: C1870000  lfs f12, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAC220: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC224: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82FAC228: EC686828  fsubs f3, f8, f13
	ctx.f[3].f64 = (((ctx.f[8].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FAC22C: C0830014  lfs f4, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82FAC230: EC4D5024  fdivs f2, f13, f10
	ctx.f[2].f64 = ((ctx.f[13].f64 / ctx.f[10].f64) as f32) as f64;
	// 82FAC234: EC2300B2  fmuls f1, f3, f2
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[2].f64) as f32) as f64);
	// 82FAC238: ED6500B2  fmuls f11, f5, f2
	ctx.f[11].f64 = (((ctx.f[5].f64 * ctx.f[2].f64) as f32) as f64);
	// 82FAC23C: ED4100B2  fmuls f10, f1, f2
	ctx.f[10].f64 = (((ctx.f[1].f64 * ctx.f[2].f64) as f32) as f64);
	// 82FAC240: ED0B383A  fmadds f8, f11, f0, f7
	ctx.f[8].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64);
	// 82FAC244: ECEA0032  fmuls f7, f10, f0
	ctx.f[7].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FAC248: ED680132  fmuls f11, f8, f4
	ctx.f[11].f64 = (((ctx.f[8].f64 * ctx.f[4].f64) as f32) as f64);
	// 82FAC24C: ECC7683A  fmadds f6, f7, f0, f13
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82FAC250: EC060132  fmuls f0, f6, f4
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[4].f64) as f32) as f64);
	// 82FAC254: 40980098  bge cr6, 0x82fac2ec
	if !ctx.cr[6].lt {
		sub_82FAC2EC(ctx, base);
		return;
	}
	// 82FAC258: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC25C: ED8C6824  fdivs f12, f12, f13
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 82FAC260: ED6C02F2  fmuls f11, f12, f11
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82FAC264: ED405A78  fmsubs f10, f0, f9, f11
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[9].f64 - ctx.f[11].f64) as f32) as f64);
	// 82FAC268: D1480000  stfs f10, 0(r8)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAC26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC270 size=96
    let mut pc: u32 = 0x82FAC270;
    'dispatch: loop {
        match pc {
            0x82FAC270 => {
    //   block [0x82FAC270..0x82FAC2D0)
	// 82FAC270: C1830010  lfs f12, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAC274: FF0B6000  fcmpu cr6, f11, f12
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[12].f64);
	// 82FAC278: 40980058  bge cr6, 0x82fac2d0
	if !ctx.cr[6].lt {
		sub_82FAC2D0(ctx, base);
		return;
	}
	// 82FAC27C: ED8C6828  fsubs f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FAC280: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAC284: C163001C  lfs f11, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAC288: C1430024  lfs f10, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAC28C: C1030028  lfs f8, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FAC290: ECE85028  fsubs f7, f8, f10
	ctx.f[7].f64 = (((ctx.f[8].f64 - ctx.f[10].f64) as f32) as f64);
	// 82FAC294: C0C30014  lfs f6, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82FAC298: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC29C: ECAB6828  fsubs f5, f11, f13
	ctx.f[5].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FAC2A0: EC8D6024  fdivs f4, f13, f12
	ctx.f[4].f64 = ((ctx.f[13].f64 / ctx.f[12].f64) as f32) as f64;
	// 82FAC2A4: EC650132  fmuls f3, f5, f4
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[4].f64) as f32) as f64);
	// 82FAC2A8: EC470132  fmuls f2, f7, f4
	ctx.f[2].f64 = (((ctx.f[7].f64 * ctx.f[4].f64) as f32) as f64);
	// 82FAC2AC: EC230132  fmuls f1, f3, f4
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[4].f64) as f32) as f64);
	// 82FAC2B0: ED82503A  fmadds f12, f2, f0, f10
	ctx.f[12].f64 = (((ctx.f[2].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 82FAC2B4: ED610032  fmuls f11, f1, f0
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FAC2B8: ED4C01B2  fmuls f10, f12, f6
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[6].f64) as f32) as f64);
	// 82FAC2BC: ED0B683A  fmadds f8, f11, f0, f13
	ctx.f[8].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82FAC2C0: ECE801B2  fmuls f7, f8, f6
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[6].f64) as f32) as f64);
	// 82FAC2C4: ECC75278  fmsubs f6, f7, f9, f10
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[9].f64 - ctx.f[10].f64) as f32) as f64);
	// 82FAC2C8: D0C80000  stfs f6, 0(r8)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAC2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC2D0 size=28
    let mut pc: u32 = 0x82FAC2D0;
    'dispatch: loop {
        match pc {
            0x82FAC2D0 => {
    //   block [0x82FAC2D0..0x82FAC2EC)
	// 82FAC2D0: D1880004  stfs f12, 4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FAC2D4: C0030028  lfs f0, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC2D8: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC2DC: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FAC2E0: ED6A6278  fmsubs f11, f10, f9, f12
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[9].f64 - ctx.f[12].f64) as f32) as f64);
	// 82FAC2E4: D1680000  stfs f11, 0(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAC2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC2EC size=12
    let mut pc: u32 = 0x82FAC2EC;
    'dispatch: loop {
        match pc {
            0x82FAC2EC => {
    //   block [0x82FAC2EC..0x82FAC2F8)
	// 82FAC2EC: EC005A78  fmsubs f0, f0, f9, f11
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64 - ctx.f[11].f64) as f32) as f64);
	// 82FAC2F0: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAC2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC2F8 size=120
    let mut pc: u32 = 0x82FAC2F8;
    'dispatch: loop {
        match pc {
            0x82FAC2F8 => {
    //   block [0x82FAC2F8..0x82FAC370)
	// 82FAC2F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82FAC2FC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAC300: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82FAC304: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82FAC308: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82FAC30C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82FAC310: C18BDD6C  lfs f12, -0x2294(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAC314: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82FAC318: C16A45E0  lfs f11, 0x45e0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(17888 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAC31C: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82FAC320: C149E37C  lfs f10, -0x1c84(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-7300 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAC324: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAC328: C128D7B8  lfs f9, -0x2848(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10312 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FAC32C: 3944440C  addi r10, r4, 0x440c
	ctx.r[10].s64 = ctx.r[4].s64 + 17420;
	// 82FAC330: C1A708A8  lfs f13, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC334: C00608A4  lfs f0, 0x8a4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC338: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82FAC33C: C1054430  lfs f8, 0x4430(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(17456 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FAC340: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FAC344: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAC348: D163000C  stfs f11, 0xc(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FAC34C: D1430010  stfs f10, 0x10(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FAC350: D1230014  stfs f9, 0x14(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82FAC354: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82FAC358: D1A3001C  stfs f13, 0x1c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82FAC35C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82FAC360: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82FAC364: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82FAC368: D103002C  stfs f8, 0x2c(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82FAC36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC370 size=44
    let mut pc: u32 = 0x82FAC370;
    'dispatch: loop {
        match pc {
            0x82FAC370 => {
    //   block [0x82FAC370..0x82FAC39C)
	// 82FAC370: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC374: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAC378: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FAC37C: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82FAC380: C1630008  lfs f11, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAC384: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC388: ED4C02F2  fmuls f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82FAC38C: ED2A0072  fmuls f9, f10, f1
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[1].f64) as f32) as f64);
	// 82FAC390: ED090072  fmuls f8, f9, f1
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[1].f64) as f32) as f64);
	// 82FAC394: EC280032  fmuls f1, f8, f0
	ctx.f[1].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FAC398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC3A0 size=64
    let mut pc: u32 = 0x82FAC3A0;
    'dispatch: loop {
        match pc {
            0x82FAC3A0 => {
    //   block [0x82FAC3A0..0x82FAC3E0)
	// 82FAC3A0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FAC3A4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAC3A8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82FAC3AC: 390944FC  addi r8, r9, 0x44fc
	ctx.r[8].s64 = ctx.r[9].s64 + 17660;
	// 82FAC3B0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82FAC3B4: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC3B8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FAC3BC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FAC3C0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82FAC3C4: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FAC3C8: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82FAC3CC: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAC3E0 size=48
    let mut pc: u32 = 0x82FAC3E0;
    'dispatch: loop {
        match pc {
            0x82FAC3E0 => {
    //   block [0x82FAC3E0..0x82FAC410)
	// 82FAC3E0: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82FAC3E4: C1830010  lfs f12, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FAC3E8: C163000C  lfs f11, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FAC3EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAC3F0: C1430008  lfs f10, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FAC3F4: C00B9530  lfs f0, -0x6ad0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27344 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAC3F8: ED2D0332  fmuls f9, f13, f12
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82FAC3FC: ED0902F2  fmuls f8, f9, f11
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 82FAC400: ECE802B2  fmuls f7, f8, f10
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[10].f64) as f32) as f64);
	// 82FAC404: ECC70072  fmuls f6, f7, f1
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[1].f64) as f32) as f64);
	// 82FAC408: EC260032  fmuls f1, f6, f0
	ctx.f[1].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FAC40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC410 size=340
    let mut pc: u32 = 0x82FAC410;
    'dispatch: loop {
        match pc {
            0x82FAC410 => {
    //   block [0x82FAC410..0x82FAC564)
	// 82FAC410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC414: 481FBD55  bl 0x831a8168
	ctx.lr = 0x82FAC418;
	sub_831A8130(ctx, base);
	// 82FAC418: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82FAC41C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC568 size=4
    let mut pc: u32 = 0x82FAC568;
    'dispatch: loop {
        match pc {
            0x82FAC568 => {
    //   block [0x82FAC568..0x82FAC56C)
	// 82FAC568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC570 size=4
    let mut pc: u32 = 0x82FAC570;
    'dispatch: loop {
        match pc {
            0x82FAC570 => {
    //   block [0x82FAC570..0x82FAC574)
	// 82FAC570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC578 size=4
    let mut pc: u32 = 0x82FAC578;
    'dispatch: loop {
        match pc {
            0x82FAC578 => {
    //   block [0x82FAC578..0x82FAC57C)
	// 82FAC578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC580 size=20
    let mut pc: u32 = 0x82FAC580;
    'dispatch: loop {
        match pc {
            0x82FAC580 => {
    //   block [0x82FAC580..0x82FAC594)
	// 82FAC580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAC588: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC58C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC590: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC598 size=8
    let mut pc: u32 = 0x82FAC598;
    'dispatch: loop {
        match pc {
            0x82FAC598 => {
    //   block [0x82FAC598..0x82FAC5A0)
	// 82FAC598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FAC59C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC5A0 size=24
    let mut pc: u32 = 0x82FAC5A0;
    'dispatch: loop {
        match pc {
            0x82FAC5A0 => {
    //   block [0x82FAC5A0..0x82FAC5B8)
	// 82FAC5A0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAC5A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAC5A8: 392A4C70  addi r9, r10, 0x4c70
	ctx.r[9].s64 = ctx.r[10].s64 + 19568;
	// 82FAC5AC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82FAC5B0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAC5B4: 4800A23C  b 0x82fb67f0
	sub_82FB67F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC5B8 size=4
    let mut pc: u32 = 0x82FAC5B8;
    'dispatch: loop {
        match pc {
            0x82FAC5B8 => {
    //   block [0x82FAC5B8..0x82FAC5BC)
	// 82FAC5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC5C0 size=12
    let mut pc: u32 = 0x82FAC5C0;
    'dispatch: loop {
        match pc {
            0x82FAC5C0 => {
    //   block [0x82FAC5C0..0x82FAC5CC)
	// 82FAC5C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAC5C4: 386B4C70  addi r3, r11, 0x4c70
	ctx.r[3].s64 = ctx.r[11].s64 + 19568;
	// 82FAC5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC5D0 size=20
    let mut pc: u32 = 0x82FAC5D0;
    'dispatch: loop {
        match pc {
            0x82FAC5D0 => {
    //   block [0x82FAC5D0..0x82FAC5E4)
	// 82FAC5D0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAC5D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC5D8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAC5DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC5E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC5E8 size=20
    let mut pc: u32 = 0x82FAC5E8;
    'dispatch: loop {
        match pc {
            0x82FAC5E8 => {
    //   block [0x82FAC5E8..0x82FAC5FC)
	// 82FAC5E8: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAC5EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC5F0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAC5F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC5F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC600 size=96
    let mut pc: u32 = 0x82FAC600;
    'dispatch: loop {
        match pc {
            0x82FAC600 => {
    //   block [0x82FAC600..0x82FAC660)
	// 82FAC600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC608: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAC60C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAC614: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAC618: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82FAC61C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82FAC620: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FAC624: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAC628: 419A0020  beq cr6, 0x82fac648
	if ctx.cr[6].eq {
	pc = 0x82FAC648; continue 'dispatch;
	}
	// 82FAC62C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC630: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FAC634: 38C00037  li r6, 0x37
	ctx.r[6].s64 = 55;
	// 82FAC638: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAC63C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FAC640: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAC644: 4BEF416D  bl 0x82ea07b0
	ctx.lr = 0x82FAC648;
	sub_82EA07B0(ctx, base);
	// 82FAC648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAC64C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAC650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAC65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC660 size=120
    let mut pc: u32 = 0x82FAC660;
    'dispatch: loop {
        match pc {
            0x82FAC660 => {
    //   block [0x82FAC660..0x82FAC6D8)
	// 82FAC660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC668: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC66C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82FAC670: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82FAC674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAC678: 4BEF9D19  bl 0x82ea6390
	ctx.lr = 0x82FAC67C;
	sub_82EA6390(ctx, base);
	// 82FAC67C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAC680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAC684: 388B4BF0  addi r4, r11, 0x4bf0
	ctx.r[4].s64 = ctx.r[11].s64 + 19440;
	// 82FAC688: 4BEF9151  bl 0x82ea57d8
	ctx.lr = 0x82FAC68C;
	sub_82EA57D8(ctx, base);
	// 82FAC68C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAC690: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAC694: 3CA054E3  lis r5, 0x54e3
	ctx.r[5].s64 = 1424162816;
	// 82FAC698: 3900001B  li r8, 0x1b
	ctx.r[8].s64 = 27;
	// 82FAC69C: 38E94BB0  addi r7, r9, 0x4bb0
	ctx.r[7].s64 = ctx.r[9].s64 + 19376;
	// 82FAC6A0: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82FAC6A4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82FAC6A8: 60A52123  ori r5, r5, 0x2123
	ctx.r[5].u64 = ctx.r[5].u64 | 8483;
	// 82FAC6AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FAC6B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC6B4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAC6B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC6BC: 4E800421  bctrl
	ctx.lr = 0x82FAC6C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAC6C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAC6C4: 4BEF9755  bl 0x82ea5e18
	ctx.lr = 0x82FAC6C8;
	sub_82EA5E18(ctx, base);
	// 82FAC6C8: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82FAC6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC6D8 size=120
    let mut pc: u32 = 0x82FAC6D8;
    'dispatch: loop {
        match pc {
            0x82FAC6D8 => {
    //   block [0x82FAC6D8..0x82FAC750)
	// 82FAC6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC6E0: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC6E4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82FAC6E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82FAC6EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAC6F0: 4BEF9CA1  bl 0x82ea6390
	ctx.lr = 0x82FAC6F4;
	sub_82EA6390(ctx, base);
	// 82FAC6F4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAC6F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAC6FC: 388B4C30  addi r4, r11, 0x4c30
	ctx.r[4].s64 = ctx.r[11].s64 + 19504;
	// 82FAC700: 4BEF90D9  bl 0x82ea57d8
	ctx.lr = 0x82FAC704;
	sub_82EA57D8(ctx, base);
	// 82FAC704: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAC708: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAC70C: 3CA054E3  lis r5, 0x54e3
	ctx.r[5].s64 = 1424162816;
	// 82FAC710: 39000027  li r8, 0x27
	ctx.r[8].s64 = 39;
	// 82FAC714: 38E94BB0  addi r7, r9, 0x4bb0
	ctx.r[7].s64 = ctx.r[9].s64 + 19376;
	// 82FAC718: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82FAC71C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82FAC720: 60A52124  ori r5, r5, 0x2124
	ctx.r[5].u64 = ctx.r[5].u64 | 8484;
	// 82FAC724: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FAC728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC72C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAC730: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC734: 4E800421  bctrl
	ctx.lr = 0x82FAC738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAC738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAC73C: 4BEF96DD  bl 0x82ea5e18
	ctx.lr = 0x82FAC740;
	sub_82EA5E18(ctx, base);
	// 82FAC740: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82FAC744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC750 size=100
    let mut pc: u32 = 0x82FAC750;
    'dispatch: loop {
        match pc {
            0x82FAC750 => {
    //   block [0x82FAC750..0x82FAC7B4)
	// 82FAC750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAC75C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAC760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAC768: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FAC76C: 4800A595  bl 0x82fb6d00
	ctx.lr = 0x82FAC770;
	sub_82FB6D00(ctx, base);
	// 82FAC770: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FAC774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAC778: 419A0020  beq cr6, 0x82fac798
	if ctx.cr[6].eq {
	pc = 0x82FAC798; continue 'dispatch;
	}
	// 82FAC77C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC780: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FAC784: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82FAC788: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAC78C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FAC790: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAC794: 4BEF401D  bl 0x82ea07b0
	ctx.lr = 0x82FAC798;
	sub_82EA07B0(ctx, base);
	// 82FAC798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAC79C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAC7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC7A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAC7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAC7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC7B8 size=4
    let mut pc: u32 = 0x82FAC7B8;
    'dispatch: loop {
        match pc {
            0x82FAC7B8 => {
    //   block [0x82FAC7B8..0x82FAC7BC)
	// 82FAC7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC7C0 size=4
    let mut pc: u32 = 0x82FAC7C0;
    'dispatch: loop {
        match pc {
            0x82FAC7C0 => {
    //   block [0x82FAC7C0..0x82FAC7C4)
	// 82FAC7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC7C8 size=20
    let mut pc: u32 = 0x82FAC7C8;
    'dispatch: loop {
        match pc {
            0x82FAC7C8 => {
    //   block [0x82FAC7C8..0x82FAC7DC)
	// 82FAC7C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC7CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAC7D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC7D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAC7D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC7E0 size=8
    let mut pc: u32 = 0x82FAC7E0;
    'dispatch: loop {
        match pc {
            0x82FAC7E0 => {
    //   block [0x82FAC7E0..0x82FAC7E8)
	// 82FAC7E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FAC7E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC7E8 size=24
    let mut pc: u32 = 0x82FAC7E8;
    'dispatch: loop {
        match pc {
            0x82FAC7E8 => {
    //   block [0x82FAC7E8..0x82FAC800)
	// 82FAC7E8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAC7EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAC7F0: 392A50EC  addi r9, r10, 0x50ec
	ctx.r[9].s64 = ctx.r[10].s64 + 20716;
	// 82FAC7F4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82FAC7F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FAC7FC: 4800CDFC  b 0x82fb95f8
	sub_82FB95F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC800 size=4
    let mut pc: u32 = 0x82FAC800;
    'dispatch: loop {
        match pc {
            0x82FAC800 => {
    //   block [0x82FAC800..0x82FAC804)
	// 82FAC800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC808 size=12
    let mut pc: u32 = 0x82FAC808;
    'dispatch: loop {
        match pc {
            0x82FAC808 => {
    //   block [0x82FAC808..0x82FAC814)
	// 82FAC808: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAC80C: 386B50EC  addi r3, r11, 0x50ec
	ctx.r[3].s64 = ctx.r[11].s64 + 20716;
	// 82FAC810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC818 size=100
    let mut pc: u32 = 0x82FAC818;
    'dispatch: loop {
        match pc {
            0x82FAC818 => {
    //   block [0x82FAC818..0x82FAC87C)
	// 82FAC818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAC824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAC828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC82C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAC830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FAC834: 4800F24D  bl 0x82fbba80
	ctx.lr = 0x82FAC838;
	sub_82FBBA80(ctx, base);
	// 82FAC838: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FAC83C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAC840: 419A0020  beq cr6, 0x82fac860
	if ctx.cr[6].eq {
	pc = 0x82FAC860; continue 'dispatch;
	}
	// 82FAC844: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC848: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FAC84C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82FAC850: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAC854: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FAC858: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAC85C: 4BEF3F55  bl 0x82ea07b0
	ctx.lr = 0x82FAC860;
	sub_82EA07B0(ctx, base);
	// 82FAC860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAC864: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAC868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAC874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAC878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC880 size=4
    let mut pc: u32 = 0x82FAC880;
    'dispatch: loop {
        match pc {
            0x82FAC880 => {
    //   block [0x82FAC880..0x82FAC884)
	// 82FAC880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC888 size=4
    let mut pc: u32 = 0x82FAC888;
    'dispatch: loop {
        match pc {
            0x82FAC888 => {
    //   block [0x82FAC888..0x82FAC88C)
	// 82FAC888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC890 size=4
    let mut pc: u32 = 0x82FAC890;
    'dispatch: loop {
        match pc {
            0x82FAC890 => {
    //   block [0x82FAC890..0x82FAC894)
	// 82FAC890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC898 size=8
    let mut pc: u32 = 0x82FAC898;
    'dispatch: loop {
        match pc {
            0x82FAC898 => {
    //   block [0x82FAC898..0x82FAC8A0)
	// 82FAC898: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAC89C: 480000B4  b 0x82fac950
	sub_82FAC950(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC8A0 size=176
    let mut pc: u32 = 0x82FAC8A0;
    'dispatch: loop {
        match pc {
            0x82FAC8A0 => {
    //   block [0x82FAC8A0..0x82FAC950)
	// 82FAC8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAC8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAC8B4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FAC8B8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAC8BC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAC8C0: 409A0020  bne cr6, 0x82fac8e0
	if !ctx.cr[6].eq {
	pc = 0x82FAC8E0; continue 'dispatch;
	}
	// 82FAC8C4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC8C8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FAC8CC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FAC8D0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAC8D4: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82FAC8D8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAC8DC: 4BEF3ED5  bl 0x82ea07b0
	ctx.lr = 0x82FAC8E0;
	sub_82EA07B0(ctx, base);
	// 82FAC8E0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAC8E4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAC8E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAC8EC: 409A0024  bne cr6, 0x82fac910
	if !ctx.cr[6].eq {
	pc = 0x82FAC910; continue 'dispatch;
	}
	// 82FAC8F0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC8F4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FAC8F8: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAC8FC: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAC900: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FAC904: 1CA80070  mulli r5, r8, 0x70
	ctx.r[5].s64 = ctx.r[8].s64 * 112;
	// 82FAC908: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAC90C: 4BEF3EA5  bl 0x82ea07b0
	ctx.lr = 0x82FAC910;
	sub_82EA07B0(ctx, base);
	// 82FAC910: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAC914: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAC918: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAC91C: 409A0020  bne cr6, 0x82fac93c
	if !ctx.cr[6].eq {
	pc = 0x82FAC93C; continue 'dispatch;
	}
	// 82FAC920: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC924: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FAC928: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FAC92C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAC930: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FAC934: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAC938: 4BEF3E79  bl 0x82ea07b0
	ctx.lr = 0x82FAC93C;
	sub_82EA07B0(ctx, base);
	// 82FAC93C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAC940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAC94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAC950 size=160
    let mut pc: u32 = 0x82FAC950;
    'dispatch: loop {
        match pc {
            0x82FAC950 => {
    //   block [0x82FAC950..0x82FAC9F0)
	// 82FAC950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAC954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAC958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAC95C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAC960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAC964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAC968: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FAC96C: 4BFFFF35  bl 0x82fac8a0
	ctx.lr = 0x82FAC970;
	sub_82FAC8A0(ctx, base);
	// 82FAC970: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FAC974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAC978: 419A005C  beq cr6, 0x82fac9d4
	if ctx.cr[6].eq {
	pc = 0x82FAC9D4; continue 'dispatch;
	}
	// 82FAC97C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FAC980: 419A0054  beq cr6, 0x82fac9d4
	if ctx.cr[6].eq {
	pc = 0x82FAC9D4; continue 'dispatch;
	}
	// 82FAC984: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAC988: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FAC98C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAC990: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FAC994: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FAC998: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FAC99C: 4198001C  blt cr6, 0x82fac9b8
	if ctx.cr[6].lt {
	pc = 0x82FAC9B8; continue 'dispatch;
	}
	// 82FAC9A0: 38C0003B  li r6, 0x3b
	ctx.r[6].s64 = 59;
	// 82FAC9A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FAC9A8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FAC9AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FAC9B0: 4BEF36B1  bl 0x82ea0060
	ctx.lr = 0x82FAC9B4;
	sub_82EA0060(ctx, base);
	// 82FAC9B4: 48000020  b 0x82fac9d4
	pc = 0x82FAC9D4; continue 'dispatch;
	// 82FAC9B8: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FAC9BC: 394B0058  addi r10, r11, 0x58
	ctx.r[10].s64 = ctx.r[11].s64 + 88;
	// 82FAC9C0: 814B0058  lwz r10, 0x58(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FAC9C4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FAC9C8: 912B005C  stw r9, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82FAC9CC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FAC9D0: 93EB0058  stw r31, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82FAC9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAC9D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAC9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAC9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAC9E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAC9E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAC9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC9F0 size=4
    let mut pc: u32 = 0x82FAC9F0;
    'dispatch: loop {
        match pc {
            0x82FAC9F0 => {
    //   block [0x82FAC9F0..0x82FAC9F4)
	// 82FAC9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAC9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAC9F8 size=4
    let mut pc: u32 = 0x82FAC9F8;
    'dispatch: loop {
        match pc {
            0x82FAC9F8 => {
    //   block [0x82FAC9F8..0x82FAC9FC)
	// 82FAC9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA00 size=4
    let mut pc: u32 = 0x82FACA00;
    'dispatch: loop {
        match pc {
            0x82FACA00 => {
    //   block [0x82FACA00..0x82FACA04)
	// 82FACA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA08 size=4
    let mut pc: u32 = 0x82FACA08;
    'dispatch: loop {
        match pc {
            0x82FACA08 => {
    //   block [0x82FACA08..0x82FACA0C)
	// 82FACA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA10 size=4
    let mut pc: u32 = 0x82FACA10;
    'dispatch: loop {
        match pc {
            0x82FACA10 => {
    //   block [0x82FACA10..0x82FACA14)
	// 82FACA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA18 size=4
    let mut pc: u32 = 0x82FACA18;
    'dispatch: loop {
        match pc {
            0x82FACA18 => {
    //   block [0x82FACA18..0x82FACA1C)
	// 82FACA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA20 size=20
    let mut pc: u32 = 0x82FACA20;
    'dispatch: loop {
        match pc {
            0x82FACA20 => {
    //   block [0x82FACA20..0x82FACA34)
	// 82FACA20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACA24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FACA28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACA2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FACA30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA38 size=8
    let mut pc: u32 = 0x82FACA38;
    'dispatch: loop {
        match pc {
            0x82FACA38 => {
    //   block [0x82FACA38..0x82FACA40)
	// 82FACA38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FACA3C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA40 size=24
    let mut pc: u32 = 0x82FACA40;
    'dispatch: loop {
        match pc {
            0x82FACA40 => {
    //   block [0x82FACA40..0x82FACA58)
	// 82FACA40: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FACA44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FACA48: 392A5814  addi r9, r10, 0x5814
	ctx.r[9].s64 = ctx.r[10].s64 + 22548;
	// 82FACA4C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82FACA50: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FACA54: 480101B4  b 0x82fbcc08
	sub_82FBCC08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA58 size=4
    let mut pc: u32 = 0x82FACA58;
    'dispatch: loop {
        match pc {
            0x82FACA58 => {
    //   block [0x82FACA58..0x82FACA5C)
	// 82FACA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACA60 size=12
    let mut pc: u32 = 0x82FACA60;
    'dispatch: loop {
        match pc {
            0x82FACA60 => {
    //   block [0x82FACA60..0x82FACA6C)
	// 82FACA60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACA64: 386B5814  addi r3, r11, 0x5814
	ctx.r[3].s64 = ctx.r[11].s64 + 22548;
	// 82FACA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FACA70 size=100
    let mut pc: u32 = 0x82FACA70;
    'dispatch: loop {
        match pc {
            0x82FACA70 => {
    //   block [0x82FACA70..0x82FACAD4)
	// 82FACA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FACA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FACA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FACA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACA84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FACA88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FACA8C: 4801062D  bl 0x82fbd0b8
	ctx.lr = 0x82FACA90;
	sub_82FBD0B8(ctx, base);
	// 82FACA90: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FACA94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FACA98: 419A0020  beq cr6, 0x82facab8
	if ctx.cr[6].eq {
	pc = 0x82FACAB8; continue 'dispatch;
	}
	// 82FACA9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACAA0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FACAA4: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82FACAA8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACAAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FACAB0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FACAB4: 4BEF3CFD  bl 0x82ea07b0
	ctx.lr = 0x82FACAB8;
	sub_82EA07B0(ctx, base);
	// 82FACAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FACABC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FACAC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FACAC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FACAC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FACACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FACAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACAD8 size=4
    let mut pc: u32 = 0x82FACAD8;
    'dispatch: loop {
        match pc {
            0x82FACAD8 => {
    //   block [0x82FACAD8..0x82FACADC)
	// 82FACAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACAE0 size=4
    let mut pc: u32 = 0x82FACAE0;
    'dispatch: loop {
        match pc {
            0x82FACAE0 => {
    //   block [0x82FACAE0..0x82FACAE4)
	// 82FACAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACAE8 size=4
    let mut pc: u32 = 0x82FACAE8;
    'dispatch: loop {
        match pc {
            0x82FACAE8 => {
    //   block [0x82FACAE8..0x82FACAEC)
	// 82FACAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACAF0 size=4
    let mut pc: u32 = 0x82FACAF0;
    'dispatch: loop {
        match pc {
            0x82FACAF0 => {
    //   block [0x82FACAF0..0x82FACAF4)
	// 82FACAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACAF8 size=4
    let mut pc: u32 = 0x82FACAF8;
    'dispatch: loop {
        match pc {
            0x82FACAF8 => {
    //   block [0x82FACAF8..0x82FACAFC)
	// 82FACAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACB00 size=4
    let mut pc: u32 = 0x82FACB00;
    'dispatch: loop {
        match pc {
            0x82FACB00 => {
    //   block [0x82FACB00..0x82FACB04)
	// 82FACB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACB08 size=4
    let mut pc: u32 = 0x82FACB08;
    'dispatch: loop {
        match pc {
            0x82FACB08 => {
    //   block [0x82FACB08..0x82FACB0C)
	// 82FACB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACB10 size=20
    let mut pc: u32 = 0x82FACB10;
    'dispatch: loop {
        match pc {
            0x82FACB10 => {
    //   block [0x82FACB10..0x82FACB24)
	// 82FACB10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACB14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FACB18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACB1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FACB20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACB28 size=8
    let mut pc: u32 = 0x82FACB28;
    'dispatch: loop {
        match pc {
            0x82FACB28 => {
    //   block [0x82FACB28..0x82FACB30)
	// 82FACB28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FACB2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACB30 size=24
    let mut pc: u32 = 0x82FACB30;
    'dispatch: loop {
        match pc {
            0x82FACB30 => {
    //   block [0x82FACB30..0x82FACB48)
	// 82FACB30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACB34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FACB38: 392B5C20  addi r9, r11, 0x5c20
	ctx.r[9].s64 = ctx.r[11].s64 + 23584;
	// 82FACB3C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FACB40: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FACB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACB48 size=12
    let mut pc: u32 = 0x82FACB48;
    'dispatch: loop {
        match pc {
            0x82FACB48 => {
    //   block [0x82FACB48..0x82FACB54)
	// 82FACB48: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACB4C: 386B5C20  addi r3, r11, 0x5c20
	ctx.r[3].s64 = ctx.r[11].s64 + 23584;
	// 82FACB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FACB58 size=100
    let mut pc: u32 = 0x82FACB58;
    'dispatch: loop {
        match pc {
            0x82FACB58 => {
    //   block [0x82FACB58..0x82FACBBC)
	// 82FACB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FACB60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FACB64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FACB68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACB6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FACB70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FACB74: 480030D5  bl 0x82fafc48
	ctx.lr = 0x82FACB78;
	sub_82FAFC48(ctx, base);
	// 82FACB78: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FACB7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FACB80: 419A0020  beq cr6, 0x82facba0
	if ctx.cr[6].eq {
	pc = 0x82FACBA0; continue 'dispatch;
	}
	// 82FACB84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACB88: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FACB8C: 38C0003B  li r6, 0x3b
	ctx.r[6].s64 = 59;
	// 82FACB90: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACB94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FACB98: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FACB9C: 4BEF3C15  bl 0x82ea07b0
	ctx.lr = 0x82FACBA0;
	sub_82EA07B0(ctx, base);
	// 82FACBA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FACBA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FACBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FACBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FACBB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FACBB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FACBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACBC0 size=20
    let mut pc: u32 = 0x82FACBC0;
    'dispatch: loop {
        match pc {
            0x82FACBC0 => {
    //   block [0x82FACBC0..0x82FACBD4)
	// 82FACBC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACBC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FACBC8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACBCC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FACBD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACBD8 size=8
    let mut pc: u32 = 0x82FACBD8;
    'dispatch: loop {
        match pc {
            0x82FACBD8 => {
    //   block [0x82FACBD8..0x82FACBE0)
	// 82FACBD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FACBDC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACBE0 size=24
    let mut pc: u32 = 0x82FACBE0;
    'dispatch: loop {
        match pc {
            0x82FACBE0 => {
    //   block [0x82FACBE0..0x82FACBF8)
	// 82FACBE0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACBE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FACBE8: 392B5C74  addi r9, r11, 0x5c74
	ctx.r[9].s64 = ctx.r[11].s64 + 23668;
	// 82FACBEC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FACBF0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FACBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACBF8 size=12
    let mut pc: u32 = 0x82FACBF8;
    'dispatch: loop {
        match pc {
            0x82FACBF8 => {
    //   block [0x82FACBF8..0x82FACC04)
	// 82FACBF8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACBFC: 386B5C74  addi r3, r11, 0x5c74
	ctx.r[3].s64 = ctx.r[11].s64 + 23668;
	// 82FACC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FACC08 size=100
    let mut pc: u32 = 0x82FACC08;
    'dispatch: loop {
        match pc {
            0x82FACC08 => {
    //   block [0x82FACC08..0x82FACC6C)
	// 82FACC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FACC10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FACC14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FACC18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACC1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FACC20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FACC24: 480125ED  bl 0x82fbf210
	ctx.lr = 0x82FACC28;
	sub_82FBF210(ctx, base);
	// 82FACC28: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FACC2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FACC30: 419A0020  beq cr6, 0x82facc50
	if ctx.cr[6].eq {
	pc = 0x82FACC50; continue 'dispatch;
	}
	// 82FACC34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACC38: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FACC3C: 38C0003A  li r6, 0x3a
	ctx.r[6].s64 = 58;
	// 82FACC40: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACC44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FACC48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FACC4C: 4BEF3B65  bl 0x82ea07b0
	ctx.lr = 0x82FACC50;
	sub_82EA07B0(ctx, base);
	// 82FACC50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FACC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FACC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FACC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FACC60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FACC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FACC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACC70 size=8
    let mut pc: u32 = 0x82FACC70;
    'dispatch: loop {
        match pc {
            0x82FACC70 => {
    //   block [0x82FACC70..0x82FACC78)
	// 82FACC70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FACC74: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACC78 size=24
    let mut pc: u32 = 0x82FACC78;
    'dispatch: loop {
        match pc {
            0x82FACC78 => {
    //   block [0x82FACC78..0x82FACC90)
	// 82FACC78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACC7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FACC80: 392B5D4C  addi r9, r11, 0x5d4c
	ctx.r[9].s64 = ctx.r[11].s64 + 23884;
	// 82FACC84: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FACC88: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FACC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACC90 size=20
    let mut pc: u32 = 0x82FACC90;
    'dispatch: loop {
        match pc {
            0x82FACC90 => {
    //   block [0x82FACC90..0x82FACCA4)
	// 82FACC90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACC94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FACC98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACC9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FACCA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACCA8 size=12
    let mut pc: u32 = 0x82FACCA8;
    'dispatch: loop {
        match pc {
            0x82FACCA8 => {
    //   block [0x82FACCA8..0x82FACCB4)
	// 82FACCA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FACCAC: 386B5D4C  addi r3, r11, 0x5d4c
	ctx.r[3].s64 = ctx.r[11].s64 + 23884;
	// 82FACCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACCB8 size=12
    let mut pc: u32 = 0x82FACCB8;
    'dispatch: loop {
        match pc {
            0x82FACCB8 => {
    //   block [0x82FACCB8..0x82FACCC4)
	// 82FACCB8: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FACCBC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FACCC0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACCC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACCC4 size=76
    let mut pc: u32 = 0x82FACCC4;
    'dispatch: loop {
        match pc {
            0x82FACCC4 => {
    //   block [0x82FACCC4..0x82FACD10)
	// 82FACCC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FACCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FACCCC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82FACCD0: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACCD4: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FACCD8: 7CA8522E  lhzx r5, r8, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FACCDC: 7D0B302E  lwzx r8, r11, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FACCE0: 7CA60734  extsh r6, r5
	ctx.r[6].s64 = ctx.r[5].s16 as i64;
	// 82FACCE4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FACCE8: 41990010  bgt cr6, 0x82faccf8
	if ctx.cr[6].gt {
	pc = 0x82FACCF8; continue 'dispatch;
	}
	// 82FACCEC: 7C860774  extsb r6, r4
	ctx.r[6].s64 = ctx.r[4].s8 as i64;
	// 82FACCF0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FACCF4: 409A0008  bne cr6, 0x82faccfc
	if !ctx.cr[6].eq {
	pc = 0x82FACCFC; continue 'dispatch;
	}
	// 82FACCF8: 98E80004  stb r7, 4(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u8 ) };
	// 82FACCFC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FACD00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FACD04: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FACD08: 4082FFC8  bne 0x82faccd0
	if !ctx.cr[0].eq {
	pc = 0x82FACCD0; continue 'dispatch;
	}
	// 82FACD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FACD10 size=140
    let mut pc: u32 = 0x82FACD10;
    'dispatch: loop {
        match pc {
            0x82FACD10 => {
    //   block [0x82FACD10..0x82FACD9C)
	// 82FACD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACD14: 481FB44D  bl 0x831a8160
	ctx.lr = 0x82FACD18;
	sub_831A8130(ctx, base);
	// 82FACD18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACD1C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FACD20: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FACD24: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FACD28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FACD2C: 837C0010  lwz r27, 0x10(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FACD30: 409A000C  bne cr6, 0x82facd3c
	if !ctx.cr[6].eq {
	pc = 0x82FACD3C; continue 'dispatch;
	}
	// 82FACD34: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 82FACD38: 3BCBA428  addi r30, r11, -0x5bd8
	ctx.r[30].s64 = ctx.r[11].s64 + -23512;
	// 82FACD3C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FACD40: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FACD44: 40990040  ble cr6, 0x82facd84
	if !ctx.cr[6].gt {
	pc = 0x82FACD84; continue 'dispatch;
	}
	// 82FACD48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FACD4C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FACD50: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FACD54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FACD58: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FACD5C: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACD60: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 82FACD64: 4E800421  bctrl
	ctx.lr = 0x82FACD68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FACD68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FACD6C: 419A0024  beq cr6, 0x82facd90
	if ctx.cr[6].eq {
	pc = 0x82FACD90; continue 'dispatch;
	}
	// 82FACD70: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FACD74: 7D7D0734  extsh r29, r11
	ctx.r[29].s64 = ctx.r[11].s16 as i64;
	// 82FACD78: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82FACD7C: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FACD80: 4198FFCC  blt cr6, 0x82facd4c
	if ctx.cr[6].lt {
	pc = 0x82FACD4C; continue 'dispatch;
	}
	// 82FACD84: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FACD88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FACD8C: 481FB424  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FACD90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FACD94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FACD98: 481FB418  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACDA0 size=12
    let mut pc: u32 = 0x82FACDA0;
    'dispatch: loop {
        match pc {
            0x82FACDA0 => {
    //   block [0x82FACDA0..0x82FACDAC)
	// 82FACDA0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FACDA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FACDA8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACDAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACDAC size=36
    let mut pc: u32 = 0x82FACDAC;
    'dispatch: loop {
        match pc {
            0x82FACDAC => {
    //   block [0x82FACDAC..0x82FACDD0)
	// 82FACDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FACDB0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82FACDB4: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FACDB8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FACDBC: 7CEA402E  lwzx r7, r10, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FACDC0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FACDC4: 99270004  stb r9, 4(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 82FACDC8: 4082FFEC  bne 0x82facdb4
	if !ctx.cr[0].eq {
	pc = 0x82FACDB4; continue 'dispatch;
	}
	// 82FACDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACDD0 size=76
    let mut pc: u32 = 0x82FACDD0;
    'dispatch: loop {
        match pc {
            0x82FACDD0 => {
    //   block [0x82FACDD0..0x82FACE1C)
	// 82FACDD0: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FACDD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FACDD8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FACDDC: 40990034  ble cr6, 0x82face10
	if !ctx.cr[6].gt {
	pc = 0x82FACE10; continue 'dispatch;
	}
	// 82FACDE0: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACDE4: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FACDE8: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 82FACDEC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FACDF0: 4198002C  blt cr6, 0x82face1c
	if ctx.cr[6].lt {
		sub_82FACE1C(ctx, base);
		return;
	}
	// 82FACDF4: 80E40010  lwz r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FACDF8: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82FACDFC: 40980020  bge cr6, 0x82face1c
	if !ctx.cr[6].lt {
		sub_82FACE1C(ctx, base);
		return;
	}
	// 82FACE00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FACE04: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FACE08: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FACE0C: 4198FFD8  blt cr6, 0x82facde4
	if ctx.cr[6].lt {
	pc = 0x82FACDE4; continue 'dispatch;
	}
	// 82FACE10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FACE14: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FACE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACE1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACE1C size=12
    let mut pc: u32 = 0x82FACE1C;
    'dispatch: loop {
        match pc {
            0x82FACE1C => {
    //   block [0x82FACE1C..0x82FACE28)
	// 82FACE1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FACE20: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FACE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACE28 size=12
    let mut pc: u32 = 0x82FACE28;
    'dispatch: loop {
        match pc {
            0x82FACE28 => {
    //   block [0x82FACE28..0x82FACE34)
	// 82FACE28: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FACE2C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FACE30: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACE34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACE34 size=60
    let mut pc: u32 = 0x82FACE34;
    'dispatch: loop {
        match pc {
            0x82FACE34 => {
    //   block [0x82FACE34..0x82FACE70)
	// 82FACE34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FACE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FACE3C: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FACE40: 7CEA402E  lwzx r7, r10, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FACE44: 88C70004  lbz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACE48: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FACE4C: 419A0010  beq cr6, 0x82face5c
	if ctx.cr[6].eq {
	pc = 0x82FACE5C; continue 'dispatch;
	}
	// 82FACE50: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACE70 size=16
    let mut pc: u32 = 0x82FACE70;
    'dispatch: loop {
        match pc {
            0x82FACE70 => {
    //   block [0x82FACE70..0x82FACE80)
	// 82FACE70: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FACE74: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82FACE78: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FACE7C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FACE80 size=56
    let mut pc: u32 = 0x82FACE80;
    'dispatch: loop {
        match pc {
            0x82FACE80 => {
    //   block [0x82FACE80..0x82FACEB8)
	// 82FACE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FACE84: 7D0B2050  subf r8, r11, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82FACE88: 80E3000C  lwz r7, 0xc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FACE8C: 7CCA382E  lwzx r6, r10, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FACE90: 88A60004  lbz r5, 4(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FACE94: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82FACE98: 419A000C  beq cr6, 0x82facea4
	if ctx.cr[6].eq {
	pc = 0x82FACEA4; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FACEB8 size=100
    let mut pc: u32 = 0x82FACEB8;
    'dispatch: loop {
        match pc {
            0x82FACEB8 => {
    //   block [0x82FACEB8..0x82FACF1C)
	// 82FACEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACEBC: 481FB2A9  bl 0x831a8164
	ctx.lr = 0x82FACEC0;
	sub_831A8130(ctx, base);
	// 82FACEC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACEC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FACEC8: 419A004C  beq cr6, 0x82facf14
	if ctx.cr[6].eq {
	pc = 0x82FACF14; continue 'dispatch;
	}
	// 82FACECC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FACED0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FACED4: 7F853850  subf r28, r5, r7
	ctx.r[28].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 82FACED8: 7F652050  subf r27, r5, r4
	ctx.r[27].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82FACEDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FACF20 size=100
    let mut pc: u32 = 0x82FACF20;
    'dispatch: loop {
        match pc {
            0x82FACF20 => {
    //   block [0x82FACF20..0x82FACF84)
	// 82FACF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACF24: 481FB245  bl 0x831a8168
	ctx.lr = 0x82FACF28;
	sub_831A8130(ctx, base);
	// 82FACF28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACF2C: 396100BC  addi r11, r1, 0xbc
	ctx.r[11].s64 = ctx.r[1].s64 + 188;
	// 82FACF30: D02100BC  stfs f1, 0xbc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82FACF34: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82FACF38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FACF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FACF88 size=124
    let mut pc: u32 = 0x82FACF88;
    'dispatch: loop {
        match pc {
            0x82FACF88 => {
    //   block [0x82FACF88..0x82FAD004)
	// 82FACF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FACF8C: 481FB1D9  bl 0x831a8164
	ctx.lr = 0x82FACF90;
	sub_831A8130(ctx, base);
	// 82FACF90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FACF94: 396100C4  addi r11, r1, 0xc4
	ctx.r[11].s64 = ctx.r[1].s64 + 196;
	// 82FACF98: D02100C4  stfs f1, 0xc4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82FACF9C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82FACFA0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FACFA4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FACFA8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD008 size=24
    let mut pc: u32 = 0x82FAD008;
    'dispatch: loop {
        match pc {
            0x82FAD008 => {
    //   block [0x82FAD008..0x82FAD020)
	// 82FAD008: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAD00C: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82FAD010: 394BFFA0  addi r10, r11, -0x60
	ctx.r[10].s64 = ctx.r[11].s64 + -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD020 size=64
    let mut pc: u32 = 0x82FAD020;
    'dispatch: loop {
        match pc {
            0x82FAD020 => {
    //   block [0x82FAD020..0x82FAD060)
	// 82FAD020: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82FAD024: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD060 size=12
    let mut pc: u32 = 0x82FAD060;
    'dispatch: loop {
        match pc {
            0x82FAD060 => {
    //   block [0x82FAD060..0x82FAD06C)
	// 82FAD060: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82FAD064: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAD068: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FAD06C size=88
    let mut pc: u32 = 0x82FAD06C;
    'dispatch: loop {
        match pc {
            0x82FAD06C => {
    //   block [0x82FAD06C..0x82FAD0C4)
	// 82FAD06C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FAD070: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82FAD074: C00908A8  lfs f0, 0x8a8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAD0C8 size=724
    let mut pc: u32 = 0x82FAD0C8;
    'dispatch: loop {
        match pc {
            0x82FAD0C8 => {
    //   block [0x82FAD0C8..0x82FAD39C)
	// 82FAD0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAD0CC: 481FB081  bl 0x831a814c
	ctx.lr = 0x82FAD0D0;
	sub_831A8130(ctx, base);
	// 82FAD0D0: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAD0D4: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82FAD0D8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FAD0DC: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82FAD0E0: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82FAD0E4: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAD0E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAD0EC: 419A0248  beq cr6, 0x82fad334
	if ctx.cr[6].eq {
	pc = 0x82FAD334; continue 'dispatch;
	}
	// 82FAD0F0: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAD0F4: 81380010  lwz r9, 0x10(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAD0F8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAD0FC: 409A0238  bne cr6, 0x82fad334
	if !ctx.cr[6].eq {
	pc = 0x82FAD334; continue 'dispatch;
	}
	// 82FAD100: 81780020  lwz r11, 0x20(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAD104: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAD108: 419A006C  beq cr6, 0x82fad174
	if ctx.cr[6].eq {
	pc = 0x82FAD174; continue 'dispatch;
	}
	// 82FAD10C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82FAD110: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82FAD114: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FAD118: 4BEF9279  bl 0x82ea6390
	ctx.lr = 0x82FAD11C;
	sub_82EA6390(ctx, base);
	// 82FAD11C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAD120: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FAD124: 388B5DEC  addi r4, r11, 0x5dec
	ctx.r[4].s64 = ctx.r[11].s64 + 24044;
	// 82FAD128: 4BEF86B1  bl 0x82ea57d8
	ctx.lr = 0x82FAD12C;
	sub_82EA57D8(ctx, base);
	// 82FAD12C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAD130: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAD134: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82FAD138: 39000175  li r8, 0x175
	ctx.r[8].s64 = 373;
	// 82FAD13C: 38E95DD0  addi r7, r9, 0x5dd0
	ctx.r[7].s64 = ctx.r[9].s64 + 24016;
	// 82FAD140: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82FAD144: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82FAD148: 60A53437  ori r5, r5, 0x3437
	ctx.r[5].u64 = ctx.r[5].u64 | 13367;
	// 82FAD14C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FAD150: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD154: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD158: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAD15C: 4E800421  bctrl
	ctx.lr = 0x82FAD160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAD160: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FAD164: 4BEF8CB5  bl 0x82ea5e18
	ctx.lr = 0x82FAD168;
	sub_82EA5E18(ctx, base);
	// 82FAD168: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FAD16C: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82FAD170: 481FB02C  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82FAD174: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAD178: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FAD17C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAD180: 40990028  ble cr6, 0x82fad1a8
	if !ctx.cr[6].gt {
	pc = 0x82FAD1A8; continue 'dispatch;
	}
	// 82FAD184: 8159000C  lwz r10, 0xc(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD188: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD18C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD190: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FAD194: 419A00C0  beq cr6, 0x82fad254
	if ctx.cr[6].eq {
	pc = 0x82FAD254; continue 'dispatch;
	}
	// 82FAD198: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FAD19C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAD1A0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAD1A4: 4198FFE4  blt cr6, 0x82fad188
	if ctx.cr[6].lt {
	pc = 0x82FAD188; continue 'dispatch;
	}
	// 82FAD1A8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FAD1AC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAD1B0: 40990098  ble cr6, 0x82fad248
	if !ctx.cr[6].gt {
	pc = 0x82FAD248; continue 'dispatch;
	}
	// 82FAD1B4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FAD1B8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FAD1BC: 8158001C  lwz r10, 0x1c(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAD1C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FAD1C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FAD1C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAD1CC: 7D2AD02E  lwzx r9, r10, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FAD1D0: 83890000  lwz r28, 0(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD1D4: 40990050  ble cr6, 0x82fad224
	if !ctx.cr[6].gt {
	pc = 0x82FAD224; continue 'dispatch;
	}
	// 82FAD1D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FAD1DC: 8179000C  lwz r11, 0xc(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD1E0: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAD1E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FAD1E8: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAD1EC: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD1F0: 7EA903A6  mtctr r21
	ctx.ctr.u64 = ctx.r[21].u64;
	// 82FAD1F4: 4E800421  bctrl
	ctx.lr = 0x82FAD1F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAD1F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FAD1FC: 409A0010  bne cr6, 0x82fad20c
	if !ctx.cr[6].eq {
	pc = 0x82FAD20C; continue 'dispatch;
	}
	// 82FAD200: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAD204: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82FAD208: 7FDB5B2E  sthx r30, r27, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u16) };
	// 82FAD20C: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82FAD210: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAD214: 7D5E0734  extsh r30, r10
	ctx.r[30].s64 = ctx.r[10].s16 as i64;
	// 82FAD218: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FAD21C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAD220: 4198FFBC  blt cr6, 0x82fad1dc
	if ctx.cr[6].lt {
	pc = 0x82FAD1DC; continue 'dispatch;
	}
	// 82FAD224: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82FAD228: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FAD22C: 419A0098  beq cr6, 0x82fad2c4
	if ctx.cr[6].eq {
	pc = 0x82FAD2C4; continue 'dispatch;
	}
	// 82FAD230: 81580010  lwz r10, 0x10(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAD234: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82FAD238: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82FAD23C: 3B7B0002  addi r27, r27, 2
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	// 82FAD240: 7F175000  cmpw cr6, r23, r10
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FAD244: 4198FF78  blt cr6, 0x82fad1bc
	if ctx.cr[6].lt {
	pc = 0x82FAD1BC; continue 'dispatch;
	}
	// 82FAD248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FAD24C: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82FAD250: 481FAF4C  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82FAD254: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82FAD258: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82FAD25C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82FAD260: 4BEF9131  bl 0x82ea6390
	ctx.lr = 0x82FAD264;
	sub_82EA6390(ctx, base);
	// 82FAD264: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAD268: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82FAD26C: 388B5DB8  addi r4, r11, 0x5db8
	ctx.r[4].s64 = ctx.r[11].s64 + 23992;
	// 82FAD270: 4BEF8569  bl 0x82ea57d8
	ctx.lr = 0x82FAD274;
	sub_82EA57D8(ctx, base);
	// 82FAD274: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FAD278: 4BEF86B1  bl 0x82ea5928
	ctx.lr = 0x82FAD27C;
	sub_82EA5928(ctx, base);
	// 82FAD27C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAD280: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAD284: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82FAD288: 3900017F  li r8, 0x17f
	ctx.r[8].s64 = 383;
	// 82FAD28C: 38E95DD0  addi r7, r9, 0x5dd0
	ctx.r[7].s64 = ctx.r[9].s64 + 24016;
	// 82FAD290: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82FAD294: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82FAD298: 60A53437  ori r5, r5, 0x3437
	ctx.r[5].u64 = ctx.r[5].u64 | 13367;
	// 82FAD29C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FAD2A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD2A4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD2A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAD2AC: 4E800421  bctrl
	ctx.lr = 0x82FAD2B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAD2B0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82FAD2B4: 4BEF8B65  bl 0x82ea5e18
	ctx.lr = 0x82FAD2B8;
	sub_82EA5E18(ctx, base);
	// 82FAD2B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FAD2BC: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82FAD2C0: 481FAEDC  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82FAD2C4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82FAD2C8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82FAD2CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAD2D0: 4BEF90C1  bl 0x82ea6390
	ctx.lr = 0x82FAD2D4;
	sub_82EA6390(ctx, base);
	// 82FAD2D4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAD2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAD2DC: 388B5DA4  addi r4, r11, 0x5da4
	ctx.r[4].s64 = ctx.r[11].s64 + 23972;
	// 82FAD2E0: 4BEF84F9  bl 0x82ea57d8
	ctx.lr = 0x82FAD2E4;
	sub_82EA57D8(ctx, base);
	// 82FAD2E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FAD2E8: 4BEF84F1  bl 0x82ea57d8
	ctx.lr = 0x82FAD2EC;
	sub_82EA57D8(ctx, base);
	// 82FAD2EC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAD2F0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAD2F4: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82FAD2F8: 39000195  li r8, 0x195
	ctx.r[8].s64 = 405;
	// 82FAD2FC: 38E95DD0  addi r7, r9, 0x5dd0
	ctx.r[7].s64 = ctx.r[9].s64 + 24016;
	// 82FAD300: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82FAD304: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82FAD308: 60A53438  ori r5, r5, 0x3438
	ctx.r[5].u64 = ctx.r[5].u64 | 13368;
	// 82FAD30C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FAD310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD314: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD318: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAD31C: 4E800421  bctrl
	ctx.lr = 0x82FAD320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAD320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FAD324: 4BEF8AF5  bl 0x82ea5e18
	ctx.lr = 0x82FAD328;
	sub_82EA5E18(ctx, base);
	// 82FAD328: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FAD32C: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82FAD330: 481FAE6C  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82FAD334: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82FAD338: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82FAD33C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FAD340: 4BEF9051  bl 0x82ea6390
	ctx.lr = 0x82FAD344;
	sub_82EA6390(ctx, base);
	// 82FAD344: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAD348: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FAD34C: 388B5D88  addi r4, r11, 0x5d88
	ctx.r[4].s64 = ctx.r[11].s64 + 23944;
	// 82FAD350: 4BEF8489  bl 0x82ea57d8
	ctx.lr = 0x82FAD354;
	sub_82EA57D8(ctx, base);
	// 82FAD354: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAD358: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAD35C: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82FAD360: 3900016E  li r8, 0x16e
	ctx.r[8].s64 = 366;
	// 82FAD364: 38E95DD0  addi r7, r9, 0x5dd0
	ctx.r[7].s64 = ctx.r[9].s64 + 24016;
	// 82FAD368: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82FAD36C: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82FAD370: 60A53435  ori r5, r5, 0x3435
	ctx.r[5].u64 = ctx.r[5].u64 | 13365;
	// 82FAD374: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FAD378: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD37C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD380: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FAD384: 4E800421  bctrl
	ctx.lr = 0x82FAD388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FAD388: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FAD38C: 4BEF8A8D  bl 0x82ea5e18
	ctx.lr = 0x82FAD390;
	sub_82EA5E18(ctx, base);
	// 82FAD390: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FAD394: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82FAD398: 481FAE04  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD3A0 size=296
    let mut pc: u32 = 0x82FAD3A0;
    'dispatch: loop {
        match pc {
            0x82FAD3A0 => {
    //   block [0x82FAD3A0..0x82FAD4C8)
	// 82FAD3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAD3A4: 481FADC5  bl 0x831a8168
	ctx.lr = 0x82FAD3A8;
	sub_831A8130(ctx, base);
	// 82FAD3A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FAD3AC: 40990118  ble cr6, 0x82fad4c4
	if !ctx.cr[6].gt {
	pc = 0x82FAD4C4; continue 'dispatch;
	}
	// 82FAD3B0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82FAD3B4: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82FAD3B8: 7FC63850  subf r30, r6, r7
	ctx.r[30].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	// 82FAD3BC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82FAD3C0: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82FAD3C4: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 82FAD3C8: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82FAD3CC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82FAD3D0: 3BEA0010  addi r31, r10, 0x10
	ctx.r[31].s64 = ctx.r[10].s64 + 16;
	// 82FAD3D4: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD3D8: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82FAD3DC: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82FAD3E0: 419A0018  beq cr6, 0x82fad3f8
	if ctx.cr[6].eq {
	pc = 0x82FAD3F8; continue 'dispatch;
	}
	// 82FAD3E4: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAD3E8: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82FAD3EC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAD3F0: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82FAD3F4: 48000008  b 0x82fad3fc
	pc = 0x82FAD3FC; continue 'dispatch;
	// 82FAD3F8: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82FAD3FC: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD4C8 size=24
    let mut pc: u32 = 0x82FAD4C8;
    'dispatch: loop {
        match pc {
            0x82FAD4C8 => {
    //   block [0x82FAD4C8..0x82FAD4E0)
	// 82FAD4C8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82FAD4CC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAD4D0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82FAD4D4: 38AA7080  addi r5, r10, 0x7080
	ctx.r[5].s64 = ctx.r[10].s64 + 28800;
	// 82FAD4D8: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82FAD4DC: 4BFFFEC4  b 0x82fad3a0
	sub_82FAD3A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD4E0 size=412
    let mut pc: u32 = 0x82FAD4E0;
    'dispatch: loop {
        match pc {
            0x82FAD4E0 => {
    //   block [0x82FAD4E0..0x82FAD67C)
	// 82FAD4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAD4E4: 481FAC79  bl 0x831a815c
	ctx.lr = 0x82FAD4E8;
	sub_831A8130(ctx, base);
	// 82FAD4E8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82FAD4EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FAD4F0: 40990188  ble cr6, 0x82fad678
	if !ctx.cr[6].gt {
	pc = 0x82FAD678; continue 'dispatch;
	}
	// 82FAD4F4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82FAD4F8: 10C0038C  vspltisw v6, 0
	for i in 0..4 {
		ctx.v[6].u32[i] = 0;
	}
	// 82FAD4FC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82FAD500: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 82FAD504: 3FC08212  lis r30, -0x7dee
	ctx.r[30].s64 = -2112749568;
	// 82FAD508: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82FAD50C: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82FAD510: 7F264850  subf r25, r6, r9
	ctx.r[25].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 82FAD514: 3B40FFF0  li r26, -0x10
	ctx.r[26].s64 = -16;
	// 82FAD518: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82FAD51C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82FAD520: 3BAAFFA0  addi r29, r10, -0x60
	ctx.r[29].s64 = ctx.r[10].s64 + -96;
	// 82FAD524: 3B88C5A0  addi r28, r8, -0x3a60
	ctx.r[28].s64 = ctx.r[8].s64 + -14944;
	// 82FAD528: 3BFFC5B0  addi r31, r31, -0x3a50
	ctx.r[31].s64 = ctx.r[31].s64 + -14928;
	// 82FAD52C: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82FAD530: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAD534: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82FAD538: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82FAD53C: 419A0018  beq cr6, 0x82fad554
	if ctx.cr[6].eq {
	pc = 0x82FAD554; continue 'dispatch;
	}
	// 82FAD540: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAD544: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82FAD548: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAD54C: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82FAD550: 48000008  b 0x82fad558
	pc = 0x82FAD558; continue 'dispatch;
	// 82FAD554: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82FAD558: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD680 size=24
    let mut pc: u32 = 0x82FAD680;
    'dispatch: loop {
        match pc {
            0x82FAD680 => {
    //   block [0x82FAD680..0x82FAD698)
	// 82FAD680: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82FAD684: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FAD688: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82FAD68C: 38AA7080  addi r5, r10, 0x7080
	ctx.r[5].s64 = ctx.r[10].s64 + 28800;
	// 82FAD690: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82FAD694: 4BFFFE4C  b 0x82fad4e0
	sub_82FAD4E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD698 size=264
    let mut pc: u32 = 0x82FAD698;
    'dispatch: loop {
        match pc {
            0x82FAD698 => {
    //   block [0x82FAD698..0x82FAD7A0)
	// 82FAD698: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82FAD69C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FAD6A0: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FAD6A4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82FAD6A8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAD6AC: 409900E8  ble cr6, 0x82fad794
	if !ctx.cr[6].gt {
	pc = 0x82FAD794; continue 'dispatch;
	}
	// 82FAD6B0: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82FAD6B4: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82FAD6B8: 7CC55050  subf r6, r5, r10
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82FAD6BC: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82FAD6C0: 3BE40020  addi r31, r4, 0x20
	ctx.r[31].s64 = ctx.r[4].s64 + 32;
	// 82FAD6C4: 3BC0FFF0  li r30, -0x10
	ctx.r[30].s64 = -16;
	// 82FAD6C8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FAD6CC: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FAD6D0: 38E70010  addi r7, r7, 0x10
	ctx.r[7].s64 = ctx.r[7].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD7A0 size=372
    let mut pc: u32 = 0x82FAD7A0;
    'dispatch: loop {
        match pc {
            0x82FAD7A0 => {
    //   block [0x82FAD7A0..0x82FAD914)
	// 82FAD7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAD7A4: 481FA9C1  bl 0x831a8164
	ctx.lr = 0x82FAD7A8;
	sub_831A8130(ctx, base);
	// 82FAD7A8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FAD7AC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82FAD7B0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAD7B4: 4099015C  ble cr6, 0x82fad910
	if !ctx.cr[6].gt {
	pc = 0x82FAD910; continue 'dispatch;
	}
	// 82FAD7B8: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82FAD7BC: 10C0038C  vspltisw v6, 0
	for i in 0..4 {
		ctx.v[6].u32[i] = 0;
	}
	// 82FAD7C0: 7F655050  subf r27, r5, r10
	ctx.r[27].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82FAD7C4: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82FAD7C8: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82FAD7CC: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82FAD7D0: 3C608212  lis r3, -0x7dee
	ctx.r[3].s64 = -2112749568;
	// 82FAD7D4: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82FAD7D8: 3B840020  addi r28, r4, 0x20
	ctx.r[28].s64 = ctx.r[4].s64 + 32;
	// 82FAD7DC: 3BA0FFF0  li r29, -0x10
	ctx.r[29].s64 = -16;
	// 82FAD7E0: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82FAD7E4: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
	// 82FAD7E8: 38E7FFA0  addi r7, r7, -0x60
	ctx.r[7].s64 = ctx.r[7].s64 + -96;
	// 82FAD7EC: 38C6C5A0  addi r6, r6, -0x3a60
	ctx.r[6].s64 = ctx.r[6].s64 + -14944;
	// 82FAD7F0: 38A5C5B0  addi r5, r5, -0x3a50
	ctx.r[5].s64 = ctx.r[5].s64 + -14928;
	// 82FAD7F4: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAD918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAD918 size=244
    let mut pc: u32 = 0x82FAD918;
    'dispatch: loop {
        match pc {
            0x82FAD918 => {
    //   block [0x82FAD918..0x82FADA0C)
	// 82FAD918: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FAD91C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAD920: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAD924: 409900E0  ble cr6, 0x82fada04
	if !ctx.cr[6].gt {
	pc = 0x82FADA04; continue 'dispatch;
	}
	// 82FAD928: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82FAD92C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAD930: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FAD934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FAD938: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FAD93C: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82FAD940: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAD944: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAD948: 7D0B302E  lwzx r8, r11, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FAD94C: 7D47522E  lhzx r10, r7, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAD950: 89680004  lbz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAD954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAD958: 419A0098  beq cr6, 0x82fad9f0
	if ctx.cr[6].eq {
	pc = 0x82FAD9F0; continue 'dispatch;
	}
	// 82FAD95C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAD960: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82FAD964: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FAD968: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82FAD96C: 419A007C  beq cr6, 0x82fad9e8
	if ctx.cr[6].eq {
	pc = 0x82FAD9E8; continue 'dispatch;
	}
	// 82FAD970: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FADA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FADA10 size=236
    let mut pc: u32 = 0x82FADA10;
    'dispatch: loop {
        match pc {
            0x82FADA10 => {
    //   block [0x82FADA10..0x82FADAFC)
	// 82FADA10: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82FADA14: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FADA18: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADA1C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FADA20: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FADA24: 409900CC  ble cr6, 0x82fadaf0
	if !ctx.cr[6].gt {
	pc = 0x82FADAF0; continue 'dispatch;
	}
	// 82FADA28: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82FADA2C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82FADA30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FADA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FADA38: 7D0B2850  subf r8, r11, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82FADA3C: 3BEA0010  addi r31, r10, 0x10
	ctx.r[31].s64 = ctx.r[10].s64 + 16;
	// 82FADA40: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FADA44: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADA48: 7FCA302E  lwzx r30, r10, r6
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FADA4C: 7D474A2E  lhzx r10, r7, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FADA50: 893E0004  lbz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADA54: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FADA58: 419A0084  beq cr6, 0x82fadadc
	if ctx.cr[6].eq {
	pc = 0x82FADADC; continue 'dispatch;
	}
	// 82FADA5C: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82FADA60: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82FADA64: 419A0070  beq cr6, 0x82fadad4
	if ctx.cr[6].eq {
	pc = 0x82FADAD4; continue 'dispatch;
	}
	// 82FADA68: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FADB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FADB00 size=164
    let mut pc: u32 = 0x82FADB00;
    'dispatch: loop {
        match pc {
            0x82FADB00 => {
    //   block [0x82FADB00..0x82FADBA4)
	// 82FADB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FADB04: 481FA65D  bl 0x831a8160
	ctx.lr = 0x82FADB08;
	sub_831A8130(ctx, base);
	// 82FADB08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FADB0C: 7CDF0734  extsh r31, r6
	ctx.r[31].s64 = ctx.r[6].s16 as i64;
	// 82FADB10: B0A100B6  sth r5, 0xb6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(182 as u32), ctx.r[5].u16 ) };
	// 82FADB14: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FADB18: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FADB1C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FADB20: B3C10050  sth r30, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u16 ) };
	// 82FADB24: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82FADB28: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FADB2C: 40990040  ble cr6, 0x82fadb6c
	if !ctx.cr[6].gt {
	pc = 0x82FADB6C; continue 'dispatch;
	}
	// 82FADB30: 7CBB0734  extsh r27, r5
	ctx.r[27].s64 = ctx.r[5].s16 as i64;
	// 82FADB34: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FADB38: 419A0034  beq cr6, 0x82fadb6c
	if ctx.cr[6].eq {
	pc = 0x82FADB6C; continue 'dispatch;
	}
	// 82FADB3C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82FADB40: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FADB44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FADB48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FADB4C: 480005BD  bl 0x82fae108
	ctx.lr = 0x82FADB50;
	sub_82FAE108(ctx, base);
	// 82FADB50: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADB54: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FADB58: 7FCB522E  lhzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FADB5C: 7FDF0734  extsh r31, r30
	ctx.r[31].s64 = ctx.r[30].s16 as i64;
	// 82FADB60: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FADB64: B3C10050  sth r30, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u16 ) };
	// 82FADB68: 4199FFCC  bgt cr6, 0x82fadb34
	if ctx.cr[6].gt {
	pc = 0x82FADB34; continue 'dispatch;
	}
	// 82FADB6C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82FADB70: 38A100B6  addi r5, r1, 0xb6
	ctx.r[5].s64 = ctx.r[1].s64 + 182;
	// 82FADB74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FADB78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FADB7C: 4800058D  bl 0x82fae108
	ctx.lr = 0x82FADB80;
	sub_82FAE108(ctx, base);
	// 82FADB80: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 82FADB84: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FADB88: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FADB8C: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FADB90: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82FADB94: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 82FADB98: 98FA0000  stb r7, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82FADB9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FADBA0: 481FA610  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FADBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FADBA8 size=484
    let mut pc: u32 = 0x82FADBA8;
    'dispatch: loop {
        match pc {
            0x82FADBA8 => {
    //   block [0x82FADBA8..0x82FADD8C)
	// 82FADBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FADBAC: 481FA5AD  bl 0x831a8158
	ctx.lr = 0x82FADBB0;
	sub_831A8130(ctx, base);
	// 82FADBB0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FADBB4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADBB8: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 82FADBBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FADBC0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FADBC4: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 82FADBC8: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82FADBCC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FADBD0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82FADBD4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FADBD8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82FADBDC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82FADBE0: 83DD0010  lwz r30, 0x10(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADBE4: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FADBE8: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 82FADBEC: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FADBF0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FADBF4: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FADBF8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FADBFC: 4199000C  bgt cr6, 0x82fadc08
	if ctx.cr[6].gt {
	pc = 0x82FADC08; continue 'dispatch;
	}
	// 82FADC00: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FADC04: 48000018  b 0x82fadc1c
	pc = 0x82FADC1C; continue 'dispatch;
	// 82FADC08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADC0C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FADC10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FADC14: 4E800421  bctrl
	ctx.lr = 0x82FADC18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FADC18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FADC1C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82FADC20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FADC24: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FADC28: 7FCB5378  or r11, r30, r10
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[10].u64;
	// 82FADC2C: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADC30: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FADC34: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FADC38: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FADC3C: 40990054  ble cr6, 0x82fadc90
	if !ctx.cr[6].gt {
	pc = 0x82FADC90; continue 'dispatch;
	}
	// 82FADC40: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FADC44: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82FADC48: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FADC4C: 40980024  bge cr6, 0x82fadc70
	if !ctx.cr[6].lt {
	pc = 0x82FADC70; continue 'dispatch;
	}
	// 82FADC50: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FADC54: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FADC58: 41980008  blt cr6, 0x82fadc60
	if ctx.cr[6].lt {
	pc = 0x82FADC60; continue 'dispatch;
	}
	// 82FADC5C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82FADC60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FADC64: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FADC68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FADC6C: 4BEF8B8D  bl 0x82ea67f8
	ctx.lr = 0x82FADC70;
	sub_82EA67F8(ctx, base);
	// 82FADC70: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FADC74: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FADC78: 40980018  bge cr6, 0x82fadc90
	if !ctx.cr[6].lt {
	pc = 0x82FADC90; continue 'dispatch;
	}
	// 82FADC7C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADC80: 7F6B51AE  stbx r27, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u8) };
	// 82FADC84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FADC88: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FADC8C: 4198FFF0  blt cr6, 0x82fadc7c
	if ctx.cr[6].lt {
	pc = 0x82FADC7C; continue 'dispatch;
	}
	// 82FADC90: 7F4B0734  extsh r11, r26
	ctx.r[11].s64 = ctx.r[26].s16 as i64;
	// 82FADC94: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADC98: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FADC9C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82FADCA0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FADCA4: 7D3E0734  extsh r30, r9
	ctx.r[30].s64 = ctx.r[9].s16 as i64;
	// 82FADCA8: 7F6B51AE  stbx r27, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u8) };
	// 82FADCAC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FADCB0: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADCB4: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FADCB8: 40980080  bge cr6, 0x82fadd38
	if !ctx.cr[6].lt {
	pc = 0x82FADD38; continue 'dispatch;
	}
	// 82FADCBC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADCC0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADCC4: 57E9083C  slwi r9, r31, 1
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FADCC8: 7D095A2E  lhzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FADCCC: 7D070734  extsh r7, r8
	ctx.r[7].s64 = ctx.r[8].s16 as i64;
	// 82FADCD0: 7CC750AE  lbzx r6, r7, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FADCD4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FADCD8: 419A0048  beq cr6, 0x82fadd20
	if ctx.cr[6].eq {
	pc = 0x82FADD20; continue 'dispatch;
	}
	// 82FADCDC: 7F7F51AE  stbx r27, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u8) };
	// 82FADCE0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADCE4: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FADCE8: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82FADCEC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FADCF0: 409A0010  bne cr6, 0x82fadd00
	if !ctx.cr[6].eq {
	pc = 0x82FADD00; continue 'dispatch;
	}
	// 82FADCF4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FADCF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FADCFC: 4BEF8B85  bl 0x82ea6880
	ctx.lr = 0x82FADD00;
	sub_82EA6880(ctx, base);
	// 82FADD00: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADD04: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADD08: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FADD0C: 7FC9532E  sthx r30, r9, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u16) };
	// 82FADD10: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADD14: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82FADD18: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADD1C: 911C0004  stw r8, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FADD20: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FADD24: 813D0010  lwz r9, 0x10(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADD28: 7D7E0734  extsh r30, r11
	ctx.r[30].s64 = ctx.r[11].s16 as i64;
	// 82FADD2C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FADD30: 7F1F4800  cmpw cr6, r31, r9
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FADD34: 4198FF8C  blt cr6, 0x82fadcc0
	if ctx.cr[6].lt {
	pc = 0x82FADCC0; continue 'dispatch;
	}
	// 82FADD38: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82FADD3C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FADD40: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FADD44: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82FADD48: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FADD4C: 409A0014  bne cr6, 0x82fadd60
	if !ctx.cr[6].eq {
	pc = 0x82FADD60; continue 'dispatch;
	}
	// 82FADD50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADD54: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FADD58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FADD5C: 4E800421  bctrl
	ctx.lr = 0x82FADD60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FADD60: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FADD64: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FADD68: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FADD6C: 409A0018  bne cr6, 0x82fadd84
	if !ctx.cr[6].eq {
	pc = 0x82FADD84; continue 'dispatch;
	}
	// 82FADD70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FADD74: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82FADD78: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FADD7C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADD80: 4BEF2A31  bl 0x82ea07b0
	ctx.lr = 0x82FADD84;
	sub_82EA07B0(ctx, base);
	// 82FADD84: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FADD88: 481FA420  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FADD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FADD90 size=508
    let mut pc: u32 = 0x82FADD90;
    'dispatch: loop {
        match pc {
            0x82FADD90 => {
    //   block [0x82FADD90..0x82FADF8C)
	// 82FADD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FADD94: 481FA3C9  bl 0x831a815c
	ctx.lr = 0x82FADD98;
	sub_831A8130(ctx, base);
	// 82FADD98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FADD9C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADDA0: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82FADDA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FADDA8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FADDAC: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 82FADDB0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FADDB4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FADDB8: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82FADDBC: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82FADDC0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82FADDC4: 83DD0010  lwz r30, 0x10(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADDC8: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FADDCC: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 82FADDD0: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FADDD4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FADDD8: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FADDDC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FADDE0: 4199000C  bgt cr6, 0x82faddec
	if ctx.cr[6].gt {
	pc = 0x82FADDEC; continue 'dispatch;
	}
	// 82FADDE4: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FADDE8: 48000018  b 0x82fade00
	pc = 0x82FADE00; continue 'dispatch;
	// 82FADDEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADDF0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FADDF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FADDF8: 4E800421  bctrl
	ctx.lr = 0x82FADDFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FADDFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FADE00: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82FADE04: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADE08: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FADE0C: 7FCA5378  or r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 | ctx.r[10].u64;
	// 82FADE10: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FADE14: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FADE18: 7F1F4800  cmpw cr6, r31, r9
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FADE1C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82FADE20: 40990054  ble cr6, 0x82fade74
	if !ctx.cr[6].gt {
	pc = 0x82FADE74; continue 'dispatch;
	}
	// 82FADE24: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82FADE28: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82FADE2C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FADE30: 40980024  bge cr6, 0x82fade54
	if !ctx.cr[6].lt {
	pc = 0x82FADE54; continue 'dispatch;
	}
	// 82FADE34: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FADE38: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FADE3C: 41980008  blt cr6, 0x82fade44
	if ctx.cr[6].lt {
	pc = 0x82FADE44; continue 'dispatch;
	}
	// 82FADE40: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82FADE44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FADE48: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FADE4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FADE50: 4BEF89A9  bl 0x82ea67f8
	ctx.lr = 0x82FADE54;
	sub_82EA67F8(ctx, base);
	// 82FADE54: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FADE58: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FADE5C: 40980018  bge cr6, 0x82fade74
	if !ctx.cr[6].lt {
	pc = 0x82FADE74; continue 'dispatch;
	}
	// 82FADE60: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADE64: 7F8B51AE  stbx r28, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u8) };
	// 82FADE68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FADE6C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FADE70: 4198FFF0  blt cr6, 0x82fade60
	if ctx.cr[6].lt {
	pc = 0x82FADE60; continue 'dispatch;
	}
	// 82FADE74: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FADE78: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82FADE7C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82FADE80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FADE84: 4099003C  ble cr6, 0x82fadec0
	if !ctx.cr[6].gt {
	pc = 0x82FADEC0; continue 'dispatch;
	}
	// 82FADE88: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82FADE8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FADE90: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADE94: 7CEB522E  lhzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FADE98: 7CEB0734  extsh r11, r7
	ctx.r[11].s64 = ctx.r[7].s16 as i64;
	// 82FADE9C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FADEA0: 419A000C  beq cr6, 0x82fadeac
	if ctx.cr[6].eq {
	pc = 0x82FADEAC; continue 'dispatch;
	}
	// 82FADEA4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADEA8: 7D0B39AE  stbx r8, r11, r7
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u8) };
	// 82FADEAC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FADEB0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FADEB4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FADEB8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FADEBC: 4198FFD4  blt cr6, 0x82fade90
	if ctx.cr[6].lt {
	pc = 0x82FADE90; continue 'dispatch;
	}
	// 82FADEC0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADEC4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82FADEC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FADECC: 4099006C  ble cr6, 0x82fadf38
	if !ctx.cr[6].gt {
	pc = 0x82FADF38; continue 'dispatch;
	}
	// 82FADED0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FADED4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADED8: 7D5E58AE  lbzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FADEDC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FADEE0: 409A0040  bne cr6, 0x82fadf20
	if !ctx.cr[6].eq {
	pc = 0x82FADF20; continue 'dispatch;
	}
	// 82FADEE4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FADEE8: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADEEC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FADEF0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FADEF4: 409A0010  bne cr6, 0x82fadf04
	if !ctx.cr[6].eq {
	pc = 0x82FADF04; continue 'dispatch;
	}
	// 82FADEF8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FADEFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FADF00: 4BEF8981  bl 0x82ea6880
	ctx.lr = 0x82FADF04;
	sub_82EA6880(ctx, base);
	// 82FADF04: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADF08: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADF0C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FADF10: 7FE9532E  sthx r31, r9, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u16) };
	// 82FADF14: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FADF18: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82FADF1C: 911B0004  stw r8, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FADF20: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FADF24: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FADF28: 7D7F0734  extsh r31, r11
	ctx.r[31].s64 = ctx.r[11].s16 as i64;
	// 82FADF2C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82FADF30: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FADF34: 4198FFA0  blt cr6, 0x82faded4
	if ctx.cr[6].lt {
	pc = 0x82FADED4; continue 'dispatch;
	}
	// 82FADF38: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82FADF3C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FADF40: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FADF44: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82FADF48: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FADF4C: 409A0014  bne cr6, 0x82fadf60
	if !ctx.cr[6].eq {
	pc = 0x82FADF60; continue 'dispatch;
	}
	// 82FADF50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FADF54: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FADF58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FADF5C: 4E800421  bctrl
	ctx.lr = 0x82FADF60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FADF60: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FADF64: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FADF68: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FADF6C: 409A0018  bne cr6, 0x82fadf84
	if !ctx.cr[6].eq {
	pc = 0x82FADF84; continue 'dispatch;
	}
	// 82FADF70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FADF74: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82FADF78: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FADF7C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FADF80: 4BEF2831  bl 0x82ea07b0
	ctx.lr = 0x82FADF84;
	sub_82EA07B0(ctx, base);
	// 82FADF84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FADF88: 481FA224  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FADF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FADF90 size=376
    let mut pc: u32 = 0x82FADF90;
    'dispatch: loop {
        match pc {
            0x82FADF90 => {
    //   block [0x82FADF90..0x82FAE108)
	// 82FADF90: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82FADF94: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FADF98: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAE108 size=212
    let mut pc: u32 = 0x82FAE108;
    'dispatch: loop {
        match pc {
            0x82FAE108 => {
    //   block [0x82FAE108..0x82FAE1DC)
	// 82FAE108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE10C: 481FA055  bl 0x831a8160
	ctx.lr = 0x82FAE110;
	sub_831A8130(ctx, base);
	// 82FAE110: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAE114: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FAE118: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FAE11C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FAE120: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FAE124: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAE128: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAE12C: 7F6BEA14  add r27, r11, r29
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FAE130: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAE134: 7F9F5850  subf r28, r31, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82FAE138: 7F0AD800  cmpw cr6, r10, r27
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FAE13C: 40980024  bge cr6, 0x82fae160
	if !ctx.cr[6].lt {
	pc = 0x82FAE160; continue 'dispatch;
	}
	// 82FAE140: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE144: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAE148: 41980008  blt cr6, 0x82fae150
	if ctx.cr[6].lt {
	pc = 0x82FAE150; continue 'dispatch;
	}
	// 82FAE14C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82FAE150: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FAE154: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAE158: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FAE15C: 4BEF869D  bl 0x82ea67f8
	ctx.lr = 0x82FAE160;
	sub_82EA67F8(ctx, base);
	// 82FAE160: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82FAE164: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE168: 57E7083C  slwi r7, r31, 1
	ctx.r[7].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FAE16C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAE170: 7D0A3A14  add r8, r10, r7
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82FAE174: 357CFFFF  addic. r11, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE178: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82FAE17C: 41800024  blt 0x82fae1a0
	if ctx.cr[0].lt {
	pc = 0x82FAE1A0; continue 'dispatch;
	}
	// 82FAE180: 5566083C  slwi r6, r11, 1
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82FAE184: 7D2A4050  subf r9, r10, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82FAE188: 7D465214  add r10, r6, r10
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 82FAE18C: 7D09522E  lhzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAE190: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE194: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82FAE198: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82FAE19C: 4080FFF0  bge 0x82fae18c
	if !ctx.cr[0].lt {
	pc = 0x82FAE18C; continue 'dispatch;
	}
	// 82FAE1A0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE1A4: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE1A8: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82FAE1AC: 41800024  blt 0x82fae1d0
	if ctx.cr[0].lt {
	pc = 0x82FAE1D0; continue 'dispatch;
	}
	// 82FAE1B0: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAE1B4: 7D2AD050  subf r9, r10, r26
	ctx.r[9].s64 = ctx.r[26].s64 - ctx.r[10].s64;
	// 82FAE1B8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82FAE1BC: 7D09522E  lhzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAE1C0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE1C4: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82FAE1C8: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82FAE1CC: 4080FFF0  bge 0x82fae1bc
	if !ctx.cr[0].lt {
	pc = 0x82FAE1BC; continue 'dispatch;
	}
	// 82FAE1D0: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82FAE1D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FAE1D8: 481F9FD8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE1E0 size=12
    let mut pc: u32 = 0x82FAE1E0;
    'dispatch: loop {
        match pc {
            0x82FAE1E0 => {
    //   block [0x82FAE1E0..0x82FAE1EC)
	// 82FAE1E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAE1E4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FAE1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE1F0 size=60
    let mut pc: u32 = 0x82FAE1F0;
    'dispatch: loop {
        match pc {
            0x82FAE1F0 => {
    //   block [0x82FAE1F0..0x82FAE22C)
	// 82FAE1F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FAE1F8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE1FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE200: 40990020  ble cr6, 0x82fae220
	if !ctx.cr[6].gt {
	pc = 0x82FAE220; continue 'dispatch;
	}
	// 82FAE204: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82FAE208: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82FAE20C: 80E3001C  lwz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE210: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE214: 7D27512E  stwx r9, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FAE218: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE21C: 4082FFF0  bne 0x82fae20c
	if !ctx.cr[0].eq {
	pc = 0x82FAE20C; continue 'dispatch;
	}
	// 82FAE220: 99030029  stb r8, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[8].u8 ) };
	// 82FAE224: 99030028  stb r8, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82FAE228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAE230 size=136
    let mut pc: u32 = 0x82FAE230;
    'dispatch: loop {
        match pc {
            0x82FAE230 => {
    //   block [0x82FAE230..0x82FAE2B8)
	// 82FAE230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAE238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAE23C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAE240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAE244: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FAE248: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE24C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE250: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAE254: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE258: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE25C: 7D5F5A14  add r10, r31, r11
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82FAE260: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FAE264: 4BEFC4E5  bl 0x82eaa748
	ctx.lr = 0x82FAE268;
	sub_82EAA748(ctx, base);
	// 82FAE268: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FAE26C: 40990024  ble cr6, 0x82fae290
	if !ctx.cr[6].gt {
	pc = 0x82FAE290; continue 'dispatch;
	}
	// 82FAE270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAE274: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82FAE278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82FAE27C: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE280: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE284: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FAE288: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE28C: 4082FFF0  bne 0x82fae27c
	if !ctx.cr[0].eq {
	pc = 0x82FAE27C; continue 'dispatch;
	}
	// 82FAE290: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAE294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAE298: 997E0029  stb r11, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82FAE29C: 995E0028  stb r10, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82FAE2A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAE2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAE2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAE2AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAE2B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAE2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAE2B8 size=132
    let mut pc: u32 = 0x82FAE2B8;
    'dispatch: loop {
        match pc {
            0x82FAE2B8 => {
    //   block [0x82FAE2B8..0x82FAE33C)
	// 82FAE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAE2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FAE2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAE2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAE2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAE2D0: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE2D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE2D8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE2DC: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE2E0: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE2E4: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82FAE2E8: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FAE2EC: 4BEFC45D  bl 0x82eaa748
	ctx.lr = 0x82FAE2F0;
	sub_82EAA748(ctx, base);
	// 82FAE2F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FAE2F4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FAE2F8: 40990020  ble cr6, 0x82fae318
	if !ctx.cr[6].gt {
	pc = 0x82FAE318; continue 'dispatch;
	}
	// 82FAE2FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAE300: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FAE304: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE308: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE30C: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FAE310: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE314: 4082FFF0  bne 0x82fae304
	if !ctx.cr[0].eq {
	pc = 0x82FAE304; continue 'dispatch;
	}
	// 82FAE318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAE31C: 997F0029  stb r11, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82FAE320: 993F0028  stb r9, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 82FAE324: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FAE328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAE32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAE330: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FAE334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAE338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE340 size=72
    let mut pc: u32 = 0x82FAE340;
    'dispatch: loop {
        match pc {
            0x82FAE340 => {
    //   block [0x82FAE340..0x82FAE388)
	// 82FAE340: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FAE344: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE348: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE34C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE350: 40990020  ble cr6, 0x82fae370
	if !ctx.cr[6].gt {
	pc = 0x82FAE370; continue 'dispatch;
	}
	// 82FAE354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAE358: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82FAE35C: 80E9001C  lwz r7, 0x1c(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE360: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE364: 7D07512E  stwx r8, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82FAE368: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE36C: 4082FFF0  bne 0x82fae35c
	if !ctx.cr[0].eq {
	pc = 0x82FAE35C; continue 'dispatch;
	}
	// 82FAE370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAE374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FAE378: 99690028  stb r11, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82FAE37C: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 82FAE380: 99490029  stb r10, 0x29(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 82FAE384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE388 size=68
    let mut pc: u32 = 0x82FAE388;
    'dispatch: loop {
        match pc {
            0x82FAE388 => {
    //   block [0x82FAE388..0x82FAE3CC)
	// 82FAE388: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FAE38C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FAE390: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE394: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE39C: 4099001C  ble cr6, 0x82fae3b8
	if !ctx.cr[6].gt {
	pc = 0x82FAE3B8; continue 'dispatch;
	}
	// 82FAE3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAE3A4: 80E9001C  lwz r7, 0x1c(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE3A8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE3AC: 7D07512E  stwx r8, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82FAE3B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE3B4: 4082FFF0  bne 0x82fae3a4
	if !ctx.cr[0].eq {
	pc = 0x82FAE3A4; continue 'dispatch;
	}
	// 82FAE3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAE3BC: 38690010  addi r3, r9, 0x10
	ctx.r[3].s64 = ctx.r[9].s64 + 16;
	// 82FAE3C0: 99690029  stb r11, 0x29(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82FAE3C4: 99090028  stb r8, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82FAE3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE3D0 size=204
    let mut pc: u32 = 0x82FAE3D0;
    'dispatch: loop {
        match pc {
            0x82FAE3D0 => {
    //   block [0x82FAE3D0..0x82FAE49C)
	// 82FAE3D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FAE3D8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82FAE3DC: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE3E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAE3E4: 40990034  ble cr6, 0x82fae418
	if !ctx.cr[6].gt {
	pc = 0x82FAE418; continue 'dispatch;
	}
	// 82FAE3E8: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE3EC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE3F0: 550607FE  clrlwi r6, r8, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82FAE3F4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FAE3F8: 419A0010  beq cr6, 0x82fae408
	if ctx.cr[6].eq {
	pc = 0x82FAE408; continue 'dispatch;
	}
	// 82FAE3FC: 550807BC  rlwinm r8, r8, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAE400: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAE404: 409A0098  bne cr6, 0x82fae49c
	if !ctx.cr[6].eq {
		sub_82FAE49C(ctx, base);
		return;
	}
	// 82FAE408: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FAE40C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FAE410: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAE414: 4198FFD8  blt cr6, 0x82fae3ec
	if ctx.cr[6].lt {
	pc = 0x82FAE3EC; continue 'dispatch;
	}
	// 82FAE418: 89640029  lbz r11, 0x29(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(41 as u32) ) } as u64;
	// 82FAE41C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAE420: 419A0034  beq cr6, 0x82fae454
	if ctx.cr[6].eq {
	pc = 0x82FAE454; continue 'dispatch;
	}
	// 82FAE424: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82FAE428: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAE42C: 40990028  ble cr6, 0x82fae454
	if !ctx.cr[6].gt {
	pc = 0x82FAE454; continue 'dispatch;
	}
	// 82FAE430: 8144001C  lwz r10, 0x1c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE434: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE438: 550607FE  clrlwi r6, r8, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82FAE43C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FAE440: 409A005C  bne cr6, 0x82fae49c
	if !ctx.cr[6].eq {
		sub_82FAE49C(ctx, base);
		return;
	}
	// 82FAE444: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAE448: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE44C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAE450: 4198FFE4  blt cr6, 0x82fae434
	if ctx.cr[6].lt {
	pc = 0x82FAE434; continue 'dispatch;
	}
	// 82FAE454: 89640028  lbz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FAE458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAE45C: 419A0034  beq cr6, 0x82fae490
	if ctx.cr[6].eq {
	pc = 0x82FAE490; continue 'dispatch;
	}
	// 82FAE460: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82FAE464: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAE468: 40990028  ble cr6, 0x82fae490
	if !ctx.cr[6].gt {
	pc = 0x82FAE490; continue 'dispatch;
	}
	// 82FAE46C: 8144001C  lwz r10, 0x1c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE470: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE474: 550607BC  rlwinm r6, r8, 0, 0x1e, 0x1e
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAE478: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FAE47C: 409A0020  bne cr6, 0x82fae49c
	if !ctx.cr[6].eq {
		sub_82FAE49C(ctx, base);
		return;
	}
	// 82FAE480: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAE484: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE488: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FAE48C: 4198FFE4  blt cr6, 0x82fae470
	if ctx.cr[6].lt {
	pc = 0x82FAE470; continue 'dispatch;
	}
	// 82FAE490: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FAE494: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FAE498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE49C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE49C size=8
    let mut pc: u32 = 0x82FAE49C;
    'dispatch: loop {
        match pc {
            0x82FAE49C => {
    //   block [0x82FAE49C..0x82FAE4A4)
	// 82FAE49C: 98E30000  stb r7, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82FAE4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAE4A8 size=200
    let mut pc: u32 = 0x82FAE4A8;
    'dispatch: loop {
        match pc {
            0x82FAE4A8 => {
    //   block [0x82FAE4A8..0x82FAE570)
	// 82FAE4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAE4B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAE4B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAE4B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAE4BC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE4C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAE4C4: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE4C8: 808A0014  lwz r4, 0x14(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAE4CC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAE4D0: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FAE4D4: 55252036  slwi r5, r9, 4
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FAE4D8: 4BEFC271  bl 0x82eaa748
	ctx.lr = 0x82FAE4DC;
	sub_82EAA748(ctx, base);
	// 82FAE4DC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAE4E4: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE4E8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FAE4EC: 4099002C  ble cr6, 0x82fae518
	if !ctx.cr[6].gt {
	pc = 0x82FAE518; continue 'dispatch;
	}
	// 82FAE4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAE4F4: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82FAE4F8: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE4FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAE500: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FAE504: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE508: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE50C: 80C70010  lwz r6, 0x10(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE510: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82FAE514: 4198FFE4  blt cr6, 0x82fae4f8
	if ctx.cr[6].lt {
	pc = 0x82FAE4F8; continue 'dispatch;
	}
	// 82FAE518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAE51C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FAE520: 997F0028  stb r11, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82FAE524: 995F0029  stb r10, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 82FAE528: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE52C: 81090020  lwz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAE530: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAE534: 40990028  ble cr6, 0x82fae55c
	if !ctx.cr[6].gt {
	pc = 0x82FAE55C; continue 'dispatch;
	}
	// 82FAE538: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82FAE53C: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FAE540: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAE544: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FAE548: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FAE54C: 81090020  lwz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAE550: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE554: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FAE558: 4198FFEC  blt cr6, 0x82fae544
	if ctx.cr[6].lt {
	pc = 0x82FAE544; continue 'dispatch;
	}
	// 82FAE55C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAE560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAE564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAE568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAE56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAE570 size=520
    let mut pc: u32 = 0x82FAE570;
    'dispatch: loop {
        match pc {
            0x82FAE570 => {
    //   block [0x82FAE570..0x82FAE778)
	// 82FAE570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE574: 481F9BE5  bl 0x831a8158
	ctx.lr = 0x82FAE578;
	sub_831A8130(ctx, base);
	// 82FAE578: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAE57C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAE580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAE584: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FAE588: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82FAE58C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82FAE590: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FAE594: 3B9F0004  addi r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 4;
	// 82FAE598: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FAE59C: 3B5F0010  addi r26, r31, 0x10
	ctx.r[26].s64 = ctx.r[31].s64 + 16;
	// 82FAE5A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FAE5A4: 3B7F001C  addi r27, r31, 0x1c
	ctx.r[27].s64 = ctx.r[31].s64 + 28;
	// 82FAE5A8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FAE5AC: 3BDF002C  addi r30, r31, 0x2c
	ctx.r[30].s64 = ctx.r[31].s64 + 44;
	// 82FAE5B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FAE5B4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FAE5B8: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FAE5BC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FAE5C0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FAE5C4: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82FAE5C8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FAE5CC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82FAE5D0: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82FAE5D4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE5D8: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FAE5DC: 552B00BE  clrlwi r11, r9, 2
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAE5E0: 83AA0010  lwz r29, 0x10(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE5E4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FAE5E8: 40980024  bge cr6, 0x82fae60c
	if !ctx.cr[6].lt {
	pc = 0x82FAE60C; continue 'dispatch;
	}
	// 82FAE5EC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE5F0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAE5F4: 41980008  blt cr6, 0x82fae5fc
	if ctx.cr[6].lt {
	pc = 0x82FAE5FC; continue 'dispatch;
	}
	// 82FAE5F8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FAE5FC: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82FAE600: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAE604: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FAE608: 4BEF81F1  bl 0x82ea67f8
	ctx.lr = 0x82FAE60C;
	sub_82EA67F8(ctx, base);
	// 82FAE60C: 93BA0004  stw r29, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FAE610: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAE614: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAE618: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FAE61C: 40980024  bge cr6, 0x82fae640
	if !ctx.cr[6].lt {
	pc = 0x82FAE640; continue 'dispatch;
	}
	// 82FAE620: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE624: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAE628: 41980008  blt cr6, 0x82fae630
	if ctx.cr[6].lt {
	pc = 0x82FAE630; continue 'dispatch;
	}
	// 82FAE62C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FAE630: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82FAE634: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAE638: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FAE63C: 4BEF81BD  bl 0x82ea67f8
	ctx.lr = 0x82FAE640;
	sub_82EA67F8(ctx, base);
	// 82FAE640: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FAE644: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAE648: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAE64C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FAE650: 40980024  bge cr6, 0x82fae674
	if !ctx.cr[6].lt {
	pc = 0x82FAE674; continue 'dispatch;
	}
	// 82FAE654: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE658: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAE65C: 41980008  blt cr6, 0x82fae664
	if ctx.cr[6].lt {
	pc = 0x82FAE664; continue 'dispatch;
	}
	// 82FAE660: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FAE664: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FAE668: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAE66C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FAE670: 4BEF8189  bl 0x82ea67f8
	ctx.lr = 0x82FAE674;
	sub_82EA67F8(ctx, base);
	// 82FAE674: 93BB0004  stw r29, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FAE678: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82FAE67C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FAE680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAE684: 409A000C  bne cr6, 0x82fae690
	if !ctx.cr[6].eq {
	pc = 0x82FAE690; continue 'dispatch;
	}
	// 82FAE688: 4BFFFBA9  bl 0x82fae230
	ctx.lr = 0x82FAE68C;
	sub_82FAE230(ctx, base);
	// 82FAE68C: 48000008  b 0x82fae694
	pc = 0x82FAE694; continue 'dispatch;
	// 82FAE690: 4BFFFC29  bl 0x82fae2b8
	ctx.lr = 0x82FAE694;
	sub_82FAE2B8(ctx, base);
	// 82FAE694: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE698: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAE69C: 838B0020  lwz r28, 0x20(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FAE6A0: 7F1CE800  cmpw cr6, r28, r29
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FAE6A4: 409900C4  ble cr6, 0x82fae768
	if !ctx.cr[6].gt {
	pc = 0x82FAE768; continue 'dispatch;
	}
	// 82FAE6A8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FAE6AC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FAE6B0: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FAE6B4: 40980024  bge cr6, 0x82fae6d8
	if !ctx.cr[6].lt {
	pc = 0x82FAE6D8; continue 'dispatch;
	}
	// 82FAE6B8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE6BC: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FAE6C0: 41980008  blt cr6, 0x82fae6c8
	if ctx.cr[6].lt {
	pc = 0x82FAE6C8; continue 'dispatch;
	}
	// 82FAE6C4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FAE6C8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FAE6CC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FAE6D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FAE6D4: 4BEF8125  bl 0x82ea67f8
	ctx.lr = 0x82FAE6D8;
	sub_82EA67F8(ctx, base);
	// 82FAE6D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FAE6DC: 7D5DE050  subf r10, r29, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82FAE6E0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FAE6E4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82FAE6E8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAE6EC: 41980058  blt cr6, 0x82fae744
	if ctx.cr[6].lt {
	pc = 0x82FAE744; continue 'dispatch;
	}
	// 82FAE6F0: 7D5DE050  subf r10, r29, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82FAE6F4: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAE6F8: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82FAE6FC: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAE700: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FAE704: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAE708: 7CE9EA14  add r7, r9, r29
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 82FAE70C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE710: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 82FAE714: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FAE718: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82FAE71C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE720: 7CCB4214  add r6, r11, r8
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82FAE724: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82FAE728: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE72C: 7CA94214  add r5, r9, r8
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FAE730: D005FFFC  stfs f0, -4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82FAE734: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE738: 7C09252E  stfsx f0, r9, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82FAE73C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82FAE740: 4082FFCC  bne 0x82fae70c
	if !ctx.cr[0].eq {
	pc = 0x82FAE70C; continue 'dispatch;
	}
	// 82FAE744: 7F07E000  cmpw cr6, r7, r28
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FAE748: 40980020  bge cr6, 0x82fae768
	if !ctx.cr[6].lt {
	pc = 0x82FAE768; continue 'dispatch;
	}
	// 82FAE74C: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAE750: 7D67E050  subf r11, r7, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[7].s64;
	// 82FAE754: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE758: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE75C: 7C0A4D2E  stfsx f0, r10, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82FAE760: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAE764: 4082FFF0  bne 0x82fae754
	if !ctx.cr[0].eq {
	pc = 0x82FAE754; continue 'dispatch;
	}
	// 82FAE768: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FAE76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAE770: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FAE774: 481F9A34  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE778 size=556
    let mut pc: u32 = 0x82FAE778;
    'dispatch: loop {
        match pc {
            0x82FAE778 => {
    //   block [0x82FAE778..0x82FAE9A4)
	// 82FAE778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE77C: 481F99E5  bl 0x831a8160
	ctx.lr = 0x82FAE780;
	sub_831A8130(ctx, base);
	// 82FAE780: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE784: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAE788: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82FAE78C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82FAE790: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FAE794: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FAE798: 550707BC  rlwinm r7, r8, 0, 0x1e, 0x1e
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAE79C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FAE7A0: 419A0098  beq cr6, 0x82fae838
	if ctx.cr[6].eq {
	pc = 0x82FAE838; continue 'dispatch;
	}
	// 82FAE7A4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE7A8: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FAE7AC: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAE7B0: 7D063A2E  lhzx r8, r6, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FAE7B4: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82FAE7B8: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82FAE7BC: 419A0034  beq cr6, 0x82fae7f0
	if ctx.cr[6].eq {
	pc = 0x82FAE7F0; continue 'dispatch;
	}
	// 82FAE7C0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FAE7C4: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAE7C8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82FAE7CC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE7D0: 61070008  ori r7, r8, 8
	ctx.r[7].u64 = ctx.r[8].u64 | 8;
	// 82FAE7D4: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FAE7D8: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAE7DC: 7CCA482E  lwzx r6, r10, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAE7E0: 54C807BC  rlwinm r8, r6, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAE7E4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAE7E8: 409AFFBC  bne cr6, 0x82fae7a4
	if !ctx.cr[6].eq {
	pc = 0x82FAE7A4; continue 'dispatch;
	}
	// 82FAE7EC: 4800004C  b 0x82fae838
	pc = 0x82FAE838; continue 'dispatch;
	// 82FAE7F0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAE7F4: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAE7F8: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE7FC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FAE800: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAE804: 7D484A14  add r10, r8, r9
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82FAE808: 7D274A14  add r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82FAE80C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAE9A8 size=528
    let mut pc: u32 = 0x82FAE9A8;
    'dispatch: loop {
        match pc {
            0x82FAE9A8 => {
    //   block [0x82FAE9A8..0x82FAEBB8)
	// 82FAE9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAE9AC: 481F97B1  bl 0x831a815c
	ctx.lr = 0x82FAE9B0;
	sub_831A8130(ctx, base);
	// 82FAE9B0: 89630029  lbz r11, 0x29(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(41 as u32) ) } as u64;
	// 82FAE9B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAE9B8: 409A01FC  bne cr6, 0x82faebb4
	if !ctx.cr[6].eq {
	pc = 0x82FAEBB4; continue 'dispatch;
	}
	// 82FAE9BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAE9C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAE9C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAE9C8: 409901E4  ble cr6, 0x82faebac
	if !ctx.cr[6].gt {
	pc = 0x82FAEBAC; continue 'dispatch;
	}
	// 82FAE9CC: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82FAE9D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAE9D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FAE9D8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAE9DC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82FAE9E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FAE9E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FAE9E8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FAE9EC: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82FAE9F0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FAE9F4: 3BEBFFA0  addi r31, r11, -0x60
	ctx.r[31].s64 = ctx.r[11].s64 + -96;
	// 82FAE9F8: 3BCAC5A0  addi r30, r10, -0x3a60
	ctx.r[30].s64 = ctx.r[10].s64 + -14944;
	// 82FAE9FC: 3BA9C5B0  addi r29, r9, -0x3a50
	ctx.r[29].s64 = ctx.r[9].s64 + -14928;
	// 82FAEA00: 3B880010  addi r28, r8, 0x10
	ctx.r[28].s64 = ctx.r[8].s64 + 16;
	// 82FAEA04: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAEA08: 7D5B582E  lwzx r10, r27, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAEA0C: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82FAEA10: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAEA14: 419A0184  beq cr6, 0x82faeb98
	if ctx.cr[6].eq {
	pc = 0x82FAEB98; continue 'dispatch;
	}
	// 82FAEA18: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEA1C: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAEA20: 7D654214  add r11, r5, r8
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82FAEA24: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAEA28: 7CC9D22E  lhzx r6, r9, r26
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FAEA2C: 7CCA0734  extsh r10, r6
	ctx.r[10].s64 = ctx.r[6].s16 as i64;
	// 82FAEA30: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82FAEA34: 419A0134  beq cr6, 0x82faeb68
	if ctx.cr[6].eq {
	pc = 0x82FAEB68; continue 'dispatch;
	}
	// 82FAEA38: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAEBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FAEBB8 size=412
    let mut pc: u32 = 0x82FAEBB8;
    'dispatch: loop {
        match pc {
            0x82FAEBB8 => {
    //   block [0x82FAEBB8..0x82FAED54)
	// 82FAEBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAEBBC: 481F95AD  bl 0x831a8168
	ctx.lr = 0x82FAEBC0;
	sub_831A8130(ctx, base);
	// 82FAEBC0: 89630028  lbz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FAEBC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FAEBC8: 409A0188  bne cr6, 0x82faed50
	if !ctx.cr[6].eq {
	pc = 0x82FAED50; continue 'dispatch;
	}
	// 82FAEBCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEBD0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAEBD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAEBD8: 40990170  ble cr6, 0x82faed48
	if !ctx.cr[6].gt {
	pc = 0x82FAED48; continue 'dispatch;
	}
	// 82FAEBDC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82FAEBE0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAEBE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FAEBE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FAEBEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FAEBF0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FAEBF4: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82FAEBF8: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82FAEBFC: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAEC00: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAEC04: 554907BC  rlwinm r9, r10, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAEC08: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAEC0C: 419A0128  beq cr6, 0x82faed34
	if ctx.cr[6].eq {
	pc = 0x82FAED34; continue 'dispatch;
	}
	// 82FAEC10: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEC14: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAEC18: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82FAEC1C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAEC20: 7D09EA2E  lhzx r8, r9, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FAEC24: 7D0A0734  extsh r10, r8
	ctx.r[10].s64 = ctx.r[8].s16 as i64;
	// 82FAEC28: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82FAEC2C: 419A00D8  beq cr6, 0x82faed04
	if ctx.cr[6].eq {
	pc = 0x82FAED04; continue 'dispatch;
	}
	// 82FAEC30: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAEC34: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAED58 size=108
    let mut pc: u32 = 0x82FAED58;
    'dispatch: loop {
        match pc {
            0x82FAED58 => {
    //   block [0x82FAED58..0x82FAEDC4)
	// 82FAED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAED60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAED64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAED68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAED6C: 4BFFFC3D  bl 0x82fae9a8
	ctx.lr = 0x82FAED70;
	sub_82FAE9A8(ctx, base);
	// 82FAED70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAED74: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAED78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAED7C: 40990020  ble cr6, 0x82faed9c
	if !ctx.cr[6].gt {
	pc = 0x82FAED9C; continue 'dispatch;
	}
	// 82FAED80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAED84: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82FAED88: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAED8C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAED90: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FAED94: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAED98: 4082FFF0  bne 0x82faed88
	if !ctx.cr[0].eq {
	pc = 0x82FAED88; continue 'dispatch;
	}
	// 82FAED9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAEDA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FAEDA4: 997F0028  stb r11, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82FAEDA8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82FAEDAC: 995F0029  stb r10, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 82FAEDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAEDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAEDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAEDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAEDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAEDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAEDC8 size=104
    let mut pc: u32 = 0x82FAEDC8;
    'dispatch: loop {
        match pc {
            0x82FAEDC8 => {
    //   block [0x82FAEDC8..0x82FAEE30)
	// 82FAEDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAEDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAEDD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAEDD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAEDD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAEDDC: 4BFFFDDD  bl 0x82faebb8
	ctx.lr = 0x82FAEDE0;
	sub_82FAEBB8(ctx, base);
	// 82FAEDE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEDE4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FAEDE8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAEDEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAEDF0: 4099001C  ble cr6, 0x82faee0c
	if !ctx.cr[6].gt {
	pc = 0x82FAEE0C; continue 'dispatch;
	}
	// 82FAEDF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FAEDF8: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAEDFC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FAEE00: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FAEE04: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FAEE08: 4082FFF0  bne 0x82faedf8
	if !ctx.cr[0].eq {
	pc = 0x82FAEDF8; continue 'dispatch;
	}
	// 82FAEE0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAEE10: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82FAEE14: 997F0029  stb r11, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82FAEE18: 993F0028  stb r9, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 82FAEE1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAEE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAEE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAEE28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAEE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAEE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAEE30 size=288
    let mut pc: u32 = 0x82FAEE30;
    'dispatch: loop {
        match pc {
            0x82FAEE30 => {
    //   block [0x82FAEE30..0x82FAEF50)
	// 82FAEE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAEE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAEE38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAEE3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAEE40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEE44: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAEE48: 4BFFFB61  bl 0x82fae9a8
	ctx.lr = 0x82FAEE4C;
	sub_82FAE9A8(ctx, base);
	// 82FAEE4C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FAEE50: 409900BC  ble cr6, 0x82faef0c
	if !ctx.cr[6].gt {
	pc = 0x82FAEF0C; continue 'dispatch;
	}
	// 82FAEE54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FAEE58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FAEE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FAEE60: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEE64: 80EA000C  lwz r7, 0xc(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAEE68: 7CC7582E  lwzx r6, r7, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAEE6C: 88A60004  lbz r5, 4(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAEE70: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82FAEE74: 419A0038  beq cr6, 0x82faeeac
	if ctx.cr[6].eq {
	pc = 0x82FAEEAC; continue 'dispatch;
	}
	// 82FAEE78: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAEE7C: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAEF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAEF50 size=420
    let mut pc: u32 = 0x82FAEF50;
    'dispatch: loop {
        match pc {
            0x82FAEF50 => {
    //   block [0x82FAEF50..0x82FAF0F4)
	// 82FAEF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAEF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAEF58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAEF5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAEF60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEF64: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAEF68: 4BFFFC51  bl 0x82faebb8
	ctx.lr = 0x82FAEF6C;
	sub_82FAEBB8(ctx, base);
	// 82FAEF6C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FAEF70: 40990140  ble cr6, 0x82faf0b0
	if !ctx.cr[6].gt {
	pc = 0x82FAF0B0; continue 'dispatch;
	}
	// 82FAEF74: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAEF78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FAEF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FAEF80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FAEF84: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82FAEF88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAEF8C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAEF90: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAEF94: 7CEA302E  lwzx r7, r10, r6
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FAEF98: 7D48222E  lhzx r10, r8, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82FAEF9C: 89070004  lbz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAEFA0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FAEFA4: 419A00C8  beq cr6, 0x82faf06c
	if ctx.cr[6].eq {
	pc = 0x82FAF06C; continue 'dispatch;
	}
	// 82FAEFA8: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAEFAC: 7D4B0734  extsh r11, r10
	ctx.r[11].s64 = ctx.r[10].s16 as i64;
	// 82FAEFB0: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82FAEFB4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FAEFB8: 419A0084  beq cr6, 0x82faf03c
	if ctx.cr[6].eq {
	pc = 0x82FAF03C; continue 'dispatch;
	}
	// 82FAEFBC: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FAEFC0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAF0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAF0F8 size=156
    let mut pc: u32 = 0x82FAF0F8;
    'dispatch: loop {
        match pc {
            0x82FAF0F8 => {
    //   block [0x82FAF0F8..0x82FAF194)
	// 82FAF0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAF0FC: 481F9071  bl 0x831a816c
	ctx.lr = 0x82FAF100;
	sub_831A8130(ctx, base);
	// 82FAF100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAF104: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FAF108: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FAF10C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82FAF110: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82FAF114: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FAF118: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAF198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAF198 size=36
    let mut pc: u32 = 0x82FAF198;
    'dispatch: loop {
        match pc {
            0x82FAF198 => {
    //   block [0x82FAF198..0x82FAF1BC)
	// 82FAF198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAF19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAF1A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAF1A4: 4BFFF805  bl 0x82fae9a8
	ctx.lr = 0x82FAF1A8;
	sub_82FAE9A8(ctx, base);
	// 82FAF1A8: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82FAF1AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAF1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAF1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAF1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAF1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAF1C0 size=36
    let mut pc: u32 = 0x82FAF1C0;
    'dispatch: loop {
        match pc {
            0x82FAF1C0 => {
    //   block [0x82FAF1C0..0x82FAF1E4)
	// 82FAF1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAF1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAF1C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAF1CC: 4BFFF9ED  bl 0x82faebb8
	ctx.lr = 0x82FAF1D0;
	sub_82FAEBB8(ctx, base);
	// 82FAF1D0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82FAF1D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAF1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAF1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAF1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAF1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAF1E8 size=1184
    let mut pc: u32 = 0x82FAF1E8;
    'dispatch: loop {
        match pc {
            0x82FAF1E8 => {
    //   block [0x82FAF1E8..0x82FAF688)
	// 82FAF1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAF1EC: 481F8F49  bl 0x831a8134
	ctx.lr = 0x82FAF1F0;
	sub_831A8130(ctx, base);
	// 82FAF1F0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAF1F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAF1F8: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82FAF1FC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82FAF200: 5630103A  slwi r16, r17, 2
	ctx.r[16].u32 = ctx.r[17].u32.wrapping_shl(2);
	ctx.r[16].u64 = ctx.r[16].u32 as u64;
	// 82FAF204: 3B510001  addi r26, r17, 1
	ctx.r[26].s64 = ctx.r[17].s64 + 1;
	// 82FAF208: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAF20C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAF210: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAF214: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82FAF218: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82FAF21C: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82FAF220: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82FAF224: 7CAB802E  lwzx r5, r11, r16
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82FAF228: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82FAF22C: 82E60010  lwz r23, 0x10(r6)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAF230: 3AAAFFA0  addi r21, r10, -0x60
	ctx.r[21].s64 = ctx.r[10].s64 + -96;
	// 82FAF234: 60A40004  ori r4, r5, 4
	ctx.r[4].u64 = ctx.r[5].u64 | 4;
	// 82FAF238: 7F1AB800  cmpw cr6, r26, r23
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FAF23C: 7C8B812E  stwx r4, r11, r16
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[16].u32), ctx.r[4].u32) };
	// 82FAF240: 3A89C5A0  addi r20, r9, -0x3a60
	ctx.r[20].s64 = ctx.r[9].s64 + -14944;
	// 82FAF244: 3A68C5B0  addi r19, r8, -0x3a50
	ctx.r[19].s64 = ctx.r[8].s64 + -14928;
	// 82FAF248: 3A470010  addi r18, r7, 0x10
	ctx.r[18].s64 = ctx.r[7].s64 + 16;
	// 82FAF24C: 4098024C  bge cr6, 0x82faf498
	if !ctx.cr[6].lt {
	pc = 0x82FAF498; continue 'dispatch;
	}
	// 82FAF250: 574B083C  slwi r11, r26, 1
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAF254: 575B103A  slwi r27, r26, 2
	ctx.r[27].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82FAF258: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82FAF25C: 5758083C  slwi r24, r26, 1
	ctx.r[24].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82FAF260: 557C2036  slwi r28, r11, 4
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FAF264: 7EDAB850  subf r22, r26, r23
	ctx.r[22].s64 = ctx.r[23].s64 - ctx.r[26].s64;
	// 82FAF268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAF26C: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAF270: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAF274: 7D4AC22E  lhzx r10, r10, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82FAF278: 7D480734  extsh r8, r10
	ctx.r[8].s64 = ctx.r[10].s16 as i64;
	// 82FAF27C: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FAF280: 7CC7482E  lwzx r6, r7, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAF284: 54C5077A  rlwinm r5, r6, 0, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAF288: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FAF28C: 419A01BC  beq cr6, 0x82faf448
	if ctx.cr[6].eq {
	pc = 0x82FAF448; continue 'dispatch;
	}
	// 82FAF290: 7D7B482E  lwzx r11, r27, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAF294: 556807FE  clrlwi r8, r11, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82FAF298: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAF29C: 419A0198  beq cr6, 0x82faf434
	if ctx.cr[6].eq {
	pc = 0x82FAF434; continue 'dispatch;
	}
	// 82FAF2A0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAF2A4: 7D440734  extsh r4, r10
	ctx.r[4].s64 = ctx.r[10].s16 as i64;
	// 82FAF2A8: 7FDC5A14  add r30, r28, r11
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82FAF2AC: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82FAF2B0: 419A0154  beq cr6, 0x82faf404
	if ctx.cr[6].eq {
	pc = 0x82FAF404; continue 'dispatch;
	}
	// 82FAF2B4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF2B8: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAF2BC: 552807BC  rlwinm r8, r9, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAF2C0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAF2C4: 409A0018  bne cr6, 0x82faf2dc
	if !ctx.cr[6].eq {
	pc = 0x82FAF2DC; continue 'dispatch;
	}
	// 82FAF2C8: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF2CC: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82FAF2D0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF2D4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FAF2D8: 4800000C  b 0x82faf2e4
	pc = 0x82FAF2E4; continue 'dispatch;
	// 82FAF2DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FAF2E0: 4BFFF499  bl 0x82fae778
	ctx.lr = 0x82FAF2E4;
	sub_82FAE778(ctx, base);
	// 82FAF2E4: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAF688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAF688 size=888
    let mut pc: u32 = 0x82FAF688;
    'dispatch: loop {
        match pc {
            0x82FAF688 => {
    //   block [0x82FAF688..0x82FAFA00)
	// 82FAF688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAF68C: 481F8AA9  bl 0x831a8134
	ctx.lr = 0x82FAF690;
	sub_831A8130(ctx, base);
	// 82FAF690: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAF694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FAF698: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82FAF69C: 39E00001  li r15, 1
	ctx.r[15].s64 = 1;
	// 82FAF6A0: 3A000000  li r16, 0
	ctx.r[16].s64 = 0;
	// 82FAF6A4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FAF6A8: 409A0078  bne cr6, 0x82faf720
	if !ctx.cr[6].eq {
	pc = 0x82FAF720; continue 'dispatch;
	}
	// 82FAF6AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAF6B0: 38910001  addi r4, r17, 1
	ctx.r[4].s64 = ctx.r[17].s64 + 1;
	// 82FAF6B4: 838B0010  lwz r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAF6B8: 7F04E000  cmpw cr6, r4, r28
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FAF6BC: 409802F8  bge cr6, 0x82faf9b4
	if !ctx.cr[6].lt {
	pc = 0x82FAF9B4; continue 'dispatch;
	}
	// 82FAF6C0: 549F103A  slwi r31, r4, 2
	ctx.r[31].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82FAF6C4: 549D083C  slwi r29, r4, 1
	ctx.r[29].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82FAF6C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAF6CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAF6D0: 7D2AEA2E  lhzx r9, r10, r29
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FAF6D4: 7D280734  extsh r8, r9
	ctx.r[8].s64 = ctx.r[9].s16 as i64;
	// 82FAF6D8: 7F088800  cmpw cr6, r8, r17
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[17].s32, &mut ctx.xer);
	// 82FAF6DC: 409A002C  bne cr6, 0x82faf708
	if !ctx.cr[6].eq {
	pc = 0x82FAF708; continue 'dispatch;
	}
	// 82FAF6E0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAF6E4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FAF6E8: 554907BC  rlwinm r9, r10, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAF6EC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAF6F0: 419A000C  beq cr6, 0x82faf6fc
	if ctx.cr[6].eq {
	pc = 0x82FAF6FC; continue 'dispatch;
	}
	// 82FAF6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FAF6F8: 4BFFF081  bl 0x82fae778
	ctx.lr = 0x82FAF6FC;
	sub_82FAE778(ctx, base);
	// 82FAF6FC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAF700: 7DEBF92E  stwx r15, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[15].u32) };
	// 82FAF704: 9A1E0029  stb r16, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[16].u8 ) };
	// 82FAF708: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82FAF70C: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FAF710: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82FAF714: 7F04E000  cmpw cr6, r4, r28
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FAF718: 4198FFB0  blt cr6, 0x82faf6c8
	if ctx.cr[6].lt {
	pc = 0x82FAF6C8; continue 'dispatch;
	}
	// 82FAF71C: 48000298  b 0x82faf9b4
	pc = 0x82FAF9B4; continue 'dispatch;
	// 82FAF720: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAF724: 562A103A  slwi r10, r17, 2
	ctx.r[10].u32 = ctx.r[17].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF728: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAF72C: 3AB10001  addi r21, r17, 1
	ctx.r[21].s64 = ctx.r[17].s64 + 1;
	// 82FAF730: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FAF734: 82490010  lwz r18, 0x10(r9)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAF738: 61070004  ori r7, r8, 4
	ctx.r[7].u64 = ctx.r[8].u64 | 4;
	// 82FAF73C: 7F159000  cmpw cr6, r21, r18
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[18].s32, &mut ctx.xer);
	// 82FAF740: 7CEA592E  stwx r7, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 82FAF744: 40980270  bge cr6, 0x82faf9b4
	if !ctx.cr[6].lt {
	pc = 0x82FAF9B4; continue 'dispatch;
	}
	// 82FAF748: 56AB083C  slwi r11, r21, 1
	ctx.r[11].u32 = ctx.r[21].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FAF74C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FAF750: 7D755A14  add r11, r21, r11
	ctx.r[11].u64 = ctx.r[21].u64 + ctx.r[11].u64;
	// 82FAF754: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAF758: 557C2036  slwi r28, r11, 4
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FAF75C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAF760: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82FAF764: 56BA103A  slwi r26, r21, 2
	ctx.r[26].u32 = ctx.r[21].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82FAF768: 56B4083C  slwi r20, r21, 1
	ctx.r[20].u32 = ctx.r[21].u32.wrapping_shl(1);
	ctx.r[20].u64 = ctx.r[20].u32 as u64;
	// 82FAF76C: 7E759050  subf r19, r21, r18
	ctx.r[19].s64 = ctx.r[18].s64 - ctx.r[21].s64;
	// 82FAF770: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82FAF774: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82FAF778: 3B2BFFA0  addi r25, r11, -0x60
	ctx.r[25].s64 = ctx.r[11].s64 + -96;
	// 82FAF77C: 3B0AC5A0  addi r24, r10, -0x3a60
	ctx.r[24].s64 = ctx.r[10].s64 + -14944;
	// 82FAF780: 3AE9C5B0  addi r23, r9, -0x3a50
	ctx.r[23].s64 = ctx.r[9].s64 + -14928;
	// 82FAF784: 3AC80010  addi r22, r8, 0x10
	ctx.r[22].s64 = ctx.r[8].s64 + 16;
	// 82FAF788: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAF78C: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FAF790: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAF794: 7D4AA22E  lhzx r10, r10, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82FAF798: 7D480734  extsh r8, r10
	ctx.r[8].s64 = ctx.r[10].s16 as i64;
	// 82FAF79C: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FAF7A0: 7CC7482E  lwzx r6, r7, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAF7A4: 54C5077A  rlwinm r5, r6, 0, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAF7A8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FAF7AC: 419A01BC  beq cr6, 0x82faf968
	if ctx.cr[6].eq {
	pc = 0x82FAF968; continue 'dispatch;
	}
	// 82FAF7B0: 7D69D02E  lwzx r11, r9, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FAF7B4: 556807FE  clrlwi r8, r11, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82FAF7B8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAF7BC: 419A0198  beq cr6, 0x82faf954
	if ctx.cr[6].eq {
	pc = 0x82FAF954; continue 'dispatch;
	}
	// 82FAF7C0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FAF7C4: 7D440734  extsh r4, r10
	ctx.r[4].s64 = ctx.r[10].s16 as i64;
	// 82FAF7C8: 7FEBE214  add r31, r11, r28
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FAF7CC: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82FAF7D0: 419A0154  beq cr6, 0x82faf924
	if ctx.cr[6].eq {
	pc = 0x82FAF924; continue 'dispatch;
	}
	// 82FAF7D4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF7D8: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAF7DC: 552807BC  rlwinm r8, r9, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82FAF7E0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FAF7E4: 409A0018  bne cr6, 0x82faf7fc
	if !ctx.cr[6].eq {
	pc = 0x82FAF7FC; continue 'dispatch;
	}
	// 82FAF7E8: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF7EC: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82FAF7F0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAF7F4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FAF7F8: 4800000C  b 0x82faf804
	pc = 0x82FAF804; continue 'dispatch;
	// 82FAF7FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FAF800: 4BFFEF79  bl 0x82fae778
	ctx.lr = 0x82FAF804;
	sub_82FAE778(ctx, base);
	// 82FAF804: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAFA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FAFA00 size=580
    let mut pc: u32 = 0x82FAFA00;
    'dispatch: loop {
        match pc {
            0x82FAFA00 => {
    //   block [0x82FAFA00..0x82FAFC44)
	// 82FAFA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAFA04: 481F873D  bl 0x831a8140
	ctx.lr = 0x82FAFA08;
	sub_831A8130(ctx, base);
	// 82FAFA08: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82FAFA0C: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82FAFA10: 9421FCE0  stwu r1, -0x320(r1)
	ea = ctx.r[1].u32.wrapping_add(-800 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAFA14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FAFA18: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82FAFA1C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAFA20: 3A5D000C  addi r18, r29, 0xc
	ctx.r[18].s64 = ctx.r[29].s64 + 12;
	// 82FAFA24: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82FAFA28: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAFA2C: 3AAB6E5C  addi r21, r11, 0x6e5c
	ctx.r[21].s64 = ctx.r[11].s64 + 28252;
	// 82FAFA30: 3A8A5EA8  addi r20, r10, 0x5ea8
	ctx.r[20].s64 = ctx.r[10].s64 + 24232;
	// 82FAFA34: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FAFA38: 40990158  ble cr6, 0x82fafb90
	if !ctx.cr[6].gt {
	pc = 0x82FAFB90; continue 'dispatch;
	}
	// 82FAFA3C: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82FAFA40: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FAFA44: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAFA48: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FAFA4C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAFA50: C3C79528  lfs f30, -0x6ad8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82FAFA54: 3AFD0008  addi r23, r29, 8
	ctx.r[23].s64 = ctx.r[29].s64 + 8;
	// 82FAFA58: 3ADD0004  addi r22, r29, 4
	ctx.r[22].s64 = ctx.r[29].s64 + 4;
	// 82FAFA5C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FAFA60: 3B885EA4  addi r28, r8, 0x5ea4
	ctx.r[28].s64 = ctx.r[8].s64 + 24228;
	// 82FAFA64: 3B695E84  addi r27, r9, 0x5e84
	ctx.r[27].s64 = ctx.r[9].s64 + 24196;
	// 82FAFA68: 3B4A5E7C  addi r26, r10, 0x5e7c
	ctx.r[26].s64 = ctx.r[10].s64 + 24188;
	// 82FAFA6C: 3B2B5E70  addi r25, r11, 0x5e70
	ctx.r[25].s64 = ctx.r[11].s64 + 24176;
	// 82FAFA70: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAFA74: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAFA78: 7D785A14  add r11, r24, r11
	ctx.r[11].u64 = ctx.r[24].u64 + ctx.r[11].u64;
	// 82FAFA7C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAFA80: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAFA84: A0EB0002  lhz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FAFA88: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAFA8C: 7CE50734  extsh r5, r7
	ctx.r[5].s64 = ctx.r[7].s16 as i64;
	// 82FAFA90: 8089000C  lwz r4, 0xc(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAFA94: 7CC30734  extsh r3, r6
	ctx.r[3].s64 = ctx.r[6].s16 as i64;
	// 82FAFA98: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FAFA9C: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FAFAA0: 7FEA402E  lwzx r31, r10, r8
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FAFAA4: 7FC9202E  lwzx r30, r9, r4
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82FAFAA8: 891F0004  lbz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAFAAC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82FAFAB0: 419A00CC  beq cr6, 0x82fafb7c
	if ctx.cr[6].eq {
	pc = 0x82FAFB7C; continue 'dispatch;
	}
	// 82FAFAB4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82FAFAB8: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82FAFABC: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82FAFAC0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAFC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAFC48 size=72
    let mut pc: u32 = 0x82FAFC48;
    'dispatch: loop {
        match pc {
            0x82FAFC48 => {
    //   block [0x82FAFC48..0x82FAFC90)
	// 82FAFC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAFC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FAFC50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FAFC54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAFC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FAFC5C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FAFC60: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FAFC64: 394B5C20  addi r10, r11, 0x5c20
	ctx.r[10].s64 = ctx.r[11].s64 + 23584;
	// 82FAFC68: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FAFC6C: 4BFFCC35  bl 0x82fac8a0
	ctx.lr = 0x82FAFC70;
	sub_82FAC8A0(ctx, base);
	// 82FAFC70: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FAFC74: 39099EAC  addi r8, r9, -0x6154
	ctx.r[8].s64 = ctx.r[9].s64 + -24916;
	// 82FAFC78: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FAFC7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FAFC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FAFC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FAFC88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FAFC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FAFC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FAFC90 size=2396
    let mut pc: u32 = 0x82FAFC90;
    'dispatch: loop {
        match pc {
            0x82FAFC90 => {
    //   block [0x82FAFC90..0x82FB05EC)
	// 82FAFC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FAFC94: 481F849D  bl 0x831a8130
	ctx.lr = 0x82FAFC98;
	sub_831A8130(ctx, base);
	// 82FAFC98: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82FAFC9C: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FAFCA0: 7C701B78  mr r16, r3
	ctx.r[16].u64 = ctx.r[3].u64;
	// 82FAFCA4: 806D0000  lwz r3, 0(r13)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FAFCA8: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82FAFCAC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FAFCB0: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 82FAFCB4: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82FAFCB8: 93C101E4  stw r30, 0x1e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(484 as u32), ctx.r[30].u32 ) };
	// 82FAFCBC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FAFCC0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FAFCC4: 7D63482E  lwzx r11, r3, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAFCC8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAFCCC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAFCD0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FAFCD4: 4098002C  bge cr6, 0x82fafd00
	if !ctx.cr[6].lt {
	pc = 0x82FAFD00; continue 'dispatch;
	}
	// 82FAFCD8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FAFCDC: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82FAFCE0: 38C85F14  addi r6, r8, 0x5f14
	ctx.r[6].s64 = ctx.r[8].s64 + 24340;
	// 82FAFCE4: 38A74628  addi r5, r7, 0x4628
	ctx.r[5].s64 = ctx.r[7].s64 + 17960;
	// 82FAFCE8: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FAFCEC: 90AA000C  stw r5, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82FAFCF0: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82FAFCF4: 38EA0010  addi r7, r10, 0x10
	ctx.r[7].s64 = ctx.r[10].s64 + 16;
	// 82FAFCF8: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FAFCFC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FAFD00: 7D43482E  lwzx r10, r3, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FAFD04: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FAFD08: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FAFD0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FAFD10: 40980020  bge cr6, 0x82fafd30
	if !ctx.cr[6].lt {
	pc = 0x82FAFD30; continue 'dispatch;
	}
	// 82FAFD14: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FAFD18: 39095F00  addi r8, r9, 0x5f00
	ctx.r[8].s64 = ctx.r[9].s64 + 24320;
	// 82FAFD1C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FAFD20: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82FAFD24: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82FAFD28: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FAFD2C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FAFD30: 80F00014  lwz r7, 0x14(r16)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FAFD34: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FAFD38: 39C00000  li r14, 0
	ctx.r[14].s64 = 0;
	// 82FAFD3C: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82FAFD40: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
	// 82FAFD44: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FAFD48: 3AEB0010  addi r23, r11, 0x10
	ctx.r[23].s64 = ctx.r[11].s64 + 16;
	// 82FAFD4C: 40990108  ble cr6, 0x82fafe54
	if !ctx.cr[6].gt {
	pc = 0x82FAFE54; continue 'dispatch;
	}
	// 82FAFD50: 7DC67378  mr r6, r14
	ctx.r[6].u64 = ctx.r[14].u64;
	// 82FAFD54: 81700010  lwz r11, 0x10(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(16 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB05F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB05F0 size=100
    let mut pc: u32 = 0x82FB05F0;
    'dispatch: loop {
        match pc {
            0x82FB05F0 => {
    //   block [0x82FB05F0..0x82FB0654)
	// 82FB05F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB05F4: 481F7B75  bl 0x831a8168
	ctx.lr = 0x82FB05F8;
	sub_831A8130(ctx, base);
	// 82FB05F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB05FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB0600: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FB0604: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FB0608: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FB060C: 4BFFEBB5  bl 0x82faf1c0
	ctx.lr = 0x82FB0610;
	sub_82FAF1C0(ctx, base);
	// 82FB0610: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB0614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FB0618: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB061C: 4BFFEB7D  bl 0x82faf198
	ctx.lr = 0x82FB0620;
	sub_82FAF198(ctx, base);
	// 82FB0620: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FB0624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FB0628: 83CA0000  lwz r30, 0(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB062C: 4BFFE79D  bl 0x82faedc8
	ctx.lr = 0x82FB0630;
	sub_82FAEDC8(ctx, base);
	// 82FB0630: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FB0634: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FB0638: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FB063C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FB0640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB0644: 80C90000  lwz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0648: 4BFFF649  bl 0x82fafc90
	ctx.lr = 0x82FB064C;
	sub_82FAFC90(ctx, base);
	// 82FB064C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FB0650: 481F7B68  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB0658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB0658 size=128
    let mut pc: u32 = 0x82FB0658;
    'dispatch: loop {
        match pc {
            0x82FB0658 => {
    //   block [0x82FB0658..0x82FB06D8)
	// 82FB0658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB065C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB0660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB0664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB0668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB066C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB0670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FB0674: 390B5C20  addi r8, r11, 0x5c20
	ctx.r[8].s64 = ctx.r[11].s64 + 23584;
	// 82FB0678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB067C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82FB0680: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82FB0684: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB0688: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FB068C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FB0690: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FB0694: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FB0698: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FB069C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FB06A0: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82FB06A4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FB06A8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FB06AC: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82FB06B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FB06B4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FB06B8: 993F0034  stb r9, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u8 ) };
	// 82FB06BC: 4800001D  bl 0x82fb06d8
	ctx.lr = 0x82FB06C0;
	sub_82FB06D8(ctx, base);
	// 82FB06C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB06C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB06C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB06CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB06D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB06D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB06D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB06D8 size=724
    let mut pc: u32 = 0x82FB06D8;
    'dispatch: loop {
        match pc {
            0x82FB06D8 => {
    //   block [0x82FB06D8..0x82FB09AC)
	// 82FB06D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB06DC: 481F7A85  bl 0x831a8160
	ctx.lr = 0x82FB06E0;
	sub_831A8130(ctx, base);
	// 82FB06E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB06E4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FB06E8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FB06EC: 3BDA0008  addi r30, r26, 8
	ctx.r[30].s64 = ctx.r[26].s64 + 8;
	// 82FB06F0: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82FB06F4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB06F8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FB06FC: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0700: 915B0004  stw r10, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FB0704: 813A000C  lwz r9, 0xc(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0708: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB070C: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB0710: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FB0714: 40980060  bge cr6, 0x82fb0774
	if !ctx.cr[6].lt {
	pc = 0x82FB0774; continue 'dispatch;
	}
	// 82FB0718: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FB071C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB0720: 409A0020  bne cr6, 0x82fb0740
	if !ctx.cr[6].eq {
	pc = 0x82FB0740; continue 'dispatch;
	}
	// 82FB0724: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0728: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FB072C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FB0730: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0734: 55453032  slwi r5, r10, 6
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FB0738: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB073C: 4BEF0075  bl 0x82ea07b0
	ctx.lr = 0x82FB0740;
	sub_82EA07B0(ctx, base);
	// 82FB0740: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0744: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FB0748: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB074C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82FB0750: 55243032  slwi r4, r9, 6
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FB0754: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB0758: 4BEEFFD9  bl 0x82ea0730
	ctx.lr = 0x82FB075C;
	sub_82EA0730(ctx, base);
	// 82FB075C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FB0760: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0764: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82FB0768: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB076C: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82FB0770: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82FB0774: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0778: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 82FB077C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0780: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82FB0784: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB0788: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FB078C: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0790: 40990050  ble cr6, 0x82fb07e0
	if !ctx.cr[6].gt {
	pc = 0x82FB07E0; continue 'dispatch;
	}
	// 82FB0794: 39680020  addi r11, r8, 0x20
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
	// 82FB0798: 39470002  addi r10, r7, 2
	ctx.r[10].s64 = ctx.r[7].s64 + 2;
	// 82FB079C: 7D083850  subf r8, r8, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82FB07A0: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82FB07A4: 38E0002E  li r7, 0x2e
	ctx.r[7].s64 = 46;
	// 82FB07A8: A0AAFFFE  lhz r5, -2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FB07AC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB07B0: B0ABFFE0  sth r5, -0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-32 as u32), ctx.r[5].u16 ) };
	// 82FB07B4: A08A0000  lhz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB07B8: B08BFFE2  sth r4, -0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-30 as u32), ctx.r[4].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB09B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FB09B0 size=260
    let mut pc: u32 = 0x82FB09B0;
    'dispatch: loop {
        match pc {
            0x82FB09B0 => {
    //   block [0x82FB09B0..0x82FB0AB4)
	// 82FB09B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB09B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB09B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FB09BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB09C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB09C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB09C8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB09CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FB09D0: 392B5F24  addi r9, r11, 0x5f24
	ctx.r[9].s64 = ctx.r[11].s64 + 24356;
	// 82FB09D4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82FB09D8: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FB09DC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 82FB09E0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB09E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB09E8: 481F7B29  bl 0x831a8510
	ctx.lr = 0x82FB09EC;
	sub_831A8510(ctx, base);
	// 82FB09EC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82FB09F0: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
	// 82FB09F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FB09F8: 38A77060  addi r5, r7, 0x7060
	ctx.r[5].s64 = ctx.r[7].s64 + 28768;
	// 82FB09FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB0A00: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB0A04: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FB0A08: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82FB0A0C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82FB0A10: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82FB0A14: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FB0A18: 915F00A0  stw r10, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82FB0A1C: 813E0034  lwz r9, 0x34(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FB0A20: 913F00A4  stw r9, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[9].u32 ) };
	// 82FB0A24: 811E0038  lwz r8, 0x38(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FB0A28: 911F00A8  stw r8, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[8].u32 ) };
	// 82FB0A2C: 80FE003C  lwz r7, 0x3c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FB0A30: D01F00B0  stfs f0, 0xb0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 82FB0A34: 90FF00AC  stw r7, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[7].u32 ) };
	// 82FB0A38: 98DF00B4  stb r6, 0xb4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[6].u8 ) };
	// 82FB0A3C: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0A40: 90DF00C0  stw r6, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[6].u32 ) };
	// 82FB0A44: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0A48: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 82FB0A4C: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0A50: 913F00C8  stw r9, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 82FB0A54: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0A58: 911F00CC  stw r8, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[8].u32 ) };
	// 82FB0A5C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0A60: 90FF00D0  stw r7, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[7].u32 ) };
	// 82FB0A64: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0A68: 90DF00D4  stw r6, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[6].u32 ) };
	// 82FB0A6C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0A70: 909F00D8  stw r4, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[4].u32 ) };
	// 82FB0A74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0A78: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 82FB0A7C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0A80: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82FB0A84: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0A88: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 82FB0A8C: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0A90: 911F00E8  stw r8, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[8].u32 ) };
	// 82FB0A94: 80E5000C  lwz r7, 0xc(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0A98: 90FF00EC  stw r7, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[7].u32 ) };
	// 82FB0A9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FB0AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB0AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB0AA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FB0AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB0AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB0AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB0AB8 size=284
    let mut pc: u32 = 0x82FB0AB8;
    'dispatch: loop {
        match pc {
            0x82FB0AB8 => {
    //   block [0x82FB0AB8..0x82FB0BD4)
	// 82FB0AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB0ABC: 481F76AD  bl 0x831a8168
	ctx.lr = 0x82FB0AC0;
	sub_831A8130(ctx, base);
	// 82FB0AC0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82FB0AC4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FB0AC8: 392A7060  addi r9, r10, 0x7060
	ctx.r[9].s64 = ctx.r[10].s64 + 28768;
	// 82FB0ACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FB0AD0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FB0AD4: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82FB0AD8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82FB0ADC: B1630008  sth r11, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82FB0AE0: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82FB0AE4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FB0AE8: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82FB0AEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FB0AF0: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 82FB0AF4: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0AF8: 3FC08201  lis r30, -0x7dff
	ctx.r[30].s64 = -2113863680;
	// 82FB0AFC: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82FB0B00: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82FB0B04: 83E80004  lwz r31, 4(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0B08: C00508A4  lfs f0, 0x8a4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB0B0C: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82FB0B10: C1840B54  lfs f12, 0xb54(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2900 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FB0B14: 83A80008  lwz r29, 8(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0B18: C1ABA9F0  lfs f13, -0x5610(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22032 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB0B1C: 93A30018  stw r29, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82FB0B20: 3FE08200  lis r31, -0x7e00
	ctx.r[31].s64 = -2113929216;
	// 82FB0B24: 8108000C  lwz r8, 0xc(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0B28: 3F808201  lis r28, -0x7dff
	ctx.r[28].s64 = -2113863680;
	// 82FB0B2C: 9103001C  stw r8, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82FB0B30: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
	// 82FB0B34: 80A70000  lwz r5, 0(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0B38: 3FA08209  lis r29, -0x7df7
	ctx.r[29].s64 = -2113339392;
	// 82FB0B3C: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82FB0B40: C17E9534  lfs f11, -0x6acc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB0B44: 80870004  lwz r4, 4(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0B48: C15F08A8  lfs f10, 0x8a8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FB0B4C: 90830024  stw r4, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 82FB0B50: C13C9450  lfs f9, -0x6bb0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FB0B54: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0B58: C11DACFC  lfs f8, -0x5304(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-21252 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FB0B5C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FB0B60: 80E7000C  lwz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0B64: 90E3002C  stw r7, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82FB0B68: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0B6C: 90A30030  stw r5, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82FB0B70: 80860004  lwz r4, 4(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0B74: 90830034  stw r4, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 82FB0B78: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0B7C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82FB0B80: 8106000C  lwz r8, 0xc(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0B84: 9103003C  stw r8, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[8].u32 ) };
	// 82FB0B88: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB0B8C: 90E30040  stw r7, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[7].u32 ) };
	// 82FB0B90: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB0B94: 90C30044  stw r6, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[6].u32 ) };
	// 82FB0B98: 80A90008  lwz r5, 8(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB0B9C: 90A30048  stw r5, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[5].u32 ) };
	// 82FB0BA0: 8089000C  lwz r4, 0xc(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB0BA4: D0030050  stfs f0, 0x50(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82FB0BA8: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82FB0BAC: 9083004C  stw r4, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[4].u32 ) };
	// 82FB0BB0: D0030058  stfs f0, 0x58(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82FB0BB4: D1A3005C  stfs f13, 0x5c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82FB0BB8: D1830060  stfs f12, 0x60(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82FB0BBC: D1630064  stfs f11, 0x64(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82FB0BC0: D1430068  stfs f10, 0x68(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82FB0BC4: D123006C  stfs f9, 0x6c(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82FB0BC8: D1030070  stfs f8, 0x70(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82FB0BCC: 99430074  stb r10, 0x74(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[10].u8 ) };
	// 82FB0BD0: 481F75E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB0BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB0BD8 size=60
    let mut pc: u32 = 0x82FB0BD8;
    'dispatch: loop {
        match pc {
            0x82FB0BD8 => {
    //   block [0x82FB0BD8..0x82FB0C14)
	// 82FB0BD8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82FB0BDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FB0BE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FB0BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB0BE8: 99230060  stb r9, 0x60(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[9].u8 ) };
	// 82FB0BEC: C1AB6150  lfs f13, 0x6150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB0BF0: 91030064  stw r8, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82FB0BF4: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB0BF8: D1A30068  stfs f13, 0x68(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82FB0BFC: D003006C  stfs f0, 0x6c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82FB0C00: D0030070  stfs f0, 0x70(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82FB0C04: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82FB0C08: D0030078  stfs f0, 0x78(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82FB0C0C: D003007C  stfs f0, 0x7c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82FB0C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB0C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FB0C18 size=720
    let mut pc: u32 = 0x82FB0C18;
    'dispatch: loop {
        match pc {
            0x82FB0C18 => {
    //   block [0x82FB0C18..0x82FB0EE8)
	// 82FB0C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB0C1C: 481F7539  bl 0x831a8154
	ctx.lr = 0x82FB0C20;
	sub_831A8130(ctx, base);
	// 82FB0C20: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82FB0C24: 481F7E55  bl 0x831a8a78
	ctx.lr = 0x82FB0C28;
	sub_831A8A40(ctx, base);
	// 82FB0C28: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB0C2C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82FB0C30: C3A4007C  lfs f29, 0x7c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(124 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82FB0C34: D3A10060  stfs f29, 0x60(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82FB0C38: 3941005C  addi r10, r1, 0x5c
	ctx.r[10].s64 = ctx.r[1].s64 + 92;
	// 82FB0C3C: C0040080  lfs f0, 0x80(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB0C40: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FB0C44: 3BA40040  addi r29, r4, 0x40
	ctx.r[29].s64 = ctx.r[4].s64 + 64;
	// 82FB0C48: FFC00050  fneg f30, f0
	ctx.f[30].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FB0C4C: D3C1005C  stfs f30, 0x5c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82FB0C50: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB0EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB0EE8 size=2324
    let mut pc: u32 = 0x82FB0EE8;
    'dispatch: loop {
        match pc {
            0x82FB0EE8 => {
    //   block [0x82FB0EE8..0x82FB17FC)
	// 82FB0EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB0EEC: 481F7245  bl 0x831a8130
	ctx.lr = 0x82FB0EF0;
	sub_831A8130(ctx, base);
	// 82FB0EF0: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82FB0EF4: 481F7B85  bl 0x831a8a78
	ctx.lr = 0x82FB0EF8;
	sub_831A8A40(ctx, base);
	// 82FB0EF8: 3980FF20  li r12, -0xe0
	ctx.r[12].s64 = -224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB1800 size=72
    let mut pc: u32 = 0x82FB1800;
    'dispatch: loop {
        match pc {
            0x82FB1800 => {
    //   block [0x82FB1800..0x82FB1848)
	// 82FB1800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB1804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB1808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB180C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB1810: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB1814: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB1818: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82FB181C: 392B5F34  addi r9, r11, 0x5f34
	ctx.r[9].s64 = ctx.r[11].s64 + 24372;
	// 82FB1820: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FB1824: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB1828: 419A000C  beq cr6, 0x82fb1834
	if ctx.cr[6].eq {
	pc = 0x82FB1834; continue 'dispatch;
	}
	// 82FB182C: 4B30EA3D  bl 0x822c0268
	ctx.lr = 0x82FB1830;
	sub_822C0268(ctx, base);
	// 82FB1830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB1834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB1838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB183C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB1840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB1844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB1848 size=36
    let mut pc: u32 = 0x82FB1848;
    'dispatch: loop {
        match pc {
            0x82FB1848 => {
    //   block [0x82FB1848..0x82FB186C)
	// 82FB1848: 7C6A0734  extsh r10, r3
	ctx.r[10].s64 = ctx.r[3].s16 as i64;
	// 82FB184C: 7C890734  extsh r9, r4
	ctx.r[9].s64 = ctx.r[4].s16 as i64;
	// 82FB1850: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FB1854: 40980018  bge cr6, 0x82fb186c
	if !ctx.cr[6].lt {
		sub_82FB186C(ctx, base);
		return;
	}
	// 82FB1858: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB185C: 7D0B2A2E  lhzx r8, r11, r5
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82FB1860: 7D070734  extsh r7, r8
	ctx.r[7].s64 = ctx.r[8].s16 as i64;
	// 82FB1864: 7F075000  cmpw cr6, r7, r10
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FB1868: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB186C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB186C size=36
    let mut pc: u32 = 0x82FB186C;
    'dispatch: loop {
        match pc {
            0x82FB186C => {
    //   block [0x82FB186C..0x82FB1890)
	// 82FB186C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FB1870: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB1874: 40990030  ble cr6, 0x82fb18a4
	if !ctx.cr[6].gt {
		sub_82FB1890(ctx, base);
		return;
	}
	// 82FB1878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB187C: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FB1880: 7CE82A2E  lhzx r7, r8, r5
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82FB1884: 7CE60734  extsh r6, r7
	ctx.r[6].s64 = ctx.r[7].s16 as i64;
	// 82FB1888: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FB188C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB1890 size=28
    let mut pc: u32 = 0x82FB1890;
    'dispatch: loop {
        match pc {
            0x82FB1890 => {
    //   block [0x82FB1890..0x82FB18AC)
	// 82FB1890: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FB1894: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82FB1898: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB189C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FB18A0: 4198FFDC  blt cr6, 0x82fb187c
	if ctx.cr[6].lt {
		sub_82FB186C(ctx, base);
		return;
	}
	// 82FB18A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FB18A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB18B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB18B0 size=208
    let mut pc: u32 = 0x82FB18B0;
    'dispatch: loop {
        match pc {
            0x82FB18B0 => {
    //   block [0x82FB18B0..0x82FB1980)
	// 82FB18B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB18B4: 481F68B1  bl 0x831a8164
	ctx.lr = 0x82FB18B8;
	sub_831A8130(ctx, base);
	// 82FB18B8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82FB18BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB18C0: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB18C4: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82FB18C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FB18CC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82FB18D0: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FB18D4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB18D8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB18DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB18E0: 40980020  bge cr6, 0x82fb1900
	if !ctx.cr[6].lt {
	pc = 0x82FB1900; continue 'dispatch;
	}
	// 82FB18E4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FB18E8: 39095F3C  addi r8, r9, 0x5f3c
	ctx.r[8].s64 = ctx.r[9].s64 + 24380;
	// 82FB18EC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB18F0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82FB18F4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82FB18F8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FB18FC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FB1900: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB1904: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FB1908: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB190C: 40990038  ble cr6, 0x82fb1944
	if !ctx.cr[6].gt {
	pc = 0x82FB1944; continue 'dispatch;
	}
	// 82FB1910: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FB1914: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB1918: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FB191C: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB1920: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB1924: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB1928: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82FB192C: 4E800421  bctrl
	ctx.lr = 0x82FB1930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB1930: 811D0014  lwz r8, 0x14(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB1934: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FB1938: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82FB193C: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FB1940: 4198FFD4  blt cr6, 0x82fb1914
	if ctx.cr[6].lt {
	pc = 0x82FB1914; continue 'dispatch;
	}
	// 82FB1944: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FB1948: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB194C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB1950: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB1954: 40980020  bge cr6, 0x82fb1974
	if !ctx.cr[6].lt {
	pc = 0x82FB1974; continue 'dispatch;
	}
	// 82FB1958: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FB195C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82FB1960: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB1964: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82FB1968: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82FB196C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FB1970: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FB1974: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FB1978: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82FB197C: 481F6838  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB1980 size=1372
    let mut pc: u32 = 0x82FB1980;
    'dispatch: loop {
        match pc {
            0x82FB1980 => {
    //   block [0x82FB1980..0x82FB1EDC)
	// 82FB1980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB1984: 481F67C9  bl 0x831a814c
	ctx.lr = 0x82FB1988;
	sub_831A8130(ctx, base);
	// 82FB1988: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82FB198C: 481F70E9  bl 0x831a8a74
	ctx.lr = 0x82FB1990;
	sub_831A8A40(ctx, base);
	// 82FB1990: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB1994: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB1998: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82FB199C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FB19A0: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 82FB19A4: 7EAA5A14  add r21, r10, r11
	ctx.r[21].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FB19A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FB19AC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB19B0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB19B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB19B8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB19BC: 40980020  bge cr6, 0x82fb19dc
	if !ctx.cr[6].lt {
	pc = 0x82FB19DC; continue 'dispatch;
	}
	// 82FB19C0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FB19C4: 39095F48  addi r8, r9, 0x5f48
	ctx.r[8].s64 = ctx.r[9].s64 + 24392;
	// 82FB19C8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB19CC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82FB19D0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82FB19D4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FB19D8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB1EE0 size=232
    let mut pc: u32 = 0x82FB1EE0;
    'dispatch: loop {
        match pc {
            0x82FB1EE0 => {
    //   block [0x82FB1EE0..0x82FB1FC8)
	// 82FB1EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB1EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB1EE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB1EEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB1EF0: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB1EF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FB1EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB1EFC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB1F00: 409900B4  ble cr6, 0x82fb1fb4
	if !ctx.cr[6].gt {
	pc = 0x82FB1FB4; continue 'dispatch;
	}
	// 82FB1F04: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB1F08: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB1F0C: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FB1F10: 419A0028  beq cr6, 0x82fb1f38
	if ctx.cr[6].eq {
	pc = 0x82FB1F38; continue 'dispatch;
	}
	// 82FB1F14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FB1F18: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FB1F1C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FB1F20: 4198FFE8  blt cr6, 0x82fb1f08
	if ctx.cr[6].lt {
	pc = 0x82FB1F08; continue 'dispatch;
	}
	// 82FB1F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB1F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB1F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB1F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB1F34: 4E800020  blr
	return;
	// 82FB1F38: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FB1F3C: 419A0078  beq cr6, 0x82fb1fb4
	if ctx.cr[6].eq {
	pc = 0x82FB1FB4; continue 'dispatch;
	}
	// 82FB1F40: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB1F44: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FB1F48: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB1F4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FB1F50: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82FB1F54: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 82FB1F58: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FB1F5C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FB1F60: 7CC7402E  lwzx r6, r7, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FB1F64: 7CC9412E  stwx r6, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 82FB1F68: 409A0008  bne cr6, 0x82fb1f70
	if !ctx.cr[6].eq {
	pc = 0x82FB1F70; continue 'dispatch;
	}
	// 82FB1F6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FB1F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB1F74: 4800E525  bl 0x82fc0498
	ctx.lr = 0x82FB1F78;
	sub_82FC0498(ctx, base);
	// 82FB1F78: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB1F7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB1F80: 419A0034  beq cr6, 0x82fb1fb4
	if ctx.cr[6].eq {
	pc = 0x82FB1FB4; continue 'dispatch;
	}
	// 82FB1F84: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FB1F88: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82FB1F8C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82FB1F90: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82FB1F94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB1F98: 409A001C  bne cr6, 0x82fb1fb4
	if !ctx.cr[6].eq {
	pc = 0x82FB1FB4; continue 'dispatch;
	}
	// 82FB1F9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB1FA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FB1FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB1FA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB1FAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FB1FB0: 4E800421  bctrl
	ctx.lr = 0x82FB1FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB1FB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB1FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB1FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB1FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB1FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB1FC8 size=24
    let mut pc: u32 = 0x82FB1FC8;
    'dispatch: loop {
        match pc {
            0x82FB1FC8 => {
    //   block [0x82FB1FC8..0x82FB1FE0)
	// 82FB1FC8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FB1FCC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FB1FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB1FD4: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB1FD8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FB1FDC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB1FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB1FE0 size=36
    let mut pc: u32 = 0x82FB1FE0;
    'dispatch: loop {
        match pc {
            0x82FB1FE0 => {
    //   block [0x82FB1FE0..0x82FB2004)
	// 82FB1FE0: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB1FE4: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB1FE8: 7F071840  cmplw cr6, r7, r3
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FB1FEC: 419A0018  beq cr6, 0x82fb2004
	if ctx.cr[6].eq {
		sub_82FB2004(ctx, base);
		return;
	}
	// 82FB1FF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FB1FF4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82FB1FF8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FB1FFC: 4198FFE8  blt cr6, 0x82fb1fe4
	if ctx.cr[6].lt {
	pc = 0x82FB1FE4; continue 'dispatch;
	}
	// 82FB2000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB2004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB2004 size=8
    let mut pc: u32 = 0x82FB2004;
    'dispatch: loop {
        match pc {
            0x82FB2004 => {
    //   block [0x82FB2004..0x82FB200C)
	// 82FB2004: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FB2008: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB200C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB200C size=52
    let mut pc: u32 = 0x82FB200C;
    'dispatch: loop {
        match pc {
            0x82FB200C => {
    //   block [0x82FB200C..0x82FB2040)
	// 82FB200C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB2010: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FB2014: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB2018: 34CAFFF8  addic. r6, r10, -8
	ctx.xer.ca = (ctx.r[10].u32 > (!(-8 as u32)));
	ctx.r[6].s64 = ctx.r[10].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82FB201C: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 82FB2020: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FB2024: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FB2028: 7C85382E  lwzx r4, r5, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FB202C: 7C88392E  stwx r4, r8, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), ctx.r[4].u32) };
	// 82FB2030: 40820008  bne 0x82fb2038
	if !ctx.cr[0].eq {
	pc = 0x82FB2038; continue 'dispatch;
	}
	// 82FB2034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FB2038: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82FB203C: 4800E45C  b 0x82fc0498
	sub_82FC0498(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB2040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB2040 size=464
    let mut pc: u32 = 0x82FB2040;
    'dispatch: loop {
        match pc {
            0x82FB2040 => {
    //   block [0x82FB2040..0x82FB2210)
	// 82FB2040: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FB2044: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB2048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FB204C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB2050: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB2054: 409901B4  ble cr6, 0x82fb2208
	if !ctx.cr[6].gt {
	pc = 0x82FB2208; continue 'dispatch;
	}
	// 82FB2058: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB205C: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82FB2060: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82FB2064: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FB2068: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FB206C: C0EBAC1C  lfs f7, -0x53e4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21476 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FB2070: C10ABA78  lfs f8, -0x4588(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FB2074: C12908A4  lfs f9, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FB2078: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB207C: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FB2080: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FB2084: EDAA082A  fadds f13, f10, f1
	ctx.f[13].f64 = ((ctx.f[10].f64 + ctx.f[1].f64) as f32) as f64;
	// 82FB2088: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB208C: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB2090: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB2094: 4099000C  ble cr6, 0x82fb20a0
	if !ctx.cr[6].gt {
	pc = 0x82FB20A0; continue 'dispatch;
	}
	// 82FB2098: FD604890  fmr f11, f9
	ctx.f[11].f64 = ctx.f[9].f64;
	// 82FB209C: 48000008  b 0x82fb20a4
	pc = 0x82FB20A4; continue 'dispatch;
	// 82FB20A0: FD604090  fmr f11, f8
	ctx.f[11].f64 = ctx.f[8].f64;
	// 82FB20A4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB20A8: 4099000C  ble cr6, 0x82fb20b4
	if !ctx.cr[6].gt {
	pc = 0x82FB20B4; continue 'dispatch;
	}
	// 82FB20AC: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82FB20B0: 48000008  b 0x82fb20b8
	pc = 0x82FB20B8; continue 'dispatch;
	// 82FB20B4: FD803890  fmr f12, f7
	ctx.f[12].f64 = ctx.f[7].f64;
	// 82FB20B8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB20BC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB20C0: 4099013C  ble cr6, 0x82fb21fc
	if !ctx.cr[6].gt {
	pc = 0x82FB21FC; continue 'dispatch;
	}
	// 82FB20C4: 80CB001C  lwz r6, 0x1c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FB20C8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82FB20CC: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB20D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB20D4: 80E90008  lwz r7, 8(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB20D8: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82FB20DC: 419800C8  blt cr6, 0x82fb21a4
	if ctx.cr[6].lt {
	pc = 0x82FB21A4; continue 'dispatch;
	}
	// 82FB20E0: 3947FFFC  addi r10, r7, -4
	ctx.r[10].s64 = ctx.r[7].s64 + -4;
	// 82FB20E4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB20E8: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FB20EC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82FB20F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FB20F4: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FB20F8: C00BFFF0  lfs f0, -0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB20FC: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FB2100: 4198000C  blt cr6, 0x82fb210c
	if ctx.cr[6].lt {
	pc = 0x82FB210C; continue 'dispatch;
	}
	// 82FB2104: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FB2108: 40990014  ble cr6, 0x82fb211c
	if !ctx.cr[6].gt {
	pc = 0x82FB211C; continue 'dispatch;
	}
	// 82FB210C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FB2110: 41980010  blt cr6, 0x82fb2120
	if ctx.cr[6].lt {
	pc = 0x82FB2120; continue 'dispatch;
	}
	// 82FB2114: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB2118: 41990008  bgt cr6, 0x82fb2120
	if ctx.cr[6].gt {
	pc = 0x82FB2120; continue 'dispatch;
	}
	// 82FB211C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FB2120: C00BFFF8  lfs f0, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB2124: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FB2128: 4198000C  blt cr6, 0x82fb2134
	if ctx.cr[6].lt {
	pc = 0x82FB2134; continue 'dispatch;
	}
	// 82FB212C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FB2130: 40990014  ble cr6, 0x82fb2144
	if !ctx.cr[6].gt {
	pc = 0x82FB2144; continue 'dispatch;
	}
	// 82FB2134: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FB2138: 41980010  blt cr6, 0x82fb2148
	if ctx.cr[6].lt {
	pc = 0x82FB2148; continue 'dispatch;
	}
	// 82FB213C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB2140: 41990008  bgt cr6, 0x82fb2148
	if ctx.cr[6].gt {
	pc = 0x82FB2148; continue 'dispatch;
	}
	// 82FB2144: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FB2148: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB214C: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FB2150: 4198000C  blt cr6, 0x82fb215c
	if ctx.cr[6].lt {
	pc = 0x82FB215C; continue 'dispatch;
	}
	// 82FB2154: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FB2158: 40990014  ble cr6, 0x82fb216c
	if !ctx.cr[6].gt {
	pc = 0x82FB216C; continue 'dispatch;
	}
	// 82FB215C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FB2160: 41980010  blt cr6, 0x82fb2170
	if ctx.cr[6].lt {
	pc = 0x82FB2170; continue 'dispatch;
	}
	// 82FB2164: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB2168: 41990008  bgt cr6, 0x82fb2170
	if ctx.cr[6].gt {
	pc = 0x82FB2170; continue 'dispatch;
	}
	// 82FB216C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FB2170: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB2174: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FB2178: 4198000C  blt cr6, 0x82fb2184
	if ctx.cr[6].lt {
	pc = 0x82FB2184; continue 'dispatch;
	}
	// 82FB217C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FB2180: 40990014  ble cr6, 0x82fb2194
	if !ctx.cr[6].gt {
	pc = 0x82FB2194; continue 'dispatch;
	}
	// 82FB2184: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FB2188: 41980010  blt cr6, 0x82fb2198
	if ctx.cr[6].lt {
	pc = 0x82FB2198; continue 'dispatch;
	}
	// 82FB218C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB2190: 41990008  bgt cr6, 0x82fb2198
	if ctx.cr[6].gt {
	pc = 0x82FB2198; continue 'dispatch;
	}
	// 82FB2194: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FB2198: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB219C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82FB21A0: 4082FF58  bne 0x82fb20f8
	if !ctx.cr[0].eq {
	pc = 0x82FB20F8; continue 'dispatch;
	}
	// 82FB21A4: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82FB21A8: 40980048  bge cr6, 0x82fb21f0
	if !ctx.cr[6].lt {
	pc = 0x82FB21F0; continue 'dispatch;
	}
	// 82FB21AC: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB21B0: 55091838  slwi r9, r8, 3
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FB21B4: 7D683850  subf r11, r8, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82FB21B8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82FB21BC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB21C0: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FB21C4: 4198000C  blt cr6, 0x82fb21d0
	if ctx.cr[6].lt {
	pc = 0x82FB21D0; continue 'dispatch;
	}
	// 82FB21C8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FB21CC: 40990014  ble cr6, 0x82fb21e0
	if !ctx.cr[6].gt {
	pc = 0x82FB21E0; continue 'dispatch;
	}
	// 82FB21D0: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FB21D4: 41980010  blt cr6, 0x82fb21e4
	if ctx.cr[6].lt {
	pc = 0x82FB21E4; continue 'dispatch;
	}
	// 82FB21D8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB21DC: 41990008  bgt cr6, 0x82fb21e4
	if ctx.cr[6].gt {
	pc = 0x82FB21E4; continue 'dispatch;
	}
	// 82FB21E0: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FB21E4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB21E8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82FB21EC: 4082FFD0  bne 0x82fb21bc
	if !ctx.cr[0].eq {
	pc = 0x82FB21BC; continue 'dispatch;
	}
	// 82FB21F0: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FB21F4: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82FB21F8: 4082FED4  bne 0x82fb20cc
	if !ctx.cr[0].eq {
	pc = 0x82FB20CC; continue 'dispatch;
	}
	// 82FB21FC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FB2200: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82FB2204: 4082FE74  bne 0x82fb2078
	if !ctx.cr[0].eq {
	pc = 0x82FB2078; continue 'dispatch;
	}
	// 82FB2208: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82FB220C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB2210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB2210 size=324
    let mut pc: u32 = 0x82FB2210;
    'dispatch: loop {
        match pc {
            0x82FB2210 => {
    //   block [0x82FB2210..0x82FB2354)
	// 82FB2210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB2214: 481F5F4D  bl 0x831a8160
	ctx.lr = 0x82FB2218;
	sub_831A8130(ctx, base);
	// 82FB2218: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB221C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FB2220: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FB2224: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB2228: 40990128  ble cr6, 0x82fb2350
	if !ctx.cr[6].gt {
	pc = 0x82FB2350; continue 'dispatch;
	}
	// 82FB222C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82FB2230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FB2234: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FB2238: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FB223C: C0EBAC1C  lfs f7, -0x53e4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21476 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FB2240: C10ABA78  lfs f8, -0x4588(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82FB2244: C12908A4  lfs f9, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FB2248: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB224C: 7D5B582E  lwzx r10, r27, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB2250: 83EA0028  lwz r31, 0x28(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FB2254: C14A0008  lfs f10, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FB2258: EDAA082A  fadds f13, f10, f1
	ctx.f[13].f64 = ((ctx.f[10].f64 + ctx.f[1].f64) as f32) as f64;
	// 82FB225C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB2260: C01D000C  lfs f0, 0xc(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB2264: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB2268: 4099000C  ble cr6, 0x82fb2274
	if !ctx.cr[6].gt {
	pc = 0x82FB2274; continue 'dispatch;
	}
	// 82FB226C: FD604890  fmr f11, f9
	ctx.f[11].f64 = ctx.f[9].f64;
	// 82FB2270: 48000008  b 0x82fb2278
	pc = 0x82FB2278; continue 'dispatch;
	// 82FB2274: FD604090  fmr f11, f8
	ctx.f[11].f64 = ctx.f[8].f64;
	// 82FB2278: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB227C: 4099000C  ble cr6, 0x82fb2288
	if !ctx.cr[6].gt {
	pc = 0x82FB2288; continue 'dispatch;
	}
	// 82FB2280: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82FB2284: 48000008  b 0x82fb228c
	pc = 0x82FB228C; continue 'dispatch;
	// 82FB2288: FD803890  fmr f12, f7
	ctx.f[12].f64 = ctx.f[7].f64;
	// 82FB228C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB2290: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FB2294: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB2298: 409900A4  ble cr6, 0x82fb233c
	if !ctx.cr[6].gt {
	pc = 0x82FB233C; continue 'dispatch;
	}
	// 82FB229C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FB22A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FB22A4: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FB22A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FB22AC: 7D2BF02E  lwzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FB22B0: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB22B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB22B8: 4099006C  ble cr6, 0x82fb2324
	if !ctx.cr[6].gt {
	pc = 0x82FB2324; continue 'dispatch;
	}
	// 82FB22BC: 54EB1838  slwi r11, r7, 3
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB22C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FB22C4: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82FB22C8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB22CC: 7C0A442E  lfsx f0, r10, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB22D0: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82FB22D4: 4198000C  blt cr6, 0x82fb22e0
	if ctx.cr[6].lt {
	pc = 0x82FB22E0; continue 'dispatch;
	}
	// 82FB22D8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82FB22DC: 40990014  ble cr6, 0x82fb22f0
	if !ctx.cr[6].gt {
	pc = 0x82FB22F0; continue 'dispatch;
	}
	// 82FB22E0: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82FB22E4: 4198002C  blt cr6, 0x82fb2310
	if ctx.cr[6].lt {
	pc = 0x82FB2310; continue 'dispatch;
	}
	// 82FB22E8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB22EC: 41990024  bgt cr6, 0x82fb2310
	if ctx.cr[6].gt {
	pc = 0x82FB2310; continue 'dispatch;
	}
	// 82FB22F0: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB22F4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82FB22F8: 7D08322E  lhzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FB22FC: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82FB2300: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB2304: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82FB2308: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FB230C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FB2310: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB2314: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82FB2318: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82FB231C: 7F044000  cmpw cr6, r4, r8
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FB2320: 4198FFA8  blt cr6, 0x82fb22c8
	if ctx.cr[6].lt {
	pc = 0x82FB22C8; continue 'dispatch;
	}
	// 82FB2324: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB2328: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FB232C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FB2330: 38C60002  addi r6, r6, 2
	ctx.r[6].s64 = ctx.r[6].s64 + 2;
	// 82FB2334: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB2338: 4198FF6C  blt cr6, 0x82fb22a4
	if ctx.cr[6].lt {
	pc = 0x82FB22A4; continue 'dispatch;
	}
	// 82FB233C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB2340: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FB2344: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82FB2348: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB234C: 4198FEFC  blt cr6, 0x82fb2248
	if ctx.cr[6].lt {
	pc = 0x82FB2248; continue 'dispatch;
	}
	// 82FB2350: 481F5E60  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB2358 size=84
    let mut pc: u32 = 0x82FB2358;
    'dispatch: loop {
        match pc {
            0x82FB2358 => {
    //   block [0x82FB2358..0x82FB23AC)
	// 82FB2358: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB235C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FB2360: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FB2364: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82FB2368: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82FB236C: 38CB5F34  addi r6, r11, 0x5f34
	ctx.r[6].s64 = ctx.r[11].s64 + 24372;
	// 82FB2370: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82FB2374: 38AA5F64  addi r5, r10, 0x5f64
	ctx.r[5].s64 = ctx.r[10].s64 + 24420;
	// 82FB2378: 39495F58  addi r10, r9, 0x5f58
	ctx.r[10].s64 = ctx.r[9].s64 + 24408;
	// 82FB237C: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82FB2380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB2384: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FB2388: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82FB238C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FB2390: C007964C  lfs f0, -0x69b4(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB2394: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FB2398: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FB239C: 91230018  stw r9, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82FB23A0: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82FB23A4: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82FB23A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB23B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB23B0 size=240
    let mut pc: u32 = 0x82FB23B0;
    'dispatch: loop {
        match pc {
            0x82FB23B0 => {
    //   block [0x82FB23B0..0x82FB24A0)
	// 82FB23B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB23B4: 481F5DB5  bl 0x831a8168
	ctx.lr = 0x82FB23B8;
	sub_831A8130(ctx, base);
	// 82FB23B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB23BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB23C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB23C4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FB23C8: 390B5F64  addi r8, r11, 0x5f64
	ctx.r[8].s64 = ctx.r[11].s64 + 24420;
	// 82FB23CC: 38E95F58  addi r7, r9, 0x5f58
	ctx.r[7].s64 = ctx.r[9].s64 + 24408;
	// 82FB23D0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB23D4: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82FB23D8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB23DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FB23E0: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FB23E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB23E8: 4099006C  ble cr6, 0x82fb2454
	if !ctx.cr[6].gt {
	pc = 0x82FB2454; continue 'dispatch;
	}
	// 82FB23EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FB23F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB23F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FB23F8: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB23FC: 4800E09D  bl 0x82fc0498
	ctx.lr = 0x82FB2400;
	sub_82FC0498(ctx, base);
	// 82FB2400: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB2404: 7C7E502E  lwzx r3, r30, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FB2408: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB240C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FB2410: 419A0030  beq cr6, 0x82fb2440
	if ctx.cr[6].eq {
	pc = 0x82FB2440; continue 'dispatch;
	}
	// 82FB2414: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FB2418: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82FB241C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82FB2420: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82FB2424: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB2428: 409A0018  bne cr6, 0x82fb2440
	if !ctx.cr[6].eq {
	pc = 0x82FB2440; continue 'dispatch;
	}
	// 82FB242C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB2430: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FB2434: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB2438: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FB243C: 4E800421  bctrl
	ctx.lr = 0x82FB2440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB2440: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB2444: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FB2448: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FB244C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB2450: 4198FFA0  blt cr6, 0x82fb23f0
	if ctx.cr[6].lt {
	pc = 0x82FB23F0; continue 'dispatch;
	}
	// 82FB2454: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FB2458: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FB245C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB2460: 409A0020  bne cr6, 0x82fb2480
	if !ctx.cr[6].eq {
	pc = 0x82FB2480; continue 'dispatch;
	}
	// 82FB2464: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB2468: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FB246C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FB2470: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB2474: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FB2478: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FB247C: 4BEEE335  bl 0x82ea07b0
	ctx.lr = 0x82FB2480;
	sub_82EA07B0(ctx, base);
	// 82FB2480: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB2484: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FB2488: 392B5F34  addi r9, r11, 0x5f34
	ctx.r[9].s64 = ctx.r[11].s64 + 24372;
	// 82FB248C: 390A9EAC  addi r8, r10, -0x6154
	ctx.r[8].s64 = ctx.r[10].s64 + -24916;
	// 82FB2490: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB2494: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB2498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FB249C: 481F5D1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB24A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB24A0 size=140
    let mut pc: u32 = 0x82FB24A0;
    'dispatch: loop {
        match pc {
            0x82FB24A0 => {
    //   block [0x82FB24A0..0x82FB252C)
	// 82FB24A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB24A4: 481F5CC9  bl 0x831a816c
	ctx.lr = 0x82FB24A8;
	sub_831A8130(ctx, base);
	// 82FB24A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB24AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FB24B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB24B4: 3BFD0010  addi r31, r29, 0x10
	ctx.r[31].s64 = ctx.r[29].s64 + 16;
	// 82FB24B8: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FB24BC: 815D0014  lwz r10, 0x14(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB24C0: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB24C4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FB24C8: 409A0010  bne cr6, 0x82fb24d8
	if !ctx.cr[6].eq {
	pc = 0x82FB24D8; continue 'dispatch;
	}
	// 82FB24CC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FB24D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB24D4: 4BEF43AD  bl 0x82ea6880
	ctx.lr = 0x82FB24D8;
	sub_82EA6880(ctx, base);
	// 82FB24D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB24DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB24E0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FB24E4: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FB24E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB24EC: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82FB24F0: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FB24F4: A0FE0004  lhz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB24F8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82FB24FC: 419A0010  beq cr6, 0x82fb250c
	if ctx.cr[6].eq {
	pc = 0x82FB250C; continue 'dispatch;
	}
	// 82FB2500: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FB2504: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FB2508: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FB250C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FB2510: 389D0008  addi r4, r29, 8
	ctx.r[4].s64 = ctx.r[29].s64 + 8;
	// 82FB2514: 409A0008  bne cr6, 0x82fb251c
	if !ctx.cr[6].eq {
	pc = 0x82FB251C; continue 'dispatch;
	}
	// 82FB2518: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FB251C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FB2520: 4800DFD1  bl 0x82fc04f0
	ctx.lr = 0x82FB2524;
	sub_82FC04F0(ctx, base);
	// 82FB2524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FB2528: 481F5C94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB2530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FB2530 size=2904
    let mut pc: u32 = 0x82FB2530;
    'dispatch: loop {
        match pc {
            0x82FB2530 => {
    //   block [0x82FB2530..0x82FB3088)
	// 82FB2530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB2534: 481F5BFD  bl 0x831a8130
	ctx.lr = 0x82FB2538;
	sub_831A8130(ctx, base);
	// 82FB2538: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82FB253C: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82FB2540: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82FB2544: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB2548: 824D0000  lwz r18, 0(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB254C: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82FB2550: 992101C7  stb r9, 0x1c7(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(455 as u32), ctx.r[9].u8 ) };
	// 82FB2554: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB2558: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FB255C: 90C101AC  stw r6, 0x1ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), ctx.r[6].u32 ) };
	// 82FB2560: 93C1019C  stw r30, 0x19c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), ctx.r[30].u32 ) };
	// 82FB2564: 7CB02B78  mr r16, r5
	ctx.r[16].u64 = ctx.r[5].u64;
	// 82FB2568: 7CEF3B78  mr r15, r7
	ctx.r[15].u64 = ctx.r[7].u64;
	// 82FB256C: 910101BC  stw r8, 0x1bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), ctx.r[8].u32 ) };
	// 82FB2570: 7D72582E  lwzx r11, r18, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB2574: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB2578: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB257C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB2580: 4098002C  bge cr6, 0x82fb25ac
	if !ctx.cr[6].lt {
	pc = 0x82FB25AC; continue 'dispatch;
	}
	// 82FB2584: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FB2588: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FB258C: 38E95FC0  addi r7, r9, 0x5fc0
	ctx.r[7].s64 = ctx.r[9].s64 + 24512;
	// 82FB2590: 38C8B438  addi r6, r8, -0x4bc8
	ctx.r[6].s64 = ctx.r[8].s64 + -19400;
	// 82FB2594: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FB2598: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82FB259C: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82FB25A0: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82FB25A4: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FB25A8: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FB25AC: 39C00014  li r14, 0x14
	ctx.r[14].s64 = 20;
	// 82FB25B0: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 82FB25B4: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 82FB25B8: 92210070  stw r17, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[17].u32 ) };
	// 82FB25BC: 39700004  addi r11, r16, 4
	ctx.r[11].s64 = ctx.r[16].s64 + 4;
	// 82FB25C0: 92210074  stw r17, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[17].u32 ) };
	// 82FB25C4: 7C72702E  lwzx r3, r18, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82FB25C8: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB25CC: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82FB25D0: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FB25D4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB25D8: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FB25DC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB25E0: 4199000C  bgt cr6, 0x82fb25ec
	if ctx.cr[6].gt {
	pc = 0x82FB25EC; continue 'dispatch;
	}
	// 82FB25E4: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FB25E8: 48000018  b 0x82fb2600
	pc = 0x82FB2600; continue 'dispatch;
	// 82FB25EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB25F0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB25F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FB25F8: 4E800421  bctrl
	ctx.lr = 0x82FB25FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB25FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB2600: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FB2604: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82FB2608: 7E0BFB78  or r11, r16, r31
	ctx.r[11].u64 = ctx.r[16].u64 | ctx.r[31].u64;
	// 82FB260C: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82FB2610: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB2614: 7F0B8000  cmpw cr6, r11, r16
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[16].s32, &mut ctx.xer);
	// 82FB2618: 40980024  bge cr6, 0x82fb263c
	if !ctx.cr[6].lt {
	pc = 0x82FB263C; continue 'dispatch;
	}
	// 82FB261C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB2620: 7F105800  cmpw cr6, r16, r11
	ctx.cr[6].compare_i32(ctx.r[16].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB2624: 41980008  blt cr6, 0x82fb262c
	if ctx.cr[6].lt {
	pc = 0x82FB262C; continue 'dispatch;
	}
	// 82FB2628: 7E0B8378  mr r11, r16
	ctx.r[11].u64 = ctx.r[16].u64;
	// 82FB262C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FB2630: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FB2634: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FB2638: 4BEF41C1  bl 0x82ea67f8
	ctx.lr = 0x82FB263C;
	sub_82EA67F8(ctx, base);
	// 82FB263C: 92210084  stw r17, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[17].u32 ) };
	// 82FB2640: 396F0004  addi r11, r15, 4
	ctx.r[11].s64 = ctx.r[15].s64 + 4;
	// 82FB2644: 7C72702E  lwzx r3, r18, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82FB2648: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 82FB264C: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB2650: 92210080  stw r17, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[17].u32 ) };
	// 82FB2654: 92010074  stw r16, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[16].u32 ) };
	// 82FB2658: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FB265C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB2660: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FB2664: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB2668: 4199000C  bgt cr6, 0x82fb2674
	if ctx.cr[6].gt {
	pc = 0x82FB2674; continue 'dispatch;
	}
	// 82FB266C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FB2670: 48000018  b 0x82fb2688
	pc = 0x82FB2688; continue 'dispatch;
	// 82FB2674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB2678: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB267C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FB2680: 4E800421  bctrl
	ctx.lr = 0x82FB2684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB2684: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB2688: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82FB268C: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82FB2690: 7DEBFB78  or r11, r15, r31
	ctx.r[11].u64 = ctx.r[15].u64 | ctx.r[31].u64;
	// 82FB2694: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82FB2698: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB269C: 7F0B7800  cmpw cr6, r11, r15
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[15].s32, &mut ctx.xer);
	// 82FB26A0: 40980024  bge cr6, 0x82fb26c4
	if !ctx.cr[6].lt {
	pc = 0x82FB26C4; continue 'dispatch;
	}
	// 82FB26A4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB26A8: 7F0F5800  cmpw cr6, r15, r11
	ctx.cr[6].compare_i32(ctx.r[15].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB26AC: 41980008  blt cr6, 0x82fb26b4
	if ctx.cr[6].lt {
	pc = 0x82FB26B4; continue 'dispatch;
	}
	// 82FB26B0: 7DEB7B78  mr r11, r15
	ctx.r[11].u64 = ctx.r[15].u64;
	// 82FB26B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FB26B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FB26BC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82FB26C0: 4BEF4139  bl 0x82ea67f8
	ctx.lr = 0x82FB26C4;
	sub_82EA67F8(ctx, base);
	// 82FB26C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FB26C8: 91E10084  stw r15, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[15].u32 ) };
	// 82FB26CC: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 82FB26D0: C3AB08A4  lfs f29, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82FB26D4: 419A0054  beq cr6, 0x82fb2728
	if ctx.cr[6].eq {
	pc = 0x82FB2728; continue 'dispatch;
	}
	// 82FB26D8: 7E298B78  mr r9, r17
	ctx.r[9].u64 = ctx.r[17].u64;
	// 82FB26DC: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82FB26E0: 7E0A8378  mr r10, r16
	ctx.r[10].u64 = ctx.r[16].u64;
	// 82FB26E4: 390BFFE0  addi r8, r11, -0x20
	ctx.r[8].s64 = ctx.r[11].s64 + -32;
	// 82FB26E8: 80C10070  lwz r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FB26EC: 38EBFFF0  addi r7, r11, -0x10
	ctx.r[7].s64 = ctx.r[11].s64 + -16;
	// 82FB26F0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB3088 size=28
    let mut pc: u32 = 0x82FB3088;
    'dispatch: loop {
        match pc {
            0x82FB3088 => {
    //   block [0x82FB3088..0x82FB30A4)
	// 82FB3088: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FB308C: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82FB3090: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82FB3094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FB3098: 80EB0020  lwz r7, 0x20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB309C: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FB30A0: 4BFFF490  b 0x82fb2530
	sub_82FB2530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB30A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB30A8 size=8
    let mut pc: u32 = 0x82FB30A8;
    'dispatch: loop {
        match pc {
            0x82FB30A8 => {
    //   block [0x82FB30A8..0x82FB30B0)
	// 82FB30A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FB30AC: 4BFFF484  b 0x82fb2530
	sub_82FB2530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB30B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FB30B0 size=3472
    let mut pc: u32 = 0x82FB30B0;
    'dispatch: loop {
        match pc {
            0x82FB30B0 => {
    //   block [0x82FB30B0..0x82FB3E40)
	// 82FB30B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB30B4: 481F507D  bl 0x831a8130
	ctx.lr = 0x82FB30B8;
	sub_831A8130(ctx, base);
	// 82FB30B8: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82FB30BC: 481F59BD  bl 0x831a8a78
	ctx.lr = 0x82FB30C0;
	sub_831A8A40(ctx, base);
	// 82FB30C0: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB30C4: 91210234  stw r9, 0x234(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(564 as u32), ctx.r[9].u32 ) };
	// 82FB30C8: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82FB30CC: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB30D0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FB30D4: 9101022C  stw r8, 0x22c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(556 as u32), ctx.r[8].u32 ) };
	// 82FB30D8: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82FB30DC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FB30E0: 90610204  stw r3, 0x204(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(516 as u32), ctx.r[3].u32 ) };
	// 82FB30E4: 90A10214  stw r5, 0x214(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(532 as u32), ctx.r[5].u32 ) };
	// 82FB30E8: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82FB30EC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FB30F0: 93C1021C  stw r30, 0x21c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(540 as u32), ctx.r[30].u32 ) };
	// 82FB30F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB30F8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB30FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB3100: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FB3104: 4098002C  bge cr6, 0x82fb3130
	if !ctx.cr[6].lt {
	pc = 0x82FB3130; continue 'dispatch;
	}
	// 82FB3108: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FB310C: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82FB3110: 38C85FCC  addi r6, r8, 0x5fcc
	ctx.r[6].s64 = ctx.r[8].s64 + 24524;
	// 82FB3114: 38A7B438  addi r5, r7, -0x4bc8
	ctx.r[5].s64 = ctx.r[7].s64 + -19400;
	// 82FB3118: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FB311C: 90AA000C  stw r5, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82FB3120: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82FB3124: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 82FB3128: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FB312C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FB3130: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82FB3134: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FB3138: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 82FB313C: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 82FB3140: 7EA95A14  add r21, r9, r11
	ctx.r[21].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FB3144: 92E10064  stw r23, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[23].u32 ) };
	// 82FB3148: 39590004  addi r10, r25, 4
	ctx.r[10].s64 = ctx.r[25].s64 + 4;
	// 82FB314C: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB3150: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82FB3154: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB3158: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FB315C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB3160: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FB3164: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB3168: 4199000C  bgt cr6, 0x82fb3174
	if ctx.cr[6].gt {
	pc = 0x82FB3174; continue 'dispatch;
	}
	// 82FB316C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FB3170: 48000018  b 0x82fb3188
	pc = 0x82FB3188; continue 'dispatch;
	// 82FB3174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB3178: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB317C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FB3180: 4E800421  bctrl
	ctx.lr = 0x82FB3184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB3184: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB3188: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FB318C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FB3190: 7F2BFB78  or r11, r25, r31
	ctx.r[11].u64 = ctx.r[25].u64 | ctx.r[31].u64;
	// 82FB3194: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82FB3198: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FB319C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB31A0: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FB31A4: 40980024  bge cr6, 0x82fb31c8
	if !ctx.cr[6].lt {
	pc = 0x82FB31C8; continue 'dispatch;
	}
	// 82FB31A8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB31AC: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB31B0: 41980008  blt cr6, 0x82fb31b8
	if ctx.cr[6].lt {
	pc = 0x82FB31B8; continue 'dispatch;
	}
	// 82FB31B4: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82FB31B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FB31BC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FB31C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FB31C4: 4BEF3635  bl 0x82ea67f8
	ctx.lr = 0x82FB31C8;
	sub_82EA67F8(ctx, base);
	// 82FB31C8: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82FB31CC: 39760004  addi r11, r22, 4
	ctx.r[11].s64 = ctx.r[22].s64 + 4;
	// 82FB31D0: 92E10070  stw r23, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[23].u32 ) };
	// 82FB31D4: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82FB31D8: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB31DC: 80750000  lwz r3, 0(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB31E0: 92E10074  stw r23, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[23].u32 ) };
	// 82FB31E4: 908100A8  stw r4, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[4].u32 ) };
	// 82FB31E8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FB31EC: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FB31F0: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FB31F4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB31F8: 4199000C  bgt cr6, 0x82fb3204
	if ctx.cr[6].gt {
	pc = 0x82FB3204; continue 'dispatch;
	}
	// 82FB31FC: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82FB3200: 48000018  b 0x82fb3218
	pc = 0x82FB3218; continue 'dispatch;
	// 82FB3204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB3208: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FB320C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FB3210: 4E800421  bctrl
	ctx.lr = 0x82FB3214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB3214: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FB3218: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82FB321C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FB3220: 7ECBFB78  or r11, r22, r31
	ctx.r[11].u64 = ctx.r[22].u64 | ctx.r[31].u64;
	// 82FB3224: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82FB3228: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82FB322C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82FB3230: 7F0BB000  cmpw cr6, r11, r22
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82FB3234: 40980024  bge cr6, 0x82fb3258
	if !ctx.cr[6].lt {
	pc = 0x82FB3258; continue 'dispatch;
	}
	// 82FB3238: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB323C: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB3240: 41980008  blt cr6, 0x82fb3248
	if ctx.cr[6].lt {
	pc = 0x82FB3248; continue 'dispatch;
	}
	// 82FB3244: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82FB3248: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FB324C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FB3250: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FB3254: 4BEF35A5  bl 0x82ea67f8
	ctx.lr = 0x82FB3258;
	sub_82EA67F8(ctx, base);
	// 82FB3258: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FB325C: 92C10074  stw r22, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[22].u32 ) };
	// 82FB3260: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82FB3264: C3AB08A4  lfs f29, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82FB3268: 419A0054  beq cr6, 0x82fb32bc
	if ctx.cr[6].eq {
	pc = 0x82FB32BC; continue 'dispatch;
	}
	// 82FB326C: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82FB3270: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82FB3274: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82FB3278: 390BFFE0  addi r8, r11, -0x20
	ctx.r[8].s64 = ctx.r[11].s64 + -32;
	// 82FB327C: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FB3280: 38EBFFF0  addi r7, r11, -0x10
	ctx.r[7].s64 = ctx.r[11].s64 + -16;
	// 82FB3284: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB3E40 size=64
    let mut pc: u32 = 0x82FB3E40;
    'dispatch: loop {
        match pc {
            0x82FB3E40 => {
    //   block [0x82FB3E40..0x82FB3E80)
	// 82FB3E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB3E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB3E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB3E4C: B0A10086  sth r5, 0x86(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[5].u16 ) };
	// 82FB3E50: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82FB3E54: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82FB3E58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FB3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB3E60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FB3E64: 38A10086  addi r5, r1, 0x86
	ctx.r[5].s64 = ctx.r[1].s64 + 134;
	// 82FB3E68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FB3E6C: 4BFFF245  bl 0x82fb30b0
	ctx.lr = 0x82FB3E70;
	sub_82FB30B0(ctx, base);
	// 82FB3E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB3E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB3E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB3E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB3E80 size=64
    let mut pc: u32 = 0x82FB3E80;
    'dispatch: loop {
        match pc {
            0x82FB3E80 => {
    //   block [0x82FB3E80..0x82FB3EC0)
	// 82FB3E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB3E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB3E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB3E8C: B0A10086  sth r5, 0x86(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[5].u16 ) };
	// 82FB3E90: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82FB3E94: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82FB3E98: 39010086  addi r8, r1, 0x86
	ctx.r[8].s64 = ctx.r[1].s64 + 134;
	// 82FB3E9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82FB3EA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FB3EA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FB3EA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FB3EAC: 4BFFF205  bl 0x82fb30b0
	ctx.lr = 0x82FB3EB0;
	sub_82FB30B0(ctx, base);
	// 82FB3EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB3EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB3EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB3EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB3EC0 size=32
    let mut pc: u32 = 0x82FB3EC0;
    'dispatch: loop {
        match pc {
            0x82FB3EC0 => {
    //   block [0x82FB3EC0..0x82FB3EE0)
	// 82FB3EC0: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82FB3EC4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82FB3EC8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82FB3ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FB3ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB3ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FB3ED8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FB3EDC: 4BFFF1D4  b 0x82fb30b0
	sub_82FB30B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB3EE0 size=32
    let mut pc: u32 = 0x82FB3EE0;
    'dispatch: loop {
        match pc {
            0x82FB3EE0 => {
    //   block [0x82FB3EE0..0x82FB3F00)
	// 82FB3EE0: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82FB3EE4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82FB3EE8: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82FB3EEC: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82FB3EF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FB3EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FB3EF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FB3EFC: 4BFFF1B4  b 0x82fb30b0
	sub_82FB30B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB3F00 size=8
    let mut pc: u32 = 0x82FB3F00;
    'dispatch: loop {
        match pc {
            0x82FB3F00 => {
    //   block [0x82FB3F00..0x82FB3F08)
	// 82FB3F00: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FB3F04: 48000004  b 0x82fb3f08
	sub_82FB3F08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB3F08 size=100
    let mut pc: u32 = 0x82FB3F08;
    'dispatch: loop {
        match pc {
            0x82FB3F08 => {
    //   block [0x82FB3F08..0x82FB3F6C)
	// 82FB3F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB3F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB3F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FB3F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB3F18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB3F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB3F20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB3F24: 4BFFE48D  bl 0x82fb23b0
	ctx.lr = 0x82FB3F28;
	sub_82FB23B0(ctx, base);
	// 82FB3F28: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FB3F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB3F30: 419A0020  beq cr6, 0x82fb3f50
	if ctx.cr[6].eq {
	pc = 0x82FB3F50; continue 'dispatch;
	}
	// 82FB3F34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB3F38: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FB3F3C: 38C0003D  li r6, 0x3d
	ctx.r[6].s64 = 61;
	// 82FB3F40: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB3F44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FB3F48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB3F4C: 4BEEC865  bl 0x82ea07b0
	ctx.lr = 0x82FB3F50;
	sub_82EA07B0(ctx, base);
	// 82FB3F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB3F54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FB3F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB3F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB3F60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FB3F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB3F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB3F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB3F70 size=1436
    let mut pc: u32 = 0x82FB3F70;
    'dispatch: loop {
        match pc {
            0x82FB3F70 => {
    //   block [0x82FB3F70..0x82FB450C)
	// 82FB3F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB3F74: 481F41ED  bl 0x831a8160
	ctx.lr = 0x82FB3F78;
	sub_831A8130(ctx, base);
	// 82FB3F78: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 82FB3F7C: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82FB3F80: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82FB3F84: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB4510 size=32
    let mut pc: u32 = 0x82FB4510;
    'dispatch: loop {
        match pc {
            0x82FB4510 => {
    //   block [0x82FB4510..0x82FB4530)
	// 82FB4510: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB4514: D023000C  stfs f1, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FB4518: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FB451C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82FB4520: 392B5FE8  addi r9, r11, 0x5fe8
	ctx.r[9].s64 = ctx.r[11].s64 + 24552;
	// 82FB4524: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82FB4528: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB4530 size=1028
    let mut pc: u32 = 0x82FB4530;
    'dispatch: loop {
        match pc {
            0x82FB4530 => {
    //   block [0x82FB4530..0x82FB4934)
	// 82FB4530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB4534: 481F3BFD  bl 0x831a8130
	ctx.lr = 0x82FB4538;
	sub_831A8130(ctx, base);
	// 82FB4538: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82FB453C: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB4938 size=316
    let mut pc: u32 = 0x82FB4938;
    'dispatch: loop {
        match pc {
            0x82FB4938 => {
    //   block [0x82FB4938..0x82FB4A74)
	// 82FB4938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB493C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB4940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB4A78 size=96
    let mut pc: u32 = 0x82FB4A78;
    'dispatch: loop {
        match pc {
            0x82FB4A78 => {
    //   block [0x82FB4A78..0x82FB4AD8)
	// 82FB4A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB4A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB4A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB4A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB4A88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB4A8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FB4A90: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82FB4A94: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82FB4A98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FB4A9C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB4AA0: 419A0020  beq cr6, 0x82fb4ac0
	if ctx.cr[6].eq {
	pc = 0x82FB4AC0; continue 'dispatch;
	}
	// 82FB4AA4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4AA8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FB4AAC: 38C0003D  li r6, 0x3d
	ctx.r[6].s64 = 61;
	// 82FB4AB0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB4AB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FB4AB8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB4ABC: 4BEEBCF5  bl 0x82ea07b0
	ctx.lr = 0x82FB4AC0;
	sub_82EA07B0(ctx, base);
	// 82FB4AC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB4AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB4AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB4ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB4AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB4AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB4AD8 size=64
    let mut pc: u32 = 0x82FB4AD8;
    'dispatch: loop {
        match pc {
            0x82FB4AD8 => {
    //   block [0x82FB4AD8..0x82FB4B18)
	// 82FB4AD8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FB4ADC: C0030080  lfs f0, 0x80(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4AE0: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4AE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82FB4AE8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4AEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB4AF0: C18A08A4  lfs f12, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FB4AF4: C169000C  lfs f11, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB4AF8: ED4B0028  fsubs f10, f11, f0
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82FB4AFC: EC0A6828  fsubs f0, f10, f13
	ctx.f[0].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FB4B00: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB4B04: 41990014  bgt cr6, 0x82fb4b18
	if ctx.cr[6].gt {
		sub_82FB4B18(ctx, base);
		return;
	}
	// 82FB4B08: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4B0C: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4B10: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FB4B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB4B18 size=128
    let mut pc: u32 = 0x82FB4B18;
    'dispatch: loop {
        match pc {
            0x82FB4B18 => {
    //   block [0x82FB4B18..0x82FB4B98)
	// 82FB4B18: C1630008  lfs f11, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB4B1C: ED4B6828  fsubs f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FB4B20: C1230040  lfs f9, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FB4B24: ED09507A  fmadds f8, f9, f1, f10
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[1].f64 + ctx.f[10].f64) as f32) as f64);
	// 82FB4B28: D1050000  stfs f8, 0(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4B2C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FB4B30: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4B34: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB4B38: 40990070  ble cr6, 0x82fb4ba8
	if !ctx.cr[6].gt {
		sub_82FB4BA8(ctx, base);
		return;
	}
	// 82FB4B3C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82FB4B40: FD80681E  fctiwz f12, f13
	ctx.f[12].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82FB4B44: D981FFF0  stfd f12, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[12].u64 ) };
	// 82FB4B48: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82FB4B4C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FB4B50: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FB4B54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB4B58: 40990040  ble cr6, 0x82fb4b98
	if !ctx.cr[6].gt {
		sub_82FB4B98(ctx, base);
		return;
	}
	// 82FB4B5C: 81230044  lwz r9, 0x44(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FB4B60: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FB4B64: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FB4B68: 41980030  blt cr6, 0x82fb4b98
	if ctx.cr[6].lt {
		sub_82FB4B98(ctx, base);
		return;
	}
	// 82FB4B6C: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4B70: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FB4B74: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FB4B78: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FB4B7C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82FB4B80: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB4B84: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4B88: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4B8C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82FB4B90: D1850000  stfs f12, 0(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB4B98 size=16
    let mut pc: u32 = 0x82FB4B98;
    'dispatch: loop {
        match pc {
            0x82FB4B98 => {
    //   block [0x82FB4B98..0x82FB4BA8)
	// 82FB4B98: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82FB4B9C: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82FB4BA0: C981FFF0  lfd f12, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB4BA4: 48000078  b 0x82fb4c1c
	sub_82FB4C10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB4BA8 size=104
    let mut pc: u32 = 0x82FB4BA8;
    'dispatch: loop {
        match pc {
            0x82FB4BA8 => {
    //   block [0x82FB4BA8..0x82FB4C10)
	// 82FB4BA8: FF0D6000  fcmpu cr6, f13, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82FB4BAC: 40980084  bge cr6, 0x82fb4c30
	if !ctx.cr[6].lt {
		sub_82FB4C10(ctx, base);
		return;
	}
	// 82FB4BB0: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82FB4BB4: FD60681E  fctiwz f11, f13
	ctx.f[11].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82FB4BB8: D961FFF0  stfd f11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[11].u64 ) };
	// 82FB4BBC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82FB4BC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FB4BC4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FB4BC8: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FB4BCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB4BD0: 40990040  ble cr6, 0x82fb4c10
	if !ctx.cr[6].gt {
		sub_82FB4C10(ctx, base);
		return;
	}
	// 82FB4BD4: 81230048  lwz r9, 0x48(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FB4BD8: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82FB4BDC: 7CEB4850  subf r7, r11, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82FB4BE0: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FB4BE4: 4198002C  blt cr6, 0x82fb4c10
	if ctx.cr[6].lt {
		sub_82FB4C10(ctx, base);
		return;
	}
	// 82FB4BE8: D1850000  stfs f12, 0(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4BEC: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FB4BF0: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FB4BF4: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FB4BF8: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB4BFC: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4C00: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4C04: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82FB4C08: D1850000  stfs f12, 0(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FB4C10 size=52
    let mut pc: u32 = 0x82FB4C10;
    'dispatch: loop {
        match pc {
            0x82FB4C10 => {
    //   block [0x82FB4C10..0x82FB4C44)
	// 82FB4C10: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82FB4C14: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82FB4C18: C981FFF0  lfd f12, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB4C1C: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 82FB4C20: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4C24: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82FB4C28: ED2A683C  fnmsubs f9, f10, f0, f13
	ctx.f[9].f64 = -(((ctx.f[10].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FB4C2C: D1250000  stfs f9, 0(r5)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4C30: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4C34: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4C38: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82FB4C3C: D1850000  stfs f12, 0(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FB4C48 size=844
    let mut pc: u32 = 0x82FB4C48;
    'dispatch: loop {
        match pc {
            0x82FB4C48 => {
    //   block [0x82FB4C48..0x82FB4F94)
	// 82FB4C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB4C4C: 481F3519  bl 0x831a8164
	ctx.lr = 0x82FB4C50;
	sub_831A8130(ctx, base);
	// 82FB4C50: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82FB4C54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB4C58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB4C5C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82FB4C60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82FB4C64: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82FB4C68: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FB4C6C: C00ABA78  lfs f0, -0x4588(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4C70: C18908A4  lfs f12, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FB4C74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB4C78: 419A002C  beq cr6, 0x82fb4ca4
	if ctx.cr[6].eq {
	pc = 0x82FB4CA4; continue 'dispatch;
	}
	// 82FB4C7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB4C84: 419A0020  beq cr6, 0x82fb4ca4
	if ctx.cr[6].eq {
	pc = 0x82FB4CA4; continue 'dispatch;
	}
	// 82FB4C88: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4C8C: C1BF0080  lfs f13, 0x80(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4C90: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FB4C94: C15F007C  lfs f10, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FB4C98: ED2B5028  fsubs f9, f11, f10
	ctx.f[9].f64 = (((ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 82FB4C9C: FD004850  fneg f8, f9
	ctx.f[8].u64 = ctx.f[9].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FB4CA0: FC084B2E  fsel f0, f8, f12, f9
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[9].f64 };
	// 82FB4CA4: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4CA8: C17F007C  lfs f11, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB4CAC: ED4D5828  fsubs f10, f13, f11
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 82FB4CB0: D15F0008  stfs f10, 8(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4CB4: FF0A6000  fcmpu cr6, f10, f12
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[12].f64);
	// 82FB4CB8: 40980008  bge cr6, 0x82fb4cc0
	if !ctx.cr[6].lt {
	pc = 0x82FB4CC0; continue 'dispatch;
	}
	// 82FB4CBC: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4CC0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82FB4CC4: 419A02B4  beq cr6, 0x82fb4f78
	if ctx.cr[6].eq {
	pc = 0x82FB4F78; continue 'dispatch;
	}
	// 82FB4CC8: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4CCC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FB4CD0: C17F0040  lfs f11, 0x40(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB4CD4: EDAB6FFA  fmadds f13, f11, f31, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 82FB4CD8: D1BF0008  stfs f13, 8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4CDC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB4CE0: 40990048  ble cr6, 0x82fb4d28
	if !ctx.cr[6].gt {
	pc = 0x82FB4D28; continue 'dispatch;
	}
	// 82FB4CE4: ED6D0024  fdivs f11, f13, f0
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82FB4CE8: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FB4CEC: FD40581E  fctiwz f10, f11
	ctx.f[10].s64 = if ctx.f[11].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[11].f64.trunc() as i32 as i64 };
	// 82FB4CF0: D9410050  stfd f10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[10].u64 ) };
	// 82FB4CF4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FB4CF8: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 82FB4CFC: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82FB4D00: C9210050  lfd f9, 0x50(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FB4D04: FD004E9C  fcfid f8, f9
	ctx.f[8].f64 = (ctx.f[9].s64 as f64);
	// 82FB4D08: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FB4D0C: FCE04018  frsp f7, f8
	ctx.f[7].f64 = (ctx.f[8].f64 as f32) as f64;
	// 82FB4D10: 911F0044  stw r8, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[8].u32 ) };
	// 82FB4D14: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82FB4D18: ECC7683C  fnmsubs f6, f7, f0, f13
	ctx.f[6].f64 = -(((ctx.f[7].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82FB4D1C: FCA661AE  fsel f5, f6, f6, f12
	ctx.f[5].f64 = if ctx.f[6].f64 >= 0.0 { ctx.f[6].f64 } else { ctx.f[12].f64 };
	// 82FB4D20: D0BF0008  stfs f5, 8(r31)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4D24: 4800005C  b 0x82fb4d80
	pc = 0x82FB4D80; continue 'dispatch;
	// 82FB4D28: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82FB4D2C: 40980054  bge cr6, 0x82fb4d80
	if !ctx.cr[6].lt {
	pc = 0x82FB4D80; continue 'dispatch;
	}
	// 82FB4D30: ED6D0024  fdivs f11, f13, f0
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82FB4D34: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FB4D38: FD405850  fneg f10, f11
	ctx.f[10].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 82FB4D3C: FD20501E  fctiwz f9, f10
	ctx.f[9].s64 = if ctx.f[10].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[10].f64.trunc() as i32 as i64 };
	// 82FB4D40: D9210050  stfd f9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[9].u64 ) };
	// 82FB4D44: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FB4D48: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FB4D4C: 7D2807B4  extsw r8, r9
	ctx.r[8].s64 = ctx.r[9].s32 as i64;
	// 82FB4D50: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FB4D54: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82FB4D58: C9010050  lfd f8, 0x50(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FB4D5C: FCE0469C  fcfid f7, f8
	ctx.f[7].f64 = (ctx.f[8].s64 as f64);
	// 82FB4D60: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82FB4D64: FCC03818  frsp f6, f7
	ctx.f[6].f64 = (ctx.f[7].f64 as f32) as f64;
	// 82FB4D68: 90FF0048  stw r7, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[7].u32 ) };
	// 82FB4D6C: 238BFFFF  subfic r28, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[28].s64 = (-1 as i64) - ctx.r[11].s64;
	// 82FB4D70: ECA6683A  fmadds f5, f6, f0, f13
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82FB4D74: EC850028  fsubs f4, f5, f0
	ctx.f[4].f64 = (((ctx.f[5].f64 - ctx.f[0].f64) as f32) as f64);
	// 82FB4D78: FC64282E  fsel f3, f4, f0, f5
	ctx.f[3].f64 = if ctx.f[4].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[5].f64 };
	// 82FB4D7C: D07F0008  stfs f3, 8(r31)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4D80: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FB4D84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB4D88: 40990030  ble cr6, 0x82fb4db8
	if !ctx.cr[6].gt {
	pc = 0x82FB4DB8; continue 'dispatch;
	}
	// 82FB4D8C: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FB4D90: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FB4D94: 4198000C  blt cr6, 0x82fb4da0
	if ctx.cr[6].lt {
	pc = 0x82FB4DA0; continue 'dispatch;
	}
	// 82FB4D98: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4D9C: 48000018  b 0x82fb4db4
	pc = 0x82FB4DB4; continue 'dispatch;
	// 82FB4DA0: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FB4DA4: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82FB4DA8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FB4DAC: 4198000C  blt cr6, 0x82fb4db8
	if ctx.cr[6].lt {
	pc = 0x82FB4DB8; continue 'dispatch;
	}
	// 82FB4DB0: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4DB4: D19F0040  stfs f12, 0x40(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82FB4DB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82FB4DBC: C1BF0074  lfs f13, 0x74(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4DC0: 395F0074  addi r10, r31, 0x74
	ctx.r[10].s64 = ctx.r[31].s64 + 116;
	// 82FB4DC4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FB4DC8: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4DCC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82FB4DD0: 4098003C  bge cr6, 0x82fb4e0c
	if !ctx.cr[6].lt {
	pc = 0x82FB4E0C; continue 'dispatch;
	}
	// 82FB4DD4: FD80FA10  fabs f12, f31
	ctx.f[12].u64 = ctx.f[31].u64 & !0x8000_0000_0000_0000u64;
	// 82FB4DD8: C17F0070  lfs f11, 0x70(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB4DDC: ED4C6AFA  fmadds f10, f12, f11, f13
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64);
	// 82FB4DE0: D14A0000  stfs f10, 0(r10)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4DE4: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 82FB4DE8: 41980024  blt cr6, 0x82fb4e0c
	if ctx.cr[6].lt {
	pc = 0x82FB4E0C; continue 'dispatch;
	}
	// 82FB4DEC: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FB4DF0: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82FB4DF4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FB4DF8: 212B0000  subfic r9, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[9].s64 = (0 as i64) - ctx.r[11].s64;
	// 82FB4DFC: 7D094910  subfe r8, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FB4E00: 550B07BC  rlwinm r11, r8, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82FB4E04: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82FB4E08: 90FF0078  stw r7, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 82FB4E0C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FB4E10: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FB4E14: 419A0010  beq cr6, 0x82fb4e24
	if ctx.cr[6].eq {
	pc = 0x82FB4E24; continue 'dispatch;
	}
	// 82FB4E18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB4E1C: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 82FB4E20: 409A0008  bne cr6, 0x82fb4e28
	if !ctx.cr[6].eq {
	pc = 0x82FB4E28; continue 'dispatch;
	}
	// 82FB4E24: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82FB4E28: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82FB4E2C: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82FB4E30: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82FB4E34: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82FB4E38: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82FB4E3C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FB4E40: C12A0000  lfs f9, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82FB4E44: ED090272  fmuls f8, f9, f9
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[9].f64) as f32) as f64);
	// 82FB4E48: C0EB000C  lfs f7, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82FB4E4C: C009A1C4  lfs f0, -0x5e3c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4E50: ECCC0032  fmuls f6, f12, f0
	ctx.f[6].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FB4E54: C1A8A2EC  lfs f13, -0x5d14(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-23828 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4E58: ECAB0032  fmuls f5, f11, f0
	ctx.f[5].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FB4E5C: C09F003C  lfs f4, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82FB4E60: EC6A0032  fmuls f3, f10, f0
	ctx.f[3].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82FB4E64: EC480272  fmuls f2, f8, f9
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 82FB4E68: EC2B337C  fnmsubs f1, f11, f13, f6
	ctx.f[1].f64 = -(((ctx.f[11].f64 * ctx.f[13].f64 - ctx.f[6].f64) as f32) as f64);
	// 82FB4E6C: EC056028  fsubs f0, f5, f12
	ctx.f[0].f64 = (((ctx.f[5].f64 - ctx.f[12].f64) as f32) as f64);
	// 82FB4E70: EDA53028  fsubs f13, f5, f6
	ctx.f[13].f64 = (((ctx.f[5].f64 - ctx.f[6].f64) as f32) as f64);
	// 82FB4E74: ED61182A  fadds f11, f1, f3
	ctx.f[11].f64 = ((ctx.f[1].f64 + ctx.f[3].f64) as f32) as f64;
	// 82FB4E78: ED401828  fsubs f10, f0, f3
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[3].f64) as f32) as f64);
	// 82FB4E7C: ED0B0232  fmuls f8, f11, f8
	ctx.f[8].f64 = (((ctx.f[11].f64 * ctx.f[8].f64) as f32) as f64);
	// 82FB4E80: ECEA382A  fadds f7, f10, f7
	ctx.f[7].f64 = ((ctx.f[10].f64 + ctx.f[7].f64) as f32) as f64;
	// 82FB4E84: ECC740BA  fmadds f6, f7, f2, f8
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[2].f64 + ctx.f[8].f64) as f32) as f64);
	// 82FB4E88: ECAD327A  fmadds f5, f13, f9, f6
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[9].f64 + ctx.f[6].f64) as f32) as f64);
	// 82FB4E8C: EC65602A  fadds f3, f5, f12
	ctx.f[3].f64 = ((ctx.f[5].f64 + ctx.f[12].f64) as f32) as f64;
	// 82FB4E90: EC430132  fmuls f2, f3, f4
	ctx.f[2].f64 = (((ctx.f[3].f64 * ctx.f[4].f64) as f32) as f64);
	// 82FB4E94: D05F000C  stfs f2, 0xc(r31)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82FB4E98: 419A0064  beq cr6, 0x82fb4efc
	if ctx.cr[6].eq {
	pc = 0x82FB4EFC; continue 'dispatch;
	}
	// 82FB4E9C: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FB4EA0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FB4EA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB4EA8: 40990054  ble cr6, 0x82fb4efc
	if !ctx.cr[6].gt {
	pc = 0x82FB4EFC; continue 'dispatch;
	}
	// 82FB4EAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FB4EB0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FB4EB4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FB4EB8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FB4EBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FB4EC0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB4EC4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4EC8: 40990010  ble cr6, 0x82fb4ed8
	if !ctx.cr[6].gt {
	pc = 0x82FB4ED8; continue 'dispatch;
	}
	// 82FB4ECC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4ED0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FB4ED4: 4800000C  b 0x82fb4ee0
	pc = 0x82FB4EE0; continue 'dispatch;
	// 82FB4ED8: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB4EDC: 7CDC00D0  neg r6, r28
	ctx.r[6].s64 = -ctx.r[28].s64;
	// 82FB4EE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82FB4EE4: 4E800421  bctrl
	ctx.lr = 0x82FB4EE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB4EE8: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FB4EEC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FB4EF0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FB4EF4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB4EF8: 4198FFB8  blt cr6, 0x82fb4eb0
	if ctx.cr[6].lt {
	pc = 0x82FB4EB0; continue 'dispatch;
	}
	// 82FB4EFC: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82FB4F00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB4F04: 419A0074  beq cr6, 0x82fb4f78
	if ctx.cr[6].eq {
	pc = 0x82FB4F78; continue 'dispatch;
	}
	// 82FB4F08: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FB4F0C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FB4F10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB4F14: 40990064  ble cr6, 0x82fb4f78
	if !ctx.cr[6].gt {
	pc = 0x82FB4F78; continue 'dispatch;
	}
	// 82FB4F18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FB4F1C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FB4F20: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FB4F24: 419A0020  beq cr6, 0x82fb4f44
	if ctx.cr[6].eq {
	pc = 0x82FB4F44; continue 'dispatch;
	}
	// 82FB4F28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB4F2C: 419A0018  beq cr6, 0x82fb4f44
	if ctx.cr[6].eq {
	pc = 0x82FB4F44; continue 'dispatch;
	}
	// 82FB4F30: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FB4F34: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB4F38: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4F3C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB4F40: 48000014  b 0x82fb4f54
	pc = 0x82FB4F54; continue 'dispatch;
	// 82FB4F44: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FB4F48: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB4F4C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB4F50: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB4F54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FB4F58: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82FB4F5C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82FB4F60: 4E800421  bctrl
	ctx.lr = 0x82FB4F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FB4F64: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FB4F68: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FB4F6C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FB4F70: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FB4F74: 4198FFA8  blt cr6, 0x82fb4f1c
	if ctx.cr[6].lt {
	pc = 0x82FB4F1C; continue 'dispatch;
	}
	// 82FB4F78: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB4F7C: C1BF007C  lfs f13, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB4F80: ED8D002A  fadds f12, f13, f0
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82FB4F84: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82FB4F88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FB4F8C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82FB4F90: 481F3224  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB4F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB4F98 size=216
    let mut pc: u32 = 0x82FB4F98;
    'dispatch: loop {
        match pc {
            0x82FB4F98 => {
    //   block [0x82FB4F98..0x82FB5070)
	// 82FB4F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB4F9C: 481F31CD  bl 0x831a8168
	ctx.lr = 0x82FB4FA0;
	sub_831A8130(ctx, base);
	// 82FB4FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB4FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB4FA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB4FAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FB4FB0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FB4FB4: 4800B5AD  bl 0x82fc0560
	ctx.lr = 0x82FB4FB8;
	sub_82FC0560(ctx, base);
	// 82FB4FB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FB4FBC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82FB4FC0: 390B2970  addi r8, r11, 0x2970
	ctx.r[8].s64 = ctx.r[11].s64 + 10608;
	// 82FB4FC4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FB4FC8: 38EA0070  addi r7, r10, 0x70
	ctx.r[7].s64 = ctx.r[10].s64 + 112;
	// 82FB4FCC: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82FB4FD0: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82FB4FD4: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FB5070 size=192
    let mut pc: u32 = 0x82FB5070;
    'dispatch: loop {
        match pc {
            0x82FB5070 => {
    //   block [0x82FB5070..0x82FB5130)
	// 82FB5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB5078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FB507C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB5080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB5084: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB5088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB508C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FB5090: 4800B4D1  bl 0x82fc0560
	ctx.lr = 0x82FB5094;
	sub_82FC0560(ctx, base);
	// 82FB5094: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB5098: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FB509C: 394B6008  addi r10, r11, 0x6008
	ctx.r[10].s64 = ctx.r[11].s64 + 24584;
	// 82FB50A0: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82FB50A4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FB50A8: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82FB50AC: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82FB50B0: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82FB50B4: 913F0088  stw r9, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 82FB50B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB50BC: 911F008C  stw r8, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[8].u32 ) };
	// 82FB50C0: C01E003C  lfs f0, 0x3c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82FB50C4: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82FB50C8: C1BE0040  lfs f13, 0x40(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82FB50CC: D1BF0040  stfs f13, 0x40(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82FB50D0: 80FE0044  lwz r7, 0x44(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FB50D4: 90FF0044  stw r7, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 82FB50D8: 80DE0048  lwz r6, 0x48(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FB50DC: 90DF0048  stw r6, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB5130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB5130 size=148
    let mut pc: u32 = 0x82FB5130;
    'dispatch: loop {
        match pc {
            0x82FB5130 => {
    //   block [0x82FB5130..0x82FB51C4)
	// 82FB5130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB5134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB5138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FB513C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB5140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB5144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB5148: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FB514C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FB5150: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FB5154: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB5158: 409A0020  bne cr6, 0x82fb5178
	if !ctx.cr[6].eq {
	pc = 0x82FB5178; continue 'dispatch;
	}
	// 82FB515C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB5160: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82FB5164: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82FB5168: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FB516C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FB5170: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FB5174: 4BEEB63D  bl 0x82ea07b0
	ctx.lr = 0x82FB5178;
	sub_82EA07B0(ctx, base);
	// 82FB5178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB517C: 4800B54D  bl 0x82fc06c8
	ctx.lr = 0x82FB5180;
	sub_82FC06C8(ctx, base);
	// 82FB5180: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82FB5184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB5188: 419A0020  beq cr6, 0x82fb51a8
	if ctx.cr[6].eq {
	pc = 0x82FB51A8; continue 'dispatch;
	}
	// 82FB518C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB5190: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82FB5194: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 82FB5198: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB519C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FB51A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FB51A4: 4BEEB60D  bl 0x82ea07b0
	ctx.lr = 0x82FB51A8;
	sub_82EA07B0(ctx, base);
	// 82FB51A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB51AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FB51B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB51B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB51B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FB51BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB51C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB51C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB51C8 size=16
    let mut pc: u32 = 0x82FB51C8;
    'dispatch: loop {
        match pc {
            0x82FB51C8 => {
    //   block [0x82FB51C8..0x82FB51D8)
	// 82FB51C8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FB51CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FB51D0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FB51D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB51D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB51D8 size=68
    let mut pc: u32 = 0x82FB51D8;
    'dispatch: loop {
        match pc {
            0x82FB51D8 => {
    //   block [0x82FB51D8..0x82FB521C)
	// 82FB51D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB51DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB51E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB51E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB51E8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FB51EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB51F0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FB51F4: 388B6020  addi r4, r11, 0x6020
	ctx.r[4].s64 = ctx.r[11].s64 + 24608;
	// 82FB51F8: 4BEF05E1  bl 0x82ea57d8
	ctx.lr = 0x82FB51FC;
	sub_82EA57D8(ctx, base);
	// 82FB51FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FB5200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB5204: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82FB5208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB520C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB5210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB5214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB5218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB5220 size=576
    let mut pc: u32 = 0x82FB5220;
    'dispatch: loop {
        match pc {
            0x82FB5220 => {
    //   block [0x82FB5220..0x82FB5324)
	// 82FB5220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB5224: 481F2F3D  bl 0x831a8160
	ctx.lr = 0x82FB5228;
	sub_831A8130(ctx, base);
	// 82FB5228: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB522C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FB5230: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 82FB5234: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82FB5238: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82FB523C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FB5240: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82FB5244: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FB5248: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FB524C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FB5250: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FB5254: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82FB5258: 48000999  bl 0x82fb5bf0
	ctx.lr = 0x82FB525C;
	sub_82FB5BF0(ctx, base);
	// 82FB525C: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FB5260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB5264: 409A0014  bne cr6, 0x82fb5278
	if !ctx.cr[6].eq {
	pc = 0x82FB5278; continue 'dispatch;
	}
	// 82FB5268: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB526C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FB5270: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FB5274: 481F2F3C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FB5278: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FB527C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FB5280: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FB5284: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FB5288: 80A10068  lwz r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FB528C: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82FB5290: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FB5294: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB5298: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FB529C: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FB52A0: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FB52A4: 7D08302E  lwzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FB52A8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB52AC: 7CEA29D6  mullw r7, r10, r5
	ctx.r[7].s64 = (ctx.r[10].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82FB52B0: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FB52B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FB52B8: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82FB52BC: 419A0024  beq cr6, 0x82fb52e0
	if ctx.cr[6].eq {
	pc = 0x82FB52E0; continue 'dispatch;
	}
	// 82FB52C0: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82FB52C4: 80670000  lwz r3, 0(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB52C8: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FB52CC: 419A0024  beq cr6, 0x82fb52f0
	if ctx.cr[6].eq {
	pc = 0x82FB52F0; continue 'dispatch;
	}
	// 82FB52D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FB52D4: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 82FB52D8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FB52DC: 4198FFE8  blt cr6, 0x82fb52c4
	if ctx.cr[6].lt {
	pc = 0x82FB52C4; continue 'dispatch;
	}
	// 82FB52E0: 989A0000  stb r4, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 82FB52E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FB52E8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FB52EC: 481F2EC4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FB52F0: 5787063E  clrlwi r7, r28, 0x18
	ctx.r[7].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82FB52F4: 2B070003  cmplwi cr6, r7, 3
	ctx.cr[6].compare_u32(ctx.r[7].u32, 3 as u32, &mut ctx.xer);
	// 82FB52F8: 4199FFE8  bgt cr6, 0x82fb52e0
	if ctx.cr[6].gt {
	pc = 0x82FB52E0; continue 'dispatch;
	}
	// 82FB52FC: 3D8082FB  lis r12, -0x7d05
	ctx.r[12].s64 = -2097479680;
	// 82FB5300: 398C5314  addi r12, r12, 0x5314
	ctx.r[12].s64 = ctx.r[12].s64 + 21268;
	// 82FB5304: 54E0103A  slwi r0, r7, 2
	ctx.r[0].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82FB5308: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82FB530C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82FB5310: 4E800420  bctr
	match ctx.r[7].u64 {
		0 => {
	pc = 0x82FB5420; continue 'dispatch;
		},
		1 => {
	pc = 0x82FB537C; continue 'dispatch;
		},
		2 => {
	pc = 0x82FB5324; continue 'dispatch;
		},
		3 => {
	pc = 0x82FB539C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82FB5314: 82FB5420  lwz r23, 0x5420(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(21536 as u32) ) } as u64;
	// 82FB5318: 82FB537C  lwz r23, 0x537c(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(21372 as u32) ) } as u64;
	// 82FB531C: 82FB5324  lwz r23, 0x5324(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(21284 as u32) ) } as u64;
	// 82FB5320: 82FB539C  lwz r23, 0x539c(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(21404 as u32) ) } as u64;
            }
            0x82FB5324 => {
    //   block [0x82FB5324..0x82FB537C)
	// 82FB5324: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB5328: 7CEB4214  add r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82FB532C: 81670004  lwz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB5330: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FB5334: 40980118  bge cr6, 0x82fb544c
	if !ctx.cr[6].lt {
	pc = 0x82FB544C; continue 'dispatch;
	}
	// 82FB5338: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82FB533C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FB5340: 419A0024  beq cr6, 0x82fb5364
	if ctx.cr[6].eq {
	pc = 0x82FB5364; continue 'dispatch;
	}
	// 82FB5344: 39680004  addi r11, r8, 4
	ctx.r[11].s64 = ctx.r[8].s64 + 4;
	// 82FB5348: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB534C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FB5350: 41980008  blt cr6, 0x82fb5358
	if ctx.cr[6].lt {
	pc = 0x82FB5358; continue 'dispatch;
	}
	// 82FB5354: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FB5358: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB535C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FB5360: 4082FFE8  bne 0x82fb5348
	if !ctx.cr[0].eq {
	pc = 0x82FB5348; continue 'dispatch;
	}
	// 82FB5364: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FB5368: 91270004  stw r9, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FB536C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FB5370: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB5374: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FB5378: 481F2E38  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            0x82FB537C => {
    //   block [0x82FB537C..0x82FB539C)
	// 82FB537C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB5380: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FB5384: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82FB5388: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82FB538C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FB5390: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB5394: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FB5398: 481F2E18  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            0x82FB539C => {
    //   block [0x82FB539C..0x82FB5420)
	// 82FB539C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB53A0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82FB53A4: 7CEB4214  add r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82FB53A8: 7C8B412E  stwx r4, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[4].u32) };
	// 82FB53AC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB53B0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FB53B4: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB53B8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FB53BC: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB53C0: 40990038  ble cr6, 0x82fb53f8
	if !ctx.cr[6].gt {
	pc = 0x82FB53F8; continue 'dispatch;
	}
	// 82FB53C4: 39680004  addi r11, r8, 4
	ctx.r[11].s64 = ctx.r[8].s64 + 4;
	// 82FB53C8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB53CC: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82FB53D0: 4099000C  ble cr6, 0x82fb53dc
	if !ctx.cr[6].gt {
	pc = 0x82FB53DC; continue 'dispatch;
	}
	// 82FB53D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82FB53D8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FB53DC: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB53E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FB53E4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FB53E8: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82FB53EC: 80A80004  lwz r5, 4(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB53F0: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FB53F4: 4198FFD4  blt cr6, 0x82fb53c8
	if ctx.cr[6].lt {
	pc = 0x82FB53C8; continue 'dispatch;
	}
	// 82FB53F8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB53FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FB5400: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FB5404: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB5408: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82FB540C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FB5410: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FB5414: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB5418: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FB541C: 481F2D94  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            0x82FB5420 => {
    //   block [0x82FB5420..0x82FB5460)
	// 82FB5420: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB5424: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB5428: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FB542C: 7D28302E  lwzx r9, r8, r6
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FB5430: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB5434: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB5438: 7D4729D6  mullw r10, r7, r5
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82FB543C: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FB5440: 7D6531D6  mullw r11, r5, r6
	ctx.r[11].s64 = (ctx.r[5].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82FB5444: 7C8B4A14  add r4, r11, r9
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FB5448: 909D0000  stw r4, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FB544C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FB5450: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FB5454: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB5458: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FB545C: 481F2D54  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB5460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB5460 size=368
    let mut pc: u32 = 0x82FB5460;
    'dispatch: loop {
        match pc {
            0x82FB5460 => {
    //   block [0x82FB5460..0x82FB55D0)
	// 82FB5460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB5464: 481F2D01  bl 0x831a8164
	ctx.lr = 0x82FB5468;
	sub_831A8130(ctx, base);
	// 82FB5468: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB546C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FB5470: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82FB5474: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FB5478: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82FB547C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB5480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FB5484: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FB5488: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FB548C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FB5490: 48000761  bl 0x82fb5bf0
	ctx.lr = 0x82FB5494;
	sub_82FB5BF0(ctx, base);
	// 82FB5494: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FB5498: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FB549C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FB54A0: 419A0128  beq cr6, 0x82fb55c8
	if ctx.cr[6].eq {
	pc = 0x82FB55C8; continue 'dispatch;
	}
	// 82FB54A4: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FB54A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82FB54AC: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FB54B0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82FB54B4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FB54B8: 83810068  lwz r28, 0x68(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FB54BC: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FB54C0: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FB54C4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB54C8: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82FB54CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB54D0: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82FB54D4: 7D49202E  lwzx r10, r9, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82FB54D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB54DC: 7D2BE1D6  mullw r9, r11, r28
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82FB54E0: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FB54E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB54E8: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FB54EC: 419A0044  beq cr6, 0x82fb5530
	if ctx.cr[6].eq {
	pc = 0x82FB5530; continue 'dispatch;
	}
	// 82FB54F0: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82FB54F4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82FB54F8: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB54FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FB5500: 41980018  blt cr6, 0x82fb5518
	if ctx.cr[6].lt {
	pc = 0x82FB5518; continue 'dispatch;
	}
	// 82FB5504: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82FB5508: 4099000C  ble cr6, 0x82fb5514
	if !ctx.cr[6].gt {
	pc = 0x82FB5514; continue 'dispatch;
	}
	// 82FB550C: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82FB5510: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82FB5514: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FB5518: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82FB551C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82FB5520: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FB5524: 4198FFD4  blt cr6, 0x82fb54f8
	if ctx.cr[6].lt {
	pc = 0x82FB54F8; continue 'dispatch;
	}
	// 82FB5528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FB552C: 409A0010  bne cr6, 0x82fb553c
	if !ctx.cr[6].eq {
	pc = 0x82FB553C; continue 'dispatch;
	}
	// 82FB5530: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FB5534: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FB5538: 481F2C7C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FB553C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FB5540: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FB5544: 419A0048  beq cr6, 0x82fb558c
	if ctx.cr[6].eq {
	pc = 0x82FB558C; continue 'dispatch;
	}
	// 82FB5548: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FB554C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB5550: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB5554: 4198001C  blt cr6, 0x82fb5570
	if ctx.cr[6].lt {
	pc = 0x82FB5570; continue 'dispatch;
	}
	// 82FB5558: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FB555C: 0CC30000  twi 6, r3, 0
	// 82FB5560: 7D091B96  divwu r8, r9, r3
	ctx.r[8].u32 = ctx.r[9].u32 / ctx.r[3].u32;
	// 82FB5564: 7CC819D6  mullw r6, r8, r3
	ctx.r[6].s64 = (ctx.r[8].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82FB5568: 7D264850  subf r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 82FB556C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FB5570: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB5574: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FB5578: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FB557C: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82FB5580: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB5584: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FB5588: 4198FFC4  blt cr6, 0x82fb554c
	if ctx.cr[6].lt {
	pc = 0x82FB554C; continue 'dispatch;
	}
	// 82FB558C: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB5590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FB5594: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FB5598: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FB559C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FB55A0: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FB55A4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB55A8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82FB55AC: 7D49202E  lwzx r10, r9, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82FB55B0: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB55B4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB55B8: 7D67E1D6  mullw r11, r7, r28
	ctx.r[11].s64 = (ctx.r[7].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82FB55BC: 7CCB2A14  add r6, r11, r5
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82FB55C0: 7D6641D6  mullw r11, r6, r8
	ctx.r[11].s64 = (ctx.r[6].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82FB55C4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FB55C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FB55CC: 481F2BE8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB55D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FB55D0 size=176
    let mut pc: u32 = 0x82FB55D0;
    'dispatch: loop {
        match pc {
            0x82FB55D0 => {
    //   block [0x82FB55D0..0x82FB5680)
	// 82FB55D0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82FB55D4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FB55D8: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FB55DC: 40980094  bge cr6, 0x82fb5670
	if !ctx.cr[6].lt {
	pc = 0x82FB5670; continue 'dispatch;
	}
	// 82FB55E0: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FB55E4: 81440024  lwz r10, 0x24(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB55E8: 54BF103A  slwi r31, r5, 2
	ctx.r[31].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82FB55EC: 80C40018  lwz r6, 0x18(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FB55F0: 7CA55A14  add r5, r5, r11
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82FB55F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FB55F8: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FB55FC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82FB5600: 7D2A4214  add r9, r10, r8
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82FB5604: 7D46F82E  lwzx r10, r6, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FB5608: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB560C: 80A90000  lwz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB5610: 7D2629D7  mullw. r9, r6, r5
	ctx.r[9].s64 = (ctx.r[6].s32 as i64) * (ctx.r[5].s32 as i64);
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FB5614: 4182004C  beq 0x82fb5660
	if ctx.cr[0].eq {
	pc = 0x82FB5660; continue 'dispatch;
	}
	// 82FB5618: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FB561C: 81240024  lwz r9, 0x24(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB5620: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FB5624: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB5628: 7CAB3396  divwu r5, r11, r6
	ctx.r[5].u32 = ctx.r[11].u32 / ctx.r[6].u32;
	// 82FB562C: 7D2531D6  mullw r9, r5, r6
	ctx.r[9].s64 = (ctx.r[5].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82FB5630: 0CC60000  twi 6, r6, 0
	// 82FB5634: 7CC95850  subf r6, r9, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82FB5638: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FB563C: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82FB5640: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82FB5644: 81240024  lwz r9, 0x24(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FB5648: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FB564C: 80A90004  lwz r5, 4(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FB5650: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FB5654: 7CC549D6  mullw r6, r5, r9
	ctx.r[6].s64 = (ctx.r[5].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82FB5658: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82FB565C: 4198FFBC  blt cr6, 0x82fb5618
	if ctx.cr[6].lt {
	pc = 0x82FB5618; continue 'dispatch;
	}
	// 82FB5660: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FB5664: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB5668: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82FB566C: 4E800020  blr
	return;
	// 82FB5670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB5674: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FB5678: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82FB567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB5680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB5680 size=84
    let mut pc: u32 = 0x82FB5680;
    'dispatch: loop {
        match pc {
            0x82FB5680 => {
    //   block [0x82FB5680..0x82FB56D4)
	// 82FB5680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB5684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB5688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB568C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82FB5690: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82FB5694: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FB5698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FB569C: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82FB56A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FB56A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FB56A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FB56AC: 4BFFFB75  bl 0x82fb5220
	ctx.lr = 0x82FB56B0;
	sub_82FB5220(ctx, base);
	// 82FB56B0: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FB56B4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FB56B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FB56BC: 409A0008  bne cr6, 0x82fb56c4
	if !ctx.cr[6].eq {
	pc = 0x82FB56C4; continue 'dispatch;
	}
	// 82FB56C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FB56C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB56C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB56CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB56D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB56D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB56D8 size=56
    let mut pc: u32 = 0x82FB56D8;
    'dispatch: loop {
        match pc {
            0x82FB56D8 => {
    //   block [0x82FB56D8..0x82FB5710)
	// 82FB56D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB56DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB56E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB56E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB56E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB56EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82FB56F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB56F4: 4BFFFB2D  bl 0x82fb5220
	ctx.lr = 0x82FB56F8;
	sub_82FB5220(ctx, base);
	// 82FB56F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB56FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB5700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB5704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB5708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB570C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FB5710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FB5710 size=56
    let mut pc: u32 = 0x82FB5710;
    'dispatch: loop {
        match pc {
            0x82FB5710 => {
    //   block [0x82FB5710..0x82FB5748)
	// 82FB5710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FB5714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FB5718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FB571C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FB5720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FB5724: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82FB5728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FB572C: 4BFFFAF5  bl 0x82fb5220
	ctx.lr = 0x82FB5730;
	sub_82FB5220(ctx, base);
	// 82FB5730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FB5734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FB5738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FB573C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FB5740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FB5744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


