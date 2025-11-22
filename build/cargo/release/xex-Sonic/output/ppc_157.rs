pub fn sub_82BD4148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD4148 size=64
    let mut pc: u32 = 0x82BD4148;
    'dispatch: loop {
        match pc {
            0x82BD4148 => {
    //   block [0x82BD4148..0x82BD4188)
	// 82BD4148: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82BD414C: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD4150: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD4154: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4158: D0032E7C  stfs f0, 0x2e7c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11900 as u32), tmp.u32 ) };
	// 82BD415C: C1AB9594  lfs f13, -0x6a6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27244 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82BD4160: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4164: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82BD4168: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82BD416C: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82BD4170: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82BD4174: B1632968  sth r11, 0x2968(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10600 as u32), ctx.r[11].u16 ) };
	// 82BD4178: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD417C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD4180: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD4188 size=16
    let mut pc: u32 = 0x82BD4188;
    'dispatch: loop {
        match pc {
            0x82BD4188 => {
    //   block [0x82BD4188..0x82BD4198)
	// 82BD4188: C0032E7C  lfs f0, 0x2e7c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11900 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD418C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82BD4190: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82BD4194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4198 size=32
    let mut pc: u32 = 0x82BD4198;
    'dispatch: loop {
        match pc {
            0x82BD4198 => {
    //   block [0x82BD4198..0x82BD41B8)
	// 82BD4198: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD419C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD41A0: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82BD41A4: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD41A8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD41AC: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD41B0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD41B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD41B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD41B8 size=36
    let mut pc: u32 = 0x82BD41B8;
    'dispatch: loop {
        match pc {
            0x82BD41B8 => {
    //   block [0x82BD41B8..0x82BD41DC)
	// 82BD41B8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD41BC: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD41C0: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD41C4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD41C8: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD41CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD41D0: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD41D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD41D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD41E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD41E0 size=36
    let mut pc: u32 = 0x82BD41E0;
    'dispatch: loop {
        match pc {
            0x82BD41E0 => {
    //   block [0x82BD41E0..0x82BD4204)
	// 82BD41E0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD41E4: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD41E8: 556B0626  rlwinm r11, r11, 0, 0x18, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD41EC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD41F0: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD41F4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD41F8: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD41FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4208 size=36
    let mut pc: u32 = 0x82BD4208;
    'dispatch: loop {
        match pc {
            0x82BD4208 => {
    //   block [0x82BD4208..0x82BD422C)
	// 82BD4208: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD420C: 548A6026  slwi r10, r4, 0xc
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(12);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4210: 556B051E  rlwinm r11, r11, 0, 0x14, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4214: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4218: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD421C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4220: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD4224: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4230 size=36
    let mut pc: u32 = 0x82BD4230;
    'dispatch: loop {
        match pc {
            0x82BD4230 => {
    //   block [0x82BD4230..0x82BD4254)
	// 82BD4230: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD4234: 548A801E  slwi r10, r4, 0x10
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4238: 556B0416  rlwinm r11, r11, 0, 0x10, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD423C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4240: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD4244: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4248: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD424C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4258 size=36
    let mut pc: u32 = 0x82BD4258;
    'dispatch: loop {
        match pc {
            0x82BD4258 => {
    //   block [0x82BD4258..0x82BD427C)
	// 82BD4258: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD425C: 548AA016  slwi r10, r4, 0x14
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4260: 556B030E  rlwinm r11, r11, 0, 0xc, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4264: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4268: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD426C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4270: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD4274: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4280 size=36
    let mut pc: u32 = 0x82BD4280;
    'dispatch: loop {
        match pc {
            0x82BD4280 => {
    //   block [0x82BD4280..0x82BD42A4)
	// 82BD4280: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD4284: 548AC00E  slwi r10, r4, 0x18
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4288: 556B0206  rlwinm r11, r11, 0, 8, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD428C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4290: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD4294: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4298: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD429C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD42A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD42A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD42A8 size=28
    let mut pc: u32 = 0x82BD42A8;
    'dispatch: loop {
        match pc {
            0x82BD42A8 => {
    //   block [0x82BD42A8..0x82BD42C4)
	// 82BD42A8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD42AC: 508BE006  rlwimi r11, r4, 0x1c, 0, 3
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[11].u64 & 0xFFFFFFFF0FFFFFFF);
	// 82BD42B0: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82BD42B4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD42B8: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82BD42BC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD42C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD42C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD42C8 size=32
    let mut pc: u32 = 0x82BD42C8;
    'dispatch: loop {
        match pc {
            0x82BD42C8 => {
    //   block [0x82BD42C8..0x82BD42E8)
	// 82BD42C8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD42CC: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD42D0: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82BD42D4: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD42D8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD42DC: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD42E0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD42E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD42E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD42E8 size=36
    let mut pc: u32 = 0x82BD42E8;
    'dispatch: loop {
        match pc {
            0x82BD42E8 => {
    //   block [0x82BD42E8..0x82BD430C)
	// 82BD42E8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD42EC: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD42F0: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD42F4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD42F8: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD42FC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4300: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD4304: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4310 size=36
    let mut pc: u32 = 0x82BD4310;
    'dispatch: loop {
        match pc {
            0x82BD4310 => {
    //   block [0x82BD4310..0x82BD4334)
	// 82BD4310: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD4314: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4318: 556B0626  rlwinm r11, r11, 0, 0x18, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD431C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4320: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD4324: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4328: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD432C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4338 size=36
    let mut pc: u32 = 0x82BD4338;
    'dispatch: loop {
        match pc {
            0x82BD4338 => {
    //   block [0x82BD4338..0x82BD435C)
	// 82BD4338: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD433C: 548A6026  slwi r10, r4, 0xc
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(12);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4340: 556B051E  rlwinm r11, r11, 0, 0x14, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4344: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4348: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD434C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4350: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD4354: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4360 size=36
    let mut pc: u32 = 0x82BD4360;
    'dispatch: loop {
        match pc {
            0x82BD4360 => {
    //   block [0x82BD4360..0x82BD4384)
	// 82BD4360: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD4364: 548A801E  slwi r10, r4, 0x10
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4368: 556B0416  rlwinm r11, r11, 0, 0x10, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD436C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4370: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD4374: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4378: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD437C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4388 size=36
    let mut pc: u32 = 0x82BD4388;
    'dispatch: loop {
        match pc {
            0x82BD4388 => {
    //   block [0x82BD4388..0x82BD43AC)
	// 82BD4388: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD438C: 548AA016  slwi r10, r4, 0x14
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4390: 556B030E  rlwinm r11, r11, 0, 0xc, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4394: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4398: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD439C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD43A0: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD43A4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD43A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD43B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD43B0 size=36
    let mut pc: u32 = 0x82BD43B0;
    'dispatch: loop {
        match pc {
            0x82BD43B0 => {
    //   block [0x82BD43B0..0x82BD43D4)
	// 82BD43B0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD43B4: 548AC00E  slwi r10, r4, 0x18
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD43B8: 556B0206  rlwinm r11, r11, 0, 8, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD43BC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD43C0: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD43C4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD43C8: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD43CC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD43D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD43D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD43D8 size=28
    let mut pc: u32 = 0x82BD43D8;
    'dispatch: loop {
        match pc {
            0x82BD43D8 => {
    //   block [0x82BD43D8..0x82BD43F4)
	// 82BD43D8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD43DC: 508BE006  rlwimi r11, r4, 0x1c, 0, 3
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[11].u64 & 0xFFFFFFFF0FFFFFFF);
	// 82BD43E0: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82BD43E4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD43E8: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82BD43EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD43F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD43F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD43F8 size=12
    let mut pc: u32 = 0x82BD43F8;
    'dispatch: loop {
        match pc {
            0x82BD43F8 => {
    //   block [0x82BD43F8..0x82BD4404)
	// 82BD43F8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD43FC: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4408 size=12
    let mut pc: u32 = 0x82BD4408;
    'dispatch: loop {
        match pc {
            0x82BD4408 => {
    //   block [0x82BD4408..0x82BD4414)
	// 82BD4408: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD440C: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4418 size=12
    let mut pc: u32 = 0x82BD4418;
    'dispatch: loop {
        match pc {
            0x82BD4418 => {
    //   block [0x82BD4418..0x82BD4424)
	// 82BD4418: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD441C: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD4420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4428 size=12
    let mut pc: u32 = 0x82BD4428;
    'dispatch: loop {
        match pc {
            0x82BD4428 => {
    //   block [0x82BD4428..0x82BD4434)
	// 82BD4428: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD442C: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD4430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4438 size=12
    let mut pc: u32 = 0x82BD4438;
    'dispatch: loop {
        match pc {
            0x82BD4438 => {
    //   block [0x82BD4438..0x82BD4444)
	// 82BD4438: A163292C  lhz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD443C: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4448 size=12
    let mut pc: u32 = 0x82BD4448;
    'dispatch: loop {
        match pc {
            0x82BD4448 => {
    //   block [0x82BD4448..0x82BD4454)
	// 82BD4448: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD444C: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82BD4450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4458 size=12
    let mut pc: u32 = 0x82BD4458;
    'dispatch: loop {
        match pc {
            0x82BD4458 => {
    //   block [0x82BD4458..0x82BD4464)
	// 82BD4458: 8963292C  lbz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD445C: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4468 size=12
    let mut pc: u32 = 0x82BD4468;
    'dispatch: loop {
        match pc {
            0x82BD4468 => {
    //   block [0x82BD4468..0x82BD4474)
	// 82BD4468: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82BD446C: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82BD4470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4478 size=12
    let mut pc: u32 = 0x82BD4478;
    'dispatch: loop {
        match pc {
            0x82BD4478 => {
    //   block [0x82BD4478..0x82BD4484)
	// 82BD4478: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD447C: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4488 size=12
    let mut pc: u32 = 0x82BD4488;
    'dispatch: loop {
        match pc {
            0x82BD4488 => {
    //   block [0x82BD4488..0x82BD4494)
	// 82BD4488: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD448C: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4498 size=12
    let mut pc: u32 = 0x82BD4498;
    'dispatch: loop {
        match pc {
            0x82BD4498 => {
    //   block [0x82BD4498..0x82BD44A4)
	// 82BD4498: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD449C: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD44A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD44A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD44A8 size=12
    let mut pc: u32 = 0x82BD44A8;
    'dispatch: loop {
        match pc {
            0x82BD44A8 => {
    //   block [0x82BD44A8..0x82BD44B4)
	// 82BD44A8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD44AC: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD44B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD44B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD44B8 size=12
    let mut pc: u32 = 0x82BD44B8;
    'dispatch: loop {
        match pc {
            0x82BD44B8 => {
    //   block [0x82BD44B8..0x82BD44C4)
	// 82BD44B8: A1632930  lhz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD44BC: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD44C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD44C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD44C8 size=12
    let mut pc: u32 = 0x82BD44C8;
    'dispatch: loop {
        match pc {
            0x82BD44C8 => {
    //   block [0x82BD44C8..0x82BD44D4)
	// 82BD44C8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD44CC: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82BD44D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD44D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD44D8 size=12
    let mut pc: u32 = 0x82BD44D8;
    'dispatch: loop {
        match pc {
            0x82BD44D8 => {
    //   block [0x82BD44D8..0x82BD44E4)
	// 82BD44D8: 89632930  lbz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD44DC: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD44E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD44E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD44E8 size=12
    let mut pc: u32 = 0x82BD44E8;
    'dispatch: loop {
        match pc {
            0x82BD44E8 => {
    //   block [0x82BD44E8..0x82BD44F4)
	// 82BD44E8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82BD44EC: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82BD44F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD44F8 size=64
    let mut pc: u32 = 0x82BD44F8;
    'dispatch: loop {
        match pc {
            0x82BD44F8 => {
    //   block [0x82BD44F8..0x82BD4538)
	// 82BD44F8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD44FC: 3960043F  li r11, 0x43f
	ctx.r[11].s64 = 1087;
	// 82BD4500: 409A0008  bne cr6, 0x82bd4508
	if !ctx.cr[6].eq {
	pc = 0x82BD4508; continue 'dispatch;
	}
	// 82BD4504: 39600400  li r11, 0x400
	ctx.r[11].s64 = 1024;
	// 82BD4508: 9163294C  stw r11, 0x294c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10572 as u32), ctx.r[11].u32 ) };
	// 82BD450C: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82BD4510: 81432944  lwz r10, 0x2944(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 82BD4514: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD4518: 516A83DE  rlwimi r10, r11, 0x10, 0xf, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x0000000000010000) | (ctx.r[10].u64 & 0xFFFFFFFFFFFEFFFF);
	// 82BD451C: 91432944  stw r10, 0x2944(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10564 as u32), ctx.r[10].u32 ) };
	// 82BD4520: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4524: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82BD4528: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD452C: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82BD4530: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4538 size=12
    let mut pc: u32 = 0x82BD4538;
    'dispatch: loop {
        match pc {
            0x82BD4538 => {
    //   block [0x82BD4538..0x82BD4544)
	// 82BD4538: 8163294C  lwz r11, 0x294c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10572 as u32) ) } as u64;
	// 82BD453C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82BD4540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4548 size=16
    let mut pc: u32 = 0x82BD4548;
    'dispatch: loop {
        match pc {
            0x82BD4548 => {
    //   block [0x82BD4548..0x82BD4558)
	// 82BD4548: 81633098  lwz r11, 0x3098(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD454C: 90832EFC  stw r4, 0x2efc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12028 as u32), ctx.r[4].u32 ) };
	// 82BD4550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD4554: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4558 size=40
    let mut pc: u32 = 0x82BD4558;
    'dispatch: loop {
        match pc {
            0x82BD4558 => {
    //   block [0x82BD4558..0x82BD4580)
	// 82BD4558: 81032884  lwz r8, 0x2884(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10372 as u32) ) } as u64;
	// 82BD455C: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82BD4560: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82BD4564: 419A001C  beq cr6, 0x82bd4580
	if ctx.cr[6].eq {
		sub_82BD4580(ctx, base);
		return;
	}
	// 82BD4568: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82BD456C: 419A0014  beq cr6, 0x82bd4580
	if ctx.cr[6].eq {
		sub_82BD4580(ctx, base);
		return;
	}
	// 82BD4570: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82BD4574: 419A000C  beq cr6, 0x82bd4580
	if ctx.cr[6].eq {
		sub_82BD4580(ctx, base);
		return;
	}
	// 82BD4578: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82BD457C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4580 size=12
    let mut pc: u32 = 0x82BD4580;
    'dispatch: loop {
        match pc {
            0x82BD4580 => {
    //   block [0x82BD4580..0x82BD458C)
	// 82BD4580: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82BD4584: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD4588: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD458C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD458C size=76
    let mut pc: u32 = 0x82BD458C;
    'dispatch: loop {
        match pc {
            0x82BD458C => {
    //   block [0x82BD458C..0x82BD45D8)
	// 82BD458C: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4590: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82BD4594: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82BD4598: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82BD459C: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82BD45A0: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD45A4: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD45A8: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD45AC: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82BD45B0: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD45B4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD45B8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD45BC: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD45C0: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82BD45C4: 91632884  stw r11, 0x2884(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10372 as u32), ctx.r[11].u32 ) };
	// 82BD45C8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD45CC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD45D0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD45D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD45D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD45D8 size=8
    let mut pc: u32 = 0x82BD45D8;
    'dispatch: loop {
        match pc {
            0x82BD45D8 => {
    //   block [0x82BD45D8..0x82BD45E0)
	// 82BD45D8: 80632EFC  lwz r3, 0x2efc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12028 as u32) ) } as u64;
	// 82BD45DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD45E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD45E0 size=16
    let mut pc: u32 = 0x82BD45E0;
    'dispatch: loop {
        match pc {
            0x82BD45E0 => {
    //   block [0x82BD45E0..0x82BD45F0)
	// 82BD45E0: 8163309C  lwz r11, 0x309c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82BD45E4: 90832F00  stw r4, 0x2f00(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12032 as u32), ctx.r[4].u32 ) };
	// 82BD45E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD45EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD45F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD45F0 size=40
    let mut pc: u32 = 0x82BD45F0;
    'dispatch: loop {
        match pc {
            0x82BD45F0 => {
    //   block [0x82BD45F0..0x82BD4618)
	// 82BD45F0: 8103288C  lwz r8, 0x288c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10380 as u32) ) } as u64;
	// 82BD45F4: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82BD45F8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82BD45FC: 419A001C  beq cr6, 0x82bd4618
	if ctx.cr[6].eq {
		sub_82BD4618(ctx, base);
		return;
	}
	// 82BD4600: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82BD4604: 419A0014  beq cr6, 0x82bd4618
	if ctx.cr[6].eq {
		sub_82BD4618(ctx, base);
		return;
	}
	// 82BD4608: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82BD460C: 419A000C  beq cr6, 0x82bd4618
	if ctx.cr[6].eq {
		sub_82BD4618(ctx, base);
		return;
	}
	// 82BD4610: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82BD4614: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4618 size=12
    let mut pc: u32 = 0x82BD4618;
    'dispatch: loop {
        match pc {
            0x82BD4618 => {
    //   block [0x82BD4618..0x82BD4624)
	// 82BD4618: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82BD461C: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD4620: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4624(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4624 size=76
    let mut pc: u32 = 0x82BD4624;
    'dispatch: loop {
        match pc {
            0x82BD4624 => {
    //   block [0x82BD4624..0x82BD4670)
	// 82BD4624: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4628: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82BD462C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82BD4630: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82BD4634: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82BD4638: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD463C: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD4640: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD4644: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82BD4648: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD464C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4650: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4654: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4658: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82BD465C: 9163288C  stw r11, 0x288c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10380 as u32), ctx.r[11].u32 ) };
	// 82BD4660: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4664: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD4668: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD466C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4670 size=8
    let mut pc: u32 = 0x82BD4670;
    'dispatch: loop {
        match pc {
            0x82BD4670 => {
    //   block [0x82BD4670..0x82BD4678)
	// 82BD4670: 80632F00  lwz r3, 0x2f00(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12032 as u32) ) } as u64;
	// 82BD4674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4678 size=16
    let mut pc: u32 = 0x82BD4678;
    'dispatch: loop {
        match pc {
            0x82BD4678 => {
    //   block [0x82BD4678..0x82BD4688)
	// 82BD4678: 816330A0  lwz r11, 0x30a0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82BD467C: 90832F04  stw r4, 0x2f04(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12036 as u32), ctx.r[4].u32 ) };
	// 82BD4680: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD4684: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4688 size=40
    let mut pc: u32 = 0x82BD4688;
    'dispatch: loop {
        match pc {
            0x82BD4688 => {
    //   block [0x82BD4688..0x82BD46B0)
	// 82BD4688: 81032890  lwz r8, 0x2890(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10384 as u32) ) } as u64;
	// 82BD468C: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82BD4690: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82BD4694: 419A001C  beq cr6, 0x82bd46b0
	if ctx.cr[6].eq {
		sub_82BD46B0(ctx, base);
		return;
	}
	// 82BD4698: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82BD469C: 419A0014  beq cr6, 0x82bd46b0
	if ctx.cr[6].eq {
		sub_82BD46B0(ctx, base);
		return;
	}
	// 82BD46A0: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82BD46A4: 419A000C  beq cr6, 0x82bd46b0
	if ctx.cr[6].eq {
		sub_82BD46B0(ctx, base);
		return;
	}
	// 82BD46A8: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82BD46AC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD46B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD46B0 size=12
    let mut pc: u32 = 0x82BD46B0;
    'dispatch: loop {
        match pc {
            0x82BD46B0 => {
    //   block [0x82BD46B0..0x82BD46BC)
	// 82BD46B0: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82BD46B4: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD46B8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD46BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD46BC size=76
    let mut pc: u32 = 0x82BD46BC;
    'dispatch: loop {
        match pc {
            0x82BD46BC => {
    //   block [0x82BD46BC..0x82BD4708)
	// 82BD46BC: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD46C0: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82BD46C4: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82BD46C8: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82BD46CC: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82BD46D0: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD46D4: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD46D8: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD46DC: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82BD46E0: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD46E4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD46E8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD46EC: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 82BD46F0: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82BD46F4: 91632890  stw r11, 0x2890(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10384 as u32), ctx.r[11].u32 ) };
	// 82BD46F8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD46FC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD4700: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4708 size=8
    let mut pc: u32 = 0x82BD4708;
    'dispatch: loop {
        match pc {
            0x82BD4708 => {
    //   block [0x82BD4708..0x82BD4710)
	// 82BD4708: 80632F04  lwz r3, 0x2f04(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12036 as u32) ) } as u64;
	// 82BD470C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4710 size=16
    let mut pc: u32 = 0x82BD4710;
    'dispatch: loop {
        match pc {
            0x82BD4710 => {
    //   block [0x82BD4710..0x82BD4720)
	// 82BD4710: 816330A4  lwz r11, 0x30a4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82BD4714: 90832F08  stw r4, 0x2f08(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12040 as u32), ctx.r[4].u32 ) };
	// 82BD4718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD471C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4720 size=40
    let mut pc: u32 = 0x82BD4720;
    'dispatch: loop {
        match pc {
            0x82BD4720 => {
    //   block [0x82BD4720..0x82BD4748)
	// 82BD4720: 81032894  lwz r8, 0x2894(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10388 as u32) ) } as u64;
	// 82BD4724: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82BD4728: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82BD472C: 419A001C  beq cr6, 0x82bd4748
	if ctx.cr[6].eq {
		sub_82BD4748(ctx, base);
		return;
	}
	// 82BD4730: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82BD4734: 419A0014  beq cr6, 0x82bd4748
	if ctx.cr[6].eq {
		sub_82BD4748(ctx, base);
		return;
	}
	// 82BD4738: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82BD473C: 419A000C  beq cr6, 0x82bd4748
	if ctx.cr[6].eq {
		sub_82BD4748(ctx, base);
		return;
	}
	// 82BD4740: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82BD4744: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4748 size=12
    let mut pc: u32 = 0x82BD4748;
    'dispatch: loop {
        match pc {
            0x82BD4748 => {
    //   block [0x82BD4748..0x82BD4754)
	// 82BD4748: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82BD474C: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD4750: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4754 size=76
    let mut pc: u32 = 0x82BD4754;
    'dispatch: loop {
        match pc {
            0x82BD4754 => {
    //   block [0x82BD4754..0x82BD47A0)
	// 82BD4754: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4758: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82BD475C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82BD4760: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82BD4764: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82BD4768: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD476C: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD4770: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD4774: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82BD4778: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD477C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4780: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD4784: 798CA7E6  rldicr r12, r12, 0x34, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(52) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4788: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82BD478C: 91632894  stw r11, 0x2894(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10388 as u32), ctx.r[11].u32 ) };
	// 82BD4790: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4794: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD4798: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD479C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD47A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD47A0 size=8
    let mut pc: u32 = 0x82BD47A0;
    'dispatch: loop {
        match pc {
            0x82BD47A0 => {
    //   block [0x82BD47A0..0x82BD47A8)
	// 82BD47A0: 80632F08  lwz r3, 0x2f08(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12040 as u32) ) } as u64;
	// 82BD47A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD47A8 size=36
    let mut pc: u32 = 0x82BD47A8;
    'dispatch: loop {
        match pc {
            0x82BD47A8 => {
    //   block [0x82BD47A8..0x82BD47CC)
	// 82BD47A8: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD47AC: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD47B0: D0032980  stfs f0, 0x2980(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10624 as u32), tmp.u32 ) };
	// 82BD47B4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD47B8: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD47BC: 798C7FE6  rldicr r12, r12, 0x2f, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(47) & 0xFFFFFFFFFFFFFFFF;
	// 82BD47C0: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD47C4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD47C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD47D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD47D0 size=16
    let mut pc: u32 = 0x82BD47D0;
    'dispatch: loop {
        match pc {
            0x82BD47D0 => {
    //   block [0x82BD47D0..0x82BD47E0)
	// 82BD47D0: C0032980  lfs f0, 0x2980(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD47D4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82BD47D8: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82BD47DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD47E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD47E0 size=36
    let mut pc: u32 = 0x82BD47E0;
    'dispatch: loop {
        match pc {
            0x82BD47E0 => {
    //   block [0x82BD47E0..0x82BD4804)
	// 82BD47E0: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD47E4: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD47E8: D003297C  stfs f0, 0x297c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10620 as u32), tmp.u32 ) };
	// 82BD47EC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD47F0: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD47F4: 798C87E6  rldicr r12, r12, 0x30, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(48) & 0xFFFFFFFFFFFFFFFF;
	// 82BD47F8: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD47FC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD4808 size=16
    let mut pc: u32 = 0x82BD4808;
    'dispatch: loop {
        match pc {
            0x82BD4808 => {
    //   block [0x82BD4808..0x82BD4818)
	// 82BD4808: C003297C  lfs f0, 0x297c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10620 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD480C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82BD4810: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82BD4814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4818 size=36
    let mut pc: u32 = 0x82BD4818;
    'dispatch: loop {
        match pc {
            0x82BD4818 => {
    //   block [0x82BD4818..0x82BD483C)
	// 82BD4818: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 82BD481C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4820: 5164003A  rlwimi r4, r11, 0, 0, 0x1d
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[4].u64 & 0xFFFFFFFF00000003);
	// 82BD4824: 798C8FE6  rldicr r12, r12, 0x31, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(49) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4828: 90832978  stw r4, 0x2978(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10616 as u32), ctx.r[4].u32 ) };
	// 82BD482C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4830: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD4834: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4840 size=12
    let mut pc: u32 = 0x82BD4840;
    'dispatch: loop {
        match pc {
            0x82BD4840 => {
    //   block [0x82BD4840..0x82BD484C)
	// 82BD4840: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 82BD4844: 556307BE  clrlwi r3, r11, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD4848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4850 size=36
    let mut pc: u32 = 0x82BD4850;
    'dispatch: loop {
        match pc {
            0x82BD4850 => {
    //   block [0x82BD4850..0x82BD4874)
	// 82BD4850: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 82BD4854: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4858: 5164003C  rlwimi r4, r11, 0, 0, 0x1e
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFE) | (ctx.r[4].u64 & 0xFFFFFFFF00000001);
	// 82BD485C: 798C1FE6  rldicr r12, r12, 0x23, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(35) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4860: 908329C0  stw r4, 0x29c0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10688 as u32), ctx.r[4].u32 ) };
	// 82BD4864: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD4868: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD486C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD4870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4878 size=12
    let mut pc: u32 = 0x82BD4878;
    'dispatch: loop {
        match pc {
            0x82BD4878 => {
    //   block [0x82BD4878..0x82BD4884)
	// 82BD4878: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 82BD487C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82BD4880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4888 size=28
    let mut pc: u32 = 0x82BD4888;
    'dispatch: loop {
        match pc {
            0x82BD4888 => {
    //   block [0x82BD4888..0x82BD48A4)
	// 82BD4888: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82BD488C: 508BAA94  rlwimi r11, r4, 0x15, 0xa, 0xa
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(21) as u64) & 0x0000000000200000) | (ctx.r[11].u64 & 0xFFFFFFFFFFDFFFFF);
	// 82BD4890: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 82BD4894: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4898: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82BD489C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD48A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD48A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD48A8 size=12
    let mut pc: u32 = 0x82BD48A8;
    'dispatch: loop {
        match pc {
            0x82BD48A8 => {
    //   block [0x82BD48A8..0x82BD48B4)
	// 82BD48A8: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82BD48AC: 55635FFE  rlwinm r3, r11, 0xb, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 82BD48B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD48B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD48B8 size=28
    let mut pc: u32 = 0x82BD48B8;
    'dispatch: loop {
        match pc {
            0x82BD48B8 => {
    //   block [0x82BD48B8..0x82BD48D4)
	// 82BD48B8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD48BC: 908328D8  stw r4, 0x28d8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10456 as u32), ctx.r[4].u32 ) };
	// 82BD48C0: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD48C4: 798C37E6  rldicr r12, r12, 0x26, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(38) & 0xFFFFFFFFFFFFFFFF;
	// 82BD48C8: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD48CC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD48D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD48D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD48D8 size=8
    let mut pc: u32 = 0x82BD48D8;
    'dispatch: loop {
        match pc {
            0x82BD48D8 => {
    //   block [0x82BD48D8..0x82BD48E0)
	// 82BD48D8: 806328D8  lwz r3, 0x28d8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10456 as u32) ) } as u64;
	// 82BD48DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD48E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD48E0 size=28
    let mut pc: u32 = 0x82BD48E0;
    'dispatch: loop {
        match pc {
            0x82BD48E0 => {
    //   block [0x82BD48E0..0x82BD48FC)
	// 82BD48E0: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82BD48E4: 508B26F6  rlwimi r11, r4, 4, 0x1b, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x0000000000000010) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFEF);
	// 82BD48E8: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 82BD48EC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD48F0: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82BD48F4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD48F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4900 size=12
    let mut pc: u32 = 0x82BD4900;
    'dispatch: loop {
        match pc {
            0x82BD4900 => {
    //   block [0x82BD4900..0x82BD490C)
	// 82BD4900: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82BD4904: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD4908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4910 size=28
    let mut pc: u32 = 0x82BD4910;
    'dispatch: loop {
        match pc {
            0x82BD4910 => {
    //   block [0x82BD4910..0x82BD492C)
	// 82BD4910: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82BD4914: 508BC00E  rlwimi r11, r4, 0x18, 0, 7
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[11].u64 & 0xFFFFFFFF00FFFFFF);
	// 82BD4918: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 82BD491C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4920: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82BD4924: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4930 size=20
    let mut pc: u32 = 0x82BD4930;
    'dispatch: loop {
        match pc {
            0x82BD4930 => {
    //   block [0x82BD4930..0x82BD4944)
	// 82BD4930: 8963293C  lbz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82BD4934: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82BD4938: 514B063A  rlwimi r11, r10, 0, 0x18, 0x1d
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000000000FC) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF03);
	// 82BD493C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD4940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD4948 size=36
    let mut pc: u32 = 0x82BD4948;
    'dispatch: loop {
        match pc {
            0x82BD4948 => {
    //   block [0x82BD4948..0x82BD496C)
	// 82BD4948: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD494C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD4950: D00329CC  stfs f0, 0x29cc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10700 as u32), tmp.u32 ) };
	// 82BD4954: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4958: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD495C: 798C07E6  rldicr r12, r12, 0x20, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4960: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD4964: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD4968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4970 size=8
    let mut pc: u32 = 0x82BD4970;
    'dispatch: loop {
        match pc {
            0x82BD4970 => {
    //   block [0x82BD4970..0x82BD4978)
	// 82BD4970: 806329CC  lwz r3, 0x29cc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10700 as u32) ) } as u64;
	// 82BD4974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD4978 size=36
    let mut pc: u32 = 0x82BD4978;
    'dispatch: loop {
        match pc {
            0x82BD4978 => {
    //   block [0x82BD4978..0x82BD499C)
	// 82BD4978: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD497C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD4980: D00329C4  stfs f0, 0x29c4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10692 as u32), tmp.u32 ) };
	// 82BD4984: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD4988: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD498C: 798C17E6  rldicr r12, r12, 0x22, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(34) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4990: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD4994: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD4998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD49A0 size=8
    let mut pc: u32 = 0x82BD49A0;
    'dispatch: loop {
        match pc {
            0x82BD49A0 => {
    //   block [0x82BD49A0..0x82BD49A8)
	// 82BD49A0: 806329C4  lwz r3, 0x29c4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10692 as u32) ) } as u64;
	// 82BD49A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD49A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD49A8 size=28
    let mut pc: u32 = 0x82BD49A8;
    'dispatch: loop {
        match pc {
            0x82BD49A8 => {
    //   block [0x82BD49A8..0x82BD49C4)
	// 82BD49A8: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD49AC: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD49B0: D00329D0  stfs f0, 0x29d0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10704 as u32), tmp.u32 ) };
	// 82BD49B4: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD49B8: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82BD49BC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD49C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD49C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD49C8 size=8
    let mut pc: u32 = 0x82BD49C8;
    'dispatch: loop {
        match pc {
            0x82BD49C8 => {
    //   block [0x82BD49C8..0x82BD49D0)
	// 82BD49C8: 806329D0  lwz r3, 0x29d0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10704 as u32) ) } as u64;
	// 82BD49CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD49D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD49D0 size=36
    let mut pc: u32 = 0x82BD49D0;
    'dispatch: loop {
        match pc {
            0x82BD49D0 => {
    //   block [0x82BD49D0..0x82BD49F4)
	// 82BD49D0: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82BD49D4: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD49D8: D00329C8  stfs f0, 0x29c8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10696 as u32), tmp.u32 ) };
	// 82BD49DC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD49E0: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD49E4: 798C0FE6  rldicr r12, r12, 0x21, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(33) & 0xFFFFFFFFFFFFFFFF;
	// 82BD49E8: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD49EC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD49F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD49F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD49F8 size=8
    let mut pc: u32 = 0x82BD49F8;
    'dispatch: loop {
        match pc {
            0x82BD49F8 => {
    //   block [0x82BD49F8..0x82BD4A00)
	// 82BD49F8: 806329C8  lwz r3, 0x29c8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10696 as u32) ) } as u64;
	// 82BD49FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4A00 size=28
    let mut pc: u32 = 0x82BD4A00;
    'dispatch: loop {
        match pc {
            0x82BD4A00 => {
    //   block [0x82BD4A00..0x82BD4A1C)
	// 82BD4A00: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82BD4A04: 508BA256  rlwimi r11, r4, 0x14, 9, 0xb
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(20) as u64) & 0x0000000000700000) | (ctx.r[11].u64 & 0xFFFFFFFFFF8FFFFF);
	// 82BD4A08: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82BD4A0C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4A10: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82BD4A14: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4A20 size=12
    let mut pc: u32 = 0x82BD4A20;
    'dispatch: loop {
        match pc {
            0x82BD4A20 => {
    //   block [0x82BD4A20..0x82BD4A2C)
	// 82BD4A20: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82BD4A24: 5563677E  rlwinm r3, r11, 0xc, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82BD4A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4A30 size=56
    let mut pc: u32 = 0x82BD4A30;
    'dispatch: loop {
        match pc {
            0x82BD4A30 => {
    //   block [0x82BD4A30..0x82BD4A68)
	// 82BD4A30: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82BD4A34: 89432ABF  lbz r10, 0x2abf(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10943 as u32) ) } as u64;
	// 82BD4A38: 508B8B1C  rlwimi r11, r4, 0x11, 0xc, 0xe
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(17) as u64) & 0x00000000000E0000) | (ctx.r[11].u64 & 0xFFFFFFFFFFF1FFFF);
	// 82BD4A3C: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD4A40: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82BD4A44: 40820024  bne 0x82bd4a68
	if !ctx.cr[0].eq {
		sub_82BD4A68(ctx, base);
		return;
	}
	// 82BD4A48: 556B031C  rlwinm r11, r11, 0, 0xc, 0xe
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4A4C: 3D400004  lis r10, 4
	ctx.r[10].s64 = 262144;
	// 82BD4A50: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD4A54: 409A0014  bne cr6, 0x82bd4a68
	if !ctx.cr[6].eq {
		sub_82BD4A68(ctx, base);
		return;
	}
	// 82BD4A58: E9630028  ld r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 82BD4A5C: 3980FEFF  li r12, -0x101
	ctx.r[12].s64 = -257;
	// 82BD4A60: 7D6B6038  and r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[12].u64;
	// 82BD4A64: 4800000C  b 0x82bd4a70
	sub_82BD4A68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4A68 size=28
    let mut pc: u32 = 0x82BD4A68;
    'dispatch: loop {
        match pc {
            0x82BD4A68 => {
    //   block [0x82BD4A68..0x82BD4A84)
	// 82BD4A68: E9630028  ld r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 82BD4A6C: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD4A70: F9630028  std r11, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 82BD4A74: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4A78: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD4A7C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4A88 size=12
    let mut pc: u32 = 0x82BD4A88;
    'dispatch: loop {
        match pc {
            0x82BD4A88 => {
    //   block [0x82BD4A88..0x82BD4A94)
	// 82BD4A88: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82BD4A8C: 55637F7E  rlwinm r3, r11, 0xf, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 82BD4A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4A98 size=28
    let mut pc: u32 = 0x82BD4A98;
    'dispatch: loop {
        match pc {
            0x82BD4A98 => {
    //   block [0x82BD4A98..0x82BD4AB4)
	// 82BD4A98: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD4A9C: 508B1F38  rlwimi r11, r4, 3, 0x1c, 0x1c
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x0000000000000008) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFF7);
	// 82BD4AA0: 91632940  stw r11, 0x2940(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82BD4AA4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4AA8: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD4AAC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4AB8 size=12
    let mut pc: u32 = 0x82BD4AB8;
    'dispatch: loop {
        match pc {
            0x82BD4AB8 => {
    //   block [0x82BD4AB8..0x82BD4AC4)
	// 82BD4AB8: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD4ABC: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD4AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4AC8 size=28
    let mut pc: u32 = 0x82BD4AC8;
    'dispatch: loop {
        match pc {
            0x82BD4AC8 => {
    //   block [0x82BD4AC8..0x82BD4AE4)
	// 82BD4AC8: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD4ACC: 508B177A  rlwimi r11, r4, 2, 0x1d, 0x1d
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(2) as u64) & 0x0000000000000004) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82BD4AD0: 91632940  stw r11, 0x2940(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82BD4AD4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4AD8: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD4ADC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4AE8 size=12
    let mut pc: u32 = 0x82BD4AE8;
    'dispatch: loop {
        match pc {
            0x82BD4AE8 => {
    //   block [0x82BD4AE8..0x82BD4AF4)
	// 82BD4AE8: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD4AEC: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD4AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4AF8 size=28
    let mut pc: u32 = 0x82BD4AF8;
    'dispatch: loop {
        match pc {
            0x82BD4AF8 => {
    //   block [0x82BD4AF8..0x82BD4B14)
	// 82BD4AF8: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD4AFC: 508B2EB4  rlwimi r11, r4, 5, 0x1a, 0x1a
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(5) as u64) & 0x0000000000000020) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFDF);
	// 82BD4B00: 91632940  stw r11, 0x2940(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82BD4B04: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4B08: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD4B0C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B18 size=12
    let mut pc: u32 = 0x82BD4B18;
    'dispatch: loop {
        match pc {
            0x82BD4B18 => {
    //   block [0x82BD4B18..0x82BD4B24)
	// 82BD4B18: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD4B1C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD4B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B28 size=20
    let mut pc: u32 = 0x82BD4B28;
    'dispatch: loop {
        match pc {
            0x82BD4B28 => {
    //   block [0x82BD4B28..0x82BD4B3C)
	// 82BD4B28: 98832942  stb r4, 0x2942(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10562 as u32), ctx.r[4].u8 ) };
	// 82BD4B2C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD4B30: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD4B34: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD4B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B40 size=8
    let mut pc: u32 = 0x82BD4B40;
    'dispatch: loop {
        match pc {
            0x82BD4B40 => {
    //   block [0x82BD4B40..0x82BD4B48)
	// 82BD4B40: 88632942  lbz r3, 0x2942(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10562 as u32) ) } as u64;
	// 82BD4B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B48 size=8
    let mut pc: u32 = 0x82BD4B48;
    'dispatch: loop {
        match pc {
            0x82BD4B48 => {
    //   block [0x82BD4B48..0x82BD4B50)
	// 82BD4B48: 9083351C  stw r4, 0x351c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13596 as u32), ctx.r[4].u32 ) };
	// 82BD4B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B50 size=8
    let mut pc: u32 = 0x82BD4B50;
    'dispatch: loop {
        match pc {
            0x82BD4B50 => {
    //   block [0x82BD4B50..0x82BD4B58)
	// 82BD4B50: 8063351C  lwz r3, 0x351c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13596 as u32) ) } as u64;
	// 82BD4B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B58 size=16
    let mut pc: u32 = 0x82BD4B58;
    'dispatch: loop {
        match pc {
            0x82BD4B58 => {
    //   block [0x82BD4B58..0x82BD4B68)
	// 82BD4B58: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82BD4B5C: 508BB890  rlwimi r11, r4, 0x17, 2, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x000000003F800000) | (ctx.r[11].u64 & 0xFFFFFFFFC07FFFFF);
	// 82BD4B60: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82BD4B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B68 size=12
    let mut pc: u32 = 0x82BD4B68;
    'dispatch: loop {
        match pc {
            0x82BD4B68 => {
    //   block [0x82BD4B68..0x82BD4B74)
	// 82BD4B68: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82BD4B6C: 55634E7E  rlwinm r3, r11, 9, 0x19, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82BD4B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4B78 size=200
    let mut pc: u32 = 0x82BD4B78;
    'dispatch: loop {
        match pc {
            0x82BD4B78 => {
    //   block [0x82BD4B78..0x82BD4C40)
	// 82BD4B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD4B7C: 485D35F1  bl 0x831a816c
	ctx.lr = 0x82BD4B80;
	sub_831A8130(ctx, base);
	// 82BD4B80: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD4B84: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4B88: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD4B8C: 892A2E94  lbz r9, 0x2e94(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(11924 as u32) ) } as u64;
	// 82BD4B90: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4B94: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82BD4B98: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82BD4B9C: 3908C288  addi r8, r8, -0x3d78
	ctx.r[8].s64 = ctx.r[8].s64 + -15736;
	// 82BD4BA0: 54A7F0BE  srwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD4BA4: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4BA8: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82BD4BAC: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4BB0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82BD4BB4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82BD4BB8: 54C6B7FE  rlwinm r6, r6, 0x16, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x000003FFu64;
	// 82BD4BBC: 7D09402E  lwzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD4BC0: 50FD5D28  rlwimi r29, r7, 0xb, 0x14, 0x14
	ctx.r[29].u64 = (((ctx.r[7].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[29].u64 & 0xFFFFFFFFFFFFF7FF);
	// 82BD4BC4: 7CC93B78  or r9, r6, r7
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[7].u64;
	// 82BD4BC8: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82BD4BCC: 7BC6FFE6  rldicr r6, r30, 0x3f, 0x3f
	ctx.r[6].u64 = (ctx.r[30].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4BD0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82BD4BD4: 7D094878  andc r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 & !ctx.r[9].u64;
	// 82BD4BD8: 78880020  clrldi r8, r4, 0x20
	ctx.r[8].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4BDC: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4BE0: 7CC84436  srd r8, r6, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = (ctx.r[6].u64) >> ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 82BD4BE4: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD4BE8: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82BD4BEC: 513FAA54  rlwimi r31, r9, 0x15, 9, 0xa
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(21) as u64) & 0x0000000000600000) | (ctx.r[31].u64 & 0xFFFFFFFFFF9FFFFF);
	// 82BD4BF0: 513FA90C  rlwimi r31, r9, 0x15, 4, 6
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(21) as u64) & 0x000000000E000000) | (ctx.r[31].u64 & 0xFFFFFFFFF1FFFFFF);
	// 82BD4BF4: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82BD4BF8: 892A2EE2  lbz r9, 0x2ee2(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4BFC: 57EA003E  slwi r10, r31, 0
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4C00: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82BD4C04: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82BD4C08: 552AF0BE  srwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4C0C: 50E6FB7E  rlwimi r6, r7, 0x1f, 0xd, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFF80000);
	// 82BD4C10: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82BD4C14: 50E6F856  rlwimi r6, r7, 0x1f, 1, 0xb
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[6].u64 & 0xFFFFFFFF800FFFFF);
	// 82BD4C18: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82BD4C1C: 54C76D3E  rlwinm r7, r6, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x0007FFFFu64;
	// 82BD4C20: 7CEA5038  and r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82BD4C24: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82BD4C28: 53AA003A  rlwimi r10, r29, 0, 0, 0x1d
	ctx.r[10].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[10].u64 & 0xFFFFFFFF00000003);
	// 82BD4C2C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82BD4C30: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4C34: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82BD4C38: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4C3C: 485D3580  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4C40 size=52
    let mut pc: u32 = 0x82BD4C40;
    'dispatch: loop {
        match pc {
            0x82BD4C40 => {
    //   block [0x82BD4C40..0x82BD4C74)
	// 82BD4C40: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD4C44: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD4C48: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4C4C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4C50: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4C54: 554BAFFE  rlwinm r11, r10, 0x15, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000007FFu64;
	// 82BD4C58: 552A5D7E  srwi r10, r9, 0x15
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(21);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4C5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82BD4C60: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82BD4C64: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82BD4C68: 516A077A  rlwimi r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x0000000000000004) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82BD4C6C: 5543077E  clrlwi r3, r10, 0x1d
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82BD4C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4C78 size=148
    let mut pc: u32 = 0x82BD4C78;
    'dispatch: loop {
        match pc {
            0x82BD4C78 => {
    //   block [0x82BD4C78..0x82BD4D0C)
	// 82BD4C78: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82BD4C7C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82BD4C80: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4C84: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82BD4C88: 54A8083C  slwi r8, r5, 1
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD4C8C: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82BD4C90: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4C94: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD4C98: 552907FA  rlwinm r9, r9, 0, 0x1f, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4C9C: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82BD4CA0: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82BD4CA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82BD4CA8: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4CAC: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4CB0: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4CB4: 78A5FFE6  rldicr r5, r5, 0x3f, 0x3f
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4CB8: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82BD4CBC: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82BD4CC0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82BD4CC4: 53FEFB7E  rlwimi r30, r31, 0x1f, 0xd, 0x1f
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[30].u64 & 0xFFFFFFFFFFF80000);
	// 82BD4CC8: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82BD4CCC: 53FEF856  rlwimi r30, r31, 0x1f, 1, 0xb
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[30].u64 & 0xFFFFFFFF800FFFFF);
	// 82BD4CD0: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4CD4: 57DF6D3E  rlwinm r31, r30, 0xd, 0x14, 0x1f
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x0007FFFFu64;
	// 82BD4CD8: 7CA73C36  srd r7, r5, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[5].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82BD4CDC: 7FE94838  and r9, r31, r9
	ctx.r[9].u64 = ctx.r[31].u64 & ctx.r[9].u64;
	// 82BD4CE0: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82BD4CE4: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82BD4CE8: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82BD4CEC: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82BD4CF0: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82BD4CF4: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4CF8: 7CEB5B78  or r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 82BD4CFC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4D00: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD4D04: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82BD4D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4D10 size=16
    let mut pc: u32 = 0x82BD4D10;
    'dispatch: loop {
        match pc {
            0x82BD4D10 => {
    //   block [0x82BD4D10..0x82BD4D20)
	// 82BD4D10: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4D14: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4D18: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82BD4D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4D20 size=200
    let mut pc: u32 = 0x82BD4D20;
    'dispatch: loop {
        match pc {
            0x82BD4D20 => {
    //   block [0x82BD4D20..0x82BD4DE8)
	// 82BD4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD4D24: 485D3449  bl 0x831a816c
	ctx.lr = 0x82BD4D28;
	sub_831A8130(ctx, base);
	// 82BD4D28: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD4D2C: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4D30: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD4D34: 892A2E94  lbz r9, 0x2e94(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(11924 as u32) ) } as u64;
	// 82BD4D38: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4D3C: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82BD4D40: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82BD4D44: 3908C288  addi r8, r8, -0x3d78
	ctx.r[8].s64 = ctx.r[8].s64 + -15736;
	// 82BD4D48: 54A7F0BE  srwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD4D4C: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4D50: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82BD4D54: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4D58: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82BD4D5C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82BD4D60: 54C6AFFE  rlwinm r6, r6, 0x15, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x000007FFu64;
	// 82BD4D64: 7D09402E  lwzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD4D68: 50FD556A  rlwimi r29, r7, 0xa, 0x15, 0x15
	ctx.r[29].u64 = (((ctx.r[7].u32).rotate_left(10) as u64) & 0x0000000000000400) | (ctx.r[29].u64 & 0xFFFFFFFFFFFFFBFF);
	// 82BD4D6C: 7CC93B78  or r9, r6, r7
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[7].u64;
	// 82BD4D70: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82BD4D74: 7BC6FFE6  rldicr r6, r30, 0x3f, 0x3f
	ctx.r[6].u64 = (ctx.r[30].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4D78: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82BD4D7C: 7D094878  andc r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 & !ctx.r[9].u64;
	// 82BD4D80: 78880020  clrldi r8, r4, 0x20
	ctx.r[8].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4D84: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4D88: 7CC84436  srd r8, r6, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = (ctx.r[6].u64) >> ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 82BD4D8C: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD4D90: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82BD4D94: 513F9AD8  rlwimi r31, r9, 0x13, 0xb, 0xc
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(19) as u64) & 0x0000000000180000) | (ctx.r[31].u64 & 0xFFFFFFFFFFE7FFFF);
	// 82BD4D98: 513F990C  rlwimi r31, r9, 0x13, 4, 6
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(19) as u64) & 0x000000000E000000) | (ctx.r[31].u64 & 0xFFFFFFFFF1FFFFFF);
	// 82BD4D9C: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82BD4DA0: 892A2EE2  lbz r9, 0x2ee2(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4DA4: 57EA003E  slwi r10, r31, 0
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4DA8: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82BD4DAC: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82BD4DB0: 552AF0BE  srwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4DB4: 50E6FB7E  rlwimi r6, r7, 0x1f, 0xd, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFF80000);
	// 82BD4DB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82BD4DBC: 50E6F856  rlwimi r6, r7, 0x1f, 1, 0xb
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[6].u64 & 0xFFFFFFFF800FFFFF);
	// 82BD4DC0: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82BD4DC4: 54C76D3E  rlwinm r7, r6, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x0007FFFFu64;
	// 82BD4DC8: 7CEA5038  and r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82BD4DCC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82BD4DD0: 53AA003A  rlwimi r10, r29, 0, 0, 0x1d
	ctx.r[10].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[10].u64 & 0xFFFFFFFF00000003);
	// 82BD4DD4: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82BD4DD8: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4DDC: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82BD4DE0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4DE4: 485D33D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4DE8 size=52
    let mut pc: u32 = 0x82BD4DE8;
    'dispatch: loop {
        match pc {
            0x82BD4DE8 => {
    //   block [0x82BD4DE8..0x82BD4E1C)
	// 82BD4DE8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD4DEC: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD4DF0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4DF4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4DF8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4DFC: 554BB7FE  rlwinm r11, r10, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000003FFu64;
	// 82BD4E00: 552A6CFE  srwi r10, r9, 0x13
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD4E04: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82BD4E08: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82BD4E0C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82BD4E10: 516A077A  rlwimi r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x0000000000000004) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82BD4E14: 5543077E  clrlwi r3, r10, 0x1d
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82BD4E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4E20 size=140
    let mut pc: u32 = 0x82BD4E20;
    'dispatch: loop {
        match pc {
            0x82BD4E20 => {
    //   block [0x82BD4E20..0x82BD4EAC)
	// 82BD4E20: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82BD4E24: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82BD4E28: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4E2C: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82BD4E30: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82BD4E34: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82BD4E38: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4E3C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD4E40: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4E44: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD4E48: 7D262B78  or r6, r9, r5
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82BD4E4C: 791FFFE6  rldicr r31, r8, 0x3f, 0x3f
	ctx.r[31].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4E50: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4E54: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4E58: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4E5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82BD4E60: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82BD4E64: 53C5FB7E  rlwimi r5, r30, 0x1f, 0xd, 0x1f
	ctx.r[5].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFF80000);
	// 82BD4E68: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82BD4E6C: 53C5F856  rlwimi r5, r30, 0x1f, 1, 0xb
	ctx.r[5].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[5].u64 & 0xFFFFFFFF800FFFFF);
	// 82BD4E70: 78FE0020  clrldi r30, r7, 0x20
	ctx.r[30].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4E74: 54A76D3E  rlwinm r7, r5, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x0007FFFFu64;
	// 82BD4E78: 7FE5F436  srd r5, r31, r30
	if (ctx.r[30].u8 & 0x40) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = (ctx.r[31].u64) >> ((ctx.r[30].u8 & 0x3F) as u32);
	}
	// 82BD4E7C: 7CE74838  and r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82BD4E80: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82BD4E84: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82BD4E88: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82BD4E8C: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82BD4E90: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82BD4E94: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4E98: 7CAB5B78  or r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82BD4E9C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4EA0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD4EA4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82BD4EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4EB0 size=16
    let mut pc: u32 = 0x82BD4EB0;
    'dispatch: loop {
        match pc {
            0x82BD4EB0 => {
    //   block [0x82BD4EB0..0x82BD4EC0)
	// 82BD4EB0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4EB4: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4EB8: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82BD4EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4EC0 size=60
    let mut pc: u32 = 0x82BD4EC0;
    'dispatch: loop {
        match pc {
            0x82BD4EC0 => {
    //   block [0x82BD4EC0..0x82BD4EFC)
	// 82BD4EC0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD4EC4: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD4EC8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD4ECC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4ED0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD4ED4: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4ED8: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4EDC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4EE0: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD4EE4: 50A8B9D0  rlwimi r8, r5, 0x17, 7, 8
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(23) as u64) & 0x0000000001800000) | (ctx.r[8].u64 & 0xFFFFFFFFFE7FFFFF);
	// 82BD4EE8: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82BD4EEC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4EF0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD4EF4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4F00 size=20
    let mut pc: u32 = 0x82BD4F00;
    'dispatch: loop {
        match pc {
            0x82BD4F00 => {
    //   block [0x82BD4F00..0x82BD4F14)
	// 82BD4F00: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD4F04: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4F08: 816B048C  lwz r11, 0x48c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1164 as u32) ) } as u64;
	// 82BD4F0C: 55634FBE  rlwinm r3, r11, 9, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82BD4F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4F18 size=148
    let mut pc: u32 = 0x82BD4F18;
    'dispatch: loop {
        match pc {
            0x82BD4F18 => {
    //   block [0x82BD4F18..0x82BD4FAC)
	// 82BD4F18: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82BD4F1C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82BD4F20: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4F24: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82BD4F28: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD4F2C: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82BD4F30: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4F34: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD4F38: 552907B8  rlwinm r9, r9, 0, 0x1e, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD4F3C: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82BD4F40: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82BD4F44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82BD4F48: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4F4C: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD4F50: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4F54: 78A5FFE6  rldicr r5, r5, 0x3f, 0x3f
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD4F58: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82BD4F5C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82BD4F60: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82BD4F64: 53FEFB7E  rlwimi r30, r31, 0x1f, 0xd, 0x1f
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[30].u64 & 0xFFFFFFFFFFF80000);
	// 82BD4F68: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82BD4F6C: 53FEF856  rlwimi r30, r31, 0x1f, 1, 0xb
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[30].u64 & 0xFFFFFFFF800FFFFF);
	// 82BD4F70: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4F74: 57DF6D3E  rlwinm r31, r30, 0xd, 0x14, 0x1f
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x0007FFFFu64;
	// 82BD4F78: 7CA73C36  srd r7, r5, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[5].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82BD4F7C: 7FE94838  and r9, r31, r9
	ctx.r[9].u64 = ctx.r[31].u64 & ctx.r[9].u64;
	// 82BD4F80: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82BD4F84: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82BD4F88: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82BD4F8C: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82BD4F90: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82BD4F94: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD4F98: 7CEB5B78  or r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 82BD4F9C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD4FA0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD4FA4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82BD4FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4FB0 size=16
    let mut pc: u32 = 0x82BD4FB0;
    'dispatch: loop {
        match pc {
            0x82BD4FB0 => {
    //   block [0x82BD4FB0..0x82BD4FC0)
	// 82BD4FB0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD4FB4: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82BD4FB8: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD4FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD4FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD4FC0 size=104
    let mut pc: u32 = 0x82BD4FC0;
    'dispatch: loop {
        match pc {
            0x82BD4FC0 => {
    //   block [0x82BD4FC0..0x82BD5028)
	// 82BD4FC0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD4FC4: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD4FC8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD4FCC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD4FD0: 55490529  rlwinm. r9, r10, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD4FD4: 4082000C  bne 0x82bd4fe0
	if !ctx.cr[0].eq {
	pc = 0x82BD4FE0; continue 'dispatch;
	}
	// 82BD4FD8: 554A056B  rlwinm. r10, r10, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD4FDC: 41820040  beq 0x82bd501c
	if ctx.cr[0].eq {
	pc = 0x82BD501C; continue 'dispatch;
	}
	// 82BD4FE0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82BD4FE4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD4FE8: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD4FEC: 394AC288  addi r10, r10, -0x3d78
	ctx.r[10].s64 = ctx.r[10].s64 + -15736;
	// 82BD4FF0: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82BD4FF4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82BD4FF8: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82BD4FFC: 78C6FFE6  rldicr r6, r6, 0x3f, 0x3f
	ctx.r[6].u64 = (ctx.r[6].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5000: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82BD5004: 7CC83C36  srd r8, r6, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = (ctx.r[6].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82BD5008: 5149C90C  rlwimi r9, r10, 0x19, 4, 6
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(25) as u64) & 0x000000000E000000) | (ctx.r[9].u64 & 0xFFFFFFFFF1FFFFFF);
	// 82BD500C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82BD5010: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5014: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82BD5018: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD501C: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD5020: 98AB2E94  stb r5, 0x2e94(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11924 as u32), ctx.r[5].u8 ) };
	// 82BD5024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5028 size=12
    let mut pc: u32 = 0x82BD5028;
    'dispatch: loop {
        match pc {
            0x82BD5028 => {
    //   block [0x82BD5028..0x82BD5034)
	// 82BD5028: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD502C: 886B2E94  lbz r3, 0x2e94(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11924 as u32) ) } as u64;
	// 82BD5030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD5038 size=92
    let mut pc: u32 = 0x82BD5038;
    'dispatch: loop {
        match pc {
            0x82BD5038 => {
    //   block [0x82BD5038..0x82BD5094)
	// 82BD5038: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82BD503C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82BD5040: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD5044: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82BD5048: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD504C: C00ADB98  lfs f0, -0x2468(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-9320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD5050: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5054: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD5058: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82BD505C: 794AFFE6  rldicr r10, r10, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5060: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82BD5064: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82BD5068: C1A10024  lfs f13, 0x24(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82BD506C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82BD5070: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82BD5074: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82BD5078: 8121FFF4  lwz r9, -0xc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82BD507C: 51282DF4  rlwimi r8, r9, 5, 0x17, 0x1a
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(5) as u64) & 0x00000000000001E0) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFE1F);
	// 82BD5080: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82BD5084: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5088: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD508C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD5098 size=68
    let mut pc: u32 = 0x82BD5098;
    'dispatch: loop {
        match pc {
            0x82BD5098 => {
    //   block [0x82BD5098..0x82BD50DC)
	// 82BD5098: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD509C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD50A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82BD50A4: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82BD50A8: C00AB41C  lfs f0, -0x4be4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19428 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD50AC: 556BB810  slwi r11, r11, 0x17
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(23);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD50B0: 7D6BE670  srawi r11, r11, 0x1c
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 28) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 28) as i64;
	// 82BD50B4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82BD50B8: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82BD50BC: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD50C0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82BD50C4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82BD50C8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82BD50CC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82BD50D0: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82BD50D4: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82BD50D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD50E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD50E0 size=92
    let mut pc: u32 = 0x82BD50E0;
    'dispatch: loop {
        match pc {
            0x82BD50E0 => {
    //   block [0x82BD50E0..0x82BD513C)
	// 82BD50E0: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82BD50E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82BD50E8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD50EC: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82BD50F0: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD50F4: C00AE25C  lfs f0, -0x1da4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7588 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD50F8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD50FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD5100: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5104: 794AFFE6  rldicr r10, r10, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5108: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD510C: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82BD5110: C1A10024  lfs f13, 0x24(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82BD5114: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82BD5118: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82BD511C: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82BD5120: 8121FFF4  lwz r9, -0xc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82BD5124: 512862A6  rlwimi r8, r9, 0xc, 0xa, 0x13
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(12) as u64) & 0x00000000003FF000) | (ctx.r[8].u64 & 0xFFFFFFFFFFC00FFF);
	// 82BD5128: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82BD512C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5130: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD5134: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD5140 size=64
    let mut pc: u32 = 0x82BD5140;
    'dispatch: loop {
        match pc {
            0x82BD5140 => {
    //   block [0x82BD5140..0x82BD5180)
	// 82BD5140: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD5144: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5148: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82BD514C: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82BD5150: C00A4934  lfs f0, 0x4934(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(18740 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD5154: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5158: 7D6BB670  srawi r11, r11, 0x16
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 22) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 22) as i64;
	// 82BD515C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82BD5160: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82BD5164: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5168: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82BD516C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82BD5170: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82BD5174: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82BD5178: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82BD517C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5180 size=108
    let mut pc: u32 = 0x82BD5180;
    'dispatch: loop {
        match pc {
            0x82BD5180 => {
    //   block [0x82BD5180..0x82BD51EC)
	// 82BD5180: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 82BD5184: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5188: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD518C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5190: 419A0050  beq cr6, 0x82bd51e0
	if ctx.cr[6].eq {
	pc = 0x82BD51E0; continue 'dispatch;
	}
	// 82BD5194: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82BD5198: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82BD519C: 556BF73E  rlwinm r11, r11, 0x1e, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD51A0: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82BD51A4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD51A8: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82BD51AC: 41990008  bgt cr6, 0x82bd51b4
	if ctx.cr[6].gt {
	pc = 0x82BD51B4; continue 'dispatch;
	}
	// 82BD51B0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82BD51B4: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD51B8: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
	// 82BD51BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82BD51C0: 516916BA  rlwimi r9, r11, 2, 0x1a, 0x1d
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(2) as u64) & 0x000000000000003C) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFC3);
	// 82BD51C4: 78EBFFE6  rldicr r11, r7, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[7].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD51C8: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82BD51CC: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82BD51D0: 7D6B5436  srd r11, r11, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD51D4: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD51D8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD51DC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD51E0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD51E4: 98AB2EAE  stb r5, 0x2eae(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11950 as u32), ctx.r[5].u8 ) };
	// 82BD51E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD51F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD51F0 size=12
    let mut pc: u32 = 0x82BD51F0;
    'dispatch: loop {
        match pc {
            0x82BD51F0 => {
    //   block [0x82BD51F0..0x82BD51FC)
	// 82BD51F0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD51F4: 886B2EAE  lbz r3, 0x2eae(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11950 as u32) ) } as u64;
	// 82BD51F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5200 size=108
    let mut pc: u32 = 0x82BD5200;
    'dispatch: loop {
        match pc {
            0x82BD5200 => {
    //   block [0x82BD5200..0x82BD526C)
	// 82BD5200: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 82BD5204: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5208: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD520C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5210: 419A0050  beq cr6, 0x82bd5260
	if ctx.cr[6].eq {
	pc = 0x82BD5260; continue 'dispatch;
	}
	// 82BD5214: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82BD5218: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82BD521C: 556BD73E  rlwinm r11, r11, 0x1a, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82BD5220: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82BD5224: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD5228: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82BD522C: 41980008  blt cr6, 0x82bd5234
	if ctx.cr[6].lt {
	pc = 0x82BD5234; continue 'dispatch;
	}
	// 82BD5230: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82BD5234: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD5238: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
	// 82BD523C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82BD5240: 516935B2  rlwimi r9, r11, 6, 0x16, 0x19
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(6) as u64) & 0x00000000000003C0) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFC3F);
	// 82BD5244: 78EBFFE6  rldicr r11, r7, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[7].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5248: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82BD524C: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5250: 7D6B5436  srd r11, r11, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD5254: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5258: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD525C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5260: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD5264: 98AB2EC8  stb r5, 0x2ec8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11976 as u32), ctx.r[5].u8 ) };
	// 82BD5268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5270 size=12
    let mut pc: u32 = 0x82BD5270;
    'dispatch: loop {
        match pc {
            0x82BD5270 => {
    //   block [0x82BD5270..0x82BD527C)
	// 82BD5270: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82BD5274: 886B2EC8  lbz r3, 0x2ec8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11976 as u32) ) } as u64;
	// 82BD5278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5280 size=76
    let mut pc: u32 = 0x82BD5280;
    'dispatch: loop {
        match pc {
            0x82BD5280 => {
    //   block [0x82BD5280..0x82BD52CC)
	// 82BD5280: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD5284: 7CAA0034  cntlzw r10, r5
	ctx.r[10].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 82BD5288: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD528C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5290: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD5294: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82BD5298: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD529C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82BD52A0: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82BD52A4: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82BD52A8: 7908FFE6  rldicr r8, r8, 0x3f, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD52AC: 54E7003A  rlwinm r7, r7, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD52B0: 7D094C36  srd r9, r8, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = (ctx.r[8].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82BD52B4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82BD52B8: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82BD52BC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD52C0: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82BD52C4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD52C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD52D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD52D0 size=28
    let mut pc: u32 = 0x82BD52D0;
    'dispatch: loop {
        match pc {
            0x82BD52D0 => {
    //   block [0x82BD52D0..0x82BD52EC)
	// 82BD52D0: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD52D4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD52D8: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82BD52DC: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD52E0: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82BD52E4: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82BD52E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD52F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD52F0 size=56
    let mut pc: u32 = 0x82BD52F0;
    'dispatch: loop {
        match pc {
            0x82BD52F0 => {
    //   block [0x82BD52F0..0x82BD5328)
	// 82BD52F0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD52F4: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD52F8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD52FC: 7D2B182E  lwzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD5300: 50A954EA  rlwimi r9, r5, 0xa, 0x13, 0x15
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(10) as u64) & 0x0000000000001C00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFE3FF);
	// 82BD5304: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD5308: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD530C: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5310: 790AFFE6  rldicr r10, r8, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5314: 7D4B5C36  srd r11, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[10].u64) >> ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82BD5318: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD531C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD5320: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5328 size=20
    let mut pc: u32 = 0x82BD5328;
    'dispatch: loop {
        match pc {
            0x82BD5328 => {
    //   block [0x82BD5328..0x82BD533C)
	// 82BD5328: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD532C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD5330: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD5334: 5563B77E  rlwinm r3, r11, 0x16, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82BD5338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5340 size=56
    let mut pc: u32 = 0x82BD5340;
    'dispatch: loop {
        match pc {
            0x82BD5340 => {
    //   block [0x82BD5340..0x82BD5378)
	// 82BD5340: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD5344: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD5348: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD534C: 7D2B182E  lwzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD5350: 50A96C24  rlwimi r9, r5, 0xd, 0x10, 0x12
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(13) as u64) & 0x000000000000E000) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF1FFF);
	// 82BD5354: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD5358: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD535C: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5360: 790AFFE6  rldicr r10, r8, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5364: 7D4B5C36  srd r11, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[10].u64) >> ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82BD5368: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD536C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD5370: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5378 size=20
    let mut pc: u32 = 0x82BD5378;
    'dispatch: loop {
        match pc {
            0x82BD5378 => {
    //   block [0x82BD5378..0x82BD538C)
	// 82BD5378: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD537C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD5380: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD5384: 55639F7E  rlwinm r3, r11, 0x13, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 82BD5388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5390 size=56
    let mut pc: u32 = 0x82BD5390;
    'dispatch: loop {
        match pc {
            0x82BD5390 => {
    //   block [0x82BD5390..0x82BD53C8)
	// 82BD5390: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD5394: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD5398: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD539C: 7D2B182E  lwzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD53A0: 50A9835E  rlwimi r9, r5, 0x10, 0xd, 0xf
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x0000000000070000) | (ctx.r[9].u64 & 0xFFFFFFFFFFF8FFFF);
	// 82BD53A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD53A8: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD53AC: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD53B0: 790AFFE6  rldicr r10, r8, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD53B4: 7D4B5C36  srd r11, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[10].u64) >> ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82BD53B8: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD53BC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD53C0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD53C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD53C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD53C8 size=20
    let mut pc: u32 = 0x82BD53C8;
    'dispatch: loop {
        match pc {
            0x82BD53C8 => {
    //   block [0x82BD53C8..0x82BD53DC)
	// 82BD53C8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD53CC: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD53D0: 7D6B1A2E  lhzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD53D4: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD53D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD53E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD53E0 size=60
    let mut pc: u32 = 0x82BD53E0;
    'dispatch: loop {
        match pc {
            0x82BD53E0 => {
    //   block [0x82BD53E0..0x82BD541C)
	// 82BD53E0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD53E4: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD53E8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD53EC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD53F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD53F4: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD53F8: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD53FC: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82BD5400: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD5404: 50A81EF8  rlwimi r8, r5, 3, 0x1b, 0x1c
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(3) as u64) & 0x0000000000000018) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFE7);
	// 82BD5408: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82BD540C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5410: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD5414: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5420 size=20
    let mut pc: u32 = 0x82BD5420;
    'dispatch: loop {
        match pc {
            0x82BD5420 => {
    //   block [0x82BD5420..0x82BD5434)
	// 82BD5420: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD5424: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5428: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82BD542C: 5563EFBE  rlwinm r3, r11, 0x1d, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD5430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5438 size=60
    let mut pc: u32 = 0x82BD5438;
    'dispatch: loop {
        match pc {
            0x82BD5438 => {
    //   block [0x82BD5438..0x82BD5474)
	// 82BD5438: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD543C: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD5440: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD5444: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5448: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD544C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5450: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5454: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD5458: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD545C: 50A8B152  rlwimi r8, r5, 0x16, 5, 9
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(22) as u64) & 0x0000000007C00000) | (ctx.r[8].u64 & 0xFFFFFFFFF83FFFFF);
	// 82BD5460: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82BD5464: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5468: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD546C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5478 size=24
    let mut pc: u32 = 0x82BD5478;
    'dispatch: loop {
        match pc {
            0x82BD5478 => {
    //   block [0x82BD5478..0x82BD5490)
	// 82BD5478: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD547C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5480: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82BD5484: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5488: 7D63DE70  srawi r3, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82BD548C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5490 size=60
    let mut pc: u32 = 0x82BD5490;
    'dispatch: loop {
        match pc {
            0x82BD5490 => {
    //   block [0x82BD5490..0x82BD54CC)
	// 82BD5490: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD5494: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD5498: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD549C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD54A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD54A4: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD54A8: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD54AC: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD54B0: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD54B4: 50A8D808  rlwimi r8, r5, 0x1b, 0, 4
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(27) as u64) & 0x00000000F8000000) | (ctx.r[8].u64 & 0xFFFFFFFF07FFFFFF);
	// 82BD54B8: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82BD54BC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD54C0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD54C4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD54C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD54D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD54D0 size=20
    let mut pc: u32 = 0x82BD54D0;
    'dispatch: loop {
        match pc {
            0x82BD54D0 => {
    //   block [0x82BD54D0..0x82BD54E4)
	// 82BD54D0: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD54D4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD54D8: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82BD54DC: 7D63DE70  srawi r3, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82BD54E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD54E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD54E8 size=60
    let mut pc: u32 = 0x82BD54E8;
    'dispatch: loop {
        match pc {
            0x82BD54E8 => {
    //   block [0x82BD54E8..0x82BD5524)
	// 82BD54E8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD54EC: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82BD54F0: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD54F4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD54F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD54FC: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5500: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5504: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82BD5508: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD550C: 50A8177A  rlwimi r8, r5, 2, 0x1d, 0x1d
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(2) as u64) & 0x0000000000000004) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82BD5510: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82BD5514: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5518: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD551C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5528 size=20
    let mut pc: u32 = 0x82BD5528;
    'dispatch: loop {
        match pc {
            0x82BD5528 => {
    //   block [0x82BD5528..0x82BD553C)
	// 82BD5528: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD552C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5530: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82BD5534: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD5538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5540 size=68
    let mut pc: u32 = 0x82BD5540;
    'dispatch: loop {
        match pc {
            0x82BD5540 => {
    //   block [0x82BD5540..0x82BD5584)
	// 82BD5540: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82BD5544: 7CAA0034  cntlzw r10, r5
	ctx.r[10].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 82BD5548: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82BD554C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5550: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD5554: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82BD5558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD555C: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5560: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD5564: 7908FFE6  rldicr r8, r8, 0x3f, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5568: 51475D28  rlwimi r7, r10, 0xb, 0x14, 0x14
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[7].u64 & 0xFFFFFFFFFFFFF7FF);
	// 82BD556C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82BD5570: 7D0B4C36  srd r11, r8, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[8].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82BD5574: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD5578: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD557C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5588 size=24
    let mut pc: u32 = 0x82BD5588;
    'dispatch: loop {
        match pc {
            0x82BD5588 => {
    //   block [0x82BD5588..0x82BD55A0)
	// 82BD5588: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82BD558C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5590: 816B0484  lwz r11, 0x484(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1156 as u32) ) } as u64;
	// 82BD5594: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82BD5598: 5563AFFE  rlwinm r3, r11, 0x15, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 82BD559C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD55A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82BD55A0 size=248
    let mut pc: u32 = 0x82BD55A0;
    'dispatch: loop {
        match pc {
            0x82BD55A0 => {
    //   block [0x82BD55A0..0x82BD5698)
	// 82BD55A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD55A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD55A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD55AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD55B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD55B4: C0033168  lfs f0, 0x3168(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12648 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD55B8: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82BD55BC: C1A33170  lfs f13, 0x3170(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12656 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82BD55C0: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82BD55C4: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82BD55C8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82BD55CC: D9A10058  stfd f13, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[13].u64 ) };
	// 82BD55D0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82BD55D4: C0033174  lfs f0, 0x3174(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12660 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD55D8: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82BD55DC: C1A3316C  lfs f13, 0x316c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12652 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82BD55E0: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82BD55E4: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82BD55E8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD55EC: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD55F0: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82BD55F4: 83E90008  lwz r31, 8(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD55F8: D9A10050  stfd f13, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[13].u64 ) };
	// 82BD55FC: 83C9000C  lwz r30, 0xc(r9)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD5600: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82BD5604: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82BD5608: 80C32E50  lwz r6, 0x2e50(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11856 as u32) ) } as u64;
	// 82BD560C: 91633184  stw r11, 0x3184(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12676 as u32), ctx.r[11].u32 ) };
	// 82BD5610: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82BD5614: 91433188  stw r10, 0x3188(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12680 as u32), ctx.r[10].u32 ) };
	// 82BD5618: 7CC82214  add r6, r8, r4
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82BD561C: 93E3318C  stw r31, 0x318c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12684 as u32), ctx.r[31].u32 ) };
	// 82BD5620: 93C33190  stw r30, 0x3190(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12688 as u32), ctx.r[30].u32 ) };
	// 82BD5624: 7CE92A14  add r7, r9, r5
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 82BD5628: 419A0034  beq cr6, 0x82bd565c
	if ctx.cr[6].eq {
	pc = 0x82BD565C; continue 'dispatch;
	}
	// 82BD562C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82BD5630: 41990008  bgt cr6, 0x82bd5638
	if ctx.cr[6].gt {
	pc = 0x82BD5638; continue 'dispatch;
	}
	// 82BD5634: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82BD5638: 7F055000  cmpw cr6, r5, r10
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82BD563C: 41990008  bgt cr6, 0x82bd5644
	if ctx.cr[6].gt {
	pc = 0x82BD5644; continue 'dispatch;
	}
	// 82BD5640: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82BD5644: 7F06F800  cmpw cr6, r6, r31
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82BD5648: 41980008  blt cr6, 0x82bd5650
	if ctx.cr[6].lt {
	pc = 0x82BD5650; continue 'dispatch;
	}
	// 82BD564C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82BD5650: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82BD5654: 41980008  blt cr6, 0x82bd565c
	if ctx.cr[6].lt {
	pc = 0x82BD565C; continue 'dispatch;
	}
	// 82BD5658: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82BD565C: 816328C4  lwz r11, 0x28c4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10436 as u32) ) } as u64;
	// 82BD5660: 814328C8  lwz r10, 0x28c8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10440 as u32) ) } as u64;
	// 82BD5664: 50AB805E  rlwimi r11, r5, 0x10, 1, 0xf
	ctx.r[11].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000007FFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF8000FFFF);
	// 82BD5668: 50EA805E  rlwimi r10, r7, 0x10, 1, 0xf
	ctx.r[10].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000007FFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF8000FFFF);
	// 82BD566C: 508B047E  rlwimi r11, r4, 0, 0x11, 0x1f
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x0000000000007FFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF8000);
	// 82BD5670: 50CA047E  rlwimi r10, r6, 0, 0x11, 0x1f
	ctx.r[10].u64 = (((ctx.r[6].u32).rotate_left(0) as u64) & 0x0000000000007FFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF8000);
	// 82BD5674: 916328C4  stw r11, 0x28c4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10436 as u32), ctx.r[11].u32 ) };
	// 82BD5678: 914328C8  stw r10, 0x28c8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10440 as u32), ctx.r[10].u32 ) };
	// 82BD567C: 4BFFD965  bl 0x82bd2fe0
	ctx.lr = 0x82BD5680;
	sub_82BD2FE0(ctx, base);
	// 82BD5680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82BD5684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD568C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD5690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD5698 size=80
    let mut pc: u32 = 0x82BD5698;
    'dispatch: loop {
        match pc {
            0x82BD5698 => {
    //   block [0x82BD5698..0x82BD56E8)
	// 82BD5698: C0033168  lfs f0, 0x3168(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12648 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD569C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82BD56A0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82BD56A4: 7C0027AE  stfiwx f0, 0, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32, tmp.u32) };
	// 82BD56A8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82BD56AC: C003316C  lfs f0, 0x316c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12652 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD56B0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82BD56B4: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD56B8: 7C045FAE  stfiwx f0, r4, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82BD56BC: C0033170  lfs f0, 0x3170(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12656 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD56C0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82BD56C4: 7C0457AE  stfiwx f0, r4, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82BD56C8: C0033174  lfs f0, 0x3174(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12660 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD56CC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82BD56D0: 7C044FAE  stfiwx f0, r4, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82BD56D4: C0033178  lfs f0, 0x3178(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12664 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD56D8: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82BD56DC: C003317C  lfs f0, 0x317c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12668 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD56E0: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82BD56E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD56E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD56E8 size=288
    let mut pc: u32 = 0x82BD56E8;
    'dispatch: loop {
        match pc {
            0x82BD56E8 => {
    //   block [0x82BD56E8..0x82BD5808)
	// 82BD56E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD56EC: 485D2A75  bl 0x831a8160
	ctx.lr = 0x82BD56F0;
	sub_831A8130(ctx, base);
	// 82BD56F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD56F4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82BD56F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD56FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82BD5700: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82BD5704: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD5708: 419A0050  beq cr6, 0x82bd5758
	if ctx.cr[6].eq {
	pc = 0x82BD5758; continue 'dispatch;
	}
	// 82BD570C: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82BD5710: 217D0011  subfic r11, r29, 0x11
	ctx.xer.ca = ctx.r[29].u32 <= 17 as u32;
	ctx.r[11].s64 = (17 as i64) - ctx.r[29].s64;
	// 82BD5714: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82BD5718: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82BD571C: 388B00DE  addi r4, r11, 0xde
	ctx.r[4].s64 = ctx.r[11].s64 + 222;
	// 82BD5720: 5547653E  srwi r7, r10, 0x14
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(20);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD5724: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD5728: 39670200  addi r11, r7, 0x200
	ctx.r[11].s64 = ctx.r[7].s64 + 512;
	// 82BD572C: 554A00FE  clrlwi r10, r10, 3
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 82BD5730: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD5734: 54871838  slwi r7, r4, 3
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD5738: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82BD573C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD5740: 7CC62850  subf r6, r6, r5
	ctx.r[6].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82BD5744: 7D67F92E  stwx r11, r7, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82BD5748: 90C906F4  stw r6, 0x6f4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1780 as u32), ctx.r[6].u32 ) };
	// 82BD574C: E97F0018  ld r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 82BD5750: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82BD5754: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5758: 397D0C2B  addi r11, r29, 0xc2b
	ctx.r[11].s64 = ctx.r[29].s64 + 3115;
	// 82BD575C: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82BD5760: 7F9BF82E  lwzx r28, r27, r31
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD5764: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82BD5768: 419A0068  beq cr6, 0x82bd57d0
	if ctx.cr[6].eq {
	pc = 0x82BD57D0; continue 'dispatch;
	}
	// 82BD576C: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 82BD5770: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5774: 419A000C  beq cr6, 0x82bd5780
	if ctx.cr[6].eq {
	pc = 0x82BD5780; continue 'dispatch;
	}
	// 82BD5778: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD577C: 48000054  b 0x82bd57d0
	pc = 0x82BD57D0; continue 'dispatch;
	// 82BD5780: 817F2AA0  lwz r11, 0x2aa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10912 as u32) ) } as u64;
	// 82BD5784: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD5788: 7D6B5039  and. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD578C: 41820044  beq 0x82bd57d0
	if ctx.cr[0].eq {
	pc = 0x82BD57D0; continue 'dispatch;
	}
	// 82BD5790: 817F34D8  lwz r11, 0x34d8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13528 as u32) ) } as u64;
	// 82BD5794: 807F34D4  lwz r3, 0x34d4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13524 as u32) ) } as u64;
	// 82BD5798: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD579C: 4198000C  blt cr6, 0x82bd57a8
	if ctx.cr[6].lt {
	pc = 0x82BD57A8; continue 'dispatch;
	}
	// 82BD57A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD57A4: 4801A35D  bl 0x82befb00
	ctx.lr = 0x82BD57A8;
	sub_82BEFB00(ctx, base);
	// 82BD57A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD57AC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82BD57B0: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82BD57B4: 538BF0BE  rlwimi r11, r28, 0x1e, 2, 0x1f
	ctx.r[11].u64 = (((ctx.r[28].u32).rotate_left(30) as u64) & 0x000000003FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFFC0000000);
	// 82BD57B8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82BD57BC: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD57C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82BD57C4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD57C8: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD57CC: 913F34D4  stw r9, 0x34d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13524 as u32), ctx.r[9].u32 ) };
	// 82BD57D0: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82BD57D4: 7FDBF92E  stwx r30, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 82BD57D8: 5749F63E  rlwinm r9, r26, 0x1e, 0x18, 0x1f
	ctx.r[9].u64 = ctx.r[26].u32 as u64 & 0x00000003u64;
	// 82BD57DC: 574AF0BF  rlwinm. r10, r26, 0x1e, 2, 0x1f
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD57E0: 992B30F0  stb r9, 0x30f0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12528 as u32), ctx.r[9].u8 ) };
	// 82BD57E4: 4182001C  beq 0x82bd5800
	if ctx.cr[0].eq {
	pc = 0x82BD5800; continue 'dispatch;
	}
	// 82BD57E8: 896B2E38  lbz r11, 0x2e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11832 as u32) ) } as u64;
	// 82BD57EC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD57F0: 419A0010  beq cr6, 0x82bd5800
	if ctx.cr[6].eq {
	pc = 0x82BD5800; continue 'dispatch;
	}
	// 82BD57F4: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD57F8: 656B0008  oris r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 524288;
	// 82BD57FC: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5800: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82BD5804: 485D29AC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5808 size=144
    let mut pc: u32 = 0x82BD5808;
    'dispatch: loop {
        match pc {
            0x82BD5808 => {
    //   block [0x82BD5808..0x82BD5898)
	// 82BD5808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD580C: 485D2961  bl 0x831a816c
	ctx.lr = 0x82BD5810;
	sub_831A8130(ctx, base);
	// 82BD5810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD5818: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82BD581C: 83DF3094  lwz r30, 0x3094(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12436 as u32) ) } as u64;
	// 82BD5820: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD5824: 419A0068  beq cr6, 0x82bd588c
	if ctx.cr[6].eq {
	pc = 0x82BD588C; continue 'dispatch;
	}
	// 82BD5828: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 82BD582C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5830: 419A000C  beq cr6, 0x82bd583c
	if ctx.cr[6].eq {
	pc = 0x82BD583C; continue 'dispatch;
	}
	// 82BD5834: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD5838: 48000054  b 0x82bd588c
	pc = 0x82BD588C; continue 'dispatch;
	// 82BD583C: 817F2AA0  lwz r11, 0x2aa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10912 as u32) ) } as u64;
	// 82BD5840: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD5844: 7D6B5039  and. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD5848: 41820044  beq 0x82bd588c
	if ctx.cr[0].eq {
	pc = 0x82BD588C; continue 'dispatch;
	}
	// 82BD584C: 817F34D8  lwz r11, 0x34d8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13528 as u32) ) } as u64;
	// 82BD5850: 807F34D4  lwz r3, 0x34d4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13524 as u32) ) } as u64;
	// 82BD5854: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD5858: 4198000C  blt cr6, 0x82bd5864
	if ctx.cr[6].lt {
	pc = 0x82BD5864; continue 'dispatch;
	}
	// 82BD585C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5860: 4801A2A1  bl 0x82befb00
	ctx.lr = 0x82BD5864;
	sub_82BEFB00(ctx, base);
	// 82BD5864: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD5868: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82BD586C: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82BD5870: 53CBF0BE  rlwimi r11, r30, 0x1e, 2, 0x1f
	ctx.r[11].u64 = (((ctx.r[30].u32).rotate_left(30) as u64) & 0x000000003FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFFC0000000);
	// 82BD5874: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82BD5878: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD587C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82BD5880: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD5884: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD5888: 913F34D4  stw r9, 0x34d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13524 as u32), ctx.r[9].u32 ) };
	// 82BD588C: 93BF3094  stw r29, 0x3094(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12436 as u32), ctx.r[29].u32 ) };
	// 82BD5890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82BD5894: 485D2928  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5898 size=68
    let mut pc: u32 = 0x82BD5898;
    'dispatch: loop {
        match pc {
            0x82BD5898 => {
    //   block [0x82BD5898..0x82BD58DC)
	// 82BD5898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD589C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD58A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD58A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD58A8: 39640C26  addi r11, r4, 0xc26
	ctx.r[11].s64 = ctx.r[4].s64 + 3110;
	// 82BD58AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD58B0: 7FEB182E  lwzx r31, r11, r3
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD58B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82BD58B8: 419A000C  beq cr6, 0x82bd58c4
	if ctx.cr[6].eq {
	pc = 0x82BD58C4; continue 'dispatch;
	}
	// 82BD58BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD58C0: 480096A1  bl 0x82bdef60
	ctx.lr = 0x82BD58C4;
	sub_82BDEF60(ctx, base);
	// 82BD58C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD58C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD58CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD58D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD58D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD58D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD58E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD58E0 size=60
    let mut pc: u32 = 0x82BD58E0;
    'dispatch: loop {
        match pc {
            0x82BD58E0 => {
    //   block [0x82BD58E0..0x82BD591C)
	// 82BD58E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD58E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD58E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD58EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD58F0: 83E330A8  lwz r31, 0x30a8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD58F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82BD58F8: 419A000C  beq cr6, 0x82bd5904
	if ctx.cr[6].eq {
	pc = 0x82BD5904; continue 'dispatch;
	}
	// 82BD58FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5900: 48009661  bl 0x82bdef60
	ctx.lr = 0x82BD5904;
	sub_82BDEF60(ctx, base);
	// 82BD5904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD590C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5920 size=60
    let mut pc: u32 = 0x82BD5920;
    'dispatch: loop {
        match pc {
            0x82BD5920 => {
    //   block [0x82BD5920..0x82BD595C)
	// 82BD5920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD5928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD592C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5930: 83E339F0  lwz r31, 0x39f0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14832 as u32) ) } as u64;
	// 82BD5934: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82BD5938: 419A000C  beq cr6, 0x82bd5944
	if ctx.cr[6].eq {
	pc = 0x82BD5944; continue 'dispatch;
	}
	// 82BD593C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5940: 48009621  bl 0x82bdef60
	ctx.lr = 0x82BD5944;
	sub_82BDEF60(ctx, base);
	// 82BD5944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD594C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5960 size=208
    let mut pc: u32 = 0x82BD5960;
    'dispatch: loop {
        match pc {
            0x82BD5960 => {
    //   block [0x82BD5960..0x82BD5A30)
	// 82BD5960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD5968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD596C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD5970: 9421F990  stwu r1, -0x670(r1)
	ea = ctx.r[1].u32.wrapping_add(-1648 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD5978: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82BD597C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD5980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82BD5984: 48010BC5  bl 0x82be6548
	ctx.lr = 0x82BD5988;
	sub_82BE6548(ctx, base);
	// 82BD5988: 3BDF3A9C  addi r30, r31, 0x3a9c
	ctx.r[30].s64 = ctx.r[31].s64 + 15004;
	// 82BD598C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82BD5990: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82BD5994: 390B0600  addi r8, r11, 0x600
	ctx.r[8].s64 = ctx.r[11].s64 + 1536;
	// 82BD5998: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD599C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD59A0: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD59A4: 40820014  bne 0x82bd59b8
	if !ctx.cr[0].eq {
	pc = 0x82BD59B8; continue 'dispatch;
	}
	// 82BD59A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82BD59AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82BD59B0: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82BD59B4: 409AFFE4  bne cr6, 0x82bd5998
	if !ctx.cr[6].eq {
	pc = 0x82BD5998; continue 'dispatch;
	}
	// 82BD59B8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD59BC: 4182005C  beq 0x82bd5a18
	if ctx.cr[0].eq {
	pc = 0x82BD5A18; continue 'dispatch;
	}
	// 82BD59C0: 4866CFFD  bl 0x832429bc
	ctx.lr = 0x82BD59C4;
	// extern call 0x832429BC → crate::xboxkrnl::KeGetCurrentProcessType
	crate::xboxkrnl::KeGetCurrentProcessType(ctx, base);
	// 82BD59C4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82BD59C8: 419A0034  beq cr6, 0x82bd59fc
	if ctx.cr[6].eq {
	pc = 0x82BD59FC; continue 'dispatch;
	}
	// 82BD59CC: 807F5D90  lwz r3, 0x5d90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23952 as u32) ) } as u64;
	// 82BD59D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82BD59D4: 419A001C  beq cr6, 0x82bd59f0
	if ctx.cr[6].eq {
	pc = 0x82BD59F0; continue 'dispatch;
	}
	// 82BD59D8: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82BD59DC: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD59E0: 40820010  bne 0x82bd59f0
	if !ctx.cr[0].eq {
	pc = 0x82BD59F0; continue 'dispatch;
	}
	// 82BD59E4: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82BD59E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82BD59EC: 485EE905  bl 0x831c42f0
	ctx.lr = 0x82BD59F0;
	sub_831C42F0(ctx, base);
	// 82BD59F0: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82BD59F4: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82BD59F8: 917F5D94  stw r11, 0x5d94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(23956 as u32), ctx.r[11].u32 ) };
	// 82BD59FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82BD5A00: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82BD5A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD5A08: 485D2B09  bl 0x831a8510
	ctx.lr = 0x82BD5A0C;
	sub_831A8510(ctx, base);
	// 82BD5A0C: 817F4144  lwz r11, 0x4144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16708 as u32) ) } as u64;
	// 82BD5A10: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82BD5A14: 917F4144  stw r11, 0x4144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16708 as u32), ctx.r[11].u32 ) };
	// 82BD5A18: 38210670  addi r1, r1, 0x670
	ctx.r[1].s64 = ctx.r[1].s64 + 1648;
	// 82BD5A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5A24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD5A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5A30 size=208
    let mut pc: u32 = 0x82BD5A30;
    'dispatch: loop {
        match pc {
            0x82BD5A30 => {
    //   block [0x82BD5A30..0x82BD5B00)
	// 82BD5A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD5A38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD5A3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD5A40: 9421F990  stwu r1, -0x670(r1)
	ea = ctx.r[1].u32.wrapping_add(-1648 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5A44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD5A48: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82BD5A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD5A50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82BD5A54: 48010B8D  bl 0x82be65e0
	ctx.lr = 0x82BD5A58;
	sub_82BE65E0(ctx, base);
	// 82BD5A58: 3BDF3A9C  addi r30, r31, 0x3a9c
	ctx.r[30].s64 = ctx.r[31].s64 + 15004;
	// 82BD5A5C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82BD5A60: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82BD5A64: 390B0600  addi r8, r11, 0x600
	ctx.r[8].s64 = ctx.r[11].s64 + 1536;
	// 82BD5A68: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD5A6C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD5A70: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD5A74: 40820014  bne 0x82bd5a88
	if !ctx.cr[0].eq {
	pc = 0x82BD5A88; continue 'dispatch;
	}
	// 82BD5A78: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82BD5A7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82BD5A80: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82BD5A84: 409AFFE4  bne cr6, 0x82bd5a68
	if !ctx.cr[6].eq {
	pc = 0x82BD5A68; continue 'dispatch;
	}
	// 82BD5A88: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD5A8C: 4182005C  beq 0x82bd5ae8
	if ctx.cr[0].eq {
	pc = 0x82BD5AE8; continue 'dispatch;
	}
	// 82BD5A90: 4866CF2D  bl 0x832429bc
	ctx.lr = 0x82BD5A94;
	// extern call 0x832429BC → crate::xboxkrnl::KeGetCurrentProcessType
	crate::xboxkrnl::KeGetCurrentProcessType(ctx, base);
	// 82BD5A94: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82BD5A98: 419A0034  beq cr6, 0x82bd5acc
	if ctx.cr[6].eq {
	pc = 0x82BD5ACC; continue 'dispatch;
	}
	// 82BD5A9C: 807F5D90  lwz r3, 0x5d90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23952 as u32) ) } as u64;
	// 82BD5AA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82BD5AA4: 419A001C  beq cr6, 0x82bd5ac0
	if ctx.cr[6].eq {
	pc = 0x82BD5AC0; continue 'dispatch;
	}
	// 82BD5AA8: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82BD5AAC: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD5AB0: 40820010  bne 0x82bd5ac0
	if !ctx.cr[0].eq {
	pc = 0x82BD5AC0; continue 'dispatch;
	}
	// 82BD5AB4: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82BD5AB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82BD5ABC: 485EE835  bl 0x831c42f0
	ctx.lr = 0x82BD5AC0;
	sub_831C42F0(ctx, base);
	// 82BD5AC0: 817F5D94  lwz r11, 0x5d94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23956 as u32) ) } as u64;
	// 82BD5AC4: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82BD5AC8: 917F5D94  stw r11, 0x5d94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(23956 as u32), ctx.r[11].u32 ) };
	// 82BD5ACC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82BD5AD0: 38A00600  li r5, 0x600
	ctx.r[5].s64 = 1536;
	// 82BD5AD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD5AD8: 485D2A39  bl 0x831a8510
	ctx.lr = 0x82BD5ADC;
	sub_831A8510(ctx, base);
	// 82BD5ADC: 817F4144  lwz r11, 0x4144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16708 as u32) ) } as u64;
	// 82BD5AE0: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82BD5AE4: 917F4144  stw r11, 0x4144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16708 as u32), ctx.r[11].u32 ) };
	// 82BD5AE8: 38210670  addi r1, r1, 0x670
	ctx.r[1].s64 = ctx.r[1].s64 + 1648;
	// 82BD5AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5AF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD5AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5B00 size=80
    let mut pc: u32 = 0x82BD5B00;
    'dispatch: loop {
        match pc {
            0x82BD5B00 => {
    //   block [0x82BD5B00..0x82BD5B50)
	// 82BD5B00: 39640282  addi r11, r4, 0x282
	ctx.r[11].s64 = ctx.r[4].s64 + 642;
	// 82BD5B04: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD5B08: 39240009  addi r9, r4, 9
	ctx.r[9].s64 = ctx.r[4].s64 + 9;
	// 82BD5B0C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5B10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD5B14: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD5B18: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82BD5B1C: 7908FFE6  rldicr r8, r8, 0x3f, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5B20: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD5B24: 7D0A4C36  srd r10, r8, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[8].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82BD5B28: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD5B2C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD5B30: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD5B34: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82BD5B38: 8125000C  lwz r9, 0xc(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD5B3C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82BD5B40: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD5B44: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD5B48: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD5B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5B50 size=244
    let mut pc: u32 = 0x82BD5B50;
    'dispatch: loop {
        match pc {
            0x82BD5B50 => {
    //   block [0x82BD5B50..0x82BD5C44)
	// 82BD5B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD5B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD5B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD5B64: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82BD5B68: 409A0070  bne cr6, 0x82bd5bd8
	if !ctx.cr[6].eq {
	pc = 0x82BD5BD8; continue 'dispatch;
	}
	// 82BD5B6C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82BD5B70: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82BD5B74: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD5B78: 4099000C  ble cr6, 0x82bd5b84
	if !ctx.cr[6].gt {
	pc = 0x82BD5B84; continue 'dispatch;
	}
	// 82BD5B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5B80: 4800E9D1  bl 0x82be4550
	ctx.lr = 0x82BD5B84;
	sub_82BE4550(ctx, base);
	// 82BD5B84: 3D60C000  lis r11, -0x4000
	ctx.r[11].s64 = -1073741824;
	// 82BD5B88: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 82BD5B8C: 616B6000  ori r11, r11, 0x6000
	ctx.r[11].u64 = ctx.r[11].u64 | 24576;
	// 82BD5B90: 614A6200  ori r10, r10, 0x6200
	ctx.r[10].u64 = ctx.r[10].u64 | 25088;
	// 82BD5B94: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5B98: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82BD5B9C: 813F31A4  lwz r9, 0x31a4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12708 as u32) ) } as u64;
	// 82BD5BA0: 3D00C000  lis r8, -0x4000
	ctx.r[8].s64 = -1073741824;
	// 82BD5BA4: 3CE0C000  lis r7, -0x4000
	ctx.r[7].s64 = -1073741824;
	// 82BD5BA8: 61086100  ori r8, r8, 0x6100
	ctx.r[8].u64 = ctx.r[8].u64 | 24832;
	// 82BD5BAC: 60E76300  ori r7, r7, 0x6300
	ctx.r[7].u64 = ctx.r[7].u64 | 25344;
	// 82BD5BB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82BD5BB4: 95230004  stwu r9, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BB8: 95430004  stwu r10, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BBC: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BC0: 95030004  stwu r8, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BC4: 817F31A8  lwz r11, 0x31a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12712 as u32) ) } as u64;
	// 82BD5BC8: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BCC: 94E30004  stwu r7, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BD0: 94C30004  stwu r6, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[3].u32 = ea;
	// 82BD5BD4: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82BD5BD8: 817F28C8  lwz r11, 0x28c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10440 as u32) ) } as u64;
	// 82BD5BDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5BE0: 815F28C4  lwz r10, 0x28c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10436 as u32) ) } as u64;
	// 82BD5BE4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD5BE8: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5BEC: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD5BF0: 7D278E70  srawi r7, r9, 0x11
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 17) as i64;
	// 82BD5BF4: 554A881C  slwi r10, r10, 0x11
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(17);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD5BF8: 7D668E70  srawi r6, r11, 0x11
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 17) as i64;
	// 82BD5BFC: 7D058E70  srawi r5, r8, 0x11
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[8].s32 >> 17) as i64;
	// 82BD5C00: 7D448E70  srawi r4, r10, 0x11
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 17) as i64;
	// 82BD5C04: 4BFFD3DD  bl 0x82bd2fe0
	ctx.lr = 0x82BD5C08;
	sub_82BD2FE0(ctx, base);
	// 82BD5C08: 817F2AA4  lwz r11, 0x2aa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10916 as u32) ) } as u64;
	// 82BD5C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5C10: 5564A7BE  rlwinm r4, r11, 0x14, 0x1e, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD5C14: 4801A0BD  bl 0x82befcd0
	ctx.lr = 0x82BD5C18;
	sub_82BEFCD0(ctx, base);
	// 82BD5C18: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82BD5C1C: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD5C20: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82BD5C24: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5C28: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD5C2C: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD5C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD5C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82BD5C48 size=600
    let mut pc: u32 = 0x82BD5C48;
    'dispatch: loop {
        match pc {
            0x82BD5C48 => {
    //   block [0x82BD5C48..0x82BD5EA0)
	// 82BD5C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD5C50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD5C54: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82BD5C58: 485D2E19  bl 0x831a8a70
	ctx.lr = 0x82BD5C5C;
	sub_831A8A40(ctx, base);
	// 82BD5C5C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5C60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD5C64: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82BD5C68: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82BD5C6C: FFA02890  fmr f29, f5
	ctx.f[29].f64 = ctx.f[5].f64;
	// 82BD5C70: FF803090  fmr f28, f6
	ctx.f[28].f64 = ctx.f[6].f64;
	// 82BD5C74: 815F3098  lwz r10, 0x3098(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD5C78: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82BD5C7C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD5C80: 40820010  bne 0x82bd5c90
	if !ctx.cr[0].eq {
	pc = 0x82BD5C90; continue 'dispatch;
	}
	// 82BD5C84: 813F30A8  lwz r9, 0x30a8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD5C88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82BD5C8C: 419A01F8  beq cr6, 0x82bd5e84
	if ctx.cr[6].eq {
	pc = 0x82BD5E84; continue 'dispatch;
	}
	// 82BD5C90: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 82BD5C94: 556806F7  rlwinm. r8, r11, 0, 0x1b, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82BD5C98: 4182000C  beq 0x82bd5ca4
	if ctx.cr[0].eq {
	pc = 0x82BD5CA4; continue 'dispatch;
	}
	// 82BD5C9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD5CA0: 48000090  b 0x82bd5d30
	pc = 0x82BD5D30; continue 'dispatch;
	// 82BD5CA4: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD5CA8: 41820080  beq 0x82bd5d28
	if ctx.cr[0].eq {
	pc = 0x82BD5D28; continue 'dispatch;
	}
	// 82BD5CAC: 817F31B8  lwz r11, 0x31b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12728 as u32) ) } as u64;
	// 82BD5CB0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD5CB4: 419A000C  beq cr6, 0x82bd5cc0
	if ctx.cr[6].eq {
	pc = 0x82BD5CC0; continue 'dispatch;
	}
	// 82BD5CB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD5CBC: 409A006C  bne cr6, 0x82bd5d28
	if !ctx.cr[6].eq {
	pc = 0x82BD5D28; continue 'dispatch;
	}
	// 82BD5CC0: 817F309C  lwz r11, 0x309c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82BD5CC4: 815F31BC  lwz r10, 0x31bc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12732 as u32) ) } as u64;
	// 82BD5CC8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD5CCC: 419A000C  beq cr6, 0x82bd5cd8
	if ctx.cr[6].eq {
	pc = 0x82BD5CD8; continue 'dispatch;
	}
	// 82BD5CD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5CD4: 409A0054  bne cr6, 0x82bd5d28
	if !ctx.cr[6].eq {
	pc = 0x82BD5D28; continue 'dispatch;
	}
	// 82BD5CD8: 817F30A0  lwz r11, 0x30a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82BD5CDC: 815F31C0  lwz r10, 0x31c0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12736 as u32) ) } as u64;
	// 82BD5CE0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD5CE4: 419A000C  beq cr6, 0x82bd5cf0
	if ctx.cr[6].eq {
	pc = 0x82BD5CF0; continue 'dispatch;
	}
	// 82BD5CE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5CEC: 409A003C  bne cr6, 0x82bd5d28
	if !ctx.cr[6].eq {
	pc = 0x82BD5D28; continue 'dispatch;
	}
	// 82BD5CF0: 817F30A4  lwz r11, 0x30a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82BD5CF4: 815F31C4  lwz r10, 0x31c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12740 as u32) ) } as u64;
	// 82BD5CF8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD5CFC: 419A000C  beq cr6, 0x82bd5d08
	if ctx.cr[6].eq {
	pc = 0x82BD5D08; continue 'dispatch;
	}
	// 82BD5D00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5D04: 409A0024  bne cr6, 0x82bd5d28
	if !ctx.cr[6].eq {
	pc = 0x82BD5D28; continue 'dispatch;
	}
	// 82BD5D08: 817F30A8  lwz r11, 0x30a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD5D0C: 815F31C8  lwz r10, 0x31c8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12744 as u32) ) } as u64;
	// 82BD5D10: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD5D14: 419A000C  beq cr6, 0x82bd5d20
	if ctx.cr[6].eq {
	pc = 0x82BD5D20; continue 'dispatch;
	}
	// 82BD5D18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD5D1C: 409A000C  bne cr6, 0x82bd5d28
	if !ctx.cr[6].eq {
	pc = 0x82BD5D28; continue 'dispatch;
	}
	// 82BD5D20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD5D24: 48000008  b 0x82bd5d2c
	pc = 0x82BD5D2C; continue 'dispatch;
	// 82BD5D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD5D2C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD5D30: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD5D34: 41820010  beq 0x82bd5d44
	if ctx.cr[0].eq {
	pc = 0x82BD5D44; continue 'dispatch;
	}
	// 82BD5D38: 817F337C  lwz r11, 0x337c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13180 as u32) ) } as u64;
	// 82BD5D3C: 80FF3380  lwz r7, 0x3380(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13184 as u32) ) } as u64;
	// 82BD5D40: 48000018  b 0x82bd5d58
	pc = 0x82BD5D58; continue 'dispatch;
	// 82BD5D44: 81490024  lwz r10, 0x24(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 82BD5D48: 554B74BE  srwi r11, r10, 0x12
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(18);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD5D4C: 554AEC7E  rlwinm r10, r10, 0x1d, 0x11, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82BD5D50: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82BD5D54: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82BD5D58: FC00F81E  fctiwz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[31].f64.trunc() as i32 as i64 };
	// 82BD5D5C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82BD5D60: FC00181E  fctiwz f0, f3
	ctx.f[0].s64 = if ctx.f[3].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[3].f64.trunc() as i32 as i64 };
	// 82BD5D64: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82BD5D68: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82BD5D6C: FC00F01E  fctiwz f0, f30
	ctx.f[0].s64 = if ctx.f[30].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[30].f64.trunc() as i32 as i64 };
	// 82BD5D70: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82BD5D74: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82BD5D78: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82BD5D7C: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82BD5D80: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82BD5D84: 40980008  bge cr6, 0x82bd5d8c
	if !ctx.cr[6].lt {
	pc = 0x82BD5D8C; continue 'dispatch;
	}
	// 82BD5D88: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD5D8C: FC00201E  fctiwz f0, f4
	ctx.f[0].s64 = if ctx.f[4].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[4].f64.trunc() as i32 as i64 };
	// 82BD5D90: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82BD5D94: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82BD5D98: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82BD5D9C: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82BD5DA0: 41980008  blt cr6, 0x82bd5da8
	if ctx.cr[6].lt {
	pc = 0x82BD5DA8; continue 'dispatch;
	}
	// 82BD5DA4: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82BD5DA8: 7D665851  subf. r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD5DAC: 7D484850  subf r10, r8, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82BD5DB0: 4180000C  blt 0x82bd5dbc
	if ctx.cr[0].lt {
	pc = 0x82BD5DBC; continue 'dispatch;
	}
	// 82BD5DB4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD5DB8: 4098000C  bge cr6, 0x82bd5dc4
	if !ctx.cr[6].lt {
	pc = 0x82BD5DC4; continue 'dispatch;
	}
	// 82BD5DBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD5DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD5DC4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82BD5DC8: D3FF3168  stfs f31, 0x3168(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12648 as u32), tmp.u32 ) };
	// 82BD5DCC: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82BD5DD0: D3DF316C  stfs f30, 0x316c(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12652 as u32), tmp.u32 ) };
	// 82BD5DD4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82BD5DD8: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD5DDC: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82BD5DE0: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82BD5DE4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82BD5DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD5DEC: FF600018  frsp f27, f0
	ctx.f[27].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82BD5DF0: D3BF3178  stfs f29, 0x3178(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12664 as u32), tmp.u32 ) };
	// 82BD5DF4: FC006E9C  fcfid f0, f13
	ctx.f[0].f64 = (ctx.f[13].s64 as f64);
	// 82BD5DF8: D39F317C  stfs f28, 0x317c(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12668 as u32), tmp.u32 ) };
	// 82BD5DFC: D37F3174  stfs f27, 0x3174(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12660 as u32), tmp.u32 ) };
	// 82BD5E00: 917F3180  stw r11, 0x3180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12672 as u32), ctx.r[11].u32 ) };
	// 82BD5E04: 389F3184  addi r4, r31, 0x3184
	ctx.r[4].s64 = ctx.r[31].s64 + 12676;
	// 82BD5E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD5E0C: FF400018  frsp f26, f0
	ctx.f[26].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82BD5E10: D35F3170  stfs f26, 0x3170(r31)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12656 as u32), tmp.u32 ) };
	// 82BD5E14: 4BFFF78D  bl 0x82bd55a0
	ctx.lr = 0x82BD5E18;
	sub_82BD55A0(ctx, base);
	// 82BD5E18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82BD5E1C: EC1CE828  fsubs f0, f28, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[28].f64 - ctx.f[29].f64) as f32) as f64);
	// 82BD5E20: D01F2918  stfs f0, 0x2918(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10520 as u32), tmp.u32 ) };
	// 82BD5E24: D3BF291C  stfs f29, 0x291c(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10524 as u32), tmp.u32 ) };
	// 82BD5E28: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82BD5E2C: EDBA0032  fmuls f13, f26, f0
	ctx.f[13].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 82BD5E30: D1BF2908  stfs f13, 0x2908(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10504 as u32), tmp.u32 ) };
	// 82BD5E34: EC1B0032  fmuls f0, f27, f0
	ctx.f[0].f64 = (((ctx.f[27].f64 * ctx.f[0].f64) as f32) as f64);
	// 82BD5E38: EDADF82A  fadds f13, f13, f31
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 82BD5E3C: D1BF290C  stfs f13, 0x290c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10508 as u32), tmp.u32 ) };
	// 82BD5E40: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82BD5E44: D1BF2910  stfs f13, 0x2910(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10512 as u32), tmp.u32 ) };
	// 82BD5E48: EC00F02A  fadds f0, f0, f30
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 82BD5E4C: D01F2914  stfs f0, 0x2914(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10516 as u32), tmp.u32 ) };
	// 82BD5E50: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD5E54: 656B0400  oris r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 67108864;
	// 82BD5E58: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5E5C: 656B0200  oris r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 33554432;
	// 82BD5E60: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5E64: 656B0100  oris r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 16777216;
	// 82BD5E68: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5E6C: 656B0080  oris r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 8388608;
	// 82BD5E70: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5E74: 656B0040  oris r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 4194304;
	// 82BD5E78: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5E7C: 656B0020  oris r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 2097152;
	// 82BD5E80: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD5E84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82BD5E88: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82BD5E8C: 485D2C31  bl 0x831a8abc
	ctx.lr = 0x82BD5E90;
	sub_831A8A8C(ctx, base);
	// 82BD5E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD5E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5EA0 size=12
    let mut pc: u32 = 0x82BD5EA0;
    'dispatch: loop {
        match pc {
            0x82BD5EA0 => {
    //   block [0x82BD5EA0..0x82BD5EAC)
	// 82BD5EA0: 90832E50  stw r4, 0x2e50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11856 as u32), ctx.r[4].u32 ) };
	// 82BD5EA4: 38833184  addi r4, r3, 0x3184
	ctx.r[4].s64 = ctx.r[3].s64 + 12676;
	// 82BD5EA8: 4BFFF6F8  b 0x82bd55a0
	sub_82BD55A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82BD5EB0 size=128
    let mut pc: u32 = 0x82BD5EB0;
    'dispatch: loop {
        match pc {
            0x82BD5EB0 => {
    //   block [0x82BD5EB0..0x82BD5F30)
	// 82BD5EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD5EB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5EBC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD5EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD5EC4: 8124000C  lwz r9, 0xc(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD5EC8: C0C40014  lfs f6, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82BD5ECC: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD5ED0: C0A40010  lfs f5, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82BD5ED4: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD5ED8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82BD5EDC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82BD5EE0: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 82BD5EE4: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82BD5EE8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82BD5EEC: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82BD5EF0: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD5EF4: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82BD5EF8: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD5EFC: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82BD5F00: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82BD5F04: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82BD5F08: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82BD5F0C: FC405818  frsp f2, f11
	ctx.f[2].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82BD5F10: FC606018  frsp f3, f12
	ctx.f[3].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82BD5F14: FC806818  frsp f4, f13
	ctx.f[4].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82BD5F18: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82BD5F1C: 4BFFFD2D  bl 0x82bd5c48
	ctx.lr = 0x82BD5F20;
	sub_82BD5C48(ctx, base);
	// 82BD5F20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD5F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD5F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD5F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD5F30 size=32
    let mut pc: u32 = 0x82BD5F30;
    'dispatch: loop {
        match pc {
            0x82BD5F30 => {
    //   block [0x82BD5F30..0x82BD5F50)
	// 82BD5F30: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82BD5F34: C0C40014  lfs f6, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82BD5F38: C0A40010  lfs f5, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82BD5F3C: C084000C  lfs f4, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82BD5F40: C0640008  lfs f3, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82BD5F44: C0440004  lfs f2, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82BD5F48: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82BD5F4C: 4BFFFCFC  b 0x82bd5c48
	sub_82BD5C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5F50 size=20
    let mut pc: u32 = 0x82BD5F50;
    'dispatch: loop {
        match pc {
            0x82BD5F50 => {
    //   block [0x82BD5F50..0x82BD5F64)
	// 82BD5F50: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82BD5F54: 554B87BF  rlwinm. r11, r10, 0x10, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD5F58: 4082000C  bne 0x82bd5f64
	if !ctx.cr[0].eq {
		sub_82BD5F64(ctx, base);
		return;
	}
	// 82BD5F5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD5F60: 4800001C  b 0x82bd5f7c
	sub_82BD5F78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5F64 size=20
    let mut pc: u32 = 0x82BD5F64;
    'dispatch: loop {
        match pc {
            0x82BD5F64 => {
    //   block [0x82BD5F64..0x82BD5F78)
	// 82BD5F64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82BD5F68: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82BD5F6C: 409A000C  bne cr6, 0x82bd5f78
	if !ctx.cr[6].eq {
		sub_82BD5F78(ctx, base);
		return;
	}
	// 82BD5F70: 616B8001  ori r11, r11, 0x8001
	ctx.r[11].u64 = ctx.r[11].u64 | 32769;
	// 82BD5F74: 48000008  b 0x82bd5f7c
	sub_82BD5F78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD5F78 size=104
    let mut pc: u32 = 0x82BD5F78;
    'dispatch: loop {
        match pc {
            0x82BD5F78 => {
    //   block [0x82BD5F78..0x82BD5FE0)
	// 82BD5F78: 616BC003  ori r11, r11, 0xc003
	ctx.r[11].u64 = ctx.r[11].u64 | 49155;
	// 82BD5F7C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD5F80: 916329BC  stw r11, 0x29bc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10684 as u32), ctx.r[11].u32 ) };
	// 82BD5F84: 91432880  stw r10, 0x2880(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10368 as u32), ctx.r[10].u32 ) };
	// 82BD5F88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82BD5F8C: 798CCFE6  rldicr r12, r12, 0x39, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(57) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5F90: E9230010  ld r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD5F94: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82BD5F98: 7D296378  or r9, r9, r12
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[12].u64;
	// 82BD5F9C: F9230010  std r9, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u64 ) };
	// 82BD5FA0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD5FA4: E9030020  ld r8, 0x20(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD5FA8: 392BC278  addi r9, r11, -0x3d88
	ctx.r[9].s64 = ctx.r[11].s64 + -15752;
	// 82BD5FAC: 798C27E6  rldicr r12, r12, 0x24, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(36) & 0xFFFFFFFFFFFFFFFF;
	// 82BD5FB0: 388AC260  addi r4, r10, -0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + -15776;
	// 82BD5FB4: 7D086378  or r8, r8, r12
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[12].u64;
	// 82BD5FB8: F9030020  std r8, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[8].u64 ) };
	// 82BD5FBC: 816BC278  lwz r11, -0x3d88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15752 as u32) ) } as u64;
	// 82BD5FC0: 91633184  stw r11, 0x3184(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12676 as u32), ctx.r[11].u32 ) };
	// 82BD5FC4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD5FC8: 91633188  stw r11, 0x3188(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12680 as u32), ctx.r[11].u32 ) };
	// 82BD5FCC: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD5FD0: 9163318C  stw r11, 0x318c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12684 as u32), ctx.r[11].u32 ) };
	// 82BD5FD4: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD5FD8: 91633190  stw r11, 0x3190(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12688 as u32), ctx.r[11].u32 ) };
	// 82BD5FDC: 4BFFFED4  b 0x82bd5eb0
	sub_82BD5EB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD5FE0 size=840
    let mut pc: u32 = 0x82BD5FE0;
    'dispatch: loop {
        match pc {
            0x82BD5FE0 => {
    //   block [0x82BD5FE0..0x82BD6328)
	// 82BD5FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD5FE4: 485D2189  bl 0x831a816c
	ctx.lr = 0x82BD5FE8;
	sub_831A8130(ctx, base);
	// 82BD5FE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD5FEC: 39640C26  addi r11, r4, 0xc26
	ctx.r[11].s64 = ctx.r[4].s64 + 3110;
	// 82BD5FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD5FF4: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82BD5FF8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82BD5FFC: 7CA6F92E  stwx r5, r6, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 82BD6000: 419A0110  beq cr6, 0x82bd6110
	if ctx.cr[6].eq {
	pc = 0x82BD6110; continue 'dispatch;
	}
	// 82BD6004: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD6008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD600C: 419A0008  beq cr6, 0x82bd6014
	if ctx.cr[6].eq {
	pc = 0x82BD6014; continue 'dispatch;
	}
	// 82BD6010: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82BD6014: 394B0A21  addi r10, r11, 0xa21
	ctx.r[10].s64 = ctx.r[11].s64 + 2593;
	// 82BD6018: 80E5001C  lwz r7, 0x1c(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82BD601C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82BD6020: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD6024: 61280107  ori r8, r9, 0x107
	ctx.r[8].u64 = ctx.r[9].u64 | 263;
	// 82BD6028: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD602C: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82BD6030: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82BD6034: 7CEAF92E  stwx r7, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u32) };
	// 82BD6038: 7C6B8670  srawi r11, r3, 0x10
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 16) as i64;
	// 82BD603C: 78670620  clrldi r7, r3, 0x38
	ctx.r[7].u64 = ctx.r[3].u64 & 0x00000000000000FFu64;
	// 82BD6040: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD6044: 39640BBF  addi r11, r4, 0xbbf
	ctx.r[11].s64 = ctx.r[4].s64 + 3007;
	// 82BD6048: 7D273C36  srd r7, r9, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[9].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82BD604C: 7C6AF82A  ldx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	// 82BD6050: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD6054: 7CE71B78  or r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[3].u64;
	// 82BD6058: 7CEAF92A  stdx r7, r10, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u64) };
	// 82BD605C: 7CEBF82E  lwzx r7, r11, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD6060: 7D46F82E  lwzx r10, r6, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD6064: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD6068: 7CEBF92E  stwx r7, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u32) };
	// 82BD606C: 419A00A4  beq cr6, 0x82bd6110
	if ctx.cr[6].eq {
	pc = 0x82BD6110; continue 'dispatch;
	}
	// 82BD6070: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD6074: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82BD6078: 419A0008  beq cr6, 0x82bd6080
	if ctx.cr[6].eq {
	pc = 0x82BD6080; continue 'dispatch;
	}
	// 82BD607C: 3BC40001  addi r30, r4, 1
	ctx.r[30].s64 = ctx.r[4].s64 + 1;
	// 82BD6080: 397E0A21  addi r11, r30, 0xa21
	ctx.r[11].s64 = ctx.r[30].s64 + 2593;
	// 82BD6084: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82BD6088: 7CC3F82E  lwzx r6, r3, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD608C: 54CA873E  rlwinm r10, r6, 0x10, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 82BD6090: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82BD6094: 419A001C  beq cr6, 0x82bd60b0
	if ctx.cr[6].eq {
	pc = 0x82BD60B0; continue 'dispatch;
	}
	// 82BD6098: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 82BD609C: 419A0014  beq cr6, 0x82bd60b0
	if ctx.cr[6].eq {
	pc = 0x82BD60B0; continue 'dispatch;
	}
	// 82BD60A0: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 82BD60A4: 419A000C  beq cr6, 0x82bd60b0
	if ctx.cr[6].eq {
	pc = 0x82BD60B0; continue 'dispatch;
	}
	// 82BD60A8: 2B0A000C  cmplwi cr6, r10, 0xc
	ctx.cr[6].compare_u32(ctx.r[10].u32, 12 as u32, &mut ctx.xer);
	// 82BD60AC: 409A0064  bne cr6, 0x82bd6110
	if !ctx.cr[6].eq {
	pc = 0x82BD6110; continue 'dispatch;
	}
	// 82BD60B0: 54CB6FFE  rlwinm r11, r6, 0xd, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x0007FFFFu64;
	// 82BD60B4: 7D6B3A79  xor. r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[7].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD60B8: 41820058  beq 0x82bd6110
	if ctx.cr[0].eq {
	pc = 0x82BD6110; continue 'dispatch;
	}
	// 82BD60BC: 3967FFFF  addi r11, r7, -1
	ctx.r[11].s64 = ctx.r[7].s64 + -1;
	// 82BD60C0: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD60C4: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 82BD60C8: 7D7D58F8  nor r29, r11, r11
	ctx.r[29].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82BD60CC: 38E7FFFD  addi r7, r7, -3
	ctx.r[7].s64 = ctx.r[7].s64 + -3;
	// 82BD60D0: 554A881C  slwi r10, r10, 0x11
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(17);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD60D4: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82BD60D8: 57BD801E  slwi r29, r29, 0x10
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(16);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82BD60DC: 556B801E  slwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD60E0: 7FAA5038  and r10, r29, r10
	ctx.r[10].u64 = ctx.r[29].u64 & ctx.r[10].u64;
	// 82BD60E4: 7D1E4214  add r8, r30, r8
	ctx.r[8].u64 = ctx.r[30].u64 + ctx.r[8].u64;
	// 82BD60E8: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD60EC: 7D0B8670  srawi r11, r8, 0x10
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 16) as i64;
	// 82BD60F0: 50CA0416  rlwimi r10, r6, 0, 0x10, 0xb
	ctx.r[10].u64 = (((ctx.r[6].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[10].u64 & 0x00000000000F0000);
	// 82BD60F4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD60F8: 7D43F92E  stwx r10, r3, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82BD60FC: 790A0620  clrldi r10, r8, 0x38
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000000000FFu64;
	// 82BD6100: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82BD6104: 7D2BF82A  ldx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	// 82BD6108: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82BD610C: 7D4BF92A  stdx r10, r11, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u64) };
	// 82BD6110: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD6114: 409A0174  bne cr6, 0x82bd6288
	if !ctx.cr[6].eq {
	pc = 0x82BD6288; continue 'dispatch;
	}
	// 82BD6118: 897F2ABF  lbz r11, 0x2abf(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10943 as u32) ) } as u64;
	// 82BD611C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82BD6120: 716A00F7  andi. r10, r11, 0xf7
	ctx.r[10].u64 = ctx.r[11].u64 & 247;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6124: 995F2ABF  stb r10, 0x2abf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10943 as u32), ctx.r[10].u8 ) };
	// 82BD6128: 419A0040  beq cr6, 0x82bd6168
	if ctx.cr[6].eq {
	pc = 0x82BD6168; continue 'dispatch;
	}
	// 82BD612C: A165001C  lhz r11, 0x1c(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82BD6130: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD6134: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82BD6138: 419A0014  beq cr6, 0x82bd614c
	if ctx.cr[6].eq {
	pc = 0x82BD614C; continue 'dispatch;
	}
	// 82BD613C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82BD6140: 419A000C  beq cr6, 0x82bd614c
	if ctx.cr[6].eq {
	pc = 0x82BD614C; continue 'dispatch;
	}
	// 82BD6144: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82BD6148: 409A0018  bne cr6, 0x82bd6160
	if !ctx.cr[6].eq {
	pc = 0x82BD6160; continue 'dispatch;
	}
	// 82BD614C: 614B0008  ori r11, r10, 8
	ctx.r[11].u64 = ctx.r[10].u64 | 8;
	// 82BD6150: 997F2ABF  stb r11, 0x2abf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10943 as u32), ctx.r[11].u8 ) };
	// 82BD6154: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD6158: 656B0010  oris r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 1048576;
	// 82BD615C: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD6160: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82BD6164: 48000010  b 0x82bd6174
	pc = 0x82BD6174; continue 'dispatch;
	// 82BD6168: 809F30A8  lwz r4, 0x30a8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD616C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD6170: 419A000C  beq cr6, 0x82bd617c
	if ctx.cr[6].eq {
	pc = 0x82BD617C; continue 'dispatch;
	}
	// 82BD6174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD6178: 4BFFFDD9  bl 0x82bd5f50
	ctx.lr = 0x82BD617C;
	sub_82BD5F50(ctx, base);
	// 82BD617C: 817F31AC  lwz r11, 0x31ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12716 as u32) ) } as u64;
	// 82BD6180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6184: 409A00D0  bne cr6, 0x82bd6254
	if !ctx.cr[6].eq {
	pc = 0x82BD6254; continue 'dispatch;
	}
	// 82BD6188: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 82BD618C: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6190: 408200C4  bne 0x82bd6254
	if !ctx.cr[0].eq {
	pc = 0x82BD6254; continue 'dispatch;
	}
	// 82BD6194: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6198: 408200BC  bne 0x82bd6254
	if !ctx.cr[0].eq {
	pc = 0x82BD6254; continue 'dispatch;
	}
	// 82BD619C: 895F2F9B  lbz r10, 0x2f9b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12187 as u32) ) } as u64;
	// 82BD61A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD61A4: 408200B0  bne 0x82bd6254
	if !ctx.cr[0].eq {
	pc = 0x82BD6254; continue 'dispatch;
	}
	// 82BD61A8: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD61AC: 4182000C  beq 0x82bd61b8
	if ctx.cr[0].eq {
	pc = 0x82BD61B8; continue 'dispatch;
	}
	// 82BD61B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD61B4: 48000094  b 0x82bd6248
	pc = 0x82BD6248; continue 'dispatch;
	// 82BD61B8: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD61BC: 41820084  beq 0x82bd6240
	if ctx.cr[0].eq {
	pc = 0x82BD6240; continue 'dispatch;
	}
	// 82BD61C0: 817F3098  lwz r11, 0x3098(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD61C4: 815F31B8  lwz r10, 0x31b8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12728 as u32) ) } as u64;
	// 82BD61C8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD61CC: 419A000C  beq cr6, 0x82bd61d8
	if ctx.cr[6].eq {
	pc = 0x82BD61D8; continue 'dispatch;
	}
	// 82BD61D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD61D4: 409A006C  bne cr6, 0x82bd6240
	if !ctx.cr[6].eq {
	pc = 0x82BD6240; continue 'dispatch;
	}
	// 82BD61D8: 817F309C  lwz r11, 0x309c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82BD61DC: 815F31BC  lwz r10, 0x31bc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12732 as u32) ) } as u64;
	// 82BD61E0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD61E4: 419A000C  beq cr6, 0x82bd61f0
	if ctx.cr[6].eq {
	pc = 0x82BD61F0; continue 'dispatch;
	}
	// 82BD61E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD61EC: 409A0054  bne cr6, 0x82bd6240
	if !ctx.cr[6].eq {
	pc = 0x82BD6240; continue 'dispatch;
	}
	// 82BD61F0: 817F30A0  lwz r11, 0x30a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82BD61F4: 815F31C0  lwz r10, 0x31c0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12736 as u32) ) } as u64;
	// 82BD61F8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD61FC: 419A000C  beq cr6, 0x82bd6208
	if ctx.cr[6].eq {
	pc = 0x82BD6208; continue 'dispatch;
	}
	// 82BD6200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6204: 409A003C  bne cr6, 0x82bd6240
	if !ctx.cr[6].eq {
	pc = 0x82BD6240; continue 'dispatch;
	}
	// 82BD6208: 817F30A4  lwz r11, 0x30a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82BD620C: 815F31C4  lwz r10, 0x31c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12740 as u32) ) } as u64;
	// 82BD6210: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD6214: 419A000C  beq cr6, 0x82bd6220
	if ctx.cr[6].eq {
	pc = 0x82BD6220; continue 'dispatch;
	}
	// 82BD6218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD621C: 409A0024  bne cr6, 0x82bd6240
	if !ctx.cr[6].eq {
	pc = 0x82BD6240; continue 'dispatch;
	}
	// 82BD6220: 817F30A8  lwz r11, 0x30a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD6224: 815F31C8  lwz r10, 0x31c8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12744 as u32) ) } as u64;
	// 82BD6228: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD622C: 419A000C  beq cr6, 0x82bd6238
	if ctx.cr[6].eq {
	pc = 0x82BD6238; continue 'dispatch;
	}
	// 82BD6230: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6234: 409A000C  bne cr6, 0x82bd6240
	if !ctx.cr[6].eq {
	pc = 0x82BD6240; continue 'dispatch;
	}
	// 82BD6238: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD623C: 48000008  b 0x82bd6244
	pc = 0x82BD6244; continue 'dispatch;
	// 82BD6240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6244: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD6248: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD624C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD6250: 40820008  bne 0x82bd6258
	if !ctx.cr[0].eq {
	pc = 0x82BD6258; continue 'dispatch;
	}
	// 82BD6254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD6258: 893F2ABC  lbz r9, 0x2abc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 82BD625C: 817F2E54  lwz r11, 0x2e54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11860 as u32) ) } as u64;
	// 82BD6260: 514907FE  rlwimi r9, r10, 0, 0x1f, 0x1f
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x0000000000000001) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFFE);
	// 82BD6264: 993F2ABC  stb r9, 0x2abc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10940 as u32), ctx.r[9].u8 ) };
	// 82BD6268: 815F3098  lwz r10, 0x3098(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD626C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD6270: 917F2E54  stw r11, 0x2e54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11860 as u32), ctx.r[11].u32 ) };
	// 82BD6274: 409A0008  bne cr6, 0x82bd627c
	if !ctx.cr[6].eq {
	pc = 0x82BD627C; continue 'dispatch;
	}
	// 82BD6278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD627C: 815F28DC  lwz r10, 0x28dc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10460 as u32) ) } as u64;
	// 82BD6280: 516A073E  rlwimi r10, r11, 0, 0x1c, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x000000000000000F) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFF0);
	// 82BD6284: 48000084  b 0x82bd6308
	pc = 0x82BD6308; continue 'dispatch;
	// 82BD6288: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82BD628C: 409A0028  bne cr6, 0x82bd62b4
	if !ctx.cr[6].eq {
	pc = 0x82BD62B4; continue 'dispatch;
	}
	// 82BD6290: 817F2E58  lwz r11, 0x2e58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11864 as u32) ) } as u64;
	// 82BD6294: 815F309C  lwz r10, 0x309c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82BD6298: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD629C: 917F2E58  stw r11, 0x2e58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11864 as u32), ctx.r[11].u32 ) };
	// 82BD62A0: 409A0008  bne cr6, 0x82bd62a8
	if !ctx.cr[6].eq {
	pc = 0x82BD62A8; continue 'dispatch;
	}
	// 82BD62A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD62A8: 815F28DC  lwz r10, 0x28dc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10460 as u32) ) } as u64;
	// 82BD62AC: 516A2636  rlwimi r10, r11, 4, 0x18, 0x1b
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(4) as u64) & 0x00000000000000F0) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF0F);
	// 82BD62B0: 48000058  b 0x82bd6308
	pc = 0x82BD6308; continue 'dispatch;
	// 82BD62B4: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 82BD62B8: 409A0028  bne cr6, 0x82bd62e0
	if !ctx.cr[6].eq {
	pc = 0x82BD62E0; continue 'dispatch;
	}
	// 82BD62BC: 817F2E5C  lwz r11, 0x2e5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11868 as u32) ) } as u64;
	// 82BD62C0: 815F30A0  lwz r10, 0x30a0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82BD62C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD62C8: 917F2E5C  stw r11, 0x2e5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11868 as u32), ctx.r[11].u32 ) };
	// 82BD62CC: 409A0008  bne cr6, 0x82bd62d4
	if !ctx.cr[6].eq {
	pc = 0x82BD62D4; continue 'dispatch;
	}
	// 82BD62D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD62D4: 815F28DC  lwz r10, 0x28dc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10460 as u32) ) } as u64;
	// 82BD62D8: 516A452E  rlwimi r10, r11, 8, 0x14, 0x17
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x0000000000000F00) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFF0FF);
	// 82BD62DC: 4800002C  b 0x82bd6308
	pc = 0x82BD6308; continue 'dispatch;
	// 82BD62E0: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82BD62E4: 409A003C  bne cr6, 0x82bd6320
	if !ctx.cr[6].eq {
	pc = 0x82BD6320; continue 'dispatch;
	}
	// 82BD62E8: 817F2E60  lwz r11, 0x2e60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11872 as u32) ) } as u64;
	// 82BD62EC: 815F30A4  lwz r10, 0x30a4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82BD62F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD62F4: 917F2E60  stw r11, 0x2e60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11872 as u32), ctx.r[11].u32 ) };
	// 82BD62F8: 409A0008  bne cr6, 0x82bd6300
	if !ctx.cr[6].eq {
	pc = 0x82BD6300; continue 'dispatch;
	}
	// 82BD62FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6300: 815F28DC  lwz r10, 0x28dc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10460 as u32) ) } as u64;
	// 82BD6304: 516A6426  rlwimi r10, r11, 0xc, 0x10, 0x13
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0FFF);
	// 82BD6308: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD630C: 915F28DC  stw r10, 0x28dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10460 as u32), ctx.r[10].u32 ) };
	// 82BD6310: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD6314: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 82BD6318: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD631C: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD6320: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD6324: 485D1E98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD6328 size=728
    let mut pc: u32 = 0x82BD6328;
    'dispatch: loop {
        match pc {
            0x82BD6328 => {
    //   block [0x82BD6328..0x82BD6600)
	// 82BD6328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD6330: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD6334: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD6338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD633C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD6340: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82BD6344: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD6348: 93DF30A8  stw r30, 0x30a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12456 as u32), ctx.r[30].u32 ) };
	// 82BD634C: 419A0120  beq cr6, 0x82bd646c
	if ctx.cr[6].eq {
	pc = 0x82BD646C; continue 'dispatch;
	}
	// 82BD6350: 817F3098  lwz r11, 0x3098(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD6354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6358: 409A0008  bne cr6, 0x82bd6360
	if !ctx.cr[6].eq {
	pc = 0x82BD6360; continue 'dispatch;
	}
	// 82BD635C: 4BFFFBF5  bl 0x82bd5f50
	ctx.lr = 0x82BD6360;
	sub_82BD5F50(ctx, base);
	// 82BD6360: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82BD6364: 917F2888  stw r11, 0x2888(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10376 as u32), ctx.r[11].u32 ) };
	// 82BD6368: 895F2ABE  lbz r10, 0x2abe(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10942 as u32) ) } as u64;
	// 82BD636C: 893F2ABF  lbz r9, 0x2abf(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10943 as u32) ) } as u64;
	// 82BD6370: 552906B5  rlwinm. r9, r9, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6374: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82BD6378: 917F2940  stw r11, 0x2940(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82BD637C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82BD6380: 516A2EB4  rlwimi r10, r11, 5, 0x1a, 0x1a
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(5) as u64) & 0x0000000000000020) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFDF);
	// 82BD6384: 995F2ABE  stb r10, 0x2abe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10942 as u32), ctx.r[10].u8 ) };
	// 82BD6388: 418200C0  beq 0x82bd6448
	if ctx.cr[0].eq {
	pc = 0x82BD6448; continue 'dispatch;
	}
	// 82BD638C: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 82BD6390: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6394: 4182000C  beq 0x82bd63a0
	if ctx.cr[0].eq {
	pc = 0x82BD63A0; continue 'dispatch;
	}
	// 82BD6398: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD639C: 48000094  b 0x82bd6430
	pc = 0x82BD6430; continue 'dispatch;
	// 82BD63A0: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD63A4: 41820084  beq 0x82bd6428
	if ctx.cr[0].eq {
	pc = 0x82BD6428; continue 'dispatch;
	}
	// 82BD63A8: 817F3098  lwz r11, 0x3098(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD63AC: 815F31B8  lwz r10, 0x31b8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12728 as u32) ) } as u64;
	// 82BD63B0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD63B4: 419A000C  beq cr6, 0x82bd63c0
	if ctx.cr[6].eq {
	pc = 0x82BD63C0; continue 'dispatch;
	}
	// 82BD63B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD63BC: 409A006C  bne cr6, 0x82bd6428
	if !ctx.cr[6].eq {
	pc = 0x82BD6428; continue 'dispatch;
	}
	// 82BD63C0: 817F309C  lwz r11, 0x309c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82BD63C4: 815F31BC  lwz r10, 0x31bc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12732 as u32) ) } as u64;
	// 82BD63C8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD63CC: 419A000C  beq cr6, 0x82bd63d8
	if ctx.cr[6].eq {
	pc = 0x82BD63D8; continue 'dispatch;
	}
	// 82BD63D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD63D4: 409A0054  bne cr6, 0x82bd6428
	if !ctx.cr[6].eq {
	pc = 0x82BD6428; continue 'dispatch;
	}
	// 82BD63D8: 817F30A0  lwz r11, 0x30a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82BD63DC: 815F31C0  lwz r10, 0x31c0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12736 as u32) ) } as u64;
	// 82BD63E0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD63E4: 419A000C  beq cr6, 0x82bd63f0
	if ctx.cr[6].eq {
	pc = 0x82BD63F0; continue 'dispatch;
	}
	// 82BD63E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD63EC: 409A003C  bne cr6, 0x82bd6428
	if !ctx.cr[6].eq {
	pc = 0x82BD6428; continue 'dispatch;
	}
	// 82BD63F0: 817F30A4  lwz r11, 0x30a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82BD63F4: 815F31C4  lwz r10, 0x31c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12740 as u32) ) } as u64;
	// 82BD63F8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD63FC: 419A000C  beq cr6, 0x82bd6408
	if ctx.cr[6].eq {
	pc = 0x82BD6408; continue 'dispatch;
	}
	// 82BD6400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6404: 409A0024  bne cr6, 0x82bd6428
	if !ctx.cr[6].eq {
	pc = 0x82BD6428; continue 'dispatch;
	}
	// 82BD6408: 817F30A8  lwz r11, 0x30a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD640C: 815F31C8  lwz r10, 0x31c8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12744 as u32) ) } as u64;
	// 82BD6410: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD6414: 419A000C  beq cr6, 0x82bd6420
	if ctx.cr[6].eq {
	pc = 0x82BD6420; continue 'dispatch;
	}
	// 82BD6418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD641C: 409A000C  bne cr6, 0x82bd6428
	if !ctx.cr[6].eq {
	pc = 0x82BD6428; continue 'dispatch;
	}
	// 82BD6420: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD6424: 48000008  b 0x82bd642c
	pc = 0x82BD642C; continue 'dispatch;
	// 82BD6428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD642C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD6430: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6434: 41820014  beq 0x82bd6448
	if ctx.cr[0].eq {
	pc = 0x82BD6448; continue 'dispatch;
	}
	// 82BD6438: 817F2880  lwz r11, 0x2880(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10368 as u32) ) } as u64;
	// 82BD643C: 815F3374  lwz r10, 0x3374(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13172 as u32) ) } as u64;
	// 82BD6440: 514B901A  rlwimi r11, r10, 0x12, 0, 0xd
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(18) as u64) & 0x00000000FFFC0000) | (ctx.r[11].u64 & 0xFFFFFFFF0003FFFF);
	// 82BD6444: 917F2880  stw r11, 0x2880(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10368 as u32), ctx.r[11].u32 ) };
	// 82BD6448: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD644C: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD6450: 798CBFE6  rldicr r12, r12, 0x37, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(55) & 0xFFFFFFFFFFFFFFFF;
	// 82BD6454: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD6458: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD645C: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD6460: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD6464: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82BD6468: 48000024  b 0x82bd648c
	pc = 0x82BD648C; continue 'dispatch;
	// 82BD646C: 817F2940  lwz r11, 0x2940(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82BD6470: 895F2ABE  lbz r10, 0x2abe(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10942 as u32) ) } as u64;
	// 82BD6474: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD6478: 714A00DF  andi. r10, r10, 0xdf
	ctx.r[10].u64 = ctx.r[10].u64 & 223;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD647C: 917F2940  stw r11, 0x2940(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82BD6480: 995F2ABE  stb r10, 0x2abe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10942 as u32), ctx.r[10].u8 ) };
	// 82BD6484: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD6488: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82BD648C: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD6490: 817F31AC  lwz r11, 0x31ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12716 as u32) ) } as u64;
	// 82BD6494: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6498: 409A00D0  bne cr6, 0x82bd6568
	if !ctx.cr[6].eq {
	pc = 0x82BD6568; continue 'dispatch;
	}
	// 82BD649C: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 82BD64A0: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD64A4: 408200C4  bne 0x82bd6568
	if !ctx.cr[0].eq {
	pc = 0x82BD6568; continue 'dispatch;
	}
	// 82BD64A8: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD64AC: 408200BC  bne 0x82bd6568
	if !ctx.cr[0].eq {
	pc = 0x82BD6568; continue 'dispatch;
	}
	// 82BD64B0: 895F2F9B  lbz r10, 0x2f9b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12187 as u32) ) } as u64;
	// 82BD64B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD64B8: 408200B0  bne 0x82bd6568
	if !ctx.cr[0].eq {
	pc = 0x82BD6568; continue 'dispatch;
	}
	// 82BD64BC: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD64C0: 4182000C  beq 0x82bd64cc
	if ctx.cr[0].eq {
	pc = 0x82BD64CC; continue 'dispatch;
	}
	// 82BD64C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD64C8: 48000094  b 0x82bd655c
	pc = 0x82BD655C; continue 'dispatch;
	// 82BD64CC: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD64D0: 41820084  beq 0x82bd6554
	if ctx.cr[0].eq {
	pc = 0x82BD6554; continue 'dispatch;
	}
	// 82BD64D4: 817F3098  lwz r11, 0x3098(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82BD64D8: 815F31B8  lwz r10, 0x31b8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12728 as u32) ) } as u64;
	// 82BD64DC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD64E0: 419A000C  beq cr6, 0x82bd64ec
	if ctx.cr[6].eq {
	pc = 0x82BD64EC; continue 'dispatch;
	}
	// 82BD64E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD64E8: 409A006C  bne cr6, 0x82bd6554
	if !ctx.cr[6].eq {
	pc = 0x82BD6554; continue 'dispatch;
	}
	// 82BD64EC: 817F309C  lwz r11, 0x309c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82BD64F0: 815F31BC  lwz r10, 0x31bc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12732 as u32) ) } as u64;
	// 82BD64F4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD64F8: 419A000C  beq cr6, 0x82bd6504
	if ctx.cr[6].eq {
	pc = 0x82BD6504; continue 'dispatch;
	}
	// 82BD64FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6500: 409A0054  bne cr6, 0x82bd6554
	if !ctx.cr[6].eq {
	pc = 0x82BD6554; continue 'dispatch;
	}
	// 82BD6504: 817F30A0  lwz r11, 0x30a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82BD6508: 815F31C0  lwz r10, 0x31c0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12736 as u32) ) } as u64;
	// 82BD650C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD6510: 419A000C  beq cr6, 0x82bd651c
	if ctx.cr[6].eq {
	pc = 0x82BD651C; continue 'dispatch;
	}
	// 82BD6514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6518: 409A003C  bne cr6, 0x82bd6554
	if !ctx.cr[6].eq {
	pc = 0x82BD6554; continue 'dispatch;
	}
	// 82BD651C: 817F30A4  lwz r11, 0x30a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82BD6520: 815F31C4  lwz r10, 0x31c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12740 as u32) ) } as u64;
	// 82BD6524: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD6528: 419A000C  beq cr6, 0x82bd6534
	if ctx.cr[6].eq {
	pc = 0x82BD6534; continue 'dispatch;
	}
	// 82BD652C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6530: 409A0024  bne cr6, 0x82bd6554
	if !ctx.cr[6].eq {
	pc = 0x82BD6554; continue 'dispatch;
	}
	// 82BD6534: 817F30A8  lwz r11, 0x30a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD6538: 815F31C8  lwz r10, 0x31c8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12744 as u32) ) } as u64;
	// 82BD653C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD6540: 419A000C  beq cr6, 0x82bd654c
	if ctx.cr[6].eq {
	pc = 0x82BD654C; continue 'dispatch;
	}
	// 82BD6544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD6548: 409A000C  bne cr6, 0x82bd6554
	if !ctx.cr[6].eq {
	pc = 0x82BD6554; continue 'dispatch;
	}
	// 82BD654C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD6550: 48000008  b 0x82bd6558
	pc = 0x82BD6558; continue 'dispatch;
	// 82BD6554: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6558: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD655C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6560: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD6564: 40820008  bne 0x82bd656c
	if !ctx.cr[0].eq {
	pc = 0x82BD656C; continue 'dispatch;
	}
	// 82BD6568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD656C: 893F2ABC  lbz r9, 0x2abc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 82BD6570: 817F2E64  lwz r11, 0x2e64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11876 as u32) ) } as u64;
	// 82BD6574: 514907FE  rlwimi r9, r10, 0, 0x1f, 0x1f
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x0000000000000001) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFFE);
	// 82BD6578: 993F2ABC  stb r9, 0x2abc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10940 as u32), ctx.r[9].u8 ) };
	// 82BD657C: 815F30A8  lwz r10, 0x30a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD6580: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD6584: 917F2E64  stw r11, 0x2e64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11876 as u32), ctx.r[11].u32 ) };
	// 82BD6588: 409A0008  bne cr6, 0x82bd6590
	if !ctx.cr[6].eq {
	pc = 0x82BD6590; continue 'dispatch;
	}
	// 82BD658C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6590: 815F2934  lwz r10, 0x2934(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82BD6594: 516A0FBC  rlwimi r10, r11, 1, 0x1e, 0x1e
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(1) as u64) & 0x0000000000000002) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFD);
	// 82BD6598: 915F2934  stw r10, 0x2934(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10548 as u32), ctx.r[10].u32 ) };
	// 82BD659C: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD65A0: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82BD65A4: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD65A8: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82BD65AC: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD65B0: 815F30A8  lwz r10, 0x30a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82BD65B4: 817F2E68  lwz r11, 0x2e68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11880 as u32) ) } as u64;
	// 82BD65B8: 917F2E68  stw r11, 0x2e68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11880 as u32), ctx.r[11].u32 ) };
	// 82BD65BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD65C0: 409A0008  bne cr6, 0x82bd65c8
	if !ctx.cr[6].eq {
	pc = 0x82BD65C8; continue 'dispatch;
	}
	// 82BD65C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD65C8: 815F2934  lwz r10, 0x2934(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82BD65CC: 514B003C  rlwimi r11, r10, 0, 0, 0x1e
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFE) | (ctx.r[11].u64 & 0xFFFFFFFF00000001);
	// 82BD65D0: 917F2934  stw r11, 0x2934(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82BD65D4: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82BD65D8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82BD65DC: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD65E0: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82BD65E4: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD65E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD65EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD65F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD65F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD65F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD65FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6600 size=4
    let mut pc: u32 = 0x82BD6600;
    'dispatch: loop {
        match pc {
            0x82BD6600 => {
    //   block [0x82BD6600..0x82BD6604)
	// 82BD6600: 4BFFF9E0  b 0x82bd5fe0
	sub_82BD5FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6608 size=1332
    let mut pc: u32 = 0x82BD6608;
    'dispatch: loop {
        match pc {
            0x82BD6608 => {
    //   block [0x82BD6608..0x82BD6B3C)
	// 82BD6608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD660C: 485D1B59  bl 0x831a8164
	ctx.lr = 0x82BD6610;
	sub_831A8130(ctx, base);
	// 82BD6610: 3D40002A  lis r10, 0x2a
	ctx.r[10].s64 = 2752512;
	// 82BD6614: 3D200018  lis r9, 0x18
	ctx.r[9].s64 = 1572864;
	// 82BD6618: 3CA0002C  lis r5, 0x2c
	ctx.r[5].s64 = 2883584;
	// 82BD661C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6620: 7067400E  andi. r7, r3, 0x400e
	ctx.r[7].u64 = ctx.r[3].u64 & 16398;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD6624: 614623B9  ori r6, r10, 0x23b9
	ctx.r[6].u64 = ctx.r[10].u64 | 9145;
	// 82BD6628: 613F2886  ori r31, r9, 0x2886
	ctx.r[31].u64 = ctx.r[9].u64 | 10374;
	// 82BD662C: 60BE83A4  ori r30, r5, 0x83a4
	ctx.r[30].u64 = ctx.r[5].u64 | 33700;
	// 82BD6630: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82BD6634: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82BD6638: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82BD663C: 39440034  addi r10, r4, 0x34
	ctx.r[10].s64 = ctx.r[4].s64 + 52;
	// 82BD6640: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 82BD6644: 419A0344  beq cr6, 0x82bd6988
	if ctx.cr[6].eq {
	pc = 0x82BD6988; continue 'dispatch;
	}
	// 82BD6648: 2B070006  cmplwi cr6, r7, 6
	ctx.cr[6].compare_u32(ctx.r[7].u32, 6 as u32, &mut ctx.xer);
	// 82BD664C: 419A02BC  beq cr6, 0x82bd6908
	if ctx.cr[6].eq {
	pc = 0x82BD6908; continue 'dispatch;
	}
	// 82BD6650: 2B070008  cmplwi cr6, r7, 8
	ctx.cr[6].compare_u32(ctx.r[7].u32, 8 as u32, &mut ctx.xer);
	// 82BD6654: 419A0238  beq cr6, 0x82bd688c
	if ctx.cr[6].eq {
	pc = 0x82BD688C; continue 'dispatch;
	}
	// 82BD6658: 2B07000A  cmplwi cr6, r7, 0xa
	ctx.cr[6].compare_u32(ctx.r[7].u32, 10 as u32, &mut ctx.xer);
	// 82BD665C: 419A019C  beq cr6, 0x82bd67f8
	if ctx.cr[6].eq {
	pc = 0x82BD67F8; continue 'dispatch;
	}
	// 82BD6660: 2B07000C  cmplwi cr6, r7, 0xc
	ctx.cr[6].compare_u32(ctx.r[7].u32, 12 as u32, &mut ctx.xer);
	// 82BD6664: 419A00D0  beq cr6, 0x82bd6734
	if ctx.cr[6].eq {
	pc = 0x82BD6734; continue 'dispatch;
	}
	// 82BD6668: 2B07000E  cmplwi cr6, r7, 0xe
	ctx.cr[6].compare_u32(ctx.r[7].u32, 14 as u32, &mut ctx.xer);
	// 82BD666C: 419A0020  beq cr6, 0x82bd668c
	if ctx.cr[6].eq {
	pc = 0x82BD668C; continue 'dispatch;
	}
	// 82BD6670: 2B074002  cmplwi cr6, r7, 0x4002
	ctx.cr[6].compare_u32(ctx.r[7].u32, 16386 as u32, &mut ctx.xer);
	// 82BD6674: 409A0338  bne cr6, 0x82bd69ac
	if !ctx.cr[6].eq {
	pc = 0x82BD69AC; continue 'dispatch;
	}
	// 82BD6678: 3D20001A  lis r9, 0x1a
	ctx.r[9].s64 = 1703936;
	// 82BD667C: 612723A6  ori r7, r9, 0x23a6
	ctx.r[7].u64 = ctx.r[9].u64 | 9126;
	// 82BD6680: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82BD6684: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82BD6688: 48000308  b 0x82bd6990
	pc = 0x82BD6990; continue 'dispatch;
	// 82BD668C: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6690: 3D20001A  lis r9, 0x1a
	ctx.r[9].s64 = 1703936;
	// 82BD6694: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82BD6698: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82BD669C: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD66A0: 612923A6  ori r9, r9, 0x23a6
	ctx.r[9].u64 = ctx.r[9].u64 | 9126;
	// 82BD66A4: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD66A8: 546704E7  rlwinm. r7, r3, 0, 0x13, 0x13
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD66AC: 996A0009  stb r11, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82BD66B0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82BD66B4: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD66B8: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD66BC: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD66C0: B10A0002  sth r8, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82BD66C4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD66C8: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD66CC: 98AA0009  stb r5, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[5].u8 ) };
	// 82BD66D0: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD66D4: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD66D8: 98EA0009  stb r7, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[7].u8 ) };
	// 82BD66DC: 41820024  beq 0x82bd6700
	if ctx.cr[0].eq {
	pc = 0x82BD6700; continue 'dispatch;
	}
	// 82BD66E0: 3D20001A  lis r9, 0x1a
	ctx.r[9].s64 = 1703936;
	// 82BD66E4: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 82BD66E8: 61292286  ori r9, r9, 0x2286
	ctx.r[9].u64 = ctx.r[9].u64 | 8838;
	// 82BD66EC: B10A0002  sth r8, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82BD66F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82BD66F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD66F8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82BD66FC: 480002A0  b 0x82bd699c
	pc = 0x82BD699C; continue 'dispatch;
	// 82BD6700: 54690421  rlwinm. r9, r3, 0, 0x10, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6704: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6708: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82BD670C: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6710: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82BD6714: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6718: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD671C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82BD6720: 4182000C  beq 0x82bd672c
	if ctx.cr[0].eq {
	pc = 0x82BD672C; continue 'dispatch;
	}
	// 82BD6724: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD6728: 48000280  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD672C: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82BD6730: 48000278  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD6734: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6738: 546904E7  rlwinm. r9, r3, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD673C: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82BD6740: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD6744: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6748: 996A0009  stb r11, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82BD674C: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6750: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6754: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6758: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD675C: 98AA0009  stb r5, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[5].u8 ) };
	// 82BD6760: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6764: 4182003C  beq 0x82bd67a0
	if ctx.cr[0].eq {
	pc = 0x82BD67A0; continue 'dispatch;
	}
	// 82BD6768: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD676C: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD6770: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82BD6774: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD6778: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82BD677C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6780: 3D00001A  lis r8, 0x1a
	ctx.r[8].s64 = 1703936;
	// 82BD6784: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 82BD6788: 611D2286  ori r29, r8, 0x2286
	ctx.r[29].u64 = ctx.r[8].u64 | 8838;
	// 82BD678C: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82BD6790: B0EA0002  sth r7, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 82BD6794: 93AA0004  stw r29, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82BD6798: 9B8A0009  stb r28, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[28].u8 ) };
	// 82BD679C: 48000200  b 0x82bd699c
	pc = 0x82BD699C; continue 'dispatch;
	// 82BD67A0: 54690421  rlwinm. r9, r3, 0, 0x10, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD67A4: 41820034  beq 0x82bd67d8
	if ctx.cr[0].eq {
	pc = 0x82BD67D8; continue 'dispatch;
	}
	// 82BD67A8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD67AC: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD67B0: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82BD67B4: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD67B8: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82BD67BC: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD67C0: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82BD67C4: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82BD67C8: B0EA0002  sth r7, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 82BD67CC: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD67D0: 9BAA0009  stb r29, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[29].u8 ) };
	// 82BD67D4: 480001C8  b 0x82bd699c
	pc = 0x82BD699C; continue 'dispatch;
	// 82BD67D8: 3D20001A  lis r9, 0x1a
	ctx.r[9].s64 = 1703936;
	// 82BD67DC: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82BD67E0: 612923A6  ori r9, r9, 0x23a6
	ctx.r[9].u64 = ctx.r[9].u64 | 9126;
	// 82BD67E4: B10A0002  sth r8, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82BD67E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82BD67EC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD67F0: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82BD67F4: 480001B4  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD67F8: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD67FC: 546904E7  rlwinm. r9, r3, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6800: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82BD6804: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD6808: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD680C: 996A0009  stb r11, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82BD6810: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6814: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6818: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD681C: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6820: 98AA0009  stb r5, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[5].u8 ) };
	// 82BD6824: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6828: 41820024  beq 0x82bd684c
	if ctx.cr[0].eq {
	pc = 0x82BD684C; continue 'dispatch;
	}
	// 82BD682C: 3D20002C  lis r9, 0x2c
	ctx.r[9].s64 = 2883584;
	// 82BD6830: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82BD6834: 612923A5  ori r9, r9, 0x23a5
	ctx.r[9].u64 = ctx.r[9].u64 | 9125;
	// 82BD6838: B10A0002  sth r8, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82BD683C: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82BD6840: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD6844: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82BD6848: 4BFFFF34  b 0x82bd677c
	pc = 0x82BD677C; continue 'dispatch;
	// 82BD684C: 54690421  rlwinm. r9, r3, 0, 0x10, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6850: 41820024  beq 0x82bd6874
	if ctx.cr[0].eq {
	pc = 0x82BD6874; continue 'dispatch;
	}
	// 82BD6854: 3D20002C  lis r9, 0x2c
	ctx.r[9].s64 = 2883584;
	// 82BD6858: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82BD685C: 612923A5  ori r9, r9, 0x23a5
	ctx.r[9].u64 = ctx.r[9].u64 | 9125;
	// 82BD6860: B10A0002  sth r8, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82BD6864: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82BD6868: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD686C: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82BD6870: 4BFFFF4C  b 0x82bd67bc
	pc = 0x82BD67BC; continue 'dispatch;
	// 82BD6874: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD6878: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD687C: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82BD6880: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD6884: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82BD6888: 48000120  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD688C: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6890: 546904E7  rlwinm. r9, r3, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6894: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82BD6898: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD689C: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD68A0: 996A0009  stb r11, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82BD68A4: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD68A8: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD68AC: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD68B0: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD68B4: 98AA0009  stb r5, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[5].u8 ) };
	// 82BD68B8: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD68BC: 4182001C  beq 0x82bd68d8
	if ctx.cr[0].eq {
	pc = 0x82BD68D8; continue 'dispatch;
	}
	// 82BD68C0: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82BD68C4: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD68C8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82BD68CC: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD68D0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82BD68D4: 4BFFFEA8  b 0x82bd677c
	pc = 0x82BD677C; continue 'dispatch;
	// 82BD68D8: 54690421  rlwinm. r9, r3, 0, 0x10, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD68DC: 4182000C  beq 0x82bd68e8
	if ctx.cr[0].eq {
	pc = 0x82BD68E8; continue 'dispatch;
	}
	// 82BD68E0: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD68E4: 4BFFFFE0  b 0x82bd68c4
	pc = 0x82BD68C4; continue 'dispatch;
	// 82BD68E8: 3D20002C  lis r9, 0x2c
	ctx.r[9].s64 = 2883584;
	// 82BD68EC: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82BD68F0: 612923A5  ori r9, r9, 0x23a5
	ctx.r[9].u64 = ctx.r[9].u64 | 9125;
	// 82BD68F4: B10A0002  sth r8, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82BD68F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82BD68FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD6900: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82BD6904: 480000A4  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD6908: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD690C: 546904E7  rlwinm. r9, r3, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6910: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82BD6914: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82BD6918: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD691C: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6920: 996A0009  stb r11, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82BD6924: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6928: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD692C: 41820024  beq 0x82bd6950
	if ctx.cr[0].eq {
	pc = 0x82BD6950; continue 'dispatch;
	}
	// 82BD6930: 3D20001A  lis r9, 0x1a
	ctx.r[9].s64 = 1703936;
	// 82BD6934: 990A0009  stb r8, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[8].u8 ) };
	// 82BD6938: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82BD693C: 61292286  ori r9, r9, 0x2286
	ctx.r[9].u64 = ctx.r[9].u64 | 8838;
	// 82BD6940: B0EA0002  sth r7, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 82BD6944: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD6948: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82BD694C: 48000050  b 0x82bd699c
	pc = 0x82BD699C; continue 'dispatch;
	// 82BD6950: 54690421  rlwinm. r9, r3, 0, 0x10, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6954: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6958: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD695C: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6960: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6964: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD6968: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82BD696C: 41820010  beq 0x82bd697c
	if ctx.cr[0].eq {
	pc = 0x82BD697C; continue 'dispatch;
	}
	// 82BD6970: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD6974: 990A0009  stb r8, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[8].u8 ) };
	// 82BD6978: 48000030  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD697C: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82BD6980: 98AA0009  stb r5, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[5].u8 ) };
	// 82BD6984: 48000024  b 0x82bd69a8
	pc = 0x82BD69A8; continue 'dispatch;
	// 82BD6988: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82BD698C: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD6990: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82BD6994: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82BD6998: 996A0009  stb r11, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82BD699C: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD69A0: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD69A4: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD69A8: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD69AC: 546706F7  rlwinm. r7, r3, 0, 0x1b, 0x1b
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD69B0: 41820034  beq 0x82bd69e4
	if ctx.cr[0].eq {
	pc = 0x82BD69E4; continue 'dispatch;
	}
	// 82BD69B4: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD69B8: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD69BC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82BD69C0: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD69C4: 38E7000C  addi r7, r7, 0xc
	ctx.r[7].s64 = ctx.r[7].s64 + 12;
	// 82BD69C8: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD69CC: 992A0009  stb r9, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[9].u8 ) };
	// 82BD69D0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD69D4: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD69D8: 54E9043E  clrlwi r9, r7, 0x10
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82BD69DC: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD69E0: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD69E4: 546706B5  rlwinm. r7, r3, 0, 0x1a, 0x1a
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD69E8: 41820034  beq 0x82bd6a1c
	if ctx.cr[0].eq {
	pc = 0x82BD6A1C; continue 'dispatch;
	}
	// 82BD69EC: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD69F0: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD69F4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82BD69F8: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD69FC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82BD6A00: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82BD6A04: 992A0009  stb r9, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[9].u8 ) };
	// 82BD6A08: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD6A0C: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6A10: 54E9043E  clrlwi r9, r7, 0x10
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82BD6A14: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6A18: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6A1C: 54670673  rlwinm. r7, r3, 0, 0x19, 0x19
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD6A20: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82BD6A24: 41820030  beq 0x82bd6a54
	if ctx.cr[0].eq {
	pc = 0x82BD6A54; continue 'dispatch;
	}
	// 82BD6A28: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6A2C: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD6A30: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6A34: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD6A38: 39270004  addi r9, r7, 4
	ctx.r[9].s64 = ctx.r[7].s64 + 4;
	// 82BD6A3C: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD6A40: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6A44: 98CA0009  stb r6, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[6].u8 ) };
	// 82BD6A48: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6A4C: 996A000A  stb r11, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82BD6A50: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6A54: 54670631  rlwinm. r7, r3, 0, 0x18, 0x18
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD6A58: 41820030  beq 0x82bd6a88
	if ctx.cr[0].eq {
	pc = 0x82BD6A88; continue 'dispatch;
	}
	// 82BD6A5C: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6A60: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD6A64: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6A68: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD6A6C: 39270004  addi r9, r7, 4
	ctx.r[9].s64 = ctx.r[7].s64 + 4;
	// 82BD6A70: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD6A74: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6A78: 98CA0009  stb r6, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[6].u8 ) };
	// 82BD6A7C: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6A80: 98AA000A  stb r5, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[5].u8 ) };
	// 82BD6A84: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6A88: 547FC73F  rlwinm. r31, r3, 0x18, 0x1c, 0x1f
	ctx.r[31].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82BD6A8C: 5466843E  srwi r6, r3, 0x10
	ctx.r[6].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82BD6A90: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82BD6A94: 41820068  beq 0x82bd6afc
	if ctx.cr[0].eq {
	pc = 0x82BD6AFC; continue 'dispatch;
	}
	// 82BD6A98: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 82BD6A9C: 7D1F4214  add r8, r31, r8
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 82BD6AA0: 38A5C358  addi r5, r5, -0x3ca8
	ctx.r[5].s64 = ctx.r[5].s64 + -15528;
	// 82BD6AA4: 54DE07BE  clrlwi r30, r6, 0x1e
	ctx.r[30].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 82BD6AA8: 3BA5FFFC  addi r29, r5, -4
	ctx.r[29].s64 = ctx.r[5].s64 + -4;
	// 82BD6AAC: 5523043E  clrlwi r3, r9, 0x10
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6AB0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82BD6AB4: 3B600005  li r27, 5
	ctx.r[27].s64 = 5;
	// 82BD6AB8: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82BD6ABC: 7FDEE8AE  lbzx r30, r30, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82BD6AC0: 54C6F0BE  srwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82BD6AC4: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82BD6AC8: 7F07F840  cmplw cr6, r7, r31
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82BD6ACC: 57C9003A  rlwinm r9, r30, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD6AD0: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82BD6AD4: 7FC92A14  add r30, r9, r5
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 82BD6AD8: 7D234A14  add r9, r3, r9
	ctx.r[9].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 82BD6ADC: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6AE0: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82BD6AE4: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82BD6AE8: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82BD6AEC: 9B6A0009  stb r27, 9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(9 as u32), ctx.r[27].u8 ) };
	// 82BD6AF0: 9B8A000A  stb r28, 0xa(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[28].u8 ) };
	// 82BD6AF4: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6AF8: 4198FFAC  blt cr6, 0x82bd6aa4
	if ctx.cr[6].lt {
	pc = 0x82BD6AA4; continue 'dispatch;
	}
	// 82BD6AFC: 38E000FF  li r7, 0xff
	ctx.r[7].s64 = 255;
	// 82BD6B00: B161FFC2  sth r11, -0x3e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-62 as u32), ctx.r[11].u16 ) };
	// 82BD6B04: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82BD6B08: 9961FFC8  stb r11, -0x38(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[11].u8 ) };
	// 82BD6B0C: B0E1FFC0  sth r7, -0x40(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[7].u16 ) };
	// 82BD6B10: 80E1FFC0  lwz r7, -0x40(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) } as u64;
	// 82BD6B14: 9961FFC9  stb r11, -0x37(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-55 as u32), ctx.r[11].u8 ) };
	// 82BD6B18: 9961FFCA  stb r11, -0x36(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-54 as u32), ctx.r[11].u8 ) };
	// 82BD6B1C: 80C1FFC8  lwz r6, -0x38(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82BD6B20: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD6B24: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82BD6B28: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82BD6B2C: 91040018  stw r8, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82BD6B30: 9164001C  stw r11, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82BD6B34: 91640030  stw r11, 0x30(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82BD6B38: 485D167C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6B40 size=16
    let mut pc: u32 = 0x82BD6B40;
    'dispatch: loop {
        match pc {
            0x82BD6B40 => {
    //   block [0x82BD6B40..0x82BD6B50)
	// 82BD6B40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6B44: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6B48: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82BD6B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD6B50 size=88
    let mut pc: u32 = 0x82BD6B50;
    'dispatch: loop {
        match pc {
            0x82BD6B50 => {
    //   block [0x82BD6B50..0x82BD6BA8)
	// 82BD6B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD6B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD6B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD6B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD6B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD6B64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6B68: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82BD6B6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82BD6B70: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82BD6B74: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82BD6B78: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD6B7C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82BD6B80: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82BD6B84: 419A000C  beq cr6, 0x82bd6b90
	if ctx.cr[6].eq {
	pc = 0x82BD6B90; continue 'dispatch;
	}
	// 82BD6B88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD6B8C: 485D1655  bl 0x831a81e0
	ctx.lr = 0x82BD6B90;
	sub_831A81E0(ctx, base);
	// 82BD6B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD6B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD6B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD6B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD6BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD6BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD6BA8 size=132
    let mut pc: u32 = 0x82BD6BA8;
    'dispatch: loop {
        match pc {
            0x82BD6BA8 => {
    //   block [0x82BD6BA8..0x82BD6C2C)
	// 82BD6BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD6BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD6BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD6BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD6BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD6BBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD6BC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD6BC4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6BC8: 7FCB2A14  add r30, r11, r5
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82BD6BCC: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD6BD0: 4199001C  bgt cr6, 0x82bd6bec
	if ctx.cr[6].gt {
	pc = 0x82BD6BEC; continue 'dispatch;
	}
	// 82BD6BD4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD6BD8: 419A0028  beq cr6, 0x82bd6c00
	if ctx.cr[6].eq {
	pc = 0x82BD6C00; continue 'dispatch;
	}
	// 82BD6BDC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6BE0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD6BE4: 485D192D  bl 0x831a8510
	ctx.lr = 0x82BD6BE8;
	sub_831A8510(ctx, base);
	// 82BD6BE8: 48000018  b 0x82bd6c00
	pc = 0x82BD6C00; continue 'dispatch;
	// 82BD6BEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD6BF0: 419A0010  beq cr6, 0x82bd6c00
	if ctx.cr[6].eq {
	pc = 0x82BD6C00; continue 'dispatch;
	}
	// 82BD6BF4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82BD6BF8: 616B4005  ori r11, r11, 0x4005
	ctx.r[11].u64 = ctx.r[11].u64 | 16389;
	// 82BD6BFC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82BD6C00: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD6C04: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82BD6C08: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD6C0C: 40990008  ble cr6, 0x82bd6c14
	if !ctx.cr[6].gt {
	pc = 0x82BD6C14; continue 'dispatch;
	}
	// 82BD6C10: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82BD6C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD6C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD6C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD6C20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD6C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD6C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6C30 size=100
    let mut pc: u32 = 0x82BD6C30;
    'dispatch: loop {
        match pc {
            0x82BD6C30 => {
    //   block [0x82BD6C30..0x82BD6C94)
	// 82BD6C30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6C34: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD6C38: 556A8FFF  rlwinm. r10, r11, 0x11, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6C3C: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82BD6C40: 5529873E  rlwinm r9, r9, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6C44: 41820010  beq 0x82bd6c54
	if ctx.cr[0].eq {
	pc = 0x82BD6C54; continue 'dispatch;
	}
	// 82BD6C48: 55680463  rlwinm. r8, r11, 0, 0x11, 0x11
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82BD6C4C: 41820008  beq 0x82bd6c54
	if ctx.cr[0].eq {
	pc = 0x82BD6C54; continue 'dispatch;
	}
	// 82BD6C50: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 82BD6C54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD6C58: 419A0014  beq cr6, 0x82bd6c6c
	if ctx.cr[6].eq {
	pc = 0x82BD6C6C; continue 'dispatch;
	}
	// 82BD6C5C: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82BD6C60: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82BD6C64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD6C68: 41980008  blt cr6, 0x82bd6c70
	if ctx.cr[6].lt {
	pc = 0x82BD6C70; continue 'dispatch;
	}
	// 82BD6C6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6C70: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6C74: 41820010  beq 0x82bd6c84
	if ctx.cr[0].eq {
	pc = 0x82BD6C84; continue 'dispatch;
	}
	// 82BD6C78: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82BD6C7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD6C80: 409A0008  bne cr6, 0x82bd6c88
	if !ctx.cr[6].eq {
	pc = 0x82BD6C88; continue 'dispatch;
	}
	// 82BD6C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6C88: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6C8C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82BD6C90: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6C94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6C94 size=8
    let mut pc: u32 = 0x82BD6C94;
    'dispatch: loop {
        match pc {
            0x82BD6C94 => {
    //   block [0x82BD6C94..0x82BD6C9C)
	// 82BD6C94: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82BD6C98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6C9C size=12
    let mut pc: u32 = 0x82BD6C9C;
    'dispatch: loop {
        match pc {
            0x82BD6C9C => {
    //   block [0x82BD6C9C..0x82BD6CA8)
	// 82BD6C9C: 552B063E  clrlwi r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82BD6CA0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD6CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6CA8 size=8
    let mut pc: u32 = 0x82BD6CA8;
    'dispatch: loop {
        match pc {
            0x82BD6CA8 => {
    //   block [0x82BD6CA8..0x82BD6CB0)
	// 82BD6CA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6CAC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6CB0 size=48
    let mut pc: u32 = 0x82BD6CB0;
    'dispatch: loop {
        match pc {
            0x82BD6CB0 => {
    //   block [0x82BD6CB0..0x82BD6CE0)
	// 82BD6CB0: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6CB4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6CB8: 41820028  beq 0x82bd6ce0
	if ctx.cr[0].eq {
		sub_82BD6CE0(ctx, base);
		return;
	}
	// 82BD6CBC: 556AB63A  rlwinm r10, r11, 0x16, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82BD6CC0: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6CC4: 514B63A6  rlwimi r11, r10, 0xc, 0xe, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x000000000003F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFC0FFF);
	// 82BD6CC8: 556AEE3A  rlwinm r10, r11, 0x1d, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD6CCC: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD6CD0: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6CD4: 514B2D74  rlwimi r11, r10, 5, 0x15, 0x1a
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(5) as u64) & 0x00000000000007E0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF81F);
	// 82BD6CD8: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD6CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6CE0 size=216
    let mut pc: u32 = 0x82BD6CE0;
    'dispatch: loop {
        match pc {
            0x82BD6CE0 => {
    //   block [0x82BD6CE0..0x82BD6DB8)
	// 82BD6CE0: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6CE4: 40820024  bne 0x82bd6d08
	if !ctx.cr[0].eq {
	pc = 0x82BD6D08; continue 'dispatch;
	}
	// 82BD6CE8: 556AD63A  rlwinm r10, r11, 0x1a, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82BD6CEC: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6CF0: 514B44AE  rlwimi r11, r10, 8, 0x12, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x0000000000003F00) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFC0FF);
	// 82BD6CF4: 556A163A  rlwinm r10, r11, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD6CF8: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD6CFC: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6D00: 516A0032  rlwimi r10, r11, 0, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82BD6D04: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD6D08: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6D0C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82BD6D10: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6D14: 396BEC68  addi r11, r11, -0x1398
	ctx.r[11].s64 = ctx.r[11].s64 + -5016;
	// 82BD6D18: 554646FE  rlwinm r6, r10, 8, 0x1b, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82BD6D1C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82BD6D20: 552436BE  srwi r4, r9, 0x1a
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shr(26);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82BD6D24: 7D6658AE  lbzx r11, r6, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82BD6D28: 7CC428AE  lbzx r6, r4, r5
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82BD6D2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82BD6D30: 41980024  blt cr6, 0x82bd6d54
	if ctx.cr[6].lt {
	pc = 0x82BD6D54; continue 'dispatch;
	}
	// 82BD6D34: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6D38: 4182001C  beq 0x82bd6d54
	if ctx.cr[0].eq {
	pc = 0x82BD6D54; continue 'dispatch;
	}
	// 82BD6D3C: 89470009  lbz r10, 9(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(9 as u32) ) } as u64;
	// 82BD6D40: 5545163A  rlwinm r5, r10, 2, 0x18, 0x1d
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD6D44: 554A0032  rlwinm r10, r10, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD6D48: 7CA5402E  lwzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6D4C: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 82BD6D50: 99470009  stb r10, 9(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 82BD6D54: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82BD6D58: 41980028  blt cr6, 0x82bd6d80
	if ctx.cr[6].lt {
	pc = 0x82BD6D80; continue 'dispatch;
	}
	// 82BD6D5C: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6D60: 554A0043  rlwinm. r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6D64: 4182001C  beq 0x82bd6d80
	if ctx.cr[0].eq {
	pc = 0x82BD6D80; continue 'dispatch;
	}
	// 82BD6D68: 8947000A  lbz r10, 0xa(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(10 as u32) ) } as u64;
	// 82BD6D6C: 5545163A  rlwinm r5, r10, 2, 0x18, 0x1d
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD6D70: 554A0032  rlwinm r10, r10, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD6D74: 7CA5402E  lwzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6D78: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 82BD6D7C: 9947000A  stb r10, 0xa(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(10 as u32), ctx.r[10].u8 ) };
	// 82BD6D80: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82BD6D84: 4098000C  bge cr6, 0x82bd6d90
	if !ctx.cr[6].lt {
	pc = 0x82BD6D90; continue 'dispatch;
	}
	// 82BD6D88: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82BD6D8C: 409A0024  bne cr6, 0x82bd6db0
	if !ctx.cr[6].eq {
	pc = 0x82BD6DB0; continue 'dispatch;
	}
	// 82BD6D90: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6D94: 556A0085  rlwinm. r10, r11, 0, 2, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6D98: 41820018  beq 0x82bd6db0
	if ctx.cr[0].eq {
	pc = 0x82BD6DB0; continue 'dispatch;
	}
	// 82BD6D9C: 556A163A  rlwinm r10, r11, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD6DA0: 556B0632  rlwinm r11, r11, 0, 0x18, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD6DA4: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6DA8: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD6DAC: 9967000B  stb r11, 0xb(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(11 as u32), ctx.r[11].u8 ) };
	// 82BD6DB0: 2B060002  cmplwi cr6, r6, 2
	ctx.cr[6].compare_u32(ctx.r[6].u32, 2 as u32, &mut ctx.xer);
	// 82BD6DB4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6DB8 size=72
    let mut pc: u32 = 0x82BD6DB8;
    'dispatch: loop {
        match pc {
            0x82BD6DB8 => {
    //   block [0x82BD6DB8..0x82BD6E00)
	// 82BD6DB8: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6DBC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82BD6DC0: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD6DC4: 516AF108  rlwimi r10, r11, 0x1e, 4, 4
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[10].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82BD6DC8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82BD6DCC: 514537BE  rlwimi r5, r10, 6, 0x1e, 0x1f
	ctx.r[5].u64 = (((ctx.r[10].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[5].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82BD6DD0: 54AA06BE  clrlwi r10, r5, 0x1a
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 82BD6DD4: 5545163A  rlwinm r5, r10, 2, 0x18, 0x1d
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD6DD8: 554A0032  rlwinm r10, r10, 0, 0, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD6DDC: 7D05402E  lwzx r8, r5, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6DE0: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 82BD6DE4: 514606BA  rlwimi r6, r10, 0, 0x1a, 0x1d
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x000000000000003C) | (ctx.r[6].u64 & 0xFFFFFFFFFFFFFFC3);
	// 82BD6DE8: 514BE084  rlwimi r11, r10, 0x1c, 2, 2
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(28) as u64) & 0x0000000020000000) | (ctx.r[11].u64 & 0xFFFFFFFFDFFFFFFF);
	// 82BD6DEC: 5149D14A  rlwimi r9, r10, 0x1a, 5, 5
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(26) as u64) & 0x0000000004000000) | (ctx.r[9].u64 & 0xFFFFFFFFFBFFFFFF);
	// 82BD6DF0: 98C70007  stb r6, 7(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(7 as u32), ctx.r[6].u8 ) };
	// 82BD6DF4: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD6DF8: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD6DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E00 size=8
    let mut pc: u32 = 0x82BD6E00;
    'dispatch: loop {
        match pc {
            0x82BD6E00 => {
    //   block [0x82BD6E00..0x82BD6E08)
	// 82BD6E00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6E04: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E08 size=8
    let mut pc: u32 = 0x82BD6E08;
    'dispatch: loop {
        match pc {
            0x82BD6E08 => {
    //   block [0x82BD6E08..0x82BD6E10)
	// 82BD6E08: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6E0C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E10 size=12
    let mut pc: u32 = 0x82BD6E10;
    'dispatch: loop {
        match pc {
            0x82BD6E10 => {
    //   block [0x82BD6E10..0x82BD6E1C)
	// 82BD6E10: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6E14: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6E18: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E1C size=20
    let mut pc: u32 = 0x82BD6E1C;
    'dispatch: loop {
        match pc {
            0x82BD6E1C => {
    //   block [0x82BD6E1C..0x82BD6E30)
	// 82BD6E1C: 556A163A  rlwinm r10, r11, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD6E20: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD6E24: 516A0032  rlwimi r10, r11, 0, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82BD6E28: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD6E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E30 size=8
    let mut pc: u32 = 0x82BD6E30;
    'dispatch: loop {
        match pc {
            0x82BD6E30 => {
    //   block [0x82BD6E30..0x82BD6E38)
	// 82BD6E30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6E34: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E38 size=40
    let mut pc: u32 = 0x82BD6E38;
    'dispatch: loop {
        match pc {
            0x82BD6E38 => {
    //   block [0x82BD6E38..0x82BD6E60)
	// 82BD6E38: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6E3C: 548A063F  clrlwi. r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6E40: 41820020  beq 0x82bd6e60
	if ctx.cr[0].eq {
		sub_82BD6E60(ctx, base);
		return;
	}
	// 82BD6E44: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6E48: 552AA6BE  rlwinm r10, r9, 0x14, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82BD6E4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD6E50: 41990008  bgt cr6, 0x82bd6e58
	if ctx.cr[6].gt {
	pc = 0x82BD6E58; continue 'dispatch;
	}
	// 82BD6E54: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD6E58: 552ADEBE  rlwinm r10, r9, 0x1b, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD6E5C: 480000D0  b 0x82bd6f2c
	sub_82BD6E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6E60 size=224
    let mut pc: u32 = 0x82BD6E60;
    'dispatch: loop {
        match pc {
            0x82BD6E60 => {
    //   block [0x82BD6E60..0x82BD6F40)
	// 82BD6E60: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6E64: 54CA0421  rlwinm. r10, r6, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD6E68: 40820024  bne 0x82bd6e8c
	if !ctx.cr[0].eq {
	pc = 0x82BD6E8C; continue 'dispatch;
	}
	// 82BD6E6C: 54CAC6BE  rlwinm r10, r6, 0x18, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82BD6E70: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD6E74: 41990008  bgt cr6, 0x82bd6e7c
	if ctx.cr[6].gt {
	pc = 0x82BD6E7C; continue 'dispatch;
	}
	// 82BD6E78: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD6E7C: 54CA06BE  clrlwi r10, r6, 0x1a
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x0000003Fu64;
	// 82BD6E80: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD6E84: 41990008  bgt cr6, 0x82bd6e8c
	if ctx.cr[6].gt {
	pc = 0x82BD6E8C; continue 'dispatch;
	}
	// 82BD6E88: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD6E8C: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6E90: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 82BD6E94: 54C436BE  srwi r4, r6, 0x1a
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shr(26);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82BD6E98: 3929EC68  addi r9, r9, -0x1398
	ctx.r[9].s64 = ctx.r[9].s64 + -5016;
	// 82BD6E9C: 554546FE  rlwinm r5, r10, 8, 0x1b, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82BD6EA0: 38690020  addi r3, r9, 0x20
	ctx.r[3].s64 = ctx.r[9].s64 + 32;
	// 82BD6EA4: 7CA548AE  lbzx r5, r5, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82BD6EA8: 7C8418AE  lbzx r4, r4, r3
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD6EAC: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82BD6EB0: 4198001C  blt cr6, 0x82bd6ecc
	if ctx.cr[6].lt {
	pc = 0x82BD6ECC; continue 'dispatch;
	}
	// 82BD6EB4: 55490001  rlwinm. r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6EB8: 41820014  beq 0x82bd6ecc
	if ctx.cr[0].eq {
	pc = 0x82BD6ECC; continue 'dispatch;
	}
	// 82BD6EBC: 554986BE  rlwinm r9, r10, 0x10, 0x1a, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82BD6EC0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD6EC4: 41990008  bgt cr6, 0x82bd6ecc
	if ctx.cr[6].gt {
	pc = 0x82BD6ECC; continue 'dispatch;
	}
	// 82BD6EC8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82BD6ECC: 2B050002  cmplwi cr6, r5, 2
	ctx.cr[6].compare_u32(ctx.r[5].u32, 2 as u32, &mut ctx.xer);
	// 82BD6ED0: 4198001C  blt cr6, 0x82bd6eec
	if ctx.cr[6].lt {
	pc = 0x82BD6EEC; continue 'dispatch;
	}
	// 82BD6ED4: 55490043  rlwinm. r9, r10, 0, 1, 1
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6ED8: 41820014  beq 0x82bd6eec
	if ctx.cr[0].eq {
	pc = 0x82BD6EEC; continue 'dispatch;
	}
	// 82BD6EDC: 5549C6BE  rlwinm r9, r10, 0x18, 0x1a, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82BD6EE0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD6EE4: 41990008  bgt cr6, 0x82bd6eec
	if ctx.cr[6].gt {
	pc = 0x82BD6EEC; continue 'dispatch;
	}
	// 82BD6EE8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82BD6EEC: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 82BD6EF0: 4098000C  bge cr6, 0x82bd6efc
	if !ctx.cr[6].lt {
	pc = 0x82BD6EFC; continue 'dispatch;
	}
	// 82BD6EF4: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82BD6EF8: 409A001C  bne cr6, 0x82bd6f14
	if !ctx.cr[6].eq {
	pc = 0x82BD6F14; continue 'dispatch;
	}
	// 82BD6EFC: 55490085  rlwinm. r9, r10, 0, 2, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6F00: 41820014  beq 0x82bd6f14
	if ctx.cr[0].eq {
	pc = 0x82BD6F14; continue 'dispatch;
	}
	// 82BD6F04: 554906BE  clrlwi r9, r10, 0x1a
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82BD6F08: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD6F0C: 41990008  bgt cr6, 0x82bd6f14
	if ctx.cr[6].gt {
	pc = 0x82BD6F14; continue 'dispatch;
	}
	// 82BD6F10: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82BD6F14: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 82BD6F18: 409A0020  bne cr6, 0x82bd6f38
	if !ctx.cr[6].eq {
	pc = 0x82BD6F38; continue 'dispatch;
	}
	// 82BD6F1C: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD6F20: 5146F108  rlwimi r6, r10, 0x1e, 4, 4
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[6].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82BD6F24: 50C937BE  rlwimi r9, r6, 6, 0x1e, 0x1f
	ctx.r[9].u64 = (((ctx.r[6].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82BD6F28: 552A06BE  clrlwi r10, r9, 0x1a
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82BD6F2C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD6F30: 41990008  bgt cr6, 0x82bd6f38
	if ctx.cr[6].gt {
	pc = 0x82BD6F38; continue 'dispatch;
	}
	// 82BD6F34: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD6F38: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD6F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6F40 size=36
    let mut pc: u32 = 0x82BD6F40;
    'dispatch: loop {
        match pc {
            0x82BD6F40 => {
    //   block [0x82BD6F40..0x82BD6F64)
	// 82BD6F40: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6F44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD6F48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82BD6F4C: 552B053E  clrlwi r11, r9, 0x14
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 82BD6F50: 5527A77F  rlwinm. r7, r9, 0x14, 0x1d, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD6F54: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82BD6F58: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82BD6F5C: 5524853E  rlwinm r4, r9, 0x10, 0x14, 0x1f
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82BD6F60: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6F64 size=124
    let mut pc: u32 = 0x82BD6F64;
    'dispatch: loop {
        match pc {
            0x82BD6F64 => {
    //   block [0x82BD6F64..0x82BD6FE0)
	// 82BD6F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD6F68: 7C6B2830  slw r11, r3, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[3].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD6F6C: 7D6B2039  and. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6F70: 40820058  bne 0x82bd6fc8
	if !ctx.cr[0].eq {
	pc = 0x82BD6FC8; continue 'dispatch;
	}
	// 82BD6F74: 896A0008  lbz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD6F78: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD6F7C: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD6F80: 552936BE  srwi r9, r9, 0x1a
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(26);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD6F84: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82BD6F88: 41980010  blt cr6, 0x82bd6f98
	if ctx.cr[6].lt {
	pc = 0x82BD6F98; continue 'dispatch;
	}
	// 82BD6F8C: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82BD6F90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82BD6F94: 40990008  ble cr6, 0x82bd6f9c
	if !ctx.cr[6].gt {
	pc = 0x82BD6F9C; continue 'dispatch;
	}
	// 82BD6F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6F9C: 5568063E  clrlwi r8, r11, 0x18
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82BD6FA0: 2B09001B  cmplwi cr6, r9, 0x1b
	ctx.cr[6].compare_u32(ctx.r[9].u32, 27 as u32, &mut ctx.xer);
	// 82BD6FA4: 41980010  blt cr6, 0x82bd6fb4
	if ctx.cr[6].lt {
	pc = 0x82BD6FB4; continue 'dispatch;
	}
	// 82BD6FA8: 2B090022  cmplwi cr6, r9, 0x22
	ctx.cr[6].compare_u32(ctx.r[9].u32, 34 as u32, &mut ctx.xer);
	// 82BD6FAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82BD6FB0: 40990008  ble cr6, 0x82bd6fb8
	if !ctx.cr[6].gt {
	pc = 0x82BD6FB8; continue 'dispatch;
	}
	// 82BD6FB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD6FB8: 5509063F  clrlwi. r9, r8, 0x18
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD6FBC: 40820024  bne 0x82bd6fe0
	if !ctx.cr[0].eq {
		sub_82BD6FE0(ctx, base);
		return;
	}
	// 82BD6FC0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6FC4: 4082001C  bne 0x82bd6fe0
	if !ctx.cr[0].eq {
		sub_82BD6FE0(ctx, base);
		return;
	}
	// 82BD6FC8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82BD6FCC: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 82BD6FD0: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD6FD4: 7F063840  cmplw cr6, r6, r7
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82BD6FD8: 4198FF90  blt cr6, 0x82bd6f68
	if ctx.cr[6].lt {
	pc = 0x82BD6F68; continue 'dispatch;
	}
	// 82BD6FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD6FE0 size=8
    let mut pc: u32 = 0x82BD6FE0;
    'dispatch: loop {
        match pc {
            0x82BD6FE0 => {
    //   block [0x82BD6FE0..0x82BD6FE8)
	// 82BD6FE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD6FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD6FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82BD6FE8 size=208
    let mut pc: u32 = 0x82BD6FE8;
    'dispatch: loop {
        match pc {
            0x82BD6FE8 => {
    //   block [0x82BD6FE8..0x82BD70B8)
	// 82BD6FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD6FEC: 485D1181  bl 0x831a816c
	ctx.lr = 0x82BD6FF0;
	sub_831A8130(ctx, base);
	// 82BD6FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD6FF4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82BD6FF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD6FFC: 418200B4  beq 0x82bd70b0
	if ctx.cr[0].eq {
	pc = 0x82BD70B0; continue 'dispatch;
	}
	// 82BD7000: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7004: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD7008: 57FEA73E  rlwinm r30, r31, 0x14, 0x1c, 0x1f
	ctx.r[30].u64 = ctx.r[31].u32 as u64 & 0x00000FFFu64;
	// 82BD700C: 7D6BF030  slw r11, r11, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7010: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD7014: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD7018: 41820098  beq 0x82bd70b0
	if ctx.cr[0].eq {
	pc = 0x82BD70B0; continue 'dispatch;
	}
	// 82BD701C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82BD7020: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82BD7024: 4BFFFF1D  bl 0x82bd6f40
	ctx.lr = 0x82BD7028;
	sub_82BD6F40(ctx, base);
	// 82BD7028: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82BD702C: 2B1E0006  cmplwi cr6, r30, 6
	ctx.cr[6].compare_u32(ctx.r[30].u32, 6 as u32, &mut ctx.xer);
	// 82BD7030: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82BD7034: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD7038: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82BD703C: 41990038  bgt cr6, 0x82bd7074
	if ctx.cr[6].gt {
	pc = 0x82BD7074; continue 'dispatch;
	}
	// 82BD7040: 2B1E0005  cmplwi cr6, r30, 5
	ctx.cr[6].compare_u32(ctx.r[30].u32, 5 as u32, &mut ctx.xer);
	// 82BD7044: 40980028  bge cr6, 0x82bd706c
	if !ctx.cr[6].lt {
	pc = 0x82BD706C; continue 'dispatch;
	}
	// 82BD7048: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD704C: 419A0064  beq cr6, 0x82bd70b0
	if ctx.cr[6].eq {
	pc = 0x82BD70B0; continue 'dispatch;
	}
	// 82BD7050: 2B1E0002  cmplwi cr6, r30, 2
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2 as u32, &mut ctx.xer);
	// 82BD7054: 40990018  ble cr6, 0x82bd706c
	if !ctx.cr[6].gt {
	pc = 0x82BD706C; continue 'dispatch;
	}
	// 82BD7058: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 82BD705C: 419A003C  beq cr6, 0x82bd7098
	if ctx.cr[6].eq {
	pc = 0x82BD7098; continue 'dispatch;
	}
	// 82BD7060: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 82BD7064: 419A0020  beq cr6, 0x82bd7084
	if ctx.cr[6].eq {
	pc = 0x82BD7084; continue 'dispatch;
	}
	// 82BD7068: 48000048  b 0x82bd70b0
	pc = 0x82BD70B0; continue 'dispatch;
	// 82BD706C: 517F4DAC  rlwimi r31, r11, 9, 0x16, 0x16
	ctx.r[31].u64 = (((ctx.r[11].u32).rotate_left(9) as u64) & 0x0000000000000200) | (ctx.r[31].u64 & 0xFFFFFFFFFFFFFDFF);
	// 82BD7070: 4800003C  b 0x82bd70ac
	pc = 0x82BD70AC; continue 'dispatch;
	// 82BD7074: 2B1E000D  cmplwi cr6, r30, 0xd
	ctx.cr[6].compare_u32(ctx.r[30].u32, 13 as u32, &mut ctx.xer);
	// 82BD7078: 419A0020  beq cr6, 0x82bd7098
	if ctx.cr[6].eq {
	pc = 0x82BD7098; continue 'dispatch;
	}
	// 82BD707C: 2B1E000E  cmplwi cr6, r30, 0xe
	ctx.cr[6].compare_u32(ctx.r[30].u32, 14 as u32, &mut ctx.xer);
	// 82BD7080: 409A0030  bne cr6, 0x82bd70b0
	if !ctx.cr[6].eq {
	pc = 0x82BD70B0; continue 'dispatch;
	}
	// 82BD7084: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82BD7088: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82BD708C: 716B000A  andi. r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u64 & 10;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD7090: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD7094: 48000014  b 0x82bd70a8
	pc = 0x82BD70A8; continue 'dispatch;
	// 82BD7098: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82BD709C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82BD70A0: 716B000A  andi. r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u64 & 10;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD70A4: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82BD70A8: 517F6426  rlwimi r31, r11, 0xc, 0x10, 0x13
	ctx.r[31].u64 = (((ctx.r[11].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0FFF);
	// 82BD70AC: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82BD70B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD70B4: 485D1108  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD70B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD70B8 size=208
    let mut pc: u32 = 0x82BD70B8;
    'dispatch: loop {
        match pc {
            0x82BD70B8 => {
    //   block [0x82BD70B8..0x82BD7188)
	// 82BD70B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD70BC: 485D10B1  bl 0x831a816c
	ctx.lr = 0x82BD70C0;
	sub_831A8130(ctx, base);
	// 82BD70C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD70C4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82BD70C8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82BD70CC: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82BD70D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD70D4: 408200AC  bne 0x82bd7180
	if !ctx.cr[0].eq {
	pc = 0x82BD7180; continue 'dispatch;
	}
	// 82BD70D8: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD70DC: 408200A4  bne 0x82bd7180
	if !ctx.cr[0].eq {
	pc = 0x82BD7180; continue 'dispatch;
	}
	// 82BD70E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82BD70E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD70E8: 4BFFFB49  bl 0x82bd6c30
	ctx.lr = 0x82BD70EC;
	sub_82BD6C30(ctx, base);
	// 82BD70EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD70F0: 41820090  beq 0x82bd7180
	if ctx.cr[0].eq {
	pc = 0x82BD7180; continue 'dispatch;
	}
	// 82BD70F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD70F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD70FC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD7100: 556406BE  clrlwi r4, r11, 0x1a
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82BD7104: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD7108: 7D6B2830  slw r11, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD710C: 7D6B3039  and. r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[6].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD7110: 41820064  beq 0x82bd7174
	if ctx.cr[0].eq {
	pc = 0x82BD7174; continue 'dispatch;
	}
	// 82BD7114: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82BD7118: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD711C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82BD7120: 7D6BF8AE  lbzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD7124: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82BD7128: 419A004C  beq cr6, 0x82bd7174
	if ctx.cr[6].eq {
	pc = 0x82BD7174; continue 'dispatch;
	}
	// 82BD712C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD7130: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7134: 396B0028  addi r11, r11, 0x28
	ctx.r[11].s64 = ctx.r[11].s64 + 40;
	// 82BD7138: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD713C: 57BE053E  clrlwi r30, r29, 0x14
	ctx.r[30].u64 = ctx.r[29].u32 as u64 & 0x00000FFFu64;
	// 82BD7140: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7144: 7D0A4A2E  lhzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82BD7148: 5508053E  clrlwi r8, r8, 0x14
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000FFFu64;
	// 82BD714C: 7CEBFA2E  lhzx r7, r11, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD7150: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82BD7154: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD7158: 7FC8192E  stwx r30, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[30].u32) };
	// 82BD715C: 7D0BFA2E  lhzx r8, r11, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD7160: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD7164: 7D0BFB2E  sthx r8, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u16) };
	// 82BD7168: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82BD716C: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD7170: 7CC65878  andc r6, r6, r11
	ctx.r[6].u64 = ctx.r[6].u64 & !ctx.r[11].u64;
	// 82BD7174: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82BD7178: 2B050004  cmplwi cr6, r5, 4
	ctx.cr[6].compare_u32(ctx.r[5].u32, 4 as u32, &mut ctx.xer);
	// 82BD717C: 4198FF88  blt cr6, 0x82bd7104
	if ctx.cr[6].lt {
	pc = 0x82BD7104; continue 'dispatch;
	}
	// 82BD7180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82BD7184: 485D1038  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD7188 size=196
    let mut pc: u32 = 0x82BD7188;
    'dispatch: loop {
        match pc {
            0x82BD7188 => {
    //   block [0x82BD7188..0x82BD724C)
	// 82BD7188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD7190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD7194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD7198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD719C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82BD71A0: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82BD71A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD71A8: 4082008C  bne 0x82bd7234
	if !ctx.cr[0].eq {
	pc = 0x82BD7234; continue 'dispatch;
	}
	// 82BD71AC: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD71B0: 40820084  bne 0x82bd7234
	if !ctx.cr[0].eq {
	pc = 0x82BD7234; continue 'dispatch;
	}
	// 82BD71B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82BD71B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD71BC: 4BFFFA75  bl 0x82bd6c30
	ctx.lr = 0x82BD71C0;
	sub_82BD6C30(ctx, base);
	// 82BD71C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD71C4: 41820070  beq 0x82bd7234
	if ctx.cr[0].eq {
	pc = 0x82BD7234; continue 'dispatch;
	}
	// 82BD71C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD71CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82BD71D0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD71D4: 556506BE  clrlwi r5, r11, 0x1a
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82BD71D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD71DC: 7D6B3030  slw r11, r11, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD71E0: 7D6B3839  and. r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD71E4: 41820044  beq 0x82bd7228
	if ctx.cr[0].eq {
	pc = 0x82BD7228; continue 'dispatch;
	}
	// 82BD71E8: 39650002  addi r11, r5, 2
	ctx.r[11].s64 = ctx.r[5].s64 + 2;
	// 82BD71EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD71F0: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82BD71F4: 7D6BF8AE  lbzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD71F8: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82BD71FC: 419A002C  beq cr6, 0x82bd7228
	if ctx.cr[6].eq {
	pc = 0x82BD7228; continue 'dispatch;
	}
	// 82BD7200: 394B0024  addi r10, r11, 0x24
	ctx.r[10].s64 = ctx.r[11].s64 + 36;
	// 82BD7204: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7208: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD720C: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7210: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD7214: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82BD7218: 7D4BFB2E  sthx r10, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u16) };
	// 82BD721C: 7D69402E  lwzx r11, r9, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD7220: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD7224: 7CE75878  andc r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[11].u64;
	// 82BD7228: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82BD722C: 2B060004  cmplwi cr6, r6, 4
	ctx.cr[6].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 82BD7230: 4198FFA8  blt cr6, 0x82bd71d8
	if ctx.cr[6].lt {
	pc = 0x82BD71D8; continue 'dispatch;
	}
	// 82BD7234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD7238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD723C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD7240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD7244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD7248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7250 size=8
    let mut pc: u32 = 0x82BD7250;
    'dispatch: loop {
        match pc {
            0x82BD7250 => {
    //   block [0x82BD7250..0x82BD7258)
	// 82BD7250: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD7254: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7258 size=8
    let mut pc: u32 = 0x82BD7258;
    'dispatch: loop {
        match pc {
            0x82BD7258 => {
    //   block [0x82BD7258..0x82BD7260)
	// 82BD7258: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD725C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7260 size=48
    let mut pc: u32 = 0x82BD7260;
    'dispatch: loop {
        match pc {
            0x82BD7260 => {
    //   block [0x82BD7260..0x82BD7290)
	// 82BD7260: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7264: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD7268: 4182003C  beq 0x82bd72a4
	if ctx.cr[0].eq {
		sub_82BD7290(ctx, base);
		return;
	}
	// 82BD726C: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82BD7270: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82BD7274: 4199001C  bgt cr6, 0x82bd7290
	if ctx.cr[6].gt {
		sub_82BD7290(ctx, base);
		return;
	}
	// 82BD7278: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD727C: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7280: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7284: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82BD7288: 512B0036  rlwimi r11, r9, 0, 0, 0x1b
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF0) | (ctx.r[11].u64 & 0xFFFFFFFF0000000F);
	// 82BD728C: 48000014  b 0x82bd72a0
	sub_82BD7290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7290 size=60
    let mut pc: u32 = 0x82BD7290;
    'dispatch: loop {
        match pc {
            0x82BD7290 => {
    //   block [0x82BD7290..0x82BD72CC)
	// 82BD7290: 2B0B003D  cmplwi cr6, r11, 0x3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 61 as u32, &mut ctx.xer);
	// 82BD7294: 409A0010  bne cr6, 0x82bd72a4
	if !ctx.cr[6].eq {
	pc = 0x82BD72A4; continue 'dispatch;
	}
	// 82BD7298: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD729C: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82BD72A0: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD72A4: 89670008  lbz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD72A8: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD72AC: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82BD72B0: 4198000C  blt cr6, 0x82bd72bc
	if ctx.cr[6].lt {
	pc = 0x82BD72BC; continue 'dispatch;
	}
	// 82BD72B4: 2B0B001B  cmplwi cr6, r11, 0x1b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 27 as u32, &mut ctx.xer);
	// 82BD72B8: 4099001C  ble cr6, 0x82bd72d4
	if !ctx.cr[6].gt {
		sub_82BD72D4(ctx, base);
		return;
	}
	// 82BD72BC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD72C0: 556B36BE  srwi r11, r11, 0x1a
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(26);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD72C4: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82BD72C8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD72CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD72CC size=8
    let mut pc: u32 = 0x82BD72CC;
    'dispatch: loop {
        match pc {
            0x82BD72CC => {
    //   block [0x82BD72CC..0x82BD72D4)
	// 82BD72CC: 2B0B0027  cmplwi cr6, r11, 0x27
	ctx.cr[6].compare_u32(ctx.r[11].u32, 39 as u32, &mut ctx.xer);
	// 82BD72D0: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD72D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD72D4 size=16
    let mut pc: u32 = 0x82BD72D4;
    'dispatch: loop {
        match pc {
            0x82BD72D4 => {
    //   block [0x82BD72D4..0x82BD72E4)
	// 82BD72D4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD72D8: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82BD72DC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD72E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD72E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD72E8 size=228
    let mut pc: u32 = 0x82BD72E8;
    'dispatch: loop {
        match pc {
            0x82BD72E8 => {
    //   block [0x82BD72E8..0x82BD73CC)
	// 82BD72E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD72EC: 485D0E5D  bl 0x831a8148
	ctx.lr = 0x82BD72F0;
	sub_831A8130(ctx, base);
	// 82BD72F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD72F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82BD72F8: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82BD72FC: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82BD7300: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82BD7304: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 82BD7308: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD730C: 557AA77F  rlwinm. r26, r11, 0x14, 0x1d, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82BD7310: 418200B4  beq 0x82bd73c4
	if ctx.cr[0].eq {
	pc = 0x82BD73C4; continue 'dispatch;
	}
	// 82BD7314: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7318: 5569053E  clrlwi r9, r11, 0x14
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD731C: 5568273E  srwi r8, r11, 0x1c
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD7320: 554626B6  rlwinm r6, r10, 4, 0x1a, 0x1b
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x0FFFFFFFu64;
	// 82BD7324: 1D49000C  mulli r10, r9, 0xc
	ctx.r[10].s64 = ctx.r[9].s64 * 12;
	// 82BD7328: 7FEA3A14  add r31, r10, r7
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82BD732C: 5579853E  rlwinm r25, r11, 0x10, 0x14, 0x1f
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD7330: 7CD84378  or r24, r6, r8
	ctx.r[24].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 82BD7334: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82BD7338: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82BD733C: 419A0088  beq cr6, 0x82bd73c4
	if ctx.cr[6].eq {
	pc = 0x82BD73C4; continue 'dispatch;
	}
	// 82BD7340: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82BD7344: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82BD7348: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD734C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD7350: 41990070  bgt cr6, 0x82bd73c0
	if ctx.cr[6].gt {
	pc = 0x82BD73C0; continue 'dispatch;
	}
	// 82BD7354: 409A0054  bne cr6, 0x82bd73a8
	if !ctx.cr[6].eq {
	pc = 0x82BD73A8; continue 'dispatch;
	}
	// 82BD7358: 7F6BF030  slw r11, r27, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[27].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD735C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7360: 7F69E030  slw r9, r27, r28
	if (ctx.r[28].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[27].u32) << ((ctx.r[28].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7364: 7D6BC038  and r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[24].u64;
	// 82BD7368: 7D29C838  and r9, r9, r25
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[25].u64;
	// 82BD736C: 7D680034  cntlzw r8, r11
	ctx.r[8].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82BD7370: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82BD7374: 554B053E  clrlwi r11, r10, 0x14
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82BD7378: 550ADFFE  rlwinm r10, r8, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82BD737C: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD7380: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82BD7384: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82BD7388: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82BD738C: 69450001  xori r5, r10, 1
	ctx.r[5].u64 = ctx.r[10].u64 ^ 1;
	// 82BD7390: 69240001  xori r4, r9, 1
	ctx.r[4].u64 = ctx.r[9].u64 ^ 1;
	// 82BD7394: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD7398: 7EC903A6  mtctr r22
	ctx.ctr.u64 = ctx.r[22].u64;
	// 82BD739C: 4E800421  bctrl
	ctx.lr = 0x82BD73A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD73A0: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82BD73A4: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD73A8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82BD73AC: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82BD73B0: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82BD73B4: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82BD73B8: 4198FF90  blt cr6, 0x82bd7348
	if ctx.cr[6].lt {
	pc = 0x82BD7348; continue 'dispatch;
	}
	// 82BD73BC: 48000008  b 0x82bd73c4
	pc = 0x82BD73C4; continue 'dispatch;
	// 82BD73C0: 93740000  stw r27, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82BD73C4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82BD73C8: 485D0DD0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD73D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD73D0 size=120
    let mut pc: u32 = 0x82BD73D0;
    'dispatch: loop {
        match pc {
            0x82BD73D0 => {
    //   block [0x82BD73D0..0x82BD7448)
	// 82BD73D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD73D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD73D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD73DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD73E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD73E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD73E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82BD73EC: 807F4DB8  lwz r3, 0x4db8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19896 as u32) ) } as u64;
	// 82BD73F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82BD73F4: 419A001C  beq cr6, 0x82bd7410
	if ctx.cr[6].eq {
	pc = 0x82BD7410; continue 'dispatch;
	}
	// 82BD73F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD73FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82BD7404: 4E800421  bctrl
	ctx.lr = 0x82BD7408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD7408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD740C: 917F4DB8  stw r11, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[11].u32 ) };
	// 82BD7410: 93DF4DB8  stw r30, 0x4db8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19896 as u32), ctx.r[30].u32 ) };
	// 82BD7414: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD7418: 419A0018  beq cr6, 0x82bd7430
	if ctx.cr[6].eq {
	pc = 0x82BD7430; continue 'dispatch;
	}
	// 82BD741C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD7424: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82BD742C: 4E800421  bctrl
	ctx.lr = 0x82BD7430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD7430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD7434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD7438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD743C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD7440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD7444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7448 size=32
    let mut pc: u32 = 0x82BD7448;
    'dispatch: loop {
        match pc {
            0x82BD7448 => {
    //   block [0x82BD7448..0x82BD7468)
	// 82BD7448: 81634DB4  lwz r11, 0x4db4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82BD744C: 708A00F9  andi. r10, r4, 0xf9
	ctx.r[10].u64 = ctx.r[4].u64 & 249;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD7450: 556B076E  rlwinm r11, r11, 0, 0x1d, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD7454: 548907FE  clrlwi r9, r4, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82BD7458: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD745C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD7460: 91634DB4  stw r11, 0x4db4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(19892 as u32), ctx.r[11].u32 ) };
	// 82BD7464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7468 size=28
    let mut pc: u32 = 0x82BD7468;
    'dispatch: loop {
        match pc {
            0x82BD7468 => {
    //   block [0x82BD7468..0x82BD7484)
	// 82BD7468: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD746C: 1D6B26D0  mulli r11, r11, 0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 * 9936;
	// 82BD7470: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD7474: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82BD7478: 908B0068  stw r4, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82BD747C: 90AB006C  stw r5, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[5].u32 ) };
	// 82BD7480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7488 size=4
    let mut pc: u32 = 0x82BD7488;
    'dispatch: loop {
        match pc {
            0x82BD7488 => {
    //   block [0x82BD7488..0x82BD748C)
	// 82BD7488: 4BFFFF48  b 0x82bd73d0
	sub_82BD73D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82BD7490 size=408
    let mut pc: u32 = 0x82BD7490;
    'dispatch: loop {
        match pc {
            0x82BD7490 => {
    //   block [0x82BD7490..0x82BD7628)
	// 82BD7490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD7494: 485D0CBD  bl 0x831a8150
	ctx.lr = 0x82BD7498;
	sub_831A8130(ctx, base);
	// 82BD7498: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD749C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD74A0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82BD74A4: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 82BD74A8: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 82BD74AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD74B0: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82BD74B4: 409A0020  bne cr6, 0x82bd74d4
	if !ctx.cr[6].eq {
	pc = 0x82BD74D4; continue 'dispatch;
	}
	// 82BD74B8: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD74BC: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD74C0: 40820014  bne 0x82bd74d4
	if !ctx.cr[0].eq {
	pc = 0x82BD74D4; continue 'dispatch;
	}
	// 82BD74C4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD74C8: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD74CC: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82BD74D0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD74D4: 56CB077D  rlwinm. r11, r22, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD74D8: 41820148  beq 0x82bd7620
	if ctx.cr[0].eq {
	pc = 0x82BD7620; continue 'dispatch;
	}
	// 82BD74DC: 83F80004  lwz r31, 4(r24)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD74E0: 57EB0529  rlwinm. r11, r31, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD74E4: 41820010  beq 0x82bd74f4
	if ctx.cr[0].eq {
	pc = 0x82BD74F4; continue 'dispatch;
	}
	// 82BD74E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82BD74EC: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82BD74F0: 4800000C  b 0x82bd74fc
	pc = 0x82BD74FC; continue 'dispatch;
	// 82BD74F4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD74F8: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82BD74FC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7500: 5579A77F  rlwinm. r25, r11, 0x14, 0x1d, 0x1f
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82BD7504: 4182011C  beq 0x82bd7620
	if ctx.cr[0].eq {
	pc = 0x82BD7620; continue 'dispatch;
	}
	// 82BD7508: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD750C: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 82BD7510: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD7514: 41980104  blt cr6, 0x82bd7618
	if ctx.cr[6].lt {
	pc = 0x82BD7618; continue 'dispatch;
	}
	// 82BD7518: 1D39000C  mulli r9, r25, 0xc
	ctx.r[9].s64 = ctx.r[25].s64 * 12;
	// 82BD751C: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82BD7520: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82BD7524: 419900F4  bgt cr6, 0x82bd7618
	if ctx.cr[6].gt {
	pc = 0x82BD7618; continue 'dispatch;
	}
	// 82BD7528: 57E926B6  rlwinm r9, r31, 4, 0x1a, 0x1b
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x0FFFFFFFu64;
	// 82BD752C: 5568273E  srwi r8, r11, 0x1c
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD7530: 7FAA3A14  add r29, r10, r7
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82BD7534: 557C853E  rlwinm r28, r11, 0x10, 0x14, 0x1f
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD7538: 7D3A4378  or r26, r9, r8
	ctx.r[26].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82BD753C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82BD7540: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82BD7544: 419A00DC  beq cr6, 0x82bd7620
	if ctx.cr[6].eq {
	pc = 0x82BD7620; continue 'dispatch;
	}
	// 82BD7548: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82BD754C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82BD7550: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82BD7554: 7F6AF030  slw r10, r27, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[27].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7558: 7D6BF030  slw r11, r11, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD755C: 7D6BE038  and r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[28].u64;
	// 82BD7560: 7F69F830  slw r9, r27, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[27].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7564: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82BD7568: 7D4AE038  and r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[28].u64;
	// 82BD756C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD7570: 7D29D038  and r9, r9, r26
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[26].u64;
	// 82BD7574: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82BD7578: 69680001  xori r8, r11, 1
	ctx.r[8].u64 = ctx.r[11].u64 ^ 1;
	// 82BD757C: 7D2B0034  cntlzw r11, r9
	ctx.r[11].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82BD7580: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82BD7584: 5569DFFE  rlwinm r9, r11, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD7588: 554B003C  rlwinm r11, r10, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD758C: 5508063F  clrlwi. r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82BD7590: 692A0001  xori r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u64 ^ 1;
	// 82BD7594: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD7598: 41820008  beq 0x82bd75a0
	if ctx.cr[0].eq {
	pc = 0x82BD75A0; continue 'dispatch;
	}
	// 82BD759C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82BD75A0: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD75A4: 41820008  beq 0x82bd75ac
	if ctx.cr[0].eq {
	pc = 0x82BD75AC; continue 'dispatch;
	}
	// 82BD75A8: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82BD75AC: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD75B0: 55490529  rlwinm. r9, r10, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD75B4: 41820008  beq 0x82bd75bc
	if ctx.cr[0].eq {
	pc = 0x82BD75BC; continue 'dispatch;
	}
	// 82BD75B8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82BD75BC: 554A0426  rlwinm r10, r10, 0, 0x10, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD75C0: 2B0A5000  cmplwi cr6, r10, 0x5000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 20480 as u32, &mut ctx.xer);
	// 82BD75C4: 409A0008  bne cr6, 0x82bd75cc
	if !ctx.cr[6].eq {
	pc = 0x82BD75CC; continue 'dispatch;
	}
	// 82BD75C8: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82BD75CC: 7D6AB038  and r10, r11, r22
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[22].u64;
	// 82BD75D0: 554A077D  rlwinm. r10, r10, 0, 0x1d, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD75D4: 4182002C  beq 0x82bd7600
	if ctx.cr[0].eq {
	pc = 0x82BD7600; continue 'dispatch;
	}
	// 82BD75D8: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD75DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82BD75E0: 80C10104  lwz r6, 0x104(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82BD75E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82BD75E8: 554A053E  clrlwi r10, r10, 0x14
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82BD75EC: 7C8AFA14  add r4, r10, r31
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82BD75F0: 7EE903A6  mtctr r23
	ctx.ctr.u64 = ctx.r[23].u64;
	// 82BD75F4: 4E800421  bctrl
	ctx.lr = 0x82BD75F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD75F8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD75FC: 41800024  blt 0x82bd7620
	if ctx.cr[0].lt {
	pc = 0x82BD7620; continue 'dispatch;
	}
	// 82BD7600: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82BD7604: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82BD7608: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82BD760C: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82BD7610: 4198FF40  blt cr6, 0x82bd7550
	if ctx.cr[6].lt {
	pc = 0x82BD7550; continue 'dispatch;
	}
	// 82BD7614: 4800000C  b 0x82bd7620
	pc = 0x82BD7620; continue 'dispatch;
	// 82BD7618: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82BD761C: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	// 82BD7620: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82BD7624: 485D0B7C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD7628 size=888
    let mut pc: u32 = 0x82BD7628;
    'dispatch: loop {
        match pc {
            0x82BD7628 => {
    //   block [0x82BD7628..0x82BD79A0)
	// 82BD7628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD762C: 485D0B05  bl 0x831a8130
	ctx.lr = 0x82BD7630;
	sub_831A8130(ctx, base);
	// 82BD7630: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD7634: 7D545378  mr r20, r10
	ctx.r[20].u64 = ctx.r[10].u64;
	// 82BD7638: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82BD763C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82BD7640: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82BD7644: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82BD7648: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82BD764C: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82BD7650: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82BD7654: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82BD7658: 7D0E4378  mr r14, r8
	ctx.r[14].u64 = ctx.r[8].u64;
	// 82BD765C: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 82BD7660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD7664: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82BD7668: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82BD766C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82BD7670: 3A20FFFF  li r17, -1
	ctx.r[17].s64 = -1;
	// 82BD7674: 39E0FFFF  li r15, -1
	ctx.r[15].s64 = -1;
	// 82BD7678: 3ACBFFFC  addi r22, r11, -4
	ctx.r[22].s64 = ctx.r[11].s64 + -4;
	// 82BD767C: 3B2AFFFC  addi r25, r10, -4
	ctx.r[25].s64 = ctx.r[10].s64 + -4;
	// 82BD7680: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82BD7684: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82BD7688: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82BD768C: 7D7F5B96  divwu r11, r31, r11
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[11].u32;
	// 82BD7690: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD7694: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82BD7698: 7D4BF850  subf r10, r11, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82BD769C: 7D4A4B96  divwu r10, r10, r9
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 82BD76A0: 419A0014  beq cr6, 0x82bd76b4
	if ctx.cr[6].eq {
	pc = 0x82BD76B4; continue 'dispatch;
	}
	// 82BD76A4: 7F1F8040  cmplw cr6, r31, r16
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[16].u32, &mut ctx.xer);
	// 82BD76A8: 409802E8  bge cr6, 0x82bd7990
	if !ctx.cr[6].lt {
	pc = 0x82BD7990; continue 'dispatch;
	}
	// 82BD76AC: 7F8B9214  add r28, r11, r18
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82BD76B0: 48000014  b 0x82bd76c4
	pc = 0x82BD76C4; continue 'dispatch;
	// 82BD76B4: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82BD76B8: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82BD76BC: 409802D4  bge cr6, 0x82bd7990
	if !ctx.cr[6].lt {
	pc = 0x82BD7990; continue 'dispatch;
	}
	// 82BD76C0: 7F8B9A14  add r28, r11, r19
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82BD76C4: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD76C8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD76CC: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82BD76D0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82BD76D4: 56A707FF  clrlwi. r7, r21, 0x1f
	ctx.r[7].u64 = ctx.r[21].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD76D8: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD76DC: A17C0006  lhz r11, 6(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(6 as u32) ) } as u64;
	// 82BD76E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82BD76E4: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD76E8: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD76EC: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD76F0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD76F4: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82BD76F8: A17C0008  lhz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD76FC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82BD7700: 41820024  beq 0x82bd7724
	if ctx.cr[0].eq {
	pc = 0x82BD7724; continue 'dispatch;
	}
	// 82BD7704: 80C10194  lwz r6, 0x194(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(404 as u32) ) } as u64;
	// 82BD7708: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82BD770C: 7C9F4B96  divwu r4, r31, r9
	ctx.r[4].u32 = ctx.r[31].u32 / ctx.r[9].u32;
	// 82BD7710: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD7714: 7E8903A6  mtctr r20
	ctx.ctr.u64 = ctx.r[20].u64;
	// 82BD7718: 4E800421  bctrl
	ctx.lr = 0x82BD771C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD771C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD7720: 41800278  blt 0x82bd7998
	if ctx.cr[0].lt {
	pc = 0x82BD7998; continue 'dispatch;
	}
	// 82BD7724: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7728: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82BD772C: 556AA73E  rlwinm r10, r11, 0x14, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD7730: 2B0A000F  cmplwi cr6, r10, 0xf
	ctx.cr[6].compare_u32(ctx.r[10].u32, 15 as u32, &mut ctx.xer);
	// 82BD7734: 419901E4  bgt cr6, 0x82bd7918
	if ctx.cr[6].gt {
	pc = 0x82BD7918; continue 'dispatch;
	}
	// 82BD7738: 3D80820D  lis r12, -0x7df3
	ctx.r[12].s64 = -2113077248;
	// 82BD773C: 398CC478  addi r12, r12, -0x3b88
	ctx.r[12].s64 = ctx.r[12].s64 + -15240;
	// 82BD7740: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82BD7744: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82BD7748: 3D8082BD  lis r12, -0x7d43
	ctx.r[12].s64 = -2101542912;
	// 82BD774C: 398C7760  addi r12, r12, 0x7760
	ctx.r[12].s64 = ctx.r[12].s64 + 30560;
	// 82BD7750: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82BD7754: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82BD7758: 60000000  nop
	// 82BD775C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82BD7760: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82BD7764: 480001B0  b 0x82bd7914
	pc = 0x82BD7914; continue 'dispatch;
	// 82BD7768: 556ADEFA  rlwinm r10, r11, 0x1b, 0x1b, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD776C: 5569F6FE  rlwinm r9, r11, 0x1e, 0x1b, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD7770: 556BB7FE  rlwinm r11, r11, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82BD7774: 7F494830  slw r9, r26, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[26].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7778: 7D4AB82E  lwzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82BD777C: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD7780: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82BD7784: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD7788: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82BD778C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD7790: 409A0184  bne cr6, 0x82bd7914
	if !ctx.cr[6].eq {
	pc = 0x82BD7914; continue 'dispatch;
	}
	// 82BD7794: 4BFFFFCC  b 0x82bd7760
	pc = 0x82BD7760; continue 'dispatch;
	// 82BD7798: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD779C: 5549967A  rlwinm r9, r10, 0x12, 0x19, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82BD77A0: 7D29702E  lwzx r9, r9, r14
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82BD77A4: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD77A8: 418200C4  beq 0x82bd786c
	if ctx.cr[0].eq {
	pc = 0x82BD786C; continue 'dispatch;
	}
	// 82BD77AC: 39EF0001  addi r15, r15, 1
	ctx.r[15].s64 = ctx.r[15].s64 + 1;
	// 82BD77B0: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82BD77B4: 2F0F0004  cmpwi cr6, r15, 4
	ctx.cr[6].compare_i32(ctx.r[15].s32, 4, &mut ctx.xer);
	// 82BD77B8: 4198000C  blt cr6, 0x82bd77c4
	if ctx.cr[6].lt {
	pc = 0x82BD77C4; continue 'dispatch;
	}
	// 82BD77BC: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82BD77C0: 48000158  b 0x82bd7918
	pc = 0x82BD7918; continue 'dispatch;
	// 82BD77C4: 91190000  stw r8, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82BD77C8: 4800014C  b 0x82bd7914
	pc = 0x82BD7914; continue 'dispatch;
	// 82BD77CC: 2F0F0000  cmpwi cr6, r15, 0
	ctx.cr[6].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 82BD77D0: 4198FFEC  blt cr6, 0x82bd77bc
	if ctx.cr[6].lt {
	pc = 0x82BD77BC; continue 'dispatch;
	}
	// 82BD77D4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD77D8: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD77DC: 5527967A  rlwinm r7, r9, 0x12, 0x19, 0x1d
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 82BD77E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82BD77E4: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD77E8: 7CE7702E  lwzx r7, r7, r14
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82BD77EC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82BD77F0: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82BD77F4: 4098000C  bge cr6, 0x82bd7800
	if !ctx.cr[6].lt {
	pc = 0x82BD7800; continue 'dispatch;
	}
	// 82BD77F8: 552A04FE  clrlwi r10, r9, 0x13
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00001FFFu64;
	// 82BD77FC: 48000074  b 0x82bd7870
	pc = 0x82BD7870; continue 'dispatch;
	// 82BD7800: 39EFFFFF  addi r15, r15, -1
	ctx.r[15].s64 = ctx.r[15].s64 + -1;
	// 82BD7804: 3B39FFFC  addi r25, r25, -4
	ctx.r[25].s64 = ctx.r[25].s64 + -4;
	// 82BD7808: 4800010C  b 0x82bd7914
	pc = 0x82BD7914; continue 'dispatch;
	// 82BD780C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7810: 554904A5  rlwinm. r9, r10, 0, 0x12, 0x12
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD7814: 40820038  bne 0x82bd784c
	if !ctx.cr[0].eq {
	pc = 0x82BD784C; continue 'dispatch;
	}
	// 82BD7818: 55490463  rlwinm. r9, r10, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD781C: 408200F8  bne 0x82bd7914
	if !ctx.cr[0].eq {
	pc = 0x82BD7914; continue 'dispatch;
	}
	// 82BD7820: 5569DEFA  rlwinm r9, r11, 0x1b, 0x1b, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD7824: 5567F6FE  rlwinm r7, r11, 0x1e, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD7828: 5566B7FE  rlwinm r6, r11, 0x16, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82BD782C: 7F473830  slw r7, r26, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[26].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7830: 7D29B82E  lwzx r9, r9, r23
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82BD7834: 7CE94838  and r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82BD7838: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82BD783C: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD7840: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 82BD7844: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82BD7848: 409A00CC  bne cr6, 0x82bd7914
	if !ctx.cr[6].eq {
	pc = 0x82BD7914; continue 'dispatch;
	}
	// 82BD784C: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 82BD7850: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82BD7854: 2F110004  cmpwi cr6, r17, 4
	ctx.cr[6].compare_i32(ctx.r[17].s32, 4, &mut ctx.xer);
	// 82BD7858: 4098FF64  bge cr6, 0x82bd77bc
	if !ctx.cr[6].lt {
	pc = 0x82BD77BC; continue 'dispatch;
	}
	// 82BD785C: 57A9801E  slwi r9, r29, 0x10
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD7860: 38FF0006  addi r7, r31, 6
	ctx.r[7].s64 = ctx.r[31].s64 + 6;
	// 82BD7864: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD7868: 91360000  stw r9, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD786C: 554A04FE  clrlwi r10, r10, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00001FFFu64;
	// 82BD7870: 557DAFFE  rlwinm r29, r11, 0x15, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 82BD7874: 1FEA0006  mulli r31, r10, 6
	ctx.r[31].s64 = ctx.r[10].s64 * 6;
	// 82BD7878: 480000A0  b 0x82bd7918
	pc = 0x82BD7918; continue 'dispatch;
	// 82BD787C: 2F110000  cmpwi cr6, r17, 0
	ctx.cr[6].compare_i32(ctx.r[17].s32, 0, &mut ctx.xer);
	// 82BD7880: 4198FF3C  blt cr6, 0x82bd77bc
	if ctx.cr[6].lt {
	pc = 0x82BD77BC; continue 'dispatch;
	}
	// 82BD7884: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7888: 3A31FFFF  addi r17, r17, -1
	ctx.r[17].s64 = ctx.r[17].s64 + -1;
	// 82BD788C: 3AD6FFFC  addi r22, r22, -4
	ctx.r[22].s64 = ctx.r[22].s64 + -4;
	// 82BD7890: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD7894: 557D843E  srwi r29, r11, 0x10
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82BD7898: 48000080  b 0x82bd7918
	pc = 0x82BD7918; continue 'dispatch;
	// 82BD789C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD78A0: 554904A5  rlwinm. r9, r10, 0, 0x12, 0x12
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD78A4: 4082FFC8  bne 0x82bd786c
	if !ctx.cr[0].eq {
	pc = 0x82BD786C; continue 'dispatch;
	}
	// 82BD78A8: 55490463  rlwinm. r9, r10, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD78AC: 40820068  bne 0x82bd7914
	if !ctx.cr[0].eq {
	pc = 0x82BD7914; continue 'dispatch;
	}
	// 82BD78B0: 5569DEFA  rlwinm r9, r11, 0x1b, 0x1b, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD78B4: 5567F6FE  rlwinm r7, r11, 0x1e, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD78B8: 5566B7FE  rlwinm r6, r11, 0x16, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82BD78BC: 7F473830  slw r7, r26, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[26].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD78C0: 7D29B82E  lwzx r9, r9, r23
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82BD78C4: 7CE94838  and r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82BD78C8: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82BD78CC: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD78D0: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 82BD78D4: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82BD78D8: 409A003C  bne cr6, 0x82bd7914
	if !ctx.cr[6].eq {
	pc = 0x82BD7914; continue 'dispatch;
	}
	// 82BD78DC: 4BFFFF90  b 0x82bd786c
	pc = 0x82BD786C; continue 'dispatch;
	// 82BD78E0: 556ADEFA  rlwinm r10, r11, 0x1b, 0x1b, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD78E4: 5569F6FE  rlwinm r9, r11, 0x1e, 0x1b, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82BD78E8: 556BB7FE  rlwinm r11, r11, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82BD78EC: 7F494830  slw r9, r26, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[26].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD78F0: 7D4AB82E  lwzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82BD78F4: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD78F8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82BD78FC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD7900: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82BD7904: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD7908: 409A000C  bne cr6, 0x82bd7914
	if !ctx.cr[6].eq {
	pc = 0x82BD7914; continue 'dispatch;
	}
	// 82BD790C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82BD7910: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82BD7914: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82BD7918: 550B063F  clrlwi. r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD791C: 41820038  beq 0x82bd7954
	if ctx.cr[0].eq {
	pc = 0x82BD7954; continue 'dispatch;
	}
	// 82BD7920: 81610194  lwz r11, 0x194(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(404 as u32) ) } as u64;
	// 82BD7924: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 82BD7928: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 82BD792C: 7E088378  mr r8, r16
	ctx.r[8].u64 = ctx.r[16].u64;
	// 82BD7930: 7E479378  mr r7, r18
	ctx.r[7].u64 = ctx.r[18].u64;
	// 82BD7934: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82BD7938: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 82BD793C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82BD7940: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82BD7944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD7948: 4BFFFB49  bl 0x82bd7490
	ctx.lr = 0x82BD794C;
	sub_82BD7490(ctx, base);
	// 82BD794C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD7950: 41800048  blt 0x82bd7998
	if ctx.cr[0].lt {
	pc = 0x82BD7998; continue 'dispatch;
	}
	// 82BD7954: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82BD7958: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD795C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD7960: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82BD7964: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82BD7968: 514B801E  rlwimi r11, r10, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82BD796C: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD7970: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82BD7974: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD7978: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82BD797C: 556B801E  slwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7980: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD7984: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD7988: 4182FCFC  beq 0x82bd7684
	if ctx.cr[0].eq {
	pc = 0x82BD7684; continue 'dispatch;
	}
	// 82BD798C: 4800000C  b 0x82bd7998
	pc = 0x82BD7998; continue 'dispatch;
	// 82BD7990: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82BD7994: 60634005  ori r3, r3, 0x4005
	ctx.r[3].u64 = ctx.r[3].u64 | 16389;
	// 82BD7998: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82BD799C: 485D07E4  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD79A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD79A0 size=372
    let mut pc: u32 = 0x82BD79A0;
    'dispatch: loop {
        match pc {
            0x82BD79A0 => {
    //   block [0x82BD79A0..0x82BD7B14)
	// 82BD79A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD79A4: 485D079D  bl 0x831a8140
	ctx.lr = 0x82BD79A8;
	sub_831A8130(ctx, base);
	// 82BD79A8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD79AC: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82BD79B0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82BD79B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82BD79B8: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82BD79BC: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82BD79C0: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 82BD79C4: 578B0739  rlwinm. r11, r28, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD79C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD79CC: 41820018  beq 0x82bd79e4
	if ctx.cr[0].eq {
	pc = 0x82BD79E4; continue 'dispatch;
	}
	// 82BD79D0: 81610154  lwz r11, 0x154(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 82BD79D4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82BD79D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82BD79DC: 4BFFFC4D  bl 0x82bd7628
	ctx.lr = 0x82BD79E0;
	sub_82BD7628(ctx, base);
	// 82BD79E0: 4800012C  b 0x82bd7b0c
	pc = 0x82BD7B0C; continue 'dispatch;
	// 82BD79E4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82BD79E8: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82BD79EC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82BD79F0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD79F4: 419A0118  beq cr6, 0x82bd7b0c
	if ctx.cr[6].eq {
	pc = 0x82BD7B0C; continue 'dispatch;
	}
	// 82BD79F8: 82E10154  lwz r23, 0x154(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 82BD79FC: 579207FE  clrlwi r18, r28, 0x1f
	ctx.r[18].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	// 82BD7A00: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7A04: 3A9E0004  addi r20, r30, 4
	ctx.r[20].s64 = ctx.r[30].s64 + 4;
	// 82BD7A08: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD7A0C: 3A7E0008  addi r19, r30, 8
	ctx.r[19].s64 = ctx.r[30].s64 + 8;
	// 82BD7A10: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7A14: 5568843E  srwi r8, r11, 0x10
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD7A18: 5547801E  slwi r7, r10, 0x10
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD7A1C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD7A20: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82BD7A24: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD7A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82BD7A2C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82BD7A30: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82BD7A34: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 82BD7A38: 91010078  stw r8, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u32 ) };
	// 82BD7A3C: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82BD7A40: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 82BD7A44: 419A002C  beq cr6, 0x82bd7a70
	if ctx.cr[6].eq {
	pc = 0x82BD7A70; continue 'dispatch;
	}
	// 82BD7A48: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82BD7A4C: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82BD7A50: 7D785B96  divwu r11, r24, r11
	ctx.r[11].u32 = ctx.r[24].u32 / ctx.r[11].u32;
	// 82BD7A54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82BD7A58: 7C8BD214  add r4, r11, r26
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82BD7A5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD7A60: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 82BD7A64: 4E800421  bctrl
	ctx.lr = 0x82BD7A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD7A68: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD7A6C: 418000A0  blt 0x82bd7b0c
	if ctx.cr[0].lt {
	pc = 0x82BD7B0C; continue 'dispatch;
	}
	// 82BD7A70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7A74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD7A78: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD7A7C: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7A80: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD7A84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD7A88: 41820034  beq 0x82bd7abc
	if ctx.cr[0].eq {
	pc = 0x82BD7ABC; continue 'dispatch;
	}
	// 82BD7A8C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82BD7A90: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82BD7A94: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82BD7A98: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82BD7A9C: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 82BD7AA0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82BD7AA4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82BD7AA8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82BD7AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD7AB0: 4BFFF9E1  bl 0x82bd7490
	ctx.lr = 0x82BD7AB4;
	sub_82BD7490(ctx, base);
	// 82BD7AB4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82BD7AB8: 41800054  blt 0x82bd7b0c
	if ctx.cr[0].lt {
	pc = 0x82BD7B0C; continue 'dispatch;
	}
	// 82BD7ABC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82BD7AC0: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82BD7AC4: 2B1A0002  cmplwi cr6, r26, 2
	ctx.cr[6].compare_u32(ctx.r[26].u32, 2 as u32, &mut ctx.xer);
	// 82BD7AC8: 4198FF78  blt cr6, 0x82bd7a40
	if ctx.cr[6].lt {
	pc = 0x82BD7A40; continue 'dispatch;
	}
	// 82BD7ACC: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82BD7AD0: 3B18000C  addi r24, r24, 0xc
	ctx.r[24].s64 = ctx.r[24].s64 + 12;
	// 82BD7AD4: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82BD7AD8: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82BD7ADC: 5568843E  srwi r8, r11, 0x10
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD7AE0: 80E10074  lwz r7, 0x74(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82BD7AE4: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD7AE8: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82BD7AEC: 5167801E  rlwimi r7, r11, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82BD7AF0: 7D4B4378  or r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82BD7AF4: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD7AF8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82BD7AFC: 90F40000  stw r7, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82BD7B00: 7F183040  cmplw cr6, r24, r6
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82BD7B04: 91730000  stw r11, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD7B08: 4198FEF8  blt cr6, 0x82bd7a00
	if ctx.cr[6].lt {
	pc = 0x82BD7A00; continue 'dispatch;
	}
	// 82BD7B0C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82BD7B10: 485D0680  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD7B18 size=132
    let mut pc: u32 = 0x82BD7B18;
    'dispatch: loop {
        match pc {
            0x82BD7B18 => {
    //   block [0x82BD7B18..0x82BD7B9C)
	// 82BD7B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD7B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD7B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD7B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD7B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD7B2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82BD7B30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82BD7B34: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7B38: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82BD7B3C: 41990044  bgt cr6, 0x82bd7b80
	if ctx.cr[6].gt {
	pc = 0x82BD7B80; continue 'dispatch;
	}
	// 82BD7B40: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7B44: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD7B48: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD7B4C: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD7B50: 40980030  bge cr6, 0x82bd7b80
	if !ctx.cr[6].lt {
	pc = 0x82BD7B80; continue 'dispatch;
	}
	// 82BD7B54: 7D5F5050  subf r10, r31, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 82BD7B58: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82BD7B5C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD7B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD7B64: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82BD7B68: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82BD7B6C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82BD7B70: 485D57E1  bl 0x831ad350
	ctx.lr = 0x82BD7B74;
	sub_831AD350(ctx, base);
	// 82BD7B74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD7B78: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82BD7B7C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD7B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD7B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD7B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD7B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD7B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD7B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD7B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7BA0 size=44
    let mut pc: u32 = 0x82BD7BA0;
    'dispatch: loop {
        match pc {
            0x82BD7BA0 => {
    //   block [0x82BD7BA0..0x82BD7BCC)
	// 82BD7BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD7BA4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82BD7BA8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7BAC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82BD7BB0: 409A0014  bne cr6, 0x82bd7bc4
	if !ctx.cr[6].eq {
	pc = 0x82BD7BC4; continue 'dispatch;
	}
	// 82BD7BB4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82BD7BB8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82BD7BBC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82BD7BC0: 4198FFE8  blt cr6, 0x82bd7ba8
	if ctx.cr[6].lt {
	pc = 0x82BD7BA8; continue 'dispatch;
	}
	// 82BD7BC4: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7BC8: 48000024  b 0x82bd7bec
	sub_82BD7BCC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7BCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7BCC size=48
    let mut pc: u32 = 0x82BD7BCC;
    'dispatch: loop {
        match pc {
            0x82BD7BCC => {
    //   block [0x82BD7BCC..0x82BD7BFC)
	// 82BD7BCC: 556AE8FA  rlwinm r10, r11, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD7BD0: 556906FE  clrlwi r9, r11, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD7BD4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD7BD8: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD7BDC: 7D4A182E  lwzx r10, r10, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD7BE0: 7D2A5039  and. r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD7BE4: 40820010  bne 0x82bd7bf4
	if !ctx.cr[0].eq {
	pc = 0x82BD7BF4; continue 'dispatch;
	}
	// 82BD7BE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82BD7BEC: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82BD7BF0: 4198FFDC  blt cr6, 0x82bd7bcc
	if ctx.cr[6].lt {
	pc = 0x82BD7BCC; continue 'dispatch;
	}
	// 82BD7BF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82BD7BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD7C00 size=472
    let mut pc: u32 = 0x82BD7C00;
    'dispatch: loop {
        match pc {
            0x82BD7C00 => {
    //   block [0x82BD7C00..0x82BD7DD8)
	// 82BD7C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD7C04: 485D0539  bl 0x831a813c
	ctx.lr = 0x82BD7C08;
	sub_831A8130(ctx, base);
	// 82BD7C08: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD7C0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82BD7C10: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 82BD7C14: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82BD7C18: 614A3B00  ori r10, r10, 0x3b00
	ctx.r[10].u64 = ctx.r[10].u64 | 15104;
	// 82BD7C1C: 39200300  li r9, 0x300
	ctx.r[9].s64 = 768;
	// 82BD7C20: 3D00C019  lis r8, -0x3fe7
	ctx.r[8].s64 = -1072103424;
	// 82BD7C24: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82BD7C28: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7C2C: 61082B00  ori r8, r8, 0x2b00
	ctx.r[8].u64 = ctx.r[8].u64 | 11008;
	// 82BD7C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD7C34: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 82BD7C38: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82BD7C3C: 3BA6C2D0  addi r29, r6, -0x3d30
	ctx.r[29].s64 = ctx.r[6].s64 + -15664;
	// 82BD7C40: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7C44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82BD7C48: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7C4C: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7C50: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82BD7C54: 94FE0004  stwu r7, 4(r30)
	ea = ctx.r[30].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[30].u32 = ea;
	// 82BD7C58: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82BD7C5C: 485D08B5  bl 0x831a8510
	ctx.lr = 0x82BD7C60;
	sub_831A8510(ctx, base);
	// 82BD7C60: 397E0060  addi r11, r30, 0x60
	ctx.r[11].s64 = ctx.r[30].s64 + 96;
	// 82BD7C64: 3D40C00A  lis r10, -0x3ff6
	ctx.r[10].s64 = -1073086464;
	// 82BD7C68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD7C6C: 614A2B00  ori r10, r10, 0x2b00
	ctx.r[10].u64 = ctx.r[10].u64 | 11008;
	// 82BD7C70: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82BD7C74: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7C78: 389D0060  addi r4, r29, 0x60
	ctx.r[4].s64 = ctx.r[29].s64 + 96;
	// 82BD7C7C: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 82BD7C80: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7C84: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82BD7C88: 951E0004  stwu r8, 4(r30)
	ea = ctx.r[30].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[30].u32 = ea;
	// 82BD7C8C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82BD7C90: 485D0881  bl 0x831a8510
	ctx.lr = 0x82BD7C94;
	sub_831A8510(ctx, base);
	// 82BD7C94: 397E0024  addi r11, r30, 0x24
	ctx.r[11].s64 = ctx.r[30].s64 + 36;
	// 82BD7C98: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82BD7C9C: 3D201000  lis r9, 0x1000
	ctx.r[9].s64 = 268435456;
	// 82BD7CA0: 614A2180  ori r10, r10, 0x2180
	ctx.r[10].u64 = ctx.r[10].u64 | 8576;
	// 82BD7CA4: 6129000E  ori r9, r9, 0xe
	ctx.r[9].u64 = ctx.r[9].u64 | 14;
	// 82BD7CA8: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD7CB0: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82BD7CB4: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 82BD7CB8: 61082100  ori r8, r8, 0x2100
	ctx.r[8].u64 = ctx.r[8].u64 | 8448;
	// 82BD7CBC: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 82BD7CC0: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82BD7CC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82BD7CCC: 38A02293  li r5, 0x2293
	ctx.r[5].s64 = 8851;
	// 82BD7CD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD7CD4: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 82BD7CD8: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7CDC: 3FC00000  lis r30, 0
	ctx.r[30].s64 = 0;
	// 82BD7CE0: 60632204  ori r3, r3, 0x2204
	ctx.r[3].u64 = ctx.r[3].u64 | 8708;
	// 82BD7CE4: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82BD7CE8: 3EC00008  lis r22, 8
	ctx.r[22].s64 = 524288;
	// 82BD7CEC: 3FA00001  lis r29, 1
	ctx.r[29].s64 = 65536;
	// 82BD7CF0: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7CF4: 3B800300  li r28, 0x300
	ctx.r[28].s64 = 768;
	// 82BD7CF8: 3B602312  li r27, 0x2312
	ctx.r[27].s64 = 8978;
	// 82BD7CFC: 63DEFFFF  ori r30, r30, 0xffff
	ctx.r[30].u64 = ctx.r[30].u64 | 65535;
	// 82BD7D00: 3B40200D  li r26, 0x200d
	ctx.r[26].s64 = 8205;
	// 82BD7D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82BD7D08: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D0C: 3B202200  li r25, 0x2200
	ctx.r[25].s64 = 8704;
	// 82BD7D10: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82BD7D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82BD7D18: 3AE02280  li r23, 0x2280
	ctx.r[23].s64 = 8832;
	// 82BD7D1C: 3AA02302  li r21, 0x2302
	ctx.r[21].s64 = 8962;
	// 82BD7D20: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D24: 38C02208  li r6, 0x2208
	ctx.r[6].s64 = 8712;
	// 82BD7D28: 62D60008  ori r22, r22, 8
	ctx.r[22].u64 = ctx.r[22].u64 | 8;
	// 82BD7D2C: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D30: 39202203  li r9, 0x2203
	ctx.r[9].s64 = 8707;
	// 82BD7D34: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D38: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82BD7D3C: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D40: 38802104  li r4, 0x2104
	ctx.r[4].s64 = 8452;
	// 82BD7D44: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD7D4C: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D50: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D54: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82BD7D58: 3E800002  lis r20, 2
	ctx.r[20].s64 = 131072;
	// 82BD7D5C: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82BD7D60: 62942080  ori r20, r20, 0x2080
	ctx.r[20].u64 = ctx.r[20].u64 | 8320;
	// 82BD7D64: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82BD7D68: 978B0004  stwu r28, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[28].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D6C: 3E200010  lis r17, 0x10
	ctx.r[17].s64 = 1048576;
	// 82BD7D70: 623D0010  ori r29, r17, 0x10
	ctx.r[29].u64 = ctx.r[17].u64 | 16;
	// 82BD7D74: 976B0004  stwu r27, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[27].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D78: 97CB0004  stwu r30, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D7C: 974B0004  stwu r26, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[26].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D80: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D84: 972B0004  stwu r25, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[25].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D88: 970B0004  stwu r24, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[24].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D8C: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D90: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D94: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D98: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7D9C: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DA0: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DA4: 96EB0004  stwu r23, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[23].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DA8: 96CB0004  stwu r22, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[22].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DAC: 96AB0004  stwu r21, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[21].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DB0: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DB4: 968B0004  stwu r20, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[20].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DB8: 966B0004  stwu r19, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[19].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DBC: 964B0004  stwu r18, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[18].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DC0: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7DC4: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82BD7DC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD7DCC: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82BD7DD0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82BD7DD4: 485D03B8  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD7DD8 size=280
    let mut pc: u32 = 0x82BD7DD8;
    'dispatch: loop {
        match pc {
            0x82BD7DD8 => {
    //   block [0x82BD7DD8..0x82BD7EF0)
	// 82BD7DD8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD7DDC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD7DE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD7DE4: 798C4FE6  rldicr r12, r12, 0x29, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(41) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7DE8: 7D696378  or r9, r11, r12
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD7DEC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD7DF0: 794B1FE6  rldicr r11, r10, 0x23, 0x3f
	ctx.r[11].u64 = (ctx.r[10].u64).rotate_left(35) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7DF4: F9230010  std r9, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u64 ) };
	// 82BD7DF8: 798C47E6  rldicr r12, r12, 0x28, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(40) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7DFC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82BD7E00: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD7E04: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD7E08: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E0C: 798C3FE6  rldicr r12, r12, 0x27, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(39) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7E10: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD7E14: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD7E18: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E1C: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD7E20: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD7E24: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82BD7E28: 798C67E6  rldicr r12, r12, 0x2c, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(44) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7E2C: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD7E30: 614A0080  ori r10, r10, 0x80
	ctx.r[10].u64 = ctx.r[10].u64 | 128;
	// 82BD7E34: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E38: 614A0040  ori r10, r10, 0x40
	ctx.r[10].u64 = ctx.r[10].u64 | 64;
	// 82BD7E3C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E40: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 82BD7E44: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E48: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD7E4C: 654A0008  oris r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 524288;
	// 82BD7E50: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82BD7E54: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD7E58: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD7E5C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E60: 654A0008  oris r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 524288;
	// 82BD7E64: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD7E68: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E6C: 654A0010  oris r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 1048576;
	// 82BD7E70: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7E74: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E78: 614A0800  ori r10, r10, 0x800
	ctx.r[10].u64 = ctx.r[10].u64 | 2048;
	// 82BD7E7C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E80: 614A0100  ori r10, r10, 0x100
	ctx.r[10].u64 = ctx.r[10].u64 | 256;
	// 82BD7E84: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E88: 614A0008  ori r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 8;
	// 82BD7E8C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E90: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD7E94: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD7E98: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82BD7E9C: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 82BD7EA0: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD7EA4: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD7EA8: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82BD7EAC: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD7EB0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD7EB4: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD7EB8: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82BD7EBC: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82BD7EC0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD7EC4: 816328C4  lwz r11, 0x28c4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10436 as u32) ) } as u64;
	// 82BD7EC8: 814328C8  lwz r10, 0x28c8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10440 as u32) ) } as u64;
	// 82BD7ECC: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD7ED0: 554A881C  slwi r10, r10, 0x11
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(17);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD7ED4: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD7ED8: 7D278E70  srawi r7, r9, 0x11
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 17) as i64;
	// 82BD7EDC: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7EE0: 7D468E70  srawi r6, r10, 0x11
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[10].s32 >> 17) as i64;
	// 82BD7EE4: 7D058E70  srawi r5, r8, 0x11
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[8].s32 >> 17) as i64;
	// 82BD7EE8: 7D648E70  srawi r4, r11, 0x11
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 17) as i64;
	// 82BD7EEC: 4BFFB0F4  b 0x82bd2fe0
	sub_82BD2FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD7EF0 size=96
    let mut pc: u32 = 0x82BD7EF0;
    'dispatch: loop {
        match pc {
            0x82BD7EF0 => {
    //   block [0x82BD7EF0..0x82BD7F50)
	// 82BD7EF0: 3965FFFC  addi r11, r5, -4
	ctx.r[11].s64 = ctx.r[5].s64 + -4;
	// 82BD7EF4: 394005C8  li r10, 0x5c8
	ctx.r[10].s64 = 1480;
	// 82BD7EF8: 7D2400D0  neg r9, r4
	ctx.r[9].s64 = -ctx.r[4].s64;
	// 82BD7EFC: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82BD7F00: 5089446E  rlwimi r9, r4, 8, 0x11, 0x17
	ctx.r[9].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000007F00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF80FF);
	// 82BD7F04: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7F08: 3D400007  lis r10, 7
	ctx.r[10].s64 = 458752;
	// 82BD7F0C: 55292376  rlwinm r9, r9, 4, 0xd, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0FFFFFFFu64;
	// 82BD7F10: 61478D00  ori r7, r10, 0x8d00
	ctx.r[7].u64 = ctx.r[10].u64 | 36096;
	// 82BD7F14: 55290566  rlwinm r9, r9, 0, 0x15, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD7F18: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82BD7F1C: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7F20: 61280001  ori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 | 1;
	// 82BD7F24: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7F28: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD7F2C: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7F30: 4082FFF8  bne 0x82bd7f28
	if !ctx.cr[0].eq {
	pc = 0x82BD7F28; continue 'dispatch;
	}
	// 82BD7F34: 39400D00  li r10, 0xd00
	ctx.r[10].s64 = 3328;
	// 82BD7F38: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7F3C: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD7F40: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82BD7F44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD7F48: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82BD7F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD7F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD7F50 size=304
    let mut pc: u32 = 0x82BD7F50;
    'dispatch: loop {
        match pc {
            0x82BD7F50 => {
    //   block [0x82BD7F50..0x82BD8080)
	// 82BD7F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD7F54: 485D0219  bl 0x831a816c
	ctx.lr = 0x82BD7F58;
	sub_831A8130(ctx, base);
	// 82BD7F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD7F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD7F60: 3C80B580  lis r4, -0x4a80
	ctx.r[4].s64 = -1249902592;
	// 82BD7F64: 38602000  li r3, 0x2000
	ctx.r[3].s64 = 8192;
	// 82BD7F68: 485EBFF1  bl 0x831c3f58
	ctx.lr = 0x82BD7F6C;
	sub_831C3F58(ctx, base);
	// 82BD7F6C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82BD7F70: 3BBE2000  addi r29, r30, 0x2000
	ctx.r[29].s64 = ctx.r[30].s64 + 8192;
	// 82BD7F74: 4082000C  bne 0x82bd7f80
	if !ctx.cr[0].eq {
	pc = 0x82BD7F80; continue 'dispatch;
	}
	// 82BD7F78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD7F7C: 480000FC  b 0x82bd8078
	pc = 0x82BD8078; continue 'dispatch;
	// 82BD7F80: 93DF35D4  stw r30, 0x35d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13780 as u32), ctx.r[30].u32 ) };
	// 82BD7F84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82BD7F88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82BD7F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD7F90: 4BFFFC71  bl 0x82bd7c00
	ctx.lr = 0x82BD7F94;
	sub_82BD7C00(ctx, base);
	// 82BD7F94: 57CB653E  srwi r11, r30, 0x14
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7F98: 57CA00FE  clrlwi r10, r30, 3
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x1FFFFFFFu64;
	// 82BD7F9C: 813F35D8  lwz r9, 0x35d8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13784 as u32) ) } as u64;
	// 82BD7FA0: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 82BD7FA4: 39030007  addi r8, r3, 7
	ctx.r[8].s64 = ctx.r[3].s64 + 7;
	// 82BD7FA8: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD7FAC: 5123000E  rlwimi r3, r9, 0, 0, 7
	ctx.r[3].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[3].u64 & 0xFFFFFFFF00FFFFFF);
	// 82BD7FB0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD7FB4: 550B1034  rlwinm r11, r8, 2, 0, 0x1a
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD7FB8: 907F35D8  stw r3, 0x35d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13784 as u32), ctx.r[3].u32 ) };
	// 82BD7FBC: 915F35DC  stw r10, 0x35dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13788 as u32), ctx.r[10].u32 ) };
	// 82BD7FC0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82BD7FC4: 7CABF214  add r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82BD7FC8: 3BDF3660  addi r30, r31, 0x3660
	ctx.r[30].s64 = ctx.r[31].s64 + 13920;
	// 82BD7FCC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82BD7FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD7FD4: 4BFFFF1D  bl 0x82bd7ef0
	ctx.lr = 0x82BD7FD8;
	sub_82BD7EF0(ctx, base);
	// 82BD7FD8: 54AB653E  srwi r11, r5, 0x14
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD7FDC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD7FE0: 54AA00FE  clrlwi r10, r5, 3
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x1FFFFFFFu64;
	// 82BD7FE4: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 82BD7FE8: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82BD7FEC: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD7FF0: 38E30007  addi r7, r3, 7
	ctx.r[7].s64 = ctx.r[3].s64 + 7;
	// 82BD7FF4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD7FF8: 5128000E  rlwimi r8, r9, 0, 0, 7
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[8].u64 & 0xFFFFFFFF00FFFFFF);
	// 82BD7FFC: 54EB1034  rlwinm r11, r7, 2, 0, 0x1a
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82BD8000: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82BD8004: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82BD8008: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82BD800C: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82BD8010: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82BD8014: 2B040070  cmplwi cr6, r4, 0x70
	ctx.cr[6].compare_u32(ctx.r[4].u32, 112 as u32, &mut ctx.xer);
	// 82BD8018: 4099FFB4  ble cr6, 0x82bd7fcc
	if !ctx.cr[6].gt {
	pc = 0x82BD7FCC; continue 'dispatch;
	}
	// 82BD801C: 3965FFFC  addi r11, r5, -4
	ctx.r[11].s64 = ctx.r[5].s64 + -4;
	// 82BD8020: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82BD8024: 3D20C000  lis r9, -0x4000
	ctx.r[9].s64 = -1073741824;
	// 82BD8028: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 82BD802C: 61293600  ori r9, r9, 0x3600
	ctx.r[9].u64 = ctx.r[9].u64 | 13824;
	// 82BD8030: 61080081  ori r8, r8, 0x81
	ctx.r[8].u64 = ctx.r[8].u64 | 129;
	// 82BD8034: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8038: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD803C: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8040: 4082FFE4  bne 0x82bd8024
	if !ctx.cr[0].eq {
	pc = 0x82BD8024; continue 'dispatch;
	}
	// 82BD8044: 54AA653E  srwi r10, r5, 0x14
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shr(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8048: 813F39E0  lwz r9, 0x39e0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14816 as u32) ) } as u64;
	// 82BD804C: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82BD8050: 394A0200  addi r10, r10, 0x200
	ctx.r[10].s64 = ctx.r[10].s64 + 512;
	// 82BD8054: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82BD8058: 554B04E6  rlwinm r11, r10, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD805C: 54AA00FE  clrlwi r10, r5, 3
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x1FFFFFFFu64;
	// 82BD8060: 7D081670  srawi r8, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82BD8064: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD8068: 5128000E  rlwimi r8, r9, 0, 0, 7
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[8].u64 & 0xFFFFFFFF00FFFFFF);
	// 82BD806C: 917F39E4  stw r11, 0x39e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14820 as u32), ctx.r[11].u32 ) };
	// 82BD8070: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD8074: 911F39E0  stw r8, 0x39e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14816 as u32), ctx.r[8].u32 ) };
	// 82BD8078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD807C: 485D0140  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8080 size=12
    let mut pc: u32 = 0x82BD8080;
    'dispatch: loop {
        match pc {
            0x82BD8080 => {
    //   block [0x82BD8080..0x82BD808C)
	// 82BD8080: 806335D4  lwz r3, 0x35d4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13780 as u32) ) } as u64;
	// 82BD8084: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 82BD8088: 485EBF68  b 0x831c3ff0
	sub_831C3FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD8090 size=224
    let mut pc: u32 = 0x82BD8090;
    'dispatch: loop {
        match pc {
            0x82BD8090 => {
    //   block [0x82BD8090..0x82BD8170)
	// 82BD8090: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82BD8094: 39440078  addi r10, r4, 0x78
	ctx.r[10].s64 = ctx.r[4].s64 + 120;
	// 82BD8098: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82BD809C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD80A0: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82BD80A4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD80A8: 7C005A2C  dcbt 0, r11
	// 82BD80AC: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82BD80B0: 7C085A2C  dcbt r8, r11
	// 82BD80B4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82BD80B8: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 82BD80BC: 40990078  ble cr6, 0x82bd8134
	if !ctx.cr[6].gt {
	pc = 0x82BD8134; continue 'dispatch;
	}
	// 82BD80C0: 3906FFFC  addi r8, r6, -4
	ctx.r[8].s64 = ctx.r[6].s64 + -4;
	// 82BD80C4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD80C8: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD80CC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82BD80D0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD80D4: 3BE00100  li r31, 0x100
	ctx.r[31].s64 = 256;
	// 82BD80D8: 7C1F5A2C  dcbt r31, r11
	// 82BD80DC: 13C45C47  vcmpneh. (lvrx128) v30, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	// load reversed into ctx.v[62] using VectorMaskR (or zero if (tmp.u32 & 0xF) == 0)
	// 82BD80E0: 3BE00040  li r31, 0x40
	ctx.r[31].s64 = 64;
	// 82BD80E4: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82BD80E8: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD8170 size=224
    let mut pc: u32 = 0x82BD8170;
    'dispatch: loop {
        match pc {
            0x82BD8170 => {
    //   block [0x82BD8170..0x82BD8250)
	// 82BD8170: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82BD8174: 39440178  addi r10, r4, 0x178
	ctx.r[10].s64 = ctx.r[4].s64 + 376;
	// 82BD8178: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82BD817C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8180: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82BD8184: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82BD8188: 7C005A2C  dcbt 0, r11
	// 82BD818C: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82BD8190: 7C085A2C  dcbt r8, r11
	// 82BD8194: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82BD8198: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 82BD819C: 40990078  ble cr6, 0x82bd8214
	if !ctx.cr[6].gt {
	pc = 0x82BD8214; continue 'dispatch;
	}
	// 82BD81A0: 3906FFFC  addi r8, r6, -4
	ctx.r[8].s64 = ctx.r[6].s64 + -4;
	// 82BD81A4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD81A8: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD81AC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82BD81B0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82BD81B4: 3BE00100  li r31, 0x100
	ctx.r[31].s64 = 256;
	// 82BD81B8: 7C1F5A2C  dcbt r31, r11
	// 82BD81BC: 13C45C47  vcmpneh. (lvrx128) v30, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	// load reversed into ctx.v[62] using VectorMaskR (or zero if (tmp.u32 & 0xF) == 0)
	// 82BD81C0: 3BE00040  li r31, 0x40
	ctx.r[31].s64 = 64;
	// 82BD81C4: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82BD81C8: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8250 size=92
    let mut pc: u32 = 0x82BD8250;
    'dispatch: loop {
        match pc {
            0x82BD8250 => {
    //   block [0x82BD8250..0x82BD82AC)
	// 82BD8250: 548BD97E  srwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD8254: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8258: 548906FE  clrlwi r9, r4, 0x1b
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82BD825C: 396B09E0  addi r11, r11, 0x9e0
	ctx.r[11].s64 = ctx.r[11].s64 + 2528;
	// 82BD8260: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD8264: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD8268: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82BD826C: 7D084830  slw r8, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8270: 7CEB182E  lwzx r7, r11, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD8274: 7D4A4830  slw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8278: 7CE94078  andc r9, r7, r8
	ctx.r[9].u64 = ctx.r[7].u64 & !ctx.r[8].u64;
	// 82BD827C: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82BD8280: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82BD8284: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82BD8288: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 82BD828C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82BD8290: 4082FFC0  bne 0x82bd8250
	if !ctx.cr[0].eq {
	pc = 0x82BD8250; continue 'dispatch;
	}
	// 82BD8294: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD8298: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD829C: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD82A0: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD82A4: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD82A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD82B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD82B0 size=92
    let mut pc: u32 = 0x82BD82B0;
    'dispatch: loop {
        match pc {
            0x82BD82B0 => {
    //   block [0x82BD82B0..0x82BD830C)
	// 82BD82B0: 548BD97E  srwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD82B4: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD82B8: 548906FE  clrlwi r9, r4, 0x1b
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82BD82BC: 396B09E4  addi r11, r11, 0x9e4
	ctx.r[11].s64 = ctx.r[11].s64 + 2532;
	// 82BD82C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD82C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD82C8: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82BD82CC: 7D084830  slw r8, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD82D0: 7CEB182E  lwzx r7, r11, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD82D4: 7D4A4830  slw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD82D8: 7CE94078  andc r9, r7, r8
	ctx.r[9].u64 = ctx.r[7].u64 & !ctx.r[8].u64;
	// 82BD82DC: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82BD82E0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82BD82E4: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82BD82E8: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 82BD82EC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82BD82F0: 4082FFC0  bne 0x82bd82b0
	if !ctx.cr[0].eq {
	pc = 0x82BD82B0; continue 'dispatch;
	}
	// 82BD82F4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD82F8: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD82FC: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD8300: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD8304: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD8308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8310 size=84
    let mut pc: u32 = 0x82BD8310;
    'dispatch: loop {
        match pc {
            0x82BD8310 => {
    //   block [0x82BD8310..0x82BD8364)
	// 82BD8310: 396409E8  addi r11, r4, 0x9e8
	ctx.r[11].s64 = ctx.r[4].s64 + 2536;
	// 82BD8314: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD8318: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD831C: 8945000B  lbz r10, 0xb(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(11 as u32) ) } as u64;
	// 82BD8320: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82BD8324: 89250007  lbz r9, 7(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(7 as u32) ) } as u64;
	// 82BD8328: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82BD832C: 89050003  lbz r8, 3(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(3 as u32) ) } as u64;
	// 82BD8330: 38A50010  addi r5, r5, 0x10
	ctx.r[5].s64 = ctx.r[5].s64 + 16;
	// 82BD8334: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82BD8338: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD833C: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82BD8340: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD8344: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD8348: 4082FFD4  bne 0x82bd831c
	if !ctx.cr[0].eq {
	pc = 0x82BD831C; continue 'dispatch;
	}
	// 82BD834C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD8350: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD8354: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD8358: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD835C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD8360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8368 size=84
    let mut pc: u32 = 0x82BD8368;
    'dispatch: loop {
        match pc {
            0x82BD8368 => {
    //   block [0x82BD8368..0x82BD83BC)
	// 82BD8368: 396409F8  addi r11, r4, 0x9f8
	ctx.r[11].s64 = ctx.r[4].s64 + 2552;
	// 82BD836C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD8370: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD8374: 8945000B  lbz r10, 0xb(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(11 as u32) ) } as u64;
	// 82BD8378: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82BD837C: 89250007  lbz r9, 7(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(7 as u32) ) } as u64;
	// 82BD8380: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82BD8384: 89050003  lbz r8, 3(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(3 as u32) ) } as u64;
	// 82BD8388: 38A50010  addi r5, r5, 0x10
	ctx.r[5].s64 = ctx.r[5].s64 + 16;
	// 82BD838C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82BD8390: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8394: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82BD8398: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD839C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD83A0: 4082FFD4  bne 0x82bd8374
	if !ctx.cr[0].eq {
	pc = 0x82BD8374; continue 'dispatch;
	}
	// 82BD83A4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD83A8: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82BD83AC: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD83B0: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82BD83B4: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD83B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD83C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82BD83C0 size=80
    let mut pc: u32 = 0x82BD83C0;
    'dispatch: loop {
        match pc {
            0x82BD83C0 => {
    //   block [0x82BD83C0..0x82BD8410)
	// 82BD83C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD83C4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD83C8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD83CC: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD83D0: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82BD83D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD83D8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD83DC: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD83E0: 90640000  stw r3, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82BD83E4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82BD83E8: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82BD83EC: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82BD83F0: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82BD83F4: 714A0340  andi. r10, r10, 0x340
	ctx.r[10].u64 = ctx.r[10].u64 & 832;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD83F8: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 82BD83FC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD8400: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD8404: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD8408: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82BD840C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8410 size=56
    let mut pc: u32 = 0x82BD8410;
    'dispatch: loop {
        match pc {
            0x82BD8410 => {
    //   block [0x82BD8410..0x82BD8448)
	// 82BD8410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8414: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82BD8418: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD841C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD8420: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82BD8424: 3D00FFFF  lis r8, -1
	ctx.r[8].s64 = -65536;
	// 82BD8428: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD842C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82BD8430: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD8434: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD8438: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD843C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82BD8440: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82BD8444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8448 size=444
    let mut pc: u32 = 0x82BD8448;
    'dispatch: loop {
        match pc {
            0x82BD8448 => {
    //   block [0x82BD8448..0x82BD8604)
	// 82BD8448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD844C: 485CFD1D  bl 0x831a8168
	ctx.lr = 0x82BD8450;
	sub_831A8130(ctx, base);
	// 82BD8450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8454: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82BD8458: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82BD845C: 83FE3194  lwz r31, 0x3194(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12692 as u32) ) } as u64;
	// 82BD8460: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82BD8464: 419A0068  beq cr6, 0x82bd84cc
	if ctx.cr[6].eq {
	pc = 0x82BD84CC; continue 'dispatch;
	}
	// 82BD8468: 817E2A9C  lwz r11, 0x2a9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10908 as u32) ) } as u64;
	// 82BD846C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD8470: 419A000C  beq cr6, 0x82bd847c
	if ctx.cr[6].eq {
	pc = 0x82BD847C; continue 'dispatch;
	}
	// 82BD8474: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD8478: 48000054  b 0x82bd84cc
	pc = 0x82BD84CC; continue 'dispatch;
	// 82BD847C: 817E2AA0  lwz r11, 0x2aa0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10912 as u32) ) } as u64;
	// 82BD8480: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8484: 7D6B5039  and. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD8488: 41820044  beq 0x82bd84cc
	if ctx.cr[0].eq {
	pc = 0x82BD84CC; continue 'dispatch;
	}
	// 82BD848C: 817E34D8  lwz r11, 0x34d8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13528 as u32) ) } as u64;
	// 82BD8490: 807E34D4  lwz r3, 0x34d4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13524 as u32) ) } as u64;
	// 82BD8494: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD8498: 4198000C  blt cr6, 0x82bd84a4
	if ctx.cr[6].lt {
	pc = 0x82BD84A4; continue 'dispatch;
	}
	// 82BD849C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD84A0: 48017661  bl 0x82befb00
	ctx.lr = 0x82BD84A4;
	sub_82BEFB00(ctx, base);
	// 82BD84A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD84A8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82BD84AC: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82BD84B0: 53EBF0BE  rlwimi r11, r31, 0x1e, 2, 0x1f
	ctx.r[11].u64 = (((ctx.r[31].u32).rotate_left(30) as u64) & 0x000000003FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFFC0000000);
	// 82BD84B4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82BD84B8: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD84BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82BD84C0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD84C4: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD84C8: 913E34D4  stw r9, 0x34d4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13524 as u32), ctx.r[9].u32 ) };
	// 82BD84CC: 93BE3194  stw r29, 0x3194(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12692 as u32), ctx.r[29].u32 ) };
	// 82BD84D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD84D4: E97E0010  ld r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	// 82BD84D8: 656B0010  oris r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 1048576;
	// 82BD84DC: F97E0010  std r11, 0x10(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD84E0: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82BD84E4: F97E0010  std r11, 0x10(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD84E8: 419A0114  beq cr6, 0x82bd85fc
	if ctx.cr[6].eq {
	pc = 0x82BD85FC; continue 'dispatch;
	}
	// 82BD84EC: 815D003C  lwz r10, 0x3c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82BD84F0: 397D0028  addi r11, r29, 0x28
	ctx.r[11].s64 = ctx.r[29].s64 + 40;
	// 82BD84F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD84F8: 419A0104  beq cr6, 0x82bd85fc
	if ctx.cr[6].eq {
	pc = 0x82BD85FC; continue 'dispatch;
	}
	// 82BD84FC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD8500: E95E0008  ld r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82BD8504: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82BD8508: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82BD850C: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82BD8510: E94B0008  ld r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82BD8514: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 82BD8518: 419A0018  beq cr6, 0x82bd8530
	if ctx.cr[6].eq {
	pc = 0x82BD8530; continue 'dispatch;
	}
	// 82BD851C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD8520: E95E0020  ld r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	// 82BD8524: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD8528: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD852C: F95E0020  std r10, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82BD8530: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD8534: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 82BD8538: 3B9E0480  addi r28, r30, 0x480
	ctx.r[28].s64 = ctx.r[30].s64 + 1152;
	// 82BD853C: 7FAAFA14  add r29, r10, r31
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82BD8540: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8544: 409800B8  bge cr6, 0x82bd85fc
	if !ctx.cr[6].lt {
	pc = 0x82BD85FC; continue 'dispatch;
	}
	// 82BD8548: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD854C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD8550: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD8554: 41820010  beq 0x82bd8564
	if ctx.cr[0].eq {
	pc = 0x82BD8564; continue 'dispatch;
	}
	// 82BD8558: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD855C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8560: 4198FFE8  blt cr6, 0x82bd8548
	if ctx.cr[6].lt {
	pc = 0x82BD8548; continue 'dispatch;
	}
	// 82BD8564: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8568: 40980094  bge cr6, 0x82bd85fc
	if !ctx.cr[6].lt {
	pc = 0x82BD85FC; continue 'dispatch;
	}
	// 82BD856C: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD8570: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8574: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD8578: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD857C: 41820078  beq 0x82bd85f4
	if ctx.cr[0].eq {
	pc = 0x82BD85F4; continue 'dispatch;
	}
	// 82BD8580: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82BD8584: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82BD8588: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82BD858C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82BD8590: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82BD8594: 485CFF7D  bl 0x831a8510
	ctx.lr = 0x82BD8598;
	sub_831A8510(ctx, base);
	// 82BD8598: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82BD859C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD85A0: 4198FFCC  blt cr6, 0x82bd856c
	if ctx.cr[6].lt {
	pc = 0x82BD856C; continue 'dispatch;
	}
	// 82BD85A4: 48000050  b 0x82bd85f4
	pc = 0x82BD85F4; continue 'dispatch;
	// 82BD85A8: A15F0002  lhz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD85AC: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD85B0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD85B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD85B8: 41820044  beq 0x82bd85fc
	if ctx.cr[0].eq {
	pc = 0x82BD85FC; continue 'dispatch;
	}
	// 82BD85BC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD85C0: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82BD85C4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD85C8: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 82BD85CC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD85D0: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD85D4: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82BD85D8: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 82BD85DC: 554A043F  clrlwi. r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD85E0: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD85E4: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82BD85E8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD85EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD85F0: 4082FFD4  bne 0x82bd85c4
	if !ctx.cr[0].eq {
	pc = 0x82BD85C4; continue 'dispatch;
	}
	// 82BD85F4: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD85F8: 4198FFB0  blt cr6, 0x82bd85a8
	if ctx.cr[6].lt {
	pc = 0x82BD85A8; continue 'dispatch;
	}
	// 82BD85FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82BD8600: 485CFBB8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8608 size=248
    let mut pc: u32 = 0x82BD8608;
    'dispatch: loop {
        match pc {
            0x82BD8608 => {
    //   block [0x82BD8608..0x82BD8700)
	// 82BD8608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD860C: 485CFB4D  bl 0x831a8158
	ctx.lr = 0x82BD8610;
	sub_831A8130(ctx, base);
	// 82BD8610: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8614: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82BD8618: 38A00368  li r5, 0x368
	ctx.r[5].s64 = 872;
	// 82BD861C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8620: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD8624: 485CFBBD  bl 0x831a81e0
	ctx.lr = 0x82BD8628;
	sub_831A81E0(ctx, base);
	// 82BD8628: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82BD862C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD8630: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82BD8634: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD8638: 3D20FFFF  lis r9, -1
	ctx.r[9].s64 = -65536;
	// 82BD863C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82BD8640: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82BD8644: 817F0368  lwz r11, 0x368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(872 as u32) ) } as u64;
	// 82BD8648: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD864C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82BD8650: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD8654: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82BD8658: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD865C: 4182009C  beq 0x82bd86f8
	if ctx.cr[0].eq {
	pc = 0x82BD86F8; continue 'dispatch;
	}
	// 82BD8660: 3B3F0028  addi r25, r31, 0x28
	ctx.r[25].s64 = ctx.r[31].s64 + 40;
	// 82BD8664: 3B7F0380  addi r27, r31, 0x380
	ctx.r[27].s64 = ctx.r[31].s64 + 896;
	// 82BD8668: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 82BD866C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8670: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82BD8674: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82BD8678: 394B0368  addi r10, r11, 0x368
	ctx.r[10].s64 = ctx.r[11].s64 + 872;
	// 82BD867C: 80EB0380  lwz r7, 0x380(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(896 as u32) ) } as u64;
	// 82BD8680: 810B0368  lwz r8, 0x368(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(872 as u32) ) } as u64;
	// 82BD8684: 38E70009  addi r7, r7, 9
	ctx.r[7].s64 = ctx.r[7].s64 + 9;
	// 82BD8688: 816B0384  lwz r11, 0x384(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(900 as u32) ) } as u64;
	// 82BD868C: 7F494214  add r26, r9, r8
	ctx.r[26].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82BD8690: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD8694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD8698: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82BD869C: 419A0044  beq cr6, 0x82bd86e0
	if ctx.cr[6].eq {
	pc = 0x82BD86E0; continue 'dispatch;
	}
	// 82BD86A0: 3BD9001C  addi r30, r25, 0x1c
	ctx.r[30].s64 = ctx.r[25].s64 + 28;
	// 82BD86A4: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82BD86A8: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82BD86AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD86B0: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD86B4: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82BD86B8: 7C8BD214  add r4, r11, r26
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82BD86BC: 7F1E2040  cmplw cr6, r30, r4
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82BD86C0: 419A0010  beq cr6, 0x82bd86d0
	if ctx.cr[6].eq {
	pc = 0x82BD86D0; continue 'dispatch;
	}
	// 82BD86C4: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82BD86C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD86CC: 485CFE45  bl 0x831a8510
	ctx.lr = 0x82BD86D0;
	sub_831A8510(ctx, base);
	// 82BD86D0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82BD86D4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82BD86D8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82BD86DC: 4082FFD0  bne 0x82bd86ac
	if !ctx.cr[0].eq {
	pc = 0x82BD86AC; continue 'dispatch;
	}
	// 82BD86E0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82BD86E4: 3718FFFF  addic. r24, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82BD86E8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD86EC: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82BD86F0: 3B3901A0  addi r25, r25, 0x1a0
	ctx.r[25].s64 = ctx.r[25].s64 + 416;
	// 82BD86F4: 4082FF78  bne 0x82bd866c
	if !ctx.cr[0].eq {
	pc = 0x82BD866C; continue 'dispatch;
	}
	// 82BD86F8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82BD86FC: 485CFAAC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8700 size=460
    let mut pc: u32 = 0x82BD8700;
    'dispatch: loop {
        match pc {
            0x82BD8700 => {
    //   block [0x82BD8700..0x82BD88CC)
	// 82BD8700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD8704: 485CFA65  bl 0x831a8168
	ctx.lr = 0x82BD8708;
	sub_831A8130(ctx, base);
	// 82BD8708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD870C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82BD8710: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82BD8714: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD8718: 419A0010  beq cr6, 0x82bd8728
	if ctx.cr[6].eq {
	pc = 0x82BD8728; continue 'dispatch;
	}
	// 82BD871C: E97E0010  ld r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	// 82BD8720: 656B0008  oris r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 524288;
	// 82BD8724: F97E0010  std r11, 0x10(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD8728: 83FE3198  lwz r31, 0x3198(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12696 as u32) ) } as u64;
	// 82BD872C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82BD8730: 419A0068  beq cr6, 0x82bd8798
	if ctx.cr[6].eq {
	pc = 0x82BD8798; continue 'dispatch;
	}
	// 82BD8734: 817E2A9C  lwz r11, 0x2a9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10908 as u32) ) } as u64;
	// 82BD8738: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD873C: 419A000C  beq cr6, 0x82bd8748
	if ctx.cr[6].eq {
	pc = 0x82BD8748; continue 'dispatch;
	}
	// 82BD8740: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD8744: 48000054  b 0x82bd8798
	pc = 0x82BD8798; continue 'dispatch;
	// 82BD8748: 817E2AA0  lwz r11, 0x2aa0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10912 as u32) ) } as u64;
	// 82BD874C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8750: 7D6B5039  and. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD8754: 41820044  beq 0x82bd8798
	if ctx.cr[0].eq {
	pc = 0x82BD8798; continue 'dispatch;
	}
	// 82BD8758: 817E34D8  lwz r11, 0x34d8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13528 as u32) ) } as u64;
	// 82BD875C: 807E34D4  lwz r3, 0x34d4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13524 as u32) ) } as u64;
	// 82BD8760: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD8764: 4198000C  blt cr6, 0x82bd8770
	if ctx.cr[6].lt {
	pc = 0x82BD8770; continue 'dispatch;
	}
	// 82BD8768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD876C: 48017395  bl 0x82befb00
	ctx.lr = 0x82BD8770;
	sub_82BEFB00(ctx, base);
	// 82BD8770: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD8774: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82BD8778: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82BD877C: 53EBF0BE  rlwimi r11, r31, 0x1e, 2, 0x1f
	ctx.r[11].u64 = (((ctx.r[31].u32).rotate_left(30) as u64) & 0x000000003FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFFC0000000);
	// 82BD8780: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82BD8784: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD8788: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82BD878C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD8790: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD8794: 913E34D4  stw r9, 0x34d4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13524 as u32), ctx.r[9].u32 ) };
	// 82BD8798: 897E2ABE  lbz r11, 0x2abe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10942 as u32) ) } as u64;
	// 82BD879C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD87A0: 93BE3198  stw r29, 0x3198(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12696 as u32), ctx.r[29].u32 ) };
	// 82BD87A4: 556B067E  clrlwi r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82BD87A8: 997E2ABE  stb r11, 0x2abe(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(10942 as u32), ctx.r[11].u8 ) };
	// 82BD87AC: 419A0118  beq cr6, 0x82bd88c4
	if ctx.cr[6].eq {
	pc = 0x82BD88C4; continue 'dispatch;
	}
	// 82BD87B0: 357D0368  addic. r11, r29, 0x368
	ctx.xer.ca = (ctx.r[29].u32 > (!(872 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + 872;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD87B4: 41820110  beq 0x82bd88c4
	if ctx.cr[0].eq {
	pc = 0x82BD88C4; continue 'dispatch;
	}
	// 82BD87B8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82BD87BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD87C0: 419A0104  beq cr6, 0x82bd88c4
	if ctx.cr[6].eq {
	pc = 0x82BD88C4; continue 'dispatch;
	}
	// 82BD87C4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD87C8: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82BD87CC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82BD87D0: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82BD87D4: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82BD87D8: E94B0008  ld r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82BD87DC: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 82BD87E0: 419A0018  beq cr6, 0x82bd87f8
	if ctx.cr[6].eq {
	pc = 0x82BD87F8; continue 'dispatch;
	}
	// 82BD87E4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82BD87E8: E95E0020  ld r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	// 82BD87EC: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82BD87F0: 7D4A6378  or r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82BD87F4: F95E0020  std r10, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82BD87F8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82BD87FC: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 82BD8800: 3B9E0480  addi r28, r30, 0x480
	ctx.r[28].s64 = ctx.r[30].s64 + 1152;
	// 82BD8804: 7FAAFA14  add r29, r10, r31
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82BD8808: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD880C: 409800B8  bge cr6, 0x82bd88c4
	if !ctx.cr[6].lt {
	pc = 0x82BD88C4; continue 'dispatch;
	}
	// 82BD8810: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD8814: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD8818: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD881C: 41820010  beq 0x82bd882c
	if ctx.cr[0].eq {
	pc = 0x82BD882C; continue 'dispatch;
	}
	// 82BD8820: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD8824: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8828: 4198FFE8  blt cr6, 0x82bd8810
	if ctx.cr[6].lt {
	pc = 0x82BD8810; continue 'dispatch;
	}
	// 82BD882C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8830: 40980094  bge cr6, 0x82bd88c4
	if !ctx.cr[6].lt {
	pc = 0x82BD88C4; continue 'dispatch;
	}
	// 82BD8834: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD8838: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD883C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD8840: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD8844: 41820078  beq 0x82bd88bc
	if ctx.cr[0].eq {
	pc = 0x82BD88BC; continue 'dispatch;
	}
	// 82BD8848: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82BD884C: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82BD8850: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82BD8854: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82BD8858: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82BD885C: 485CFCB5  bl 0x831a8510
	ctx.lr = 0x82BD8860;
	sub_831A8510(ctx, base);
	// 82BD8860: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82BD8864: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8868: 4198FFCC  blt cr6, 0x82bd8834
	if ctx.cr[6].lt {
	pc = 0x82BD8834; continue 'dispatch;
	}
	// 82BD886C: 48000050  b 0x82bd88bc
	pc = 0x82BD88BC; continue 'dispatch;
	// 82BD8870: A15F0002  lhz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD8874: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8878: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82BD887C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD8880: 41820044  beq 0x82bd88c4
	if ctx.cr[0].eq {
	pc = 0x82BD88C4; continue 'dispatch;
	}
	// 82BD8884: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD8888: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82BD888C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8890: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 82BD8894: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8898: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD889C: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82BD88A0: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 82BD88A4: 554A043F  clrlwi. r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD88A8: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD88AC: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82BD88B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD88B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82BD88B8: 4082FFD4  bne 0x82bd888c
	if !ctx.cr[0].eq {
	pc = 0x82BD888C; continue 'dispatch;
	}
	// 82BD88BC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD88C0: 4198FFB0  blt cr6, 0x82bd8870
	if ctx.cr[6].lt {
	pc = 0x82BD8870; continue 'dispatch;
	}
	// 82BD88C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82BD88C8: 485CF8F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD88D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD88D0 size=20
    let mut pc: u32 = 0x82BD88D0;
    'dispatch: loop {
        match pc {
            0x82BD88D0 => {
    //   block [0x82BD88D0..0x82BD88E4)
	// 82BD88D0: 90832E2C  stw r4, 0x2e2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11820 as u32), ctx.r[4].u32 ) };
	// 82BD88D4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82BD88D8: 656B0008  oris r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 524288;
	// 82BD88DC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82BD88E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD88E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD88E8 size=60
    let mut pc: u32 = 0x82BD88E8;
    'dispatch: loop {
        match pc {
            0x82BD88E8 => {
    //   block [0x82BD88E8..0x82BD8924)
	// 82BD88E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD88EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD88F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD88F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD88F8: 83E32E2C  lwz r31, 0x2e2c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11820 as u32) ) } as u64;
	// 82BD88FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82BD8900: 419A000C  beq cr6, 0x82bd890c
	if ctx.cr[6].eq {
	pc = 0x82BD890C; continue 'dispatch;
	}
	// 82BD8904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD8908: 48006659  bl 0x82bdef60
	ctx.lr = 0x82BD890C;
	sub_82BDEF60(ctx, base);
	// 82BD890C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD8910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD8914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD8918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD891C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD8920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8928 size=236
    let mut pc: u32 = 0x82BD8928;
    'dispatch: loop {
        match pc {
            0x82BD8928 => {
    //   block [0x82BD8928..0x82BD8A14)
	// 82BD8928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD892C: 485CF839  bl 0x831a8164
	ctx.lr = 0x82BD8930;
	sub_831A8130(ctx, base);
	// 82BD8930: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8934: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82BD8938: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82BD893C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82BD8940: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82BD8944: FB610050  std r27, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u64 ) };
	// 82BD8948: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82BD894C: FB610058  std r27, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u64 ) };
	// 82BD8950: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8954: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82BD8958: 48000028  b 0x82bd8980
	pc = 0x82BD8980; continue 'dispatch;
	// 82BD895C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD8960: 41990008  bgt cr6, 0x82bd8968
	if ctx.cr[6].gt {
	pc = 0x82BD8968; continue 'dispatch;
	}
	// 82BD8964: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82BD8968: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82BD896C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD8970: 390000FF  li r8, 0xff
	ctx.r[8].s64 = 255;
	// 82BD8974: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82BD8978: 7D0B49AE  stbx r8, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82BD897C: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8980: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82BD8984: 409AFFD8  bne cr6, 0x82bd895c
	if !ctx.cr[6].eq {
	pc = 0x82BD895C; continue 'dispatch;
	}
	// 82BD8988: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 * 12;
	// 82BD898C: 38AB0038  addi r5, r11, 0x38
	ctx.r[5].s64 = ctx.r[11].s64 + 56;
	// 82BD8990: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD8998: 485CF849  bl 0x831a81e0
	ctx.lr = 0x82BD899C;
	sub_831A81E0(ctx, base);
	// 82BD899C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82BD89A0: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82BD89A4: 3D200010  lis r9, 0x10
	ctx.r[9].s64 = 1048576;
	// 82BD89A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD89AC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82BD89B0: 61290005  ori r9, r9, 5
	ctx.r[9].u64 = ctx.r[9].u64 | 5;
	// 82BD89B4: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82BD89B8: 3CE0FFFF  lis r7, -1
	ctx.r[7].s64 = -65536;
	// 82BD89BC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82BD89C0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD89C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD89C8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82BD89CC: 937F0030  stw r27, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[27].u32 ) };
	// 82BD89D0: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82BD89D4: F95F0028  std r10, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u64 ) };
	// 82BD89D8: 419A0034  beq cr6, 0x82bd8a0c
	if ctx.cr[6].eq {
	pc = 0x82BD8A0C; continue 'dispatch;
	}
	// 82BD89DC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82BD89E0: 395F0034  addi r10, r31, 0x34
	ctx.r[10].s64 = ctx.r[31].s64 + 52;
	// 82BD89E4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD89E8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82BD89EC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD89F0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD89F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82BD89F8: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD89FC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82BD8A00: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82BD8A04: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD8A08: 4082FFDC  bne 0x82bd89e4
	if !ctx.cr[0].eq {
	pc = 0x82BD89E4; continue 'dispatch;
	}
	// 82BD8A0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82BD8A10: 485CF7A4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8A18 size=132
    let mut pc: u32 = 0x82BD8A18;
    'dispatch: loop {
        match pc {
            0x82BD8A18 => {
    //   block [0x82BD8A18..0x82BD8A9C)
	// 82BD8A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD8A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD8A20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82BD8A24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD8A28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8A2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82BD8A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8A34: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82BD8A38: A13E0000  lhz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8A3C: 48000010  b 0x82bd8a4c
	pc = 0x82BD8A4C; continue 'dispatch;
	// 82BD8A40: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82BD8A44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82BD8A48: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8A4C: 2B0900FF  cmplwi cr6, r9, 0xff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 255 as u32, &mut ctx.xer);
	// 82BD8A50: 409AFFF0  bne cr6, 0x82bd8a40
	if !ctx.cr[6].eq {
	pc = 0x82BD8A40; continue 'dispatch;
	}
	// 82BD8A54: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82BD8A58: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 82BD8A5C: 386B0038  addi r3, r11, 0x38
	ctx.r[3].s64 = ctx.r[11].s64 + 56;
	// 82BD8A60: 485EB4F9  bl 0x831c3f58
	ctx.lr = 0x82BD8A64;
	sub_831C3F58(ctx, base);
	// 82BD8A64: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82BD8A68: 4082000C  bne 0x82bd8a74
	if !ctx.cr[0].eq {
	pc = 0x82BD8A74; continue 'dispatch;
	}
	// 82BD8A6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD8A70: 48000014  b 0x82bd8a84
	pc = 0x82BD8A84; continue 'dispatch;
	// 82BD8A74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82BD8A78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD8A7C: 4BFFFEAD  bl 0x82bd8928
	ctx.lr = 0x82BD8A80;
	sub_82BD8928(ctx, base);
	// 82BD8A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD8A84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD8A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD8A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD8A90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82BD8A94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD8A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8AA0 size=128
    let mut pc: u32 = 0x82BD8AA0;
    'dispatch: loop {
        match pc {
            0x82BD8AA0 => {
    //   block [0x82BD8AA0..0x82BD8B20)
	// 82BD8AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD8AA4: 485CF6C9  bl 0x831a816c
	ctx.lr = 0x82BD8AA8;
	sub_831A8130(ctx, base);
	// 82BD8AA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8AAC: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82BD8AB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82BD8AB4: 83A50000  lwz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8AB8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82BD8ABC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82BD8AC0: 41980008  blt cr6, 0x82bd8ac8
	if ctx.cr[6].lt {
	pc = 0x82BD8AC8; continue 'dispatch;
	}
	// 82BD8AC4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82BD8AC8: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82BD8ACC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD8AD0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD8AD4: 419A0044  beq cr6, 0x82bd8b18
	if ctx.cr[6].eq {
	pc = 0x82BD8B18; continue 'dispatch;
	}
	// 82BD8AD8: 38830034  addi r4, r3, 0x34
	ctx.r[4].s64 = ctx.r[3].s64 + 52;
	// 82BD8ADC: 1CAB000C  mulli r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 * 12;
	// 82BD8AE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82BD8AE4: 485CFA2D  bl 0x831a8510
	ctx.lr = 0x82BD8AE8;
	sub_831A8510(ctx, base);
	// 82BD8AE8: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82BD8AEC: 4099002C  ble cr6, 0x82bd8b18
	if !ctx.cr[6].gt {
	pc = 0x82BD8B18; continue 'dispatch;
	}
	// 82BD8AF0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82BD8AF4: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 * 12;
	// 82BD8AF8: 812AC330  lwz r9, -0x3cd0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15568 as u32) ) } as u64;
	// 82BD8AFC: 394AC330  addi r10, r10, -0x3cd0
	ctx.r[10].s64 = ctx.r[10].s64 + -15568;
	// 82BD8B00: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82BD8B04: 7D2BF12E  stwx r9, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u32) };
	// 82BD8B08: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD8B0C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD8B10: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD8B14: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82BD8B18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD8B1C: 485CF6A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8B20 size=20
    let mut pc: u32 = 0x82BD8B20;
    'dispatch: loop {
        match pc {
            0x82BD8B20 => {
    //   block [0x82BD8B20..0x82BD8B34)
	// 82BD8B20: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82BD8B24: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82BD8B28: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82BD8B2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82BD8B30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8B34 size=112
    let mut pc: u32 = 0x82BD8B34;
    'dispatch: loop {
        match pc {
            0x82BD8B34 => {
    //   block [0x82BD8B34..0x82BD8BA4)
	// 82BD8B34: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8B38: 54663830  slwi r6, r3, 7
	ctx.r[6].u32 = ctx.r[3].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82BD8B3C: A0EB0002  lhz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82BD8B40: 54683E7E  srwi r8, r3, 0x19
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shr(25);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD8B44: 5529803E  rotlwi r9, r9, 0x10
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(16)) as u64;
	// 82BD8B48: 88AB0008  lbz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD8B4C: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 82BD8B50: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD8B54: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD8B58: 88EB0009  lbz r7, 9(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 82BD8B5C: 54A5403E  rotlwi r5, r5, 8
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 82BD8B60: 888B000A  lbz r4, 0xa(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82BD8B64: 7D294278  xor r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 ^ ctx.r[8].u64;
	// 82BD8B68: 7CA83B78  or r8, r5, r7
	ctx.r[8].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 82BD8B6C: 55273E7E  srwi r7, r9, 0x19
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shr(25);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD8B70: 55293830  slwi r9, r9, 7
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD8B74: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD8B78: 7CE94B78  or r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 82BD8B7C: 7D082378  or r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 82BD8B80: 7D293278  xor r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 ^ ctx.r[6].u64;
	// 82BD8B84: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD8B88: 55273E7E  srwi r7, r9, 0x19
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shr(25);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD8B8C: 55293830  slwi r9, r9, 7
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD8B90: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82BD8B94: 7CE94B78  or r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 82BD8B98: 7D034A78  xor r3, r8, r9
	ctx.r[3].u64 = ctx.r[8].u64 ^ ctx.r[9].u64;
	// 82BD8B9C: 4082FF98  bne 0x82bd8b34
	if !ctx.cr[0].eq {
	pc = 0x82BD8B34; continue 'dispatch;
	}
	// 82BD8BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8BA8 size=232
    let mut pc: u32 = 0x82BD8BA8;
    'dispatch: loop {
        match pc {
            0x82BD8BA8 => {
    //   block [0x82BD8BA8..0x82BD8C90)
	// 82BD8BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD8BAC: 485CF5C1  bl 0x831a816c
	ctx.lr = 0x82BD8BB0;
	sub_831A8130(ctx, base);
	// 82BD8BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8BB4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82BD8BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD8BBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82BD8BC0: 409A0014  bne cr6, 0x82bd8bd4
	if !ctx.cr[6].eq {
	pc = 0x82BD8BD4; continue 'dispatch;
	}
	// 82BD8BC4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82BD8BC8: 409A000C  bne cr6, 0x82bd8bd4
	if !ctx.cr[6].eq {
	pc = 0x82BD8BD4; continue 'dispatch;
	}
	// 82BD8BCC: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	// 82BD8BD0: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82BD8BD4: 548B07FF  clrlwi. r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD8BD8: 4182000C  beq 0x82bd8be4
	if ctx.cr[0].eq {
	pc = 0x82BD8BE4; continue 'dispatch;
	}
	// 82BD8BDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8BE0: 48000014  b 0x82bd8bf4
	pc = 0x82BD8BF4; continue 'dispatch;
	// 82BD8BE4: 817F2AA8  lwz r11, 0x2aa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10920 as u32) ) } as u64;
	// 82BD8BE8: 53C6446E  rlwimi r6, r30, 8, 0x11, 0x17
	ctx.r[6].u64 = (((ctx.r[30].u32).rotate_left(8) as u64) & 0x0000000000007F00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF80FF);
	// 82BD8BEC: 50CB2576  rlwimi r11, r6, 4, 0x15, 0x1b
	ctx.r[11].u64 = (((ctx.r[6].u32).rotate_left(4) as u64) & 0x00000000000007F0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF80F);
	// 82BD8BF0: 50CB2366  rlwimi r11, r6, 4, 0xd, 0x13
	ctx.r[11].u64 = (((ctx.r[6].u32).rotate_left(4) as u64) & 0x000000000007F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFF80FFF);
	// 82BD8BF4: 917F2AA8  stw r11, 0x2aa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10920 as u32), ctx.r[11].u32 ) };
	// 82BD8BF8: 815F31AC  lwz r10, 0x31ac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12716 as u32) ) } as u64;
	// 82BD8BFC: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82BD8C00: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82BD8C04: 7D5D2038  and r29, r10, r4
	ctx.r[29].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 82BD8C08: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD8C0C: 40990010  ble cr6, 0x82bd8c1c
	if !ctx.cr[6].gt {
	pc = 0x82BD8C1C; continue 'dispatch;
	}
	// 82BD8C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD8C14: 4800B93D  bl 0x82be4550
	ctx.lr = 0x82BD8C18;
	sub_82BE4550(ctx, base);
	// 82BD8C18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82BD8C1C: 67A9C001  oris r9, r29, 0xc001
	ctx.r[9].u64 = ctx.r[29].u64 | 3221291008;
	// 82BD8C20: 57CA1838  slwi r10, r30, 3
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8C24: 61293F00  ori r9, r9, 0x3f00
	ctx.r[9].u64 = ctx.r[9].u64 | 16128;
	// 82BD8C28: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82BD8C2C: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C30: 391E06BC  addi r8, r30, 0x6bc
	ctx.r[8].s64 = ctx.r[30].s64 + 1724;
	// 82BD8C34: 80FF35DC  lwz r7, 0x35dc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13788 as u32) ) } as u64;
	// 82BD8C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82BD8C3C: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD8C40: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C44: 80FF35D8  lwz r7, 0x35d8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13784 as u32) ) } as u64;
	// 82BD8C48: 54E7023E  clrlwi r7, r7, 8
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82BD8C4C: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C50: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C54: 814A35E4  lwz r10, 0x35e4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13796 as u32) ) } as u64;
	// 82BD8C58: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C5C: 7D48F82E  lwzx r10, r8, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82BD8C60: 554A023E  clrlwi r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82BD8C64: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C68: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C6C: 815F39E4  lwz r10, 0x39e4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14820 as u32) ) } as u64;
	// 82BD8C70: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C74: 815F39E0  lwz r10, 0x39e0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14816 as u32) ) } as u64;
	// 82BD8C78: 554A023E  clrlwi r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82BD8C7C: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82BD8C80: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82BD8C84: 4BFFF155  bl 0x82bd7dd8
	ctx.lr = 0x82BD8C88;
	sub_82BD7DD8(ctx, base);
	// 82BD8C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD8C8C: 485CF530  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8C90 size=36
    let mut pc: u32 = 0x82BD8C90;
    'dispatch: loop {
        match pc {
            0x82BD8C90 => {
    //   block [0x82BD8C90..0x82BD8CB4)
	// 82BD8C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8C94: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD8C98: 81632AA8  lwz r11, 0x2aa8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10920 as u32) ) } as u64;
	// 82BD8C9C: 556BA67E  rlwinm r11, r11, 0x14, 0x19, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD8CA0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD8CA4: 81632AA8  lwz r11, 0x2aa8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10920 as u32) ) } as u64;
	// 82BD8CA8: 556BE67E  rlwinm r11, r11, 0x1c, 0x19, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82BD8CAC: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD8CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8CB8 size=4
    let mut pc: u32 = 0x82BD8CB8;
    'dispatch: loop {
        match pc {
            0x82BD8CB8 => {
    //   block [0x82BD8CB8..0x82BD8CBC)
	// 82BD8CB8: 485EB638  b 0x831c42f0
	sub_831C42F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8CC0 size=16
    let mut pc: u32 = 0x82BD8CC0;
    'dispatch: loop {
        match pc {
            0x82BD8CC0 => {
    //   block [0x82BD8CC0..0x82BD8CD0)
	// 82BD8CC0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82BD8CC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82BD8CC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82BD8CCC: 485EB624  b 0x831c42f0
	sub_831C42F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8CD0 size=240
    let mut pc: u32 = 0x82BD8CD0;
    'dispatch: loop {
        match pc {
            0x82BD8CD0 => {
    //   block [0x82BD8CD0..0x82BD8DC0)
	// 82BD8CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD8CD4: 485CF495  bl 0x831a8168
	ctx.lr = 0x82BD8CD8;
	sub_831A8130(ctx, base);
	// 82BD8CD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD8CE0: 549D063F  clrlwi. r29, r4, 0x18
	ctx.r[29].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82BD8CE4: 4082001C  bne 0x82bd8d00
	if !ctx.cr[0].eq {
	pc = 0x82BD8D00; continue 'dispatch;
	}
	// 82BD8CE8: 3D6082BD  lis r11, -0x7d43
	ctx.r[11].s64 = -2101542912;
	// 82BD8CEC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8CF0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82BD8CF4: 38CB6B40  addi r6, r11, 0x6b40
	ctx.r[6].s64 = ctx.r[11].s64 + 27456;
	// 82BD8CF8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82BD8CFC: 485D4715  bl 0x831ad410
	ctx.lr = 0x82BD8D00;
	sub_831AD410(ctx, base);
	// 82BD8D00: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82BD8D04: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8D08: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82BD8D0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD8D10: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82BD8D14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8D18: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82BD8D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD8D20: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82BD8D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82BD8D28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82BD8D2C: 40990078  ble cr6, 0x82bd8da4
	if !ctx.cr[6].gt {
	pc = 0x82BD8DA4; continue 'dispatch;
	}
	// 82BD8D30: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 82BD8D34: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8D38: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82BD8D3C: 552B673E  rlwinm r11, r9, 0xc, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000FFFFFu64;
	// 82BD8D40: 7D4B40AE  lbzx r10, r11, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82BD8D44: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 82BD8D48: 409A0040  bne cr6, 0x82bd8d88
	if !ctx.cr[6].eq {
	pc = 0x82BD8D88; continue 'dispatch;
	}
	// 82BD8D4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD8D50: 419A000C  beq cr6, 0x82bd8d5c
	if ctx.cr[6].eq {
	pc = 0x82BD8D5C; continue 'dispatch;
	}
	// 82BD8D54: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82BD8D58: 4800000C  b 0x82bd8d64
	pc = 0x82BD8D64; continue 'dispatch;
	// 82BD8D5C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82BD8D60: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82BD8D64: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82BD8D68: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82BD8D6C: 4098000C  bge cr6, 0x82bd8d78
	if !ctx.cr[6].lt {
	pc = 0x82BD8D78; continue 'dispatch;
	}
	// 82BD8D70: 7C6B5830  slw r11, r3, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[3].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8D74: 7D642378  or r4, r11, r4
	ctx.r[4].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82BD8D78: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82BD8D7C: 4098000C  bge cr6, 0x82bd8d88
	if !ctx.cr[6].lt {
	pc = 0x82BD8D88; continue 'dispatch;
	}
	// 82BD8D80: 7C6B5030  slw r11, r3, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[3].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8D84: 7D652B78  or r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 82BD8D88: 5149831E  rlwimi r9, r10, 0x10, 0xc, 0xf
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000000F0000) | (ctx.r[9].u64 & 0xFFFFFFFFFFF0FFFF);
	// 82BD8D8C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82BD8D90: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82BD8D94: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82BD8D98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8D9C: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD8DA0: 4198FF94  blt cr6, 0x82bd8d34
	if ctx.cr[6].lt {
	pc = 0x82BD8D34; continue 'dispatch;
	}
	// 82BD8DA4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82BD8DA8: 409A0010  bne cr6, 0x82bd8db8
	if !ctx.cr[6].eq {
	pc = 0x82BD8DB8; continue 'dispatch;
	}
	// 82BD8DAC: 7F042840  cmplw cr6, r4, r5
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82BD8DB0: 419A0008  beq cr6, 0x82bd8db8
	if ctx.cr[6].eq {
	pc = 0x82BD8DB8; continue 'dispatch;
	}
	// 82BD8DB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD8DB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82BD8DBC: 485CF3FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD8DC0 size=140
    let mut pc: u32 = 0x82BD8DC0;
    'dispatch: loop {
        match pc {
            0x82BD8DC0 => {
    //   block [0x82BD8DC0..0x82BD8E4C)
	// 82BD8DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD8DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82BD8DC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82BD8DCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD8DD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD8DD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8DD8: 38A023A0  li r5, 0x23a0
	ctx.r[5].s64 = 9120;
	// 82BD8DDC: 909F2564  stw r4, 0x2564(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(9572 as u32), ctx.r[4].u32 ) };
	// 82BD8DE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8DE4: 917F2560  stw r11, 0x2560(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(9568 as u32), ctx.r[11].u32 ) };
	// 82BD8DE8: F97F23A0  std r11, 0x23a0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(9120 as u32), ctx.r[11].u64 ) };
	// 82BD8DEC: F97F23A8  std r11, 0x23a8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(9128 as u32), ctx.r[11].u64 ) };
	// 82BD8DF0: F97F23B0  std r11, 0x23b0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(9136 as u32), ctx.r[11].u64 ) };
	// 82BD8DF4: 485CF3ED  bl 0x831a81e0
	ctx.lr = 0x82BD8DF8;
	sub_831A81E0(ctx, base);
	// 82BD8DF8: 38A00120  li r5, 0x120
	ctx.r[5].s64 = 288;
	// 82BD8DFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8E00: 387F23F8  addi r3, r31, 0x23f8
	ctx.r[3].s64 = ctx.r[31].s64 + 9208;
	// 82BD8E04: 485CF3DD  bl 0x831a81e0
	ctx.lr = 0x82BD8E08;
	sub_831A81E0(ctx, base);
	// 82BD8E08: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD8E0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8E10: 387F2518  addi r3, r31, 0x2518
	ctx.r[3].s64 = ctx.r[31].s64 + 9496;
	// 82BD8E14: 485CF3CD  bl 0x831a81e0
	ctx.lr = 0x82BD8E18;
	sub_831A81E0(ctx, base);
	// 82BD8E18: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD8E1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8E20: 387F2538  addi r3, r31, 0x2538
	ctx.r[3].s64 = ctx.r[31].s64 + 9528;
	// 82BD8E24: 485CF3BD  bl 0x831a81e0
	ctx.lr = 0x82BD8E28;
	sub_831A81E0(ctx, base);
	// 82BD8E28: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82BD8E2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD8E30: 387F23B8  addi r3, r31, 0x23b8
	ctx.r[3].s64 = ctx.r[31].s64 + 9144;
	// 82BD8E34: 485CF3AD  bl 0x831a81e0
	ctx.lr = 0x82BD8E38;
	sub_831A81E0(ctx, base);
	// 82BD8E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82BD8E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD8E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82BD8E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82BD8E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8E50 size=108
    let mut pc: u32 = 0x82BD8E50;
    'dispatch: loop {
        match pc {
            0x82BD8E50 => {
    //   block [0x82BD8E50..0x82BD8EBC)
	// 82BD8E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8E54: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD8E58: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82BD8E5C: 9163255C  stw r11, 0x255c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(9564 as u32), ctx.r[11].u32 ) };
	// 82BD8E60: 419A0020  beq cr6, 0x82bd8e80
	if ctx.cr[6].eq {
	pc = 0x82BD8E80; continue 'dispatch;
	}
	// 82BD8E64: 39632518  addi r11, r3, 0x2518
	ctx.r[11].s64 = ctx.r[3].s64 + 9496;
	// 82BD8E68: 548AE8FA  rlwinm r10, r4, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 82BD8E6C: 548806FE  clrlwi r8, r4, 0x1b
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82BD8E70: 7D284030  slw r8, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8E74: 7CEA582E  lwzx r7, r10, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82BD8E78: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82BD8E7C: 7D0A592E  stwx r8, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82BD8E80: 39632538  addi r11, r3, 0x2538
	ctx.r[11].s64 = ctx.r[3].s64 + 9528;
	// 82BD8E84: 548AE8FA  rlwinm r10, r4, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 82BD8E88: 548806FE  clrlwi r8, r4, 0x1b
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82BD8E8C: 5487D97E  srwi r7, r4, 5
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD8E90: 7D284030  slw r8, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8E94: 7CCA582E  lwzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82BD8E98: 20E7001F  subfic r7, r7, 0x1f
	ctx.xer.ca = ctx.r[7].u32 <= 31 as u32;
	ctx.r[7].s64 = (31 as i64) - ctx.r[7].s64;
	// 82BD8E9C: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 82BD8EA0: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82BD8EA4: 7D0A592E  stwx r8, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82BD8EA8: 7D2B3836  sld r11, r9, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[9].u64) << ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82BD8EAC: E94323A8  ld r10, 0x23a8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(9128 as u32) ) };
	// 82BD8EB0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD8EB4: F96323A8  std r11, 0x23a8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(9128 as u32), ctx.r[11].u64 ) };
	// 82BD8EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8EC0 size=132
    let mut pc: u32 = 0x82BD8EC0;
    'dispatch: loop {
        match pc {
            0x82BD8EC0 => {
    //   block [0x82BD8EC0..0x82BD8F44)
	// 82BD8EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD8EC4: E94323A0  ld r10, 0x23a0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(9120 as u32) ) };
	// 82BD8EC8: 5489F0BE  srwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD8ECC: 81032564  lwz r8, 0x2564(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9572 as u32) ) } as u64;
	// 82BD8ED0: 9163255C  stw r11, 0x255c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(9564 as u32), ctx.r[11].u32 ) };
	// 82BD8ED4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82BD8ED8: 2169003F  subfic r11, r9, 0x3f
	ctx.xer.ca = ctx.r[9].u32 <= 63 as u32;
	ctx.r[11].s64 = (63 as i64) - ctx.r[9].s64;
	// 82BD8EDC: 392323B8  addi r9, r3, 0x23b8
	ctx.r[9].s64 = ctx.r[3].s64 + 9144;
	// 82BD8EE0: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82BD8EE4: 7CEB5836  sld r11, r7, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[7].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82BD8EE8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD8EEC: F96323A0  std r11, 0x23a0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(9120 as u32), ctx.r[11].u64 ) };
	// 82BD8EF0: 81684DB4  lwz r11, 0x4db4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82BD8EF4: 556B35EE  rlwinm r11, r11, 6, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82BD8EF8: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82BD8EFC: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8F00: 396A0030  addi r11, r10, 0x30
	ctx.r[11].s64 = ctx.r[10].s64 + 48;
	// 82BD8F04: 5548E8FA  rlwinm r8, r10, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82BD8F08: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD8F0C: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD8F10: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82BD8F14: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82BD8F18: 80C50004  lwz r6, 4(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD8F1C: 7CEA5030  slw r10, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8F20: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82BD8F24: 80E50008  lwz r7, 8(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD8F28: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82BD8F2C: 80E5000C  lwz r7, 0xc(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD8F30: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82BD8F34: 7D68482E  lwzx r11, r8, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82BD8F38: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD8F3C: 7D68492E  stwx r11, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82BD8F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8F48 size=56
    let mut pc: u32 = 0x82BD8F48;
    'dispatch: loop {
        match pc {
            0x82BD8F48 => {
    //   block [0x82BD8F48..0x82BD8F80)
	// 82BD8F48: 548AD97E  srwi r10, r4, 5
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8F4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82BD8F50: 394A08EE  addi r10, r10, 0x8ee
	ctx.r[10].s64 = ctx.r[10].s64 + 2286;
	// 82BD8F54: 548906FE  clrlwi r9, r4, 0x1b
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82BD8F58: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8F5C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD8F60: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8F64: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82BD8F68: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82BD8F6C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82BD8F70: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD8F74: 69430001  xori r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u64 ^ 1;
	// 82BD8F78: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82BD8F7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8F80 size=48
    let mut pc: u32 = 0x82BD8F80;
    'dispatch: loop {
        match pc {
            0x82BD8F80 => {
    //   block [0x82BD8F80..0x82BD8FB0)
	// 82BD8F80: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82BD8F84: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8F88: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82BD8F8C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82BD8F90: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD8F94: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD8F98: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD8F9C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD8FA0: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82BD8FA4: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD8FA8: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82BD8FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD8FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD8FB0 size=116
    let mut pc: u32 = 0x82BD8FB0;
    'dispatch: loop {
        match pc {
            0x82BD8FB0 => {
    //   block [0x82BD8FB0..0x82BD9024)
	// 82BD8FB0: 81430070  lwz r10, 0x70(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82BD8FB4: 3963006C  addi r11, r3, 0x6c
	ctx.r[11].s64 = ctx.r[3].s64 + 108;
	// 82BD8FB8: 8123006C  lwz r9, 0x6c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82BD8FBC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD8FC0: 7D2307B4  extsw r3, r9
	ctx.r[3].s64 = ctx.r[9].s32 as i64;
	// 82BD8FC4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82BD8FC8: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD8FCC: 40980064  bge cr6, 0x82bd9030
	if !ctx.cr[6].lt {
		sub_82BD9024(ctx, base);
		return;
	}
	// 82BD8FD0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD8FD4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD8FD8: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD8FDC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD8FE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD8FE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD8FE8: 5568A73E  rlwinm r8, r11, 0x14, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD8FEC: 7D4A4030  slw r10, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD8FF0: 714A607E  andi. r10, r10, 0x607e
	ctx.r[10].u64 = ctx.r[10].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD8FF4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD8FF8: 4182002C  beq 0x82bd9024
	if ctx.cr[0].eq {
		sub_82BD9024(ctx, base);
		return;
	}
	// 82BD8FFC: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9000: 40820024  bne 0x82bd9024
	if !ctx.cr[0].eq {
		sub_82BD9024(ctx, base);
		return;
	}
	// 82BD9004: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9008: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD900C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82BD9010: 41990014  bgt cr6, 0x82bd9024
	if ctx.cr[6].gt {
		sub_82BD9024(ctx, base);
		return;
	}
	// 82BD9014: 556BA77E  rlwinm r11, r11, 0x14, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD9018: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD901C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD9020: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9024 size=20
    let mut pc: u32 = 0x82BD9024;
    'dispatch: loop {
        match pc {
            0x82BD9024 => {
    //   block [0x82BD9024..0x82BD9038)
	// 82BD9024: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82BD9028: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD902C: 4198FFB4  blt cr6, 0x82bd8fe0
	if ctx.cr[6].lt {
		sub_82BD8FB0(ctx, base);
		return;
	}
	// 82BD9030: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD9034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD9038 size=240
    let mut pc: u32 = 0x82BD9038;
    'dispatch: loop {
        match pc {
            0x82BD9038 => {
    //   block [0x82BD9038..0x82BD9128)
	// 82BD9038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD903C: 485CF131  bl 0x831a816c
	ctx.lr = 0x82BD9040;
	sub_831A8130(ctx, base);
	// 82BD9040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD9044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82BD9048: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82BD904C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82BD9050: 4BFFFF61  bl 0x82bd8fb0
	ctx.lr = 0x82BD9054;
	sub_82BD8FB0(ctx, base);
	// 82BD9054: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD9058: 418200C8  beq 0x82bd9120
	if ctx.cr[0].eq {
	pc = 0x82BD9120; continue 'dispatch;
	}
	// 82BD905C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9060: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82BD9064: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82BD9068: 5569053E  clrlwi r9, r11, 0x14
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD906C: 7D29F050  subf r9, r9, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 82BD9070: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD9074: 7D4A4830  slw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD9078: 419A0018  beq cr6, 0x82bd9090
	if ctx.cr[6].eq {
	pc = 0x82BD9090; continue 'dispatch;
	}
	// 82BD907C: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD9080: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD9084: 516A0406  rlwimi r10, r11, 0, 0x10, 3
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFF000FFFF) | (ctx.r[10].u64 & 0x000000000FFF0000);
	// 82BD9088: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82BD908C: 4800001C  b 0x82bd90a8
	pc = 0x82BD90A8; continue 'dispatch;
	// 82BD9090: 3D20F000  lis r9, -0x1000
	ctx.r[9].s64 = -268435456;
	// 82BD9094: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82BD9098: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82BD909C: 5149811E  rlwimi r9, r10, 0x10, 4, 0xf
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000FFF0000) | (ctx.r[9].u64 & 0xFFFFFFFFF000FFFF);
	// 82BD90A0: 7D2B5838  and r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82BD90A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD90A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD90AC: 816B4DB4  lwz r11, 0x4db4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19892 as u32) ) } as u64;
	// 82BD90B0: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82BD90B4: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD90B8: 41820068  beq 0x82bd9120
	if ctx.cr[0].eq {
	pc = 0x82BD9120; continue 'dispatch;
	}
	// 82BD90BC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82BD90C0: 419A0060  beq cr6, 0x82bd9120
	if ctx.cr[6].eq {
	pc = 0x82BD9120; continue 'dispatch;
	}
	// 82BD90C4: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82BD90C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82BD90CC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD90D0: 409A0050  bne cr6, 0x82bd9120
	if !ctx.cr[6].eq {
	pc = 0x82BD9120; continue 'dispatch;
	}
	// 82BD90D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD90D8: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD90DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD90E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD90E4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82BD90E8: 997F007C  stb r11, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u8 ) };
	// 82BD90EC: 485CF0F5  bl 0x831a81e0
	ctx.lr = 0x82BD90F0;
	sub_831A81E0(ctx, base);
	// 82BD90F0: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD90F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD90F8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82BD90FC: 485CF0E5  bl 0x831a81e0
	ctx.lr = 0x82BD9100;
	sub_831A81E0(ctx, base);
	// 82BD9100: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD9104: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD9108: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82BD910C: 485CF0D5  bl 0x831a81e0
	ctx.lr = 0x82BD9110;
	sub_831A81E0(ctx, base);
	// 82BD9110: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82BD9114: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD9118: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82BD911C: 485CF0C5  bl 0x831a81e0
	ctx.lr = 0x82BD9120;
	sub_831A81E0(ctx, base);
	// 82BD9120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82BD9124: 485CF098  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9128 size=20
    let mut pc: u32 = 0x82BD9128;
    'dispatch: loop {
        match pc {
            0x82BD9128 => {
    //   block [0x82BD9128..0x82BD913C)
	// 82BD9128: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD912C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9130: 4098000C  bge cr6, 0x82bd913c
	if !ctx.cr[6].lt {
		sub_82BD913C(ctx, base);
		return;
	}
	// 82BD9134: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD9138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD913C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD913C size=140
    let mut pc: u32 = 0x82BD913C;
    'dispatch: loop {
        match pc {
            0x82BD913C => {
    //   block [0x82BD913C..0x82BD91C8)
	// 82BD913C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82BD9140: 419A00B8  beq cr6, 0x82bd91f8
	if ctx.cr[6].eq {
		sub_82BD91C8(ctx, base);
		return;
	}
	// 82BD9144: 81430070  lwz r10, 0x70(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82BD9148: 3963006C  addi r11, r3, 0x6c
	ctx.r[11].s64 = ctx.r[3].s64 + 108;
	// 82BD914C: 8103006C  lwz r8, 0x6c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82BD9150: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD9154: 7D0A07B4  extsw r10, r8
	ctx.r[10].s64 = ctx.r[8].s32 as i64;
	// 82BD9158: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82BD915C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD9160: 40980098  bge cr6, 0x82bd91f8
	if !ctx.cr[6].lt {
		sub_82BD91C8(ctx, base);
		return;
	}
	// 82BD9164: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9168: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD916C: 550B1838  slwi r11, r8, 3
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD9170: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82BD9174: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82BD9178: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD917C: 5569A73E  rlwinm r9, r11, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD9180: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD9184: 7129607E  andi. r9, r9, 0x607e
	ctx.r[9].u64 = ctx.r[9].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD9188: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD918C: 4182002C  beq 0x82bd91b8
	if ctx.cr[0].eq {
	pc = 0x82BD91B8; continue 'dispatch;
	}
	// 82BD9190: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9194: 40820024  bne 0x82bd91b8
	if !ctx.cr[0].eq {
	pc = 0x82BD91B8; continue 'dispatch;
	}
	// 82BD9198: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD919C: 5569053E  clrlwi r9, r11, 0x14
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD91A0: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82BD91A4: 41990014  bgt cr6, 0x82bd91b8
	if ctx.cr[6].gt {
	pc = 0x82BD91B8; continue 'dispatch;
	}
	// 82BD91A8: 556BA77E  rlwinm r11, r11, 0x14, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD91AC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82BD91B0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD91B4: 41980014  blt cr6, 0x82bd91c8
	if ctx.cr[6].lt {
		sub_82BD91C8(ctx, base);
		return;
	}
	// 82BD91B8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82BD91BC: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82BD91C0: 4198FFB8  blt cr6, 0x82bd9178
	if ctx.cr[6].lt {
	pc = 0x82BD9178; continue 'dispatch;
	}
	// 82BD91C4: 48000034  b 0x82bd91f8
	sub_82BD91C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD91C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD91C8 size=64
    let mut pc: u32 = 0x82BD91C8;
    'dispatch: loop {
        match pc {
            0x82BD91C8 => {
    //   block [0x82BD91C8..0x82BD9208)
	// 82BD91C8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD91CC: 556A053E  clrlwi r10, r11, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD91D0: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD91D4: 7D4A2050  subf r10, r10, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 82BD91D8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD91DC: 7D0A5030  slw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD91E0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82BD91E4: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD91E8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82BD91EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD91F0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82BD91F4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD91F8: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82BD91FC: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 82BD9200: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD9204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9208 size=32
    let mut pc: u32 = 0x82BD9208;
    'dispatch: loop {
        match pc {
            0x82BD9208 => {
    //   block [0x82BD9208..0x82BD9228)
	// 82BD9208: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82BD920C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD9210: 419A0058  beq cr6, 0x82bd9268
	if ctx.cr[6].eq {
		sub_82BD9268(ctx, base);
		return;
	}
	// 82BD9214: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD9218: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD921C: 4098000C  bge cr6, 0x82bd9228
	if !ctx.cr[6].lt {
		sub_82BD9228(ctx, base);
		return;
	}
	// 82BD9220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD9224: 48000014  b 0x82bd9238
	sub_82BD9228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9228 size=64
    let mut pc: u32 = 0x82BD9228;
    'dispatch: loop {
        match pc {
            0x82BD9228 => {
    //   block [0x82BD9228..0x82BD9268)
	// 82BD9228: 8143006C  lwz r10, 0x6c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82BD922C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD9230: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82BD9234: 394BFFF8  addi r10, r11, -8
	ctx.r[10].s64 = ctx.r[11].s64 + -8;
	// 82BD9238: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD923C: 556BA73F  rlwinm. r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9240: 41820028  beq 0x82bd9268
	if ctx.cr[0].eq {
		sub_82BD9268(ctx, base);
		return;
	}
	// 82BD9244: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82BD9248: 40990014  ble cr6, 0x82bd925c
	if !ctx.cr[6].gt {
	pc = 0x82BD925C; continue 'dispatch;
	}
	// 82BD924C: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82BD9250: 40990018  ble cr6, 0x82bd9268
	if !ctx.cr[6].gt {
		sub_82BD9268(ctx, base);
		return;
	}
	// 82BD9254: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 82BD9258: 41990010  bgt cr6, 0x82bd9268
	if ctx.cr[6].gt {
		sub_82BD9268(ctx, base);
		return;
	}
	// 82BD925C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9260: 5563A77E  rlwinm r3, r11, 0x14, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD9264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9268 size=8
    let mut pc: u32 = 0x82BD9268;
    'dispatch: loop {
        match pc {
            0x82BD9268 => {
    //   block [0x82BD9268..0x82BD9270)
	// 82BD9268: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82BD926C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9270 size=624
    //   switch @ 0x82BD92AC: r9 with 12 label(s)
    //       case  0 → 0x82BD92BC
    //       case  1 → 0x82BD92BC
    //       case  2 → 0x82BD932C
    //       case  3 → 0x82BD932C
    //       case  4 → 0x82BD92E0
    //       case  5 → 0x82BD92E0
    //       case  6 → 0x82BD9300
    //       case  7 → 0x82BD932C
    //       case  8 → 0x82BD9300
    //       case  9 → 0x82BD9318
    //       case 10 → 0x82BD92BC
    //       case 11 → 0x82BD92BC
    let mut pc: u32 = 0x82BD9270;
    'dispatch: loop {
        match pc {
            0x82BD9270 => {
    //   block [0x82BD9270..0x82BD92BC)
	// 82BD9270: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82BD9274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9278: 418200BC  beq 0x82bd9334
	if ctx.cr[0].eq {
	pc = 0x82BD9334; continue 'dispatch;
	}
	// 82BD927C: 81470004  lwz r10, 4(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9280: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9284: 5549A73E  rlwinm r9, r10, 0x14, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82BD9288: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82BD928C: 2B09000B  cmplwi cr6, r9, 0xb
	ctx.cr[6].compare_u32(ctx.r[9].u32, 11 as u32, &mut ctx.xer);
	// 82BD9290: 4199009C  bgt cr6, 0x82bd932c
	if ctx.cr[6].gt {
	pc = 0x82BD932C; continue 'dispatch;
	}
	// 82BD9294: 3D80820D  lis r12, -0x7df3
	ctx.r[12].s64 = -2113077248;
	// 82BD9298: 398CC488  addi r12, r12, -0x3b78
	ctx.r[12].s64 = ctx.r[12].s64 + -15224;
	// 82BD929C: 7C0C48AE  lbzx r0, r12, r9
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82BD92A0: 3D8082BE  lis r12, -0x7d42
	ctx.r[12].s64 = -2101477376;
	// 82BD92A4: 398C92BC  addi r12, r12, -0x6d44
	ctx.r[12].s64 = ctx.r[12].s64 + -27972;
	// 82BD92A8: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82BD92AC: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82BD92B0: 60000000  nop
	// 82BD92B4: 60000000  nop
	// 82BD92B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82BD92BC => {
    //   block [0x82BD92BC..0x82BD92E0)
	// 82BD92BC: 554AC7BE  rlwinm r10, r10, 0x18, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82BD92C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD92C4: 214A0003  subfic r10, r10, 3
	ctx.xer.ca = ctx.r[10].u32 <= 3 as u32;
	ctx.r[10].s64 = (3 as i64) - ctx.r[10].s64;
	// 82BD92C8: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD92CC: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD92D0: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD92D4: 516A0416  rlwimi r10, r11, 0, 0x10, 0xb
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[10].u64 & 0x00000000000F0000);
	// 82BD92D8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD92DC: 48000050  b 0x82bd932c
	pc = 0x82BD932C; continue 'dispatch;
            }
            0x82BD92E0 => {
    //   block [0x82BD92E0..0x82BD9300)
	// 82BD92E0: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD92E4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82BD92E8: 554A7F3E  rlwinm r10, r10, 0xf, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0001FFFFu64;
	// 82BD92EC: 214A000F  subfic r10, r10, 0xf
	ctx.xer.ca = ctx.r[10].u32 <= 15 as u32;
	ctx.r[10].s64 = (15 as i64) - ctx.r[10].s64;
	// 82BD92F0: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD92F4: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82BD92F8: 516A001E  rlwimi r10, r11, 0, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82BD92FC: 4BFFFFDC  b 0x82bd92d8
	pc = 0x82BD92D8; continue 'dispatch;
            }
            0x82BD9300 => {
    //   block [0x82BD9300..0x82BD9318)
	// 82BD9300: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9304: 552704A5  rlwinm. r7, r9, 0, 0x12, 0x12
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82BD9308: 40820024  bne 0x82bd932c
	if !ctx.cr[0].eq {
	pc = 0x82BD932C; continue 'dispatch;
	}
	// 82BD930C: 55290463  rlwinm. r9, r9, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD9310: 4082001C  bne 0x82bd932c
	if !ctx.cr[0].eq {
	pc = 0x82BD932C; continue 'dispatch;
	}
	// 82BD9314: 4BFFFFA8  b 0x82bd92bc
	pc = 0x82BD92BC; continue 'dispatch;
            }
            0x82BD9318 => {
    //   block [0x82BD9318..0x82BD932C)
	// 82BD9318: 554A056C  rlwinm r10, r10, 0, 0x15, 0x16
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD931C: 2B0A0600  cmplwi cr6, r10, 0x600
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1536 as u32, &mut ctx.xer);
	// 82BD9320: 409A000C  bne cr6, 0x82bd932c
	if !ctx.cr[6].eq {
	pc = 0x82BD932C; continue 'dispatch;
	}
	// 82BD9324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD9328: 91480018  stw r10, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	pc = 0x82BD932C; continue 'dispatch;
            }
            0x82BD932C => {
    //   block [0x82BD932C..0x82BD94E0)
	// 82BD932C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD9330: 480001A8  b 0x82bd94d8
	pc = 0x82BD94D8; continue 'dispatch;
	// 82BD9334: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82BD9338: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD933C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9340: 409A0034  bne cr6, 0x82bd9374
	if !ctx.cr[6].eq {
	pc = 0x82BD9374; continue 'dispatch;
	}
	// 82BD9344: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9348: 9068000C  stw r3, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82BD934C: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82BD9350: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD9354: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82BD9358: 91680010  stw r11, 0x10(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82BD935C: 41820010  beq 0x82bd936c
	if ctx.cr[0].eq {
	pc = 0x82BD936C; continue 'dispatch;
	}
	// 82BD9360: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9364: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82BD9368: 41820008  beq 0x82bd9370
	if ctx.cr[0].eq {
	pc = 0x82BD9370; continue 'dispatch;
	}
	// 82BD936C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82BD9370: 91680014  stw r11, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82BD9374: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9378: 548A063F  clrlwi. r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD937C: 4182002C  beq 0x82bd93a8
	if ctx.cr[0].eq {
	pc = 0x82BD93A8; continue 'dispatch;
	}
	// 82BD9380: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9384: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD9388: 554906FF  clrlwi. r9, r10, 0x1b
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82BD938C: 5549A6BE  rlwinm r9, r10, 0x14, 0x1a, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82BD9390: 419A000C  beq cr6, 0x82bd939c
	if ctx.cr[6].eq {
	pc = 0x82BD939C; continue 'dispatch;
	}
	// 82BD9394: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82BD9398: 40980008  bge cr6, 0x82bd93a0
	if !ctx.cr[6].lt {
	pc = 0x82BD93A0; continue 'dispatch;
	}
	// 82BD939C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82BD93A0: 554ADEBE  rlwinm r10, r10, 0x1b, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD93A4: 4800011C  b 0x82bd94c0
	pc = 0x82BD94C0; continue 'dispatch;
	// 82BD93A8: 81270008  lwz r9, 8(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD93AC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82BD93B0: 80A70000  lwz r5, 0(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD93B4: 394AEC68  addi r10, r10, -0x1398
	ctx.r[10].s64 = ctx.r[10].s64 + -5016;
	// 82BD93B8: 552646FE  rlwinm r6, r9, 8, 0x1b, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82BD93BC: 388A0020  addi r4, r10, 0x20
	ctx.r[4].s64 = ctx.r[10].s64 + 32;
	// 82BD93C0: 54BF36BE  srwi r31, r5, 0x1a
	ctx.r[31].u32 = ctx.r[5].u32.wrapping_shr(26);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82BD93C4: 7CC650AE  lbzx r6, r6, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82BD93C8: 7C9F20AE  lbzx r4, r31, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82BD93CC: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82BD93D0: 41980028  blt cr6, 0x82bd93f8
	if ctx.cr[6].lt {
	pc = 0x82BD93F8; continue 'dispatch;
	}
	// 82BD93D4: 552A0001  rlwinm. r10, r9, 0, 0, 0
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD93D8: 41820020  beq 0x82bd93f8
	if ctx.cr[0].eq {
	pc = 0x82BD93F8; continue 'dispatch;
	}
	// 82BD93DC: 89470009  lbz r10, 9(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(9 as u32) ) } as u64;
	// 82BD93E0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD93E4: 554A06BE  clrlwi r10, r10, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82BD93E8: 419A000C  beq cr6, 0x82bd93f4
	if ctx.cr[6].eq {
	pc = 0x82BD93F4; continue 'dispatch;
	}
	// 82BD93EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD93F0: 40980008  bge cr6, 0x82bd93f8
	if !ctx.cr[6].lt {
	pc = 0x82BD93F8; continue 'dispatch;
	}
	// 82BD93F4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD93F8: 2B060002  cmplwi cr6, r6, 2
	ctx.cr[6].compare_u32(ctx.r[6].u32, 2 as u32, &mut ctx.xer);
	// 82BD93FC: 41980028  blt cr6, 0x82bd9424
	if ctx.cr[6].lt {
	pc = 0x82BD9424; continue 'dispatch;
	}
	// 82BD9400: 552A0043  rlwinm. r10, r9, 0, 1, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD9404: 41820020  beq 0x82bd9424
	if ctx.cr[0].eq {
	pc = 0x82BD9424; continue 'dispatch;
	}
	// 82BD9408: 8947000A  lbz r10, 0xa(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(10 as u32) ) } as u64;
	// 82BD940C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD9410: 554A06BE  clrlwi r10, r10, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82BD9414: 419A000C  beq cr6, 0x82bd9420
	if ctx.cr[6].eq {
	pc = 0x82BD9420; continue 'dispatch;
	}
	// 82BD9418: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD941C: 40980008  bge cr6, 0x82bd9424
	if !ctx.cr[6].lt {
	pc = 0x82BD9424; continue 'dispatch;
	}
	// 82BD9420: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD9424: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 82BD9428: 4098000C  bge cr6, 0x82bd9434
	if !ctx.cr[6].lt {
	pc = 0x82BD9434; continue 'dispatch;
	}
	// 82BD942C: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82BD9430: 409A0024  bne cr6, 0x82bd9454
	if !ctx.cr[6].eq {
	pc = 0x82BD9454; continue 'dispatch;
	}
	// 82BD9434: 552A0085  rlwinm. r10, r9, 0, 2, 2
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD9438: 4182001C  beq 0x82bd9454
	if ctx.cr[0].eq {
	pc = 0x82BD9454; continue 'dispatch;
	}
	// 82BD943C: 552A06BE  clrlwi r10, r9, 0x1a
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82BD9440: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD9444: 419A000C  beq cr6, 0x82bd9450
	if ctx.cr[6].eq {
	pc = 0x82BD9450; continue 'dispatch;
	}
	// 82BD9448: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD944C: 40980008  bge cr6, 0x82bd9454
	if !ctx.cr[6].lt {
	pc = 0x82BD9454; continue 'dispatch;
	}
	// 82BD9450: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD9454: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 82BD9458: 409A002C  bne cr6, 0x82bd9484
	if !ctx.cr[6].eq {
	pc = 0x82BD9484; continue 'dispatch;
	}
	// 82BD945C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82BD9460: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9464: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD9468: 512AF108  rlwimi r10, r9, 0x1e, 4, 4
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[10].u64 & 0xFFFFFFFFF7FFFFFF);
	// 82BD946C: 514737BE  rlwimi r7, r10, 6, 0x1e, 0x1f
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[7].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82BD9470: 54EA06BE  clrlwi r10, r7, 0x1a
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x0000003Fu64;
	// 82BD9474: 419A000C  beq cr6, 0x82bd9480
	if ctx.cr[6].eq {
	pc = 0x82BD9480; continue 'dispatch;
	}
	// 82BD9478: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD947C: 40980008  bge cr6, 0x82bd9484
	if !ctx.cr[6].lt {
	pc = 0x82BD9484; continue 'dispatch;
	}
	// 82BD9480: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD9484: 54AA0421  rlwinm. r10, r5, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD9488: 54AA06BE  clrlwi r10, r5, 0x1a
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000003Fu64;
	// 82BD948C: 4182001C  beq 0x82bd94a8
	if ctx.cr[0].eq {
	pc = 0x82BD94A8; continue 'dispatch;
	}
	// 82BD9490: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 82BD9494: 41980040  blt cr6, 0x82bd94d4
	if ctx.cr[6].lt {
	pc = 0x82BD94D4; continue 'dispatch;
	}
	// 82BD9498: 2B0A0025  cmplwi cr6, r10, 0x25
	ctx.cr[6].compare_u32(ctx.r[10].u32, 37 as u32, &mut ctx.xer);
	// 82BD949C: 41990038  bgt cr6, 0x82bd94d4
	if ctx.cr[6].gt {
	pc = 0x82BD94D4; continue 'dispatch;
	}
	// 82BD94A0: 90680018  stw r3, 0x18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82BD94A4: 48000030  b 0x82bd94d4
	pc = 0x82BD94D4; continue 'dispatch;
	// 82BD94A8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD94AC: 419A000C  beq cr6, 0x82bd94b8
	if ctx.cr[6].eq {
	pc = 0x82BD94B8; continue 'dispatch;
	}
	// 82BD94B0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD94B4: 40980008  bge cr6, 0x82bd94bc
	if !ctx.cr[6].lt {
	pc = 0x82BD94BC; continue 'dispatch;
	}
	// 82BD94B8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD94BC: 54AAC6BE  rlwinm r10, r5, 0x18, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82BD94C0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82BD94C4: 419A000C  beq cr6, 0x82bd94d0
	if ctx.cr[6].eq {
	pc = 0x82BD94D0; continue 'dispatch;
	}
	// 82BD94C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82BD94CC: 40980008  bge cr6, 0x82bd94d4
	if !ctx.cr[6].lt {
	pc = 0x82BD94D4; continue 'dispatch;
	}
	// 82BD94D0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82BD94D4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82BD94D8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82BD94DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD94E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82BD94E0 size=480
    let mut pc: u32 = 0x82BD94E0;
    'dispatch: loop {
        match pc {
            0x82BD94E0 => {
    //   block [0x82BD94E0..0x82BD96C0)
	// 82BD94E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82BD94E4: 485CEC6D  bl 0x831a8150
	ctx.lr = 0x82BD94E8;
	sub_831A8130(ctx, base);
	// 82BD94E8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82BD94EC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82BD94F0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82BD94F4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82BD94F8: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82BD94FC: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9500: 83780100  lwz r27, 0x100(r24)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(256 as u32) ) } as u64;
	// 82BD9504: 3B40000C  li r26, 0xc
	ctx.r[26].s64 = 12;
	// 82BD9508: 83980104  lwz r28, 0x104(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(260 as u32) ) } as u64;
	// 82BD950C: 418200C4  beq 0x82bd95d0
	if ctx.cr[0].eq {
	pc = 0x82BD95D0; continue 'dispatch;
	}
	// 82BD9510: 7D7BE050  subf r11, r27, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 82BD9514: 7FABD3D7  divw. r29, r11, r26
	ctx.r[29].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82BD9518: 418200B8  beq 0x82bd95d0
	if ctx.cr[0].eq {
	pc = 0x82BD95D0; continue 'dispatch;
	}
	// 82BD951C: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82BD9520: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82BD9524: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD9528: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 82BD952C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82BD9530: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82BD9534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD9538: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD953C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82BD9540: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD9544: A17FFFFE  lhz r11, -2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82BD9548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82BD954C: A17FFFFC  lhz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82BD9550: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9554: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD9558: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD955C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82BD9560: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9564: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82BD9568: 7EE903A6  mtctr r23
	ctx.ctr.u64 = ctx.r[23].u64;
	// 82BD956C: 4E800421  bctrl
	ctx.lr = 0x82BD9570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD9570: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 82BD9574: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82BD9578: 38DE0001  addi r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 1;
	// 82BD957C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82BD9580: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82BD9584: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82BD9588: 7EE903A6  mtctr r23
	ctx.ctr.u64 = ctx.r[23].u64;
	// 82BD958C: 4E800421  bctrl
	ctx.lr = 0x82BD9590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82BD9590: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82BD9594: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82BD9598: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82BD959C: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82BD95A0: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82BD95A4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82BD95A8: 514B801E  rlwimi r11, r10, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82BD95AC: 917FFFFC  stw r11, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82BD95B0: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82BD95B4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82BD95B8: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD95BC: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD95C0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD95C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82BD95C8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82BD95CC: 4082FF58  bne 0x82bd9524
	if !ctx.cr[0].eq {
	pc = 0x82BD9524; continue 'dispatch;
	}
	// 82BD95D0: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD95D4: 418200E4  beq 0x82bd96b8
	if ctx.cr[0].eq {
	pc = 0x82BD96B8; continue 'dispatch;
	}
	// 82BD95D8: 7D5BE050  subf r10, r27, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 82BD95DC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82BD95E0: 7F4AD3D6  divw r26, r10, r26
	ctx.r[26].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 82BD95E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82BD95E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82BD95EC: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82BD95F0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82BD95F4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82BD95F8: 419A00C0  beq cr6, 0x82bd96b8
	if ctx.cr[6].eq {
	pc = 0x82BD96B8; continue 'dispatch;
	}
	// 82BD95FC: 3BDB0008  addi r30, r27, 8
	ctx.r[30].s64 = ctx.r[27].s64 + 8;
	// 82BD9600: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82BD9604: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82BD9608: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 82BD960C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD9610: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82BD9614: 813EFFF8  lwz r9, -8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82BD9618: 5568843E  srwi r8, r11, 0x10
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82BD961C: 5547801E  slwi r7, r10, 0x10
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82BD9620: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82BD9624: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82BD9628: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD962C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82BD9630: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82BD9634: 91010078  stw r8, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u32 ) };
	// 82BD9638: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82BD963C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9640: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD9644: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82BD9648: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD964C: 716B607E  andi. r11, r11, 0x607e
	ctx.r[11].u64 = ctx.r[11].u64 & 24702;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82BD9650: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82BD9654: 41820024  beq 0x82bd9678
	if ctx.cr[0].eq {
	pc = 0x82BD9678; continue 'dispatch;
	}
	// 82BD9658: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82BD965C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82BD9660: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82BD9664: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82BD9668: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82BD966C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82BD9670: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82BD9674: 4BFFDC75  bl 0x82bd72e8
	ctx.lr = 0x82BD9678;
	sub_82BD72E8(ctx, base);
	// 82BD9678: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82BD967C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82BD9680: 4082FFBC  bne 0x82bd963c
	if !ctx.cr[0].eq {
	pc = 0x82BD963C; continue 'dispatch;
	}
	// 82BD9684: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82BD9688: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82BD968C: 4082FF78  bne 0x82bd9604
	if !ctx.cr[0].eq {
	pc = 0x82BD9604; continue 'dispatch;
	}
	// 82BD9690: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82BD9694: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82BD9698: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82BD969C: 409A0014  bne cr6, 0x82bd96b0
	if !ctx.cr[6].eq {
	pc = 0x82BD96B0; continue 'dispatch;
	}
	// 82BD96A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD96A4: 419A0014  beq cr6, 0x82bd96b8
	if ctx.cr[6].eq {
	pc = 0x82BD96B8; continue 'dispatch;
	}
	// 82BD96A8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82BD96AC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82BD96B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82BD96B4: 409AFF34  bne cr6, 0x82bd95e8
	if !ctx.cr[6].eq {
	pc = 0x82BD95E8; continue 'dispatch;
	}
	// 82BD96B8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82BD96BC: 485CEAE4  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD96C0 size=16
    let mut pc: u32 = 0x82BD96C0;
    'dispatch: loop {
        match pc {
            0x82BD96C0 => {
    //   block [0x82BD96C0..0x82BD96D0)
	// 82BD96C0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD96C4: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD96C8: 2B0B0013  cmplwi cr6, r11, 0x13
	ctx.cr[6].compare_u32(ctx.r[11].u32, 19 as u32, &mut ctx.xer);
	// 82BD96CC: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD96D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD96D0 size=152
    let mut pc: u32 = 0x82BD96D0;
    'dispatch: loop {
        match pc {
            0x82BD96D0 => {
    //   block [0x82BD96D0..0x82BD9768)
	// 82BD96D0: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD96D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82BD96D8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82BD96DC: 5529077E  clrlwi r9, r9, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82BD96E0: 2B090007  cmplwi cr6, r9, 7
	ctx.cr[6].compare_u32(ctx.r[9].u32, 7 as u32, &mut ctx.xer);
	// 82BD96E4: 419A001C  beq cr6, 0x82bd9700
	if ctx.cr[6].eq {
	pc = 0x82BD9700; continue 'dispatch;
	}
	// 82BD96E8: 5569E8FA  rlwinm r9, r11, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD96EC: 556806FE  clrlwi r8, r11, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD96F0: 7D484030  slw r8, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD96F4: 7CE9182E  lwzx r7, r9, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD96F8: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82BD96FC: 7D09192E  stwx r8, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 82BD9700: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9704: 552906B8  rlwinm r9, r9, 0, 0x1a, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD9708: 2B090038  cmplwi cr6, r9, 0x38
	ctx.cr[6].compare_u32(ctx.r[9].u32, 56 as u32, &mut ctx.xer);
	// 82BD970C: 419A0020  beq cr6, 0x82bd972c
	if ctx.cr[6].eq {
	pc = 0x82BD972C; continue 'dispatch;
	}
	// 82BD9710: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82BD9714: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82BD9718: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD971C: 7D494830  slw r9, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD9720: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD9724: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD9728: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD972C: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD9730: 552905F2  rlwinm r9, r9, 0, 0x17, 0x19
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD9734: 2B0901C0  cmplwi cr6, r9, 0x1c0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 448 as u32, &mut ctx.xer);
	// 82BD9738: 419A0020  beq cr6, 0x82bd9758
	if ctx.cr[6].eq {
	pc = 0x82BD9758; continue 'dispatch;
	}
	// 82BD973C: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82BD9740: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82BD9744: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD9748: 7D494830  slw r9, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD974C: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD9750: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD9754: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD9758: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82BD975C: 5529052C  rlwinm r9, r9, 0, 0x14, 0x16
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD9760: 2B090E00  cmplwi cr6, r9, 0xe00
	ctx.cr[6].compare_u32(ctx.r[9].u32, 3584 as u32, &mut ctx.xer);
	// 82BD9764: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9768 size=32
    let mut pc: u32 = 0x82BD9768;
    'dispatch: loop {
        match pc {
            0x82BD9768 => {
    //   block [0x82BD9768..0x82BD9788)
	// 82BD9768: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82BD976C: 5569E8FA  rlwinm r9, r11, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82BD9770: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82BD9774: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD9778: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD977C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD9780: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82BD9784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9788 size=44
    let mut pc: u32 = 0x82BD9788;
    'dispatch: loop {
        match pc {
            0x82BD9788 => {
    //   block [0x82BD9788..0x82BD97B4)
	// 82BD9788: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD978C: 552B06FE  clrlwi r11, r9, 0x1b
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD9790: 2B0B0012  cmplwi cr6, r11, 0x12
	ctx.cr[6].compare_u32(ctx.r[11].u32, 18 as u32, &mut ctx.xer);
	// 82BD9794: 41990134  bgt cr6, 0x82bd98c8
	if ctx.cr[6].gt {
		sub_82BD98C8(ctx, base);
		return;
	}
	// 82BD9798: 419A00F8  beq cr6, 0x82bd9890
	if ctx.cr[6].eq {
		sub_82BD9890(ctx, base);
		return;
	}
	// 82BD979C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82BD97A0: 419A00B0  beq cr6, 0x82bd9850
	if ctx.cr[6].eq {
		sub_82BD9850(ctx, base);
		return;
	}
	// 82BD97A4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82BD97A8: 419A0014  beq cr6, 0x82bd97bc
	if ctx.cr[6].eq {
		sub_82BD97BC(ctx, base);
		return;
	}
	// 82BD97AC: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82BD97B0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD97B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD97B4 size=8
    let mut pc: u32 = 0x82BD97B4;
    'dispatch: loop {
        match pc {
            0x82BD97B4 => {
    //   block [0x82BD97B4..0x82BD97BC)
	// 82BD97B4: 2B0B0011  cmplwi cr6, r11, 0x11
	ctx.cr[6].compare_u32(ctx.r[11].u32, 17 as u32, &mut ctx.xer);
	// 82BD97B8: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD97BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD97BC size=108
    let mut pc: u32 = 0x82BD97BC;
    'dispatch: loop {
        match pc {
            0x82BD97BC => {
    //   block [0x82BD97BC..0x82BD9828)
	// 82BD97BC: 552937BE  rlwinm r9, r9, 6, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x03FFFFFFu64;
	// 82BD97C0: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82BD97C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82BD97C8: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82BD97CC: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82BD97D0: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD97D4: 7D694830  slw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD97D8: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD97DC: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD97E0: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD97E4: 81260008  lwz r9, 8(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD97E8: 55290422  rlwinm r9, r9, 0, 0x10, 0x11
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD97EC: 2B094000  cmplwi cr6, r9, 0x4000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16384 as u32, &mut ctx.xer);
	// 82BD97F0: 41980028  blt cr6, 0x82bd9818
	if ctx.cr[6].lt {
	pc = 0x82BD9818; continue 'dispatch;
	}
	// 82BD97F4: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD97F8: 552927BE  rlwinm r9, r9, 4, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0FFFFFFFu64;
	// 82BD97FC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82BD9800: 5528E8FA  rlwinm r8, r9, 0x1d, 3, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 82BD9804: 552906FE  clrlwi r9, r9, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82BD9808: 7D694830  slw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD980C: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD9810: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82BD9814: 7D28192E  stwx r9, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82BD9818: 81260008  lwz r9, 8(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82BD981C: 55290422  rlwinm r9, r9, 0, 0x10, 0x11
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82BD9820: 2B098000  cmplwi cr6, r9, 0x8000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32768 as u32, &mut ctx.xer);
	// 82BD9824: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82BD9828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82BD9828 size=40
    let mut pc: u32 = 0x82BD9828;
    'dispatch: loop {
        match pc {
            0x82BD9828 => {
    //   block [0x82BD9828..0x82BD9850)
	// 82BD9828: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82BD982C: 552917BE  srwi r9, r9, 0x1e
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82BD9830: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82BD9834: 5549E8FA  rlwinm r9, r10, 0x1d, 3, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82BD9838: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82BD983C: 7D6B5030  slw r11, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82BD9840: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82BD9844: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82BD9848: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82BD984C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


