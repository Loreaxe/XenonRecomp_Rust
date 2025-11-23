pub fn sub_82E3E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E628 size=8
    let mut pc: u32 = 0x82E3E628;
    'dispatch: loop {
        match pc {
            0x82E3E628 => {
    //   block [0x82E3E628..0x82E3E630)
	// 82E3E628: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82E3E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E630 size=8
    let mut pc: u32 = 0x82E3E630;
    'dispatch: loop {
        match pc {
            0x82E3E630 => {
    //   block [0x82E3E630..0x82E3E638)
	// 82E3E630: 38630070  addi r3, r3, 0x70
	ctx.r[3].s64 = ctx.r[3].s64 + 112;
	// 82E3E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E638 size=28
    let mut pc: u32 = 0x82E3E638;
    'dispatch: loop {
        match pc {
            0x82E3E638 => {
    //   block [0x82E3E638..0x82E3E654)
	// 82E3E638: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3E658 size=208
    let mut pc: u32 = 0x82E3E658;
    'dispatch: loop {
        match pc {
            0x82E3E658 => {
    //   block [0x82E3E658..0x82E3E728)
	// 82E3E658: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3E65C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3E660: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3E664: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3E668: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82E3E66C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3E670: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3E674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E728 size=156
    let mut pc: u32 = 0x82E3E728;
    'dispatch: loop {
        match pc {
            0x82E3E728 => {
    //   block [0x82E3E728..0x82E3E7C4)
	// 82E3E728: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E3E72C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3E730: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3E734: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3E738: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3E7C8 size=1236
    let mut pc: u32 = 0x82E3E7C8;
    'dispatch: loop {
        match pc {
            0x82E3E7C8 => {
    //   block [0x82E3E7C8..0x82E3E7FC)
	// 82E3E7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3E7CC: 4BE6AC39  bl 0x82ca9404
	ctx.lr = 0x82E3E7D0;
	sub_82CA93D0(ctx, base);
	// 82E3E7D0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3E7D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3E7D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3E7DC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3E7E0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3E7E4: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82E3E7E8: 40980020  bge cr6, 0x82e3e808
	if !ctx.cr[6].lt {
	pc = 0x82E3E808; continue 'dispatch;
	}
	// 82E3E7EC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3E7F0: 2F040018  cmpwi cr6, r4, 0x18
	ctx.cr[6].compare_i32(ctx.r[4].s32, 24, &mut ctx.xer);
	// 82E3E7F4: 41990008  bgt cr6, 0x82e3e7fc
	if ctx.cr[6].gt {
	pc = 0x82E3E7FC; continue 'dispatch;
	}
	// 82E3E7F8: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	pc = 0x82E3E7FC; continue 'dispatch;
            }
            0x82E3E7FC => {
    //   block [0x82E3E7FC..0x82E3E808)
	// 82E3E7FC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3E800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3E804: 4BF1870D  bl 0x82d56f10
	ctx.lr = 0x82E3E808;
	sub_82D56F10(ctx, base);
	pc = 0x82E3E808; continue 'dispatch;
            }
            0x82E3E808 => {
    //   block [0x82E3E808..0x82E3EC9C)
	// 82E3E808: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82E3E80C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3E810: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82E3E814: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E3E818: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82E3E81C: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E3E820: C1BF0060  lfs f13, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E824: D1A100C0  stfs f13, 0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82E3E828: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82E3E82C: C1BF0064  lfs f13, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E830: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82E3E834: D1A100C4  stfs f13, 0xc4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82E3E838: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82E3E83C: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3E840: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3E844: C1BF0068  lfs f13, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E848: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E3E84C: D1A100C8  stfs f13, 0xc8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82E3E850: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 82E3E854: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82E3E858: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82E3E85C: D00100BC  stfs f0, 0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82E3E860: D00100DC  stfs f0, 0xdc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 82E3E864: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82E3E868: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3ECA0 size=2276
    let mut pc: u32 = 0x82E3ECA0;
    'dispatch: loop {
        match pc {
            0x82E3ECA0 => {
    //   block [0x82E3ECA0..0x82E3ECE4)
	// 82E3ECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3ECA4: 4BE6A745  bl 0x82ca93e8
	ctx.lr = 0x82E3ECA8;
	sub_82CA93D0(ctx, base);
	// 82E3ECA8: DBE1FF90  stfd f31, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[31].u64 ) };
	// 82E3ECAC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3ECB0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ECB4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3ECB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3ECBC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3ECC0: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3ECC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3ECC8: 419A001C  beq cr6, 0x82e3ece4
	if ctx.cr[6].eq {
	pc = 0x82E3ECE4; continue 'dispatch;
	}
	// 82E3ECCC: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3ECD0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3ECD4: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3ECD8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ECDC: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3ECE0: 48000010  b 0x82e3ecf0
	pc = 0x82E3ECF0; continue 'dispatch;
            }
            0x82E3ECE4 => {
    //   block [0x82E3ECE4..0x82E3ECF0)
	// 82E3ECE4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3ECE8: 4BF16369  bl 0x82d55050
	ctx.lr = 0x82E3ECEC;
	sub_82D55050(ctx, base);
	// 82E3ECEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82E3ECF0; continue 'dispatch;
            }
            0x82E3ECF0 => {
    //   block [0x82E3ECF0..0x82E3ED20)
	// 82E3ECF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3ECF4: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82E3ECF8: 419A0028  beq cr6, 0x82e3ed20
	if ctx.cr[6].eq {
	pc = 0x82E3ED20; continue 'dispatch;
	}
	// 82E3ECFC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3ED00: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82E3ED04: 928B0004  stw r20, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 82E3ED08: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E3ED0C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3ED10: 928B000C  stw r20, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 82E3ED14: 928B0010  stw r20, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[20].u32 ) };
	// 82E3ED18: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3ED1C: 48000008  b 0x82e3ed24
	pc = 0x82E3ED24; continue 'dispatch;
            }
            0x82E3ED20 => {
    //   block [0x82E3ED20..0x82E3ED24)
	// 82E3ED20: 7E9EA378  mr r30, r20
	ctx.r[30].u64 = ctx.r[20].u64;
	pc = 0x82E3ED24; continue 'dispatch;
            }
            0x82E3ED24 => {
    //   block [0x82E3ED24..0x82E3ED50)
	// 82E3ED24: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E3ED28: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3ED2C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3ED30: 3BBC0001  addi r29, r28, 1
	ctx.r[29].s64 = ctx.r[28].s64 + 1;
	// 82E3ED34: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3ED38: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3ED3C: 40980024  bge cr6, 0x82e3ed60
	if !ctx.cr[6].lt {
	pc = 0x82E3ED60; continue 'dispatch;
	}
	// 82E3ED40: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3ED44: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3ED48: 41980008  blt cr6, 0x82e3ed50
	if ctx.cr[6].lt {
	pc = 0x82E3ED50; continue 'dispatch;
	}
	// 82E3ED4C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82E3ED50; continue 'dispatch;
            }
            0x82E3ED50 => {
    //   block [0x82E3ED50..0x82E3ED60)
	// 82E3ED50: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3ED54: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3ED58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3ED5C: 4BF181B5  bl 0x82d56f10
	ctx.lr = 0x82E3ED60;
	sub_82D56F10(ctx, base);
	pc = 0x82E3ED60; continue 'dispatch;
            }
            0x82E3ED60 => {
    //   block [0x82E3ED60..0x82E3F584)
	// 82E3ED60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3ED64: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3ED68: C01F0060  lfs f0, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3ED6C: 578A2036  slwi r10, r28, 4
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3ED70: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E3ED74: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3ED78: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E3ED7C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3ED80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ED84: C01F0068  lfs f0, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3ED88: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E3ED8C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E3ED90: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3F588 size=176
    let mut pc: u32 = 0x82E3F588;
    'dispatch: loop {
        match pc {
            0x82E3F588 => {
    //   block [0x82E3F588..0x82E3F638)
	// 82E3F588: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3F58C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E3F590: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3F594: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3F598: 39295BAC  addi r9, r9, 0x5bac
	ctx.r[9].s64 = ctx.r[9].s64 + 23468;
	// 82E3F59C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3F5A0: C0080C14  lfs f0, 0xc14(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F5A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3F5A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3F638 size=140
    let mut pc: u32 = 0x82E3F638;
    'dispatch: loop {
        match pc {
            0x82E3F638 => {
    //   block [0x82E3F638..0x82E3F6C4)
	// 82E3F638: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3F63C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E3F640: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3F644: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3F648: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82E3F64C: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3F6C8 size=64
    let mut pc: u32 = 0x82E3F6C8;
    'dispatch: loop {
        match pc {
            0x82E3F6C8 => {
    //   block [0x82E3F6C8..0x82E3F708)
	// 82E3F6C8: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3F708 size=348
    let mut pc: u32 = 0x82E3F708;
    'dispatch: loop {
        match pc {
            0x82E3F708 => {
    //   block [0x82E3F708..0x82E3F740)
	// 82E3F708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3F70C: 4BE69CF9  bl 0x82ca9404
	ctx.lr = 0x82E3F710;
	sub_82CA93D0(ctx, base);
	// 82E3F710: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E3F714: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3F718: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E3F71C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E3F720: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3F724: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3F728: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82E3F72C: 40980020  bge cr6, 0x82e3f74c
	if !ctx.cr[6].lt {
	pc = 0x82E3F74C; continue 'dispatch;
	}
	// 82E3F730: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3F734: 2F040018  cmpwi cr6, r4, 0x18
	ctx.cr[6].compare_i32(ctx.r[4].s32, 24, &mut ctx.xer);
	// 82E3F738: 41990008  bgt cr6, 0x82e3f740
	if ctx.cr[6].gt {
	pc = 0x82E3F740; continue 'dispatch;
	}
	// 82E3F73C: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	pc = 0x82E3F740; continue 'dispatch;
            }
            0x82E3F740 => {
    //   block [0x82E3F740..0x82E3F74C)
	// 82E3F740: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3F744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3F748: 4BF177C9  bl 0x82d56f10
	ctx.lr = 0x82E3F74C;
	sub_82D56F10(ctx, base);
	pc = 0x82E3F74C; continue 'dispatch;
            }
            0x82E3F74C => {
    //   block [0x82E3F74C..0x82E3F768)
	// 82E3F74C: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82E3F750: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E3F754: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82E3F758: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82E3F75C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3F760: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3F764: C3EB0EE0  lfs f31, 0xee0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3808 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x82E3F768; continue 'dispatch;
            }
            0x82E3F768 => {
    //   block [0x82E3F768..0x82E3F794)
	// 82E3F768: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3F76C: E93C0068  ld r9, 0x68(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(104 as u32) ) };
	// 82E3F770: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E3F774: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3F778: E95C0060  ld r10, 0x60(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) };
	// 82E3F77C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3F780: F92B0008  std r9, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 82E3F784: 419A0010  beq cr6, 0x82e3f794
	if ctx.cr[6].eq {
	pc = 0x82E3F794; continue 'dispatch;
	}
	// 82E3F788: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F78C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E3F790: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	pc = 0x82E3F794; continue 'dispatch;
            }
            0x82E3F794 => {
    //   block [0x82E3F794..0x82E3F7AC)
	// 82E3F794: 57CB07BC  rlwinm r11, r30, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3F798: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3F79C: 419A0010  beq cr6, 0x82e3f7ac
	if ctx.cr[6].eq {
	pc = 0x82E3F7AC; continue 'dispatch;
	}
	// 82E3F7A0: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F7A4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E3F7A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	pc = 0x82E3F7AC; continue 'dispatch;
            }
            0x82E3F7AC => {
    //   block [0x82E3F7AC..0x82E3F7C4)
	// 82E3F7AC: 57CB077A  rlwinm r11, r30, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3F7B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3F7B4: 419A0010  beq cr6, 0x82e3f7c4
	if ctx.cr[6].eq {
	pc = 0x82E3F7C4; continue 'dispatch;
	}
	// 82E3F7B8: C0010058  lfs f0, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F7BC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E3F7C0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x82E3F7C4; continue 'dispatch;
            }
            0x82E3F7C4 => {
    //   block [0x82E3F7C4..0x82E3F83C)
	// 82E3F7C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3F7C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E3F7CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3F7D0: 4BF16BD9  bl 0x82d563a8
	ctx.lr = 0x82E3F7D4;
	sub_82D563A8(ctx, base);
	// 82E3F7D4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E3F7D8: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82E3F7DC: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 82E3F7E0: 4198FF88  blt cr6, 0x82e3f768
	if ctx.cr[6].lt {
	pc = 0x82E3F768; continue 'dispatch;
	}
	// 82E3F7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E3F7E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E3F7EC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E3F7F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3F7F4: 550B2036  slwi r11, r8, 4
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3F7F8: 7CA72278  xor r7, r5, r4
	ctx.r[7].u64 = ctx.r[5].u64 ^ ctx.r[4].u64;
	// 82E3F7FC: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82E3F800: 4098003C  bge cr6, 0x82e3f83c
	if !ctx.cr[6].lt {
	pc = 0x82E3F83C; continue 'dispatch;
	}
	// 82E3F804: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F808: 39480001  addi r10, r8, 1
	ctx.r[10].s64 = ctx.r[8].s64 + 1;
	pc = 0x82E3F83C; continue 'dispatch;
            }
            0x82E3F83C => {
    //   block [0x82E3F83C..0x82E3F864)
	// 82E3F83C: 54A5083C  slwi r5, r5, 1
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3F840: 2F050008  cmpwi cr6, r5, 8
	ctx.cr[6].compare_i32(ctx.r[5].s32, 8, &mut ctx.xer);
	// 82E3F844: 4198FFB4  blt cr6, 0x82e3f7f8
	if ctx.cr[6].lt {
	pc = 0x82E3F7F8; continue 'dispatch;
	}
	// 82E3F848: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82E3F84C: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 82E3F850: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 82E3F854: 4198FF9C  blt cr6, 0x82e3f7f0
	if ctx.cr[6].lt {
	pc = 0x82E3F7F0; continue 'dispatch;
	}
	// 82E3F858: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E3F85C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E3F860: 4BE69BF4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3F868 size=2400
    let mut pc: u32 = 0x82E3F868;
    'dispatch: loop {
        match pc {
            0x82E3F868 => {
    //   block [0x82E3F868..0x82E3F8AC)
	// 82E3F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3F86C: 4BE69B7D  bl 0x82ca93e8
	ctx.lr = 0x82E3F870;
	sub_82CA93D0(ctx, base);
	// 82E3F870: DBE1FF90  stfd f31, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[31].u64 ) };
	// 82E3F874: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3F878: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F87C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3F880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3F884: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3F888: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3F88C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3F890: 419A001C  beq cr6, 0x82e3f8ac
	if ctx.cr[6].eq {
	pc = 0x82E3F8AC; continue 'dispatch;
	}
	// 82E3F894: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3F898: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3F89C: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3F8A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F8A4: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3F8A8: 48000010  b 0x82e3f8b8
	pc = 0x82E3F8B8; continue 'dispatch;
            }
            0x82E3F8AC => {
    //   block [0x82E3F8AC..0x82E3F8B8)
	// 82E3F8AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3F8B0: 4BF157A1  bl 0x82d55050
	ctx.lr = 0x82E3F8B4;
	sub_82D55050(ctx, base);
	// 82E3F8B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82E3F8B8; continue 'dispatch;
            }
            0x82E3F8B8 => {
    //   block [0x82E3F8B8..0x82E3F8E8)
	// 82E3F8B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3F8BC: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82E3F8C0: 419A0028  beq cr6, 0x82e3f8e8
	if ctx.cr[6].eq {
	pc = 0x82E3F8E8; continue 'dispatch;
	}
	// 82E3F8C4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3F8C8: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82E3F8CC: 928B0004  stw r20, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 82E3F8D0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E3F8D4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3F8D8: 928B000C  stw r20, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 82E3F8DC: 928B0010  stw r20, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[20].u32 ) };
	// 82E3F8E0: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3F8E4: 48000008  b 0x82e3f8ec
	pc = 0x82E3F8EC; continue 'dispatch;
            }
            0x82E3F8E8 => {
    //   block [0x82E3F8E8..0x82E3F8EC)
	// 82E3F8E8: 7E9EA378  mr r30, r20
	ctx.r[30].u64 = ctx.r[20].u64;
	pc = 0x82E3F8EC; continue 'dispatch;
            }
            0x82E3F8EC => {
    //   block [0x82E3F8EC..0x82E3F918)
	// 82E3F8EC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E3F8F0: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3F8F4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3F8F8: 3BBC0001  addi r29, r28, 1
	ctx.r[29].s64 = ctx.r[28].s64 + 1;
	// 82E3F8FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3F900: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3F904: 40980024  bge cr6, 0x82e3f928
	if !ctx.cr[6].lt {
	pc = 0x82E3F928; continue 'dispatch;
	}
	// 82E3F908: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3F90C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3F910: 41980008  blt cr6, 0x82e3f918
	if ctx.cr[6].lt {
	pc = 0x82E3F918; continue 'dispatch;
	}
	// 82E3F914: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82E3F918; continue 'dispatch;
            }
            0x82E3F918 => {
    //   block [0x82E3F918..0x82E3F928)
	// 82E3F918: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3F91C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3F920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3F924: 4BF175ED  bl 0x82d56f10
	ctx.lr = 0x82E3F928;
	sub_82D56F10(ctx, base);
	pc = 0x82E3F928; continue 'dispatch;
            }
            0x82E3F928 => {
    //   block [0x82E3F928..0x82E401C8)
	// 82E3F928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3F92C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3F930: C01F0060  lfs f0, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F934: 578A2036  slwi r10, r28, 4
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3F938: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E3F93C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E3F940: C1BF0064  lfs f13, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3F944: C01F0068  lfs f0, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F948: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3F94C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F950: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E3F954: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E3F958: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E3F95C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E401C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E401C8 size=4
    let mut pc: u32 = 0x82E401C8;
    'dispatch: loop {
        match pc {
            0x82E401C8 => {
    //   block [0x82E401C8..0x82E401CC)
	// 82E401C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E401D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E401D0 size=168
    let mut pc: u32 = 0x82E401D0;
    'dispatch: loop {
        match pc {
            0x82E401D0 => {
    //   block [0x82E401D0..0x82E40278)
	// 82E401D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E401D4: 4BE69235  bl 0x82ca9408
	ctx.lr = 0x82E401D8;
	sub_82CA93D0(ctx, base);
	// 82E401D8: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82E401DC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E401E0: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E401E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E401E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E40278 size=1864
    let mut pc: u32 = 0x82E40278;
    'dispatch: loop {
        match pc {
            0x82E40278 => {
    //   block [0x82E40278..0x82E409C0)
	// 82E40278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4027C: 4BE69161  bl 0x82ca93dc
	ctx.lr = 0x82E40280;
	sub_82CA93D0(ctx, base);
	// 82E40280: 3981FF80  addi r12, r1, -0x80
	ctx.r[12].s64 = ctx.r[1].s64 + -128;
	// 82E40284: 481C6749  bl 0x830069cc
	ctx.lr = 0x82E40288;
	sub_83006760(ctx, base);
	// 82E40288: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4028C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E40290: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82E40294: 397E0070  addi r11, r30, 0x70
	ctx.r[11].s64 = ctx.r[30].s64 + 112;
	// 82E40298: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E4029C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82E402A0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E409C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E409C0 size=100
    let mut pc: u32 = 0x82E409C0;
    'dispatch: loop {
        match pc {
            0x82E409C0 => {
    //   block [0x82E409C0..0x82E40A08)
	// 82E409C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E409C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E409C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E409CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E409D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E409D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E409D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E409DC: 48005915  bl 0x82e462f0
	ctx.lr = 0x82E409E0;
	sub_82E462F0(ctx, base);
	// 82E409E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E409E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E409E8: 419A0020  beq cr6, 0x82e40a08
	if ctx.cr[6].eq {
	pc = 0x82E40A08; continue 'dispatch;
	}
	// 82E409EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E409F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E409F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82E409F8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E409FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E40A00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E40A04: 4BF148C5  bl 0x82d552c8
	ctx.lr = 0x82E40A08;
	sub_82D552C8(ctx, base);
	pc = 0x82E40A08; continue 'dispatch;
            }
            0x82E40A08 => {
    //   block [0x82E40A08..0x82E40A24)
	// 82E40A08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E40A0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E40A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E40A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E40A18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E40A1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E40A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E40A28 size=4
    let mut pc: u32 = 0x82E40A28;
    'dispatch: loop {
        match pc {
            0x82E40A28 => {
    //   block [0x82E40A28..0x82E40A2C)
	// 82E40A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E40A30 size=168
    let mut pc: u32 = 0x82E40A30;
    'dispatch: loop {
        match pc {
            0x82E40A30 => {
    //   block [0x82E40A30..0x82E40AD8)
	// 82E40A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E40A34: 4BE689D5  bl 0x82ca9408
	ctx.lr = 0x82E40A38;
	sub_82CA93D0(ctx, base);
	// 82E40A38: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82E40A3C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E40A40: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E40A44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E40A48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E40AD8 size=11644
    let mut pc: u32 = 0x82E40AD8;
    'dispatch: loop {
        match pc {
            0x82E40AD8 => {
    //   block [0x82E40AD8..0x82E43854)
	// 82E40AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E40ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E40AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E40AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E40AE8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82E40AEC: 9421EB40  stwu r1, -0x14c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-5312 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E40AF0: 906114D4  stw r3, 0x14d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(5332 as u32), ctx.r[3].u32 ) };
	// 82E40AF4: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40AF8: 396B0060  addi r11, r11, 0x60
	ctx.r[11].s64 = ctx.r[11].s64 + 96;
	// 82E40AFC: 91610188  stw r11, 0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 82E40B00: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B04: 396B0070  addi r11, r11, 0x70
	ctx.r[11].s64 = ctx.r[11].s64 + 112;
	// 82E40B08: 91610184  stw r11, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 82E40B0C: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B10: C00B0080  lfs f0, 0x80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E40B14: D0010180  stfs f0, 0x180(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82E40B18: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B1C: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E40B20: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82E40B24: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B28: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E40B2C: 91610134  stw r11, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82E40B30: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82E40B34: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E40B38: 91610144  stw r11, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 82E40B3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E40B40: 91610138  stw r11, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 82E40B44: 39610138  addi r11, r1, 0x138
	ctx.r[11].s64 = ctx.r[1].s64 + 312;
	// 82E40B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E40B4C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E40B50: 39610138  addi r11, r1, 0x138
	ctx.r[11].s64 = ctx.r[1].s64 + 312;
	// 82E40B54: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E40B58: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E40B5C: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 82E40B60: 4BD56319  bl 0x82b96e78
	ctx.lr = 0x82E40B64;
	sub_82B96E78(ctx, base);
	// 82E40B64: 81610188  lwz r11, 0x188(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E43858 size=4
    let mut pc: u32 = 0x82E43858;
    'dispatch: loop {
        match pc {
            0x82E43858 => {
    //   block [0x82E43858..0x82E4385C)
	// 82E43858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E43860 size=176
    let mut pc: u32 = 0x82E43860;
    'dispatch: loop {
        match pc {
            0x82E43860 => {
    //   block [0x82E43860..0x82E43910)
	// 82E43860: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E43864: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E43868: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4386C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E43870: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E43874: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E43910 size=728
    let mut pc: u32 = 0x82E43910;
    'dispatch: loop {
        match pc {
            0x82E43910 => {
    //   block [0x82E43910..0x82E43988)
	// 82E43910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E43914: 4BE65AED  bl 0x82ca9400
	ctx.lr = 0x82E43918;
	sub_82CA93D0(ctx, base);
	// 82E43918: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 82E4391C: 4BE6A3BD  bl 0x82cadcd8
	ctx.lr = 0x82E43920;
	sub_82CADCA0(ctx, base);
	// 82E43920: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E43924: FFE01090  fmr f31, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[2].f64;
	// 82E43928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4392C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E43930: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E43934: D3A10134  stfs f29, 0x134(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 82E43938: C38B0C18  lfs f28, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E4393C: FF1FE000  fcmpu cr6, f31, f28
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[28].f64);
	// 82E43940: 419A0298  beq cr6, 0x82e43bd8
	if ctx.cr[6].eq {
	pc = 0x82E43BD8; continue 'dispatch;
	}
	// 82E43944: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E43948: 3B7D000C  addi r27, r29, 0xc
	ctx.r[27].s64 = ctx.r[29].s64 + 12;
	// 82E4394C: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82E43950: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82E43954: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E43958: C3CB0C14  lfs f30, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E4395C: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E43960: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E43964: 3B9F0003  addi r28, r31, 3
	ctx.r[28].s64 = ctx.r[31].s64 + 3;
	// 82E43968: 83DD0010  lwz r30, 0x10(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4396C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E43970: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E43974: 40980024  bge cr6, 0x82e43998
	if !ctx.cr[6].lt {
	pc = 0x82E43998; continue 'dispatch;
	}
	// 82E43978: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4397C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E43980: 41980008  blt cr6, 0x82e43988
	if ctx.cr[6].lt {
	pc = 0x82E43988; continue 'dispatch;
	}
	// 82E43984: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82E43988; continue 'dispatch;
            }
            0x82E43988 => {
    //   block [0x82E43988..0x82E43998)
	// 82E43988: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E4398C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E43990: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E43994: 4BF1357D  bl 0x82d56f10
	ctx.lr = 0x82E43998;
	sub_82D56F10(ctx, base);
	pc = 0x82E43998; continue 'dispatch;
            }
            0x82E43998 => {
    //   block [0x82E43998..0x82E439C4)
	// 82E43998: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E4399C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E439A0: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E439A4: 3B8B0003  addi r28, r11, 3
	ctx.r[28].s64 = ctx.r[11].s64 + 3;
	// 82E439A8: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E439AC: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E439B0: 40980024  bge cr6, 0x82e439d4
	if !ctx.cr[6].lt {
	pc = 0x82E439D4; continue 'dispatch;
	}
	// 82E439B4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E439B8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E439BC: 41980008  blt cr6, 0x82e439c4
	if ctx.cr[6].lt {
	pc = 0x82E439C4; continue 'dispatch;
	}
	// 82E439C0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82E439C4; continue 'dispatch;
            }
            0x82E439C4 => {
    //   block [0x82E439C4..0x82E439D4)
	// 82E439C4: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82E439C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E439CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E439D0: 4BF13541  bl 0x82d56f10
	ctx.lr = 0x82E439D4;
	sub_82D56F10(ctx, base);
	pc = 0x82E439D4; continue 'dispatch;
            }
            0x82E439D4 => {
    //   block [0x82E439D4..0x82E43BD8)
	// 82E439D4: 939B0004  stw r28, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E439D8: 38E10134  addi r7, r1, 0x134
	ctx.r[7].s64 = ctx.r[1].s64 + 308;
	// 82E439DC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E439E0: 38C10134  addi r6, r1, 0x134
	ctx.r[6].s64 = ctx.r[1].s64 + 308;
	// 82E439E4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E439E8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E439EC: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E439F0: 57E82036  slwi r8, r31, 4
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	pc = 0x82E43BD8; continue 'dispatch;
            }
            0x82E43BD8 => {
    //   block [0x82E43BD8..0x82E43BE8)
	// 82E43BD8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82E43BDC: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 82E43BE0: 4BE6A145  bl 0x82cadd24
	ctx.lr = 0x82E43BE4;
	sub_82CADCEC(ctx, base);
	// 82E43BE4: 4BE6586C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E43BE8 size=1272
    let mut pc: u32 = 0x82E43BE8;
    'dispatch: loop {
        match pc {
            0x82E43BE8 => {
    //   block [0x82E43BE8..0x82E43C38)
	// 82E43BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E43BEC: 4BE65821  bl 0x82ca940c
	ctx.lr = 0x82E43BF0;
	sub_82CA93D0(ctx, base);
	// 82E43BF0: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82E43BF4: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E43BF8: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E43BFC: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E43C00: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E43C04: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E43C08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E43C0C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E43C10: 93C10174  stw r30, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 82E43C14: 806B0050  lwz r3, 0x50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E43C18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E43C1C: 419A001C  beq cr6, 0x82e43c38
	if ctx.cr[6].eq {
	pc = 0x82E43C38; continue 'dispatch;
	}
	// 82E43C20: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E43C24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E43C28: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E43C2C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E43C30: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E43C34: 48000010  b 0x82e43c44
	pc = 0x82E43C44; continue 'dispatch;
            }
            0x82E43C38 => {
    //   block [0x82E43C38..0x82E43C44)
	// 82E43C38: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E43C3C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E43C40: 4BF11411  bl 0x82d55050
	ctx.lr = 0x82E43C44;
	sub_82D55050(ctx, base);
	pc = 0x82E43C44; continue 'dispatch;
            }
            0x82E43C44 => {
    //   block [0x82E43C44..0x82E43C74)
	// 82E43C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E43C48: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E43C4C: 419A0028  beq cr6, 0x82e43c74
	if ctx.cr[6].eq {
	pc = 0x82E43C74; continue 'dispatch;
	}
	// 82E43C50: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E43C54: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E43C58: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E43C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E43C60: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E43C64: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E43C68: 93A30010  stw r29, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E43C6C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E43C70: 48000008  b 0x82e43c78
	pc = 0x82E43C78; continue 'dispatch;
            }
            0x82E43C74 => {
    //   block [0x82E43C74..0x82E43C78)
	// 82E43C74: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	pc = 0x82E43C78; continue 'dispatch;
            }
            0x82E43C78 => {
    //   block [0x82E43C78..0x82E43CB8)
	// 82E43C78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E43C7C: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82E43C80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E43C84: C3BE006C  lfs f29, 0x6c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E43C88: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E43C8C: C00A5C94  lfs f0, 0x5c94(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(23700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E43C90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E43C94: EFFD0032  fmuls f31, f29, f0
	ctx.f[31].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E43C98: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82E43C9C: C00A5C90  lfs f0, 0x5c90(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(23696 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E43CA0: EFDD0032  fmuls f30, f29, f0
	ctx.f[30].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E43CA4: 40980020  bge cr6, 0x82e43cc4
	if !ctx.cr[6].lt {
	pc = 0x82E43CC4; continue 'dispatch;
	}
	// 82E43CA8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E43CAC: 2F04000C  cmpwi cr6, r4, 0xc
	ctx.cr[6].compare_i32(ctx.r[4].s32, 12, &mut ctx.xer);
	// 82E43CB0: 41990008  bgt cr6, 0x82e43cb8
	if ctx.cr[6].gt {
	pc = 0x82E43CB8; continue 'dispatch;
	}
	// 82E43CB4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	pc = 0x82E43CB8; continue 'dispatch;
            }
            0x82E43CB8 => {
    //   block [0x82E43CB8..0x82E43CC4)
	// 82E43CB8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E43CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E43CC0: 4BF13251  bl 0x82d56f10
	ctx.lr = 0x82E43CC4;
	sub_82D56F10(ctx, base);
	pc = 0x82E43CC4; continue 'dispatch;
            }
            0x82E43CC4 => {
    //   block [0x82E43CC4..0x82E440E0)
	// 82E43CC4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E43CC8: D3E100F0  stfs f31, 0xf0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 82E43CCC: D3C100F8  stfs f30, 0xf8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 82E43CD0: FDA0F850  fneg f13, f31
	ctx.f[13].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E43CD4: D3C100A4  stfs f30, 0xa4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82E43CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E43CDC: D3E100A8  stfs f31, 0xa8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82E43CE0: FD80F050  fneg f12, f30
	ctx.f[12].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E43CE4: D3C10110  stfs f30, 0x110(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 82E43CE8: C00A0C18  lfs f0, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E43CEC: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82E43CF0: D00100F4  stfs f0, 0xf4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 82E43CF4: D00100FC  stfs f0, 0xfc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 82E43CF8: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82E43CFC: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82E43D00: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E43D04: 394100F0  addi r10, r1, 0xf0
	ctx.r[10].s64 = ctx.r[1].s64 + 240;
	// 82E43D08: D3E10114  stfs f31, 0x114(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 82E43D0C: D0010118  stfs f0, 0x118(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 82E43D10: D001011C  stfs f0, 0x11c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 82E43D14: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E440E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E440E0 size=4
    let mut pc: u32 = 0x82E440E0;
    'dispatch: loop {
        match pc {
            0x82E440E0 => {
    //   block [0x82E440E0..0x82E440E4)
	// 82E440E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E440E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E440E8 size=136
    let mut pc: u32 = 0x82E440E8;
    'dispatch: loop {
        match pc {
            0x82E440E8 => {
    //   block [0x82E440E8..0x82E44170)
	// 82E440E8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E440EC: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E440F0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E440F4: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E440F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E440FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44170 size=572
    let mut pc: u32 = 0x82E44170;
    'dispatch: loop {
        match pc {
            0x82E44170 => {
    //   block [0x82E44170..0x82E441F0)
	// 82E44170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44174: 4BE65281  bl 0x82ca93f4
	ctx.lr = 0x82E44178;
	sub_82CA93D0(ctx, base);
	// 82E44178: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4417C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44180: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E44184: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4418C: 419A0218  beq cr6, 0x82e443a4
	if ctx.cr[6].eq {
	pc = 0x82E443A4; continue 'dispatch;
	}
	// 82E44190: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44194: 3AEBFFFF  addi r23, r11, -1
	ctx.r[23].s64 = ctx.r[11].s64 + -1;
	// 82E44198: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82E4419C: 41980208  blt cr6, 0x82e443a4
	if ctx.cr[6].lt {
	pc = 0x82E443A4; continue 'dispatch;
	}
	// 82E441A0: 56EB083C  slwi r11, r23, 1
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E441A4: 7D775A14  add r11, r23, r11
	ctx.r[11].u64 = ctx.r[23].u64 + ctx.r[11].u64;
	// 82E441A8: 5578103A  slwi r24, r11, 2
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82E441AC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E441B0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E441B4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E441B8: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E441BC: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E441C0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E441C4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E441C8: 7D6AC214  add r11, r10, r24
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82E441CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E441D0: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E441D4: 555B2036  slwi r27, r10, 4
	ctx.r[27].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82E441D8: 832B0008  lwz r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E441DC: 7F89DA14  add r28, r9, r27
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[27].u64;
	// 82E441E0: 409A0010  bne cr6, 0x82e441f0
	if !ctx.cr[6].eq {
	pc = 0x82E441F0; continue 'dispatch;
	}
	// 82E441E4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E441E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E441EC: 4BF12DAD  bl 0x82d56f98
	ctx.lr = 0x82E441F0;
	sub_82D56F98(ctx, base);
	pc = 0x82E441F0; continue 'dispatch;
            }
            0x82E441F0 => {
    //   block [0x82E441F0..0x82E443A4)
	// 82E441F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E441F4: 57BA2036  slwi r26, r29, 4
	ctx.r[26].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82E441F8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E441FC: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E44200: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E44204: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E44208: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82E443A4; continue 'dispatch;
            }
            0x82E443A4 => {
    //   block [0x82E443A4..0x82E443AC)
	// 82E443A4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E443A8: 4BE6509C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E443B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E443B0 size=580
    let mut pc: u32 = 0x82E443B0;
    'dispatch: loop {
        match pc {
            0x82E443B0 => {
    //   block [0x82E443B0..0x82E44468)
	// 82E443B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E443B4: 4BE65045  bl 0x82ca93f8
	ctx.lr = 0x82E443B8;
	sub_82CA93D0(ctx, base);
	// 82E443B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E443BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E443C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E443C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E443C8: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82E443CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E443D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E443D4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82E443D8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E443DC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E443E0: 4BF19B31  bl 0x82d5df10
	ctx.lr = 0x82E443E4;
	sub_82D5DF10(ctx, base);
	// 82E443E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E443E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E443EC: 4BF19E3D  bl 0x82d5e228
	ctx.lr = 0x82E443F0;
	sub_82D5E228(ctx, base);
	// 82E443F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E443F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E443F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E443FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44404: 4E800421  bctrl
	ctx.lr = 0x82E44408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44408: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4440C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44410: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44414: 419A01D0  beq cr6, 0x82e445e4
	if ctx.cr[6].eq {
	pc = 0x82E445E4; continue 'dispatch;
	}
	// 82E44418: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4441C: 4BF19E0D  bl 0x82d5e228
	ctx.lr = 0x82E44420;
	sub_82D5E228(ctx, base);
	// 82E44420: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44424: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4442C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44434: 4E800421  bctrl
	ctx.lr = 0x82E44438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44438: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4443C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44444: 419A01A0  beq cr6, 0x82e445e4
	if ctx.cr[6].eq {
	pc = 0x82E445E4; continue 'dispatch;
	}
	// 82E44448: 835D0004  lwz r26, 4(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4444C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E44450: 4BF19D79  bl 0x82d5e1c8
	ctx.lr = 0x82E44454;
	sub_82D5E1C8(ctx, base);
	// 82E44454: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82E44458: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4445C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E44460: 419A0064  beq cr6, 0x82e444c4
	if ctx.cr[6].eq {
	pc = 0x82E444C4; continue 'dispatch;
	}
	// 82E44464: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
            }
            0x82E44468 => {
    //   block [0x82E44468..0x82E444C4)
	// 82E44468: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4446C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44470: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44474: 4BF19D25  bl 0x82d5e198
	ctx.lr = 0x82E44478;
	sub_82D5E198(ctx, base);
	// 82E44478: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4447C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44480: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E44484: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44488: 4BF19D11  bl 0x82d5e198
	ctx.lr = 0x82E4448C;
	sub_82D5E198(ctx, base);
	// 82E4448C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44490: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44498: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4449C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E444A0: 4E800421  bctrl
	ctx.lr = 0x82E444A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E444A4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E444A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E444AC: 419A0134  beq cr6, 0x82e445e0
	if ctx.cr[6].eq {
	pc = 0x82E445E0; continue 'dispatch;
	}
	// 82E444B0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E444B4: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82E444B8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E444BC: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E444C0: 4198FFA8  blt cr6, 0x82e44468
	if ctx.cr[6].lt {
	pc = 0x82E44468; continue 'dispatch;
	}
            }
            0x82E444C4 => {
    //   block [0x82E444C4..0x82E444E8)
	// 82E444C4: 835D0010  lwz r26, 0x10(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E444C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E444CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E444D0: 4BF19CF9  bl 0x82d5e1c8
	ctx.lr = 0x82E444D4;
	sub_82D5E1C8(ctx, base);
	// 82E444D4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E444D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E444DC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E444E0: 419A0078  beq cr6, 0x82e44558
	if ctx.cr[6].eq {
	pc = 0x82E44558; continue 'dispatch;
	}
	// 82E444E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82E444E8; continue 'dispatch;
            }
            0x82E444E8 => {
    //   block [0x82E444E8..0x82E44558)
	// 82E444E8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E444EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E444F0: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E444F4: 4BF19CA5  bl 0x82d5e198
	ctx.lr = 0x82E444F8;
	sub_82D5E198(ctx, base);
	// 82E444F8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E444FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44500: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E44504: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44508: 4BF19D21  bl 0x82d5e228
	ctx.lr = 0x82E4450C;
	sub_82D5E228(ctx, base);
	// 82E4450C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44510: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44514: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E44518: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4451C: 4BF19D0D  bl 0x82d5e228
	ctx.lr = 0x82E44520;
	sub_82D5E228(ctx, base);
	// 82E44520: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44524: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4452C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44534: 4E800421  bctrl
	ctx.lr = 0x82E44538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44538: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4453C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44540: 419A00A0  beq cr6, 0x82e445e0
	if ctx.cr[6].eq {
	pc = 0x82E445E0; continue 'dispatch;
	}
	// 82E44544: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E44548: 3B7B0014  addi r27, r27, 0x14
	ctx.r[27].s64 = ctx.r[27].s64 + 20;
	// 82E4454C: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82E44550: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E44554: 4198FF94  blt cr6, 0x82e444e8
	if ctx.cr[6].lt {
	pc = 0x82E444E8; continue 'dispatch;
	}
            }
            0x82E44558 => {
    //   block [0x82E44558..0x82E445E0)
	// 82E44558: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E4455C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44560: 4BF19C69  bl 0x82d5e1c8
	ctx.lr = 0x82E44564;
	sub_82D5E1C8(ctx, base);
	// 82E44564: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44568: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4456C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44570: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44578: 4E800421  bctrl
	ctx.lr = 0x82E4457C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4457C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44584: 419A005C  beq cr6, 0x82e445e0
	if ctx.cr[6].eq {
	pc = 0x82E445E0; continue 'dispatch;
	}
	// 82E44588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4458C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82E44590: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E44594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44598: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4459C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E445A0: 4E800421  bctrl
	ctx.lr = 0x82E445A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E445A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E445A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E445AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E445B0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E445B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E445B8: 4E800421  bctrl
	ctx.lr = 0x82E445BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E445BC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E445C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E445C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E445C8: 419A001C  beq cr6, 0x82e445e4
	if ctx.cr[6].eq {
	pc = 0x82E445E4; continue 'dispatch;
	}
	// 82E445CC: 4BF19AED  bl 0x82d5e0b8
	ctx.lr = 0x82E445D0;
	sub_82D5E0B8(ctx, base);
	// 82E445D0: 7D7BCA14  add r11, r27, r25
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82E445D4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E445D8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E445DC: 4BE64E6C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E445E0 => {
    //   block [0x82E445E0..0x82E445E4)
	// 82E445E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	pc = 0x82E445E4; continue 'dispatch;
            }
            0x82E445E4 => {
    //   block [0x82E445E4..0x82E445F4)
	// 82E445E4: 4BF19AD5  bl 0x82d5e0b8
	ctx.lr = 0x82E445E8;
	sub_82D5E0B8(ctx, base);
	// 82E445E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E445EC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E445F0: 4BE64E58  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E445F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E445F8 size=1132
    let mut pc: u32 = 0x82E445F8;
    'dispatch: loop {
        match pc {
            0x82E445F8 => {
    //   block [0x82E445F8..0x82E44710)
	// 82E445F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E445FC: 4BE64DFD  bl 0x82ca93f8
	ctx.lr = 0x82E44600;
	sub_82CA93D0(ctx, base);
	// 82E44600: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44608: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E4460C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E44610: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E44614: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E44618: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4461C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E44620: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E44624: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4462C: 4E800421  bctrl
	ctx.lr = 0x82E44630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4463C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44644: 4E800421  bctrl
	ctx.lr = 0x82E44648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44648: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4464C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44650: 419A0388  beq cr6, 0x82e449d8
	if ctx.cr[6].eq {
	pc = 0x82E449D8; continue 'dispatch;
	}
	// 82E44654: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44658: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E4465C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44664: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4466C: 4E800421  bctrl
	ctx.lr = 0x82E44670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44674: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4467C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44684: 4E800421  bctrl
	ctx.lr = 0x82E44688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44688: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4468C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44690: 419A0348  beq cr6, 0x82e449d8
	if ctx.cr[6].eq {
	pc = 0x82E449D8; continue 'dispatch;
	}
	// 82E44694: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44698: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E4469C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E446A0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E446A4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E446A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E446AC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E446B0: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 82E446B4: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82E446B8: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E446BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E446C0: 4E800421  bctrl
	ctx.lr = 0x82E446C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E446C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E446C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E446CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E446D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E446D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E446D8: 4E800421  bctrl
	ctx.lr = 0x82E446DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E446DC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E446E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E446E4: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E446E8: 419A02C8  beq cr6, 0x82e449b0
	if ctx.cr[6].eq {
	pc = 0x82E449B0; continue 'dispatch;
	}
	// 82E446EC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E446F0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E446F4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E446F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E446FC: 40980028  bge cr6, 0x82e44724
	if !ctx.cr[6].lt {
	pc = 0x82E44724; continue 'dispatch;
	}
	// 82E44700: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E44704: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E44708: 40980008  bge cr6, 0x82e44710
	if !ctx.cr[6].lt {
	pc = 0x82E44710; continue 'dispatch;
	}
	// 82E4470C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
            }
            0x82E44710 => {
    //   block [0x82E44710..0x82E44724)
	// 82E44710: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E44714: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E44718: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E4471C: 4BF127F5  bl 0x82d56f10
	ctx.lr = 0x82E44720;
	sub_82D56F10(ctx, base);
	// 82E44720: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	pc = 0x82E44724; continue 'dispatch;
            }
            0x82E44724 => {
    //   block [0x82E44724..0x82E44738)
	// 82E44724: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E44728: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E4472C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44730: 419A0080  beq cr6, 0x82e447b0
	if ctx.cr[6].eq {
	pc = 0x82E447B0; continue 'dispatch;
	}
	// 82E44734: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	pc = 0x82E44738; continue 'dispatch;
            }
            0x82E44738 => {
    //   block [0x82E44738..0x82E447B0)
	// 82E44738: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4473C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E44740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44744: 7FDD5A14  add r30, r29, r11
	ctx.r[30].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E44748: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4474C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44750: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44758: 4E800421  bctrl
	ctx.lr = 0x82E4475C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4475C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44760: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E44764: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82E44768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4476C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44774: 4E800421  bctrl
	ctx.lr = 0x82E44778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44778: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4477C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44784: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4478C: 4E800421  bctrl
	ctx.lr = 0x82E44790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44790: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44794: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44798: 419A0214  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E4479C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E447A0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E447A4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82E447A8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E447AC: 4198FF8C  blt cr6, 0x82e44738
	if ctx.cr[6].lt {
	pc = 0x82E44738; continue 'dispatch;
	}
            }
            0x82E447B0 => {
    //   block [0x82E447B0..0x82E44818)
	// 82E447B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E447B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E447B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E447BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E447C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E447C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E447C8: 4E800421  bctrl
	ctx.lr = 0x82E447CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E447CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E447D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E447D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E447D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E447DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E447E0: 4E800421  bctrl
	ctx.lr = 0x82E447E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E447E4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E447E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E447EC: 419A01C0  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E447F0: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E447F4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E447F8: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E447FC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E44800: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44804: 40980028  bge cr6, 0x82e4482c
	if !ctx.cr[6].lt {
	pc = 0x82E4482C; continue 'dispatch;
	}
	// 82E44808: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4480C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E44810: 40980008  bge cr6, 0x82e44818
	if !ctx.cr[6].lt {
	pc = 0x82E44818; continue 'dispatch;
	}
	// 82E44814: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
            }
            0x82E44818 => {
    //   block [0x82E44818..0x82E4482C)
	// 82E44818: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E4481C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E44820: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E44824: 4BF126ED  bl 0x82d56f10
	ctx.lr = 0x82E44828;
	sub_82D56F10(ctx, base);
	// 82E44828: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	pc = 0x82E4482C; continue 'dispatch;
            }
            0x82E4482C => {
    //   block [0x82E4482C..0x82E44840)
	// 82E4482C: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E44830: 93DB0004  stw r30, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E44834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44838: 419A009C  beq cr6, 0x82e448d4
	if ctx.cr[6].eq {
	pc = 0x82E448D4; continue 'dispatch;
	}
	// 82E4483C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	pc = 0x82E44840; continue 'dispatch;
            }
            0x82E44840 => {
    //   block [0x82E44840..0x82E448D4)
	// 82E44840: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44844: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E44848: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4484C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44850: 7FDD5A14  add r30, r29, r11
	ctx.r[30].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E44854: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44858: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4485C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44860: 4E800421  bctrl
	ctx.lr = 0x82E44864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44864: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44868: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E4486C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82E44870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44874: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4487C: 4E800421  bctrl
	ctx.lr = 0x82E44880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44880: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44884: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E44888: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E4488C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44890: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44898: 4E800421  bctrl
	ctx.lr = 0x82E4489C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4489C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E448A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E448A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E448AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E448B0: 4E800421  bctrl
	ctx.lr = 0x82E448B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E448B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E448BC: 419A00F0  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E448C0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E448C4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E448C8: 3BBD0018  addi r29, r29, 0x18
	ctx.r[29].s64 = ctx.r[29].s64 + 24;
	// 82E448CC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E448D0: 4198FF70  blt cr6, 0x82e44840
	if ctx.cr[6].lt {
	pc = 0x82E44840; continue 'dispatch;
	}
            }
            0x82E448D4 => {
    //   block [0x82E448D4..0x82E449A0)
	// 82E448D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E448DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E448E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E448E4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E448E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E448EC: 4E800421  bctrl
	ctx.lr = 0x82E448F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E448F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E448F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E448FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44904: 4E800421  bctrl
	ctx.lr = 0x82E44908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44908: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4490C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44910: 419A009C  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E44914: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44918: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82E4491C: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E44920: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44924: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44928: 4BF10B51  bl 0x82d55478
	ctx.lr = 0x82E4492C;
	sub_82D55478(ctx, base);
	// 82E4492C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44930: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44934: 80BA0000  lwz r5, 0(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4493C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44940: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44948: 4E800421  bctrl
	ctx.lr = 0x82E4494C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4494C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44950: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44954: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44958: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4495C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44960: 4E800421  bctrl
	ctx.lr = 0x82E44964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44964: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4496C: 409A0078  bne cr6, 0x82e449e4
	if !ctx.cr[6].eq {
	pc = 0x82E449E4; continue 'dispatch;
	}
	// 82E44970: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44974: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44978: 4BF10B59  bl 0x82d554d0
	ctx.lr = 0x82E4497C;
	sub_82D554D0(ctx, base);
	// 82E4497C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E44980: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E44984: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E44988: 409A0018  bne cr6, 0x82e449a0
	if !ctx.cr[6].eq {
	pc = 0x82E449A0; continue 'dispatch;
	}
	// 82E4498C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E44990: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44994: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E44998: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4499C: 4BF1092D  bl 0x82d552c8
	ctx.lr = 0x82E449A0;
	sub_82D552C8(ctx, base);
            }
            0x82E449A0 => {
    //   block [0x82E449A0..0x82E449AC)
	// 82E449A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E449A4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E449A8: 4BE64AA0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E449AC => {
    //   block [0x82E449AC..0x82E449B0)
	// 82E449AC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	pc = 0x82E449B0; continue 'dispatch;
            }
            0x82E449B0 => {
    //   block [0x82E449B0..0x82E449D8)
	// 82E449B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E449B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E449B8: 409A0020  bne cr6, 0x82e449d8
	if !ctx.cr[6].eq {
	pc = 0x82E449D8; continue 'dispatch;
	}
	// 82E449BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E449C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E449C4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E449C8: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E449CC: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E449D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E449D4: 4BF108F5  bl 0x82d552c8
	ctx.lr = 0x82E449D8;
	sub_82D552C8(ctx, base);
	pc = 0x82E449D8; continue 'dispatch;
            }
            0x82E449D8 => {
    //   block [0x82E449D8..0x82E449E4)
	// 82E449D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E449DC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E449E0: 4BE64A68  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E449E4 => {
    //   block [0x82E449E4..0x82E449FC)
	// 82E449E4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E449E8: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82E449EC: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E449F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E449F4: 419A003C  beq cr6, 0x82e44a30
	if ctx.cr[6].eq {
	pc = 0x82E44A30; continue 'dispatch;
	}
	// 82E449F8: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	pc = 0x82E449FC; continue 'dispatch;
            }
            0x82E449FC => {
    //   block [0x82E449FC..0x82E44A1C)
	// 82E449FC: 7D69202E  lwzx r11, r9, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E44A00: 7D492214  add r10, r9, r4
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82E44A04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44A08: 41980014  blt cr6, 0x82e44a1c
	if ctx.cr[6].lt {
	pc = 0x82E44A1C; continue 'dispatch;
	}
	// 82E44A0C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44A10: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E44A14: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82E44A18: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	pc = 0x82E44A1C; continue 'dispatch;
            }
            0x82E44A1C => {
    //   block [0x82E44A1C..0x82E44A30)
	// 82E44A1C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E44A20: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E44A24: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82E44A28: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E44A2C: 4198FFD0  blt cr6, 0x82e449fc
	if ctx.cr[6].lt {
	pc = 0x82E449FC; continue 'dispatch;
	}
	pc = 0x82E44A30; continue 'dispatch;
            }
            0x82E44A30 => {
    //   block [0x82E44A30..0x82E44A58)
	// 82E44A30: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E44A34: EBE10060  ld r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E44A38: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E44A3C: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E44A40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E44A44: 409A0014  bne cr6, 0x82e44a58
	if !ctx.cr[6].eq {
	pc = 0x82E44A58; continue 'dispatch;
	}
	// 82E44A48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E44A4C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44A50: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E44A54: 4BF10875  bl 0x82d552c8
	ctx.lr = 0x82E44A58;
	sub_82D552C8(ctx, base);
	pc = 0x82E44A58; continue 'dispatch;
            }
            0x82E44A58 => {
    //   block [0x82E44A58..0x82E44A64)
	// 82E44A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44A5C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E44A60: 4BE649E8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44A68 size=584
    let mut pc: u32 = 0x82E44A68;
    'dispatch: loop {
        match pc {
            0x82E44A68 => {
    //   block [0x82E44A68..0x82E44B4C)
	// 82E44A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44A6C: 4BE64989  bl 0x82ca93f4
	ctx.lr = 0x82E44A70;
	sub_82CA93D0(ctx, base);
	// 82E44A70: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44A74: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E44A78: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E44A7C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E44A80: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82E44A84: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E44A88: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E44A8C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82E44A90: 4828F9F9  bl 0x830d4488
	ctx.lr = 0x82E44A94;
	sub_830D4488(ctx, base);
	// 82E44A94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E44A98: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44A9C: 3B2B3BA8  addi r25, r11, 0x3ba8
	ctx.r[25].s64 = ctx.r[11].s64 + 15272;
	// 82E44AA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E44AA4: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 82E44AA8: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82E44AAC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E44AB0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E44AB4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E44AB8: B1610086  sth r11, 0x86(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[11].u16 ) };
	// 82E44ABC: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82E44AC0: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82E44AC4: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E44AC8: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82E44ACC: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82E44AD0: 4BF13AC9  bl 0x82d58598
	ctx.lr = 0x82E44AD4;
	sub_82D58598(ctx, base);
	// 82E44AD4: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82E44AD8: 93E100A0  stw r31, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u32 ) };
	// 82E44ADC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44AE0: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 82E44AE4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E44AE8: 93A100A8  stw r29, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82E44AEC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E44AF0: 93E100AC  stw r31, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82E44AF4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E44AF8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E44AFC: 93A100B4  stw r29, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 82E44B00: 93E100B8  stw r31, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 82E44B04: 93E100BC  stw r31, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[31].u32 ) };
	// 82E44B08: 93A100C0  stw r29, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 82E44B0C: 93E100C4  stw r31, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 82E44B10: 93E100C8  stw r31, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 82E44B14: 93A100CC  stw r29, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 82E44B18: 93E100D0  stw r31, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[31].u32 ) };
	// 82E44B1C: 4828FB5D  bl 0x830d4678
	ctx.lr = 0x82E44B20;
	sub_830D4678(ctx, base);
	// 82E44B20: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44B24: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44B28: 812100B0  lwz r9, 0xb0(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E44B2C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E44B30: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E44B34: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44B38: 40980024  bge cr6, 0x82e44b5c
	if !ctx.cr[6].lt {
	pc = 0x82E44B5C; continue 'dispatch;
	}
	// 82E44B3C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E44B40: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E44B44: 40980008  bge cr6, 0x82e44b4c
	if !ctx.cr[6].lt {
	pc = 0x82E44B4C; continue 'dispatch;
	}
	// 82E44B48: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82E44B4C; continue 'dispatch;
            }
            0x82E44B4C => {
    //   block [0x82E44B4C..0x82E44B5C)
	// 82E44B4C: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E44B50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E44B54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E44B58: 4BF123B9  bl 0x82d56f10
	ctx.lr = 0x82E44B5C;
	sub_82D56F10(ctx, base);
	pc = 0x82E44B5C; continue 'dispatch;
            }
            0x82E44B5C => {
    //   block [0x82E44B5C..0x82E44B6C)
	// 82E44B5C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E44B60: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82E44B64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44B68: 40990084  ble cr6, 0x82e44bec
	if !ctx.cr[6].gt {
	pc = 0x82E44BEC; continue 'dispatch;
	}
	pc = 0x82E44B6C; continue 'dispatch;
            }
            0x82E44B6C => {
    //   block [0x82E44B6C..0x82E44B8C)
	// 82E44B6C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44B70: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44B74: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E44B78: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44B7C: 409A0010  bne cr6, 0x82e44b8c
	if !ctx.cr[6].eq {
	pc = 0x82E44B8C; continue 'dispatch;
	}
	// 82E44B80: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82E44B84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E44B88: 4BF12411  bl 0x82d56f98
	ctx.lr = 0x82E44B8C;
	sub_82D56F98(ctx, base);
	pc = 0x82E44B8C; continue 'dispatch;
            }
            0x82E44B8C => {
    //   block [0x82E44B8C..0x82E44BEC)
	// 82E44B8C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44B90: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E44B94: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44B98: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E44B9C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E44BA0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E44BA4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E44BA8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E44BAC: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E44BB0: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E44BB4: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E44BB8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E44BBC: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E44BC0: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E44BC4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44BC8: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82E44BCC: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E44BD0: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E44BD4: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82E44BD8: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44BDC: F94B0010  std r10, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82E44BE0: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E44BE4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44BE8: 4198FF84  blt cr6, 0x82e44b6c
	if ctx.cr[6].lt {
	pc = 0x82E44B6C; continue 'dispatch;
	}
	pc = 0x82E44BEC; continue 'dispatch;
            }
            0x82E44BEC => {
    //   block [0x82E44BEC..0x82E44C64)
	// 82E44BEC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82E44BF0: 893B0001  lbz r9, 1(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E44BF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E44BF8: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E44BFC: 394A7D58  addi r10, r10, 0x7d58
	ctx.r[10].s64 = ctx.r[10].s64 + 32088;
	// 82E44C00: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E44C04: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82E44C08: 7B450020  clrldi r5, r26, 0x20
	ctx.r[5].u64 = ctx.r[26].u64 & 0x00000000FFFFFFFFu64;
	// 82E44C0C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82E44C10: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E44C14: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E44C18: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E44C1C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E44C20: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E44C24: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82E44C28: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82E44C2C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44C30: 4BFFF781  bl 0x82e443b0
	ctx.lr = 0x82E44C34;
	sub_82E443B0(ctx, base);
	// 82E44C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44C38: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82E44C3C: 4BF309ED  bl 0x82d75628
	ctx.lr = 0x82E44C40;
	sub_82D75628(ctx, base);
	// 82E44C40: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E44C44: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82E44C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44C4C: 409A0018  bne cr6, 0x82e44c64
	if !ctx.cr[6].eq {
	pc = 0x82E44C64; continue 'dispatch;
	}
	// 82E44C50: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E44C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E44C58: 419A000C  beq cr6, 0x82e44c64
	if ctx.cr[6].eq {
	pc = 0x82E44C64; continue 'dispatch;
	}
	// 82E44C5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E44C60: 4BFF2FC1  bl 0x82e37c20
	ctx.lr = 0x82E44C64;
	sub_82E37C20(ctx, base);
	pc = 0x82E44C64; continue 'dispatch;
            }
            0x82E44C64 => {
    //   block [0x82E44C64..0x82E44C9C)
	// 82E44C64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E44C68: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E44C6C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E44C70: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E44C74: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E44C78: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E44C7C: 409A0020  bne cr6, 0x82e44c9c
	if !ctx.cr[6].eq {
	pc = 0x82E44C9C; continue 'dispatch;
	}
	// 82E44C80: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44C84: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E44C88: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E44C8C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E44C90: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E44C94: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44C98: 4BF10631  bl 0x82d552c8
	ctx.lr = 0x82E44C9C;
	sub_82D552C8(ctx, base);
	pc = 0x82E44C9C; continue 'dispatch;
            }
            0x82E44C9C => {
    //   block [0x82E44C9C..0x82E44CB0)
	// 82E44C9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E44CA0: 4828F949  bl 0x830d45e8
	ctx.lr = 0x82E44CA4;
	sub_830D45E8(ctx, base);
	// 82E44CA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44CA8: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82E44CAC: 4BE64798  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44CB0 size=168
    let mut pc: u32 = 0x82E44CB0;
    'dispatch: loop {
        match pc {
            0x82E44CB0 => {
    //   block [0x82E44CB0..0x82E44CE8)
	// 82E44CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E44CB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E44CBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E44CC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44CC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E44CCC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44CD0: 4B33FBC1  bl 0x82184890
	ctx.lr = 0x82E44CD4;
	sub_82184890(ctx, base);
	// 82E44CD4: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E44CDC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E44CE0: 40990050  ble cr6, 0x82e44d30
	if !ctx.cr[6].gt {
	pc = 0x82E44D30; continue 'dispatch;
	}
	// 82E44CE4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82E44CE8; continue 'dispatch;
            }
            0x82E44CE8 => {
    //   block [0x82E44CE8..0x82E44D08)
	// 82E44CE8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44CEC: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E44CF0: 419A0018  beq cr6, 0x82e44d08
	if ctx.cr[6].eq {
	pc = 0x82E44D08; continue 'dispatch;
	}
	// 82E44CF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E44CF8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E44CFC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E44D00: 4198FFE8  blt cr6, 0x82e44ce8
	if ctx.cr[6].lt {
	pc = 0x82E44CE8; continue 'dispatch;
	}
	// 82E44D04: 4800002C  b 0x82e44d30
	pc = 0x82E44D30; continue 'dispatch;
            }
            0x82E44D08 => {
    //   block [0x82E44D08..0x82E44D30)
	// 82E44D08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44D0C: 41980024  blt cr6, 0x82e44d30
	if ctx.cr[6].lt {
	pc = 0x82E44D30; continue 'dispatch;
	}
	// 82E44D10: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44D14: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E44D18: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44D1C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E44D20: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E44D24: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E44D28: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44D2C: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	pc = 0x82E44D30; continue 'dispatch;
            }
            0x82E44D30 => {
    //   block [0x82E44D30..0x82E44D58)
	// 82E44D30: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44D34: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44D38: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44D3C: 48474C19  bl 0x832b9954
	ctx.lr = 0x82E44D40;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44D40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E44D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E44D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E44D4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E44D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E44D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44D58 size=72
    let mut pc: u32 = 0x82E44D58;
    'dispatch: loop {
        match pc {
            0x82E44D58 => {
    //   block [0x82E44D58..0x82E44DA0)
	// 82E44D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E44D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E44D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44D6C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44D70: 4B33FB21  bl 0x82184890
	ctx.lr = 0x82E44D74;
	sub_82184890(ctx, base);
	// 82E44D74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E44D78: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44D7C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82E44D80: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E44D84: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82E44D88: 48474BCD  bl 0x832b9954
	ctx.lr = 0x82E44D8C;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44D8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E44D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E44D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E44D98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E44D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44DA0 size=152
    let mut pc: u32 = 0x82E44DA0;
    'dispatch: loop {
        match pc {
            0x82E44DA0 => {
    //   block [0x82E44DA0..0x82E44DE0)
	// 82E44DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44DA4: 4BE64655  bl 0x82ca93f8
	ctx.lr = 0x82E44DA8;
	sub_82CA93D0(ctx, base);
	// 82E44DA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44DAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44DB0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44DB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44DB8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44DBC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E44DC0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44DC4: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82E44DC8: 4B33FAC9  bl 0x82184890
	ctx.lr = 0x82E44DCC;
	sub_82184890(ctx, base);
	// 82E44DCC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44DD0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44DD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44DD8: 40990048  ble cr6, 0x82e44e20
	if !ctx.cr[6].gt {
	pc = 0x82E44E20; continue 'dispatch;
	}
	// 82E44DDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E44DE0; continue 'dispatch;
            }
            0x82E44DE0 => {
    //   block [0x82E44DE0..0x82E44E20)
	// 82E44DE0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44DE4: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82E44DE8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E44DEC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44DF0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44DF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44DF8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44DFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44E00: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44E04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44E08: 4E800421  bctrl
	ctx.lr = 0x82E44E0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44E0C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44E10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44E14: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44E18: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44E1C: 4198FFC4  blt cr6, 0x82e44de0
	if ctx.cr[6].lt {
	pc = 0x82E44DE0; continue 'dispatch;
	}
            }
            0x82E44E20 => {
    //   block [0x82E44E20..0x82E44E38)
	// 82E44E20: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44E24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44E28: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44E2C: 48474B29  bl 0x832b9954
	ctx.lr = 0x82E44E30;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44E30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E44E34: 4BE64614  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44E38 size=152
    let mut pc: u32 = 0x82E44E38;
    'dispatch: loop {
        match pc {
            0x82E44E38 => {
    //   block [0x82E44E38..0x82E44E78)
	// 82E44E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44E3C: 4BE645BD  bl 0x82ca93f8
	ctx.lr = 0x82E44E40;
	sub_82CA93D0(ctx, base);
	// 82E44E40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44E44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44E48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44E4C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44E50: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44E54: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E44E58: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44E5C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82E44E60: 4B33FA31  bl 0x82184890
	ctx.lr = 0x82E44E64;
	sub_82184890(ctx, base);
	// 82E44E64: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44E68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44E6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44E70: 40990048  ble cr6, 0x82e44eb8
	if !ctx.cr[6].gt {
	pc = 0x82E44EB8; continue 'dispatch;
	}
	// 82E44E74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E44E78; continue 'dispatch;
            }
            0x82E44E78 => {
    //   block [0x82E44E78..0x82E44EB8)
	// 82E44E78: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44E7C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82E44E80: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E44E84: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44E88: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44E8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44E90: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44E94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44E98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44EA0: 4E800421  bctrl
	ctx.lr = 0x82E44EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44EA4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44EA8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44EAC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44EB0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44EB4: 4198FFC4  blt cr6, 0x82e44e78
	if ctx.cr[6].lt {
	pc = 0x82E44E78; continue 'dispatch;
	}
            }
            0x82E44EB8 => {
    //   block [0x82E44EB8..0x82E44ED0)
	// 82E44EB8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44EBC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44EC0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44EC4: 48474A91  bl 0x832b9954
	ctx.lr = 0x82E44EC8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44EC8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E44ECC: 4BE6457C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44ED0 size=136
    let mut pc: u32 = 0x82E44ED0;
    'dispatch: loop {
        match pc {
            0x82E44ED0 => {
    //   block [0x82E44ED0..0x82E44F08)
	// 82E44ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44ED4: 4BE6452D  bl 0x82ca9400
	ctx.lr = 0x82E44ED8;
	sub_82CA93D0(ctx, base);
	// 82E44ED8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44EE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44EE4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44EE8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44EEC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44EF0: 4B33F9A1  bl 0x82184890
	ctx.lr = 0x82E44EF4;
	sub_82184890(ctx, base);
	// 82E44EF4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44EF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44EFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44F00: 40990040  ble cr6, 0x82e44f40
	if !ctx.cr[6].gt {
	pc = 0x82E44F40; continue 'dispatch;
	}
	// 82E44F04: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E44F08; continue 'dispatch;
            }
            0x82E44F08 => {
    //   block [0x82E44F08..0x82E44F40)
	// 82E44F08: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44F0C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44F10: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44F14: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44F18: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44F1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44F20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44F24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44F28: 4E800421  bctrl
	ctx.lr = 0x82E44F2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44F2C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44F30: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44F34: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44F38: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44F3C: 4198FFCC  blt cr6, 0x82e44f08
	if ctx.cr[6].lt {
	pc = 0x82E44F08; continue 'dispatch;
	}
            }
            0x82E44F40 => {
    //   block [0x82E44F40..0x82E44F58)
	// 82E44F40: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44F44: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44F48: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44F4C: 48474A09  bl 0x832b9954
	ctx.lr = 0x82E44F50;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44F50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E44F54: 4BE644FC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44F58 size=136
    let mut pc: u32 = 0x82E44F58;
    'dispatch: loop {
        match pc {
            0x82E44F58 => {
    //   block [0x82E44F58..0x82E44F90)
	// 82E44F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44F5C: 4BE644A5  bl 0x82ca9400
	ctx.lr = 0x82E44F60;
	sub_82CA93D0(ctx, base);
	// 82E44F60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44F68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44F6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44F70: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44F74: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44F78: 4B33F919  bl 0x82184890
	ctx.lr = 0x82E44F7C;
	sub_82184890(ctx, base);
	// 82E44F7C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44F80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44F84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44F88: 40990040  ble cr6, 0x82e44fc8
	if !ctx.cr[6].gt {
	pc = 0x82E44FC8; continue 'dispatch;
	}
	// 82E44F8C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E44F90; continue 'dispatch;
            }
            0x82E44F90 => {
    //   block [0x82E44F90..0x82E44FC8)
	// 82E44F90: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44F94: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44F98: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44F9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44FA0: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44FA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44FA8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44FB0: 4E800421  bctrl
	ctx.lr = 0x82E44FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44FB4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44FB8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44FBC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44FC0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44FC4: 4198FFCC  blt cr6, 0x82e44f90
	if ctx.cr[6].lt {
	pc = 0x82E44F90; continue 'dispatch;
	}
            }
            0x82E44FC8 => {
    //   block [0x82E44FC8..0x82E44FE0)
	// 82E44FC8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44FCC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44FD0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44FD4: 48474981  bl 0x832b9954
	ctx.lr = 0x82E44FD8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44FD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E44FDC: 4BE64474  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44FE0 size=136
    let mut pc: u32 = 0x82E44FE0;
    'dispatch: loop {
        match pc {
            0x82E44FE0 => {
    //   block [0x82E44FE0..0x82E45018)
	// 82E44FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44FE4: 4BE6441D  bl 0x82ca9400
	ctx.lr = 0x82E44FE8;
	sub_82CA93D0(ctx, base);
	// 82E44FE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44FEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44FF0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44FF4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44FF8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44FFC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45000: 4B33F891  bl 0x82184890
	ctx.lr = 0x82E45004;
	sub_82184890(ctx, base);
	// 82E45004: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45008: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4500C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45010: 40990040  ble cr6, 0x82e45050
	if !ctx.cr[6].gt {
	pc = 0x82E45050; continue 'dispatch;
	}
	// 82E45014: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E45018; continue 'dispatch;
            }
            0x82E45018 => {
    //   block [0x82E45018..0x82E45050)
	// 82E45018: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4501C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45020: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45024: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45028: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4502C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45030: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45038: 4E800421  bctrl
	ctx.lr = 0x82E4503C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4503C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45040: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45044: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45048: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4504C: 4198FFCC  blt cr6, 0x82e45018
	if ctx.cr[6].lt {
	pc = 0x82E45018; continue 'dispatch;
	}
            }
            0x82E45050 => {
    //   block [0x82E45050..0x82E45068)
	// 82E45050: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45054: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45058: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E4505C: 484748F9  bl 0x832b9954
	ctx.lr = 0x82E45060;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45060: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45064: 4BE643EC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45068 size=192
    let mut pc: u32 = 0x82E45068;
    'dispatch: loop {
        match pc {
            0x82E45068 => {
    //   block [0x82E45068..0x82E450BC)
	// 82E45068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4506C: 4BE64391  bl 0x82ca93fc
	ctx.lr = 0x82E45070;
	sub_82CA93D0(ctx, base);
	// 82E45070: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82E45074: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82E45078: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E4507C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45080: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45084: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E45088: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4508C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E45090: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45094: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82E45098: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E4509C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82E450A0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E450A4: 4B33F7ED  bl 0x82184890
	ctx.lr = 0x82E450A8;
	sub_82184890(ctx, base);
	// 82E450A8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E450AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E450B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E450B4: 40990050  ble cr6, 0x82e45104
	if !ctx.cr[6].gt {
	pc = 0x82E45104; continue 'dispatch;
	}
	// 82E450B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E450BC; continue 'dispatch;
            }
            0x82E450BC => {
    //   block [0x82E450BC..0x82E45104)
	// 82E450BC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E450C0: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82E450C4: FC60E890  fmr f3, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82E450C8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E450CC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82E450D0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E450D4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E450D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E450DC: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E450E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E450E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E450E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E450EC: 4E800421  bctrl
	ctx.lr = 0x82E450F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E450F0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E450F4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E450F8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E450FC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45100: 4198FFBC  blt cr6, 0x82e450bc
	if ctx.cr[6].lt {
	pc = 0x82E450BC; continue 'dispatch;
	}
            }
            0x82E45104 => {
    //   block [0x82E45104..0x82E45128)
	// 82E45104: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45108: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E4510C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45110: 48474845  bl 0x832b9954
	ctx.lr = 0x82E45114;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45114: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E45118: CBA1FFA8  lfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82E4511C: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82E45120: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82E45124: 4BE64328  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45128 size=136
    let mut pc: u32 = 0x82E45128;
    'dispatch: loop {
        match pc {
            0x82E45128 => {
    //   block [0x82E45128..0x82E45160)
	// 82E45128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4512C: 4BE642D5  bl 0x82ca9400
	ctx.lr = 0x82E45130;
	sub_82CA93D0(ctx, base);
	// 82E45130: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45134: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45138: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4513C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45140: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45144: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45148: 4B33F749  bl 0x82184890
	ctx.lr = 0x82E4514C;
	sub_82184890(ctx, base);
	// 82E4514C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45150: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45154: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45158: 40990040  ble cr6, 0x82e45198
	if !ctx.cr[6].gt {
	pc = 0x82E45198; continue 'dispatch;
	}
	// 82E4515C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E45160; continue 'dispatch;
            }
            0x82E45160 => {
    //   block [0x82E45160..0x82E45198)
	// 82E45160: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45164: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45168: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E4516C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45170: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45178: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E4517C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45180: 4E800421  bctrl
	ctx.lr = 0x82E45184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45184: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45188: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E4518C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45190: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45194: 4198FFCC  blt cr6, 0x82e45160
	if ctx.cr[6].lt {
	pc = 0x82E45160; continue 'dispatch;
	}
            }
            0x82E45198 => {
    //   block [0x82E45198..0x82E451B0)
	// 82E45198: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4519C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E451A0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E451A4: 484747B1  bl 0x832b9954
	ctx.lr = 0x82E451A8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E451A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E451AC: 4BE642A4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E451B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E451B0 size=144
    let mut pc: u32 = 0x82E451B0;
    'dispatch: loop {
        match pc {
            0x82E451B0 => {
    //   block [0x82E451B0..0x82E451EC)
	// 82E451B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E451B4: 4BE64249  bl 0x82ca93fc
	ctx.lr = 0x82E451B8;
	sub_82CA93D0(ctx, base);
	// 82E451B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E451BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E451C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E451C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E451C8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E451CC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E451D0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E451D4: 4B33F6BD  bl 0x82184890
	ctx.lr = 0x82E451D8;
	sub_82184890(ctx, base);
	// 82E451D8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E451DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E451E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E451E4: 40990044  ble cr6, 0x82e45228
	if !ctx.cr[6].gt {
	pc = 0x82E45228; continue 'dispatch;
	}
	// 82E451E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E451EC; continue 'dispatch;
            }
            0x82E451EC => {
    //   block [0x82E451EC..0x82E45228)
	// 82E451EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E451F0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E451F4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E451F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E451FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45200: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45208: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4520C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45210: 4E800421  bctrl
	ctx.lr = 0x82E45214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45214: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45218: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E4521C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45220: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45224: 4198FFC8  blt cr6, 0x82e451ec
	if ctx.cr[6].lt {
	pc = 0x82E451EC; continue 'dispatch;
	}
            }
            0x82E45228 => {
    //   block [0x82E45228..0x82E45240)
	// 82E45228: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4522C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45230: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45234: 48474721  bl 0x832b9954
	ctx.lr = 0x82E45238;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45238: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4523C: 4BE64210  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45240 size=88
    let mut pc: u32 = 0x82E45240;
    'dispatch: loop {
        match pc {
            0x82E45240 => {
    //   block [0x82E45240..0x82E45298)
	// 82E45240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45244: 4BE641C1  bl 0x82ca9404
	ctx.lr = 0x82E45248;
	sub_82CA93D0(ctx, base);
	// 82E45248: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4524C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45250: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E45254: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E45258: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E4525C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E45260: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E45264: 4BF11295  bl 0x82d564f8
	ctx.lr = 0x82E45268;
	sub_82D564F8(ctx, base);
	// 82E45268: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4526C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E45270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E45274: 4BF11285  bl 0x82d564f8
	ctx.lr = 0x82E45278;
	sub_82D564F8(ctx, base);
	// 82E45278: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4527C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E45280: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E45284: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E45288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4528C: 4BFFFF25  bl 0x82e451b0
	ctx.lr = 0x82E45290;
	sub_82E451B0(ctx, base);
	// 82E45290: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E45294: 4BE641C0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45298 size=104
    let mut pc: u32 = 0x82E45298;
    'dispatch: loop {
        match pc {
            0x82E45298 => {
    //   block [0x82E45298..0x82E45300)
	// 82E45298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4529C: 4BE64169  bl 0x82ca9404
	ctx.lr = 0x82E452A0;
	sub_82CA93D0(ctx, base);
	// 82E452A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E452A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E452A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E452AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E452B0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E452B4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E452B8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E452BC: 4BF1604D  bl 0x82d5b308
	ctx.lr = 0x82E452C0;
	sub_82D5B308(ctx, base);
	// 82E452C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E452C4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E452C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E452CC: 4BF1122D  bl 0x82d564f8
	ctx.lr = 0x82E452D0;
	sub_82D564F8(ctx, base);
	// 82E452D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E452D4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E452D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E452DC: 4BF1121D  bl 0x82d564f8
	ctx.lr = 0x82E452E0;
	sub_82D564F8(ctx, base);
	// 82E452E0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E452E4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E452E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E452EC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E452F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E452F4: 4BFFFEBD  bl 0x82e451b0
	ctx.lr = 0x82E452F8;
	sub_82E451B0(ctx, base);
	// 82E452F8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E452FC: 4BE64158  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45300 size=136
    let mut pc: u32 = 0x82E45300;
    'dispatch: loop {
        match pc {
            0x82E45300 => {
    //   block [0x82E45300..0x82E45388)
	// 82E45300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45304: 4BE640FD  bl 0x82ca9400
	ctx.lr = 0x82E45308;
	sub_82CA93D0(ctx, base);
	// 82E45308: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4530C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45388 size=164
    let mut pc: u32 = 0x82E45388;
    'dispatch: loop {
        match pc {
            0x82E45388 => {
    //   block [0x82E45388..0x82E4542C)
	// 82E45388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4538C: 4BE64079  bl 0x82ca9404
	ctx.lr = 0x82E45390;
	sub_82CA93D0(ctx, base);
	// 82E45390: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4539C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E453A0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E453A4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E453A8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E453AC: 4BF1114D  bl 0x82d564f8
	ctx.lr = 0x82E453B0;
	sub_82D564F8(ctx, base);
	// 82E453B0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45430 size=76
    let mut pc: u32 = 0x82E45430;
    'dispatch: loop {
        match pc {
            0x82E45430 => {
    //   block [0x82E45430..0x82E4547C)
	// 82E45430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45434: 4BE63FD1  bl 0x82ca9404
	ctx.lr = 0x82E45438;
	sub_82CA93D0(ctx, base);
	// 82E45438: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4543C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E45440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E45444: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E45448: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E4544C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E45450: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E45454: 4BF15EB5  bl 0x82d5b308
	ctx.lr = 0x82E45458;
	sub_82D5B308(ctx, base);
	// 82E45458: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82E4545C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E45460: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E45464: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E45468: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4546C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E45470: 4BFFFF19  bl 0x82e45388
	ctx.lr = 0x82E45474;
	sub_82E45388(ctx, base);
	// 82E45474: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E45478: 4BE63FDC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45480 size=628
    let mut pc: u32 = 0x82E45480;
    'dispatch: loop {
        match pc {
            0x82E45480 => {
    //   block [0x82E45480..0x82E456F4)
	// 82E45480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45484: 4BE63F81  bl 0x82ca9404
	ctx.lr = 0x82E45488;
	sub_82CA93D0(ctx, base);
	// 82E45488: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E456F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E456F8 size=172
    let mut pc: u32 = 0x82E456F8;
    'dispatch: loop {
        match pc {
            0x82E456F8 => {
    //   block [0x82E456F8..0x82E457A4)
	// 82E456F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E456FC: 4BE63D05  bl 0x82ca9400
	ctx.lr = 0x82E45700;
	sub_82CA93D0(ctx, base);
	// 82E45700: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E45704: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E457A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E457A8 size=76
    let mut pc: u32 = 0x82E457A8;
    'dispatch: loop {
        match pc {
            0x82E457A8 => {
    //   block [0x82E457A8..0x82E457F4)
	// 82E457A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E457AC: 4BE63C61  bl 0x82ca940c
	ctx.lr = 0x82E457B0;
	sub_82CA93D0(ctx, base);
	// 82E457B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E457B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E457B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E457BC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E457C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E457C4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E457C8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E457CC: 4BF10D2D  bl 0x82d564f8
	ctx.lr = 0x82E457D0;
	sub_82D564F8(ctx, base);
	// 82E457D0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E457D4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E457D8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E457DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E457E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E457E4: 4BFFFF15  bl 0x82e456f8
	ctx.lr = 0x82E457E8;
	sub_82E456F8(ctx, base);
	// 82E457E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E457EC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E457F0: 4BE63C6C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E457F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E457F8 size=76
    let mut pc: u32 = 0x82E457F8;
    'dispatch: loop {
        match pc {
            0x82E457F8 => {
    //   block [0x82E457F8..0x82E45844)
	// 82E457F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E457FC: 4BE63C11  bl 0x82ca940c
	ctx.lr = 0x82E45800;
	sub_82CA93D0(ctx, base);
	// 82E45800: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E45804: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4580C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E45810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E45814: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E45818: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E4581C: 4BF10B8D  bl 0x82d563a8
	ctx.lr = 0x82E45820;
	sub_82D563A8(ctx, base);
	// 82E45820: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E45824: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E45828: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4582C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E45830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E45834: 4BFFFEC5  bl 0x82e456f8
	ctx.lr = 0x82E45838;
	sub_82E456F8(ctx, base);
	// 82E45838: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4583C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E45840: 4BE63C1C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E45848 size=724
    let mut pc: u32 = 0x82E45848;
    'dispatch: loop {
        match pc {
            0x82E45848 => {
    //   block [0x82E45848..0x82E45B1C)
	// 82E45848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4584C: 4BE63BB5  bl 0x82ca9400
	ctx.lr = 0x82E45850;
	sub_82CA93D0(ctx, base);
	// 82E45850: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45B20 size=136
    let mut pc: u32 = 0x82E45B20;
    'dispatch: loop {
        match pc {
            0x82E45B20 => {
    //   block [0x82E45B20..0x82E45B58)
	// 82E45B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45B24: 4BE638DD  bl 0x82ca9400
	ctx.lr = 0x82E45B28;
	sub_82CA93D0(ctx, base);
	// 82E45B28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45B2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45B30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45B34: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45B38: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45B3C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45B40: 4B33ED51  bl 0x82184890
	ctx.lr = 0x82E45B44;
	sub_82184890(ctx, base);
	// 82E45B44: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45B48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45B4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45B50: 40990040  ble cr6, 0x82e45b90
	if !ctx.cr[6].gt {
	pc = 0x82E45B90; continue 'dispatch;
	}
	// 82E45B54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E45B58; continue 'dispatch;
            }
            0x82E45B58 => {
    //   block [0x82E45B58..0x82E45B90)
	// 82E45B58: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45B5C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45B60: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45B64: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45B68: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45B6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45B70: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E45B74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45B78: 4E800421  bctrl
	ctx.lr = 0x82E45B7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45B7C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45B80: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45B84: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45B88: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45B8C: 4198FFCC  blt cr6, 0x82e45b58
	if ctx.cr[6].lt {
	pc = 0x82E45B58; continue 'dispatch;
	}
            }
            0x82E45B90 => {
    //   block [0x82E45B90..0x82E45BA8)
	// 82E45B90: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45B94: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45B98: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45B9C: 48473DB9  bl 0x832b9954
	ctx.lr = 0x82E45BA0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45BA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45BA4: 4BE638AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45BA8 size=144
    let mut pc: u32 = 0x82E45BA8;
    'dispatch: loop {
        match pc {
            0x82E45BA8 => {
    //   block [0x82E45BA8..0x82E45BE4)
	// 82E45BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45BAC: 4BE63851  bl 0x82ca93fc
	ctx.lr = 0x82E45BB0;
	sub_82CA93D0(ctx, base);
	// 82E45BB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45BB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45BBC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45BC0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45BC4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E45BC8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45BCC: 4B33ECC5  bl 0x82184890
	ctx.lr = 0x82E45BD0;
	sub_82184890(ctx, base);
	// 82E45BD0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45BD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45BD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45BDC: 40990044  ble cr6, 0x82e45c20
	if !ctx.cr[6].gt {
	pc = 0x82E45C20; continue 'dispatch;
	}
	// 82E45BE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E45BE4; continue 'dispatch;
            }
            0x82E45BE4 => {
    //   block [0x82E45BE4..0x82E45C20)
	// 82E45BE4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45BE8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E45BEC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45BF0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45BF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45BF8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45BFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45C00: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E45C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45C08: 4E800421  bctrl
	ctx.lr = 0x82E45C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45C0C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45C10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45C14: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45C18: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45C1C: 4198FFC8  blt cr6, 0x82e45be4
	if ctx.cr[6].lt {
	pc = 0x82E45BE4; continue 'dispatch;
	}
            }
            0x82E45C20 => {
    //   block [0x82E45C20..0x82E45C38)
	// 82E45C20: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45C24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45C28: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45C2C: 48473D29  bl 0x832b9954
	ctx.lr = 0x82E45C30;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45C30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45C34: 4BE63818  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45C38 size=144
    let mut pc: u32 = 0x82E45C38;
    'dispatch: loop {
        match pc {
            0x82E45C38 => {
    //   block [0x82E45C38..0x82E45C74)
	// 82E45C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45C3C: 4BE637C1  bl 0x82ca93fc
	ctx.lr = 0x82E45C40;
	sub_82CA93D0(ctx, base);
	// 82E45C40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45C44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45C48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45C4C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45C50: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45C54: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E45C58: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45C5C: 4B33EC35  bl 0x82184890
	ctx.lr = 0x82E45C60;
	sub_82184890(ctx, base);
	// 82E45C60: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45C64: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45C68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45C6C: 40990044  ble cr6, 0x82e45cb0
	if !ctx.cr[6].gt {
	pc = 0x82E45CB0; continue 'dispatch;
	}
	// 82E45C70: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E45C74; continue 'dispatch;
            }
            0x82E45C74 => {
    //   block [0x82E45C74..0x82E45CB0)
	// 82E45C74: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45C78: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E45C7C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45C80: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45C84: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45C88: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45C90: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E45C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45C98: 4E800421  bctrl
	ctx.lr = 0x82E45C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45C9C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45CA0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45CA4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45CA8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45CAC: 4198FFC8  blt cr6, 0x82e45c74
	if ctx.cr[6].lt {
	pc = 0x82E45C74; continue 'dispatch;
	}
            }
            0x82E45CB0 => {
    //   block [0x82E45CB0..0x82E45CC8)
	// 82E45CB0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45CB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45CB8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45CBC: 48473C99  bl 0x832b9954
	ctx.lr = 0x82E45CC0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45CC0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45CC4: 4BE63788  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45CC8 size=136
    let mut pc: u32 = 0x82E45CC8;
    'dispatch: loop {
        match pc {
            0x82E45CC8 => {
    //   block [0x82E45CC8..0x82E45D00)
	// 82E45CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45CCC: 4BE63735  bl 0x82ca9400
	ctx.lr = 0x82E45CD0;
	sub_82CA93D0(ctx, base);
	// 82E45CD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45CD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45CD8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45CDC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45CE0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45CE4: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45CE8: 4B33EBA9  bl 0x82184890
	ctx.lr = 0x82E45CEC;
	sub_82184890(ctx, base);
	// 82E45CEC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45CF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45CF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45CF8: 40990040  ble cr6, 0x82e45d38
	if !ctx.cr[6].gt {
	pc = 0x82E45D38; continue 'dispatch;
	}
	// 82E45CFC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82E45D00; continue 'dispatch;
            }
            0x82E45D00 => {
    //   block [0x82E45D00..0x82E45D38)
	// 82E45D00: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45D04: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45D08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45D0C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45D10: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45D14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45D18: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E45D1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45D20: 4E800421  bctrl
	ctx.lr = 0x82E45D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45D24: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45D28: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45D2C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45D30: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45D34: 4198FFCC  blt cr6, 0x82e45d00
	if ctx.cr[6].lt {
	pc = 0x82E45D00; continue 'dispatch;
	}
            }
            0x82E45D38 => {
    //   block [0x82E45D38..0x82E45D50)
	// 82E45D38: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45D3C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45D40: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45D44: 48473C11  bl 0x832b9954
	ctx.lr = 0x82E45D48;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45D48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45D4C: 4BE63704  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45D50 size=248
    let mut pc: u32 = 0x82E45D50;
    'dispatch: loop {
        match pc {
            0x82E45D50 => {
    //   block [0x82E45D50..0x82E45DB8)
	// 82E45D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45D54: 4BE636B1  bl 0x82ca9404
	ctx.lr = 0x82E45D58;
	sub_82CA93D0(ctx, base);
	// 82E45D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45D5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E45D60: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45D64: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E45D68: 396B5CCC  addi r11, r11, 0x5ccc
	ctx.r[11].s64 = ctx.r[11].s64 + 23756;
	// 82E45D6C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E45D70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E45D74: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82E45D78: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82E45D7C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E45D80: B13B0006  sth r9, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E45D84: 939B0008  stw r28, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E45D88: 939B000C  stw r28, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E45D8C: 911B0010  stw r8, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E45D90: 7C67502E  lwzx r3, r7, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E45D94: 83A30060  lwz r29, 0x60(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E45D98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E45D9C: 419A001C  beq cr6, 0x82e45db8
	if ctx.cr[6].eq {
	pc = 0x82E45DB8; continue 'dispatch;
	}
	// 82E45DA0: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E45DA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E45DA8: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E45DAC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45DB0: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E45DB4: 48000010  b 0x82e45dc4
	pc = 0x82E45DC4; continue 'dispatch;
            }
            0x82E45DB8 => {
    //   block [0x82E45DB8..0x82E45DC4)
	// 82E45DB8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82E45DBC: 4BF0F295  bl 0x82d55050
	ctx.lr = 0x82E45DC0;
	sub_82D55050(ctx, base);
	// 82E45DC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	pc = 0x82E45DC4; continue 'dispatch;
            }
            0x82E45DC4 => {
    //   block [0x82E45DC4..0x82E45E08)
	// 82E45DC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E45DC8: 419A0070  beq cr6, 0x82e45e38
	if ctx.cr[6].eq {
	pc = 0x82E45E38; continue 'dispatch;
	}
	// 82E45DCC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82E45DD0: 3BFD0028  addi r31, r29, 0x28
	ctx.r[31].s64 = ctx.r[29].s64 + 40;
	// 82E45DD4: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82E45DD8: 3BCBB0A0  addi r30, r11, -0x4f60
	ctx.r[30].s64 = ctx.r[11].s64 + -20320;
	// 82E45DDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E45DE0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E45DE4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E45DE8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E45DEC: 4B33EAA5  bl 0x82184890
	ctx.lr = 0x82E45DF0;
	sub_82184890(ctx, base);
	// 82E45DF0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E45DF4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E45DF8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E45DFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E45E00: 419A0008  beq cr6, 0x82e45e08
	if ctx.cr[6].eq {
	pc = 0x82E45E08; continue 'dispatch;
	}
	// 82E45E04: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	pc = 0x82E45E08; continue 'dispatch;
            }
            0x82E45E08 => {
    //   block [0x82E45E08..0x82E45E38)
	// 82E45E08: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45E0C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E45E10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E45E14: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82E45E18: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45E1C: 48473B39  bl 0x832b9954
	ctx.lr = 0x82E45E20;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45E20: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 82E45E24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E45E28: 48473E6D  bl 0x832b9c94
	ctx.lr = 0x82E45E2C;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82E45E2C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45E30: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82E45E34: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	pc = 0x82E45E38; continue 'dispatch;
            }
            0x82E45E38 => {
    //   block [0x82E45E38..0x82E45E48)
	// 82E45E38: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E45E3C: 939B0014  stw r28, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82E45E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E45E44: 4BE63610  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45E48 size=116
    let mut pc: u32 = 0x82E45E48;
    'dispatch: loop {
        match pc {
            0x82E45E48 => {
    //   block [0x82E45E48..0x82E45E88)
	// 82E45E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45E4C: 4BE635C1  bl 0x82ca940c
	ctx.lr = 0x82E45E50;
	sub_82CA93D0(ctx, base);
	// 82E45E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45E54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45E58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E45E5C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45E60: 4B33EA31  bl 0x82184890
	ctx.lr = 0x82E45E64;
	sub_82184890(ctx, base);
	// 82E45E64: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82E45E68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45E6C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E45E70: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E45E74: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45E78: 409A0010  bne cr6, 0x82e45e88
	if !ctx.cr[6].eq {
	pc = 0x82E45E88; continue 'dispatch;
	}
	// 82E45E7C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E45E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E45E84: 4BF11115  bl 0x82d56f98
	ctx.lr = 0x82E45E88;
	sub_82D56F98(ctx, base);
	pc = 0x82E45E88; continue 'dispatch;
            }
            0x82E45E88 => {
    //   block [0x82E45E88..0x82E45EBC)
	// 82E45E88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E45E8C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82E45E90: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45E94: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E45E98: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82E45E9C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E45EA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E45EA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E45EA8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45EAC: F9230020  std r9, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u64 ) };
	// 82E45EB0: 48473AA5  bl 0x832b9954
	ctx.lr = 0x82E45EB4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45EB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E45EB8: 4BE635A4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45EC0 size=384
    let mut pc: u32 = 0x82E45EC0;
    'dispatch: loop {
        match pc {
            0x82E45EC0 => {
    //   block [0x82E45EC0..0x82E46040)
	// 82E45EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45EC4: 4BE63549  bl 0x82ca940c
	ctx.lr = 0x82E45EC8;
	sub_82CA93D0(ctx, base);
	// 82E45EC8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E45ECC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E45ED0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45ED4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E45ED8: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E45EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45EE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E45EE4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E45EE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46040 size=92
    let mut pc: u32 = 0x82E46040;
    'dispatch: loop {
        match pc {
            0x82E46040 => {
    //   block [0x82E46040..0x82E4609C)
	// 82E46040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4604C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46050: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E46054: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4605C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E46060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E46064: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E46068: 4BF152A1  bl 0x82d5b308
	ctx.lr = 0x82E4606C;
	sub_82D5B308(ctx, base);
	// 82E4606C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E46070: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E46074: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E46078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4607C: 4BFFFE45  bl 0x82e45ec0
	ctx.lr = 0x82E46080;
	sub_82E45EC0(ctx, base);
	// 82E46080: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E46084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4608C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E46090: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E46094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E460A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E460A0 size=64
    let mut pc: u32 = 0x82E460A0;
    'dispatch: loop {
        match pc {
            0x82E460A0 => {
    //   block [0x82E460A0..0x82E460E0)
	// 82E460A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E460A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E460A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E460AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E460B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E460B4: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E460B8: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82E460BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E460C0: 4BF0F189  bl 0x82d55248
	ctx.lr = 0x82E460C4;
	sub_82D55248(ctx, base);
	// 82E460C4: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82E460C8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E460CC: 4BFFFC85  bl 0x82e45d50
	ctx.lr = 0x82E460D0;
	sub_82E45D50(ctx, base);
	// 82E460D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E460D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E460D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E460DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E460E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E460E0 size=128
    let mut pc: u32 = 0x82E460E0;
    'dispatch: loop {
        match pc {
            0x82E460E0 => {
    //   block [0x82E460E0..0x82E46114)
	// 82E460E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E460E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E460E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E460EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E460F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E460F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E460F8: 396B5CCC  addi r11, r11, 0x5ccc
	ctx.r[11].s64 = ctx.r[11].s64 + 23756;
	// 82E460FC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E46100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E46104: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46108: 419A000C  beq cr6, 0x82e46114
	if ctx.cr[6].eq {
	pc = 0x82E46114; continue 'dispatch;
	}
	// 82E4610C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E46110: 4BF39709  bl 0x82d7f818
	ctx.lr = 0x82E46114;
	sub_82D7F818(ctx, base);
	pc = 0x82E46114; continue 'dispatch;
            }
            0x82E46114 => {
    //   block [0x82E46114..0x82E46140)
	// 82E46114: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46118: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4611C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E46120: 409A0020  bne cr6, 0x82e46140
	if !ctx.cr[6].eq {
	pc = 0x82E46140; continue 'dispatch;
	}
	// 82E46124: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46128: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4612C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E46130: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46134: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E46138: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4613C: 4BF0F18D  bl 0x82d552c8
	ctx.lr = 0x82E46140;
	sub_82D552C8(ctx, base);
	pc = 0x82E46140; continue 'dispatch;
            }
            0x82E46140 => {
    //   block [0x82E46140..0x82E46160)
	// 82E46140: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E46144: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E46148: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4614C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E46150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E46158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4615C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46160 size=100
    let mut pc: u32 = 0x82E46160;
    'dispatch: loop {
        match pc {
            0x82E46160 => {
    //   block [0x82E46160..0x82E461A8)
	// 82E46160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4616C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4617C: 4BFFFF65  bl 0x82e460e0
	ctx.lr = 0x82E46180;
	sub_82E460E0(ctx, base);
	// 82E46180: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E46184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E46188: 419A0020  beq cr6, 0x82e461a8
	if ctx.cr[6].eq {
	pc = 0x82E461A8; continue 'dispatch;
	}
	// 82E4618C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46190: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E46194: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E46198: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4619C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E461A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E461A4: 4BF0F125  bl 0x82d552c8
	ctx.lr = 0x82E461A8;
	sub_82D552C8(ctx, base);
	pc = 0x82E461A8; continue 'dispatch;
            }
            0x82E461A8 => {
    //   block [0x82E461A8..0x82E461C4)
	// 82E461A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E461AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E461B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E461B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E461B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E461BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E461C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E461C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E461C8 size=140
    let mut pc: u32 = 0x82E461C8;
    'dispatch: loop {
        match pc {
            0x82E461C8 => {
    //   block [0x82E461C8..0x82E46214)
	// 82E461C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E461CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E461D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E461D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E461D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E461DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E461E0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E461E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E461E8: 409A002C  bne cr6, 0x82e46214
	if !ctx.cr[6].eq {
	pc = 0x82E46214; continue 'dispatch;
	}
	// 82E461EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E461F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E461F4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E461F8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E461FC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E46200: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E46204: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E46208: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4620C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E46210: 4BF0F0B9  bl 0x82d552c8
	ctx.lr = 0x82E46214;
	sub_82D552C8(ctx, base);
	pc = 0x82E46214; continue 'dispatch;
            }
            0x82E46214 => {
    //   block [0x82E46214..0x82E46240)
	// 82E46214: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46218: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4621C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E46220: 409A0020  bne cr6, 0x82e46240
	if !ctx.cr[6].eq {
	pc = 0x82E46240; continue 'dispatch;
	}
	// 82E46224: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46228: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4622C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E46230: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46234: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E46238: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4623C: 4BF0F08D  bl 0x82d552c8
	ctx.lr = 0x82E46240;
	sub_82D552C8(ctx, base);
	pc = 0x82E46240; continue 'dispatch;
            }
            0x82E46240 => {
    //   block [0x82E46240..0x82E46254)
	// 82E46240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E46244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4624C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46258 size=152
    let mut pc: u32 = 0x82E46258;
    'dispatch: loop {
        match pc {
            0x82E46258 => {
    //   block [0x82E46258..0x82E462BC)
	// 82E46258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E46264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4626C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46274: 4BFFFF55  bl 0x82e461c8
	ctx.lr = 0x82E46278;
	sub_82E461C8(ctx, base);
	// 82E46278: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4627C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E46280: 419A0054  beq cr6, 0x82e462d4
	if ctx.cr[6].eq {
	pc = 0x82E462D4; continue 'dispatch;
	}
	// 82E46284: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E46288: 419A004C  beq cr6, 0x82e462d4
	if ctx.cr[6].eq {
	pc = 0x82E462D4; continue 'dispatch;
	}
	// 82E4628C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46290: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E46294: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E46298: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4629C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E462A0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E462A4: 41980018  blt cr6, 0x82e462bc
	if ctx.cr[6].lt {
	pc = 0x82E462BC; continue 'dispatch;
	}
	// 82E462A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E462AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E462B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E462B4: 4BF0EE75  bl 0x82d55128
	ctx.lr = 0x82E462B8;
	sub_82D55128(ctx, base);
	// 82E462B8: 4800001C  b 0x82e462d4
	pc = 0x82E462D4; continue 'dispatch;
            }
            0x82E462BC => {
    //   block [0x82E462BC..0x82E462D4)
	// 82E462BC: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E462C0: 812B0050  lwz r9, 0x50(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E462C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E462C8: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E462CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E462D0: 93EB0050  stw r31, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	pc = 0x82E462D4; continue 'dispatch;
            }
            0x82E462D4 => {
    //   block [0x82E462D4..0x82E462F0)
	// 82E462D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E462D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E462DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E462E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E462E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E462E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E462EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E462F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E462F0 size=92
    let mut pc: u32 = 0x82E462F0;
    'dispatch: loop {
        match pc {
            0x82E462F0 => {
    //   block [0x82E462F0..0x82E4632C)
	// 82E462F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E462F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E462F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E462FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46304: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E46308: 396B5BAC  addi r11, r11, 0x5bac
	ctx.r[11].s64 = ctx.r[11].s64 + 23468;
	// 82E4630C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E46314: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46318: 419A0014  beq cr6, 0x82e4632c
	if ctx.cr[6].eq {
	pc = 0x82E4632C; continue 'dispatch;
	}
	// 82E4631C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E46320: 4BFFFF39  bl 0x82e46258
	ctx.lr = 0x82E46324;
	sub_82E46258(ctx, base);
	// 82E46324: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46328: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82E4632C; continue 'dispatch;
            }
            0x82E4632C => {
    //   block [0x82E4632C..0x82E4634C)
	// 82E4632C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E46330: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E46334: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4633C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E46344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46350 size=20
    let mut pc: u32 = 0x82E46350;
    'dispatch: loop {
        match pc {
            0x82E46350 => {
    //   block [0x82E46350..0x82E46364)
	// 82E46350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46354: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E46358: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4635C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E46360: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46368 size=32
    let mut pc: u32 = 0x82E46368;
    'dispatch: loop {
        match pc {
            0x82E46368 => {
    //   block [0x82E46368..0x82E46388)
	// 82E46368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4636C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E46370: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E46374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E46378: 396B5D20  addi r11, r11, 0x5d20
	ctx.r[11].s64 = ctx.r[11].s64 + 23840;
	// 82E4637C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E46380: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46388 size=12
    let mut pc: u32 = 0x82E46388;
    'dispatch: loop {
        match pc {
            0x82E46388 => {
    //   block [0x82E46388..0x82E46394)
	// 82E46388: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4638C: 386B5D20  addi r3, r11, 0x5d20
	ctx.r[3].s64 = ctx.r[11].s64 + 23840;
	// 82E46390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46398 size=100
    let mut pc: u32 = 0x82E46398;
    'dispatch: loop {
        match pc {
            0x82E46398 => {
    //   block [0x82E46398..0x82E463E0)
	// 82E46398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E463A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E463A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E463A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E463AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E463B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E463B4: 48005085  bl 0x82e4b438
	ctx.lr = 0x82E463B8;
	sub_82E4B438(ctx, base);
	// 82E463B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E463BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E463C0: 419A0020  beq cr6, 0x82e463e0
	if ctx.cr[6].eq {
	pc = 0x82E463E0; continue 'dispatch;
	}
	// 82E463C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E463C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E463CC: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E463D0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E463D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E463D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E463DC: 4BF0EEED  bl 0x82d552c8
	ctx.lr = 0x82E463E0;
	sub_82D552C8(ctx, base);
	pc = 0x82E463E0; continue 'dispatch;
            }
            0x82E463E0 => {
    //   block [0x82E463E0..0x82E463FC)
	// 82E463E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E463E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E463E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E463EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E463F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E463F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E463F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46400 size=104
    let mut pc: u32 = 0x82E46400;
    'dispatch: loop {
        match pc {
            0x82E46400 => {
    //   block [0x82E46400..0x82E46450)
	// 82E46400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4640C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46414: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4641C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46420: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46424: 419A002C  beq cr6, 0x82e46450
	if ctx.cr[6].eq {
	pc = 0x82E46450; continue 'dispatch;
	}
	// 82E46428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4642C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E46434: 4E800421  bctrl
	ctx.lr = 0x82E46438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E46438: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4643C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E46440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E46444: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E46448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4644C: 4E800421  bctrl
	ctx.lr = 0x82E46450;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E46450 => {
    //   block [0x82E46450..0x82E46468)
	// 82E46450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E46454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4645C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E46460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46468 size=72
    let mut pc: u32 = 0x82E46468;
    'dispatch: loop {
        match pc {
            0x82E46468 => {
    //   block [0x82E46468..0x82E4649C)
	// 82E46468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4646C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4647C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46480: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E46484: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E46488: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4648C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46490: 419A000C  beq cr6, 0x82e4649c
	if ctx.cr[6].eq {
	pc = 0x82E4649C; continue 'dispatch;
	}
	// 82E46494: 4B9FF31D  bl 0x828457b0
	ctx.lr = 0x82E46498;
	sub_828457B0(ctx, base);
	// 82E46498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82E4649C; continue 'dispatch;
            }
            0x82E4649C => {
    //   block [0x82E4649C..0x82E464B0)
	// 82E4649C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E464A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E464A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E464A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E464AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E464B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E464B0 size=72
    let mut pc: u32 = 0x82E464B0;
    'dispatch: loop {
        match pc {
            0x82E464B0 => {
    //   block [0x82E464B0..0x82E464E4)
	// 82E464B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E464B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E464B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E464BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E464C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E464C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E464C8: 396B5D58  addi r11, r11, 0x5d58
	ctx.r[11].s64 = ctx.r[11].s64 + 23896;
	// 82E464CC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E464D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E464D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E464D8: 419A000C  beq cr6, 0x82e464e4
	if ctx.cr[6].eq {
	pc = 0x82E464E4; continue 'dispatch;
	}
	// 82E464DC: 4B9FF2D5  bl 0x828457b0
	ctx.lr = 0x82E464E0;
	sub_828457B0(ctx, base);
	// 82E464E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82E464E4; continue 'dispatch;
            }
            0x82E464E4 => {
    //   block [0x82E464E4..0x82E464F8)
	// 82E464E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E464E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E464EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E464F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E464F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46500 size=16
    let mut pc: u32 = 0x82E46500;
    'dispatch: loop {
        match pc {
            0x82E46500 => {
    //   block [0x82E46500..0x82E46510)
	// 82E46500: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E46504: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E46508: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82E4650C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E46510 size=20
    let mut pc: u32 = 0x82E46510;
    'dispatch: loop {
        match pc {
            0x82E46510 => {
    //   block [0x82E46510..0x82E46524)
	// 82E46510: C003005C  lfs f0, 0x5c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E46514: C1A30058  lfs f13, 0x58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E46518: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4651C: EC20682A  fadds f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E46520: 4BF3A790  b 0x82d80cb0
	sub_82D80CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46528 size=8
    let mut pc: u32 = 0x82E46528;
    'dispatch: loop {
        match pc {
            0x82E46528 => {
    //   block [0x82E46528..0x82E46530)
	// 82E46528: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82E4652C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46530 size=16
    let mut pc: u32 = 0x82E46530;
    'dispatch: loop {
        match pc {
            0x82E46530 => {
    //   block [0x82E46530..0x82E46540)
	// 82E46530: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46540 size=8
    let mut pc: u32 = 0x82E46540;
    'dispatch: loop {
        match pc {
            0x82E46540 => {
    //   block [0x82E46540..0x82E46548)
	// 82E46540: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E46544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46550 size=292
    let mut pc: u32 = 0x82E46550;
    'dispatch: loop {
        match pc {
            0x82E46550 => {
    //   block [0x82E46550..0x82E46674)
	// 82E46550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4655C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46568: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E4656C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46570: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82E46574: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82E46578: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4657C: 81290018  lwz r9, 0x18(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46678 size=560
    let mut pc: u32 = 0x82E46678;
    'dispatch: loop {
        match pc {
            0x82E46678 => {
    //   block [0x82E46678..0x82E466A4)
	// 82E46678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4667C: 4BE62D89  bl 0x82ca9404
	ctx.lr = 0x82E46680;
	sub_82CA93D0(ctx, base);
	// 82E46680: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46684: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4668C: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E46690: 4BF39D31  bl 0x82d803c0
	ctx.lr = 0x82E46694;
	sub_82D803C0(ctx, base);
	// 82E46694: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E46698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4669C: 419A0008  beq cr6, 0x82e466a4
	if ctx.cr[6].eq {
	pc = 0x82E466A4; continue 'dispatch;
	}
	// 82E466A0: 4BF39D41  bl 0x82d803e0
	ctx.lr = 0x82E466A4;
	sub_82D803E0(ctx, base);
	pc = 0x82E466A4; continue 'dispatch;
            }
            0x82E466A4 => {
    //   block [0x82E466A4..0x82E468A8)
	// 82E466A4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E468A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E468A8 size=320
    let mut pc: u32 = 0x82E468A8;
    'dispatch: loop {
        match pc {
            0x82E468A8 => {
    //   block [0x82E468A8..0x82E469E8)
	// 82E468A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E468AC: 4BE62B61  bl 0x82ca940c
	ctx.lr = 0x82E468B0;
	sub_82CA93D0(ctx, base);
	// 82E468B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E468B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E468B8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E468BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E468C0: 54BD3032  slwi r29, r5, 6
	ctx.r[29].u32 = ctx.r[5].u32.wrapping_shl(6);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E468C4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E468C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E469E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E469E8 size=232
    let mut pc: u32 = 0x82E469E8;
    'dispatch: loop {
        match pc {
            0x82E469E8 => {
    //   block [0x82E469E8..0x82E46AD0)
	// 82E469E8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82E469EC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46AD0 size=536
    let mut pc: u32 = 0x82E46AD0;
    'dispatch: loop {
        match pc {
            0x82E46AD0 => {
    //   block [0x82E46AD0..0x82E46CE8)
	// 82E46AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46AD4: 4BE6292D  bl 0x82ca9400
	ctx.lr = 0x82E46AD8;
	sub_82CA93D0(ctx, base);
	// 82E46AD8: 38C40010  addi r6, r4, 0x10
	ctx.r[6].s64 = ctx.r[4].s64 + 16;
	// 82E46ADC: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82E46AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E46AE4: 3905000C  addi r8, r5, 0xc
	ctx.r[8].s64 = ctx.r[5].s64 + 12;
	// 82E46AE8: 3BE50030  addi r31, r5, 0x30
	ctx.r[31].s64 = ctx.r[5].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E46CE8 size=128
    let mut pc: u32 = 0x82E46CE8;
    'dispatch: loop {
        match pc {
            0x82E46CE8 => {
    //   block [0x82E46CE8..0x82E46D20)
	// 82E46CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46CEC: 4BE62719  bl 0x82ca9404
	ctx.lr = 0x82E46CF0;
	sub_82CA93D0(ctx, base);
	// 82E46CF0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E46CF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46CFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E46D00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E46D04: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82E46D08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E46D0C: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46D10: C3EB0BF8  lfs f31, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E46D14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E46D18: 40990040  ble cr6, 0x82e46d58
	if !ctx.cr[6].gt {
	pc = 0x82E46D58; continue 'dispatch;
	}
	// 82E46D1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82E46D20; continue 'dispatch;
            }
            0x82E46D20 => {
    //   block [0x82E46D20..0x82E46D44)
	// 82E46D20: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E46D24: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E46D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E46D2C: 7CBE5A14  add r5, r30, r11
	ctx.r[5].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E46D30: 4B433969  bl 0x8227a698
	ctx.lr = 0x82E46D34;
	sub_8227A698(ctx, base);
	// 82E46D34: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E46D38: 4098000C  bge cr6, 0x82e46d44
	if !ctx.cr[6].lt {
	pc = 0x82E46D44; continue 'dispatch;
	}
	// 82E46D3C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E46D40: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	pc = 0x82E46D44; continue 'dispatch;
            }
            0x82E46D44 => {
    //   block [0x82E46D44..0x82E46D58)
	// 82E46D44: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46D48: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E46D4C: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82E46D50: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E46D54: 4198FFCC  blt cr6, 0x82e46d20
	if ctx.cr[6].lt {
	pc = 0x82E46D20; continue 'dispatch;
	}
	pc = 0x82E46D58; continue 'dispatch;
            }
            0x82E46D58 => {
    //   block [0x82E46D58..0x82E46D68)
	// 82E46D58: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E46D5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E46D60: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E46D64: 4BE626F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E46D68 size=1184
    let mut pc: u32 = 0x82E46D68;
    'dispatch: loop {
        match pc {
            0x82E46D68 => {
    //   block [0x82E46D68..0x82E46E10)
	// 82E46D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46D6C: 4BE62669  bl 0x82ca93d4
	ctx.lr = 0x82E46D70;
	sub_82CA93D0(ctx, base);
	// 82E46D70: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 82E46D74: 4BE66F61  bl 0x82cadcd4
	ctx.lr = 0x82E46D78;
	sub_82CADCA0(ctx, base);
	// 82E46D78: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46D7C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E46D80: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82E46D84: 7CAF2B78  mr r15, r5
	ctx.r[15].u64 = ctx.r[5].u64;
	// 82E46D88: 3A910008  addi r20, r17, 8
	ctx.r[20].s64 = ctx.r[17].s64 + 8;
	// 82E46D8C: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82E46D90: 817B0078  lwz r11, 0x78(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46D94: 3A1B0078  addi r16, r27, 0x78
	ctx.r[16].s64 = ctx.r[27].s64 + 120;
	// 82E46D98: C0110008  lfs f0, 8(r17)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E46D9C: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82E46DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46DA4: 40990454  ble cr6, 0x82e471f8
	if !ctx.cr[6].gt {
	pc = 0x82E471F8; continue 'dispatch;
	}
	// 82E46DA8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E46DAC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E46DB0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E46DB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E46DB8: 3B3B0074  addi r25, r27, 0x74
	ctx.r[25].s64 = ctx.r[27].s64 + 116;
	// 82E46DBC: C3680A5C  lfs f27, 0xa5c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2652 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82E46DC0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E46DC4: C3890A78  lfs f28, 0xa78(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2680 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E46DC8: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82E46DCC: C3CA0C18  lfs f30, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E46DD0: 3AC00060  li r22, 0x60
	ctx.r[22].s64 = 96;
	// 82E46DD4: C3AB0A74  lfs f29, 0xa74(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2676 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E46DD8: 3AE000D0  li r23, 0xd0
	ctx.r[23].s64 = 208;
	// 82E46DDC: 3A600130  li r19, 0x130
	ctx.r[19].s64 = 304;
	// 82E46DE0: 3AA00190  li r21, 0x190
	ctx.r[21].s64 = 400;
	// 82E46DE4: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E46DEC: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82E46DF0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E46DF4: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46DF8: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82E46DFC: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E46E00: 81280078  lwz r9, 0x78(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46E04: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E46E08: 40990024  ble cr6, 0x82e46e2c
	if !ctx.cr[6].gt {
	pc = 0x82E46E2C; continue 'dispatch;
	}
	// 82E46E0C: 81680074  lwz r11, 0x74(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(116 as u32) ) } as u64;
	pc = 0x82E46E10; continue 'dispatch;
            }
            0x82E46E10 => {
    //   block [0x82E46E10..0x82E46E2C)
	// 82E46E10: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46E14: 2B071300  cmplwi cr6, r7, 0x1300
	ctx.cr[6].compare_u32(ctx.r[7].u32, 4864 as u32, &mut ctx.xer);
	// 82E46E18: 419A02BC  beq cr6, 0x82e470d4
	if ctx.cr[6].eq {
	pc = 0x82E470D4; continue 'dispatch;
	}
	// 82E46E1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E46E20: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E46E24: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E46E28: 4198FFE8  blt cr6, 0x82e46e10
	if ctx.cr[6].lt {
	pc = 0x82E46E10; continue 'dispatch;
	}
	pc = 0x82E46E2C; continue 'dispatch;
            }
            0x82E46E2C => {
    //   block [0x82E46E2C..0x82E46E50)
	// 82E46E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46E30: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E46E34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46E38: 419A0094  beq cr6, 0x82e46ecc
	if ctx.cr[6].eq {
	pc = 0x82E46ECC; continue 'dispatch;
	}
	// 82E46E3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46E40: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E46E44: 40990028  ble cr6, 0x82e46e6c
	if !ctx.cr[6].gt {
	pc = 0x82E46E6C; continue 'dispatch;
	}
	// 82E46E48: 81080074  lwz r8, 0x74(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E46E4C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	pc = 0x82E46E50; continue 'dispatch;
            }
            0x82E46E50 => {
    //   block [0x82E46E50..0x82E46E6C)
	// 82E46E50: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46E54: 2B071300  cmplwi cr6, r7, 0x1300
	ctx.cr[6].compare_u32(ctx.r[7].u32, 4864 as u32, &mut ctx.xer);
	// 82E46E58: 419A0284  beq cr6, 0x82e470dc
	if ctx.cr[6].eq {
	pc = 0x82E470DC; continue 'dispatch;
	}
	// 82E46E5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E46E60: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E46E64: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E46E68: 4198FFE8  blt cr6, 0x82e46e50
	if ctx.cr[6].lt {
	pc = 0x82E46E50; continue 'dispatch;
	}
	pc = 0x82E46E6C; continue 'dispatch;
            }
            0x82E46E6C => {
    //   block [0x82E46E6C..0x82E46E88)
	// 82E46E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46E70: 815B0084  lwz r10, 0x84(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E46E74: 557E003E  slwi r30, r11, 0
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E46E78: 3BAAFFFF  addi r29, r10, -1
	ctx.r[29].s64 = ctx.r[10].s64 + -1;
	// 82E46E7C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E46E80: 4198004C  blt cr6, 0x82e46ecc
	if ctx.cr[6].lt {
	pc = 0x82E46ECC; continue 'dispatch;
	}
	// 82E46E84: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	pc = 0x82E46E88; continue 'dispatch;
            }
            0x82E46E88 => {
    //   block [0x82E46E88..0x82E46EBC)
	// 82E46E88: 817B0080  lwz r11, 0x80(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E46E8C: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E46E90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E46E94: 419A0028  beq cr6, 0x82e46ebc
	if ctx.cr[6].eq {
	pc = 0x82E46EBC; continue 'dispatch;
	}
	// 82E46E98: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E46E9C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46EA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E46EA4: 7CDA5A14  add r6, r26, r11
	ctx.r[6].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82E46EA8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E46EAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46EB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46EB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E46EB8: 4E800421  bctrl
	ctx.lr = 0x82E46EBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E46EBC => {
    //   block [0x82E46EBC..0x82E46ECC)
	// 82E46EBC: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82E46EC0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E46EC4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E46EC8: 4098FFC0  bge cr6, 0x82e46e88
	if !ctx.cr[6].lt {
	pc = 0x82E46E88; continue 'dispatch;
	}
	pc = 0x82E46ECC; continue 'dispatch;
            }
            0x82E46ECC => {
    //   block [0x82E46ECC..0x82E46F10)
	// 82E46ECC: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46ED0: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82E46ED4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E46ED8: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E46EDC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E46EE0: 409A0304  bne cr6, 0x82e471e4
	if !ctx.cr[6].eq {
	pc = 0x82E471E4; continue 'dispatch;
	}
	// 82E46EE4: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46EE8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E46EEC: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E46EF0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E46EF4: 419A02F0  beq cr6, 0x82e471e4
	if ctx.cr[6].eq {
	pc = 0x82E471E4; continue 'dispatch;
	}
	// 82E46EF8: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E46EFC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E46F00: 419A0010  beq cr6, 0x82e46f10
	if ctx.cr[6].eq {
	pc = 0x82E46F10; continue 'dispatch;
	}
	// 82E46F04: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E46F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46F0C: 409A0008  bne cr6, 0x82e46f14
	if !ctx.cr[6].eq {
	pc = 0x82E46F14; continue 'dispatch;
	}
	pc = 0x82E46F10; continue 'dispatch;
            }
            0x82E46F10 => {
    //   block [0x82E46F10..0x82E46F14)
	// 82E46F10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82E46F14; continue 'dispatch;
            }
            0x82E46F14 => {
    //   block [0x82E46F14..0x82E470D4)
	// 82E46F14: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E46F18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46F1C: 409A02C8  bne cr6, 0x82e471e4
	if !ctx.cr[6].eq {
	pc = 0x82E471E4; continue 'dispatch;
	}
	// 82E46F20: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46F24: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82E46F28: 3BFD00D0  addi r31, r29, 0xd0
	ctx.r[31].s64 = ctx.r[29].s64 + 208;
	pc = 0x82E470D4; continue 'dispatch;
            }
            0x82E470D4 => {
    //   block [0x82E470D4..0x82E470DC)
	// 82E470D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E470D8: 4BFFFD58  b 0x82e46e30
	pc = 0x82E46E30; continue 'dispatch;
            }
            0x82E470DC => {
    //   block [0x82E470DC..0x82E471E4)
	// 82E470DC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E470E0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E470E4: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E470E8: 4BFFFD88  b 0x82e46e70
	pc = 0x82E46E70; continue 'dispatch;
	// 82E470EC: D3C10090  stfs f30, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82E470F0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            0x82E471E4 => {
    //   block [0x82E471E4..0x82E471F8)
	// 82E471E4: 81700000  lwz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E471E8: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 82E471EC: 3B5A0030  addi r26, r26, 0x30
	ctx.r[26].s64 = ctx.r[26].s64 + 48;
	// 82E471F0: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E471F4: 4198FBF0  blt cr6, 0x82e46de4
	if ctx.cr[6].lt {
	pc = 0x82E46DE4; continue 'dispatch;
	}
	pc = 0x82E471F8; continue 'dispatch;
            }
            0x82E471F8 => {
    //   block [0x82E471F8..0x82E47208)
	// 82E471F8: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 82E471FC: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 82E47200: 4BE66B21  bl 0x82cadd20
	ctx.lr = 0x82E47204;
	sub_82CADCEC(ctx, base);
	// 82E47204: 4BE62220  b 0x82ca9424
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E47208 size=88
    let mut pc: u32 = 0x82E47208;
    'dispatch: loop {
        match pc {
            0x82E47208 => {
    //   block [0x82E47208..0x82E4721C)
	// 82E47208: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4720C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E47210: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E47214: 40990024  ble cr6, 0x82e47238
	if !ctx.cr[6].gt {
	pc = 0x82E47238; continue 'dispatch;
	}
	// 82E47218: 81430080  lwz r10, 0x80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	pc = 0x82E4721C; continue 'dispatch;
            }
            0x82E4721C => {
    //   block [0x82E4721C..0x82E47238)
	// 82E4721C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47220: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E47224: 419A0018  beq cr6, 0x82e4723c
	if ctx.cr[6].eq {
	pc = 0x82E4723C; continue 'dispatch;
	}
	// 82E47228: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4722C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E47230: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E47234: 4198FFE8  blt cr6, 0x82e4721c
	if ctx.cr[6].lt {
	pc = 0x82E4721C; continue 'dispatch;
	}
	pc = 0x82E47238; continue 'dispatch;
            }
            0x82E47238 => {
    //   block [0x82E47238..0x82E4723C)
	// 82E47238: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82E4723C; continue 'dispatch;
            }
            0x82E4723C => {
    //   block [0x82E4723C..0x82E47260)
	// 82E4723C: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47240: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E47244: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47248: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E4724C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47250: 91430084  stw r10, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82E47254: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E47258: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E4725C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47260 size=100
    let mut pc: u32 = 0x82E47260;
    'dispatch: loop {
        match pc {
            0x82E47260 => {
    //   block [0x82E47260..0x82E4728C)
	// 82E47260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47264: 4BE621A1  bl 0x82ca9404
	ctx.lr = 0x82E47268;
	sub_82CA93D0(ctx, base);
	// 82E47268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4726C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E47270: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E47274: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E47278: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4727C: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E47280: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47284: 41980038  blt cr6, 0x82e472bc
	if ctx.cr[6].lt {
	pc = 0x82E472BC; continue 'dispatch;
	}
	// 82E47288: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	pc = 0x82E4728C; continue 'dispatch;
            }
            0x82E4728C => {
    //   block [0x82E4728C..0x82E472BC)
	// 82E4728C: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47290: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E47294: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E47298: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4729C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E472A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E472A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E472A8: 4E800421  bctrl
	ctx.lr = 0x82E472AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E472AC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E472B0: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E472B4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E472B8: 4098FFD4  bge cr6, 0x82e4728c
	if !ctx.cr[6].lt {
	pc = 0x82E4728C; continue 'dispatch;
	}
            }
            0x82E472BC => {
    //   block [0x82E472BC..0x82E472C4)
	// 82E472BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E472C0: 4BE62194  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E472C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E472C8 size=92
    let mut pc: u32 = 0x82E472C8;
    'dispatch: loop {
        match pc {
            0x82E472C8 => {
    //   block [0x82E472C8..0x82E472F0)
	// 82E472C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E472CC: 4BE6213D  bl 0x82ca9408
	ctx.lr = 0x82E472D0;
	sub_82CA93D0(ctx, base);
	// 82E472D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E472D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E472D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E472DC: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E472E0: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E472E4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E472E8: 41980034  blt cr6, 0x82e4731c
	if ctx.cr[6].lt {
	pc = 0x82E4731C; continue 'dispatch;
	}
	// 82E472EC: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	pc = 0x82E472F0; continue 'dispatch;
            }
            0x82E472F0 => {
    //   block [0x82E472F0..0x82E4731C)
	// 82E472F0: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E472F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E472F8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E472FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47300: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47308: 4E800421  bctrl
	ctx.lr = 0x82E4730C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4730C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E47310: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E47314: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47318: 4098FFD8  bge cr6, 0x82e472f0
	if !ctx.cr[6].lt {
	pc = 0x82E472F0; continue 'dispatch;
	}
            }
            0x82E4731C => {
    //   block [0x82E4731C..0x82E47324)
	// 82E4731C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E47320: 4BE62138  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47328 size=92
    let mut pc: u32 = 0x82E47328;
    'dispatch: loop {
        match pc {
            0x82E47328 => {
    //   block [0x82E47328..0x82E47350)
	// 82E47328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4732C: 4BE620DD  bl 0x82ca9408
	ctx.lr = 0x82E47330;
	sub_82CA93D0(ctx, base);
	// 82E47330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47334: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E47338: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4733C: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47340: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E47344: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47348: 41980034  blt cr6, 0x82e4737c
	if ctx.cr[6].lt {
	pc = 0x82E4737C; continue 'dispatch;
	}
	// 82E4734C: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	pc = 0x82E47350; continue 'dispatch;
            }
            0x82E47350 => {
    //   block [0x82E47350..0x82E4737C)
	// 82E47350: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47354: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E47358: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4735C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47360: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E47364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47368: 4E800421  bctrl
	ctx.lr = 0x82E4736C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4736C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E47370: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E47374: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47378: 4098FFD8  bge cr6, 0x82e47350
	if !ctx.cr[6].lt {
	pc = 0x82E47350; continue 'dispatch;
	}
            }
            0x82E4737C => {
    //   block [0x82E4737C..0x82E47384)
	// 82E4737C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E47380: 4BE620D8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47388 size=176
    let mut pc: u32 = 0x82E47388;
    'dispatch: loop {
        match pc {
            0x82E47388 => {
    //   block [0x82E47388..0x82E47438)
	// 82E47388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E47390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E47394: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47398: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4739C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E473A0: 394B5D40  addi r10, r11, 0x5d40
	ctx.r[10].s64 = ctx.r[11].s64 + 23872;
	// 82E473A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E473A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E473AC: 392B5D58  addi r9, r11, 0x5d58
	ctx.r[9].s64 = ctx.r[11].s64 + 23896;
	// 82E473B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E473B4: 390B17E0  addi r8, r11, 0x17e0
	ctx.r[8].s64 = ctx.r[11].s64 + 6112;
	// 82E473B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E473BC: B0BF0006  sth r5, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82E473C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E473C4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E473C8: 38EB5D88  addi r7, r11, 0x5d88
	ctx.r[7].s64 = ctx.r[11].s64 + 23944;
	// 82E473CC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E473D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E473D4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E473D8: 38CB5D70  addi r6, r11, 0x5d70
	ctx.r[6].s64 = ctx.r[11].s64 + 23920;
	// 82E473DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E473E0: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E473E4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E473E8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E473EC: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82E473F0: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E473F4: 915F007C  stw r10, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82E473F8: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E473FC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E47400: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E47404: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82E47408: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82E4740C: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82E47410: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82E47414: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82E47418: 915F00A0  stw r10, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82E4741C: 4BFFF25D  bl 0x82e46678
	ctx.lr = 0x82E47420;
	sub_82E46678(ctx, base);
	// 82E47420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E47424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E47428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4742C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E47430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E47434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47438 size=440
    let mut pc: u32 = 0x82E47438;
    'dispatch: loop {
        match pc {
            0x82E47438 => {
    //   block [0x82E47438..0x82E4748C)
	// 82E47438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4743C: 4BE61FC5  bl 0x82ca9400
	ctx.lr = 0x82E47440;
	sub_82CA93D0(ctx, base);
	// 82E47440: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47448: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4744C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E47450: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E47454: 396B17E0  addi r11, r11, 0x17e0
	ctx.r[11].s64 = ctx.r[11].s64 + 6112;
	// 82E47458: 394A5D88  addi r10, r10, 0x5d88
	ctx.r[10].s64 = ctx.r[10].s64 + 23944;
	// 82E4745C: 811F0090  lwz r8, 0x90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47460: 39295D70  addi r9, r9, 0x5d70
	ctx.r[9].s64 = ctx.r[9].s64 + 23920;
	// 82E47464: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E47468: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82E4746C: 3B7F000C  addi r27, r31, 0xc
	ctx.r[27].s64 = ctx.r[31].s64 + 12;
	// 82E47470: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E47474: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82E47478: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4747C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E47480: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E47484: 4099002C  ble cr6, 0x82e474b0
	if !ctx.cr[6].gt {
	pc = 0x82E474B0; continue 'dispatch;
	}
	// 82E47488: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	pc = 0x82E4748C; continue 'dispatch;
            }
            0x82E4748C => {
    //   block [0x82E4748C..0x82E474B0)
	// 82E4748C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E47490: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E47494: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E47498: 4BF3E981  bl 0x82d85e18
	ctx.lr = 0x82E4749C;
	sub_82D85E18(ctx, base);
	// 82E4749C: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E474A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E474A4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E474A8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E474AC: 4198FFE0  blt cr6, 0x82e4748c
	if ctx.cr[6].lt {
	pc = 0x82E4748C; continue 'dispatch;
	}
	pc = 0x82E474B0; continue 'dispatch;
            }
            0x82E474B0 => {
    //   block [0x82E474B0..0x82E474C8)
	// 82E474B0: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E474B4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82E474B8: 935F0090  stw r26, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[26].u32 ) };
	// 82E474BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E474C0: 4099002C  ble cr6, 0x82e474ec
	if !ctx.cr[6].gt {
	pc = 0x82E474EC; continue 'dispatch;
	}
	// 82E474C4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	pc = 0x82E474C8; continue 'dispatch;
            }
            0x82E474C8 => {
    //   block [0x82E474C8..0x82E474EC)
	// 82E474C8: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E474CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E474D0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E474D4: 4BF523DD  bl 0x82d998b0
	ctx.lr = 0x82E474D8;
	sub_82D998B0(ctx, base);
	// 82E474D8: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E474DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E474E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E474E4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E474E8: 4198FFE0  blt cr6, 0x82e474c8
	if ctx.cr[6].lt {
	pc = 0x82E474C8; continue 'dispatch;
	}
	pc = 0x82E474EC; continue 'dispatch;
            }
            0x82E474EC => {
    //   block [0x82E474EC..0x82E47534)
	// 82E474EC: 38A01300  li r5, 0x1300
	ctx.r[5].s64 = 4864;
	// 82E474F0: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E474F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E474F8: 935F009C  stw r26, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[26].u32 ) };
	// 82E474FC: 4BF38FCD  bl 0x82d804c8
	ctx.lr = 0x82E47500;
	sub_82D804C8(ctx, base);
	// 82E47500: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E47504: 4BF38EDD  bl 0x82d803e0
	ctx.lr = 0x82E47508;
	sub_82D803E0(ctx, base);
	// 82E47508: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E4750C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47510: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E47514: 409A0020  bne cr6, 0x82e47534
	if !ctx.cr[6].eq {
	pc = 0x82E47534; continue 'dispatch;
	}
	// 82E47518: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4751C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E47520: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E47524: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E47528: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4752C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E47530: 4BF0DD99  bl 0x82d552c8
	ctx.lr = 0x82E47534;
	sub_82D552C8(ctx, base);
	pc = 0x82E47534; continue 'dispatch;
            }
            0x82E47534 => {
    //   block [0x82E47534..0x82E47560)
	// 82E47534: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E47538: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4753C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E47540: 409A0020  bne cr6, 0x82e47560
	if !ctx.cr[6].eq {
	pc = 0x82E47560; continue 'dispatch;
	}
	// 82E47544: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47548: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4754C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E47550: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E47554: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E47558: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4755C: 4BF0DD6D  bl 0x82d552c8
	ctx.lr = 0x82E47560;
	sub_82D552C8(ctx, base);
	pc = 0x82E47560; continue 'dispatch;
            }
            0x82E47560 => {
    //   block [0x82E47560..0x82E4758C)
	// 82E47560: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E47564: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47568: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4756C: 409A0020  bne cr6, 0x82e4758c
	if !ctx.cr[6].eq {
	pc = 0x82E4758C; continue 'dispatch;
	}
	// 82E47570: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47574: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E47578: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4757C: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47580: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E47584: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E47588: 4BF0DD41  bl 0x82d552c8
	ctx.lr = 0x82E4758C;
	sub_82D552C8(ctx, base);
	pc = 0x82E4758C; continue 'dispatch;
            }
            0x82E4758C => {
    //   block [0x82E4758C..0x82E475C4)
	// 82E4758C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E47590: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47594: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E47598: 409A002C  bne cr6, 0x82e475c4
	if !ctx.cr[6].eq {
	pc = 0x82E475C4; continue 'dispatch;
	}
	// 82E4759C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E475A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E475A4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E475A8: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E475AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E475B0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E475B4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E475B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E475BC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E475C0: 4BF0DD09  bl 0x82d552c8
	ctx.lr = 0x82E475C4;
	sub_82D552C8(ctx, base);
	pc = 0x82E475C4; continue 'dispatch;
            }
            0x82E475C4 => {
    //   block [0x82E475C4..0x82E475F0)
	// 82E475C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E475C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E475CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82E475D0: 396B5D58  addi r11, r11, 0x5d58
	ctx.r[11].s64 = ctx.r[11].s64 + 23896;
	// 82E475D4: 394A5D40  addi r10, r10, 0x5d40
	ctx.r[10].s64 = ctx.r[10].s64 + 23872;
	// 82E475D8: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82E475DC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E475E0: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E475E4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E475E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E475EC: 4BE61E64  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E475F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E475F0 size=112
    let mut pc: u32 = 0x82E475F0;
    'dispatch: loop {
        match pc {
            0x82E475F0 => {
    //   block [0x82E475F0..0x82E4762C)
	// 82E475F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E475F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E475F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E475FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E47600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47604: 3BE30080  addi r31, r3, 0x80
	ctx.r[31].s64 = ctx.r[3].s64 + 128;
	// 82E47608: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4760C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47610: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47614: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E47618: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4761C: 409A0010  bne cr6, 0x82e4762c
	if !ctx.cr[6].eq {
	pc = 0x82E4762C; continue 'dispatch;
	}
	// 82E47620: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E47624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E47628: 4BF0F971  bl 0x82d56f98
	ctx.lr = 0x82E4762C;
	sub_82D56F98(ctx, base);
	pc = 0x82E4762C; continue 'dispatch;
            }
            0x82E4762C => {
    //   block [0x82E4762C..0x82E47660)
	// 82E4762C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47630: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47634: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47638: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E4763C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47640: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E47644: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E47648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4764C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E47650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E47654: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E47658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47660 size=344
    let mut pc: u32 = 0x82E47660;
    'dispatch: loop {
        match pc {
            0x82E47660 => {
    //   block [0x82E47660..0x82E4768C)
	// 82E47660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E47668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4766C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E47670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47678: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4767C: 357FFFF8  addic. r11, r31, -8
	ctx.xer.ca = (ctx.r[31].u32 > (!(-8 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47680: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E47684: 40820008  bne 0x82e4768c
	if !ctx.cr[0].eq {
	pc = 0x82E4768C; continue 'dispatch;
	}
	// 82E47688: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	pc = 0x82E4768C; continue 'dispatch;
            }
            0x82E4768C => {
    //   block [0x82E4768C..0x82E476DC)
	// 82E4768C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E47690: 4BF3E789  bl 0x82d85e18
	ctx.lr = 0x82E47694;
	sub_82D85E18(ctx, base);
	// 82E47694: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E47698: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82E4769C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E476A0: 419800AC  blt cr6, 0x82e4774c
	if ctx.cr[6].lt {
	pc = 0x82E4774C; continue 'dispatch;
	}
	// 82E476A4: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E476A8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E476AC: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82E476B0: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E476B4: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E476B8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E476BC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E476C0: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E476C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E476C8: 409A0014  bne cr6, 0x82e476dc
	if !ctx.cr[6].eq {
	pc = 0x82E476DC; continue 'dispatch;
	}
	// 82E476CC: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E476D0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E476D4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E476D8: 48000008  b 0x82e476e0
	pc = 0x82E476E0; continue 'dispatch;
            }
            0x82E476DC => {
    //   block [0x82E476DC..0x82E476E0)
	// 82E476DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82E476E0; continue 'dispatch;
            }
            0x82E476E0 => {
    //   block [0x82E476E0..0x82E4773C)
	// 82E476E0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E476E4: 409A0058  bne cr6, 0x82e4773c
	if !ctx.cr[6].eq {
	pc = 0x82E4773C; continue 'dispatch;
	}
	// 82E476E8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E476EC: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E476F0: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82E476F4: 7D675214  add r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82E476F8: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E476FC: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E47700: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82E47704: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47708: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	pc = 0x82E4773C; continue 'dispatch;
            }
            0x82E4773C => {
    //   block [0x82E4773C..0x82E4774C)
	// 82E4773C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82E47740: 38E7FFD0  addi r7, r7, -0x30
	ctx.r[7].s64 = ctx.r[7].s64 + -48;
	// 82E47744: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E47748: 4098FF6C  bge cr6, 0x82e476b4
	if !ctx.cr[6].lt {
	pc = 0x82E476B4; continue 'dispatch;
	}
	pc = 0x82E4774C; continue 'dispatch;
            }
            0x82E4774C => {
    //   block [0x82E4774C..0x82E477B8)
	// 82E4774C: 813F0088  lwz r9, 0x88(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E47750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E47754: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E47758: 40990024  ble cr6, 0x82e4777c
	if !ctx.cr[6].gt {
	pc = 0x82E4777C; continue 'dispatch;
	}
	// 82E4775C: 815F0084  lwz r10, 0x84(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47760: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47764: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E47768: 419A0018  beq cr6, 0x82e47780
	if ctx.cr[6].eq {
	pc = 0x82E47780; continue 'dispatch;
	}
	// 82E4776C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E47770: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E47774: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E47778: 4198FFE8  blt cr6, 0x82e47760
	if ctx.cr[6].lt {
	pc = 0x82E47760; continue 'dispatch;
	}
	// 82E4777C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E47780: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E47784: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E47788: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4778C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E47790: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47794: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E47798: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4779C: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E477A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E477A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E477A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E477AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E477B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E477B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E477B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E477B8 size=344
    let mut pc: u32 = 0x82E477B8;
    'dispatch: loop {
        match pc {
            0x82E477B8 => {
    //   block [0x82E477B8..0x82E477E4)
	// 82E477B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E477BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E477C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E477C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E477C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E477CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E477D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E477D4: 357FFFF4  addic. r11, r31, -0xc
	ctx.xer.ca = (ctx.r[31].u32 > (!(-12 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E477D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E477DC: 40820008  bne 0x82e477e4
	if !ctx.cr[0].eq {
	pc = 0x82E477E4; continue 'dispatch;
	}
	// 82E477E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	pc = 0x82E477E4; continue 'dispatch;
            }
            0x82E477E4 => {
    //   block [0x82E477E4..0x82E47834)
	// 82E477E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E477E8: 4BF520C9  bl 0x82d998b0
	ctx.lr = 0x82E477EC;
	sub_82D998B0(ctx, base);
	// 82E477EC: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E477F0: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82E477F4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E477F8: 419800AC  blt cr6, 0x82e478a4
	if ctx.cr[6].lt {
	pc = 0x82E478A4; continue 'dispatch;
	}
	// 82E477FC: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47800: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E47804: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82E47808: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E4780C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E47810: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E47814: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E47818: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4781C: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82E47820: 409A0014  bne cr6, 0x82e47834
	if !ctx.cr[6].eq {
	pc = 0x82E47834; continue 'dispatch;
	}
	// 82E47824: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47828: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E4782C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E47830: 48000008  b 0x82e47838
	pc = 0x82E47838; continue 'dispatch;
            }
            0x82E47834 => {
    //   block [0x82E47834..0x82E47838)
	// 82E47834: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82E47838; continue 'dispatch;
            }
            0x82E47838 => {
    //   block [0x82E47838..0x82E47894)
	// 82E47838: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4783C: 409A0058  bne cr6, 0x82e47894
	if !ctx.cr[6].eq {
	pc = 0x82E47894; continue 'dispatch;
	}
	// 82E47840: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E47844: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E47848: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82E4784C: 7D675214  add r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82E47850: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47854: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E47858: 913F006C  stw r9, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82E4785C: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47860: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	pc = 0x82E47894; continue 'dispatch;
            }
            0x82E47894 => {
    //   block [0x82E47894..0x82E478A4)
	// 82E47894: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82E47898: 38E7FFD0  addi r7, r7, -0x30
	ctx.r[7].s64 = ctx.r[7].s64 + -48;
	// 82E4789C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E478A0: 4098FF6C  bge cr6, 0x82e4780c
	if !ctx.cr[6].lt {
	pc = 0x82E4780C; continue 'dispatch;
	}
	pc = 0x82E478A4; continue 'dispatch;
            }
            0x82E478A4 => {
    //   block [0x82E478A4..0x82E47910)
	// 82E478A4: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E478A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E478AC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E478B0: 40990024  ble cr6, 0x82e478d4
	if !ctx.cr[6].gt {
	pc = 0x82E478D4; continue 'dispatch;
	}
	// 82E478B4: 815F008C  lwz r10, 0x8c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E478B8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E478BC: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E478C0: 419A0018  beq cr6, 0x82e478d8
	if ctx.cr[6].eq {
	pc = 0x82E478D8; continue 'dispatch;
	}
	// 82E478C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E478C8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E478CC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E478D0: 4198FFE8  blt cr6, 0x82e478b8
	if ctx.cr[6].lt {
	pc = 0x82E478B8; continue 'dispatch;
	}
	// 82E478D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E478D8: 815F0090  lwz r10, 0x90(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E478DC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E478E0: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E478E4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E478E8: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E478EC: 915F0090  stw r10, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82E478F0: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E478F4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E478F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E478FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E47900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E47904: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E47908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4790C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47910 size=304
    let mut pc: u32 = 0x82E47910;
    'dispatch: loop {
        match pc {
            0x82E47910 => {
    //   block [0x82E47910..0x82E4797C)
	// 82E47910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47914: 4BE61AF5  bl 0x82ca9408
	ctx.lr = 0x82E47918;
	sub_82CA93D0(ctx, base);
	// 82E47918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4791C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E47924: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E47928: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4792C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47930: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E47934: 3BAB5DB8  addi r29, r11, 0x5db8
	ctx.r[29].s64 = ctx.r[11].s64 + 23992;
	// 82E47938: 409A0044  bne cr6, 0x82e4797c
	if !ctx.cr[6].eq {
	pc = 0x82E4797C; continue 'dispatch;
	}
	// 82E4793C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47940: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E47944: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E47948: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E4794C: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47950: 80DF0074  lwz r6, 0x74(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E47954: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E47958: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82E4795C: 83890008  lwz r28, 8(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47960: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E47964: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E47968: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E4796C: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47970: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E47974: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 82E47978: 4E800421  bctrl
	ctx.lr = 0x82E4797C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4797C => {
    //   block [0x82E4797C..0x82E479BC)
	// 82E4797C: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E47980: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47984: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47988: 409A0034  bne cr6, 0x82e479bc
	if !ctx.cr[6].eq {
	pc = 0x82E479BC; continue 'dispatch;
	}
	// 82E4798C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47990: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E47994: 80FF0090  lwz r7, 0x90(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47998: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E4799C: 388B5DA8  addi r4, r11, 0x5da8
	ctx.r[4].s64 = ctx.r[11].s64 + 23976;
	// 82E479A0: 80DF008C  lwz r6, 0x8c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E479A4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E479A8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E479AC: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E479B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E479B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E479B8: 4E800421  bctrl
	ctx.lr = 0x82E479BC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E479BC => {
    //   block [0x82E479BC..0x82E479F8)
	// 82E479BC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E479C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E479C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E479C8: 409A0030  bne cr6, 0x82e479f8
	if !ctx.cr[6].eq {
	pc = 0x82E479F8; continue 'dispatch;
	}
	// 82E479CC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E479D0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E479D4: 813F009C  lwz r9, 0x9c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E479D8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E479DC: 80DF0098  lwz r6, 0x98(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E479E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E479E4: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E479E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E479EC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E479F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E479F4: 4E800421  bctrl
	ctx.lr = 0x82E479F8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E479F8 => {
    //   block [0x82E479F8..0x82E47A38)
	// 82E479F8: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E479FC: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47A00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47A04: 409A0034  bne cr6, 0x82e47a38
	if !ctx.cr[6].eq {
	pc = 0x82E47A38; continue 'dispatch;
	}
	// 82E47A08: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47A0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E47A10: 80FF0084  lwz r7, 0x84(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47A14: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47A18: 388B5D9C  addi r4, r11, 0x5d9c
	ctx.r[4].s64 = ctx.r[11].s64 + 23964;
	// 82E47A1C: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47A20: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E47A24: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E47A28: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47A2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E47A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47A34: 4E800421  bctrl
	ctx.lr = 0x82E47A38;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E47A38 => {
    //   block [0x82E47A38..0x82E47A40)
	// 82E47A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E47A3C: 4BE61A1C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E47A40 size=280
    let mut pc: u32 = 0x82E47A40;
    'dispatch: loop {
        match pc {
            0x82E47A40 => {
    //   block [0x82E47A40..0x82E47B58)
	// 82E47A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47A44: 4BE619C9  bl 0x82ca940c
	ctx.lr = 0x82E47A48;
	sub_82CA93D0(ctx, base);
	// 82E47A48: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E47A4C: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47A50: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E47A54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E47A58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E47A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47A60: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82E47A64: C3EB0C64  lfs f31, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E47A68: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82E47A6C: D3FE0004  stfs f31, 4(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E47A70: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E47A74: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82E47A78: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E47A7C: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E47A80: C01F005C  lfs f0, 0x5c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E47A84: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E47A88: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E47A8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47A90: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E47A94: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82E47A98: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47A9C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82E47AA0: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47AA4: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82E47AA8: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47AAC: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82E47AB0: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E47AB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82E47AB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47ABC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E47AC0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E47B58 size=2872
    let mut pc: u32 = 0x82E47B58;
    'dispatch: loop {
        match pc {
            0x82E47B58 => {
    //   block [0x82E47B58..0x82E47B98)
	// 82E47B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47B5C: 4BE61879  bl 0x82ca93d4
	ctx.lr = 0x82E47B60;
	sub_82CA93D0(ctx, base);
	// 82E47B60: DBA1FF58  stfd f29, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[29].u64 ) };
	// 82E47B64: DBC1FF60  stfd f30, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[30].u64 ) };
	// 82E47B68: DBE1FF68  stfd f31, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 82E47B6C: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47B70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E47B74: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82E47B78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E47B7C: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82E47B80: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 82E47B84: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47B88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47B8C: 40990030  ble cr6, 0x82e47bbc
	if !ctx.cr[6].gt {
	pc = 0x82E47BBC; continue 'dispatch;
	}
	// 82E47B90: 3B7E0008  addi r27, r30, 8
	ctx.r[27].s64 = ctx.r[30].s64 + 8;
	// 82E47B94: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	pc = 0x82E47B98; continue 'dispatch;
            }
            0x82E47B98 => {
    //   block [0x82E47B98..0x82E47BBC)
	// 82E47B98: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E47B9C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E47BA0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E47BA4: 4BF3E275  bl 0x82d85e18
	ctx.lr = 0x82E47BA8;
	sub_82D85E18(ctx, base);
	// 82E47BA8: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47BAC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E47BB0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E47BB4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E47BB8: 4198FFE0  blt cr6, 0x82e47b98
	if ctx.cr[6].lt {
	pc = 0x82E47B98; continue 'dispatch;
	}
	pc = 0x82E47BBC; continue 'dispatch;
            }
            0x82E47BBC => {
    //   block [0x82E47BBC..0x82E47BD8)
	// 82E47BBC: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E47BC0: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 82E47BC4: 927E0090  stw r19, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[19].u32 ) };
	// 82E47BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47BCC: 40990030  ble cr6, 0x82e47bfc
	if !ctx.cr[6].gt {
	pc = 0x82E47BFC; continue 'dispatch;
	}
	// 82E47BD0: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 82E47BD4: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	pc = 0x82E47BD8; continue 'dispatch;
            }
            0x82E47BD8 => {
    //   block [0x82E47BD8..0x82E47BFC)
	// 82E47BD8: 817E0098  lwz r11, 0x98(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E47BDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E47BE0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E47BE4: 4BF51CCD  bl 0x82d998b0
	ctx.lr = 0x82E47BE8;
	sub_82D998B0(ctx, base);
	// 82E47BE8: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E47BEC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E47BF0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E47BF4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E47BF8: 4198FFE0  blt cr6, 0x82e47bd8
	if ctx.cr[6].lt {
	pc = 0x82E47BD8; continue 'dispatch;
	}
	pc = 0x82E47BFC; continue 'dispatch;
            }
            0x82E47BFC => {
    //   block [0x82E47BFC..0x82E47C4C)
	// 82E47BFC: 927E009C  stw r19, 0x9c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(156 as u32), ctx.r[19].u32 ) };
	// 82E47C00: 3A400004  li r18, 4
	ctx.r[18].s64 = 4;
	// 82E47C04: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E47C08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E47C0C: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47C10: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E47C14: 7D5D5214  add r10, r29, r10
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 82E47C18: C3EB0C64  lfs f31, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E47C1C: 7C72882E  lwzx r3, r18, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82E47C20: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E47C24: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E47C28: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47C2C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E47C30: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E47C34: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82E47C38: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E47C3C: 41990010  bgt cr6, 0x82e47c4c
	if ctx.cr[6].gt {
	pc = 0x82E47C4C; continue 'dispatch;
	}
	// 82E47C40: 7D745B78  mr r20, r11
	ctx.r[20].u64 = ctx.r[11].u64;
	// 82E47C44: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E47C48: 48000018  b 0x82e47c60
	pc = 0x82E47C60; continue 'dispatch;
            }
            0x82E47C4C => {
    //   block [0x82E47C4C..0x82E47C60)
	// 82E47C4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47C50: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E47C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47C58: 4E800421  bctrl
	ctx.lr = 0x82E47C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E47C5C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
            }
            0x82E47C60 => {
    //   block [0x82E47C60..0x82E47DF4)
	// 82E47C60: 82FF0014  lwz r23, 0x14(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E47C64: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 82E47C68: 67B08000  oris r16, r29, 0x8000
	ctx.r[16].u64 = ctx.r[29].u64 | 2147483648;
	// 82E47C6C: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82E47C70: 3B00FFE0  li r24, -0x20
	ctx.r[24].s64 = -32;
	// 82E47C74: 3B20FFF0  li r25, -0x10
	ctx.r[25].s64 = -16;
	// 82E47C78: 2F170004  cmpwi cr6, r23, 4
	ctx.cr[6].compare_i32(ctx.r[23].s32, 4, &mut ctx.xer);
	// 82E47C7C: 41980178  blt cr6, 0x82e47df4
	if ctx.cr[6].lt {
	pc = 0x82E47DF4; continue 'dispatch;
	}
	// 82E47C80: 3977FFFC  addi r11, r23, -4
	ctx.r[11].s64 = ctx.r[23].s64 + -4;
	// 82E47C84: 20B40010  subfic r5, r20, 0x10
	ctx.xer.ca = ctx.r[20].u32 <= 16 as u32;
	ctx.r[5].s64 = (16 as i64) - ctx.r[20].s64;
	// 82E47C88: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47C8C: 21140040  subfic r8, r20, 0x40
	ctx.xer.ca = ctx.r[20].u32 <= 64 as u32;
	ctx.r[8].s64 = (64 as i64) - ctx.r[20].s64;
	// 82E47C90: 3B4B0001  addi r26, r11, 1
	ctx.r[26].s64 = ctx.r[11].s64 + 1;
	// 82E47C94: 39740050  addi r11, r20, 0x50
	ctx.r[11].s64 = ctx.r[20].s64 + 80;
	// 82E47C98: 20F4FFB0  subfic r7, r20, -0x50
	ctx.xer.ca = ctx.r[20].u32 <= -80 as u32;
	ctx.r[7].s64 = (-80 as i64) - ctx.r[20].s64;
	// 82E47C9C: 5746103A  slwi r6, r26, 2
	ctx.r[6].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E47CA0: 3B80FFB0  li r28, -0x50
	ctx.r[28].s64 = -80;
	// 82E47CA4: 3BA0FFC0  li r29, -0x40
	ctx.r[29].s64 = -64;
	// 82E47CA8: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82E47CAC: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82E47CB0: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82E47CB4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47CB8: 7D275A14  add r9, r7, r11
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82E47CBC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	pc = 0x82E47DF4; continue 'dispatch;
            }
            0x82E47DF4 => {
    //   block [0x82E47DF4..0x82E48690)
	// 82E47DF4: 7F06B800  cmpw cr6, r6, r23
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82E47DF8: 4098007C  bge cr6, 0x82e47e74
	if !ctx.cr[6].lt {
	pc = 0x82E47E74; continue 'dispatch;
	}
	// 82E47DFC: 54CB083C  slwi r11, r6, 1
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47E00: 2114FFE0  subfic r8, r20, -0x20
	ctx.xer.ca = ctx.r[20].u32 <= -32 as u32;
	ctx.r[8].s64 = (-32 as i64) - ctx.r[20].s64;
	// 82E47E04: 7D665A14  add r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82E47E08: 7CE6B850  subf r7, r6, r23
	ctx.r[7].s64 = ctx.r[23].s64 - ctx.r[6].s64;
	// 82E47E0C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47E10: 7D6BA214  add r11, r11, r20
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 82E47E14: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82E47E18: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47E1C: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E47E20: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E48690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E48690 size=2868
    let mut pc: u32 = 0x82E48690;
    'dispatch: loop {
        match pc {
            0x82E48690 => {
    //   block [0x82E48690..0x82E48704)
	// 82E48690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E48694: 4BE60D3D  bl 0x82ca93d0
	ctx.lr = 0x82E48698;
	sub_82CA93D0(ctx, base);
	// 82E48698: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82E4869C: 4BE65631  bl 0x82cadccc
	ctx.lr = 0x82E486A0;
	sub_82CADCA0(ctx, base);
	// 82E486A0: 9421FDB0  stwu r1, -0x250(r1)
	ea = ctx.r[1].u32.wrapping_add(-592 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E486A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E486A8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E486AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E486B0: 9081026C  stw r4, 0x26c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(620 as u32), ctx.r[4].u32 ) };
	// 82E486B4: 7DEA5A14  add r15, r10, r11
	ctx.r[15].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E486B8: 90A10274  stw r5, 0x274(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(628 as u32), ctx.r[5].u32 ) };
	// 82E486BC: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82E486C0: 7CF03B78  mr r16, r7
	ctx.r[16].u64 = ctx.r[7].u64;
	// 82E486C4: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E486C8: 814F0000  lwz r10, 0(r15)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E486CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E486D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E486D4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E486D8: 4098002C  bge cr6, 0x82e48704
	if !ctx.cr[6].lt {
	pc = 0x82E48704; continue 'dispatch;
	}
	// 82E486DC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E486E0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E486E4: 39295E14  addi r9, r9, 0x5e14
	ctx.r[9].s64 = ctx.r[9].s64 + 24084;
	// 82E486E8: 39085E0C  addi r8, r8, 0x5e0c
	ctx.r[8].s64 = ctx.r[8].s64 + 24076;
	// 82E486EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E486F0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82E486F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E486F8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82E486FC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E48700: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82E48704; continue 'dispatch;
            }
            0x82E48704 => {
    //   block [0x82E48704..0x82E491C4)
	// 82E48704: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E48708: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E4870C: C01F005C  lfs f0, 0x5c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E48710: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82E48714: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E48718: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82E4871C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E48720: C3C40008  lfs f30, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E48724: 39DF0030  addi r14, r31, 0x30
	ctx.r[14].s64 = ctx.r[31].s64 + 48;
	// 82E48728: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4872C: 3A600010  li r19, 0x10
	ctx.r[19].s64 = 16;
	// 82E48730: 92810060  stw r20, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E491C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E491C8 size=1916
    let mut pc: u32 = 0x82E491C8;
    'dispatch: loop {
        match pc {
            0x82E491C8 => {
    //   block [0x82E491C8..0x82E49220)
	// 82E491C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E491CC: 4BE60221  bl 0x82ca93ec
	ctx.lr = 0x82E491D0;
	sub_82CA93D0(ctx, base);
	// 82E491D0: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82E491D4: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E491D8: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E491DC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E491E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E491E4: 7EBE5A14  add r21, r30, r11
	ctx.r[21].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E491E8: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82E491EC: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82E491F0: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E491F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E491F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E491FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E49200: 40980020  bge cr6, 0x82e49220
	if !ctx.cr[6].lt {
	pc = 0x82E49220; continue 'dispatch;
	}
	// 82E49204: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E49208: 39295E28  addi r9, r9, 0x5e28
	ctx.r[9].s64 = ctx.r[9].s64 + 24104;
	// 82E4920C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E49210: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E49214: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82E49218: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E4921C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82E49220; continue 'dispatch;
            }
            0x82E49220 => {
    //   block [0x82E49220..0x82E49238)
	// 82E49220: 897F00B0  lbz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E49224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E49228: 419A0010  beq cr6, 0x82e49238
	if ctx.cr[6].eq {
	pc = 0x82E49238; continue 'dispatch;
	}
	// 82E4922C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E49230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E49234: 4BFFE80D  bl 0x82e47a40
	ctx.lr = 0x82E49238;
	sub_82E47A40(ctx, base);
	pc = 0x82E49238; continue 'dispatch;
            }
            0x82E49238 => {
    //   block [0x82E49238..0x82E49290)
	// 82E49238: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E4923C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E49240: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49244: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E49248: 7F1E4A14  add r24, r30, r9
	ctx.r[24].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 82E4924C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E49250: 3F408000  lis r26, -0x8000
	ctx.r[26].s64 = -2147483648;
	// 82E49254: 3BCB000A  addi r30, r11, 0xa
	ctx.r[30].s64 = ctx.r[11].s64 + 10;
	// 82E49258: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 82E4925C: 57CA3032  slwi r10, r30, 6
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E49260: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49264: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82E49268: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E4926C: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82E49270: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E49274: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E49278: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E4927C: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E49280: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E49284: 4199000C  bgt cr6, 0x82e49290
	if ctx.cr[6].gt {
	pc = 0x82E49290; continue 'dispatch;
	}
	// 82E49288: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E4928C: 4800001C  b 0x82e492a8
	pc = 0x82E492A8; continue 'dispatch;
            }
            0x82E49290 => {
    //   block [0x82E49290..0x82E492A8)
	// 82E49290: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49294: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E49298: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4929C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E492A0: 4E800421  bctrl
	ctx.lr = 0x82E492A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E492A4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
            }
            0x82E492A8 => {
    //   block [0x82E492A8..0x82E492E4)
	// 82E492A8: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E492AC: 7FC9D378  or r9, r30, r26
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[26].u64;
	// 82E492B0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E492B4: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E492B8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82E492BC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E492C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E492C4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E492C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E492CC: 40990070  ble cr6, 0x82e4933c
	if !ctx.cr[6].gt {
	pc = 0x82E4933C; continue 'dispatch;
	}
	// 82E492D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E492D4: 3B7F0040  addi r27, r31, 0x40
	ctx.r[27].s64 = ctx.r[31].s64 + 64;
	// 82E492D8: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E492DC: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E492E0: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x82E492E4; continue 'dispatch;
            }
            0x82E492E4 => {
    //   block [0x82E492E4..0x82E4933C)
	// 82E492E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E492E8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E492EC: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E492F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E492F4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E492F8: 7C9C5A14  add r4, r28, r11
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82E492FC: 7CBD4A14  add r5, r29, r9
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 82E49300: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E49304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49308: 4E800421  bctrl
	ctx.lr = 0x82E4930C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4930C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E49310: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E49314: C03F00A4  lfs f1, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E49318: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E4931C: 4BFFD58D  bl 0x82e468a8
	ctx.lr = 0x82E49320;
	sub_82E468A8(ctx, base);
	// 82E49320: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49324: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E49328: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 82E4932C: 3B9C0030  addi r28, r28, 0x30
	ctx.r[28].s64 = ctx.r[28].s64 + 48;
	// 82E49330: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49334: 4198FFB0  blt cr6, 0x82e492e4
	if ctx.cr[6].lt {
	pc = 0x82E492E4; continue 'dispatch;
	}
	// 82E49338: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
            }
            0x82E4933C => {
    //   block [0x82E4933C..0x82E49370)
	// 82E4933C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E49340: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49344: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E49348: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E4934C: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49350: 40980034  bge cr6, 0x82e49384
	if !ctx.cr[6].lt {
	pc = 0x82E49384; continue 'dispatch;
	}
	// 82E49354: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E49358: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4935C: 40980028  bge cr6, 0x82e49384
	if !ctx.cr[6].lt {
	pc = 0x82E49384; continue 'dispatch;
	}
	// 82E49360: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E49364: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49368: 41980008  blt cr6, 0x82e49370
	if ctx.cr[6].lt {
	pc = 0x82E49370; continue 'dispatch;
	}
	// 82E4936C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x82E49370; continue 'dispatch;
            }
            0x82E49370 => {
    //   block [0x82E49370..0x82E49384)
	// 82E49370: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82E49374: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E49378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4937C: 4BF0DB95  bl 0x82d56f10
	ctx.lr = 0x82E49380;
	sub_82D56F10(ctx, base);
	// 82E49380: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	pc = 0x82E49384; continue 'dispatch;
            }
            0x82E49384 => {
    //   block [0x82E49384..0x82E493C0)
	// 82E49384: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49388: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4938C: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E49390: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E49394: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82E49398: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 82E4939C: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82E493A0: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E493A4: 55442036  slwi r4, r10, 4
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E493A8: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E493AC: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82E493B0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E493B4: 4199000C  bgt cr6, 0x82e493c0
	if ctx.cr[6].gt {
	pc = 0x82E493C0; continue 'dispatch;
	}
	// 82E493B8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E493BC: 48000018  b 0x82e493d4
	pc = 0x82E493D4; continue 'dispatch;
            }
            0x82E493C0 => {
    //   block [0x82E493C0..0x82E493D4)
	// 82E493C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E493C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E493C8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E493CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E493D0: 4E800421  bctrl
	ctx.lr = 0x82E493D4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E493D4 => {
    //   block [0x82E493D4..0x82E49424)
	// 82E493D4: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E493D8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E493DC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E493E0: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E493E4: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82E493E8: 7FAAD378  or r10, r29, r26
	ctx.r[10].u64 = ctx.r[29].u64 | ctx.r[26].u64;
	// 82E493EC: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82E493F0: 393E0001  addi r9, r30, 1
	ctx.r[9].s64 = ctx.r[30].s64 + 1;
	// 82E493F4: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82E493F8: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 82E493FC: 93410088  stw r26, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[26].u32 ) };
	// 82E49400: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E49404: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82E49408: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4940C: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E49410: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E49414: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E49418: 4199000C  bgt cr6, 0x82e49424
	if ctx.cr[6].gt {
	pc = 0x82E49424; continue 'dispatch;
	}
	// 82E4941C: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E49420: 4800001C  b 0x82e4943c
	pc = 0x82E4943C; continue 'dispatch;
            }
            0x82E49424 => {
    //   block [0x82E49424..0x82E4943C)
	// 82E49424: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49428: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E4942C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E49430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49434: 4E800421  bctrl
	ctx.lr = 0x82E49438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49438: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
            }
            0x82E4943C => {
    //   block [0x82E4943C..0x82E49944)
	// 82E4943C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E49948 size=1424
    let mut pc: u32 = 0x82E49948;
    'dispatch: loop {
        match pc {
            0x82E49948 => {
    //   block [0x82E49948..0x82E499A0)
	// 82E49948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4994C: 4BE5FAA1  bl 0x82ca93ec
	ctx.lr = 0x82E49950;
	sub_82CA93D0(ctx, base);
	// 82E49950: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82E49954: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E49958: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4995C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E49960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E49964: 7EA85A14  add r21, r8, r11
	ctx.r[21].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E49968: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82E4996C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82E49970: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49974: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E49978: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4997C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E49980: 40980020  bge cr6, 0x82e499a0
	if !ctx.cr[6].lt {
	pc = 0x82E499A0; continue 'dispatch;
	}
	// 82E49984: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E49988: 39295E28  addi r9, r9, 0x5e28
	ctx.r[9].s64 = ctx.r[9].s64 + 24104;
	// 82E4998C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E49990: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E49994: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82E49998: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E4999C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82E499A0; continue 'dispatch;
            }
            0x82E499A0 => {
    //   block [0x82E499A0..0x82E499F8)
	// 82E499A0: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E499A4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E499A8: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E499AC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E499B0: 7F084A14  add r24, r8, r9
	ctx.r[24].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82E499B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E499B8: 3F408000  lis r26, -0x8000
	ctx.r[26].s64 = -2147483648;
	// 82E499BC: 3BCB000A  addi r30, r11, 0xa
	ctx.r[30].s64 = ctx.r[11].s64 + 10;
	// 82E499C0: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82E499C4: 57CA3032  slwi r10, r30, 6
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E499C8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E499CC: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82E499D0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E499D4: 93410068  stw r26, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 82E499D8: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E499DC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E499E0: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E499E4: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E499E8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E499EC: 4199000C  bgt cr6, 0x82e499f8
	if ctx.cr[6].gt {
	pc = 0x82E499F8; continue 'dispatch;
	}
	// 82E499F0: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E499F4: 4800001C  b 0x82e49a10
	pc = 0x82E49A10; continue 'dispatch;
            }
            0x82E499F8 => {
    //   block [0x82E499F8..0x82E49A10)
	// 82E499F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E499FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E49A00: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E49A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49A08: 4E800421  bctrl
	ctx.lr = 0x82E49A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49A0C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
            }
            0x82E49A10 => {
    //   block [0x82E49A10..0x82E49A4C)
	// 82E49A10: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49A14: 7FC9D378  or r9, r30, r26
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[26].u64;
	// 82E49A18: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E49A1C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E49A20: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82E49A24: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E49A28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E49A2C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82E49A30: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82E49A34: 40990070  ble cr6, 0x82e49aa4
	if !ctx.cr[6].gt {
	pc = 0x82E49AA4; continue 'dispatch;
	}
	// 82E49A38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49A3C: 3B7F0040  addi r27, r31, 0x40
	ctx.r[27].s64 = ctx.r[31].s64 + 64;
	// 82E49A40: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E49A44: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E49A48: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x82E49A4C; continue 'dispatch;
            }
            0x82E49A4C => {
    //   block [0x82E49A4C..0x82E49AA4)
	// 82E49A4C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49A50: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E49A54: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E49A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E49A5C: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E49A60: 7C9C5A14  add r4, r28, r11
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82E49A64: 7CBD4A14  add r5, r29, r9
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 82E49A68: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E49A6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49A70: 4E800421  bctrl
	ctx.lr = 0x82E49A74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49A74: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E49A78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E49A7C: C03F00A4  lfs f1, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E49A80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E49A84: 4BFFCE25  bl 0x82e468a8
	ctx.lr = 0x82E49A88;
	sub_82E468A8(ctx, base);
	// 82E49A88: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49A8C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E49A90: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 82E49A94: 3B9C0030  addi r28, r28, 0x30
	ctx.r[28].s64 = ctx.r[28].s64 + 48;
	// 82E49A98: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49A9C: 4198FFB0  blt cr6, 0x82e49a4c
	if ctx.cr[6].lt {
	pc = 0x82E49A4C; continue 'dispatch;
	}
	// 82E49AA0: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
            }
            0x82E49AA4 => {
    //   block [0x82E49AA4..0x82E49AD8)
	// 82E49AA4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E49AA8: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49AAC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E49AB0: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E49AB4: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49AB8: 40980034  bge cr6, 0x82e49aec
	if !ctx.cr[6].lt {
	pc = 0x82E49AEC; continue 'dispatch;
	}
	// 82E49ABC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E49AC0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49AC4: 40980028  bge cr6, 0x82e49aec
	if !ctx.cr[6].lt {
	pc = 0x82E49AEC; continue 'dispatch;
	}
	// 82E49AC8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E49ACC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49AD0: 41980008  blt cr6, 0x82e49ad8
	if ctx.cr[6].lt {
	pc = 0x82E49AD8; continue 'dispatch;
	}
	// 82E49AD4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x82E49AD8; continue 'dispatch;
            }
            0x82E49AD8 => {
    //   block [0x82E49AD8..0x82E49AEC)
	// 82E49AD8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82E49ADC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E49AE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E49AE4: 4BF0D42D  bl 0x82d56f10
	ctx.lr = 0x82E49AE8;
	sub_82D56F10(ctx, base);
	// 82E49AE8: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	pc = 0x82E49AEC; continue 'dispatch;
            }
            0x82E49AEC => {
    //   block [0x82E49AEC..0x82E49B28)
	// 82E49AEC: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49AF0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49AF4: 7FC95214  add r30, r9, r10
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E49AF8: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E49AFC: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82E49B00: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82E49B04: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82E49B08: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E49B0C: 55442036  slwi r4, r10, 4
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E49B10: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E49B14: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82E49B18: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E49B1C: 4199000C  bgt cr6, 0x82e49b28
	if ctx.cr[6].gt {
	pc = 0x82E49B28; continue 'dispatch;
	}
	// 82E49B20: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E49B24: 48000018  b 0x82e49b3c
	pc = 0x82E49B3C; continue 'dispatch;
            }
            0x82E49B28 => {
    //   block [0x82E49B28..0x82E49B3C)
	// 82E49B28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49B2C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E49B30: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E49B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49B38: 4E800421  bctrl
	ctx.lr = 0x82E49B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E49B3C => {
    //   block [0x82E49B3C..0x82E49B74)
	// 82E49B3C: 7FCBD378  or r11, r30, r26
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[26].u64;
	// 82E49B40: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49B44: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49B48: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82E49B4C: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E49B50: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82E49B54: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E49B58: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E49B5C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82E49B60: 40980024  bge cr6, 0x82e49b84
	if !ctx.cr[6].lt {
	pc = 0x82E49B84; continue 'dispatch;
	}
	// 82E49B64: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E49B68: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49B6C: 41980008  blt cr6, 0x82e49b74
	if ctx.cr[6].lt {
	pc = 0x82E49B74; continue 'dispatch;
	}
	// 82E49B70: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82E49B74; continue 'dispatch;
            }
            0x82E49B74 => {
    //   block [0x82E49B74..0x82E49B84)
	// 82E49B74: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E49B78: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E49B7C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E49B80: 4BF0D391  bl 0x82d56f10
	ctx.lr = 0x82E49B84;
	sub_82D56F10(ctx, base);
	pc = 0x82E49B84; continue 'dispatch;
            }
            0x82E49B84 => {
    //   block [0x82E49B84..0x82E49ED8)
	// 82E49B84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E49ED8 size=132
    let mut pc: u32 = 0x82E49ED8;
    'dispatch: loop {
        match pc {
            0x82E49ED8 => {
    //   block [0x82E49ED8..0x82E49F5C)
	// 82E49ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E49EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E49EE0: 9421FC60  stwu r1, -0x3a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-928 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E49EE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49EE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E49EEC: 394B0E2C  addi r10, r11, 0xe2c
	ctx.r[10].s64 = ctx.r[11].s64 + 3628;
	// 82E49EF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49EF4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E49EF8: 38C101F0  addi r6, r1, 0x1f0
	ctx.r[6].s64 = ctx.r[1].s64 + 496;
	// 82E49EFC: 91210204  stw r9, 0x204(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(516 as u32), ctx.r[9].u32 ) };
	// 82E49F00: 914101F0  stw r10, 0x1f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(496 as u32), ctx.r[10].u32 ) };
	// 82E49F04: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E49F08: 39610210  addi r11, r1, 0x210
	ctx.r[11].s64 = ctx.r[1].s64 + 528;
	// 82E49F0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E49F10: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E49F14: D00101F4  stfs f0, 0x1f4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), tmp.u32 ) };
	// 82E49F18: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82E49F1C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E49F20: 91610200  stw r11, 0x200(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), ctx.r[11].u32 ) };
	// 82E49F24: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E49F28: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E49F2C: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82E49F30: 91610208  stw r11, 0x208(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(520 as u32), ctx.r[11].u32 ) };
	// 82E49F34: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E49F38: 4BFFE759  bl 0x82e48690
	ctx.lr = 0x82E49F3C;
	sub_82E48690(ctx, base);
	// 82E49F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E49F40: 4B4159C9  bl 0x8225f908
	ctx.lr = 0x82E49F44;
	sub_8225F908(ctx, base);
	// 82E49F44: 386101F0  addi r3, r1, 0x1f0
	ctx.r[3].s64 = ctx.r[1].s64 + 496;
	// 82E49F48: 4B4159C1  bl 0x8225f908
	ctx.lr = 0x82E49F4C;
	sub_8225F908(ctx, base);
	// 82E49F4C: 382103A0  addi r1, r1, 0x3a0
	ctx.r[1].s64 = ctx.r[1].s64 + 928;
	// 82E49F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E49F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E49F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E49F60 size=96
    let mut pc: u32 = 0x82E49F60;
    'dispatch: loop {
        match pc {
            0x82E49F60 => {
    //   block [0x82E49F60..0x82E49FC0)
	// 82E49F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E49F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E49F68: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E49F6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49F70: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E49F74: 396B0E2C  addi r11, r11, 0xe2c
	ctx.r[11].s64 = ctx.r[11].s64 + 3628;
	// 82E49F78: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E49F7C: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E49F80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E49F84: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E49F88: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E49F8C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E49F90: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E49F94: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82E49F98: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E49F9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E49FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E49FA4: 4BFFF225  bl 0x82e491c8
	ctx.lr = 0x82E49FA8;
	sub_82E491C8(ctx, base);
	// 82E49FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E49FAC: 4B41595D  bl 0x8225f908
	ctx.lr = 0x82E49FB0;
	sub_8225F908(ctx, base);
	// 82E49FB0: 38210200  addi r1, r1, 0x200
	ctx.r[1].s64 = ctx.r[1].s64 + 512;
	// 82E49FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E49FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E49FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E49FC0 size=64
    let mut pc: u32 = 0x82E49FC0;
    'dispatch: loop {
        match pc {
            0x82E49FC0 => {
    //   block [0x82E49FC0..0x82E49FD8)
	// 82E49FC0: 81240078  lwz r9, 0x78(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49FC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E49FC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E49FCC: 40990028  ble cr6, 0x82e49ff4
	if !ctx.cr[6].gt {
	pc = 0x82E49FF4; continue 'dispatch;
	}
	// 82E49FD0: 81040074  lwz r8, 0x74(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E49FD4: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	pc = 0x82E49FD8; continue 'dispatch;
            }
            0x82E49FD8 => {
    //   block [0x82E49FD8..0x82E49FF4)
	// 82E49FD8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49FDC: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82E49FE0: 419A0020  beq cr6, 0x82e4a000
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82E4A000);
		return;
	}
	// 82E49FE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E49FE8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E49FEC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49FF0: 4198FFE8  blt cr6, 0x82e49fd8
	if ctx.cr[6].lt {
	pc = 0x82E49FD8; continue 'dispatch;
	}
	pc = 0x82E49FF4; continue 'dispatch;
            }
            0x82E49FF4 => {
    //   block [0x82E49FF4..0x82E4A000)
	// 82E49FF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E49FF8: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82E49FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A018 size=8
    let mut pc: u32 = 0x82E4A018;
    'dispatch: loop {
        match pc {
            0x82E4A018 => {
    //   block [0x82E4A018..0x82E4A020)
	// 82E4A018: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A01C: 4800000C  b 0x82e4a028
	sub_82E4A028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A020 size=8
    let mut pc: u32 = 0x82E4A020;
    'dispatch: loop {
        match pc {
            0x82E4A020 => {
    //   block [0x82E4A020..0x82E4A028)
	// 82E4A020: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A024: 48000004  b 0x82e4a028
	sub_82E4A028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A028 size=100
    let mut pc: u32 = 0x82E4A028;
    'dispatch: loop {
        match pc {
            0x82E4A028 => {
    //   block [0x82E4A028..0x82E4A070)
	// 82E4A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A044: 4BFFD3F5  bl 0x82e47438
	ctx.lr = 0x82E4A048;
	sub_82E47438(ctx, base);
	// 82E4A048: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A04C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A050: 419A0020  beq cr6, 0x82e4a070
	if ctx.cr[6].eq {
	pc = 0x82E4A070; continue 'dispatch;
	}
	// 82E4A054: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A058: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A05C: 38C00033  li r6, 0x33
	ctx.r[6].s64 = 51;
	// 82E4A060: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A064: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A068: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A06C: 4BF0B25D  bl 0x82d552c8
	ctx.lr = 0x82E4A070;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A070; continue 'dispatch;
            }
            0x82E4A070 => {
    //   block [0x82E4A070..0x82E4A08C)
	// 82E4A070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A074: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A080: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A090 size=20
    let mut pc: u32 = 0x82E4A090;
    'dispatch: loop {
        match pc {
            0x82E4A090 => {
    //   block [0x82E4A090..0x82E4A0A4)
	// 82E4A090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A094: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A098: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A09C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A0A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A0C0 size=44
    let mut pc: u32 = 0x82E4A0C0;
    'dispatch: loop {
        match pc {
            0x82E4A0C0 => {
    //   block [0x82E4A0C0..0x82E4A0EC)
	// 82E4A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A0C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A0CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A0D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4A0D4: 4800001D  bl 0x82e4a0f0
	ctx.lr = 0x82E4A0D8;
	sub_82E4A0F0(ctx, base);
	// 82E4A0D8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4A0DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4A0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A0F0 size=168
    let mut pc: u32 = 0x82E4A0F0;
    'dispatch: loop {
        match pc {
            0x82E4A0F0 => {
    //   block [0x82E4A0F0..0x82E4A198)
	// 82E4A0F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A0F4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E4A0F8: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4A0FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A100: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4A104: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4A108: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E4A10C: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4A110: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4A114: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A118: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4A11C: 38CB5EB8  addi r6, r11, 0x5eb8
	ctx.r[6].s64 = ctx.r[11].s64 + 24248;
	// 82E4A120: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A124: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A128: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A12C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4A130: 38AB5EAC  addi r5, r11, 0x5eac
	ctx.r[5].s64 = ctx.r[11].s64 + 24236;
	// 82E4A134: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A138: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4A13C: 394B5E98  addi r10, r11, 0x5e98
	ctx.r[10].s64 = ctx.r[11].s64 + 24216;
	// 82E4A140: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A144: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4A148: 38E75D40  addi r7, r7, 0x5d40
	ctx.r[7].s64 = ctx.r[7].s64 + 23872;
	// 82E4A14C: 392B5E8C  addi r9, r11, 0x5e8c
	ctx.r[9].s64 = ctx.r[11].s64 + 24204;
	// 82E4A150: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A154: 388B5E80  addi r4, r11, 0x5e80
	ctx.r[4].s64 = ctx.r[11].s64 + 24192;
	// 82E4A158: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A15C: 390B5E68  addi r8, r11, 0x5e68
	ctx.r[8].s64 = ctx.r[11].s64 + 24168;
	// 82E4A160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4A164: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E4A168: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E4A16C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A170: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E4A174: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E4A178: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E4A17C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A180: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E4A184: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E4A188: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E4A18C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4A190: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E4A194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A198 size=100
    let mut pc: u32 = 0x82E4A198;
    'dispatch: loop {
        match pc {
            0x82E4A198 => {
    //   block [0x82E4A198..0x82E4A1E0)
	// 82E4A198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A1AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A1B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A1B4: 48008DE5  bl 0x82e52f98
	ctx.lr = 0x82E4A1B8;
	sub_82E52F98(ctx, base);
	// 82E4A1B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A1C0: 419A0020  beq cr6, 0x82e4a1e0
	if ctx.cr[6].eq {
	pc = 0x82E4A1E0; continue 'dispatch;
	}
	// 82E4A1C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A1C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A1CC: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4A1D0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A1D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A1D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A1DC: 4BF0B0ED  bl 0x82d552c8
	ctx.lr = 0x82E4A1E0;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A1E0; continue 'dispatch;
            }
            0x82E4A1E0 => {
    //   block [0x82E4A1E0..0x82E4A1FC)
	// 82E4A1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A1E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A1F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A1F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A200 size=8
    let mut pc: u32 = 0x82E4A200;
    'dispatch: loop {
        match pc {
            0x82E4A200 => {
    //   block [0x82E4A200..0x82E4A208)
	// 82E4A200: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A204: 4BFFFF94  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A208 size=8
    let mut pc: u32 = 0x82E4A208;
    'dispatch: loop {
        match pc {
            0x82E4A208 => {
    //   block [0x82E4A208..0x82E4A210)
	// 82E4A208: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A20C: 4BFFFF8C  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A210 size=8
    let mut pc: u32 = 0x82E4A210;
    'dispatch: loop {
        match pc {
            0x82E4A210 => {
    //   block [0x82E4A210..0x82E4A218)
	// 82E4A210: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4A214: 4BFFFF84  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A218 size=8
    let mut pc: u32 = 0x82E4A218;
    'dispatch: loop {
        match pc {
            0x82E4A218 => {
    //   block [0x82E4A218..0x82E4A220)
	// 82E4A218: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82E4A21C: 4BFFFF7C  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A220 size=8
    let mut pc: u32 = 0x82E4A220;
    'dispatch: loop {
        match pc {
            0x82E4A220 => {
    //   block [0x82E4A220..0x82E4A228)
	// 82E4A220: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4A224: 4BFFFF74  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A228 size=4
    let mut pc: u32 = 0x82E4A228;
    'dispatch: loop {
        match pc {
            0x82E4A228 => {
    //   block [0x82E4A228..0x82E4A22C)
	// 82E4A228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A230 size=4
    let mut pc: u32 = 0x82E4A230;
    'dispatch: loop {
        match pc {
            0x82E4A230 => {
    //   block [0x82E4A230..0x82E4A234)
	// 82E4A230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A238 size=20
    let mut pc: u32 = 0x82E4A238;
    'dispatch: loop {
        match pc {
            0x82E4A238 => {
    //   block [0x82E4A238..0x82E4A24C)
	// 82E4A238: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A23C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A240: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A248: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A250 size=32
    let mut pc: u32 = 0x82E4A250;
    'dispatch: loop {
        match pc {
            0x82E4A250 => {
    //   block [0x82E4A250..0x82E4A270)
	// 82E4A250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A254: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4A258: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A260: 396B6004  addi r11, r11, 0x6004
	ctx.r[11].s64 = ctx.r[11].s64 + 24580;
	// 82E4A264: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4A268: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A270 size=12
    let mut pc: u32 = 0x82E4A270;
    'dispatch: loop {
        match pc {
            0x82E4A270 => {
    //   block [0x82E4A270..0x82E4A27C)
	// 82E4A270: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A274: 386B6004  addi r3, r11, 0x6004
	ctx.r[3].s64 = ctx.r[11].s64 + 24580;
	// 82E4A278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A280 size=100
    let mut pc: u32 = 0x82E4A280;
    'dispatch: loop {
        match pc {
            0x82E4A280 => {
    //   block [0x82E4A280..0x82E4A2C8)
	// 82E4A280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A29C: 48009E25  bl 0x82e540c0
	ctx.lr = 0x82E4A2A0;
	sub_82E540C0(ctx, base);
	// 82E4A2A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A2A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A2A8: 419A0020  beq cr6, 0x82e4a2c8
	if ctx.cr[6].eq {
	pc = 0x82E4A2C8; continue 'dispatch;
	}
	// 82E4A2AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A2B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A2B4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82E4A2B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A2BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A2C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A2C4: 4BF0B005  bl 0x82d552c8
	ctx.lr = 0x82E4A2C8;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A2C8; continue 'dispatch;
            }
            0x82E4A2C8 => {
    //   block [0x82E4A2C8..0x82E4A2E4)
	// 82E4A2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A2CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A2D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A2DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A2E8 size=20
    let mut pc: u32 = 0x82E4A2E8;
    'dispatch: loop {
        match pc {
            0x82E4A2E8 => {
    //   block [0x82E4A2E8..0x82E4A2FC)
	// 82E4A2E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A2EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A2F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A2F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A300 size=136
    let mut pc: u32 = 0x82E4A300;
    'dispatch: loop {
        match pc {
            0x82E4A300 => {
    //   block [0x82E4A300..0x82E4A388)
	// 82E4A300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A304: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4A308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A30C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E4A310: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4A314: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A318: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4A31C: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4A320: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82E4A324: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4A328: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4A32C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4A330: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E4A334: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E4A338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A33C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A340: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82E4A344: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4A348: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4A34C: 38E760F8  addi r7, r7, 0x60f8
	ctx.r[7].s64 = ctx.r[7].s64 + 24824;
	// 82E4A350: 38C660EC  addi r6, r6, 0x60ec
	ctx.r[6].s64 = ctx.r[6].s64 + 24812;
	// 82E4A354: 38A560D8  addi r5, r5, 0x60d8
	ctx.r[5].s64 = ctx.r[5].s64 + 24792;
	// 82E4A358: 396B60CC  addi r11, r11, 0x60cc
	ctx.r[11].s64 = ctx.r[11].s64 + 24780;
	// 82E4A35C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A360: 388460C0  addi r4, r4, 0x60c0
	ctx.r[4].s64 = ctx.r[4].s64 + 24768;
	// 82E4A364: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4A368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4A36C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E4A370: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E4A374: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82E4A378: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E4A37C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E4A380: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E4A384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A388 size=12
    let mut pc: u32 = 0x82E4A388;
    'dispatch: loop {
        match pc {
            0x82E4A388 => {
    //   block [0x82E4A388..0x82E4A394)
	// 82E4A388: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A38C: 386B60F8  addi r3, r11, 0x60f8
	ctx.r[3].s64 = ctx.r[11].s64 + 24824;
	// 82E4A390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A398 size=8
    let mut pc: u32 = 0x82E4A398;
    'dispatch: loop {
        match pc {
            0x82E4A398 => {
    //   block [0x82E4A398..0x82E4A3A0)
	// 82E4A398: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4A39C: 4800001C  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A3A0 size=8
    let mut pc: u32 = 0x82E4A3A0;
    'dispatch: loop {
        match pc {
            0x82E4A3A0 => {
    //   block [0x82E4A3A0..0x82E4A3A8)
	// 82E4A3A0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A3A4: 48000014  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A3A8 size=8
    let mut pc: u32 = 0x82E4A3A8;
    'dispatch: loop {
        match pc {
            0x82E4A3A8 => {
    //   block [0x82E4A3A8..0x82E4A3B0)
	// 82E4A3A8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A3AC: 4800000C  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A3B0 size=8
    let mut pc: u32 = 0x82E4A3B0;
    'dispatch: loop {
        match pc {
            0x82E4A3B0 => {
    //   block [0x82E4A3B0..0x82E4A3B8)
	// 82E4A3B0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4A3B4: 48000004  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A3B8 size=100
    let mut pc: u32 = 0x82E4A3B8;
    'dispatch: loop {
        match pc {
            0x82E4A3B8 => {
    //   block [0x82E4A3B8..0x82E4A400)
	// 82E4A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A3C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A3C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A3C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A3CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A3D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A3D4: 4B815105  bl 0x8265f4d8
	ctx.lr = 0x82E4A3D8;
	sub_8265F4D8(ctx, base);
	// 82E4A3D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A3DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A3E0: 419A0020  beq cr6, 0x82e4a400
	if ctx.cr[6].eq {
	pc = 0x82E4A400; continue 'dispatch;
	}
	// 82E4A3E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A3E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A3EC: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4A3F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A3F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A3F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A3FC: 4BF0AECD  bl 0x82d552c8
	ctx.lr = 0x82E4A400;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A400; continue 'dispatch;
            }
            0x82E4A400 => {
    //   block [0x82E4A400..0x82E4A41C)
	// 82E4A400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A410: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A420 size=32
    let mut pc: u32 = 0x82E4A420;
    'dispatch: loop {
        match pc {
            0x82E4A420 => {
    //   block [0x82E4A420..0x82E4A440)
	// 82E4A420: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A424: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4A428: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A42C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A430: 396B6194  addi r11, r11, 0x6194
	ctx.r[11].s64 = ctx.r[11].s64 + 24980;
	// 82E4A434: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4A438: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A440 size=20
    let mut pc: u32 = 0x82E4A440;
    'dispatch: loop {
        match pc {
            0x82E4A440 => {
    //   block [0x82E4A440..0x82E4A454)
	// 82E4A440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A448: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A44C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A450: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A458 size=12
    let mut pc: u32 = 0x82E4A458;
    'dispatch: loop {
        match pc {
            0x82E4A458 => {
    //   block [0x82E4A458..0x82E4A464)
	// 82E4A458: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A45C: 386B6194  addi r3, r11, 0x6194
	ctx.r[3].s64 = ctx.r[11].s64 + 24980;
	// 82E4A460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A468 size=4
    let mut pc: u32 = 0x82E4A468;
    'dispatch: loop {
        match pc {
            0x82E4A468 => {
    //   block [0x82E4A468..0x82E4A46C)
	// 82E4A468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A470 size=4
    let mut pc: u32 = 0x82E4A470;
    'dispatch: loop {
        match pc {
            0x82E4A470 => {
    //   block [0x82E4A470..0x82E4A474)
	// 82E4A470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A478 size=4
    let mut pc: u32 = 0x82E4A478;
    'dispatch: loop {
        match pc {
            0x82E4A478 => {
    //   block [0x82E4A478..0x82E4A47C)
	// 82E4A478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A480 size=8
    let mut pc: u32 = 0x82E4A480;
    'dispatch: loop {
        match pc {
            0x82E4A480 => {
    //   block [0x82E4A480..0x82E4A488)
	// 82E4A480: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A484: 4800008C  b 0x82e4a510
	sub_82E4A510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A488 size=4
    let mut pc: u32 = 0x82E4A488;
    'dispatch: loop {
        match pc {
            0x82E4A488 => {
    //   block [0x82E4A488..0x82E4A48C)
	// 82E4A488: 48000008  b 0x82e4a490
	sub_82E4A490(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A490 size=128
    let mut pc: u32 = 0x82E4A490;
    'dispatch: loop {
        match pc {
            0x82E4A490 => {
    //   block [0x82E4A490..0x82E4A4D0)
	// 82E4A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A49C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A4A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A4A4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4A4A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A4AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A4B0: 409A0020  bne cr6, 0x82e4a4d0
	if !ctx.cr[6].eq {
	pc = 0x82E4A4D0; continue 'dispatch;
	}
	// 82E4A4B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A4B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A4BC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A4C0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4A4C4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A4C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A4CC: 4BF0ADFD  bl 0x82d552c8
	ctx.lr = 0x82E4A4D0;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A4D0; continue 'dispatch;
            }
            0x82E4A4D0 => {
    //   block [0x82E4A4D0..0x82E4A4FC)
	// 82E4A4D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4A4D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A4D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A4DC: 409A0020  bne cr6, 0x82e4a4fc
	if !ctx.cr[6].eq {
	pc = 0x82E4A4FC; continue 'dispatch;
	}
	// 82E4A4E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A4E4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A4E8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A4EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A4F0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A4F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A4F8: 4BF0ADD1  bl 0x82d552c8
	ctx.lr = 0x82E4A4FC;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A4FC; continue 'dispatch;
            }
            0x82E4A4FC => {
    //   block [0x82E4A4FC..0x82E4A510)
	// 82E4A4FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A510 size=184
    let mut pc: u32 = 0x82E4A510;
    'dispatch: loop {
        match pc {
            0x82E4A510 => {
    //   block [0x82E4A510..0x82E4A558)
	// 82E4A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A51C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A52C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4A530: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A534: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A538: 409A0020  bne cr6, 0x82e4a558
	if !ctx.cr[6].eq {
	pc = 0x82E4A558; continue 'dispatch;
	}
	// 82E4A53C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A540: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A544: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A548: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A54C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A550: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A554: 4BF0AD75  bl 0x82d552c8
	ctx.lr = 0x82E4A558;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A558; continue 'dispatch;
            }
            0x82E4A558 => {
    //   block [0x82E4A558..0x82E4A594)
	// 82E4A558: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A560: 419A004C  beq cr6, 0x82e4a5ac
	if ctx.cr[6].eq {
	pc = 0x82E4A5AC; continue 'dispatch;
	}
	// 82E4A564: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A568: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A56C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A570: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E4A574: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E4A578: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4A57C: 41980018  blt cr6, 0x82e4a594
	if ctx.cr[6].lt {
	pc = 0x82E4A594; continue 'dispatch;
	}
	// 82E4A580: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E4A584: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E4A588: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E4A58C: 4BF0AB9D  bl 0x82d55128
	ctx.lr = 0x82E4A590;
	sub_82D55128(ctx, base);
	// 82E4A590: 4800001C  b 0x82e4a5ac
	pc = 0x82E4A5AC; continue 'dispatch;
            }
            0x82E4A594 => {
    //   block [0x82E4A594..0x82E4A5AC)
	// 82E4A594: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E4A598: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E4A59C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4A5A0: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82E4A5A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4A5A8: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x82E4A5AC; continue 'dispatch;
            }
            0x82E4A5AC => {
    //   block [0x82E4A5AC..0x82E4A5C8)
	// 82E4A5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A5BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A5C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A5E0 size=92
    let mut pc: u32 = 0x82E4A5E0;
    'dispatch: loop {
        match pc {
            0x82E4A5E0 => {
    //   block [0x82E4A5E0..0x82E4A628)
	// 82E4A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A5F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A5F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4A5F8: 419A0030  beq cr6, 0x82e4a628
	if ctx.cr[6].eq {
	pc = 0x82E4A628; continue 'dispatch;
	}
	// 82E4A5FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4A600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A604: 4BF40785  bl 0x82d8ad88
	ctx.lr = 0x82E4A608;
	sub_82D8AD88(ctx, base);
	// 82E4A608: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4A610: 396B646C  addi r11, r11, 0x646c
	ctx.r[11].s64 = ctx.r[11].s64 + 25708;
	// 82E4A614: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E4A618: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A61C: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E4A620: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82E4A624: 913F005C  stw r9, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	pc = 0x82E4A628; continue 'dispatch;
            }
            0x82E4A628 => {
    //   block [0x82E4A628..0x82E4A63C)
	// 82E4A628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4A62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A640 size=52
    let mut pc: u32 = 0x82E4A640;
    'dispatch: loop {
        match pc {
            0x82E4A640 => {
    //   block [0x82E4A640..0x82E4A674)
	// 82E4A640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A648: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4A650: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A654: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4A658: 4BF40731  bl 0x82d8ad88
	ctx.lr = 0x82E4A65C;
	sub_82D8AD88(ctx, base);
	// 82E4A65C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A660: 386B646C  addi r3, r11, 0x646c
	ctx.r[3].s64 = ctx.r[11].s64 + 25708;
	// 82E4A664: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E4A668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A678 size=148
    let mut pc: u32 = 0x82E4A678;
    'dispatch: loop {
        match pc {
            0x82E4A678 => {
    //   block [0x82E4A678..0x82E4A6C0)
	// 82E4A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A68C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A690: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A694: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4A698: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A69C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A6A0: 409A0020  bne cr6, 0x82e4a6c0
	if !ctx.cr[6].eq {
	pc = 0x82E4A6C0; continue 'dispatch;
	}
	// 82E4A6A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A6AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A6B0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4A6B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A6B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A6BC: 4BF0AC0D  bl 0x82d552c8
	ctx.lr = 0x82E4A6C0;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A6C0; continue 'dispatch;
            }
            0x82E4A6C0 => {
    //   block [0x82E4A6C0..0x82E4A6F0)
	// 82E4A6C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A6C4: 4BF4072D  bl 0x82d8adf0
	ctx.lr = 0x82E4A6C8;
	sub_82D8ADF0(ctx, base);
	// 82E4A6C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A6CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A6D0: 419A0020  beq cr6, 0x82e4a6f0
	if ctx.cr[6].eq {
	pc = 0x82E4A6F0; continue 'dispatch;
	}
	// 82E4A6D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A6D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A6DC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82E4A6E0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A6E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A6E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A6EC: 4BF0ABDD  bl 0x82d552c8
	ctx.lr = 0x82E4A6F0;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A6F0; continue 'dispatch;
            }
            0x82E4A6F0 => {
    //   block [0x82E4A6F0..0x82E4A70C)
	// 82E4A6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A6F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A710 size=4
    let mut pc: u32 = 0x82E4A710;
    'dispatch: loop {
        match pc {
            0x82E4A710 => {
    //   block [0x82E4A710..0x82E4A714)
	// 82E4A710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A718 size=20
    let mut pc: u32 = 0x82E4A718;
    'dispatch: loop {
        match pc {
            0x82E4A718 => {
    //   block [0x82E4A718..0x82E4A72C)
	// 82E4A718: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A71C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A720: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A728: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A730 size=32
    let mut pc: u32 = 0x82E4A730;
    'dispatch: loop {
        match pc {
            0x82E4A730 => {
    //   block [0x82E4A730..0x82E4A750)
	// 82E4A730: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A734: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4A738: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A73C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A740: 396B6500  addi r11, r11, 0x6500
	ctx.r[11].s64 = ctx.r[11].s64 + 25856;
	// 82E4A744: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4A748: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A750 size=12
    let mut pc: u32 = 0x82E4A750;
    'dispatch: loop {
        match pc {
            0x82E4A750 => {
    //   block [0x82E4A750..0x82E4A75C)
	// 82E4A750: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A754: 386B6500  addi r3, r11, 0x6500
	ctx.r[3].s64 = ctx.r[11].s64 + 25856;
	// 82E4A758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A760 size=100
    let mut pc: u32 = 0x82E4A760;
    'dispatch: loop {
        match pc {
            0x82E4A760 => {
    //   block [0x82E4A760..0x82E4A7A8)
	// 82E4A760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A76C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A77C: 4800ADC5  bl 0x82e55540
	ctx.lr = 0x82E4A780;
	sub_82E55540(ctx, base);
	// 82E4A780: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A788: 419A0020  beq cr6, 0x82e4a7a8
	if ctx.cr[6].eq {
	pc = 0x82E4A7A8; continue 'dispatch;
	}
	// 82E4A78C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A790: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A794: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E4A798: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A79C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A7A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A7A4: 4BF0AB25  bl 0x82d552c8
	ctx.lr = 0x82E4A7A8;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A7A8; continue 'dispatch;
            }
            0x82E4A7A8 => {
    //   block [0x82E4A7A8..0x82E4A7C4)
	// 82E4A7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A7AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A7B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A7BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A7C8 size=4
    let mut pc: u32 = 0x82E4A7C8;
    'dispatch: loop {
        match pc {
            0x82E4A7C8 => {
    //   block [0x82E4A7C8..0x82E4A7CC)
	// 82E4A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A7D0 size=20
    let mut pc: u32 = 0x82E4A7D0;
    'dispatch: loop {
        match pc {
            0x82E4A7D0 => {
    //   block [0x82E4A7D0..0x82E4A7E4)
	// 82E4A7D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A7D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A7D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A7DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A7E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A800 size=44
    let mut pc: u32 = 0x82E4A800;
    'dispatch: loop {
        match pc {
            0x82E4A800 => {
    //   block [0x82E4A800..0x82E4A82C)
	// 82E4A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A808: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A80C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4A814: 4800001D  bl 0x82e4a830
	ctx.lr = 0x82E4A818;
	sub_82E4A830(ctx, base);
	// 82E4A818: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4A81C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4A820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A830 size=168
    let mut pc: u32 = 0x82E4A830;
    'dispatch: loop {
        match pc {
            0x82E4A830 => {
    //   block [0x82E4A830..0x82E4A8D8)
	// 82E4A830: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A834: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E4A838: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4A83C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A840: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4A844: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4A848: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E4A84C: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4A850: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4A854: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A858: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4A85C: 38CB6614  addi r6, r11, 0x6614
	ctx.r[6].s64 = ctx.r[11].s64 + 26132;
	// 82E4A860: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A864: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A868: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A86C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4A870: 38AB6608  addi r5, r11, 0x6608
	ctx.r[5].s64 = ctx.r[11].s64 + 26120;
	// 82E4A874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A878: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4A87C: 394B65F4  addi r10, r11, 0x65f4
	ctx.r[10].s64 = ctx.r[11].s64 + 26100;
	// 82E4A880: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A884: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4A888: 38E75D40  addi r7, r7, 0x5d40
	ctx.r[7].s64 = ctx.r[7].s64 + 23872;
	// 82E4A88C: 392B65E8  addi r9, r11, 0x65e8
	ctx.r[9].s64 = ctx.r[11].s64 + 26088;
	// 82E4A890: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A894: 388B65DC  addi r4, r11, 0x65dc
	ctx.r[4].s64 = ctx.r[11].s64 + 26076;
	// 82E4A898: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A89C: 390B65C4  addi r8, r11, 0x65c4
	ctx.r[8].s64 = ctx.r[11].s64 + 26052;
	// 82E4A8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4A8A4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E4A8A8: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E4A8AC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A8B0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E4A8B4: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E4A8B8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E4A8BC: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A8C0: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E4A8C4: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E4A8C8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E4A8CC: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4A8D0: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E4A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A8D8 size=100
    let mut pc: u32 = 0x82E4A8D8;
    'dispatch: loop {
        match pc {
            0x82E4A8D8 => {
    //   block [0x82E4A8D8..0x82E4A920)
	// 82E4A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A8EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A8F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A8F4: 4800AF0D  bl 0x82e55800
	ctx.lr = 0x82E4A8F8;
	sub_82E55800(ctx, base);
	// 82E4A8F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A8FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A900: 419A0020  beq cr6, 0x82e4a920
	if ctx.cr[6].eq {
	pc = 0x82E4A920; continue 'dispatch;
	}
	// 82E4A904: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A908: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A90C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4A910: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A914: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A918: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A91C: 4BF0A9AD  bl 0x82d552c8
	ctx.lr = 0x82E4A920;
	sub_82D552C8(ctx, base);
	pc = 0x82E4A920; continue 'dispatch;
            }
            0x82E4A920 => {
    //   block [0x82E4A920..0x82E4A93C)
	// 82E4A920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A930: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A940 size=8
    let mut pc: u32 = 0x82E4A940;
    'dispatch: loop {
        match pc {
            0x82E4A940 => {
    //   block [0x82E4A940..0x82E4A948)
	// 82E4A940: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A944: 4BFFFF94  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A948 size=8
    let mut pc: u32 = 0x82E4A948;
    'dispatch: loop {
        match pc {
            0x82E4A948 => {
    //   block [0x82E4A948..0x82E4A950)
	// 82E4A948: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A94C: 4BFFFF8C  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A950 size=8
    let mut pc: u32 = 0x82E4A950;
    'dispatch: loop {
        match pc {
            0x82E4A950 => {
    //   block [0x82E4A950..0x82E4A958)
	// 82E4A950: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4A954: 4BFFFF84  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A958 size=8
    let mut pc: u32 = 0x82E4A958;
    'dispatch: loop {
        match pc {
            0x82E4A958 => {
    //   block [0x82E4A958..0x82E4A960)
	// 82E4A958: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82E4A95C: 4BFFFF7C  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A960 size=8
    let mut pc: u32 = 0x82E4A960;
    'dispatch: loop {
        match pc {
            0x82E4A960 => {
    //   block [0x82E4A960..0x82E4A968)
	// 82E4A960: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4A964: 4BFFFF74  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A968 size=20
    let mut pc: u32 = 0x82E4A968;
    'dispatch: loop {
        match pc {
            0x82E4A968 => {
    //   block [0x82E4A968..0x82E4A97C)
	// 82E4A968: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A96C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A970: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A978: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A980 size=4
    let mut pc: u32 = 0x82E4A980;
    'dispatch: loop {
        match pc {
            0x82E4A980 => {
    //   block [0x82E4A980..0x82E4A984)
	// 82E4A980: 4800B3A0  b 0x82e55d20
	sub_82E55D20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A988 size=4
    let mut pc: u32 = 0x82E4A988;
    'dispatch: loop {
        match pc {
            0x82E4A988 => {
    //   block [0x82E4A988..0x82E4A98C)
	// 82E4A988: 4800B398  b 0x82e55d20
	sub_82E55D20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A990 size=4
    let mut pc: u32 = 0x82E4A990;
    'dispatch: loop {
        match pc {
            0x82E4A990 => {
    //   block [0x82E4A990..0x82E4A994)
	// 82E4A990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A998 size=4
    let mut pc: u32 = 0x82E4A998;
    'dispatch: loop {
        match pc {
            0x82E4A998 => {
    //   block [0x82E4A998..0x82E4A99C)
	// 82E4A998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A9A0 size=44
    let mut pc: u32 = 0x82E4A9A0;
    'dispatch: loop {
        match pc {
            0x82E4A9A0 => {
    //   block [0x82E4A9A0..0x82E4A9CC)
	// 82E4A9A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4A9A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A9A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4A9AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A9B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E4A9B4: 394A6860  addi r10, r10, 0x6860
	ctx.r[10].s64 = ctx.r[10].s64 + 26720;
	// 82E4A9B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4A9BC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E4A9C0: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E4A9C4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4A9C8: 4BF53D78  b 0x82d9e740
	sub_82D9E740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A9CC size=4
    let mut pc: u32 = 0x82E4A9CC;
    'dispatch: loop {
        match pc {
            0x82E4A9CC => {
    //   block [0x82E4A9CC..0x82E4A9D0)
	// 82E4A9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A9D0 size=64
    let mut pc: u32 = 0x82E4A9D0;
    'dispatch: loop {
        match pc {
            0x82E4A9D0 => {
    //   block [0x82E4A9D0..0x82E4AA10)
	// 82E4A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A9D8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A9DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A9E4: 396B6860  addi r11, r11, 0x6860
	ctx.r[11].s64 = ctx.r[11].s64 + 26720;
	// 82E4A9E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A9EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4A9F0: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 82E4A9F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4A9F8: 4BF53D49  bl 0x82d9e740
	ctx.lr = 0x82E4A9FC;
	sub_82D9E740(ctx, base);
	// 82E4A9FC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4AA00: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82E4AA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AA10 size=100
    let mut pc: u32 = 0x82E4AA10;
    'dispatch: loop {
        match pc {
            0x82E4AA10 => {
    //   block [0x82E4AA10..0x82E4AA58)
	// 82E4AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AA18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AA1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AA20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AA24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AA28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AA2C: 4800B475  bl 0x82e55ea0
	ctx.lr = 0x82E4AA30;
	sub_82E55EA0(ctx, base);
	// 82E4AA30: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AA34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AA38: 419A0020  beq cr6, 0x82e4aa58
	if ctx.cr[6].eq {
	pc = 0x82E4AA58; continue 'dispatch;
	}
	// 82E4AA3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AA40: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AA44: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E4AA48: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AA4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AA50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AA54: 4BF0A875  bl 0x82d552c8
	ctx.lr = 0x82E4AA58;
	sub_82D552C8(ctx, base);
	pc = 0x82E4AA58; continue 'dispatch;
            }
            0x82E4AA58 => {
    //   block [0x82E4AA58..0x82E4AA74)
	// 82E4AA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AA5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AA68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AA78 size=20
    let mut pc: u32 = 0x82E4AA78;
    'dispatch: loop {
        match pc {
            0x82E4AA78 => {
    //   block [0x82E4AA78..0x82E4AA8C)
	// 82E4AA78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AA7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AA80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AA84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AA88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AA90 size=32
    let mut pc: u32 = 0x82E4AA90;
    'dispatch: loop {
        match pc {
            0x82E4AA90 => {
    //   block [0x82E4AA90..0x82E4AAB0)
	// 82E4AA90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AA94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4AA98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AA9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AAA0: 396B6950  addi r11, r11, 0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + 26960;
	// 82E4AAA4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AAA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AAB0 size=12
    let mut pc: u32 = 0x82E4AAB0;
    'dispatch: loop {
        match pc {
            0x82E4AAB0 => {
    //   block [0x82E4AAB0..0x82E4AABC)
	// 82E4AAB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AAB4: 386B6950  addi r3, r11, 0x6950
	ctx.r[3].s64 = ctx.r[11].s64 + 26960;
	// 82E4AAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AAC0 size=100
    let mut pc: u32 = 0x82E4AAC0;
    'dispatch: loop {
        match pc {
            0x82E4AAC0 => {
    //   block [0x82E4AAC0..0x82E4AB08)
	// 82E4AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AAC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AAD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AAD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AAD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AADC: 4800B54D  bl 0x82e56028
	ctx.lr = 0x82E4AAE0;
	sub_82E56028(ctx, base);
	// 82E4AAE0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AAE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AAE8: 419A0020  beq cr6, 0x82e4ab08
	if ctx.cr[6].eq {
	pc = 0x82E4AB08; continue 'dispatch;
	}
	// 82E4AAEC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AAF0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AAF4: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 82E4AAF8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AAFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AB00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AB04: 4BF0A7C5  bl 0x82d552c8
	ctx.lr = 0x82E4AB08;
	sub_82D552C8(ctx, base);
	pc = 0x82E4AB08; continue 'dispatch;
            }
            0x82E4AB08 => {
    //   block [0x82E4AB08..0x82E4AB24)
	// 82E4AB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AB0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AB18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AB1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB28 size=32
    let mut pc: u32 = 0x82E4AB28;
    'dispatch: loop {
        match pc {
            0x82E4AB28 => {
    //   block [0x82E4AB28..0x82E4AB48)
	// 82E4AB28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AB2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4AB30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AB34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AB38: 396B6A04  addi r11, r11, 0x6a04
	ctx.r[11].s64 = ctx.r[11].s64 + 27140;
	// 82E4AB3C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AB40: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB48 size=20
    let mut pc: u32 = 0x82E4AB48;
    'dispatch: loop {
        match pc {
            0x82E4AB48 => {
    //   block [0x82E4AB48..0x82E4AB5C)
	// 82E4AB48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AB50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AB58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB60 size=12
    let mut pc: u32 = 0x82E4AB60;
    'dispatch: loop {
        match pc {
            0x82E4AB60 => {
    //   block [0x82E4AB60..0x82E4AB6C)
	// 82E4AB60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AB64: 386B6A04  addi r3, r11, 0x6a04
	ctx.r[3].s64 = ctx.r[11].s64 + 27140;
	// 82E4AB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB70 size=20
    let mut pc: u32 = 0x82E4AB70;
    'dispatch: loop {
        match pc {
            0x82E4AB70 => {
    //   block [0x82E4AB70..0x82E4AB84)
	// 82E4AB70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AB78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AB80: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB88 size=32
    let mut pc: u32 = 0x82E4AB88;
    'dispatch: loop {
        match pc {
            0x82E4AB88 => {
    //   block [0x82E4AB88..0x82E4ABA8)
	// 82E4AB88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AB8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4AB90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AB94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AB98: 396B6A54  addi r11, r11, 0x6a54
	ctx.r[11].s64 = ctx.r[11].s64 + 27220;
	// 82E4AB9C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4ABA0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4ABA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ABA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4ABA8 size=12
    let mut pc: u32 = 0x82E4ABA8;
    'dispatch: loop {
        match pc {
            0x82E4ABA8 => {
    //   block [0x82E4ABA8..0x82E4ABB4)
	// 82E4ABA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4ABAC: 386B6A54  addi r3, r11, 0x6a54
	ctx.r[3].s64 = ctx.r[11].s64 + 27220;
	// 82E4ABB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4ABB8 size=96
    let mut pc: u32 = 0x82E4ABB8;
    'dispatch: loop {
        match pc {
            0x82E4ABB8 => {
    //   block [0x82E4ABB8..0x82E4AC00)
	// 82E4ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4ABC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4ABC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4ABC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4ABCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4ABD0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4ABD4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E4ABD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4ABDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4ABE0: 419A0020  beq cr6, 0x82e4ac00
	if ctx.cr[6].eq {
	pc = 0x82E4AC00; continue 'dispatch;
	}
	// 82E4ABE4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ABE8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4ABEC: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82E4ABF0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4ABF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4ABF8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4ABFC: 4BF0A6CD  bl 0x82d552c8
	ctx.lr = 0x82E4AC00;
	sub_82D552C8(ctx, base);
	pc = 0x82E4AC00; continue 'dispatch;
            }
            0x82E4AC00 => {
    //   block [0x82E4AC00..0x82E4AC18)
	// 82E4AC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AC04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4AC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AC10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC18 size=20
    let mut pc: u32 = 0x82E4AC18;
    'dispatch: loop {
        match pc {
            0x82E4AC18 => {
    //   block [0x82E4AC18..0x82E4AC2C)
	// 82E4AC18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AC1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AC20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AC24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AC28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC30 size=32
    let mut pc: u32 = 0x82E4AC30;
    'dispatch: loop {
        match pc {
            0x82E4AC30 => {
    //   block [0x82E4AC30..0x82E4AC50)
	// 82E4AC30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AC34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4AC38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AC40: 396B6AA0  addi r11, r11, 0x6aa0
	ctx.r[11].s64 = ctx.r[11].s64 + 27296;
	// 82E4AC44: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AC48: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC50 size=12
    let mut pc: u32 = 0x82E4AC50;
    'dispatch: loop {
        match pc {
            0x82E4AC50 => {
    //   block [0x82E4AC50..0x82E4AC5C)
	// 82E4AC50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AC54: 386B6AA0  addi r3, r11, 0x6aa0
	ctx.r[3].s64 = ctx.r[11].s64 + 27296;
	// 82E4AC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AC60 size=196
    let mut pc: u32 = 0x82E4AC60;
    'dispatch: loop {
        match pc {
            0x82E4AC60 => {
    //   block [0x82E4AC60..0x82E4AC90)
	// 82E4AC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AC64: 4BE5E7A9  bl 0x82ca940c
	ctx.lr = 0x82E4AC68;
	sub_82CA93D0(ctx, base);
	// 82E4AC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AC6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AC70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AC74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4AC78: 396B6AA0  addi r11, r11, 0x6aa0
	ctx.r[11].s64 = ctx.r[11].s64 + 27296;
	// 82E4AC7C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4AC80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4AC84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AC88: 4099005C  ble cr6, 0x82e4ace4
	if !ctx.cr[6].gt {
	pc = 0x82E4ACE4; continue 'dispatch;
	}
	// 82E4AC8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82E4AC90; continue 'dispatch;
            }
            0x82E4AC90 => {
    //   block [0x82E4AC90..0x82E4ACD0)
	// 82E4AC90: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4AC94: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4AC98: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AC9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4ACA0: 419A0030  beq cr6, 0x82e4acd0
	if ctx.cr[6].eq {
	pc = 0x82E4ACD0; continue 'dispatch;
	}
	// 82E4ACA4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4ACA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4ACAC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4ACB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4ACB4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4ACB8: 409A0018  bne cr6, 0x82e4acd0
	if !ctx.cr[6].eq {
	pc = 0x82E4ACD0; continue 'dispatch;
	}
	// 82E4ACBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ACC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4ACC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ACC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4ACCC: 4E800421  bctrl
	ctx.lr = 0x82E4ACD0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4ACD0 => {
    //   block [0x82E4ACD0..0x82E4ACE4)
	// 82E4ACD0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4ACD4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4ACD8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4ACDC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4ACE0: 4198FFB0  blt cr6, 0x82e4ac90
	if ctx.cr[6].lt {
	pc = 0x82E4AC90; continue 'dispatch;
	}
	pc = 0x82E4ACE4; continue 'dispatch;
            }
            0x82E4ACE4 => {
    //   block [0x82E4ACE4..0x82E4AD10)
	// 82E4ACE4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4ACE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4ACEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4ACF0: 409A0020  bne cr6, 0x82e4ad10
	if !ctx.cr[6].eq {
	pc = 0x82E4AD10; continue 'dispatch;
	}
	// 82E4ACF4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ACF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4ACFC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4AD00: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4AD04: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4AD08: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4AD0C: 4BF0A5BD  bl 0x82d552c8
	ctx.lr = 0x82E4AD10;
	sub_82D552C8(ctx, base);
	pc = 0x82E4AD10; continue 'dispatch;
            }
            0x82E4AD10 => {
    //   block [0x82E4AD10..0x82E4AD24)
	// 82E4AD10: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4AD14: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4AD18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AD20: 4BE5E73C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AD28 size=100
    let mut pc: u32 = 0x82E4AD28;
    'dispatch: loop {
        match pc {
            0x82E4AD28 => {
    //   block [0x82E4AD28..0x82E4AD70)
	// 82E4AD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AD30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AD34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AD38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AD3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AD40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AD44: 4BFFFF1D  bl 0x82e4ac60
	ctx.lr = 0x82E4AD48;
	sub_82E4AC60(ctx, base);
	// 82E4AD48: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AD4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AD50: 419A0020  beq cr6, 0x82e4ad70
	if ctx.cr[6].eq {
	pc = 0x82E4AD70; continue 'dispatch;
	}
	// 82E4AD54: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AD58: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AD5C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E4AD60: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AD64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AD68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AD6C: 4BF0A55D  bl 0x82d552c8
	ctx.lr = 0x82E4AD70;
	sub_82D552C8(ctx, base);
	pc = 0x82E4AD70; continue 'dispatch;
            }
            0x82E4AD70 => {
    //   block [0x82E4AD70..0x82E4AD8C)
	// 82E4AD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AD74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AD80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AD84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AD90 size=32
    let mut pc: u32 = 0x82E4AD90;
    'dispatch: loop {
        match pc {
            0x82E4AD90 => {
    //   block [0x82E4AD90..0x82E4ADB0)
	// 82E4AD90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AD94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4AD98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AD9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4ADA0: 396B6BE4  addi r11, r11, 0x6be4
	ctx.r[11].s64 = ctx.r[11].s64 + 27620;
	// 82E4ADA4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4ADA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4ADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4ADB0 size=20
    let mut pc: u32 = 0x82E4ADB0;
    'dispatch: loop {
        match pc {
            0x82E4ADB0 => {
    //   block [0x82E4ADB0..0x82E4ADC4)
	// 82E4ADB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ADB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4ADB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ADBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4ADC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4ADC8 size=12
    let mut pc: u32 = 0x82E4ADC8;
    'dispatch: loop {
        match pc {
            0x82E4ADC8 => {
    //   block [0x82E4ADC8..0x82E4ADD4)
	// 82E4ADC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4ADCC: 386B6BE4  addi r3, r11, 0x6be4
	ctx.r[3].s64 = ctx.r[11].s64 + 27620;
	// 82E4ADD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4ADD8 size=100
    let mut pc: u32 = 0x82E4ADD8;
    'dispatch: loop {
        match pc {
            0x82E4ADD8 => {
    //   block [0x82E4ADD8..0x82E4AE20)
	// 82E4ADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4ADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4ADE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4ADE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4ADE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4ADEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4ADF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4ADF4: 4BF3C7FD  bl 0x82d875f0
	ctx.lr = 0x82E4ADF8;
	sub_82D875F0(ctx, base);
	// 82E4ADF8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4ADFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AE00: 419A0020  beq cr6, 0x82e4ae20
	if ctx.cr[6].eq {
	pc = 0x82E4AE20; continue 'dispatch;
	}
	// 82E4AE04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AE08: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AE0C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82E4AE10: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AE14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AE18: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AE1C: 4BF0A4AD  bl 0x82d552c8
	ctx.lr = 0x82E4AE20;
	sub_82D552C8(ctx, base);
	pc = 0x82E4AE20; continue 'dispatch;
            }
            0x82E4AE20 => {
    //   block [0x82E4AE20..0x82E4AE3C)
	// 82E4AE20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AE24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AE30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AE34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE40 size=4
    let mut pc: u32 = 0x82E4AE40;
    'dispatch: loop {
        match pc {
            0x82E4AE40 => {
    //   block [0x82E4AE40..0x82E4AE44)
	// 82E4AE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE48 size=32
    let mut pc: u32 = 0x82E4AE48;
    'dispatch: loop {
        match pc {
            0x82E4AE48 => {
    //   block [0x82E4AE48..0x82E4AE68)
	// 82E4AE48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AE4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4AE50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AE54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AE58: 396B6FE4  addi r11, r11, 0x6fe4
	ctx.r[11].s64 = ctx.r[11].s64 + 28644;
	// 82E4AE5C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AE60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE68 size=20
    let mut pc: u32 = 0x82E4AE68;
    'dispatch: loop {
        match pc {
            0x82E4AE68 => {
    //   block [0x82E4AE68..0x82E4AE7C)
	// 82E4AE68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AE6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AE70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AE74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AE78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE80 size=12
    let mut pc: u32 = 0x82E4AE80;
    'dispatch: loop {
        match pc {
            0x82E4AE80 => {
    //   block [0x82E4AE80..0x82E4AE8C)
	// 82E4AE80: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AE84: 386B6FE4  addi r3, r11, 0x6fe4
	ctx.r[3].s64 = ctx.r[11].s64 + 28644;
	// 82E4AE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AE90 size=72
    let mut pc: u32 = 0x82E4AE90;
    'dispatch: loop {
        match pc {
            0x82E4AE90 => {
    //   block [0x82E4AE90..0x82E4AEC4)
	// 82E4AE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AE98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AE9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AEA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AEA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AEA8: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82E4AEAC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E4AEB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4AEB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AEB8: 419A000C  beq cr6, 0x82e4aec4
	if ctx.cr[6].eq {
	pc = 0x82E4AEC4; continue 'dispatch;
	}
	// 82E4AEBC: 4B9FA8F5  bl 0x828457b0
	ctx.lr = 0x82E4AEC0;
	sub_828457B0(ctx, base);
	// 82E4AEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82E4AEC4; continue 'dispatch;
            }
            0x82E4AEC4 => {
    //   block [0x82E4AEC4..0x82E4AED8)
	// 82E4AEC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4AEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AED8 size=20
    let mut pc: u32 = 0x82E4AED8;
    'dispatch: loop {
        match pc {
            0x82E4AED8 => {
    //   block [0x82E4AED8..0x82E4AEEC)
	// 82E4AED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AEDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AEE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AEE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AEE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AF08 size=44
    let mut pc: u32 = 0x82E4AF08;
    'dispatch: loop {
        match pc {
            0x82E4AF08 => {
    //   block [0x82E4AF08..0x82E4AF34)
	// 82E4AF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AF10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AF14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4AF1C: 4800001D  bl 0x82e4af38
	ctx.lr = 0x82E4AF20;
	sub_82E4AF38(ctx, base);
	// 82E4AF20: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4AF24: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4AF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AF38 size=152
    let mut pc: u32 = 0x82E4AF38;
    'dispatch: loop {
        match pc {
            0x82E4AF38 => {
    //   block [0x82E4AF38..0x82E4AFD0)
	// 82E4AF38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AF3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4AF40: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E4AF44: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4AF48: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4AF4C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4AF50: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4AF54: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4AF58: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E4AF5C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4AF60: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4AF64: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4AF68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AF6C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4AF70: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E4AF74: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4AF78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4AF7C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4AF80: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4AF84: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E4AF88: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4AF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AF90: 38E77044  addi r7, r7, 0x7044
	ctx.r[7].s64 = ctx.r[7].s64 + 28740;
	// 82E4AF94: 396B709C  addi r11, r11, 0x709c
	ctx.r[11].s64 = ctx.r[11].s64 + 28828;
	// 82E4AF98: 38C67090  addi r6, r6, 0x7090
	ctx.r[6].s64 = ctx.r[6].s64 + 28816;
	// 82E4AF9C: 394A707C  addi r10, r10, 0x707c
	ctx.r[10].s64 = ctx.r[10].s64 + 28796;
	// 82E4AFA0: 39297070  addi r9, r9, 0x7070
	ctx.r[9].s64 = ctx.r[9].s64 + 28784;
	// 82E4AFA4: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82E4AFA8: 38A57064  addi r5, r5, 0x7064
	ctx.r[5].s64 = ctx.r[5].s64 + 28772;
	// 82E4AFAC: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E4AFB0: 39087054  addi r8, r8, 0x7054
	ctx.r[8].s64 = ctx.r[8].s64 + 28756;
	// 82E4AFB4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AFB8: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E4AFBC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4AFC0: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4AFC4: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82E4AFC8: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E4AFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AFD0 size=100
    let mut pc: u32 = 0x82E4AFD0;
    'dispatch: loop {
        match pc {
            0x82E4AFD0 => {
    //   block [0x82E4AFD0..0x82E4B018)
	// 82E4AFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AFE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AFE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AFEC: 4800BF3D  bl 0x82e56f28
	ctx.lr = 0x82E4AFF0;
	sub_82E56F28(ctx, base);
	// 82E4AFF0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AFF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AFF8: 419A0020  beq cr6, 0x82e4b018
	if ctx.cr[6].eq {
	pc = 0x82E4B018; continue 'dispatch;
	}
	// 82E4AFFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B000: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B004: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4B008: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B00C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4B010: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B014: 4BF0A2B5  bl 0x82d552c8
	ctx.lr = 0x82E4B018;
	sub_82D552C8(ctx, base);
	pc = 0x82E4B018; continue 'dispatch;
            }
            0x82E4B018 => {
    //   block [0x82E4B018..0x82E4B034)
	// 82E4B018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B01C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4B020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4B024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4B028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4B02C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4B030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B038 size=8
    let mut pc: u32 = 0x82E4B038;
    'dispatch: loop {
        match pc {
            0x82E4B038 => {
    //   block [0x82E4B038..0x82E4B040)
	// 82E4B038: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4B03C: 4BFFFF94  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B040 size=8
    let mut pc: u32 = 0x82E4B040;
    'dispatch: loop {
        match pc {
            0x82E4B040 => {
    //   block [0x82E4B040..0x82E4B048)
	// 82E4B040: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4B044: 4BFFFF8C  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B048 size=8
    let mut pc: u32 = 0x82E4B048;
    'dispatch: loop {
        match pc {
            0x82E4B048 => {
    //   block [0x82E4B048..0x82E4B050)
	// 82E4B048: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4B04C: 4BFFFF84  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B050 size=8
    let mut pc: u32 = 0x82E4B050;
    'dispatch: loop {
        match pc {
            0x82E4B050 => {
    //   block [0x82E4B050..0x82E4B058)
	// 82E4B050: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82E4B054: 4BFFFF7C  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B058 size=8
    let mut pc: u32 = 0x82E4B058;
    'dispatch: loop {
        match pc {
            0x82E4B058 => {
    //   block [0x82E4B058..0x82E4B060)
	// 82E4B058: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4B05C: 4BFFFF74  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B060 size=32
    let mut pc: u32 = 0x82E4B060;
    'dispatch: loop {
        match pc {
            0x82E4B060 => {
    //   block [0x82E4B060..0x82E4B080)
	// 82E4B060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B064: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82E4B068: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B06C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4B070: 396B7134  addi r11, r11, 0x7134
	ctx.r[11].s64 = ctx.r[11].s64 + 28980;
	// 82E4B074: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4B078: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4B07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B080 size=20
    let mut pc: u32 = 0x82E4B080;
    'dispatch: loop {
        match pc {
            0x82E4B080 => {
    //   block [0x82E4B080..0x82E4B094)
	// 82E4B080: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B084: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4B088: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B090: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B098 size=12
    let mut pc: u32 = 0x82E4B098;
    'dispatch: loop {
        match pc {
            0x82E4B098 => {
    //   block [0x82E4B098..0x82E4B0A4)
	// 82E4B098: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B09C: 386B7134  addi r3, r11, 0x7134
	ctx.r[3].s64 = ctx.r[11].s64 + 28980;
	// 82E4B0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B0A8 size=332
    let mut pc: u32 = 0x82E4B0A8;
    'dispatch: loop {
        match pc {
            0x82E4B0A8 => {
    //   block [0x82E4B0A8..0x82E4B0F8)
	// 82E4B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B0AC: 4BE5E355  bl 0x82ca9400
	ctx.lr = 0x82E4B0B0;
	sub_82CA93D0(ctx, base);
	// 82E4B0B0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B0B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4B0B8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4B0BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4B0C0: 4BF36971  bl 0x82d81a30
	ctx.lr = 0x82E4B0C4;
	sub_82D81A30(ctx, base);
	// 82E4B0C4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B0C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B0CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B0D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B0D4: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B0D8: 38800310  li r4, 0x310
	ctx.r[4].s64 = 784;
	// 82E4B0DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B0E0: 409A0018  bne cr6, 0x82e4b0f8
	if !ctx.cr[6].eq {
	pc = 0x82E4B0F8; continue 'dispatch;
	}
	// 82E4B0E4: 4BF0A165  bl 0x82d55248
	ctx.lr = 0x82E4B0E8;
	sub_82D55248(ctx, base);
	// 82E4B0E8: 39600310  li r11, 0x310
	ctx.r[11].s64 = 784;
	// 82E4B0EC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E4B0F0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B0F4: 48000014  b 0x82e4b108
	pc = 0x82E4B108; continue 'dispatch;
            }
            0x82E4B0F8 => {
    //   block [0x82E4B0F8..0x82E4B108)
	// 82E4B0F8: 4BF0A151  bl 0x82d55248
	ctx.lr = 0x82E4B0FC;
	sub_82D55248(ctx, base);
	// 82E4B0FC: 39600310  li r11, 0x310
	ctx.r[11].s64 = 784;
	// 82E4B100: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B104: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82E4B108; continue 'dispatch;
            }
            0x82E4B108 => {
    //   block [0x82E4B108..0x82E4B12C)
	// 82E4B108: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 82E4B10C: 60A5C3B4  ori r5, r5, 0xc3b4
	ctx.r[5].u64 = ctx.r[5].u64 | 50100;
	// 82E4B110: 4BF2F3F9  bl 0x82d7a508
	ctx.lr = 0x82E4B114;
	sub_82D7A508(ctx, base);
	// 82E4B114: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 82E4B118: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4B11C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B120: 419A000C  beq cr6, 0x82e4b12c
	if ctx.cr[6].eq {
	pc = 0x82E4B12C; continue 'dispatch;
	}
	// 82E4B124: 807C0074  lwz r3, 0x74(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E4B128: 4BF682F9  bl 0x82db3420
	ctx.lr = 0x82E4B12C;
	sub_82DB3420(ctx, base);
	pc = 0x82E4B12C; continue 'dispatch;
            }
            0x82E4B12C => {
    //   block [0x82E4B12C..0x82E4B148)
	// 82E4B12C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B130: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B134: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82E4B138: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B13C: 40990084  ble cr6, 0x82e4b1c0
	if !ctx.cr[6].gt {
	pc = 0x82E4B1C0; continue 'dispatch;
	}
	// 82E4B140: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E4B144: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	pc = 0x82E4B148; continue 'dispatch;
            }
            0x82E4B148 => {
    //   block [0x82E4B148..0x82E4B1AC)
	// 82E4B148: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B14C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B150: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B154: 4BF33CAD  bl 0x82d7ee00
	ctx.lr = 0x82E4B158;
	sub_82D7EE00(ctx, base);
	// 82E4B158: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B15C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B160: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B164: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B168: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B16C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B170: 4E800421  bctrl
	ctx.lr = 0x82E4B174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4B174: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B17C: 419A0030  beq cr6, 0x82e4b1ac
	if ctx.cr[6].eq {
	pc = 0x82E4B1AC; continue 'dispatch;
	}
	// 82E4B180: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B184: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E4B188: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4B18C: 9BA10058  stb r29, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u8 ) };
	// 82E4B190: 9BA10059  stb r29, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[29].u8 ) };
	// 82E4B194: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82E4B198: 9B610060  stb r27, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u8 ) };
	// 82E4B19C: 9B610061  stb r27, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[27].u8 ) };
	// 82E4B1A0: 9BA10062  stb r29, 0x62(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[29].u8 ) };
	// 82E4B1A4: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B1A8: 4800DEB1  bl 0x82e59058
	ctx.lr = 0x82E4B1AC;
	sub_82E59058(ctx, base);
            }
            0x82E4B1AC => {
    //   block [0x82E4B1AC..0x82E4B1C0)
	// 82E4B1AC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B1B0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82E4B1B4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B1B8: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B1BC: 4198FF8C  blt cr6, 0x82e4b148
	if ctx.cr[6].lt {
	pc = 0x82E4B148; continue 'dispatch;
	}
	pc = 0x82E4B1C0; continue 'dispatch;
            }
            0x82E4B1C0 => {
    //   block [0x82E4B1C0..0x82E4B1EC)
	// 82E4B1C0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B1C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B1C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B1CC: 419A0020  beq cr6, 0x82e4b1ec
	if ctx.cr[6].eq {
	pc = 0x82E4B1EC; continue 'dispatch;
	}
	// 82E4B1D0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4B1D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B1D8: 419A0014  beq cr6, 0x82e4b1ec
	if ctx.cr[6].eq {
	pc = 0x82E4B1EC; continue 'dispatch;
	}
	// 82E4B1DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E4B1E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4B1E4: 4BF322DD  bl 0x82d7d4c0
	ctx.lr = 0x82E4B1E8;
	sub_82D7D4C0(ctx, base);
	// 82E4B1E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	pc = 0x82E4B1EC; continue 'dispatch;
            }
            0x82E4B1EC => {
    //   block [0x82E4B1EC..0x82E4B1F4)
	// 82E4B1EC: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 82E4B1F0: 4BE5E260  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B1F8 size=232
    let mut pc: u32 = 0x82E4B1F8;
    'dispatch: loop {
        match pc {
            0x82E4B1F8 => {
    //   block [0x82E4B1F8..0x82E4B224)
	// 82E4B1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B1FC: 4BE5E20D  bl 0x82ca9408
	ctx.lr = 0x82E4B200;
	sub_82CA93D0(ctx, base);
	// 82E4B200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B204: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4B208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4B20C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E4B210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4B214: 813C000C  lwz r9, 0xc(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B218: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E4B21C: 40990024  ble cr6, 0x82e4b240
	if !ctx.cr[6].gt {
	pc = 0x82E4B240; continue 'dispatch;
	}
	// 82E4B220: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82E4B224; continue 'dispatch;
            }
            0x82E4B224 => {
    //   block [0x82E4B224..0x82E4B240)
	// 82E4B224: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B228: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4B22C: 419A004C  beq cr6, 0x82e4b278
	if ctx.cr[6].eq {
	pc = 0x82E4B278; continue 'dispatch;
	}
	// 82E4B230: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4B234: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E4B238: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4B23C: 4198FFE8  blt cr6, 0x82e4b224
	if ctx.cr[6].lt {
	pc = 0x82E4B224; continue 'dispatch;
	}
	pc = 0x82E4B240; continue 'dispatch;
            }
            0x82E4B240 => {
    //   block [0x82E4B240..0x82E4B244)
	// 82E4B240: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	pc = 0x82E4B244; continue 'dispatch;
            }
            0x82E4B244 => {
    //   block [0x82E4B244..0x82E4B258)
	// 82E4B244: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4B24C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E4B250: 40990038  ble cr6, 0x82e4b288
	if !ctx.cr[6].gt {
	pc = 0x82E4B288; continue 'dispatch;
	}
	// 82E4B254: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82E4B258; continue 'dispatch;
            }
            0x82E4B258 => {
    //   block [0x82E4B258..0x82E4B278)
	// 82E4B258: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B25C: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4B260: 419A0020  beq cr6, 0x82e4b280
	if ctx.cr[6].eq {
	pc = 0x82E4B280; continue 'dispatch;
	}
	// 82E4B264: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4B268: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E4B26C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4B270: 4198FFE8  blt cr6, 0x82e4b258
	if ctx.cr[6].lt {
	pc = 0x82E4B258; continue 'dispatch;
	}
	// 82E4B274: 48000014  b 0x82e4b288
	pc = 0x82E4B288; continue 'dispatch;
            }
            0x82E4B278 => {
    //   block [0x82E4B278..0x82E4B280)
	// 82E4B278: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E4B27C: 4BFFFFC8  b 0x82e4b244
	pc = 0x82E4B244; continue 'dispatch;
            }
            0x82E4B280 => {
    //   block [0x82E4B280..0x82E4B288)
	// 82E4B280: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82E4B284: 409A0054  bne cr6, 0x82e4b2d8
	if !ctx.cr[6].eq {
	pc = 0x82E4B2D8; continue 'dispatch;
	}
	pc = 0x82E4B288; continue 'dispatch;
            }
            0x82E4B288 => {
    //   block [0x82E4B288..0x82E4B2B8)
	// 82E4B288: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82E4B28C: 419A004C  beq cr6, 0x82e4b2d8
	if ctx.cr[6].eq {
	pc = 0x82E4B2D8; continue 'dispatch;
	}
	// 82E4B290: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E4B294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B298: 409A0020  bne cr6, 0x82e4b2b8
	if !ctx.cr[6].eq {
	pc = 0x82E4B2B8; continue 'dispatch;
	}
	// 82E4B29C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B2A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B2A4: 4BF39FB5  bl 0x82d85258
	ctx.lr = 0x82E4B2A8;
	sub_82D85258(ctx, base);
	// 82E4B2A8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B2AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4B2B4: 419A0008  beq cr6, 0x82e4b2bc
	if ctx.cr[6].eq {
	pc = 0x82E4B2BC; continue 'dispatch;
	}
	pc = 0x82E4B2B8; continue 'dispatch;
            }
            0x82E4B2B8 => {
    //   block [0x82E4B2B8..0x82E4B2BC)
	// 82E4B2B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82E4B2BC; continue 'dispatch;
            }
            0x82E4B2BC => {
    //   block [0x82E4B2BC..0x82E4B2D8)
	// 82E4B2BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B2C0: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E4B2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B2C8: 4BF38121  bl 0x82d833e8
	ctx.lr = 0x82E4B2CC;
	sub_82D833E8(ctx, base);
	// 82E4B2CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4B2D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B2D4: 4BF379B5  bl 0x82d82c88
	ctx.lr = 0x82E4B2D8;
	sub_82D82C88(ctx, base);
	pc = 0x82E4B2D8; continue 'dispatch;
            }
            0x82E4B2D8 => {
    //   block [0x82E4B2D8..0x82E4B2E0)
	// 82E4B2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B2DC: 4BE5E17C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B2E0 size=128
    let mut pc: u32 = 0x82E4B2E0;
    'dispatch: loop {
        match pc {
            0x82E4B2E0 => {
    //   block [0x82E4B2E0..0x82E4B308)
	// 82E4B2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B2E4: 4BE5E125  bl 0x82ca9408
	ctx.lr = 0x82E4B2E8;
	sub_82CA93D0(ctx, base);
	// 82E4B2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B2EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4B2F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4B2F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B2F8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B2FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B300: 40990040  ble cr6, 0x82e4b340
	if !ctx.cr[6].gt {
	pc = 0x82E4B340; continue 'dispatch;
	}
	// 82E4B304: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82E4B308; continue 'dispatch;
            }
            0x82E4B308 => {
    //   block [0x82E4B308..0x82E4B32C)
	// 82E4B308: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B30C: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E4B310: 806B0038  lwz r3, 0x38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E4B314: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B318: 419A0014  beq cr6, 0x82e4b32c
	if ctx.cr[6].eq {
	pc = 0x82E4B32C; continue 'dispatch;
	}
	// 82E4B31C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4B320: 4BF0D6F1  bl 0x82d58a10
	ctx.lr = 0x82E4B324;
	sub_82D58A10(ctx, base);
	// 82E4B324: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E4B328: 419A0024  beq cr6, 0x82e4b34c
	if ctx.cr[6].eq {
	pc = 0x82E4B34C; continue 'dispatch;
	}
	pc = 0x82E4B32C; continue 'dispatch;
            }
            0x82E4B32C => {
    //   block [0x82E4B32C..0x82E4B340)
	// 82E4B32C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B330: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B334: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B338: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B33C: 4198FFCC  blt cr6, 0x82e4b308
	if ctx.cr[6].lt {
	pc = 0x82E4B308; continue 'dispatch;
	}
	pc = 0x82E4B340; continue 'dispatch;
            }
            0x82E4B340 => {
    //   block [0x82E4B340..0x82E4B34C)
	// 82E4B340: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4B344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B348: 4BE5E110  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4B34C => {
    //   block [0x82E4B34C..0x82E4B360)
	// 82E4B34C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B350: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4B354: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B35C: 4BE5E0FC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B360 size=168
    let mut pc: u32 = 0x82E4B360;
    'dispatch: loop {
        match pc {
            0x82E4B360 => {
    //   block [0x82E4B360..0x82E4B388)
	// 82E4B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B364: 4BE5E095  bl 0x82ca93f8
	ctx.lr = 0x82E4B368;
	sub_82CA93D0(ctx, base);
	// 82E4B368: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B36C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E4B370: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E4B374: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E4B378: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B37C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B380: 40990070  ble cr6, 0x82e4b3f0
	if !ctx.cr[6].gt {
	pc = 0x82E4B3F0; continue 'dispatch;
	}
	// 82E4B384: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	pc = 0x82E4B388; continue 'dispatch;
            }
            0x82E4B388 => {
    //   block [0x82E4B388..0x82E4B3A4)
	// 82E4B388: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B38C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4B390: 7FCBD82E  lwzx r30, r11, r27
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4B394: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B39C: 40990040  ble cr6, 0x82e4b3dc
	if !ctx.cr[6].gt {
	pc = 0x82E4B3DC; continue 'dispatch;
	}
	// 82E4B3A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82E4B3A4; continue 'dispatch;
            }
            0x82E4B3A4 => {
    //   block [0x82E4B3A4..0x82E4B3C8)
	// 82E4B3A4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B3A8: 7FBF582E  lwzx r29, r31, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B3AC: 807D0070  lwz r3, 0x70(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E4B3B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B3B4: 419A0014  beq cr6, 0x82e4b3c8
	if ctx.cr[6].eq {
	pc = 0x82E4B3C8; continue 'dispatch;
	}
	// 82E4B3B8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E4B3BC: 4BF0D655  bl 0x82d58a10
	ctx.lr = 0x82E4B3C0;
	sub_82D58A10(ctx, base);
	// 82E4B3C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E4B3C4: 419A0038  beq cr6, 0x82e4b3fc
	if ctx.cr[6].eq {
	pc = 0x82E4B3FC; continue 'dispatch;
	}
	pc = 0x82E4B3C8; continue 'dispatch;
            }
            0x82E4B3C8 => {
    //   block [0x82E4B3C8..0x82E4B3DC)
	// 82E4B3C8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B3CC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E4B3D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B3D4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B3D8: 4198FFCC  blt cr6, 0x82e4b3a4
	if ctx.cr[6].lt {
	pc = 0x82E4B3A4; continue 'dispatch;
	}
	pc = 0x82E4B3DC; continue 'dispatch;
            }
            0x82E4B3DC => {
    //   block [0x82E4B3DC..0x82E4B3F0)
	// 82E4B3DC: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B3E0: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E4B3E4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E4B3E8: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B3EC: 4198FF9C  blt cr6, 0x82e4b388
	if ctx.cr[6].lt {
	pc = 0x82E4B388; continue 'dispatch;
	}
	pc = 0x82E4B3F0; continue 'dispatch;
            }
            0x82E4B3F0 => {
    //   block [0x82E4B3F0..0x82E4B3FC)
	// 82E4B3F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4B3F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4B3F8: 4BE5E050  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4B3FC => {
    //   block [0x82E4B3FC..0x82E4B408)
	// 82E4B3FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4B400: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4B404: 4BE5E044  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B408 size=48
    let mut pc: u32 = 0x82E4B408;
    'dispatch: loop {
        match pc {
            0x82E4B408 => {
    //   block [0x82E4B408..0x82E4B438)
	// 82E4B408: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B40C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E4B410: 394B5D20  addi r10, r11, 0x5d20
	ctx.r[10].s64 = ctx.r[11].s64 + 23840;
	// 82E4B414: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4B418: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82E4B41C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E4B420: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4B424: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E4B428: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E4B42C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4B430: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4B434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B438 size=264
    let mut pc: u32 = 0x82E4B438;
    'dispatch: loop {
        match pc {
            0x82E4B438 => {
    //   block [0x82E4B438..0x82E4B498)
	// 82E4B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B43C: 4BE5DFD1  bl 0x82ca940c
	ctx.lr = 0x82E4B440;
	sub_82CA93D0(ctx, base);
	// 82E4B440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B44C: 396B5D20  addi r11, r11, 0x5d20
	ctx.r[11].s64 = ctx.r[11].s64 + 23840;
	// 82E4B450: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B454: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B458: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4B45C: 419A003C  beq cr6, 0x82e4b498
	if ctx.cr[6].eq {
	pc = 0x82E4B498; continue 'dispatch;
	}
	// 82E4B460: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B464: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B468: 419A0030  beq cr6, 0x82e4b498
	if ctx.cr[6].eq {
	pc = 0x82E4B498; continue 'dispatch;
	}
	// 82E4B46C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4B470: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4B474: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4B478: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B47C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4B480: 409A0018  bne cr6, 0x82e4b498
	if !ctx.cr[6].eq {
	pc = 0x82E4B498; continue 'dispatch;
	}
	// 82E4B484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B488: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4B48C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B494: 4E800421  bctrl
	ctx.lr = 0x82E4B498;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4B498 => {
    //   block [0x82E4B498..0x82E4B4AC)
	// 82E4B498: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B49C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B4A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B4A4: 4099005C  ble cr6, 0x82e4b500
	if !ctx.cr[6].gt {
	pc = 0x82E4B500; continue 'dispatch;
	}
	// 82E4B4A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82E4B4AC; continue 'dispatch;
            }
            0x82E4B4AC => {
    //   block [0x82E4B4AC..0x82E4B4EC)
	// 82E4B4AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B4B0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4B4B4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B4B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B4BC: 419A0030  beq cr6, 0x82e4b4ec
	if ctx.cr[6].eq {
	pc = 0x82E4B4EC; continue 'dispatch;
	}
	// 82E4B4C0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4B4C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4B4C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4B4CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B4D0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4B4D4: 409A0018  bne cr6, 0x82e4b4ec
	if !ctx.cr[6].eq {
	pc = 0x82E4B4EC; continue 'dispatch;
	}
	// 82E4B4D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B4DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4B4E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B4E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B4E8: 4E800421  bctrl
	ctx.lr = 0x82E4B4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4B4EC => {
    //   block [0x82E4B4EC..0x82E4B500)
	// 82E4B4EC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B4F0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B4F4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B4F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B4FC: 4198FFB0  blt cr6, 0x82e4b4ac
	if ctx.cr[6].lt {
	pc = 0x82E4B4AC; continue 'dispatch;
	}
	pc = 0x82E4B500; continue 'dispatch;
            }
            0x82E4B500 => {
    //   block [0x82E4B500..0x82E4B52C)
	// 82E4B500: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B504: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4B508: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4B50C: 409A0020  bne cr6, 0x82e4b52c
	if !ctx.cr[6].eq {
	pc = 0x82E4B52C; continue 'dispatch;
	}
	// 82E4B510: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B514: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4B518: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4B51C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B520: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4B524: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4B528: 4BF09DA1  bl 0x82d552c8
	ctx.lr = 0x82E4B52C;
	sub_82D552C8(ctx, base);
	pc = 0x82E4B52C; continue 'dispatch;
            }
            0x82E4B52C => {
    //   block [0x82E4B52C..0x82E4B540)
	// 82E4B52C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4B530: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4B534: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4B538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4B53C: 4BE5DF20  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B540 size=256
    let mut pc: u32 = 0x82E4B540;
    'dispatch: loop {
        match pc {
            0x82E4B540 => {
    //   block [0x82E4B540..0x82E4B58C)
	// 82E4B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B544: 4BE5DEC5  bl 0x82ca9408
	ctx.lr = 0x82E4B548;
	sub_82CA93D0(ctx, base);
	// 82E4B548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B54C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B550: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4B554: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E4B558: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B560: 409A002C  bne cr6, 0x82e4b58c
	if !ctx.cr[6].eq {
	pc = 0x82E4B58C; continue 'dispatch;
	}
	// 82E4B564: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B568: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B56C: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82E4B570: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82E4B574: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B578: 4BF09CD1  bl 0x82d55248
	ctx.lr = 0x82E4B57C;
	sub_82D55248(ctx, base);
	// 82E4B57C: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82E4B580: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B584: 4BF364AD  bl 0x82d81a30
	ctx.lr = 0x82E4B588;
	sub_82D81A30(ctx, base);
	// 82E4B588: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	pc = 0x82E4B58C; continue 'dispatch;
            }
            0x82E4B58C => {
    //   block [0x82E4B58C..0x82E4B61C)
	// 82E4B58C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B590: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B594: 4BF2D1A5  bl 0x82d78738
	ctx.lr = 0x82E4B598;
	sub_82D78738(ctx, base);
	// 82E4B598: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82E4B59C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B5A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4B5A4: 4BF30285  bl 0x82d7b828
	ctx.lr = 0x82E4B5A8;
	sub_82D7B828(ctx, base);
	// 82E4B5A8: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E4B5AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B5B0: 419A0088  beq cr6, 0x82e4b638
	if ctx.cr[6].eq {
	pc = 0x82E4B638; continue 'dispatch;
	}
	// 82E4B5B4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B5B8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B5BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B5C0: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B5C4: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82E4B5C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B5CC: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82E4B5D0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82E4B5D4: 4BF09C75  bl 0x82d55248
	ctx.lr = 0x82E4B5D8;
	sub_82D55248(ctx, base);
	// 82E4B5D8: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 82E4B5DC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B5E0: 48000759  bl 0x82e4bd38
	ctx.lr = 0x82E4B5E4;
	sub_82E4BD38(ctx, base);
	// 82E4B5E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4B5E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4B5EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B5F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4B5F4: 4800CD45  bl 0x82e58338
	ctx.lr = 0x82E4B5F8;
	sub_82E58338(ctx, base);
	// 82E4B5F8: 9BBE0040  stb r29, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E4B5FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B600: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B604: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4B608: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B60C: 409A0010  bne cr6, 0x82e4b61c
	if !ctx.cr[6].eq {
	pc = 0x82E4B61C; continue 'dispatch;
	}
	// 82E4B610: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4B614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B618: 4BF0B981  bl 0x82d56f98
	ctx.lr = 0x82E4B61C;
	sub_82D56F98(ctx, base);
	pc = 0x82E4B61C; continue 'dispatch;
            }
            0x82E4B61C => {
    //   block [0x82E4B61C..0x82E4B638)
	// 82E4B61C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B620: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B624: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4B628: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E4B62C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B630: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4B634: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82E4B638; continue 'dispatch;
            }
            0x82E4B638 => {
    //   block [0x82E4B638..0x82E4B640)
	// 82E4B638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B63C: 4BE5DE1C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B640 size=1780
    let mut pc: u32 = 0x82E4B640;
    'dispatch: loop {
        match pc {
            0x82E4B640 => {
    //   block [0x82E4B640..0x82E4B69C)
	// 82E4B640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B644: 4BE5DD8D  bl 0x82ca93d0
	ctx.lr = 0x82E4B648;
	sub_82CA93D0(ctx, base);
	// 82E4B648: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B64C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B650: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E4B654: 39C00004  li r14, 4
	ctx.r[14].s64 = 4;
	// 82E4B658: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B65C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B664: 9301012C  stw r24, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[24].u32 ) };
	// 82E4B668: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B66C: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 82E4B670: 4BF09BD9  bl 0x82d55248
	ctx.lr = 0x82E4B674;
	sub_82D55248(ctx, base);
	// 82E4B674: 3B400044  li r26, 0x44
	ctx.r[26].s64 = 68;
	// 82E4B678: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B67C: 4BF377ED  bl 0x82d82e68
	ctx.lr = 0x82E4B680;
	sub_82D82E68(ctx, base);
	// 82E4B680: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B684: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4B688: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E4B68C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B690: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B694: 4099002C  ble cr6, 0x82e4b6c0
	if !ctx.cr[6].gt {
	pc = 0x82E4B6C0; continue 'dispatch;
	}
	// 82E4B698: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	pc = 0x82E4B69C; continue 'dispatch;
            }
            0x82E4B69C => {
    //   block [0x82E4B69C..0x82E4B6C0)
	// 82E4B69C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B6A0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B6A4: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B6A8: 4BF37D41  bl 0x82d833e8
	ctx.lr = 0x82E4B6AC;
	sub_82D833E8(ctx, base);
	// 82E4B6AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B6B0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4B6B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E4B6B8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B6BC: 4198FFE0  blt cr6, 0x82e4b69c
	if ctx.cr[6].lt {
	pc = 0x82E4B69C; continue 'dispatch;
	}
	pc = 0x82E4B6C0; continue 'dispatch;
            }
            0x82E4B6C0 => {
    //   block [0x82E4B6C0..0x82E4B6D4)
	// 82E4B6C0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4B6C4: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B6C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B6CC: 4099002C  ble cr6, 0x82e4b6f8
	if !ctx.cr[6].gt {
	pc = 0x82E4B6F8; continue 'dispatch;
	}
	// 82E4B6D0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	pc = 0x82E4B6D4; continue 'dispatch;
            }
            0x82E4B6D4 => {
    //   block [0x82E4B6D4..0x82E4B6F8)
	// 82E4B6D4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4B6D8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B6DC: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B6E0: 4BF37EA9  bl 0x82d83588
	ctx.lr = 0x82E4B6E4;
	sub_82D83588(ctx, base);
	// 82E4B6E4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4B6E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B6EC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B6F0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B6F4: 4198FFE0  blt cr6, 0x82e4b6d4
	if ctx.cr[6].lt {
	pc = 0x82E4B6D4; continue 'dispatch;
	}
	pc = 0x82E4B6F8; continue 'dispatch;
            }
            0x82E4B6F8 => {
    //   block [0x82E4B6F8..0x82E4B70C)
	// 82E4B6F8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B6FC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B704: 4099002C  ble cr6, 0x82e4b730
	if !ctx.cr[6].gt {
	pc = 0x82E4B730; continue 'dispatch;
	}
	// 82E4B708: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	pc = 0x82E4B70C; continue 'dispatch;
            }
            0x82E4B70C => {
    //   block [0x82E4B70C..0x82E4B730)
	// 82E4B70C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B710: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B714: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B718: 4BF37DE1  bl 0x82d834f8
	ctx.lr = 0x82E4B71C;
	sub_82D834F8(ctx, base);
	// 82E4B71C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B720: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B724: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B728: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B72C: 4198FFE0  blt cr6, 0x82e4b70c
	if ctx.cr[6].lt {
	pc = 0x82E4B70C; continue 'dispatch;
	}
	pc = 0x82E4B730; continue 'dispatch;
            }
            0x82E4B730 => {
    //   block [0x82E4B730..0x82E4B744)
	// 82E4B730: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4B734: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B738: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B73C: 4099002C  ble cr6, 0x82e4b768
	if !ctx.cr[6].gt {
	pc = 0x82E4B768; continue 'dispatch;
	}
	// 82E4B740: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	pc = 0x82E4B744; continue 'dispatch;
            }
            0x82E4B744 => {
    //   block [0x82E4B744..0x82E4B768)
	// 82E4B744: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E4B748: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B74C: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4B750: 4BF37D21  bl 0x82d83470
	ctx.lr = 0x82E4B754;
	sub_82D83470(ctx, base);
	// 82E4B754: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4B758: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B75C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B760: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B764: 4198FFE0  blt cr6, 0x82e4b744
	if ctx.cr[6].lt {
	pc = 0x82E4B744; continue 'dispatch;
	}
	pc = 0x82E4B768; continue 'dispatch;
            }
            0x82E4B768 => {
    //   block [0x82E4B768..0x82E4B7B4)
	// 82E4B768: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B76C: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B770: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B774: 3B790008  addi r27, r25, 8
	ctx.r[27].s64 = ctx.r[25].s64 + 8;
	// 82E4B778: 4BF09AD1  bl 0x82d55248
	ctx.lr = 0x82E4B77C;
	sub_82D55248(ctx, base);
	// 82E4B77C: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B780: 4BF376E9  bl 0x82d82e68
	ctx.lr = 0x82E4B784;
	sub_82D82E68(ctx, base);
	// 82E4B784: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B78C: 396B71B0  addi r11, r11, 0x71b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29104;
	// 82E4B790: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B794: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B798: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B79C: 9B9F0040  stb r28, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82E4B7A0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B7A4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B7A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B7AC: 40990048  ble cr6, 0x82e4b7f4
	if !ctx.cr[6].gt {
	pc = 0x82E4B7F4; continue 'dispatch;
	}
	// 82E4B7B0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	pc = 0x82E4B7B4; continue 'dispatch;
            }
            0x82E4B7B4 => {
    //   block [0x82E4B7B4..0x82E4B7E0)
	// 82E4B7B4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B7B8: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B7BC: 896400D8  lbz r11, 0xd8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4B7C0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E4B7C4: 409A001C  bne cr6, 0x82e4b7e0
	if !ctx.cr[6].eq {
	pc = 0x82E4B7E0; continue 'dispatch;
	}
	// 82E4B7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B7CC: 4BF37C1D  bl 0x82d833e8
	ctx.lr = 0x82E4B7D0;
	sub_82D833E8(ctx, base);
	// 82E4B7D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B7D4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B7D8: 4BF374B1  bl 0x82d82c88
	ctx.lr = 0x82E4B7DC;
	sub_82D82C88(ctx, base);
	// 82E4B7DC: 4800000C  b 0x82e4b7e8
	pc = 0x82E4B7E8; continue 'dispatch;
            }
            0x82E4B7E0 => {
    //   block [0x82E4B7E0..0x82E4B7E8)
	// 82E4B7E0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4B7E4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	pc = 0x82E4B7E8; continue 'dispatch;
            }
            0x82E4B7E8 => {
    //   block [0x82E4B7E8..0x82E4B7F4)
	// 82E4B7E8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B7EC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B7F0: 4198FFC4  blt cr6, 0x82e4b7b4
	if ctx.cr[6].lt {
	pc = 0x82E4B7B4; continue 'dispatch;
	}
	pc = 0x82E4B7F4; continue 'dispatch;
            }
            0x82E4B7F4 => {
    //   block [0x82E4B7F4..0x82E4B808)
	// 82E4B7F4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B7F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B7FC: 419A000C  beq cr6, 0x82e4b808
	if ctx.cr[6].eq {
	pc = 0x82E4B808; continue 'dispatch;
	}
	// 82E4B800: 93F80000  stw r31, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E4B804: 48000020  b 0x82e4b824
	pc = 0x82E4B824; continue 'dispatch;
            }
            0x82E4B808 => {
    //   block [0x82E4B808..0x82E4B824)
	// 82E4B808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B80C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4B810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B814: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B81C: 4E800421  bctrl
	ctx.lr = 0x82E4B820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4B820: 93980000  stw r28, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
            }
            0x82E4B824 => {
    //   block [0x82E4B824..0x82E4B8B0)
	// 82E4B824: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B828: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B82C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B830: 4BF09A19  bl 0x82d55248
	ctx.lr = 0x82E4B834;
	sub_82D55248(ctx, base);
	// 82E4B834: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B838: 4BF37631  bl 0x82d82e68
	ctx.lr = 0x82E4B83C;
	sub_82D82E68(ctx, base);
	// 82E4B83C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B840: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B844: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82E4B848: 396B7194  addi r11, r11, 0x7194
	ctx.r[11].s64 = ctx.r[11].s64 + 29076;
	// 82E4B84C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B850: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B854: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B858: 9B830040  stb r28, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82E4B85C: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B860: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B864: 4BF099E5  bl 0x82d55248
	ctx.lr = 0x82E4B868;
	sub_82D55248(ctx, base);
	// 82E4B868: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B86C: 4BF375FD  bl 0x82d82e68
	ctx.lr = 0x82E4B870;
	sub_82D82E68(ctx, base);
	// 82E4B870: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B874: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82E4B878: 3A590014  addi r18, r25, 0x14
	ctx.r[18].s64 = ctx.r[25].s64 + 20;
	// 82E4B87C: 396B717C  addi r11, r11, 0x717c
	ctx.r[11].s64 = ctx.r[11].s64 + 29052;
	// 82E4B880: 3A790020  addi r19, r25, 0x20
	ctx.r[19].s64 = ctx.r[25].s64 + 32;
	// 82E4B884: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B888: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B88C: 9B830040  stb r28, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82E4B890: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B894: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B898: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B89C: 40990388  ble cr6, 0x82e4bc24
	if !ctx.cr[6].gt {
	pc = 0x82E4BC24; continue 'dispatch;
	}
	// 82E4B8A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B8A4: 3E008000  lis r16, -0x8000
	ctx.r[16].s64 = -2147483648;
	// 82E4B8A8: 396B7168  addi r11, r11, 0x7168
	ctx.r[11].s64 = ctx.r[11].s64 + 29032;
	// 82E4B8AC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	pc = 0x82E4B8B0; continue 'dispatch;
            }
            0x82E4B8B0 => {
    //   block [0x82E4B8B0..0x82E4B918)
	// 82E4B8B0: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B8B4: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B8B8: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B8BC: 4BF0998D  bl 0x82d55248
	ctx.lr = 0x82E4B8C0;
	sub_82D55248(ctx, base);
	// 82E4B8C0: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 82E4B8C4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B8C8: 4BF375A1  bl 0x82d82e68
	ctx.lr = 0x82E4B8CC;
	sub_82D82E68(ctx, base);
	// 82E4B8CC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4B8D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4B8D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B8D8: 917D0038  stw r11, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B8DC: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B8E0: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B8E4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B8E8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B8EC: 4BF3996D  bl 0x82d85258
	ctx.lr = 0x82E4B8F0;
	sub_82D85258(ctx, base);
	// 82E4B8F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4B8F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4B8F8: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B8FC: 997D0040  stb r11, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E4B900: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B904: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B908: 4BF37AE1  bl 0x82d833e8
	ctx.lr = 0x82E4B90C;
	sub_82D833E8(ctx, base);
	// 82E4B90C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4B910: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B914: 4BF37375  bl 0x82d82c88
	ctx.lr = 0x82E4B918;
	sub_82D82C88(ctx, base);
	pc = 0x82E4B918; continue 'dispatch;
            }
            0x82E4B918 => {
    //   block [0x82E4B918..0x82E4B930)
	// 82E4B918: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B91C: 7F94E378  mr r20, r28
	ctx.r[20].u64 = ctx.r[28].u64;
	// 82E4B920: 7F8FE378  mr r15, r28
	ctx.r[15].u64 = ctx.r[28].u64;
	// 82E4B924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B928: 409901F4  ble cr6, 0x82e4bb1c
	if !ctx.cr[6].gt {
	pc = 0x82E4BB1C; continue 'dispatch;
	}
	// 82E4B92C: 7F91E378  mr r17, r28
	ctx.r[17].u64 = ctx.r[28].u64;
	pc = 0x82E4B930; continue 'dispatch;
            }
            0x82E4B930 => {
    //   block [0x82E4B930..0x82E4B94C)
	// 82E4B930: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B934: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B938: 81520004  lwz r10, 4(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B93C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4B940: 7EAB882E  lwzx r21, r11, r17
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82E4B944: 40990090  ble cr6, 0x82e4b9d4
	if !ctx.cr[6].gt {
	pc = 0x82E4B9D4; continue 'dispatch;
	}
	// 82E4B948: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	pc = 0x82E4B94C; continue 'dispatch;
            }
            0x82E4B94C => {
    //   block [0x82E4B94C..0x82E4B978)
	// 82E4B94C: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B950: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B954: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B958: 7F0BA840  cmplw cr6, r11, r21
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4B95C: 419A001C  beq cr6, 0x82e4b978
	if ctx.cr[6].eq {
	pc = 0x82E4B978; continue 'dispatch;
	}
	// 82E4B960: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B964: 7F0BA840  cmplw cr6, r11, r21
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4B968: 419A0010  beq cr6, 0x82e4b978
	if ctx.cr[6].eq {
	pc = 0x82E4B978; continue 'dispatch;
	}
	// 82E4B96C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4B970: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B974: 48000054  b 0x82e4b9c8
	pc = 0x82E4B9C8; continue 'dispatch;
            }
            0x82E4B978 => {
    //   block [0x82E4B978..0x82E4B9BC)
	// 82E4B978: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4B97C: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82E4B980: 4BF37B79  bl 0x82d834f8
	ctx.lr = 0x82E4B984;
	sub_82D834F8(ctx, base);
	// 82E4B984: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B988: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B98C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B990: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B994: 7D4B5A78  xor r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82E4B998: 7D65AA78  xor r5, r11, r21
	ctx.r[5].u64 = ctx.r[11].u64 ^ ctx.r[21].u64;
	// 82E4B99C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E4B9A0: 419A001C  beq cr6, 0x82e4b9bc
	if ctx.cr[6].eq {
	pc = 0x82E4B9BC; continue 'dispatch;
	}
	// 82E4B9A4: 896500D8  lbz r11, 0xd8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4B9A8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E4B9AC: 419A0010  beq cr6, 0x82e4b9bc
	if ctx.cr[6].eq {
	pc = 0x82E4B9BC; continue 'dispatch;
	}
	// 82E4B9B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4B9B4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B9B8: 4BFFF841  bl 0x82e4b1f8
	ctx.lr = 0x82E4B9BC;
	sub_82E4B1F8(ctx, base);
	pc = 0x82E4B9BC; continue 'dispatch;
            }
            0x82E4B9BC => {
    //   block [0x82E4B9BC..0x82E4B9C8)
	// 82E4B9BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B9C0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B9C4: 4BF37385  bl 0x82d82d48
	ctx.lr = 0x82E4B9C8;
	sub_82D82D48(ctx, base);
	pc = 0x82E4B9C8; continue 'dispatch;
            }
            0x82E4B9C8 => {
    //   block [0x82E4B9C8..0x82E4B9D4)
	// 82E4B9C8: 81720004  lwz r11, 4(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B9CC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B9D0: 4198FF7C  blt cr6, 0x82e4b94c
	if ctx.cr[6].lt {
	pc = 0x82E4B94C; continue 'dispatch;
	}
	pc = 0x82E4B9D4; continue 'dispatch;
            }
            0x82E4B9D4 => {
    //   block [0x82E4B9D4..0x82E4B9E8)
	// 82E4B9D4: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B9D8: 7F98E378  mr r24, r28
	ctx.r[24].u64 = ctx.r[28].u64;
	// 82E4B9DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B9E0: 40990128  ble cr6, 0x82e4bb08
	if !ctx.cr[6].gt {
	pc = 0x82E4BB08; continue 'dispatch;
	}
	// 82E4B9E4: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	pc = 0x82E4B9E8; continue 'dispatch;
            }
            0x82E4B9E8 => {
    //   block [0x82E4B9E8..0x82E4BA2C)
	// 82E4B9E8: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82E4B9EC: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E4B9F0: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82E4B9F4: 92010070  stw r16, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[16].u32 ) };
	// 82E4B9F8: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B9FC: 7C7A582E  lwzx r3, r26, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BA00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BA04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4BA08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BA0C: 4E800421  bctrl
	ctx.lr = 0x82E4BA10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BA10: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E4BA14: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4BA18: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82E4BA1C: 7F96E378  mr r22, r28
	ctx.r[22].u64 = ctx.r[28].u64;
	// 82E4BA20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BA24: 409900A4  ble cr6, 0x82e4bac8
	if !ctx.cr[6].gt {
	pc = 0x82E4BAC8; continue 'dispatch;
	}
	// 82E4BA28: 7F97E378  mr r23, r28
	ctx.r[23].u64 = ctx.r[28].u64;
            }
            0x82E4BA2C => {
    //   block [0x82E4BA2C..0x82E4BA74)
	// 82E4BA2C: 7D57202E  lwzx r10, r23, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E4BA30: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4BA34: 409A0080  bne cr6, 0x82e4bab4
	if !ctx.cr[6].eq {
	pc = 0x82E4BAB4; continue 'dispatch;
	}
	// 82E4BA38: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BA3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4BA40: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82E4BA44: 7C9A582E  lwzx r4, r26, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BA48: 4BF37B41  bl 0x82d83588
	ctx.lr = 0x82E4BA4C;
	sub_82D83588(ctx, base);
	// 82E4BA4C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E4BA50: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4BA54: 4BF37385  bl 0x82d82dd8
	ctx.lr = 0x82E4BA58;
	sub_82D82DD8(ctx, base);
	// 82E4BA58: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E4BA5C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4BA60: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82E4BA64: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4BA68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BA6C: 40990048  ble cr6, 0x82e4bab4
	if !ctx.cr[6].gt {
	pc = 0x82E4BAB4; continue 'dispatch;
	}
	// 82E4BA70: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	pc = 0x82E4BA74; continue 'dispatch;
            }
            0x82E4BA74 => {
    //   block [0x82E4BA74..0x82E4BAA4)
	// 82E4BA74: 7D5F202E  lwzx r10, r31, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E4BA78: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4BA7C: 419A0028  beq cr6, 0x82e4baa4
	if ctx.cr[6].eq {
	pc = 0x82E4BAA4; continue 'dispatch;
	}
	// 82E4BA80: 5545003E  slwi r5, r10, 0
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4BA84: 894500D8  lbz r10, 0xd8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4BA88: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82E4BA8C: 419A0018  beq cr6, 0x82e4baa4
	if ctx.cr[6].eq {
	pc = 0x82E4BAA4; continue 'dispatch;
	}
	// 82E4BA90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4BA94: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4BA98: 4BFFF761  bl 0x82e4b1f8
	ctx.lr = 0x82E4BA9C;
	sub_82E4B1F8(ctx, base);
	// 82E4BA9C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E4BAA0: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	pc = 0x82E4BAA4; continue 'dispatch;
            }
            0x82E4BAA4 => {
    //   block [0x82E4BAA4..0x82E4BAB4)
	// 82E4BAA4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4BAA8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4BAAC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BAB0: 4198FFC4  blt cr6, 0x82e4ba74
	if ctx.cr[6].lt {
	pc = 0x82E4BA74; continue 'dispatch;
	}
	pc = 0x82E4BAB4; continue 'dispatch;
            }
            0x82E4BAB4 => {
    //   block [0x82E4BAB4..0x82E4BAC8)
	// 82E4BAB4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82E4BAB8: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 82E4BABC: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BAC0: 4198FF6C  blt cr6, 0x82e4ba2c
	if ctx.cr[6].lt {
	pc = 0x82E4BA2C; continue 'dispatch;
	}
	// 82E4BAC4: 82E10060  lwz r23, 0x60(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	pc = 0x82E4BAC8; continue 'dispatch;
            }
            0x82E4BAC8 => {
    //   block [0x82E4BAC8..0x82E4BADC)
	// 82E4BAC8: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82E4BACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BAD0: 409A000C  bne cr6, 0x82e4badc
	if !ctx.cr[6].eq {
	pc = 0x82E4BADC; continue 'dispatch;
	}
	// 82E4BAD4: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82E4BAD8: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	pc = 0x82E4BADC; continue 'dispatch;
            }
            0x82E4BADC => {
    //   block [0x82E4BADC..0x82E4BAFC)
	// 82E4BADC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E4BAE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4BAE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4BAE8: 409A0014  bne cr6, 0x82e4bafc
	if !ctx.cr[6].eq {
	pc = 0x82E4BAFC; continue 'dispatch;
	}
	// 82E4BAEC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4BAF0: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4BAF4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4BAF8: 4BF097D1  bl 0x82d552c8
	ctx.lr = 0x82E4BAFC;
	sub_82D552C8(ctx, base);
	pc = 0x82E4BAFC; continue 'dispatch;
            }
            0x82E4BAFC => {
    //   block [0x82E4BAFC..0x82E4BB08)
	// 82E4BAFC: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BB00: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BB04: 4198FEE4  blt cr6, 0x82e4b9e8
	if ctx.cr[6].lt {
	pc = 0x82E4B9E8; continue 'dispatch;
	}
	pc = 0x82E4BB08; continue 'dispatch;
            }
            0x82E4BB08 => {
    //   block [0x82E4BB08..0x82E4BB1C)
	// 82E4BB08: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BB0C: 39EF0001  addi r15, r15, 1
	ctx.r[15].s64 = ctx.r[15].s64 + 1;
	// 82E4BB10: 3A310004  addi r17, r17, 4
	ctx.r[17].s64 = ctx.r[17].s64 + 4;
	// 82E4BB14: 7F0F5800  cmpw cr6, r15, r11
	ctx.cr[6].compare_i32(ctx.r[15].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BB18: 4198FE18  blt cr6, 0x82e4b930
	if ctx.cr[6].lt {
	pc = 0x82E4B930; continue 'dispatch;
	}
	pc = 0x82E4BB1C; continue 'dispatch;
            }
            0x82E4BB1C => {
    //   block [0x82E4BB1C..0x82E4BB70)
	// 82E4BB1C: 568B063E  clrlwi r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	// 82E4BB20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB24: 409AFDF4  bne cr6, 0x82e4b918
	if !ctx.cr[6].eq {
	pc = 0x82E4B918; continue 'dispatch;
	}
	// 82E4BB28: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4BB2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BB30: 409A00A0  bne cr6, 0x82e4bbd0
	if !ctx.cr[6].eq {
	pc = 0x82E4BBD0; continue 'dispatch;
	}
	// 82E4BB34: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4BB38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BB3C: 409A0094  bne cr6, 0x82e4bbd0
	if !ctx.cr[6].eq {
	pc = 0x82E4BBD0; continue 'dispatch;
	}
	// 82E4BB40: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BB44: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BB48: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E4BB4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BB50: 419A0020  beq cr6, 0x82e4bb70
	if ctx.cr[6].eq {
	pc = 0x82E4BB70; continue 'dispatch;
	}
	// 82E4BB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BB58: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82E4BB5C: 4BF396FD  bl 0x82d85258
	ctx.lr = 0x82E4BB60;
	sub_82D85258(ctx, base);
	// 82E4BB60: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BB64: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E4BB68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB6C: 419A0008  beq cr6, 0x82e4bb74
	if ctx.cr[6].eq {
	pc = 0x82E4BB74; continue 'dispatch;
	}
	pc = 0x82E4BB70; continue 'dispatch;
            }
            0x82E4BB70 => {
    //   block [0x82E4BB70..0x82E4BB74)
	// 82E4BB70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	pc = 0x82E4BB74; continue 'dispatch;
            }
            0x82E4BB74 => {
    //   block [0x82E4BB74..0x82E4BB88)
	// 82E4BB74: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4BB78: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4BB7C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E4BB80: 419A0008  beq cr6, 0x82e4bb88
	if ctx.cr[6].eq {
	pc = 0x82E4BB88; continue 'dispatch;
	}
	// 82E4BB84: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	pc = 0x82E4BB88; continue 'dispatch;
            }
            0x82E4BB88 => {
    //   block [0x82E4BB88..0x82E4BBA4)
	// 82E4BB88: 89630040  lbz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E4BB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB90: 409A0014  bne cr6, 0x82e4bba4
	if !ctx.cr[6].eq {
	pc = 0x82E4BBA4; continue 'dispatch;
	}
	// 82E4BB94: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E4BB98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB9C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E4BBA0: 419A0008  beq cr6, 0x82e4bba8
	if ctx.cr[6].eq {
	pc = 0x82E4BBA8; continue 'dispatch;
	}
	pc = 0x82E4BBA4; continue 'dispatch;
            }
            0x82E4BBA4 => {
    //   block [0x82E4BBA4..0x82E4BBA8)
	// 82E4BBA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82E4BBA8; continue 'dispatch;
            }
            0x82E4BBA8 => {
    //   block [0x82E4BBA8..0x82E4BBD0)
	// 82E4BBA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BBAC: 99630040  stb r11, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E4BBB0: 4BF37839  bl 0x82d833e8
	ctx.lr = 0x82E4BBB4;
	sub_82D833E8(ctx, base);
	// 82E4BBB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BBB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BBBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4BBC0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BBC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BBC8: 4E800421  bctrl
	ctx.lr = 0x82E4BBCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BBCC: 48000048  b 0x82e4bc14
	pc = 0x82E4BC14; continue 'dispatch;
            }
            0x82E4BBD0 => {
    //   block [0x82E4BBD0..0x82E4BBF8)
	// 82E4BBD0: 8161012C  lwz r11, 0x12c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BBD4: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82E4BBD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BBDC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BBE0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4BBE4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BBE8: 409A0010  bne cr6, 0x82e4bbf8
	if !ctx.cr[6].eq {
	pc = 0x82E4BBF8; continue 'dispatch;
	}
	// 82E4BBEC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4BBF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BBF4: 4BF0B3A5  bl 0x82d56f98
	ctx.lr = 0x82E4BBF8;
	sub_82D56F98(ctx, base);
	pc = 0x82E4BBF8; continue 'dispatch;
            }
            0x82E4BBF8 => {
    //   block [0x82E4BBF8..0x82E4BC14)
	// 82E4BBF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BBFC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC00: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4BC04: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82E4BC08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BC0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4BC10: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82E4BC14; continue 'dispatch;
            }
            0x82E4BC14 => {
    //   block [0x82E4BC14..0x82E4BC24)
	// 82E4BC14: 3B790008  addi r27, r25, 8
	ctx.r[27].s64 = ctx.r[25].s64 + 8;
	// 82E4BC18: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BC1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC20: 4199FC90  bgt cr6, 0x82e4b8b0
	if ctx.cr[6].gt {
	pc = 0x82E4B8B0; continue 'dispatch;
	}
	pc = 0x82E4BC24; continue 'dispatch;
            }
            0x82E4BC24 => {
    //   block [0x82E4BC24..0x82E4BC40)
	// 82E4BC24: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4BC28: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC30: 419A0010  beq cr6, 0x82e4bc40
	if ctx.cr[6].eq {
	pc = 0x82E4BC40; continue 'dispatch;
	}
	// 82E4BC34: 83E1012C  lwz r31, 0x12c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BC38: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E4BC3C: 48000020  b 0x82e4bc5c
	pc = 0x82E4BC5C; continue 'dispatch;
            }
            0x82E4BC40 => {
    //   block [0x82E4BC40..0x82E4BC5C)
	// 82E4BC40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BC48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BC50: 4E800421  bctrl
	ctx.lr = 0x82E4BC54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BC54: 83E1012C  lwz r31, 0x12c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BC58: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
            }
            0x82E4BC5C => {
    //   block [0x82E4BC5C..0x82E4BC74)
	// 82E4BC5C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4BC60: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BC64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC68: 419A000C  beq cr6, 0x82e4bc74
	if ctx.cr[6].eq {
	pc = 0x82E4BC74; continue 'dispatch;
	}
	// 82E4BC6C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E4BC70: 4800001C  b 0x82e4bc8c
	pc = 0x82E4BC8C; continue 'dispatch;
            }
            0x82E4BC74 => {
    //   block [0x82E4BC74..0x82E4BC8C)
	// 82E4BC74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BC7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BC84: 4E800421  bctrl
	ctx.lr = 0x82E4BC88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BC88: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
            }
            0x82E4BC8C => {
    //   block [0x82E4BC8C..0x82E4BCE8)
	// 82E4BC8C: 3BB9002C  addi r29, r25, 0x2c
	ctx.r[29].s64 = ctx.r[25].s64 + 44;
	// 82E4BC90: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BC94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC98: 40990078  ble cr6, 0x82e4bd10
	if !ctx.cr[6].gt {
	pc = 0x82E4BD10; continue 'dispatch;
	}
	// 82E4BC9C: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4BCA0: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4BCA4: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4BCA8: 4BF095A1  bl 0x82d55248
	ctx.lr = 0x82E4BCAC;
	sub_82D55248(ctx, base);
	// 82E4BCAC: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 82E4BCB0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4BCB4: 4BF371B5  bl 0x82d82e68
	ctx.lr = 0x82E4BCB8;
	sub_82D82E68(ctx, base);
	// 82E4BCB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4BCBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4BCC0: 396B891C  addi r11, r11, -0x76e4
	ctx.r[11].s64 = ctx.r[11].s64 + -30436;
	// 82E4BCC4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4BCC8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4BCCC: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4BCD0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4BCD4: 8161012C  lwz r11, 0x12c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BCD8: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4BCDC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BCE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BCE4: 40990030  ble cr6, 0x82e4bd14
	if !ctx.cr[6].gt {
	pc = 0x82E4BD14; continue 'dispatch;
	}
	pc = 0x82E4BCE8; continue 'dispatch;
            }
            0x82E4BCE8 => {
    //   block [0x82E4BCE8..0x82E4BD10)
	// 82E4BCE8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BCF0: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BCF4: 4BF3777D  bl 0x82d83470
	ctx.lr = 0x82E4BCF8;
	sub_82D83470(ctx, base);
	// 82E4BCF8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BCFC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4BD00: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E4BD04: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BD08: 4198FFE0  blt cr6, 0x82e4bce8
	if ctx.cr[6].lt {
	pc = 0x82E4BCE8; continue 'dispatch;
	}
	// 82E4BD0C: 48000008  b 0x82e4bd14
	pc = 0x82E4BD14; continue 'dispatch;
            }
            0x82E4BD10 => {
    //   block [0x82E4BD10..0x82E4BD14)
	// 82E4BD10: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	pc = 0x82E4BD14; continue 'dispatch;
            }
            0x82E4BD14 => {
    //   block [0x82E4BD14..0x82E4BD34)
	// 82E4BD14: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BD18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BD1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4BD20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BD24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BD28: 4E800421  bctrl
	ctx.lr = 0x82E4BD2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BD2C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82E4BD30: 4BE5D6F0  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4BD38 size=80
    let mut pc: u32 = 0x82E4BD38;
    'dispatch: loop {
        match pc {
            0x82E4BD38 => {
    //   block [0x82E4BD38..0x82E4BD88)
	// 82E4BD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4BD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4BD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4BD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4BD48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4BD4C: 4BF3711D  bl 0x82d82e68
	ctx.lr = 0x82E4BD50;
	sub_82D82E68(ctx, base);
	// 82E4BD50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4BD54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4BD58: 396B6950  addi r11, r11, 0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + 26960;
	// 82E4BD5C: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E4BD60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BD64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4BD68: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82E4BD6C: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E4BD70: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82E4BD74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4BD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4BD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4BD80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4BD88 size=168
    let mut pc: u32 = 0x82E4BD88;
    'dispatch: loop {
        match pc {
            0x82E4BD88 => {
    //   block [0x82E4BD88..0x82E4BDB8)
	// 82E4BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4BD8C: 4BE5D681  bl 0x82ca940c
	ctx.lr = 0x82E4BD90;
	sub_82CA93D0(ctx, base);
	// 82E4BD90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4BD94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4BD98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4BD9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4BDA0: 396B71E4  addi r11, r11, 0x71e4
	ctx.r[11].s64 = ctx.r[11].s64 + 29156;
	// 82E4BDA4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BDA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4BDAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4BDB0: 40990040  ble cr6, 0x82e4bdf0
	if !ctx.cr[6].gt {
	pc = 0x82E4BDF0; continue 'dispatch;
	}
	// 82E4BDB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82E4BDB8; continue 'dispatch;
            }
            0x82E4BDB8 => {
    //   block [0x82E4BDB8..0x82E4BDDC)
	// 82E4BDB8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BDBC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4BDC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4BDC4: 419A0018  beq cr6, 0x82e4bddc
	if ctx.cr[6].eq {
	pc = 0x82E4BDDC; continue 'dispatch;
	}
	// 82E4BDC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BDCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BDD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BDD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BDD8: 4E800421  bctrl
	ctx.lr = 0x82E4BDDC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4BDDC => {
    //   block [0x82E4BDDC..0x82E4BDF0)
	// 82E4BDDC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BDE0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4BDE4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4BDE8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BDEC: 4198FFCC  blt cr6, 0x82e4bdb8
	if ctx.cr[6].lt {
	pc = 0x82E4BDB8; continue 'dispatch;
	}
	pc = 0x82E4BDF0; continue 'dispatch;
            }
            0x82E4BDF0 => {
    //   block [0x82E4BDF0..0x82E4BE1C)
	// 82E4BDF0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4BDF4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4BDF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4BDFC: 409A0020  bne cr6, 0x82e4be1c
	if !ctx.cr[6].eq {
	pc = 0x82E4BE1C; continue 'dispatch;
	}
	// 82E4BE00: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE04: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4BE08: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4BE0C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BE10: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4BE14: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4BE18: 4BF094B1  bl 0x82d552c8
	ctx.lr = 0x82E4BE1C;
	sub_82D552C8(ctx, base);
	pc = 0x82E4BE1C; continue 'dispatch;
            }
            0x82E4BE1C => {
    //   block [0x82E4BE1C..0x82E4BE30)
	// 82E4BE1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4BE20: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4BE24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4BE28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4BE2C: 4BE5D630  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4BE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4BE30 size=632
    let mut pc: u32 = 0x82E4BE30;
    'dispatch: loop {
        match pc {
            0x82E4BE30 => {
    //   block [0x82E4BE30..0x82E4BED0)
	// 82E4BE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4BE34: 4BE5D5C9  bl 0x82ca93fc
	ctx.lr = 0x82E4BE38;
	sub_82CA93D0(ctx, base);
	// 82E4BE38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4BE3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4BE40: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BE44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E4BE48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4BE4C: 388BA314  addi r4, r11, -0x5cec
	ctx.r[4].s64 = ctx.r[11].s64 + -23788;
	// 82E4BE50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4BE54: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE58: 4BF09939  bl 0x82d55790
	ctx.lr = 0x82E4BE5C;
	sub_82D55790(ctx, base);
	// 82E4BE5C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE60: 3B200070  li r25, 0x70
	ctx.r[25].s64 = 112;
	// 82E4BE64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BE68: 419A008C  beq cr6, 0x82e4bef4
	if ctx.cr[6].eq {
	pc = 0x82E4BEF4; continue 'dispatch;
	}
	// 82E4BE6C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BE70: 3B4B9DD4  addi r26, r11, -0x622c
	ctx.r[26].s64 = ctx.r[11].s64 + -25132;
	// 82E4BE74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE78: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E4BE7C: 419A0078  beq cr6, 0x82e4bef4
	if ctx.cr[6].eq {
	pc = 0x82E4BEF4; continue 'dispatch;
	}
	// 82E4BE80: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE84: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4BE88: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82E4BE8C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE90: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E4BE94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BE98: 4BF093B1  bl 0x82d55248
	ctx.lr = 0x82E4BE9C;
	sub_82D55248(ctx, base);
	// 82E4BE9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BEA0: B3230004  sth r25, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u16 ) };
	// 82E4BEA4: 4BF796A5  bl 0x82dc5548
	ctx.lr = 0x82E4BEA8;
	sub_82DC5548(ctx, base);
	// 82E4BEA8: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82E4BEAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4BEB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BEB4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BEB8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4BEBC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BEC0: 409A0010  bne cr6, 0x82e4bed0
	if !ctx.cr[6].eq {
	pc = 0x82E4BED0; continue 'dispatch;
	}
	// 82E4BEC4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4BEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BECC: 4BF0B0CD  bl 0x82d56f98
	ctx.lr = 0x82E4BED0;
	sub_82D56F98(ctx, base);
	pc = 0x82E4BED0; continue 'dispatch;
            }
            0x82E4BED0 => {
    //   block [0x82E4BED0..0x82E4BEF4)
	// 82E4BED0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BED4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BED8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4BEDC: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E4BEE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BEE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4BEE8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4BEEC: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4BEF0: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	pc = 0x82E4BEF4; continue 'dispatch;
            }
            0x82E4BEF4 => {
    //   block [0x82E4BEF4..0x82E4BFA8)
	// 82E4BEF4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BEF8: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BEFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4BF00: 388BA114  addi r4, r11, -0x5eec
	ctx.r[4].s64 = ctx.r[11].s64 + -24300;
	// 82E4BF04: 4BF0988D  bl 0x82d55790
	ctx.lr = 0x82E4BF08;
	sub_82D55790(ctx, base);
	// 82E4BF08: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BF10: 419A0098  beq cr6, 0x82e4bfa8
	if ctx.cr[6].eq {
	pc = 0x82E4BFA8; continue 'dispatch;
	}
	// 82E4BF14: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BF18: 3B4B9D24  addi r26, r11, -0x62dc
	ctx.r[26].s64 = ctx.r[11].s64 + -25308;
	// 82E4BF1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF20: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E4BF24: 419A0084  beq cr6, 0x82e4bfa8
	if ctx.cr[6].eq {
	pc = 0x82E4BFA8; continue 'dispatch;
	}
	// 82E4BF28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF2C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4BF30: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82E4BF34: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF38: 38800130  li r4, 0x130
	ctx.r[4].s64 = 304;
	// 82E4BF3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BF40: 4BF09309  bl 0x82d55248
	ctx.lr = 0x82E4BF44;
	sub_82D55248(ctx, base);
	// 82E4BF44: 39600130  li r11, 0x130
	ctx.r[11].s64 = 304;
	// 82E4BF48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BF4C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4BF50: 4BF77D79  bl 0x82dc3cc8
	ctx.lr = 0x82E4BF54;
	sub_82DC3CC8(ctx, base);
	// 82E4BF54: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82E4BF58: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4BF5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BF60: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BF64: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4BF68: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BF6C: 409A00C0  bne cr6, 0x82e4c02c
	if !ctx.cr[6].eq {
	pc = 0x82E4C02C; continue 'dispatch;
	}
	// 82E4BF70: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4BF74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BF78: 4BF0B021  bl 0x82d56f98
	ctx.lr = 0x82E4BF7C;
	sub_82D56F98(ctx, base);
	// 82E4BF7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BF80: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF84: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4BF88: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E4BF8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BF90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4BF94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4BF98: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4BF9C: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E4BFA0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4BFA4: 4BE5D4A8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4BFA8 => {
    //   block [0x82E4BFA8..0x82E4C02C)
	// 82E4BFA8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BFAC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4BFB4: 388BA854  addi r4, r11, -0x57ac
	ctx.r[4].s64 = ctx.r[11].s64 + -22444;
	// 82E4BFB8: 4BF097D9  bl 0x82d55790
	ctx.lr = 0x82E4BFBC;
	sub_82D55790(ctx, base);
	// 82E4BFBC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BFC4: 419A0094  beq cr6, 0x82e4c058
	if ctx.cr[6].eq {
	pc = 0x82E4C058; continue 'dispatch;
	}
	// 82E4BFC8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BFCC: 3B4B9E04  addi r26, r11, -0x61fc
	ctx.r[26].s64 = ctx.r[11].s64 + -25084;
	// 82E4BFD0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFD4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E4BFD8: 419A0080  beq cr6, 0x82e4c058
	if ctx.cr[6].eq {
	pc = 0x82E4C058; continue 'dispatch;
	}
	// 82E4BFDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFE0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4BFE4: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82E4BFE8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFEC: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E4BFF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BFF4: 4BF09255  bl 0x82d55248
	ctx.lr = 0x82E4BFF8;
	sub_82D55248(ctx, base);
	// 82E4BFF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BFFC: B3230004  sth r25, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u16 ) };
	// 82E4C000: 48294901  bl 0x830e0900
	ctx.lr = 0x82E4C004;
	sub_830E0900(ctx, base);
	// 82E4C004: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82E4C008: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4C00C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4C010: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C014: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4C018: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4C01C: 409A0010  bne cr6, 0x82e4c02c
	if !ctx.cr[6].eq {
	pc = 0x82E4C02C; continue 'dispatch;
	}
	// 82E4C020: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4C024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C028: 4BF0AF71  bl 0x82d56f98
	ctx.lr = 0x82E4C02C;
	sub_82D56F98(ctx, base);
	pc = 0x82E4C02C; continue 'dispatch;
            }
            0x82E4C02C => {
    //   block [0x82E4C02C..0x82E4C058)
	// 82E4C02C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C030: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C034: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4C038: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E4C03C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C040: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4C044: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4C048: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4C04C: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E4C050: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C054: 4BE5D3F8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4C058 => {
    //   block [0x82E4C058..0x82E4C0A0)
	// 82E4C058: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4C05C: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4C064: 388B8FF0  addi r4, r11, -0x7010
	ctx.r[4].s64 = ctx.r[11].s64 + -28688;
	// 82E4C068: 4BF09729  bl 0x82d55790
	ctx.lr = 0x82E4C06C;
	sub_82D55790(ctx, base);
	// 82E4C06C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C074: 419A002C  beq cr6, 0x82e4c0a0
	if ctx.cr[6].eq {
	pc = 0x82E4C0A0; continue 'dispatch;
	}
	// 82E4C078: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C07C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4C080: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4C084: 419A001C  beq cr6, 0x82e4c0a0
	if ctx.cr[6].eq {
	pc = 0x82E4C0A0; continue 'dispatch;
	}
	// 82E4C088: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4C08C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E4C090: 409A0010  bne cr6, 0x82e4c0a0
	if !ctx.cr[6].eq {
	pc = 0x82E4C0A0; continue 'dispatch;
	}
	// 82E4C094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4C098: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4C09C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82E4C0A0; continue 'dispatch;
            }
            0x82E4C0A0 => {
    //   block [0x82E4C0A0..0x82E4C0A8)
	// 82E4C0A0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C0A4: 4BE5D3A8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C0A8 size=408
    let mut pc: u32 = 0x82E4C0A8;
    'dispatch: loop {
        match pc {
            0x82E4C0A8 => {
    //   block [0x82E4C0A8..0x82E4C10C)
	// 82E4C0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C0AC: 4BE5D349  bl 0x82ca93f4
	ctx.lr = 0x82E4C0B0;
	sub_82CA93D0(ctx, base);
	// 82E4C0B0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C0B4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82E4C0B8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82E4C0BC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E4C0C0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E4C0C4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E4C0C8: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82E4C0CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E4C0D0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E4C0D4: 419A015C  beq cr6, 0x82e4c230
	if ctx.cr[6].eq {
	pc = 0x82E4C230; continue 'dispatch;
	}
	// 82E4C0D8: 7CEB0774  extsb r11, r7
	ctx.r[11].s64 = ctx.r[7].s8 as i64;
	// 82E4C0DC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4C0E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4C0E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C0E8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82E4C0EC: 388000A8  li r4, 0xa8
	ctx.r[4].s64 = 168;
	// 82E4C0F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C0F4: 419A0018  beq cr6, 0x82e4c10c
	if ctx.cr[6].eq {
	pc = 0x82E4C10C; continue 'dispatch;
	}
	// 82E4C0F8: 4BF09151  bl 0x82d55248
	ctx.lr = 0x82E4C0FC;
	sub_82D55248(ctx, base);
	// 82E4C0FC: 396000A8  li r11, 0xa8
	ctx.r[11].s64 = 168;
	// 82E4C100: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4C104: 4828944D  bl 0x830d5550
	ctx.lr = 0x82E4C108;
	sub_830D5550(ctx, base);
	// 82E4C108: 48000014  b 0x82e4c11c
	pc = 0x82E4C11C; continue 'dispatch;
            }
            0x82E4C10C => {
    //   block [0x82E4C10C..0x82E4C11C)
	// 82E4C10C: 4BF0913D  bl 0x82d55248
	ctx.lr = 0x82E4C110;
	sub_82D55248(ctx, base);
	// 82E4C110: 396000A8  li r11, 0xa8
	ctx.r[11].s64 = 168;
	// 82E4C114: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4C118: 48288E69  bl 0x830d4f80
	ctx.lr = 0x82E4C11C;
	sub_830D4F80(ctx, base);
	pc = 0x82E4C11C; continue 'dispatch;
            }
            0x82E4C11C => {
    //   block [0x82E4C11C..0x82E4C154)
	// 82E4C11C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C120: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82E4C124: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82E4C128: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82E4C12C: 396B71E4  addi r11, r11, 0x71e4
	ctx.r[11].s64 = ctx.r[11].s64 + 29156;
	// 82E4C130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C134: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E4C138: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E4C13C: B3810076  sth r28, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[28].u16 ) };
	// 82E4C140: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E4C144: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4C148: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E4C14C: 409A0008  bne cr6, 0x82e4c154
	if !ctx.cr[6].eq {
	pc = 0x82E4C154; continue 'dispatch;
	}
	// 82E4C150: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	pc = 0x82E4C154; continue 'dispatch;
            }
            0x82E4C154 => {
    //   block [0x82E4C154..0x82E4C1C8)
	// 82E4C154: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C158: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82E4C15C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E4C160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C164: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4C168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4C16C: 4E800421  bctrl
	ctx.lr = 0x82E4C170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4C170: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E4C174: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E4C178: 4BF254E1  bl 0x82d71658
	ctx.lr = 0x82E4C17C;
	sub_82D71658(ctx, base);
	// 82E4C17C: 39600073  li r11, 0x73
	ctx.r[11].s64 = 115;
	// 82E4C180: 9B810068  stb r28, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u8 ) };
	// 82E4C184: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E4C188: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E4C18C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82E4C190: 3960006E  li r11, 0x6e
	ctx.r[11].s64 = 110;
	// 82E4C194: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82E4C198: 39600061  li r11, 0x61
	ctx.r[11].s64 = 97;
	// 82E4C19C: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 82E4C1A0: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82E4C1A4: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 82E4C1A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4C1AC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E4C1B0: 419A0018  beq cr6, 0x82e4c1c8
	if ctx.cr[6].eq {
	pc = 0x82E4C1C8; continue 'dispatch;
	}
	// 82E4C1B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E4C1B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4C1BC: 4BF254AD  bl 0x82d71668
	ctx.lr = 0x82E4C1C0;
	sub_82D71668(ctx, base);
	// 82E4C1C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C1C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
            }
            0x82E4C1C8 => {
    //   block [0x82E4C1C8..0x82E4C228)
	// 82E4C1C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C1CC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4C1D0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E4C1D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C1D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E4C1DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4C1E0: 4E800421  bctrl
	ctx.lr = 0x82E4C1E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4C1E4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C1E8: 7C6A0034  cntlzw r10, r3
	ctx.r[10].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E4C1EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C1F0: 555EDFFE  rlwinm r30, r10, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E4C1F4: 419A0034  beq cr6, 0x82e4c228
	if ctx.cr[6].eq {
	pc = 0x82E4C228; continue 'dispatch;
	}
	// 82E4C1F8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4C1FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4C200: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4C204: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4C208: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4C20C: 409A001C  bne cr6, 0x82e4c228
	if !ctx.cr[6].eq {
	pc = 0x82E4C228; continue 'dispatch;
	}
	// 82E4C210: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C214: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4C218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C21C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4C224: 4E800421  bctrl
	ctx.lr = 0x82E4C228;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4C228 => {
    //   block [0x82E4C228..0x82E4C230)
	// 82E4C228: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4C22C: 4BFFFB5D  bl 0x82e4bd88
	ctx.lr = 0x82E4C230;
	sub_82E4BD88(ctx, base);
	pc = 0x82E4C230; continue 'dispatch;
            }
            0x82E4C230 => {
    //   block [0x82E4C230..0x82E4C240)
	// 82E4C230: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E4C234: 9BD70000  stb r30, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E4C238: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E4C23C: 4BE5D208  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C240 size=308
    let mut pc: u32 = 0x82E4C240;
    'dispatch: loop {
        match pc {
            0x82E4C240 => {
    //   block [0x82E4C240..0x82E4C2A8)
	// 82E4C240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C244: 4BE5D1C5  bl 0x82ca9408
	ctx.lr = 0x82E4C248;
	sub_82CA93D0(ctx, base);
	// 82E4C248: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C24C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4C250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4C254: 419A0054  beq cr6, 0x82e4c2a8
	if ctx.cr[6].eq {
	pc = 0x82E4C2A8; continue 'dispatch;
	}
	// 82E4C258: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E4C25C: 419A004C  beq cr6, 0x82e4c2a8
	if ctx.cr[6].eq {
	pc = 0x82E4C2A8; continue 'dispatch;
	}
	// 82E4C260: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C264: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4C268: 396B71F8  addi r11, r11, 0x71f8
	ctx.r[11].s64 = ctx.r[11].s64 + 29176;
	// 82E4C26C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4C270: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C274: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C278: B1410066  sth r10, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[10].u16 ) };
	// 82E4C27C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E4C280: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4C284: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E4C288: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82E4C28C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E4C290: 48288B89  bl 0x830d4e18
	ctx.lr = 0x82E4C294;
	sub_830D4E18(ctx, base);
	// 82E4C294: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4C298: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E4C29C: 409A0018  bne cr6, 0x82e4c2b4
	if !ctx.cr[6].eq {
	pc = 0x82E4C2B4; continue 'dispatch;
	}
	// 82E4C2A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C2A4: 482887AD  bl 0x830d4a50
	ctx.lr = 0x82E4C2A8;
	sub_830D4A50(ctx, base);
	pc = 0x82E4C2A8; continue 'dispatch;
            }
            0x82E4C2A8 => {
    //   block [0x82E4C2A8..0x82E4C2B4)
	// 82E4C2A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4C2AC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C2B0: 4BE5D1A8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4C2B4 => {
    //   block [0x82E4C2B4..0x82E4C2F8)
	// 82E4C2B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4C2B8: 386BB078  addi r3, r11, -0x4f88
	ctx.r[3].s64 = ctx.r[11].s64 + -20360;
	// 82E4C2BC: 4BF09495  bl 0x82d55750
	ctx.lr = 0x82E4C2C0;
	sub_82D55750(ctx, base);
	// 82E4C2C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C2C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4C2C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4C2CC: 4BF2520D  bl 0x82d714d8
	ctx.lr = 0x82E4C2D0;
	sub_82D714D8(ctx, base);
	// 82E4C2D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4C2D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4C2D8: 409A0064  bne cr6, 0x82e4c33c
	if !ctx.cr[6].eq {
	pc = 0x82E4C33C; continue 'dispatch;
	}
	// 82E4C2DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C2E0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E4C2E4: 388B7204  addi r4, r11, 0x7204
	ctx.r[4].s64 = ctx.r[11].s64 + 29188;
	// 82E4C2E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C2EC: 396B5D2C  addi r11, r11, 0x5d2c
	ctx.r[11].s64 = ctx.r[11].s64 + 23852;
	// 82E4C2F0: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82E4C2F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x82E4C2F8; continue 'dispatch;
            }
            0x82E4C2F8 => {
    //   block [0x82E4C2F8..0x82E4C33C)
	// 82E4C2F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4C2FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4C300: 4BF252A9  bl 0x82d715a8
	ctx.lr = 0x82E4C304;
	sub_82D715A8(ctx, base);
	// 82E4C304: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4C308: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4C30C: 409A0030  bne cr6, 0x82e4c33c
	if !ctx.cr[6].eq {
	pc = 0x82E4C33C; continue 'dispatch;
	}
	// 82E4C310: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E4C314: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4C318: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4C31C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C320: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E4C324: 409AFFD4  bne cr6, 0x82e4c2f8
	if !ctx.cr[6].eq {
	pc = 0x82E4C2F8; continue 'dispatch;
	}
	// 82E4C328: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C32C: 48288725  bl 0x830d4a50
	ctx.lr = 0x82E4C330;
	sub_830D4A50(ctx, base);
	// 82E4C330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C334: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C338: 4BE5D120  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4C33C => {
    //   block [0x82E4C33C..0x82E4C360)
	// 82E4C33C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4C340: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C344: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4C348: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C34C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4C350: 419A0010  beq cr6, 0x82e4c360
	if ctx.cr[6].eq {
	pc = 0x82E4C360; continue 'dispatch;
	}
	// 82E4C354: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4C358: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4C35C: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82E4C360; continue 'dispatch;
            }
            0x82E4C360 => {
    //   block [0x82E4C360..0x82E4C374)
	// 82E4C360: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C364: 482886ED  bl 0x830d4a50
	ctx.lr = 0x82E4C368;
	sub_830D4A50(ctx, base);
	// 82E4C368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C36C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C370: 4BE5D0E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C378 size=128
    let mut pc: u32 = 0x82E4C378;
    'dispatch: loop {
        match pc {
            0x82E4C378 => {
    //   block [0x82E4C378..0x82E4C3F8)
	// 82E4C378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C37C: 4BE5D085  bl 0x82ca9400
	ctx.lr = 0x82E4C380;
	sub_82CA93D0(ctx, base);
	// 82E4C380: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C384: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E4C388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C390: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4C394: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E4C398: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E4C39C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E4C3A0: 4BF093B1  bl 0x82d55750
	ctx.lr = 0x82E4C3A4;
	sub_82D55750(ctx, base);
	// 82E4C3A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C3A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C3AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4C3B0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4C3B4: 4BF25065  bl 0x82d71418
	ctx.lr = 0x82E4C3B8;
	sub_82D71418(ctx, base);
	// 82E4C3B8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E4C3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E4C3C0: 38AB7BC4  addi r5, r11, 0x7bc4
	ctx.r[5].s64 = ctx.r[11].s64 + 31684;
	// 82E4C3C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4C3C8: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E4C3CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4C3D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4C3D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4C3D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4C3DC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E4C3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C3E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4C3E8: 4BFFFCC1  bl 0x82e4c0a8
	ctx.lr = 0x82E4C3EC;
	sub_82E4C0A8(ctx, base);
	// 82E4C3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C3F0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C3F4: 4BE5D05C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C3F8 size=132
    let mut pc: u32 = 0x82E4C3F8;
    'dispatch: loop {
        match pc {
            0x82E4C3F8 => {
    //   block [0x82E4C3F8..0x82E4C47C)
	// 82E4C3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C3FC: 4BE5D005  bl 0x82ca9400
	ctx.lr = 0x82E4C400;
	sub_82CA93D0(ctx, base);
	// 82E4C400: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C404: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4C408: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C40C: 3BCBB078  addi r30, r11, -0x4f88
	ctx.r[30].s64 = ctx.r[11].s64 + -20360;
	// 82E4C410: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4C414: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C418: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E4C41C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E4C420: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E4C424: 4BF0932D  bl 0x82d55750
	ctx.lr = 0x82E4C428;
	sub_82D55750(ctx, base);
	// 82E4C428: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C42C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C430: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4C434: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4C438: 4BF24FE1  bl 0x82d71418
	ctx.lr = 0x82E4C43C;
	sub_82D71418(ctx, base);
	// 82E4C43C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E4C440: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E4C444: 38AB7BC4  addi r5, r11, 0x7bc4
	ctx.r[5].s64 = ctx.r[11].s64 + 31684;
	// 82E4C448: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4C44C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E4C450: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4C454: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4C458: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4C45C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4C460: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E4C464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C468: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4C46C: 4BFFFC3D  bl 0x82e4c0a8
	ctx.lr = 0x82E4C470;
	sub_82E4C0A8(ctx, base);
	// 82E4C470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C474: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C478: 4BE5CFD8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C480 size=152
    let mut pc: u32 = 0x82E4C480;
    'dispatch: loop {
        match pc {
            0x82E4C480 => {
    //   block [0x82E4C480..0x82E4C504)
	// 82E4C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C484: 4BE5CF7D  bl 0x82ca9400
	ctx.lr = 0x82E4C488;
	sub_82CA93D0(ctx, base);
	// 82E4C488: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C48C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4C490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C494: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4C498: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E4C49C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E4C4A0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E4C4A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4C4A8: 419A005C  beq cr6, 0x82e4c504
	if ctx.cr[6].eq {
	pc = 0x82E4C504; continue 'dispatch;
	}
	// 82E4C4AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E4C4B0: 419A0054  beq cr6, 0x82e4c504
	if ctx.cr[6].eq {
	pc = 0x82E4C504; continue 'dispatch;
	}
	// 82E4C4B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C4B8: 4BFFEF51  bl 0x82e4b408
	ctx.lr = 0x82E4C4BC;
	sub_82E4B408(ctx, base);
	// 82E4C4BC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E4C4C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4C4C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C4C8: 4BFFF079  bl 0x82e4b540
	ctx.lr = 0x82E4C4CC;
	sub_82E4B540(ctx, base);
	// 82E4C4CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4C4D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4C4D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4C4D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E4C4DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4C4E0: 4BFFFF19  bl 0x82e4c3f8
	ctx.lr = 0x82E4C4E4;
	sub_82E4C3F8(ctx, base);
	// 82E4C4E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4C4E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C4EC: 8BCB0000  lbz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C4F0: 4BFFEF49  bl 0x82e4b438
	ctx.lr = 0x82E4C4F4;
	sub_82E4B438(ctx, base);
	// 82E4C4F4: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E4C4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C4FC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C500: 4BE5CF50  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4C504 => {
    //   block [0x82E4C504..0x82E4C518)
	// 82E4C504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4C508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C50C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E4C510: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C514: 4BE5CF3C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C518 size=100
    let mut pc: u32 = 0x82E4C518;
    'dispatch: loop {
        match pc {
            0x82E4C518 => {
    //   block [0x82E4C518..0x82E4C560)
	// 82E4C518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4C520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4C524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4C528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C52C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C530: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4C534: 4BFFF855  bl 0x82e4bd88
	ctx.lr = 0x82E4C538;
	sub_82E4BD88(ctx, base);
	// 82E4C538: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4C53C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C540: 419A0020  beq cr6, 0x82e4c560
	if ctx.cr[6].eq {
	pc = 0x82E4C560; continue 'dispatch;
	}
	// 82E4C544: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C548: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4C54C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82E4C550: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C554: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4C558: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C55C: 4BF08D6D  bl 0x82d552c8
	ctx.lr = 0x82E4C560;
	sub_82D552C8(ctx, base);
	pc = 0x82E4C560; continue 'dispatch;
            }
            0x82E4C560 => {
    //   block [0x82E4C560..0x82E4C57C)
	// 82E4C560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4C568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4C56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4C570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4C574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4C578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C580 size=100
    let mut pc: u32 = 0x82E4C580;
    'dispatch: loop {
        match pc {
            0x82E4C580 => {
    //   block [0x82E4C580..0x82E4C5C8)
	// 82E4C580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4C588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4C58C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4C590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4C59C: 482884B5  bl 0x830d4a50
	ctx.lr = 0x82E4C5A0;
	sub_830D4A50(ctx, base);
	// 82E4C5A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4C5A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C5A8: 419A0020  beq cr6, 0x82e4c5c8
	if ctx.cr[6].eq {
	pc = 0x82E4C5C8; continue 'dispatch;
	}
	// 82E4C5AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C5B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4C5B4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82E4C5B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C5BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4C5C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C5C4: 4BF08D05  bl 0x82d552c8
	ctx.lr = 0x82E4C5C8;
	sub_82D552C8(ctx, base);
	pc = 0x82E4C5C8; continue 'dispatch;
            }
            0x82E4C5C8 => {
    //   block [0x82E4C5C8..0x82E4C5E4)
	// 82E4C5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C5CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4C5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4C5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4C5D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4C5DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4C5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C5E8 size=24
    let mut pc: u32 = 0x82E4C5E8;
    'dispatch: loop {
        match pc {
            0x82E4C5E8 => {
    //   block [0x82E4C5E8..0x82E4C600)
	// 82E4C5E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4C5EC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C5F0: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82E4C5F4: 4199000C  bgt cr6, 0x82e4c600
	if ctx.cr[6].gt {
		sub_82E4C600(ctx, base);
		return;
	}
	// 82E4C5F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4C5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C600 size=144
    let mut pc: u32 = 0x82E4C600;
    'dispatch: loop {
        match pc {
            0x82E4C600 => {
    //   block [0x82E4C600..0x82E4C690)
	// 82E4C600: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E4C604: 4099FFF4  ble cr6, 0x82e4c5f8
	if !ctx.cr[6].gt {
		sub_82E4C5E8(ctx, base);
		return;
	}
	// 82E4C608: EDA10072  fmuls f13, f1, f1
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C60C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4C610: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E4C614: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 82E4C618: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E4C61C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E4C620: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C624: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C690 size=24
    let mut pc: u32 = 0x82E4C690;
    'dispatch: loop {
        match pc {
            0x82E4C690 => {
    //   block [0x82E4C690..0x82E4C6A8)
	// 82E4C690: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4C694: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C698: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E4C69C: 4199000C  bgt cr6, 0x82e4c6a8
	if ctx.cr[6].gt {
		sub_82E4C6A8(ctx, base);
		return;
	}
	// 82E4C6A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4C6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C6A8 size=180
    let mut pc: u32 = 0x82E4C6A8;
    'dispatch: loop {
        match pc {
            0x82E4C6A8 => {
    //   block [0x82E4C6A8..0x82E4C75C)
	// 82E4C6A8: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C6AC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4C6B0: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C6B4: ED0D0372  fmuls f8, f13, f13
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C6B8: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C6BC: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E4C6C0: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C6C4: ECEC0332  fmuls f7, f12, f12
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4C6C8: ED200032  fmuls f9, f0, f0
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4C6CC: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 82E4C6D0: C1490C14  lfs f10, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E4C6D4: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82E4C6D8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C760 size=72
    let mut pc: u32 = 0x82E4C760;
    'dispatch: loop {
        match pc {
            0x82E4C760 => {
    //   block [0x82E4C760..0x82E4C7A8)
	// 82E4C760: C1A40050  lfs f13, 0x50(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C764: C1840064  lfs f12, 0x64(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C768: EC0D6028  fsubs f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C76C: C1640078  lfs f11, 0x78(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E4C770: FC00636E  fsel f0, f0, f13, f12
	ctx.f[0].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 82E4C774: ED405828  fsubs f10, f0, f11
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E4C778: FC0A582E  fsel f0, f10, f0, f11
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[11].f64 };
	// 82E4C77C: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82E4C780: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C784: ED2C0028  fsubs f9, f12, f0
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C788: ED0B0028  fsubs f8, f11, f0
	ctx.f[8].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C78C: FDAA036E  fsel f13, f10, f13, f0
	ctx.f[13].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82E4C790: D1A40050  stfs f13, 0x50(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4C794: FDA9032E  fsel f13, f9, f12, f0
	ctx.f[13].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82E4C798: D1A40064  stfs f13, 0x64(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E4C79C: FC0802EE  fsel f0, f8, f11, f0
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[0].f64 };
	// 82E4C7A0: D0040078  stfs f0, 0x78(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E4C7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C7A8 size=172
    let mut pc: u32 = 0x82E4C7A8;
    'dispatch: loop {
        match pc {
            0x82E4C7A8 => {
    //   block [0x82E4C7A8..0x82E4C854)
	// 82E4C7A8: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C7AC: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C7B0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C7B4: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C7B8: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C7BC: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C7C0: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4C7C4: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C7C8: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C7CC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C7D0: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C7D4: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C7D8: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C7DC: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4C7E0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C7E4: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C7E8: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C7EC: C1850028  lfs f12, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C7F0: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C7F4: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C7F8: D0050028  stfs f0, 0x28(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4C7FC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C800: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C804: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C808: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C80C: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C810: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4C814: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4C818: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C81C: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4C820: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C824: C1850018  lfs f12, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C828: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C82C: D0050018  stfs f0, 0x18(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4C830: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4C834: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C838: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C83C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C840: C1850020  lfs f12, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C844: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C848: D0050020  stfs f0, 0x20(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4C84C: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4C850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C858 size=172
    let mut pc: u32 = 0x82E4C858;
    'dispatch: loop {
        match pc {
            0x82E4C858 => {
    //   block [0x82E4C858..0x82E4C904)
	// 82E4C858: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C85C: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C860: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C864: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C868: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C86C: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C870: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4C874: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C878: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C87C: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C880: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C884: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C888: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C88C: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4C890: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C894: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C898: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C89C: C1850028  lfs f12, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8A0: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C8A4: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C8A8: D0050028  stfs f0, 0x28(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4C8AC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C8B0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C8B4: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C8B8: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8BC: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C8C0: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4C8C4: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4C8C8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C8CC: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4C8D0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C8D4: C1850018  lfs f12, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8D8: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C8DC: D0050018  stfs f0, 0x18(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4C8E0: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4C8E4: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C8E8: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C8EC: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C8F0: C1850020  lfs f12, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8F4: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C8F8: D0050020  stfs f0, 0x20(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4C8FC: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4C900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4C908 size=80
    let mut pc: u32 = 0x82E4C908;
    'dispatch: loop {
        match pc {
            0x82E4C908 => {
    //   block [0x82E4C908..0x82E4C958)
	// 82E4C908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4C910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C914: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E4C918: D0210084  stfs f1, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E4C91C: 39410084  addi r10, r1, 0x84
	ctx.r[10].s64 = ctx.r[1].s64 + 132;
	// 82E4C920: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E4C924: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4C928: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C958 size=76
    let mut pc: u32 = 0x82E4C958;
    'dispatch: loop {
        match pc {
            0x82E4C958 => {
    //   block [0x82E4C958..0x82E4C9A4)
	// 82E4C958: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C95C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82E4C960: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C964: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82E4C968: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C96C: C1830028  lfs f12, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C9A8 size=688
    let mut pc: u32 = 0x82E4C9A8;
    'dispatch: loop {
        match pc {
            0x82E4C9A8 => {
    //   block [0x82E4C9A8..0x82E4CA28)
	// 82E4C9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C9AC: 4BE5CA61  bl 0x82ca940c
	ctx.lr = 0x82E4C9B0;
	sub_82CA93D0(ctx, base);
	// 82E4C9B0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4C9B4: 4BE612F1  bl 0x82cadca4
	ctx.lr = 0x82E4C9B8;
	sub_82CADCA0(ctx, base);
	// 82E4C9B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4C9BC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C9C0: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C9C4: 3FA08200  lis r29, -0x7e00
	ctx.r[29].s64 = -2113929216;
	// 82E4C9C8: 3CC08204  lis r6, -0x7dfc
	ctx.r[6].s64 = -2113667072;
	// 82E4C9CC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82E4C9D0: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4C9D4: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E4C9D8: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C9DC: 3CA05555  lis r5, 0x5555
	ctx.r[5].s64 = 1431633920;
	// 82E4C9E0: C0BD0A4C  lfs f5, 0xa4c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2636 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4C9E4: C0C6BDF4  lfs f6, -0x420c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-16908 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E4C9E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4C9EC: C0270C4C  lfs f1, 0xc4c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E4C9F0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E4C9F4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E4C9F8: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82E4C9FC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E4CA00: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82E4CA04: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CA08: 60A55556  ori r5, r5, 0x5556
	ctx.r[5].u64 = ctx.r[5].u64 | 21846;
	// 82E4CA0C: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CA10: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CA14: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E4CA18: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CA1C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CA20: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CA24: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	pc = 0x82E4CA28; continue 'dispatch;
            }
            0x82E4CA28 => {
    //   block [0x82E4CA28..0x82E4CC58)
	// 82E4CA28: 7CEB2896  mulhw r7, r11, r5
	ctx.r[7].s64 = ((ctx.r[11].s32 as i64 * ctx.r[5].s32 as i64) >> 32);
	// 82E4CA2C: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CA30: 54E60FFE  srwi r6, r7, 0x1f
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4CA34: C1680000  lfs f11, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E4CA38: ED2C0332  fmuls f9, f12, f12
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CA3C: C323000C  lfs f25, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82E4CA40: C2E30010  lfs f23, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82E4CA44: EC8B02F2  fmuls f4, f11, f11
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CA48: C2A30024  lfs f21, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82E4CA4C: C2C30018  lfs f22, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82E4CA50: C263001C  lfs f19, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 82E4CA54: C2430028  lfs f18, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 82E4CA58: C2830030  lfs f20, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 82E4CA5C: C223002C  lfs f17, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 82E4CA60: EC690332  fmuls f3, f9, f12
	ctx.f[3].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CA64: ED0402F2  fmuls f8, f4, f11
	ctx.f[8].f64 = (((ctx.f[4].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CA68: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E4CA6C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82E4CA70: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82E4CA74: 54E6083C  slwi r6, r7, 1
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4CA78: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 82E4CA7C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82E4CA80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4CA84: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82E4CA88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4CA8C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E4CA90: 7CC7FA14  add r6, r7, r31
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82E4CA94: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82E4CA98: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4CA9C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E4CAA0: 7C06242E  lfsx f0, r6, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CAA4: EF800332  fmuls f28, f0, f12
	ctx.f[28].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CAA8: 7DA7242E  lfsx f13, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CAAC: ED4D5828  fsubs f10, f13, f11
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E4CAB0: EFE0602A  fadds f31, f0, f12
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 82E4CAB4: EC400032  fmuls f2, f0, f0
	ctx.f[2].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CAB8: EFAD582A  fadds f29, f13, f11
	ctx.f[29].f64 = ((ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64;
	// 82E4CABC: EFCD0372  fmuls f30, f13, f13
	ctx.f[30].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CAC0: ECE06028  fsubs f7, f0, f12
	ctx.f[7].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CAC4: EF9C0072  fmuls f28, f28, f1
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CAC8: EF1FCABA  fmadds f24, f31, f10, f25
	ctx.f[24].f64 = (((ctx.f[31].f64 * ctx.f[10].f64 + ctx.f[25].f64) as f32) as f64);
	// 82E4CACC: D303000C  stfs f24, 0xc(r3)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4CAD0: EF5F483A  fmadds f26, f31, f0, f9
	ctx.f[26].f64 = (((ctx.f[31].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 82E4CAD4: EFBD237A  fmadds f29, f29, f13, f4
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64);
	// 82E4CAD8: EF7E0372  fmuls f27, f30, f13
	ctx.f[27].f64 = (((ctx.f[30].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CADC: EFDE02F2  fmuls f30, f30, f11
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CAE0: EC840372  fmuls f4, f4, f13
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CAE4: EF22E1BA  fmadds f25, f2, f6, f28
	ctx.f[25].f64 = (((ctx.f[2].f64 * ctx.f[6].f64 + ctx.f[28].f64) as f32) as f64);
	// 82E4CAE8: EF89E1BA  fmadds f28, f9, f6, f28
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[6].f64 + ctx.f[28].f64) as f32) as f64);
	// 82E4CAEC: EFFD437A  fmadds f31, f29, f13, f8
	ctx.f[31].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[8].f64) as f32) as f64);
	// 82E4CAF0: ED39482A  fadds f9, f25, f9
	ctx.f[9].f64 = ((ctx.f[25].f64 + ctx.f[9].f64) as f32) as f64;
	// 82E4CAF4: EF3A183A  fmadds f25, f26, f0, f3
	ctx.f[25].f64 = (((ctx.f[26].f64 * ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64);
	// 82E4CAF8: EF5ABABA  fmadds f26, f26, f10, f23
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[10].f64 + ctx.f[23].f64) as f32) as f64);
	// 82E4CAFC: D3430010  stfs f26, 0x10(r3)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CB00: EF9C102A  fadds f28, f28, f2
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[2].f64) as f32) as f64;
	// 82E4CB04: EC420032  fmuls f2, f2, f0
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CB08: EDFF0372  fmuls f15, f31, f13
	ctx.f[15].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CB0C: EE090332  fmuls f16, f9, f12
	ctx.f[16].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CB10: EEF90032  fmuls f23, f25, f0
	ctx.f[23].f64 = (((ctx.f[25].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CB14: EF39B2BA  fmadds f25, f25, f10, f22
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[10].f64 + ctx.f[22].f64) as f32) as f64);
	// 82E4CB18: C2C30014  lfs f22, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82E4CB1C: D3230018  stfs f25, 0x18(r3)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CB20: EC42817A  fmadds f2, f2, f5, f16
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[5].f64 + ctx.f[16].f64) as f32) as f64);
	// 82E4CB24: EEE3BB3A  fmadds f23, f3, f12, f23
	ctx.f[23].f64 = (((ctx.f[3].f64 * ctx.f[12].f64 + ctx.f[23].f64) as f32) as f64);
	// 82E4CB28: EC630172  fmuls f3, f3, f5
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[5].f64) as f32) as f64);
	// 82E4CB2C: EE087AFA  fmadds f16, f8, f11, f15
	ctx.f[16].f64 = (((ctx.f[8].f64 * ctx.f[11].f64 + ctx.f[15].f64) as f32) as f64);
	// 82E4CB30: EEF7AABA  fmadds f23, f23, f10, f21
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[10].f64 + ctx.f[21].f64) as f32) as f64);
	// 82E4CB34: C2A30020  lfs f21, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82E4CB38: EC7C183A  fmadds f3, f28, f0, f3
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64);
	// 82E4CB3C: D2E30024  stfs f23, 0x24(r3)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CB40: EF9C02F2  fmuls f28, f28, f11
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CB44: EC6302F2  fmuls f3, f3, f11
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CB48: EF89E37A  fmadds f28, f9, f13, f28
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[28].f64) as f32) as f64);
	// 82E4CB4C: ED3FA9FA  fmadds f9, f31, f7, f21
	ctx.f[9].f64 = (((ctx.f[31].f64 * ctx.f[7].f64 + ctx.f[21].f64) as f32) as f64);
	// 82E4CB50: ED7DB1FA  fmadds f11, f29, f7, f22
	ctx.f[11].f64 = (((ctx.f[29].f64 * ctx.f[7].f64 + ctx.f[22].f64) as f32) as f64);
	// 82E4CB54: EFFE0072  fmuls f31, f30, f1
	ctx.f[31].f64 = (((ctx.f[30].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CB58: EC421B7A  fmadds f2, f2, f13, f3
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[3].f64) as f32) as f64);
	// 82E4CB5C: EC7C9ABA  fmadds f3, f28, f10, f19
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[10].f64 + ctx.f[19].f64) as f32) as f64);
	// 82E4CB60: EDB0A1FA  fmadds f13, f16, f7, f20
	ctx.f[13].f64 = (((ctx.f[16].f64 * ctx.f[7].f64 + ctx.f[20].f64) as f32) as f64);
	// 82E4CB64: ED4292BA  fmadds f10, f2, f10, f18
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[10].f64 + ctx.f[18].f64) as f32) as f64);
	// 82E4CB68: EC5E01B2  fmuls f2, f30, f6
	ctx.f[2].f64 = (((ctx.f[30].f64 * ctx.f[6].f64) as f32) as f64);
	// 82E4CB6C: EC44107A  fmadds f2, f4, f1, f2
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64);
	// 82E4CB70: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CB74: EC84F9BA  fmadds f4, f4, f6, f31
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[6].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CB78: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CB7C: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E4CB80: D063001C  stfs f3, 0x1c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E4CB84: D1430028  stfs f10, 0x28(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CB88: EC5B117A  fmadds f2, f27, f5, f2
	ctx.f[2].f64 = (((ctx.f[27].f64 * ctx.f[5].f64 + ctx.f[2].f64) as f32) as f64);
	// 82E4CB8C: EC88217A  fmadds f4, f8, f5, f4
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[4].f64) as f32) as f64);
	// 82E4CB90: ED02402A  fadds f8, f2, f8
	ctx.f[8].f64 = ((ctx.f[2].f64 + ctx.f[8].f64) as f32) as f64;
	// 82E4CB94: EC84D82A  fadds f4, f4, f27
	ctx.f[4].f64 = ((ctx.f[4].f64 + ctx.f[27].f64) as f32) as f64;
	// 82E4CB98: EC080032  fmuls f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CB9C: EC04033A  fmadds f0, f4, f12, f0
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CBA0: EC0089FA  fmadds f0, f0, f7, f17
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 + ctx.f[17].f64) as f32) as f64);
	// 82E4CBA4: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E4CBA8: 409AFE80  bne cr6, 0x82e4ca28
	if !ctx.cr[6].eq {
	pc = 0x82E4CA28; continue 'dispatch;
	}
	// 82E4CBAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CBB0: C18B0BFC  lfs f12, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E4CBB8: ED180332  fmuls f8, f24, f12
	ctx.f[8].f64 = (((ctx.f[24].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBBC: D103000C  stfs f8, 0xc(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4CBC0: C18BC40C  lfs f12, -0x3bf4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15348 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CBC8: ECFA0332  fmuls f7, f26, f12
	ctx.f[7].f64 = (((ctx.f[26].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBCC: D0E30010  stfs f7, 0x10(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CBD0: C18B0C7C  lfs f12, 0xc7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3196 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CBD8: ED190332  fmuls f8, f25, f12
	ctx.f[8].f64 = (((ctx.f[25].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBDC: D1030018  stfs f8, 0x18(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CBE0: C18B0AB4  lfs f12, 0xab4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2740 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4CBE8: ECF70332  fmuls f7, f23, f12
	ctx.f[7].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBEC: D0E30024  stfs f7, 0x24(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CBF0: C18B2F00  lfs f12, 0x2f00(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12032 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBF4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4CBF8: ED6B0332  fmuls f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBFC: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CC00: C18B721C  lfs f12, 0x721c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CC04: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82E4CC08: ED290332  fmuls f9, f9, f12
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CC0C: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CC10: C18B1354  lfs f12, 0x1354(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4948 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CC14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CC18: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CC1C: D1830030  stfs f12, 0x30(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E4CC20: C1AB0AA8  lfs f13, 0xaa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2728 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CC28: ED630372  fmuls f11, f3, f13
	ctx.f[11].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC2C: D163001C  stfs f11, 0x1c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E4CC30: C1AB0AA0  lfs f13, 0xaa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2720 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4CC38: ED8A0372  fmuls f12, f10, f13
	ctx.f[12].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC3C: D1830028  stfs f12, 0x28(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CC40: C1AB7218  lfs f13, 0x7218(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29208 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC44: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC48: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E4CC4C: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4CC50: 4BE610A1  bl 0x82cadcf0
	ctx.lr = 0x82E4CC54;
	sub_82CADCEC(ctx, base);
	// 82E4CC54: 4BE5C808  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4CC58 size=584
    let mut pc: u32 = 0x82E4CC58;
    'dispatch: loop {
        match pc {
            0x82E4CC58 => {
    //   block [0x82E4CC58..0x82E4CEA0)
	// 82E4CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4CC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4CC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4CC64: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82E4CC68: 4BE61055  bl 0x82cadcbc
	ctx.lr = 0x82E4CC6C;
	sub_82CADCA0(ctx, base);
	// 82E4CC6C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4CC70: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4CC74: 4BFFFD35  bl 0x82e4c9a8
	ctx.lr = 0x82E4CC78;
	sub_82E4C9A8(ctx, base);
	// 82E4CC78: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CC7C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4CC80: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC84: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4CC88: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC8C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4CC90: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CC94: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4CC98: C17F0004  lfs f11, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E4CC9C: C0A40008  lfs f5, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4CCA0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4CCA4: C03F0008  lfs f1, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E4CCA8: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4CCAC: 7D29FC2E  lfsx f9, r9, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E4CCB0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4CCB4: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E4CCB8: C1030014  lfs f8, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82E4CCBC: C3E3000C  lfs f31, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4CCC0: C0E30018  lfs f7, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E4CCC4: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CCC8: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CCCC: EC004824  fdivs f0, f0, f9
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[9].f64) as f32) as f64;
	// 82E4CCD0: C0C30020  lfs f6, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E4CCD4: C0630024  lfs f3, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E4CCD8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4CCDC: C083001C  lfs f4, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82E4CCE0: C0430030  lfs f2, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E4CCE4: C1290C4C  lfs f9, 0xc4c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3148 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E4CCE8: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82E4CCEC: EDAC6AFC  fnmsubs f13, f12, f11, f13
	ctx.f[13].f64 = -(((ctx.f[12].f64 * ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E4CCF0: ED870032  fmuls f12, f7, f0
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CCF4: EFC60032  fmuls f30, f6, f0
	ctx.f[30].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CCF8: EFA30032  fmuls f29, f3, f0
	ctx.f[29].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CCFC: EF820032  fmuls f28, f2, f0
	ctx.f[28].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD00: ED65687C  fnmsubs f11, f5, f1, f13
	ctx.f[11].f64 = -(((ctx.f[5].f64 * ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E4CD04: EDAA0032  fmuls f13, f10, f0
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD08: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E4CD0C: EDA80032  fmuls f13, f8, f0
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD10: D1A30038  stfs f13, 0x38(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E4CD14: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CD18: EC200032  fmuls f1, f0, f0
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD1C: EDA80372  fmuls f13, f8, f13
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CD20: 7CABFC2E  lfsx f5, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4CD24: D3C30044  stfs f30, 0x44(r3)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82E4CD28: D1830040  stfs f12, 0x40(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82E4CD2C: EFFF02F2  fmuls f31, f31, f11
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CD30: EFC10032  fmuls f30, f1, f0
	ctx.f[30].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD34: EDA56ABA  fmadds f13, f5, f10, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CD38: EF1E0032  fmuls f24, f30, f0
	ctx.f[24].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD3C: EDADF82A  fadds f13, f13, f31
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E4CD40: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CD44: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CD48: D1A3003C  stfs f13, 0x3c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82E4CD4C: 7D8BFC2E  lfsx f12, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CD50: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CD54: ECAC02B2  fmuls f5, f12, f10
	ctx.f[5].f64 = (((ctx.f[12].f64 * ctx.f[10].f64) as f32) as f64);
	// 82E4CD58: D3A3004C  stfs f29, 0x4c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82E4CD5C: EFA40332  fmuls f29, f4, f12
	ctx.f[29].f64 = (((ctx.f[4].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CD60: D3830050  stfs f28, 0x50(r3)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4CD64: EF870332  fmuls f28, f7, f12
	ctx.f[28].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CD68: ECAD2A3A  fmadds f5, f13, f8, f5
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64);
	// 82E4CD6C: EFBD0372  fmuls f29, f29, f13
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CD70: ECA5FA7A  fmadds f5, f5, f9, f31
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CD74: EFBD0272  fmuls f29, f29, f9
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[9].f64) as f32) as f64);
	// 82E4CD78: ECA5EAFA  fmadds f5, f5, f11, f29
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[11].f64 + ctx.f[29].f64) as f32) as f64);
	// 82E4CD7C: EFA60372  fmuls f29, f6, f13
	ctx.f[29].f64 = (((ctx.f[6].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CD80: EDBD2B7A  fmadds f13, f29, f13, f5
	ctx.f[13].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 82E4CD84: C3A3002C  lfs f29, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E4CD88: C0A9BDF4  lfs f5, -0x420c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16908 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4CD8C: EDBC6B3A  fmadds f13, f28, f12, f13
	ctx.f[13].f64 = (((ctx.f[28].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CD90: C3830028  lfs f28, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E4CD94: EDAD07B2  fmuls f13, f13, f30
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E4CD98: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82E4CD9C: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CDA0: EF4D02B2  fmuls f26, f13, f10
	ctx.f[26].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 82E4CDA4: 7D8AFC2E  lfsx f12, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CDA8: EF640372  fmuls f27, f4, f13
	ctx.f[27].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDAC: EF230372  fmuls f25, f3, f13
	ctx.f[25].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDB0: ED4A02F2  fmuls f10, f10, f11
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CDB4: EC1C0032  fmuls f0, f28, f0
	ctx.f[0].f64 = (((ctx.f[28].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CDB8: D0030058  stfs f0, 0x58(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E4CDBC: EC0CD23A  fmadds f0, f12, f8, f26
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[8].f64 + ctx.f[26].f64) as f32) as f64);
	// 82E4CDC0: EF7B0332  fmuls f27, f27, f12
	ctx.f[27].f64 = (((ctx.f[27].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDC4: EEFD0372  fmuls f23, f29, f13
	ctx.f[23].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDC8: EEC70372  fmuls f22, f7, f13
	ctx.f[22].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDCC: EEBC0332  fmuls f21, f28, f12
	ctx.f[21].f64 = (((ctx.f[28].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDD0: EF390372  fmuls f25, f25, f13
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDD4: EC00F97A  fmadds f0, f0, f5, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[5].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CDD8: EFE60332  fmuls f31, f6, f12
	ctx.f[31].f64 = (((ctx.f[6].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDDC: EF570332  fmuls f26, f23, f12
	ctx.f[26].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDE0: EEE20332  fmuls f23, f2, f12
	ctx.f[23].f64 = (((ctx.f[2].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDE4: ED150372  fmuls f8, f21, f13
	ctx.f[8].f64 = (((ctx.f[21].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDE8: EC0002F2  fmuls f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CDEC: EFFF0332  fmuls f31, f31, f12
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDF0: EF5A0332  fmuls f26, f26, f12
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDF4: EEF70332  fmuls f23, f23, f12
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDF8: ED080372  fmuls f8, f8, f13
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDFC: EFFBFA7A  fmadds f31, f27, f9, f31
	ctx.f[31].f64 = (((ctx.f[27].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CE00: EF7A0172  fmuls f27, f26, f5
	ctx.f[27].f64 = (((ctx.f[26].f64 * ctx.f[5].f64) as f32) as f64);
	// 82E4CE04: EFF6FB7A  fmadds f31, f22, f13, f31
	ctx.f[31].f64 = (((ctx.f[22].f64 * ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CE08: EC1F017A  fmadds f0, f31, f5, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[5].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE0C: EC00DAFA  fmadds f0, f0, f11, f27
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[27].f64) as f32) as f64);
	// 82E4CE10: EC08017A  fmadds f0, f8, f5, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE14: EC17033A  fmadds f0, f23, f12, f0
	ctx.f[0].f64 = (((ctx.f[23].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE18: EC19037A  fmadds f0, f25, f13, f0
	ctx.f[0].f64 = (((ctx.f[25].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE1C: EC000632  fmuls f0, f0, f24
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[24].f64) as f32) as f64);
	// 82E4CE20: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CE24: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E4CE28: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CE2C: EC020032  fmuls f0, f2, f0
	ctx.f[0].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CE30: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CE34: EC0D077A  fmadds f0, f13, f29, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE38: EC0602FA  fmadds f0, f6, f11, f0
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE3C: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CE40: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CE44: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E4CE48: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CE4C: ED870372  fmuls f12, f7, f13
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CE50: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CE54: ED1C0372  fmuls f8, f28, f13
	ctx.f[8].f64 = (((ctx.f[28].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CE58: ED84603A  fmadds f12, f4, f0, f12
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4CE5C: ED080032  fmuls f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CE60: ED8C527A  fmadds f12, f12, f9, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[9].f64 + ctx.f[10].f64) as f32) as f64);
	// 82E4CE64: ED480272  fmuls f10, f8, f9
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 82E4CE68: ED8C52FA  fmadds f12, f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 82E4CE6C: ED7D0032  fmuls f11, f29, f0
	ctx.f[11].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CE70: ED430372  fmuls f10, f3, f13
	ctx.f[10].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CE74: EC0B603A  fmadds f0, f11, f0, f12
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4CE78: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE7C: EC0007B2  fmuls f0, f0, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E4CE80: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E4CE84: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E4CE88: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82E4CE8C: 4BE60E7D  bl 0x82cadd08
	ctx.lr = 0x82E4CE90;
	sub_82CADCEC(ctx, base);
	// 82E4CE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4CE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4CE98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4CE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4CEA0 size=340
    let mut pc: u32 = 0x82E4CEA0;
    'dispatch: loop {
        match pc {
            0x82E4CEA0 => {
    //   block [0x82E4CEA0..0x82E4CFF4)
	// 82E4CEA0: C1A30064  lfs f13, 0x64(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEA4: C0030068  lfs f0, 0x68(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEA8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E4CEAC: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4CEB0: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEB4: C003006C  lfs f0, 0x6c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEB8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E4CEBC: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4CEC0: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEC4: C0030070  lfs f0, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEC8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E4CECC: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4CED0: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CED4: C0030078  lfs f0, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CED8: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E4CEDC: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CEE0: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4CEE4: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEE8: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEEC: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E4CEF0: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CEF4: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CEF8: C1A30078  lfs f13, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEFC: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF00: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E4CF04: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF08: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CF0C: C0030080  lfs f0, 0x80(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF10: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF14: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CF18: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4CF1C: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CF20: C0030084  lfs f0, 0x84(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF24: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF28: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CF2C: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CF30: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CF34: C0030088  lfs f0, 0x88(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF38: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF3C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CF40: D0070020  stfs f0, 0x20(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CF44: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4CF48: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF4C: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CF50: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CF54: C1870000  lfs f12, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CF58: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CF5C: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CF60: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4CF64: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CF68: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CF6C: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF70: C1870014  lfs f12, 0x14(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CF74: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CF78: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CF7C: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CF80: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CF84: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CF88: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF8C: C1870028  lfs f12, 0x28(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CF90: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CF94: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CF98: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CF9C: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFA0: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CFA4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CFA8: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFAC: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CFB0: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4CFB4: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CFB8: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFBC: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CFC0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CFC4: C1A70018  lfs f13, 0x18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFC8: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CFCC: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CFD0: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CFD4: C1A60000  lfs f13, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFD8: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CFDC: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CFE0: C1A70020  lfs f13, 0x20(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFE4: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CFE8: D0070020  stfs f0, 0x20(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CFEC: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4CFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4CFF8 size=184
    let mut pc: u32 = 0x82E4CFF8;
    'dispatch: loop {
        match pc {
            0x82E4CFF8 => {
    //   block [0x82E4CFF8..0x82E4D0B0)
	// 82E4CFF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4D0B0 size=760
    let mut pc: u32 = 0x82E4D0B0;
    'dispatch: loop {
        match pc {
            0x82E4D0B0 => {
    //   block [0x82E4D0B0..0x82E4D3A8)
	// 82E4D0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D0B4: 4BE5C341  bl 0x82ca93f4
	ctx.lr = 0x82E4D0B8;
	sub_82CA93D0(ctx, base);
	// 82E4D0B8: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82E4D0BC: 4BE60C11  bl 0x82cadccc
	ctx.lr = 0x82E4D0C0;
	sub_82CADCA0(ctx, base);
	// 82E4D0C0: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4D3A8 size=424
    let mut pc: u32 = 0x82E4D3A8;
    'dispatch: loop {
        match pc {
            0x82E4D3A8 => {
    //   block [0x82E4D3A8..0x82E4D3DC)
	// 82E4D3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4D3B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4D3B4: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82E4D3B8: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E4D3BC: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E4D3C0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4D3C4: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 82E4D3C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4D3CC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E4D3D0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4D3D4: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 82E4D3D8: 4199000C  bgt cr6, 0x82e4d3e4
	if ctx.cr[6].gt {
	pc = 0x82E4D3E4; continue 'dispatch;
	}
	pc = 0x82E4D3DC; continue 'dispatch;
            }
            0x82E4D3DC => {
    //   block [0x82E4D3DC..0x82E4D3E4)
	// 82E4D3DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4D3E0: 48000150  b 0x82e4d530
	pc = 0x82E4D530; continue 'dispatch;
            }
            0x82E4D3E4 => {
    //   block [0x82E4D3E4..0x82E4D530)
	// 82E4D3E4: FF011800  fcmpu cr6, f1, f3
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[3].f64);
	// 82E4D3E8: 4099FFF4  ble cr6, 0x82e4d3dc
	if !ctx.cr[6].gt {
	pc = 0x82E4D3DC; continue 'dispatch;
	}
	// 82E4D3EC: FF030000  fcmpu cr6, f3, f0
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[0].f64);
	// 82E4D3F0: 4099FFEC  ble cr6, 0x82e4d3dc
	if !ctx.cr[6].gt {
	pc = 0x82E4D3DC; continue 'dispatch;
	}
	// 82E4D3F4: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 82E4D3F8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4D3FC: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82E4D400: EDA10072  fmuls f13, f1, f1
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4D404: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82E4D408: ED411828  fsubs f10, f1, f3
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 82E4D40C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	pc = 0x82E4D530; continue 'dispatch;
            }
            0x82E4D530 => {
    //   block [0x82E4D530..0x82E4D550)
	// 82E4D530: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82E4D534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4D538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4D53C: CBA1FFD8  lfd f29, -0x28(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E4D540: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E4D544: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4D548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4D54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4D550 size=516
    let mut pc: u32 = 0x82E4D550;
    'dispatch: loop {
        match pc {
            0x82E4D550 => {
    //   block [0x82E4D550..0x82E4D730)
	// 82E4D550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4D558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4D55C: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82E4D560: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E4D564: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E4D568: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4D56C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E4D570: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4D574: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E4D578: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4D57C: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 82E4D580: 409901B0  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D584: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D588: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82E4D58C: 409901A4  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D590: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D594: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82E4D598: 40990198  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D59C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D5A0: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82E4D5A4: 4099018C  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D5A8: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82E4D5AC: 40990184  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D5B0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E4D5B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4D5B8: D00100B0  stfs f0, 0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 82E4D5BC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4D5C0: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 82E4D5C4: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D5C8: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	pc = 0x82E4D730; continue 'dispatch;
            }
            0x82E4D730 => {
    //   block [0x82E4D730..0x82E4D754)
	// 82E4D730: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4D734: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82E4D738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4D73C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4D740: CBA1FFD8  lfd f29, -0x28(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E4D744: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E4D748: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4D74C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4D750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4D758 size=608
    let mut pc: u32 = 0x82E4D758;
    'dispatch: loop {
        match pc {
            0x82E4D758 => {
    //   block [0x82E4D758..0x82E4D9B8)
	// 82E4D758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D75C: 4BE5BCA1  bl 0x82ca93fc
	ctx.lr = 0x82E4D760;
	sub_82CA93D0(ctx, base);
	// 82E4D760: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82E4D764: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82E4D768: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E4D76C: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4D9B8 size=1104
    let mut pc: u32 = 0x82E4D9B8;
    'dispatch: loop {
        match pc {
            0x82E4D9B8 => {
    //   block [0x82E4D9B8..0x82E4DD78)
	// 82E4D9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D9BC: 4BE5BA15  bl 0x82ca93d0
	ctx.lr = 0x82E4D9C0;
	sub_82CA93D0(ctx, base);
	// 82E4D9C0: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82E4D9C4: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82E4D9C8: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82E4D9CC: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4D9D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4D9D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4D9D8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E4D9DC: 3B230088  addi r25, r3, 0x88
	ctx.r[25].s64 = ctx.r[3].s64 + 136;
	// 82E4D9E0: 3B030084  addi r24, r3, 0x84
	ctx.r[24].s64 = ctx.r[3].s64 + 132;
	// 82E4D9E4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4D9E8: 3AE30080  addi r23, r3, 0x80
	ctx.r[23].s64 = ctx.r[3].s64 + 128;
	// 82E4D9EC: C3AA0C18  lfs f29, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E4D9F0: 3AC3007C  addi r22, r3, 0x7c
	ctx.r[22].s64 = ctx.r[3].s64 + 124;
	// 82E4D9F4: D3A30088  stfs f29, 0x88(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82E4D9F8: 3AA30078  addi r21, r3, 0x78
	ctx.r[21].s64 = ctx.r[3].s64 + 120;
	// 82E4D9FC: D3A30084  stfs f29, 0x84(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E4DA00: 3A830074  addi r20, r3, 0x74
	ctx.r[20].s64 = ctx.r[3].s64 + 116;
	// 82E4DA04: D3A30080  stfs f29, 0x80(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E4DA08: 3A630070  addi r19, r3, 0x70
	ctx.r[19].s64 = ctx.r[3].s64 + 112;
	// 82E4DA0C: D3A3007C  stfs f29, 0x7c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E4DA10: 3A43006C  addi r18, r3, 0x6c
	ctx.r[18].s64 = ctx.r[3].s64 + 108;
	// 82E4DA14: D3A30078  stfs f29, 0x78(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E4DA18: 3A230068  addi r17, r3, 0x68
	ctx.r[17].s64 = ctx.r[3].s64 + 104;
	// 82E4DA1C: D3A30074  stfs f29, 0x74(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E4DA20: 3BC30064  addi r30, r3, 0x64
	ctx.r[30].s64 = ctx.r[3].s64 + 100;
	// 82E4DA24: D3A30070  stfs f29, 0x70(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E4DA28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4DA2C: D3A3006C  stfs f29, 0x6c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E4DA30: D3A30068  stfs f29, 0x68(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E4DA34: D3A30064  stfs f29, 0x64(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E4DA38: 40990340  ble cr6, 0x82e4dd78
	if !ctx.cr[6].gt {
	pc = 0x82E4DD78; continue 'dispatch;
	}
	// 82E4DA3C: 7D705B78  mr r16, r11
	ctx.r[16].u64 = ctx.r[11].u64;
	// 82E4DA40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DA44: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82E4DA48: 3D405555  lis r10, 0x5555
	ctx.r[10].s64 = 1431633920;
	// 82E4DA4C: 3B5D000C  addi r26, r29, 0xc
	ctx.r[26].s64 = ctx.r[29].s64 + 12;
	// 82E4DA50: 7DFC7B78  mr r28, r15
	ctx.r[28].u64 = ctx.r[15].u64;
	// 82E4DA54: C3CB0C14  lfs f30, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E4DA58: 615F5556  ori r31, r10, 0x5556
	ctx.r[31].u64 = ctx.r[10].u64 | 21846;
	// 82E4DA5C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82E4DD78; continue 'dispatch;
            }
            0x82E4DD78 => {
    //   block [0x82E4DD78..0x82E4DE08)
	// 82E4DD78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DD7C: C1B10000  lfs f13, 0(r17)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4DD80: C1540000  lfs f10, 0(r20)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E4DD84: C1150000  lfs f8, 0(r21)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82E4DD88: C0F60000  lfs f7, 0(r22)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E4DD8C: C1920000  lfs f12, 0(r18)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4DD90: C00B0BFC  lfs f0, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4DD94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DD98: ED6D0032  fmuls f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4DD9C: C1B30000  lfs f13, 0(r19)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4DDA0: ED2D0032  fmuls f9, f13, f0
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4DDA4: C0D70000  lfs f6, 0(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E4DDA8: C0B90000  lfs f5, 0(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4DDAC: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4DDB0: ECC60032  fmuls f6, f6, f0
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4DDB4: D1710000  stfs f11, 0(r17)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDB8: C1AB0CA8  lfs f13, 0xca8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3240 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4DDBC: ED4A0372  fmuls f10, f10, f13
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4DDC0: D1920000  stfs f12, 0(r18)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDC4: ED080372  fmuls f8, f8, f13
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4DDC8: D1330000  stfs f9, 0(r19)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDCC: EDA70372  fmuls f13, f7, f13
	ctx.f[13].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4DDD0: C0F80000  lfs f7, 0(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E4DDD4: ECE70032  fmuls f7, f7, f0
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4DDD8: D1540000  stfs f10, 0(r20)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDDC: EC050032  fmuls f0, f5, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4DDE0: D1150000  stfs f8, 0(r21)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDE4: D1B60000  stfs f13, 0(r22)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDE8: D0D70000  stfs f6, 0(r23)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDEC: D0F80000  stfs f7, 0(r24)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDF0: D0190000  stfs f0, 0(r25)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4DDF4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82E4DDF8: CBA1FF50  lfd f29, -0xb0(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-176 as u32) ) };
	// 82E4DDFC: CBC1FF58  lfd f30, -0xa8(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	// 82E4DE00: CBE1FF60  lfd f31, -0xa0(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	// 82E4DE04: 4BE5B61C  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4DE08 size=564
    let mut pc: u32 = 0x82E4DE08;
    'dispatch: loop {
        match pc {
            0x82E4DE08 => {
    //   block [0x82E4DE08..0x82E4DE48)
	// 82E4DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4DE10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4DE14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4DE18: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82E4DE1C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E4DE20: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4DE24: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4DE28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DE2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4DE30: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4DE34: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4DE38: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82E4DE3C: 4199000C  bgt cr6, 0x82e4de48
	if ctx.cr[6].gt {
	pc = 0x82E4DE48; continue 'dispatch;
	}
	// 82E4DE40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DE44: C3CB0C14  lfs f30, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	pc = 0x82E4DE48; continue 'dispatch;
            }
            0x82E4DE48 => {
    //   block [0x82E4DE48..0x82E4E03C)
	// 82E4DE48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E4DE4C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4DE50: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 82E4DE54: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4DE58: 4BFFF1A1  bl 0x82e4cff8
	ctx.lr = 0x82E4DE5C;
	sub_82E4CFF8(ctx, base);
	// 82E4DE5C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4DE60: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4DE64: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E4DE68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4E040 size=496
    let mut pc: u32 = 0x82E4E040;
    'dispatch: loop {
        match pc {
            0x82E4E040 => {
    //   block [0x82E4E040..0x82E4E080)
	// 82E4E040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E04C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E050: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82E4E054: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E4E058: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E05C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4E060: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4E064: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E068: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4E06C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4E070: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82E4E074: 4199000C  bgt cr6, 0x82e4e080
	if ctx.cr[6].gt {
	pc = 0x82E4E080; continue 'dispatch;
	}
	// 82E4E078: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4E07C: 48000194  b 0x82e4e210
	pc = 0x82E4E210; continue 'dispatch;
            }
            0x82E4E080 => {
    //   block [0x82E4E080..0x82E4E210)
	// 82E4E080: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E4E084: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4E088: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4E08C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E090: 4BFFEF69  bl 0x82e4cff8
	ctx.lr = 0x82E4E094;
	sub_82E4CFF8(ctx, base);
	// 82E4E094: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4E098: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E4E09C: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E4E0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	pc = 0x82E4E210; continue 'dispatch;
            }
            0x82E4E210 => {
    //   block [0x82E4E210..0x82E4E230)
	// 82E4E210: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82E4E214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E21C: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E4E220: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E4E224: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4E230 size=1664
    let mut pc: u32 = 0x82E4E230;
    'dispatch: loop {
        match pc {
            0x82E4E230 => {
    //   block [0x82E4E230..0x82E4E26C)
	// 82E4E230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E234: 4BE5B1D5  bl 0x82ca9408
	ctx.lr = 0x82E4E238;
	sub_82CA93D0(ctx, base);
	// 82E4E238: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E4E23C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E240: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E4E244: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4E248: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4E24C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E250: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82E4E254: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4E258: 41990014  bgt cr6, 0x82e4e26c
	if ctx.cr[6].gt {
	pc = 0x82E4E26C; continue 'dispatch;
	}
	// 82E4E25C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4E260: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4E264: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E4E268: 4BE5B1F0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4E26C => {
    //   block [0x82E4E26C..0x82E4E2A0)
	// 82E4E26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E270: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4E274: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4E278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E4E27C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4E280: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82E4E284: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E4E288: 40990018  ble cr6, 0x82e4e2a0
	if !ctx.cr[6].gt {
	pc = 0x82E4E2A0; continue 'dispatch;
	}
	// 82E4E28C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E4E290: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4E294: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4E298: 4BF08C79  bl 0x82d56f10
	ctx.lr = 0x82E4E29C;
	sub_82D56F10(ctx, base);
	// 82E4E29C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	pc = 0x82E4E2A0; continue 'dispatch;
            }
            0x82E4E2A0 => {
    //   block [0x82E4E2A0..0x82E4E2C8)
	// 82E4E2A0: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E4E2A4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E4E2A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E4E2AC: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82E4E2B0: 419800F4  blt cr6, 0x82e4e3a4
	if ctx.cr[6].lt {
	pc = 0x82E4E3A4; continue 'dispatch;
	}
	// 82E4E2B4: 393CFFFC  addi r9, r28, -4
	ctx.r[9].s64 = ctx.r[28].s64 + -4;
	// 82E4E2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E2BC: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4E2C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E4E2C4: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	pc = 0x82E4E2C8; continue 'dispatch;
            }
            0x82E4E2C8 => {
    //   block [0x82E4E2C8..0x82E4E3A4)
	// 82E4E2C8: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E2CC: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82E4E2D0: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82E4E2D4: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E2D8: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E2DC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4E2E0: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E2E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4E2E8: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E2EC: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E2F0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E2F4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E2F8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E2FC: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4E300: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E304: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E308: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E30C: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4E310: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E314: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E318: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E31C: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4E320: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E324: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E328: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E32C: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E330: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4E334: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E338: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E33C: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82E4E340: D007FFF0  stfs f0, -0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82E4E344: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E348: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E34C: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E350: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4E354: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E358: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E35C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E360: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E364: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4E368: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E36C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E370: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82E4E374: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E378: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E37C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E380: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E4E384: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E388: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E38C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E390: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E394: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E4E398: D0080038  stfs f0, 0x38(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E4E39C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3A0: 409AFF28  bne cr6, 0x82e4e2c8
	if !ctx.cr[6].eq {
	pc = 0x82E4E2C8; continue 'dispatch;
	}
	pc = 0x82E4E3A4; continue 'dispatch;
            }
            0x82E4E3A4 => {
    //   block [0x82E4E3A4..0x82E4E3B4)
	// 82E4E3A4: 7F06E000  cmpw cr6, r6, r28
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E4E3A8: 4098004C  bge cr6, 0x82e4e3f4
	if !ctx.cr[6].lt {
	pc = 0x82E4E3F4; continue 'dispatch;
	}
	// 82E4E3AC: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4E3B0: 7D26E050  subf r9, r6, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[6].s64;
	pc = 0x82E4E3B4; continue 'dispatch;
            }
            0x82E4E3B4 => {
    //   block [0x82E4E3B4..0x82E4E3F4)
	// 82E4E3B4: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E3B8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4E3BC: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82E4E3C0: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3C4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E3C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4E3CC: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E3D0: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E3D4: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3D8: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E3DC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E3E0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E3E4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E4E3E8: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4E3EC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3F0: 409AFFC4  bne cr6, 0x82e4e3b4
	if !ctx.cr[6].eq {
	pc = 0x82E4E3B4; continue 'dispatch;
	}
	pc = 0x82E4E3F4; continue 'dispatch;
            }
            0x82E4E3F4 => {
    //   block [0x82E4E3F4..0x82E4E8B0)
	// 82E4E3F4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E4E3F8: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E3FC: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82E4E400: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E4E404: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E4E408: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4E8B0 size=668
    let mut pc: u32 = 0x82E4E8B0;
    'dispatch: loop {
        match pc {
            0x82E4E8B0 => {
    //   block [0x82E4E8B0..0x82E4E8F4)
	// 82E4E8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E8B4: 4BE5AB4D  bl 0x82ca9400
	ctx.lr = 0x82E4E8B8;
	sub_82CA93D0(ctx, base);
	// 82E4E8B8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E4E8BC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E8C0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4E8C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4E8C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E8CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4E8D0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4E8D4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E4E8D8: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E8DC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82E4E8E0: 41990014  bgt cr6, 0x82e4e8f4
	if ctx.cr[6].gt {
	pc = 0x82E4E8F4; continue 'dispatch;
	}
	// 82E4E8E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4E8E8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E4E8EC: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4E8F0: 4BE5AB60  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4E8F4 => {
    //   block [0x82E4E8F4..0x82E4E91C)
	// 82E4E8F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4E8F8: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82E4E8FC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E4E900: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82E4E904: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82E4E908: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82E4E90C: 4099001C  ble cr6, 0x82e4e928
	if !ctx.cr[6].gt {
	pc = 0x82E4E928; continue 'dispatch;
	}
	// 82E4E910: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4E914: 41980008  blt cr6, 0x82e4e91c
	if ctx.cr[6].lt {
	pc = 0x82E4E91C; continue 'dispatch;
	}
	// 82E4E918: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x82E4E91C; continue 'dispatch;
            }
            0x82E4E91C => {
    //   block [0x82E4E91C..0x82E4E928)
	// 82E4E91C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E4E920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E924: 4BF085ED  bl 0x82d56f10
	ctx.lr = 0x82E4E928;
	sub_82D56F10(ctx, base);
	pc = 0x82E4E928; continue 'dispatch;
            }
            0x82E4E928 => {
    //   block [0x82E4E928..0x82E4E950)
	// 82E4E928: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82E4E92C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82E4E930: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4E934: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82E4E938: 419800F4  blt cr6, 0x82e4ea2c
	if ctx.cr[6].lt {
	pc = 0x82E4EA2C; continue 'dispatch;
	}
	// 82E4E93C: 393DFFFC  addi r9, r29, -4
	ctx.r[9].s64 = ctx.r[29].s64 + -4;
	// 82E4E940: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E4E944: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4E948: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E4E94C: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	pc = 0x82E4E950; continue 'dispatch;
            }
            0x82E4E950 => {
    //   block [0x82E4E950..0x82E4EA2C)
	// 82E4E950: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E954: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E958: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82E4E95C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4E960: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4E964: 7C0B3D2E  stfsx f0, r11, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82E4E968: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E96C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E970: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E974: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E978: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E97C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E980: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4E984: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E988: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4E98C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E990: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E994: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E998: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4E99C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9A0: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9A4: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9A8: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4E9AC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9B0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9B4: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4E9B8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9BC: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4E9C0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9C4: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9C8: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82E4E9CC: D007FFF0  stfs f0, -0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82E4E9D0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9D4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9D8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9DC: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4E9E0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9E4: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9E8: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4E9EC: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9F0: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4E9F4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9F8: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9FC: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82E4EA00: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA04: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA08: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA0C: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E4EA10: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA14: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA18: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4EA1C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA20: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E4EA24: D0080038  stfs f0, 0x38(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E4EA28: 409AFF28  bne cr6, 0x82e4e950
	if !ctx.cr[6].eq {
	pc = 0x82E4E950; continue 'dispatch;
	}
	pc = 0x82E4EA2C; continue 'dispatch;
            }
            0x82E4EA2C => {
    //   block [0x82E4EA2C..0x82E4EA3C)
	// 82E4EA2C: 7F06E800  cmpw cr6, r6, r29
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E4EA30: 4098004C  bge cr6, 0x82e4ea7c
	if !ctx.cr[6].lt {
	pc = 0x82E4EA7C; continue 'dispatch;
	}
	// 82E4EA34: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4EA38: 7D26E850  subf r9, r6, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	pc = 0x82E4EA3C; continue 'dispatch;
            }
            0x82E4EA3C => {
    //   block [0x82E4EA3C..0x82E4EA7C)
	// 82E4EA3C: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA40: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA44: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4EA48: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4EA4C: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82E4EA50: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA54: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA58: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA5C: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4EA60: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA64: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA68: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4EA6C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA70: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E4EA74: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4EA78: 409AFFC4  bne cr6, 0x82e4ea3c
	if !ctx.cr[6].eq {
	pc = 0x82E4EA3C; continue 'dispatch;
	}
	pc = 0x82E4EA7C; continue 'dispatch;
            }
            0x82E4EA7C => {
    //   block [0x82E4EA7C..0x82E4EB08)
	// 82E4EA7C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82E4EA80: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82E4EA84: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E4EA88: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82E4EA8C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4EA90: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82E4EA94: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E4EA98: 93610088  stw r27, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 82E4EA9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4EAA0: 9381008C  stw r28, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 82E4EAA4: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E4EAA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EAAC: 93810090  stw r28, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 82E4EAB0: 93610094  stw r27, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 82E4EAB4: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82E4EAB8: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E4EABC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E4EAC0: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82E4EAC4: 48295CA5  bl 0x830e4768
	ctx.lr = 0x82E4EAC8;
	sub_830E4768(ctx, base);
	// 82E4EAC8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E4EACC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4EAD0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4EAD4: 4BFFF335  bl 0x82e4de08
	ctx.lr = 0x82E4EAD8;
	sub_82E4DE08(ctx, base);
	// 82E4EAD8: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4EADC: D3FA0004  stfs f31, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4EAE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4EAE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4EAE8: 409A0020  bne cr6, 0x82e4eb08
	if !ctx.cr[6].eq {
	pc = 0x82E4EB08; continue 'dispatch;
	}
	// 82E4EAEC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4EAF0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4EAF4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4EAF8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4EAFC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4EB00: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4EB04: 4BF067C5  bl 0x82d552c8
	ctx.lr = 0x82E4EB08;
	sub_82D552C8(ctx, base);
	pc = 0x82E4EB08; continue 'dispatch;
            }
            0x82E4EB08 => {
    //   block [0x82E4EB08..0x82E4EB3C)
	// 82E4EB08: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4EB0C: 4BFF76BD  bl 0x82e461c8
	ctx.lr = 0x82E4EB10;
	sub_82E461C8(ctx, base);
	// 82E4EB10: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4EB14: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4EB18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4EB1C: 409A0020  bne cr6, 0x82e4eb3c
	if !ctx.cr[6].eq {
	pc = 0x82E4EB3C; continue 'dispatch;
	}
	// 82E4EB20: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4EB24: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4EB28: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4EB2C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EB30: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4EB34: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4EB38: 4BF06791  bl 0x82d552c8
	ctx.lr = 0x82E4EB3C;
	sub_82D552C8(ctx, base);
	pc = 0x82E4EB3C; continue 'dispatch;
            }
            0x82E4EB3C => {
    //   block [0x82E4EB3C..0x82E4EB4C)
	// 82E4EB3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4EB40: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E4EB44: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4EB48: 4BE5A908  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4EB50 size=2688
    let mut pc: u32 = 0x82E4EB50;
    'dispatch: loop {
        match pc {
            0x82E4EB50 => {
    //   block [0x82E4EB50..0x82E4EB88)
	// 82E4EB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EB54: 4BE5A8B5  bl 0x82ca9408
	ctx.lr = 0x82E4EB58;
	sub_82CA93D0(ctx, base);
	// 82E4EB58: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82E4EB5C: 4BE5F165  bl 0x82cadcc0
	ctx.lr = 0x82E4EB60;
	sub_82CADCA0(ctx, base);
	// 82E4EB60: 9421FAA0  stwu r1, -0x560(r1)
	ea = ctx.r[1].u32.wrapping_add(-1376 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EB64: FEC01090  fmr f22, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[22].f64 = ctx.f[2].f64;
	// 82E4EB68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4EB6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4EB70: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E4EB74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4EB78: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E4EB7C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4EB80: FF16F800  fcmpu cr6, f22, f31
	ctx.cr[6].compare_f64(ctx.f[22].f64, ctx.f[31].f64);
	// 82E4EB84: 41990018  bgt cr6, 0x82e4eb9c
	if ctx.cr[6].gt {
	pc = 0x82E4EB9C; continue 'dispatch;
	}
	pc = 0x82E4EB88; continue 'dispatch;
            }
            0x82E4EB88 => {
    //   block [0x82E4EB88..0x82E4EB9C)
	// 82E4EB88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4EB8C: 38210560  addi r1, r1, 0x560
	ctx.r[1].s64 = ctx.r[1].s64 + 1376;
	// 82E4EB90: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82E4EB94: 4BE5F179  bl 0x82cadd0c
	ctx.lr = 0x82E4EB98;
	sub_82CADCEC(ctx, base);
	// 82E4EB98: 4BE5A8C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4EB9C => {
    //   block [0x82E4EB9C..0x82E4F5D0)
	// 82E4EB9C: FF1DF800  fcmpu cr6, f29, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[31].f64);
	// 82E4EBA0: 4099FFE8  ble cr6, 0x82e4eb88
	if !ctx.cr[6].gt {
	pc = 0x82E4EB88; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4F5D0 size=864
    let mut pc: u32 = 0x82E4F5D0;
    'dispatch: loop {
        match pc {
            0x82E4F5D0 => {
    //   block [0x82E4F5D0..0x82E4F91C)
	// 82E4F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F5D4: 4BE59E39  bl 0x82ca940c
	ctx.lr = 0x82E4F5D8;
	sub_82CA93D0(ctx, base);
	// 82E4F5D8: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4F5DC: 4BE5E6FD  bl 0x82cadcd8
	ctx.lr = 0x82E4F5E0;
	sub_82CADCA0(ctx, base);
	// 82E4F5E0: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F5E4: FF801090  fmr f28, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[2].f64;
	// 82E4F5E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4F5EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F5F0: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4F5F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F5F8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E4F5FC: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4F600: FF1CF800  fcmpu cr6, f28, f31
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[31].f64);
	// 82E4F604: 40990318  ble cr6, 0x82e4f91c
	if !ctx.cr[6].gt {
	pc = 0x82E4F91C; continue 'dispatch;
	}
	// 82E4F608: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82E4F60C: 40990310  ble cr6, 0x82e4f91c
	if !ctx.cr[6].gt {
	pc = 0x82E4F91C; continue 'dispatch;
	}
	pc = 0x82E4F91C; continue 'dispatch;
            }
            0x82E4F91C => {
    //   block [0x82E4F91C..0x82E4F930)
	// 82E4F91C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4F920: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82E4F924: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4F928: 4BE5E3FD  bl 0x82cadd24
	ctx.lr = 0x82E4F92C;
	sub_82CADCEC(ctx, base);
	// 82E4F92C: 4BE59B30  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4F930 size=644
    let mut pc: u32 = 0x82E4F930;
    'dispatch: loop {
        match pc {
            0x82E4F930 => {
    //   block [0x82E4F930..0x82E4F980)
	// 82E4F930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F934: 4BE59AD1  bl 0x82ca9404
	ctx.lr = 0x82E4F938;
	sub_82CA93D0(ctx, base);
	// 82E4F938: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82E4F93C: 4BE5E39D  bl 0x82cadcd8
	ctx.lr = 0x82E4F940;
	sub_82CADCA0(ctx, base);
	// 82E4F940: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F944: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4F948: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4F94C: 3941008C  addi r10, r1, 0x8c
	ctx.r[10].s64 = ctx.r[1].s64 + 140;
	// 82E4F950: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E4F954: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E4F958: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F95C: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82E4F960: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 82E4F964: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82E4F968: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4F96C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4F970: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82E4F974: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82E4F978: 4198009C  blt cr6, 0x82e4fa14
	if ctx.cr[6].lt {
	pc = 0x82E4FA14; continue 'dispatch;
	}
	// 82E4F97C: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	pc = 0x82E4F980; continue 'dispatch;
            }
            0x82E4F980 => {
    //   block [0x82E4F980..0x82E4F99C)
	// 82E4F980: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F984: 7FFE502E  lwzx r31, r30, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4F988: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4F98C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E4F990: 409A000C  bne cr6, 0x82e4f99c
	if !ctx.cr[6].eq {
	pc = 0x82E4F99C; continue 'dispatch;
	}
	// 82E4F994: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4F998: 48000010  b 0x82e4f9a8
	pc = 0x82E4F9A8; continue 'dispatch;
            }
            0x82E4F99C => {
    //   block [0x82E4F99C..0x82E4F9A8)
	// 82E4F99C: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4F9A0: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E4F9A4: 409A0060  bne cr6, 0x82e4fa04
	if !ctx.cr[6].eq {
	pc = 0x82E4FA04; continue 'dispatch;
	}
	pc = 0x82E4F9A8; continue 'dispatch;
            }
            0x82E4F9A8 => {
    //   block [0x82E4F9A8..0x82E4F9E8)
	// 82E4F9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F9AC: 419A0058  beq cr6, 0x82e4fa04
	if ctx.cr[6].eq {
	pc = 0x82E4FA04; continue 'dispatch;
	}
	// 82E4F9B0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F9B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4F9B8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4F9BC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4F9C0: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4F9C4: 7D7E512E  stwx r11, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82E4F9C8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E4F9CC: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4F9D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4F9D4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4F9D8: 409A0010  bne cr6, 0x82e4f9e8
	if !ctx.cr[6].eq {
	pc = 0x82E4F9E8; continue 'dispatch;
	}
	// 82E4F9DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4F9E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4F9E4: 4BF075B5  bl 0x82d56f98
	ctx.lr = 0x82E4F9E8;
	sub_82D56F98(ctx, base);
	pc = 0x82E4F9E8; continue 'dispatch;
            }
            0x82E4F9E8 => {
    //   block [0x82E4F9E8..0x82E4FA04)
	// 82E4F9E8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4F9EC: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4F9F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4F9F4: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82E4F9F8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4F9FC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82E4FA00: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	pc = 0x82E4FA04; continue 'dispatch;
            }
            0x82E4FA04 => {
    //   block [0x82E4FA04..0x82E4FA14)
	// 82E4FA04: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82E4FA08: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E4FA0C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4FA10: 4098FF70  bge cr6, 0x82e4f980
	if !ctx.cr[6].lt {
	pc = 0x82E4F980; continue 'dispatch;
	}
	pc = 0x82E4FA14; continue 'dispatch;
            }
            0x82E4FA14 => {
    //   block [0x82E4FA14..0x82E4FA34)
	// 82E4FA14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4FA18: 3BC9FFFF  addi r30, r9, -1
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	// 82E4FA1C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4FA20: C38B0C18  lfs f28, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E4FA24: FFA0E090  fmr f29, f28
	ctx.f[29].f64 = ctx.f[28].f64;
	// 82E4FA28: FFE0E090  fmr f31, f28
	ctx.f[31].f64 = ctx.f[28].f64;
	// 82E4FA2C: 4198004C  blt cr6, 0x82e4fa78
	if ctx.cr[6].lt {
	pc = 0x82E4FA78; continue 'dispatch;
	}
	// 82E4FA30: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	pc = 0x82E4FA34; continue 'dispatch;
            }
            0x82E4FA34 => {
    //   block [0x82E4FA34..0x82E4FA4C)
	// 82E4FA34: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4FA38: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4FA3C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4FA40: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E4FA44: 409A0008  bne cr6, 0x82e4fa4c
	if !ctx.cr[6].eq {
	pc = 0x82E4FA4C; continue 'dispatch;
	}
	// 82E4FA48: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	pc = 0x82E4FA4C; continue 'dispatch;
            }
            0x82E4FA4C => {
    //   block [0x82E4FA4C..0x82E4FA78)
	// 82E4FA4C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82E4FA50: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E4FA54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4FA58: 4BFFFED9  bl 0x82e4f930
	ctx.lr = 0x82E4FA5C;
	sub_82E4F930(ctx, base);
	// 82E4FA5C: EC01F828  fsubs f0, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FA60: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E4FA64: EFA1E82A  fadds f29, f1, f29
	ctx.f[29].f64 = ((ctx.f[1].f64 + ctx.f[29].f64) as f32) as f64;
	// 82E4FA68: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E4FA6C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4FA70: FFE0F86E  fsel f31, f0, f1, f31
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 82E4FA74: 4098FFC0  bge cr6, 0x82e4fa34
	if !ctx.cr[6].lt {
	pc = 0x82E4FA34; continue 'dispatch;
	}
	pc = 0x82E4FA78; continue 'dispatch;
            }
            0x82E4FA78 => {
    //   block [0x82E4FA78..0x82E4FA90)
	// 82E4FA78: 897B00D8  lbz r11, 0xd8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4FA7C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E4FA80: 419A0010  beq cr6, 0x82e4fa90
	if ctx.cr[6].eq {
	pc = 0x82E4FA90; continue 'dispatch;
	}
	// 82E4FA84: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E4FA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4FA8C: 409A0008  bne cr6, 0x82e4fa94
	if !ctx.cr[6].eq {
	pc = 0x82E4FA94; continue 'dispatch;
	}
	pc = 0x82E4FA90; continue 'dispatch;
            }
            0x82E4FA90 => {
    //   block [0x82E4FA90..0x82E4FA94)
	// 82E4FA90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82E4FA94; continue 'dispatch;
            }
            0x82E4FA94 => {
    //   block [0x82E4FA94..0x82E4FAA0)
	// 82E4FA94: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E4FA98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4FA9C: 419A0044  beq cr6, 0x82e4fae0
	if ctx.cr[6].eq {
	pc = 0x82E4FAE0; continue 'dispatch;
	}
	pc = 0x82E4FAA0; continue 'dispatch;
            }
            0x82E4FAA0 => {
    //   block [0x82E4FAA0..0x82E4FACC)
	// 82E4FAA0: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E4FAA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4FAA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FAAC: 409A0020  bne cr6, 0x82e4facc
	if !ctx.cr[6].eq {
	pc = 0x82E4FACC; continue 'dispatch;
	}
	// 82E4FAB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FAB4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4FAB8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4FABC: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4FAC0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4FAC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4FAC8: 4BF05801  bl 0x82d552c8
	ctx.lr = 0x82E4FACC;
	sub_82D552C8(ctx, base);
	pc = 0x82E4FACC; continue 'dispatch;
            }
            0x82E4FACC => {
    //   block [0x82E4FACC..0x82E4FAE0)
	// 82E4FACC: FC20E090  fmr f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[28].f64;
	// 82E4FAD0: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E4FAD4: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82E4FAD8: 4BE5E24D  bl 0x82cadd24
	ctx.lr = 0x82E4FADC;
	sub_82CADCEC(ctx, base);
	// 82E4FADC: 4BE59978  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E4FAE0 => {
    //   block [0x82E4FAE0..0x82E4FBA0)
	// 82E4FAE0: 817B00D0  lwz r11, 0xd0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(208 as u32) ) } as u64;
	// 82E4FAE4: 387B00D0  addi r3, r27, 0xd0
	ctx.r[3].s64 = ctx.r[27].s64 + 208;
	// 82E4FAE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4FAEC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4FAF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FAF4: 4E800421  bctrl
	ctx.lr = 0x82E4FAF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FAF8: C0010064  lfs f0, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4FAFC: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4FB00: FF1DE000  fcmpu cr6, f29, f28
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[28].f64);
	// 82E4FB04: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E4FB08: FD6C682E  fsel f11, f12, f0, f13
	ctx.f[11].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82E4FB0C: C1810050  lfs f12, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4FB10: ED4C5828  fsubs f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E4FB14: FF8A5B2E  fsel f28, f10, f12, f11
	ctx.f[28].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[11].f64 };
	// 82E4FB18: 419AFF88  beq cr6, 0x82e4faa0
	if ctx.cr[6].eq {
	pc = 0x82E4FAA0; continue 'dispatch;
	}
	// 82E4FB1C: EFDC07B2  fmuls f30, f28, f30
	ctx.f[30].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E4FB20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4FB24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E4FB28: ED7DF028  fsubs f11, f29, f30
	ctx.f[11].f64 = (((ctx.f[29].f64 - ctx.f[30].f64) as f32) as f64);
	// 82E4FB2C: FD6BEFAE  fsel f11, f11, f30, f29
	ctx.f[11].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[29].f64 };
	// 82E4FB30: ED4BF828  fsubs f10, f11, f31
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB34: FFEAFAEE  fsel f31, f10, f11, f31
	ctx.f[31].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[31].f64 };
	// 82E4FB38: ED40F828  fsubs f10, f0, f31
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB3C: ED6CF828  fsubs f11, f12, f31
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB40: ED2DF828  fsubs f9, f13, f31
	ctx.f[9].f64 = (((ctx.f[13].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB44: FC0AF82E  fsel f0, f10, f0, f31
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 82E4FB48: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E4FB4C: FD8BFB2E  fsel f12, f11, f12, f31
	ctx.f[12].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[31].f64 };
	// 82E4FB50: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4FB54: FC09FB6E  fsel f0, f9, f13, f31
	ctx.f[0].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[31].f64 };
	// 82E4FB58: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E4FB5C: 4BF343A5  bl 0x82d83f00
	ctx.lr = 0x82E4FB60;
	sub_82D83F00(ctx, base);
	// 82E4FB60: EC1CE82A  fadds f0, f28, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[28].f64 + ctx.f[29].f64) as f32) as f64;
	// 82E4FB64: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E4FB68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4FB6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FB70: EDBE0028  fsubs f13, f30, f0
	ctx.f[13].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4FB74: FC0DF02E  fsel f0, f13, f0, f30
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[30].f64 };
	// 82E4FB78: EDA0F828  fsubs f13, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB7C: FFEDF82E  fsel f31, f13, f0, f31
	ctx.f[31].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 82E4FB80: 409A0020  bne cr6, 0x82e4fba0
	if !ctx.cr[6].eq {
	pc = 0x82E4FBA0; continue 'dispatch;
	}
	// 82E4FB84: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FB88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4FB8C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4FB90: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4FB94: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4FB98: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4FB9C: 4BF0572D  bl 0x82d552c8
	ctx.lr = 0x82E4FBA0;
	sub_82D552C8(ctx, base);
            }
            0x82E4FBA0 => {
    //   block [0x82E4FBA0..0x82E4FBB4)
	// 82E4FBA0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4FBA4: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E4FBA8: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82E4FBAC: 4BE5E179  bl 0x82cadd24
	ctx.lr = 0x82E4FBB0;
	sub_82CADCEC(ctx, base);
	// 82E4FBB0: 4BE598A4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FBB8 size=308
    let mut pc: u32 = 0x82E4FBB8;
    'dispatch: loop {
        match pc {
            0x82E4FBB8 => {
    //   block [0x82E4FBB8..0x82E4FC1C)
	// 82E4FBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FBBC: 4BE59845  bl 0x82ca9400
	ctx.lr = 0x82E4FBC0;
	sub_82CA93D0(ctx, base);
	// 82E4FBC0: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E4FBC4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4FBCC: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FBD0: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 82E4FBD4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4FBD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4FBDC: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82E4FBE0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82E4FBE4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4FBE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4FBEC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4FBF0: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4FBF4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4FBF8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E4FBFC: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82E4FC00: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4FC04: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E4FC08: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E4FC0C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E4FC10: 4199000C  bgt cr6, 0x82e4fc1c
	if ctx.cr[6].gt {
	pc = 0x82E4FC1C; continue 'dispatch;
	}
	// 82E4FC14: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E4FC18: 4800001C  b 0x82e4fc34
	pc = 0x82E4FC34; continue 'dispatch;
            }
            0x82E4FC1C => {
    //   block [0x82E4FC1C..0x82E4FC34)
	// 82E4FC1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FC20: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E4FC24: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4FC28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FC2C: 4E800421  bctrl
	ctx.lr = 0x82E4FC30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FC30: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
            }
            0x82E4FC34 => {
    //   block [0x82E4FC34..0x82E4FC58)
	// 82E4FC34: 7FEBF378  or r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 82E4FC38: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4FC3C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82E4FC40: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E4FC44: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E4FC48: 4099003C  ble cr6, 0x82e4fc84
	if !ctx.cr[6].gt {
	pc = 0x82E4FC84; continue 'dispatch;
	}
	// 82E4FC4C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4FC50: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E4FC54: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82E4FC58; continue 'dispatch;
            }
            0x82E4FC58 => {
    //   block [0x82E4FC58..0x82E4FC84)
	// 82E4FC58: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FC5C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4FC60: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4FC64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4FC68: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E4FC6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4FC70: 7D09392E  stwx r8, r9, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u32) };
	// 82E4FC74: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4FC78: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E4FC7C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E4FC80: 409AFFD8  bne cr6, 0x82e4fc58
	if !ctx.cr[6].eq {
	pc = 0x82E4FC58; continue 'dispatch;
	}
	pc = 0x82E4FC84; continue 'dispatch;
            }
            0x82E4FC84 => {
    //   block [0x82E4FC84..0x82E4FCBC)
	// 82E4FC84: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E4FC88: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4FC8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FC90: 4BFFFCA1  bl 0x82e4f930
	ctx.lr = 0x82E4FC94;
	sub_82E4F930(ctx, base);
	// 82E4FC94: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4FC98: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4FC9C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E4FCA0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82E4FCA4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E4FCA8: 409A0014  bne cr6, 0x82e4fcbc
	if !ctx.cr[6].eq {
	pc = 0x82E4FCBC; continue 'dispatch;
	}
	// 82E4FCAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FCB0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4FCB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FCB8: 4E800421  bctrl
	ctx.lr = 0x82E4FCBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82E4FCBC => {
    //   block [0x82E4FCBC..0x82E4FCE0)
	// 82E4FCBC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4FCC0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4FCC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FCC8: 409A0018  bne cr6, 0x82e4fce0
	if !ctx.cr[6].eq {
	pc = 0x82E4FCE0; continue 'dispatch;
	}
	// 82E4FCCC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4FCD0: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4FCD4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4FCD8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4FCDC: 4BF055ED  bl 0x82d552c8
	ctx.lr = 0x82E4FCE0;
	sub_82D552C8(ctx, base);
	pc = 0x82E4FCE0; continue 'dispatch;
            }
            0x82E4FCE0 => {
    //   block [0x82E4FCE0..0x82E4FCEC)
	// 82E4FCE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4FCE4: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4FCE8: 4BE59768  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4FCF0 size=2232
    let mut pc: u32 = 0x82E4FCF0;
    'dispatch: loop {
        match pc {
            0x82E4FCF0 => {
    //   block [0x82E4FCF0..0x82E505A8)
	// 82E4FCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FCF4: 4BE596F9  bl 0x82ca93ec
	ctx.lr = 0x82E4FCF8;
	sub_82CA93D0(ctx, base);
	// 82E4FCF8: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82E4FCFC: 4BE5DFC1  bl 0x82cadcbc
	ctx.lr = 0x82E4FD00;
	sub_82CADCA0(ctx, base);
	// 82E4FD00: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E505A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E505A8 size=1228
    let mut pc: u32 = 0x82E505A8;
    'dispatch: loop {
        match pc {
            0x82E505A8 => {
    //   block [0x82E505A8..0x82E50A74)
	// 82E505A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E505AC: 4BE58E49  bl 0x82ca93f4
	ctx.lr = 0x82E505B0;
	sub_82CA93D0(ctx, base);
	// 82E505B0: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82E505B4: 4BE5D71D  bl 0x82cadcd0
	ctx.lr = 0x82E505B8;
	sub_82CADCA0(ctx, base);
	// 82E505B8: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50A78 size=1704
    //   switch @ 0x82E50B14: r11 with 27 label(s)
    //       case  0 → 0x82E50E38
    //       case  1 → 0x82E50B84
    //       case  2 → 0x82E50EE4
    //       case  3 → 0x82E50C84
    //       case  4 → 0x82E50B9C
    //       case  5 → 0x82E50EC4
    //       case  6 → 0x82E50BC0
    //       case  7 → 0x82E50BC0
    //       case  8 → 0x82E50E38
    //       case  9 → 0x82E50E38
    //       case 10 → 0x82E50E38
    //       case 11 → 0x82E50CE4
    //       case 12 → 0x82E50D50
    //       case 13 → 0x82E51108
    //       case 14 → 0x82E51108
    //       case 15 → 0x82E51108
    //       case 16 → 0x82E50D98
    //       case 17 → 0x82E50E38
    //       case 18 → 0x82E50E38
    //       case 19 → 0x82E50E38
    //       case 20 → 0x82E51108
    //       case 21 → 0x82E51108
    //       case 22 → 0x82E51108
    //       case 23 → 0x82E51108
    //       case 24 → 0x82E50CD0
    //       case 25 → 0x82E51108
    //       case 26 → 0x82E50D74
    let mut pc: u32 = 0x82E50A78;
    'dispatch: loop {
        match pc {
            0x82E50A78 => {
    //   block [0x82E50A78..0x82E50B84)
	// 82E50A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50A7C: 4BE58979  bl 0x82ca93f4
	ctx.lr = 0x82E50A80;
	sub_82CA93D0(ctx, base);
	// 82E50A80: DBC1FFA0  stfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 82E50A84: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82E50A88: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82E50A8C: 481B5F29  bl 0x830069b4
	ctx.lr = 0x82E50A90;
	sub_83006760(ctx, base);
	// 82E50A90: 9421FA50  stwu r1, -0x5b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1456 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50A94: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82E50A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50A9C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E50AA0: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	pc = 0x82E50B84; continue 'dispatch;
            }
            0x82E50B84 => {
    //   block [0x82E50B84..0x82E50B9C)
	// 82E50B84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50B88: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50B8C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E50B90: C04B0C14  lfs f2, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E50B94: 4BFFBA55  bl 0x82e4c5e8
	ctx.lr = 0x82E50B98;
	sub_82E4C5E8(ctx, base);
	// 82E50B98: 48000374  b 0x82e50f0c
	pc = 0x82E50F0C; continue 'dispatch;
            }
            0x82E50B9C => {
    //   block [0x82E50B9C..0x82E50BC0)
	// 82E50B9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	pc = 0x82E50BC0; continue 'dispatch;
            }
            0x82E50BC0 => {
    //   block [0x82E50BC0..0x82E50C84)
	// 82E50BC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50BC4: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82E50BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50BCC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E50BD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50BD4: 4E800421  bctrl
	ctx.lr = 0x82E50BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50BD8: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50BDC: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82E50BE0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E50BE4: 39790001  addi r11, r25, 1
	ctx.r[11].s64 = ctx.r[25].s64 + 1;
	// 82E50BE8: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E50BEC: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E50BF0: 83A30020  lwz r29, 0x20(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E50BF4: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E50BF8: 7D7D2214  add r11, r29, r4
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[4].u64;
	// 82E50BFC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E50C00: 4199000C  bgt cr6, 0x82e50c0c
	if ctx.cr[6].gt {
	pc = 0x82E50C0C; continue 'dispatch;
	}
	// 82E50C04: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E50C08: 48000018  b 0x82e50c20
	pc = 0x82E50C20; continue 'dispatch;
	// 82E50C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50C10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50C14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50C18: 4E800421  bctrl
	ctx.lr = 0x82E50C1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50C1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E50C20: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E50C24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E50C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50C2C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82E50C30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50C34: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50C3C: 4E800421  bctrl
	ctx.lr = 0x82E50C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50C40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50C44: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82E50C48: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82E50C4C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E50C50: C02B0C14  lfs f1, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50C54: 4BFFDC5D  bl 0x82e4e8b0
	ctx.lr = 0x82E50C58;
	sub_82E4E8B0(ctx, base);
	// 82E50C58: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E50C5C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50C60: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82E50C64: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E50C68: 409A02A4  bne cr6, 0x82e50f0c
	if !ctx.cr[6].eq {
	pc = 0x82E50F0C; continue 'dispatch;
	}
	// 82E50C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50C70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E50C74: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50C7C: 4E800421  bctrl
	ctx.lr = 0x82E50C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50C80: 4800028C  b 0x82e50f0c
	pc = 0x82E50F0C; continue 'dispatch;
            }
            0x82E50C84 => {
    //   block [0x82E50C84..0x82E50CD0)
	// 82E50C84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	pc = 0x82E50CD0; continue 'dispatch;
            }
            0x82E50CD0 => {
    //   block [0x82E50CD0..0x82E50CE4)
	// 82E50CD0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50CD4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50CD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50CDC: 4BFFFD9D  bl 0x82e50a78
	ctx.lr = 0x82E50CE0;
	sub_82E50A78(ctx, base);
	// 82E50CE0: 48000428  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50CE4 => {
    //   block [0x82E50CE4..0x82E50D50)
	// 82E50CE4: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82E50CE8: 38A100E0  addi r5, r1, 0xe0
	ctx.r[5].s64 = ctx.r[1].s64 + 224;
	// 82E50CEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50CF0: 38610250  addi r3, r1, 0x250
	ctx.r[3].s64 = ctx.r[1].s64 + 592;
	pc = 0x82E50D50; continue 'dispatch;
            }
            0x82E50D50 => {
    //   block [0x82E50D50..0x82E50D74)
	// 82E50D50: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82E50D54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50D58: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 82E50D5C: 4BF065A5  bl 0x82d57300
	ctx.lr = 0x82E50D60;
	sub_82D57300(ctx, base);
	// 82E50D60: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50D64: 38810290  addi r4, r1, 0x290
	ctx.r[4].s64 = ctx.r[1].s64 + 656;
	// 82E50D68: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50D6C: 4BFFFD0D  bl 0x82e50a78
	ctx.lr = 0x82E50D70;
	sub_82E50A78(ctx, base);
	// 82E50D70: 48000398  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50D74 => {
    //   block [0x82E50D74..0x82E50D98)
	// 82E50D74: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82E50D78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50D7C: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 82E50D80: 4BF06581  bl 0x82d57300
	ctx.lr = 0x82E50D84;
	sub_82D57300(ctx, base);
	// 82E50D84: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50D88: 38810210  addi r4, r1, 0x210
	ctx.r[4].s64 = ctx.r[1].s64 + 528;
	// 82E50D8C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50D90: 4BFFFCE9  bl 0x82e50a78
	ctx.lr = 0x82E50D94;
	sub_82E50A78(ctx, base);
	// 82E50D94: 48000374  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50D98 => {
    //   block [0x82E50D98..0x82E50E38)
	// 82E50D98: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E50D9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E50DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50DA4: 40990364  ble cr6, 0x82e51108
	if !ctx.cr[6].gt {
	pc = 0x82E51108; continue 'dispatch;
	}
	// 82E50DA8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E50DAC: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82E50DB0: 3B5C0020  addi r26, r28, 0x20
	ctx.r[26].s64 = ctx.r[28].s64 + 32;
	// 82E50DB4: 3B3C0030  addi r25, r28, 0x30
	ctx.r[25].s64 = ctx.r[28].s64 + 48;
	// 82E50DB8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E50DBC: 3B0B39E0  addi r24, r11, 0x39e0
	ctx.r[24].s64 = ctx.r[11].s64 + 14816;
	// 82E50DC0: 39610120  addi r11, r1, 0x120
	ctx.r[11].s64 = ctx.r[1].s64 + 288;
	pc = 0x82E50E38; continue 'dispatch;
            }
            0x82E50E38 => {
    //   block [0x82E50E38..0x82E50EC4)
	// 82E50E38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50E40: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E48: 4E800421  bctrl
	ctx.lr = 0x82E50E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50E50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E54: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50E58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E5C: 4E800421  bctrl
	ctx.lr = 0x82E50E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50E64: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82E50E68: 419A02A0  beq cr6, 0x82e51108
	if ctx.cr[6].eq {
	pc = 0x82E51108; continue 'dispatch;
	}
	// 82E50E6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E70: 38A102D0  addi r5, r1, 0x2d0
	ctx.r[5].s64 = ctx.r[1].s64 + 720;
	// 82E50E74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E50E78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50E7C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50E80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E84: 4E800421  bctrl
	ctx.lr = 0x82E50E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50E8C: 419A0010  beq cr6, 0x82e50e9c
	if ctx.cr[6].eq {
	pc = 0x82E50E9C; continue 'dispatch;
	}
	// 82E50E90: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50E94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50E98: 4BFFFBE1  bl 0x82e50a78
	ctx.lr = 0x82E50E9C;
	sub_82E50A78(ctx, base);
	// 82E50E9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50EA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E50EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50EA8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E50EAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50EB0: 4E800421  bctrl
	ctx.lr = 0x82E50EB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50EB8: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82E50EBC: 409AFFB0  bne cr6, 0x82e50e6c
	if !ctx.cr[6].eq {
	pc = 0x82E50E6C; continue 'dispatch;
	}
	// 82E50EC0: 48000248  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50EC4 => {
    //   block [0x82E50EC4..0x82E50EE4)
	// 82E50EC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50EC8: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50ECC: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82E50ED0: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E50ED4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E50ED8: C04B0C14  lfs f2, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E50EDC: 4BFFDC75  bl 0x82e4eb50
	ctx.lr = 0x82E50EE0;
	sub_82E4EB50(ctx, base);
	// 82E50EE0: 4800002C  b 0x82e50f0c
	pc = 0x82E50F0C; continue 'dispatch;
            }
            0x82E50EE4 => {
    //   block [0x82E50EE4..0x82E51108)
	// 82E50EE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50EE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50EEC: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82E50EF0: C3EB0C14  lfs f31, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E50EF4: 4BF6C14D  bl 0x82dbd040
	ctx.lr = 0x82E50EF8;
	sub_82DBD040(ctx, base);
	// 82E50EF8: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E50EFC: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82E50F00: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E50F04: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E50F08: 4BFFE6C9  bl 0x82e4f5d0
	ctx.lr = 0x82E50F0C;
	sub_82E4F5D0(ctx, base);
	// 82E50F0C: C0210060  lfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50F10: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 82E50F14: 419A01F4  beq cr6, 0x82e51108
	if ctx.cr[6].eq {
	pc = 0x82E51108; continue 'dispatch;
	}
	// 82E50F18: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E50F1C: 4BFFB9ED  bl 0x82e4c908
	ctx.lr = 0x82E50F20;
	sub_82E4C908(ctx, base);
	// 82E50F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E50F24: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82E50F28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E50F2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E50F30: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E50F34: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E50F38: 480004E9  bl 0x82e51420
	ctx.lr = 0x82E50F3C;
	sub_82E51420(ctx, base);
	// 82E50F3C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E50F40: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82E50F44: C3F70000  lfs f31, 0(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E50F48: 39570020  addi r10, r23, 0x20
	ctx.r[10].s64 = ctx.r[23].s64 + 32;
	// 82E50F4C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E50F50: C3D70004  lfs f30, 4(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E50F54: 39610190  addi r11, r1, 0x190
	ctx.r[11].s64 = ctx.r[1].s64 + 400;
	// 82E50F58: D3E10180  stfs f31, 0x180(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82E50F5C: D3C10184  stfs f30, 0x184(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), tmp.u32 ) };
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E51108 => {
    //   block [0x82E51108..0x82E51120)
	// 82E51108: 382105B0  addi r1, r1, 0x5b0
	ctx.r[1].s64 = ctx.r[1].s64 + 1456;
	// 82E5110C: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82E51110: 481B5B3D  bl 0x83006c4c
	ctx.lr = 0x82E51114;
	sub_830069F8(ctx, base);
	// 82E51114: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82E51118: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82E5111C: 4BE58328  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51120 size=380
    let mut pc: u32 = 0x82E51120;
    'dispatch: loop {
        match pc {
            0x82E51120 => {
    //   block [0x82E51120..0x82E5129C)
	// 82E51120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5112C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E51130: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E51134: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51138: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E5113C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E51140: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E51144: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E51148: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E512A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E512A0 size=384
    let mut pc: u32 = 0x82E512A0;
    'dispatch: loop {
        match pc {
            0x82E512A0 => {
    //   block [0x82E512A0..0x82E51420)
	// 82E512A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E512A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E512A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E512AC: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E512B0: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E512B4: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E512B8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E512BC: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E512C0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E512C4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E512C8: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E51420 size=136
    let mut pc: u32 = 0x82E51420;
    'dispatch: loop {
        match pc {
            0x82E51420 => {
    //   block [0x82E51420..0x82E514A8)
	// 82E51420: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E51424: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82E51428: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82E5142C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E51430: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E51434: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51438: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E5143C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E51440: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E51444: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


