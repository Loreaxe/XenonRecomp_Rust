pub fn sub_8224E418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E418 size=28
    let mut pc: u32 = 0x8224E418;
    'dispatch: loop {
        match pc {
            0x8224E418 => {
    //   block [0x8224E418..0x8224E434)
	// 8224E418: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8224E41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8224E420: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 8224E424: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E428: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8224E42C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8224E430: 4800006C  b 0x8224e49c
	sub_8224E48C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E434 size=8
    let mut pc: u32 = 0x8224E434;
    'dispatch: loop {
        match pc {
            0x8224E434 => {
    //   block [0x8224E434..0x8224E43C)
	// 8224E434: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8224E438: 4BFFFF80  b 0x8224e3b8
	sub_8224E3B4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E43C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E43C size=8
    let mut pc: u32 = 0x8224E43C;
    'dispatch: loop {
        match pc {
            0x8224E43C => {
    //   block [0x8224E43C..0x8224E444)
	// 8224E43C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E440: 4BFFFF08  b 0x8224e348
	sub_8224E344(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E444(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E444 size=16
    let mut pc: u32 = 0x8224E444;
    'dispatch: loop {
        match pc {
            0x8224E444 => {
    //   block [0x8224E444..0x8224E454)
	// 8224E444: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 8224E448: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8224E44C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8224E450: 4BFFFF6C  b 0x8224e3bc
	sub_8224E3B4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E454 size=8
    let mut pc: u32 = 0x8224E454;
    'dispatch: loop {
        match pc {
            0x8224E454 => {
    //   block [0x8224E454..0x8224E45C)
	// 8224E454: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8224E458: 4BFFFF90  b 0x8224e3e8
	sub_8224E3E4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E45C size=8
    let mut pc: u32 = 0x8224E45C;
    'dispatch: loop {
        match pc {
            0x8224E45C => {
    //   block [0x8224E45C..0x8224E464)
	// 8224E45C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E460: 4BFFFF58  b 0x8224e3b8
	sub_8224E3B4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E464(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E464 size=8
    let mut pc: u32 = 0x8224E464;
    'dispatch: loop {
        match pc {
            0x8224E464 => {
    //   block [0x8224E464..0x8224E46C)
	// 8224E464: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E468: 4BFFFF80  b 0x8224e3e8
	sub_8224E3E4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E46C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E46C size=32
    let mut pc: u32 = 0x8224E46C;
    'dispatch: loop {
        match pc {
            0x8224E46C => {
    //   block [0x8224E46C..0x8224E48C)
	// 8224E46C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E474: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E478: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8224E47C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8224E480: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8224E484: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8224E488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E48C size=28
    let mut pc: u32 = 0x8224E48C;
    'dispatch: loop {
        match pc {
            0x8224E48C => {
    //   block [0x8224E48C..0x8224E4A8)
	// 8224E48C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E490: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224E494: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8224E498: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8224E49C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224E4A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224E4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E4A8 size=56
    let mut pc: u32 = 0x8224E4A8;
    'dispatch: loop {
        match pc {
            0x8224E4A8 => {
    //   block [0x8224E4A8..0x8224E4E0)
	// 8224E4A8: 3944FFFE  addi r10, r4, -2
	ctx.r[10].s64 = ctx.r[4].s64 + -2;
	// 8224E4AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8224E4B0: 2B0A0024  cmplwi cr6, r10, 0x24
	ctx.cr[6].compare_u32(ctx.r[10].u32, 36 as u32, &mut ctx.xer);
	// 8224E4B4: 41990144  bgt cr6, 0x8224e5f8
	if ctx.cr[6].gt {
		sub_8224E5F8(ctx, base);
		return;
	}
	// 8224E4B8: 3D808204  lis r12, -0x7dfc
	ctx.r[12].s64 = -2113667072;
	// 8224E4BC: 398C27F0  addi r12, r12, 0x27f0
	ctx.r[12].s64 = ctx.r[12].s64 + 10224;
	// 8224E4C0: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8224E4C4: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8224E4C8: 3D808225  lis r12, -0x7ddb
	ctx.r[12].s64 = -2111504384;
	// 8224E4CC: 398CE4E0  addi r12, r12, -0x1b20
	ctx.r[12].s64 = ctx.r[12].s64 + -6944;
	// 8224E4D0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 8224E4D4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 8224E4D8: 60000000  nop
	// 8224E4DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E4E0 size=20
    let mut pc: u32 = 0x8224E4E0;
    'dispatch: loop {
        match pc {
            0x8224E4E0 => {
    //   block [0x8224E4E0..0x8224E4F4)
	// 8224E4E0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8224E4E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E4E8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E4EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8224E4F0: 48000114  b 0x8224e604
	sub_8224E5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E4F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E4F4 size=24
    let mut pc: u32 = 0x8224E4F4;
    'dispatch: loop {
        match pc {
            0x8224E4F4 => {
    //   block [0x8224E4F4..0x8224E50C)
	// 8224E4F4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8224E4F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8224E4FC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8224E500: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8224E504: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8224E508: 48000100  b 0x8224e608
	sub_8224E5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E50C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E50C size=32
    let mut pc: u32 = 0x8224E50C;
    'dispatch: loop {
        match pc {
            0x8224E50C => {
    //   block [0x8224E50C..0x8224E52C)
	// 8224E50C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8224E510: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8224E514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8224E518: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8224E51C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8224E520: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8224E524: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8224E528: 480000E4  b 0x8224e60c
	sub_8224E5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E52C size=36
    let mut pc: u32 = 0x8224E52C;
    'dispatch: loop {
        match pc {
            0x8224E52C => {
    //   block [0x8224E52C..0x8224E550)
	// 8224E52C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8224E530: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8224E534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8224E538: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8224E53C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224E540: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8224E544: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8224E548: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8224E54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E550 size=28
    let mut pc: u32 = 0x8224E550;
    'dispatch: loop {
        match pc {
            0x8224E550 => {
    //   block [0x8224E550..0x8224E56C)
	// 8224E550: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8224E554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E558: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E55C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8224E560: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8224E564: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8224E568: 480000A4  b 0x8224e60c
	sub_8224E5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E56C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E56C size=12
    let mut pc: u32 = 0x8224E56C;
    'dispatch: loop {
        match pc {
            0x8224E56C => {
    //   block [0x8224E56C..0x8224E578)
	// 8224E56C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8224E570: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8224E574: 4800008C  b 0x8224e600
	sub_8224E5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E578 size=8
    let mut pc: u32 = 0x8224E578;
    'dispatch: loop {
        match pc {
            0x8224E578 => {
    //   block [0x8224E578..0x8224E580)
	// 8224E578: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8224E57C: 4BFFFF68  b 0x8224e4e4
	sub_8224E4E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E580 size=28
    let mut pc: u32 = 0x8224E580;
    'dispatch: loop {
        match pc {
            0x8224E580 => {
    //   block [0x8224E580..0x8224E59C)
	// 8224E580: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8224E584: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8224E588: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E58C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8224E590: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8224E594: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8224E598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E59C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E59C size=12
    let mut pc: u32 = 0x8224E59C;
    'dispatch: loop {
        match pc {
            0x8224E59C => {
    //   block [0x8224E59C..0x8224E5A8)
	// 8224E59C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8224E5A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8224E5A4: 4BFFFF58  b 0x8224e4fc
	sub_8224E4F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5A8 size=12
    let mut pc: u32 = 0x8224E5A8;
    'dispatch: loop {
        match pc {
            0x8224E5A8 => {
    //   block [0x8224E5A8..0x8224E5B4)
	// 8224E5A8: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 8224E5AC: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8224E5B0: 4BFFFF84  b 0x8224e534
	sub_8224E52C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5B4 size=28
    let mut pc: u32 = 0x8224E5B4;
    'dispatch: loop {
        match pc {
            0x8224E5B4 => {
    //   block [0x8224E5B4..0x8224E5D0)
	// 8224E5B4: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8224E5B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8224E5BC: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 8224E5C0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E5C4: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8224E5C8: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8224E5CC: 4800003C  b 0x8224e608
	sub_8224E5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5D0 size=8
    let mut pc: u32 = 0x8224E5D0;
    'dispatch: loop {
        match pc {
            0x8224E5D0 => {
    //   block [0x8224E5D0..0x8224E5D8)
	// 8224E5D0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8224E5D4: 4BFFFF80  b 0x8224e554
	sub_8224E550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5D8 size=8
    let mut pc: u32 = 0x8224E5D8;
    'dispatch: loop {
        match pc {
            0x8224E5D8 => {
    //   block [0x8224E5D8..0x8224E5E0)
	// 8224E5D8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E5DC: 4BFFFF08  b 0x8224e4e4
	sub_8224E4E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5E0 size=8
    let mut pc: u32 = 0x8224E5E0;
    'dispatch: loop {
        match pc {
            0x8224E5E0 => {
    //   block [0x8224E5E0..0x8224E5E8)
	// 8224E5E0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8224E5E4: 4BFFFFA0  b 0x8224e584
	sub_8224E580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5E8 size=8
    let mut pc: u32 = 0x8224E5E8;
    'dispatch: loop {
        match pc {
            0x8224E5E8 => {
    //   block [0x8224E5E8..0x8224E5F0)
	// 8224E5E8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E5EC: 4BFFFF68  b 0x8224e554
	sub_8224E550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5F0 size=8
    let mut pc: u32 = 0x8224E5F0;
    'dispatch: loop {
        match pc {
            0x8224E5F0 => {
    //   block [0x8224E5F0..0x8224E5F8)
	// 8224E5F0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8224E5F4: 4BFFFF90  b 0x8224e584
	sub_8224E580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E5F8 size=28
    let mut pc: u32 = 0x8224E5F8;
    'dispatch: loop {
        match pc {
            0x8224E5F8 => {
    //   block [0x8224E5F8..0x8224E614)
	// 8224E5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E5FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224E600: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8224E604: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8224E608: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224E60C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224E610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8224E618 size=372
    let mut pc: u32 = 0x8224E618;
    'dispatch: loop {
        match pc {
            0x8224E618 => {
    //   block [0x8224E618..0x8224E78C)
	// 8224E618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224E61C: 4BE40479  bl 0x8208ea94
	ctx.lr = 0x8224E620;
	sub_8208EA60(ctx, base);
	// 8224E620: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224E624: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8224E628: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8224E62C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8224E630: 4BFFFBB1  bl 0x8224e1e0
	ctx.lr = 0x8224E634;
	sub_8224E1E0(ctx, base);
	// 8224E634: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8224E638: 4082000C  bne 0x8224e644
	if !ctx.cr[0].eq {
	pc = 0x8224E644; continue 'dispatch;
	}
	// 8224E63C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224E640: 48000144  b 0x8224e784
	pc = 0x8224E784; continue 'dispatch;
	// 8224E644: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8224E648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8224E64C: 4BFFFCC5  bl 0x8224e310
	ctx.lr = 0x8224E650;
	sub_8224E310(ctx, base);
	// 8224E650: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 8224E654: 4099006C  ble cr6, 0x8224e6c0
	if !ctx.cr[6].gt {
	pc = 0x8224E6C0; continue 'dispatch;
	}
	// 8224E658: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8224E65C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8224E660: 409900EC  ble cr6, 0x8224e74c
	if !ctx.cr[6].gt {
	pc = 0x8224E74C; continue 'dispatch;
	}
	// 8224E664: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8224E668: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8224E66C: 7FC75850  subf r30, r7, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 8224E670: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8224E674: 7D5E402E  lwzx r10, r30, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8224E678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224E67C: 80A80000  lwz r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224E680: 7D491E71  srawi. r9, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8224E684: 40810028  ble 0x8224e6ac
	if !ctx.cr[0].gt {
	pc = 0x8224E6AC; continue 'dispatch;
	}
	// 8224E688: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8224E68C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E690: 7CFF3A14  add r7, r31, r7
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 8224E694: 7CBB5630  sraw r27, r5, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[5].s32 >> tmp.u32) as i64;
	// 8224E698: 7F6759AE  stbx r27, r7, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u8) };
	// 8224E69C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224E6A0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8224E6A4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8224E6A8: 4198FFEC  blt cr6, 0x8224e694
	if ctx.cr[6].lt {
	pc = 0x8224E694; continue 'dispatch;
	}
	// 8224E6AC: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8224E6B0: 7FE9FA14  add r31, r9, r31
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 8224E6B4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8224E6B8: 4082FFBC  bne 0x8224e674
	if !ctx.cr[0].eq {
	pc = 0x8224E674; continue 'dispatch;
	}
	// 8224E6BC: 48000090  b 0x8224e74c
	pc = 0x8224E74C; continue 'dispatch;
	// 8224E6C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8224E6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8224E6C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8224E6CC: 40990054  ble cr6, 0x8224e720
	if !ctx.cr[6].gt {
	pc = 0x8224E720; continue 'dispatch;
	}
	// 8224E6D0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8224E6D4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8224E6D8: 7C875050  subf r4, r7, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 8224E6DC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8224E6E0: 7D2B202E  lwzx r9, r11, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8224E6E4: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8224E6E8: 40980014  bge cr6, 0x8224e6fc
	if !ctx.cr[6].lt {
	pc = 0x8224E6FC; continue 'dispatch;
	}
	// 8224E6EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8224E6F0: 7CE74830  slw r7, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8224E6F4: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8224E6F8: 48000008  b 0x8224e700
	pc = 0x8224E700; continue 'dispatch;
	// 8224E6FC: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8224E700: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224E704: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8224E708: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8224E70C: 7FE73838  and r7, r31, r7
	ctx.r[7].u64 = ctx.r[31].u64 & ctx.r[7].u64;
	// 8224E710: 7CE74030  slw r7, r7, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8224E714: 7CE52B78  or r5, r7, r5
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 8224E718: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8224E71C: 4082FFC4  bne 0x8224e6e0
	if !ctx.cr[0].eq {
	pc = 0x8224E6E0; continue 'dispatch;
	}
	// 8224E720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224E724: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8224E728: 40990024  ble cr6, 0x8224e74c
	if !ctx.cr[6].gt {
	pc = 0x8224E74C; continue 'dispatch;
	}
	// 8224E72C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224E730: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8224E734: 7CA95630  sraw r9, r5, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[5].s32 >> tmp.u32) as i64;
	// 8224E738: 7D2B41AE  stbx r9, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u8) };
	// 8224E73C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224E740: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8224E744: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8224E748: 4198FFE8  blt cr6, 0x8224e730
	if ctx.cr[6].lt {
	pc = 0x8224E730; continue 'dispatch;
	}
	// 8224E74C: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 8224E750: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224E754: 392B2FB0  addi r9, r11, 0x2fb0
	ctx.r[9].s64 = ctx.r[11].s64 + 12208;
	// 8224E758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224E75C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8224E760: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8224E764: 40990020  ble cr6, 0x8224e784
	if !ctx.cr[6].gt {
	pc = 0x8224E784; continue 'dispatch;
	}
	// 8224E768: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8224E76C: 7D485A78  xor r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 8224E770: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8224E774: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224E778: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8224E77C: 7D28E1AE  stbx r9, r8, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[28].u32), ctx.r[9].u8) };
	// 8224E780: 4198FFE8  blt cr6, 0x8224e768
	if ctx.cr[6].lt {
	pc = 0x8224E768; continue 'dispatch;
	}
	// 8224E784: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8224E788: 4BE4035C  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224E790 size=56
    let mut pc: u32 = 0x8224E790;
    'dispatch: loop {
        match pc {
            0x8224E790 => {
    //   block [0x8224E790..0x8224E7C8)
	// 8224E790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224E794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224E798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224E79C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8224E7A0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8224E7A4: 4BFFFAA5  bl 0x8224e248
	ctx.lr = 0x8224E7A8;
	sub_8224E248(ctx, base);
	// 8224E7A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8224E7AC: 4182000C  beq 0x8224e7b8
	if ctx.cr[0].eq {
	pc = 0x8224E7B8; continue 'dispatch;
	}
	// 8224E7B0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8224E7B4: 4BFFFE65  bl 0x8224e618
	ctx.lr = 0x8224E7B8;
	sub_8224E618(ctx, base);
	// 8224E7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8224E7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224E7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224E7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224E7C8 size=104
    let mut pc: u32 = 0x8224E7C8;
    'dispatch: loop {
        match pc {
            0x8224E7C8 => {
    //   block [0x8224E7C8..0x8224E830)
	// 8224E7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224E7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224E7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224E7D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8224E7D8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8224E7DC: 409A0044  bne cr6, 0x8224e820
	if !ctx.cr[6].eq {
	pc = 0x8224E820; continue 'dispatch;
	}
	// 8224E7E0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8224E7E4: 4BFFFACD  bl 0x8224e2b0
	ctx.lr = 0x8224E7E8;
	sub_8224E2B0(ctx, base);
	// 8224E7E8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8224E7EC: 41980018  blt cr6, 0x8224e804
	if ctx.cr[6].lt {
	pc = 0x8224E804; continue 'dispatch;
	}
	// 8224E7F0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224E7F4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224E7F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8224E7FC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224E800: 48000020  b 0x8224e820
	pc = 0x8224E820; continue 'dispatch;
	// 8224E804: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224E808: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224E80C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224E810: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 8224E814: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8224E818: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 8224E81C: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8224E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8224E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8224E830 size=64
    let mut pc: u32 = 0x8224E830;
    'dispatch: loop {
        match pc {
            0x8224E830 => {
    //   block [0x8224E830..0x8224E870)
	// 8224E830: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 8224E834: 3D404E80  lis r10, 0x4e80
	ctx.r[10].s64 = 1317011456;
	// 8224E838: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8224E83C: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224E840: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224E844: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224E848: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8224E84C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8224E850: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8224E854: 55690050  rlwinm r9, r11, 0, 1, 8
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8224E858: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8224E85C: 40990020  ble cr6, 0x8224e87c
	if !ctx.cr[6].gt {
		sub_8224E87C(ctx, base);
		return;
	}
	// 8224E860: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224E864: 4182000C  beq 0x8224e870
	if ctx.cr[0].eq {
		sub_8224E870(ctx, base);
		return;
	}
	// 8224E868: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8224E86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E870 size=12
    let mut pc: u32 = 0x8224E870;
    'dispatch: loop {
        match pc {
            0x8224E870 => {
    //   block [0x8224E870..0x8224E87C)
	// 8224E870: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 8224E874: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8224E878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E87C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224E87C size=16
    let mut pc: u32 = 0x8224E87C;
    'dispatch: loop {
        match pc {
            0x8224E87C => {
    //   block [0x8224E87C..0x8224E88C)
	// 8224E87C: FC00001E  fctiwz f0, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224E880: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8224E884: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8224E888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8224E890 size=28
    let mut pc: u32 = 0x8224E890;
    'dispatch: loop {
        match pc {
            0x8224E890 => {
    //   block [0x8224E890..0x8224E8AC)
	// 8224E890: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224E894: C00B0B14  lfs f0, 0xb14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2836 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224E898: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8224E89C: 41980010  blt cr6, 0x8224e8ac
	if ctx.cr[6].lt {
		sub_8224E8AC(ctx, base);
		return;
	}
	// 8224E8A0: 3C6007FF  lis r3, 0x7ff
	ctx.r[3].s64 = 134152192;
	// 8224E8A4: 6063FFFE  ori r3, r3, 0xfffe
	ctx.r[3].u64 = ctx.r[3].u64 | 65534;
	// 8224E8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E8AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8224E8AC size=24
    let mut pc: u32 = 0x8224E8AC;
    'dispatch: loop {
        match pc {
            0x8224E8AC => {
    //   block [0x8224E8AC..0x8224E8C4)
	// 8224E8AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224E8B0: C00B0B28  lfs f0, 0xb28(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2856 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224E8B4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8224E8B8: 4199000C  bgt cr6, 0x8224e8c4
	if ctx.cr[6].gt {
		sub_8224E8C4(ctx, base);
		return;
	}
	// 8224E8BC: 3C600800  lis r3, 0x800
	ctx.r[3].s64 = 134217728;
	// 8224E8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E8C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8224E8C4 size=104
    let mut pc: u32 = 0x8224E8C4;
    'dispatch: loop {
        match pc {
            0x8224E8C4 => {
    //   block [0x8224E8C4..0x8224E92C)
	// 8224E8C4: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8224E8C8: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8224E8CC: 7D6BBE70  srawi r11, r11, 0x17
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 23) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 23) as i64;
	// 8224E8D0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8224E8D4: 216B0081  subfic r11, r11, 0x81
	ctx.xer.ca = ctx.r[11].u32 <= 129 as u32;
	ctx.r[11].s64 = (129 as i64) - ctx.r[11].s64;
	// 8224E8D8: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 8224E8DC: 40990008  ble cr6, 0x8224e8e4
	if !ctx.cr[6].gt {
	pc = 0x8224E8E4; continue 'dispatch;
	}
	// 8224E8E0: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 8224E8E4: 394B0019  addi r10, r11, 0x19
	ctx.r[10].s64 = ctx.r[11].s64 + 25;
	// 8224E8E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8224E8EC: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 8224E8F0: 556AE006  slwi r10, r11, 0x1c
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(28);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224E8F4: 7D2B4036  sld r11, r9, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[9].u64) << ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 8224E8F8: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8224E8FC: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224E900: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224E904: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224E908: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8224E90C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224E910: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8224E914: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8224E918: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224E91C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8224E920: 556B013E  clrlwi r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 8224E924: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224E928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8224E930 size=1248
    let mut pc: u32 = 0x8224E930;
    'dispatch: loop {
        match pc {
            0x8224E930 => {
    //   block [0x8224E930..0x8224EE10)
	// 8224E930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224E934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224E938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8224E93C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224E940: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8224E944: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224E948: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8224E94C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8224E950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8224E954: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 8224E958: 41990048  bgt cr6, 0x8224e9a0
	if ctx.cr[6].gt {
	pc = 0x8224E9A0; continue 'dispatch;
	}
	// 8224E95C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224E960: 409A000C  bne cr6, 0x8224e96c
	if !ctx.cr[6].eq {
	pc = 0x8224E96C; continue 'dispatch;
	}
	// 8224E964: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224E968: 4800048C  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224E96C: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 8224E970: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8224E974: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8224E978: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8224E97C: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 8224E980: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224E984: C80A2888  lfd f0, 0x2888(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(10376 as u32) ) };
	// 8224E988: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8224E98C: C9AB0718  lfd f13, 0x718(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 8224E990: FC0C683C  fnmsub f0, f12, f0, f13
	ctx.f[0].f64 = -(ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64);
	// 8224E994: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8224E998: 4098045C  bge cr6, 0x8224edf4
	if !ctx.cr[6].lt {
	pc = 0x8224EDF4; continue 'dispatch;
	}
	// 8224E99C: 4BFFFFC8  b 0x8224e964
	pc = 0x8224E964; continue 'dispatch;
	// 8224E9A0: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 8224E9A4: 41980194  blt cr6, 0x8224eb38
	if ctx.cr[6].lt {
	pc = 0x8224EB38; continue 'dispatch;
	}
	// 8224E9A8: 419A035C  beq cr6, 0x8224ed04
	if ctx.cr[6].eq {
	pc = 0x8224ED04; continue 'dispatch;
	}
	// 8224E9AC: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 8224E9B0: 419802EC  blt cr6, 0x8224ec9c
	if ctx.cr[6].lt {
	pc = 0x8224EC9C; continue 'dispatch;
	}
	// 8224E9B4: 419A0230  beq cr6, 0x8224ebe4
	if ctx.cr[6].eq {
	pc = 0x8224EBE4; continue 'dispatch;
	}
	// 8224E9B8: 2B060005  cmplwi cr6, r6, 5
	ctx.cr[6].compare_u32(ctx.r[6].u32, 5 as u32, &mut ctx.xer);
	// 8224E9BC: 4198015C  blt cr6, 0x8224eb18
	if ctx.cr[6].lt {
	pc = 0x8224EB18; continue 'dispatch;
	}
	// 8224E9C0: 419A010C  beq cr6, 0x8224eacc
	if ctx.cr[6].eq {
	pc = 0x8224EACC; continue 'dispatch;
	}
	// 8224E9C4: 2B060007  cmplwi cr6, r6, 7
	ctx.cr[6].compare_u32(ctx.r[6].u32, 7 as u32, &mut ctx.xer);
	// 8224E9C8: 409AFF9C  bne cr6, 0x8224e964
	if !ctx.cr[6].eq {
	pc = 0x8224E964; continue 'dispatch;
	}
	// 8224E9CC: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8224E9D0: 2F1F0020  cmpwi cr6, r31, 0x20
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32, &mut ctx.xer);
	// 8224E9D4: 409A000C  bne cr6, 0x8224e9e0
	if !ctx.cr[6].eq {
	pc = 0x8224E9E0; continue 'dispatch;
	}
	// 8224E9D8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8224E9DC: 48000418  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224E9E0: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 8224E9E4: 409A0068  bne cr6, 0x8224ea4c
	if !ctx.cr[6].eq {
	pc = 0x8224EA4C; continue 'dispatch;
	}
	// 8224E9E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8224E9EC: 57CB2834  slwi r11, r30, 5
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224E9F0: C80A2870  lfd f0, 0x2870(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(10352 as u32) ) };
	// 8224E9F4: FC1F0032  fmul f0, f31, f0
	ctx.f[0].f64 = ctx.f[31].f64 * ctx.f[0].f64;
	// 8224E9F8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224E9FC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8224EA00: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8224EA04: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224EA08: 556A0047  rlwinm. r10, r11, 0, 1, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8224EA0C: 41820028  beq 0x8224ea34
	if ctx.cr[0].eq {
	pc = 0x8224EA34; continue 'dispatch;
	}
	// 8224EA10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224EA14: C00B06F8  lfs f0, 0x6f8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224EA18: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8224EA1C: 40980010  bge cr6, 0x8224ea2c
	if !ctx.cr[6].lt {
	pc = 0x8224EA2C; continue 'dispatch;
	}
	// 8224EA20: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 8224EA24: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8224EA28: 480003CC  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EA2C: 38607FFF  li r3, 0x7fff
	ctx.r[3].s64 = 32767;
	// 8224EA30: 480003C4  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EA34: 7D6B6E71  srawi. r11, r11, 0xd
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 13) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 13) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224EA38: 4080000C  bge 0x8224ea44
	if !ctx.cr[0].lt {
	pc = 0x8224EA44; continue 'dispatch;
	}
	// 8224EA3C: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 8224EA40: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8224EA44: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8224EA48: 480003AC  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EA4C: 2F1F000A  cmpwi cr6, r31, 0xa
	ctx.cr[6].compare_i32(ctx.r[31].s32, 10, &mut ctx.xer);
	// 8224EA50: 4198FF14  blt cr6, 0x8224e964
	if ctx.cr[6].lt {
	pc = 0x8224E964; continue 'dispatch;
	}
	// 8224EA54: 3880FF84  li r4, -0x7c
	ctx.r[4].s64 = -124;
	// 8224EA58: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8224EA5C: 4BE4199D  bl 0x820903f8
	ctx.lr = 0x8224EA60;
	sub_820903F8(ctx, base);
	// 8224EA60: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8224EA64: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8224EA68: 217F0012  subfic r11, r31, 0x12
	ctx.xer.ca = ctx.r[31].u32 <= 18 as u32;
	ctx.r[11].s64 = (18 as i64) - ctx.r[31].s64;
	// 8224EA6C: 7FCB5830  slw r11, r30, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[30].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8224EA70: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8224EA74: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224EA78: 556A004B  rlwinm. r10, r11, 0, 1, 5
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8224EA7C: 41820034  beq 0x8224eab0
	if ctx.cr[0].eq {
	pc = 0x8224EAB0; continue 'dispatch;
	}
	// 8224EA80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224EA84: C00B06F8  lfs f0, 0x6f8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224EA88: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8224EA8C: 40980010  bge cr6, 0x8224ea9c
	if !ctx.cr[6].lt {
	pc = 0x8224EA9C; continue 'dispatch;
	}
	// 8224EA90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8224EA94: 7D6BF830  slw r11, r11, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 8224EA98: 48000010  b 0x8224eaa8
	pc = 0x8224EAA8; continue 'dispatch;
	// 8224EA9C: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8224EAA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8224EAA4: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8224EAA8: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 8224EAAC: 48000348  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EAB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8224EAB4: 213F001A  subfic r9, r31, 0x1a
	ctx.xer.ca = ctx.r[31].u32 <= 26 as u32;
	ctx.r[9].s64 = (26 as i64) - ctx.r[31].s64;
	// 8224EAB8: 7D4AF830  slw r10, r10, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 8224EABC: 7D6B4E30  sraw r11, r11, r9
	tmp.u32 = ctx.r[9].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 8224EAC0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8224EAC4: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8224EAC8: 4800032C  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EACC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224EAD0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8224EAD4: C00B1EF4  lfs f0, 0x1ef4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7924 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224EAD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224EADC: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8224EAE0: C1AA06F8  lfs f13, 0x6f8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1784 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8224EAE4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8224EAE8: C9AB1070  lfd f13, 0x1070(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4208 as u32) ) };
	// 8224EAEC: 40990018  ble cr6, 0x8224eb04
	if !ctx.cr[6].gt {
	pc = 0x8224EB04; continue 'dispatch;
	}
	// 8224EAF0: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 8224EAF4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224EAF8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8224EAFC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8224EB00: 480002F4  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EB04: FC006828  fsub f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 8224EB08: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224EB0C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8224EB10: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8224EB14: 480002E0  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EB18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8224EB1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8224EB20: C80B2880  lfd f0, 0x2880(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(10368 as u32) ) };
	// 8224EB24: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8224EB28: 41990090  bgt cr6, 0x8224ebb8
	if ctx.cr[6].gt {
	pc = 0x8224EBB8; continue 'dispatch;
	}
	// 8224EB2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224EB30: C00B0FF0  lfs f0, 0xff0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4080 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224EB34: EFFF0032  fmuls f31, f31, f0
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8224EB38: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 8224EB3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8224EB40: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224EB44: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 8224EB48: 7D4BF830  slw r11, r10, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 8224EB4C: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 8224EB50: C8092888  lfd f0, 0x2888(r9)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(10376 as u32) ) };
	// 8224EB54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8224EB58: 7D0B07B4  extsw r11, r8
	ctx.r[11].s64 = ctx.r[8].s32 as i64;
	// 8224EB5C: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224EB60: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8224EB64: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224EB68: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224EB6C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8224EB70: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 8224EB74: FDA06018  frsp f13, f12
	ctx.f[13].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8224EB78: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224EB7C: EC2D07FA  fmadds f1, f13, f31, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 8224EB80: 4BFFFCB1  bl 0x8224e830
	ctx.lr = 0x8224EB84;
	sub_8224E830(ctx, base);
	// 8224EB84: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8224EB88: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8224EB8C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224EB90: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224EB94: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224EB98: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224EB9C: C18A06F8  lfs f12, 0x6f8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1784 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8224EBA0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8224EBA4: 4099FDC0  ble cr6, 0x8224e964
	if !ctx.cr[6].gt {
	pc = 0x8224E964; continue 'dispatch;
	}
	// 8224EBA8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8224EBAC: 4198023C  blt cr6, 0x8224ede8
	if ctx.cr[6].lt {
	pc = 0x8224EDE8; continue 'dispatch;
	}
	// 8224EBB0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8224EBB4: 48000240  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EBB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224EBBC: C84B0FE8  lfd f2, 0xfe8(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4072 as u32) ) };
	// 8224EBC0: 4BE41269  bl 0x8208fe28
	ctx.lr = 0x8224EBC4;
	sub_8208FE28(ctx, base);
	// 8224EBC4: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8224EBC8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8224EBCC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8224EBD0: C80B2878  lfd f0, 0x2878(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(10360 as u32) ) };
	// 8224EBD4: C9AA2868  lfd f13, 0x2868(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(10344 as u32) ) };
	// 8224EBD8: FC0C6838  fmsub f0, f12, f0, f13
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64;
	// 8224EBDC: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224EBE0: 4BFFFF58  b 0x8224eb38
	pc = 0x8224EB38; continue 'dispatch;
	// 8224EBE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8224EBE8: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8224EBEC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8224EBF0: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8224EBF4: C00A06F8  lfs f0, 0x6f8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224EBF8: 7FCA4670  srawi r10, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 8224EBFC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8224EC00: 41980050  blt cr6, 0x8224ec50
	if ctx.cr[6].lt {
	pc = 0x8224EC50; continue 'dispatch;
	}
	// 8224EC04: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 8224EC08: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8224EC0C: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 8224EC10: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8224EC14: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8224EC18: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8224EC1C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8224EC20: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8224EC24: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224EC28: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8224EC2C: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224EC30: FC006818  frsp f0, f13
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8224EC34: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8224EC38: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8224EC3C: 409801B8  bge cr6, 0x8224edf4
	if !ctx.cr[6].lt {
	pc = 0x8224EDF4; continue 'dispatch;
	}
	// 8224EC40: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224EC44: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8224EC48: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8224EC4C: 480001A8  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EC50: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 8224EC54: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8224EC58: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 8224EC5C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8224EC60: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8224EC64: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8224EC68: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224EC6C: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224EC70: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224EC74: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8224EC78: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224EC7C: FC006818  frsp f0, f13
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8224EC80: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 8224EC84: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8224EC88: 4099016C  ble cr6, 0x8224edf4
	if !ctx.cr[6].gt {
	pc = 0x8224EDF4; continue 'dispatch;
	}
	// 8224EC8C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224EC90: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8224EC94: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8224EC98: 4800015C  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EC9C: 7FCB4670  srawi r11, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 8224ECA0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8224ECA4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8224ECA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8224ECAC: 7D6807B4  extsw r8, r11
	ctx.r[8].s64 = ctx.r[11].s32 as i64;
	// 8224ECB0: 7D2BF830  slw r11, r9, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 8224ECB4: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 8224ECB8: C1AA06F8  lfs f13, 0x6f8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1784 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8224ECBC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 8224ECC0: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224ECC4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224ECC8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224ECCC: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8224ECD0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8224ECD4: 4099FC90  ble cr6, 0x8224e964
	if !ctx.cr[6].gt {
	pc = 0x8224E964; continue 'dispatch;
	}
	// 8224ECD8: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8224ECDC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224ECE0: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224ECE4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8224ECE8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8224ECEC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8224ECF0: 40980104  bge cr6, 0x8224edf4
	if !ctx.cr[6].lt {
	pc = 0x8224EDF4; continue 'dispatch;
	}
	// 8224ECF4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224ECF8: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8224ECFC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8224ED00: 480000F4  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224ED04: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8224ED08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8224ED0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8224ED10: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8224ED14: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 8224ED18: C00A06F8  lfs f0, 0x6f8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8224ED1C: 57CB083E  rotlwi r11, r30, 1
	ctx.r[11].u64 = ((ctx.r[30].u32).rotate_left(1)) as u64;
	// 8224ED20: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8224ED24: 7D5E1BD6  divw r10, r30, r3
	ctx.r[10].s32 = ctx.r[30].s32 / ctx.r[3].s32;
	// 8224ED28: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224ED2C: 7D4A4670  srawi r10, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 8224ED30: 7C6B5878  andc r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & !ctx.r[11].u64;
	// 8224ED34: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8224ED38: 0CC30000  twi 6, r3, 0
	// 8224ED3C: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8224ED40: 0CABFFFF  twi 5, r11, -1
	// 8224ED44: 41980050  blt cr6, 0x8224ed94
	if ctx.cr[6].lt {
	pc = 0x8224ED94; continue 'dispatch;
	}
	// 8224ED48: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8224ED4C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8224ED50: C1A906E0  lfs f13, 0x6e0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(1760 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8224ED54: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224ED58: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224ED5C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224ED60: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8224ED64: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8224ED68: 4098008C  bge cr6, 0x8224edf4
	if !ctx.cr[6].lt {
	pc = 0x8224EDF4; continue 'dispatch;
	}
	// 8224ED6C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8224ED70: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224ED74: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224ED78: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8224ED7C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8224ED80: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8224ED84: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224ED88: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8224ED8C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8224ED90: 48000064  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224ED94: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8224ED98: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8224ED9C: C1A92050  lfs f13, 0x2050(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8272 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8224EDA0: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224EDA4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8224EDA8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8224EDAC: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 8224EDB0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8224EDB4: 4199000C  bgt cr6, 0x8224edc0
	if ctx.cr[6].gt {
	pc = 0x8224EDC0; continue 'dispatch;
	}
	// 8224EDB8: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 8224EDBC: 48000038  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EDC0: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8224EDC4: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8224EDC8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8224EDCC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8224EDD0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8224EDD4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8224EDD8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224EDDC: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8224EDE0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8224EDE4: 48000010  b 0x8224edf4
	pc = 0x8224EDF4; continue 'dispatch;
	// 8224EDE8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8224EDEC: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8224EDF0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8224EDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224EDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224EDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224EE00: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8224EE04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8224EE08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224EE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EE10 size=24
    let mut pc: u32 = 0x8224EE10;
    'dispatch: loop {
        match pc {
            0x8224EE10 => {
    //   block [0x8224EE10..0x8224EE28)
	// 8224EE10: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EE14: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8224EE18: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8224EE1C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8224EE20: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8224EE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EE28 size=44
    let mut pc: u32 = 0x8224EE28;
    'dispatch: loop {
        match pc {
            0x8224EE28 => {
    //   block [0x8224EE28..0x8224EE54)
	// 8224EE28: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EE2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8224EE30: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224EE34: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224EE38: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224EE3C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EE40: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224EE44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224EE48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224EE4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224EE50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EE58 size=32
    let mut pc: u32 = 0x8224EE58;
    'dispatch: loop {
        match pc {
            0x8224EE58 => {
    //   block [0x8224EE58..0x8224EE78)
	// 8224EE58: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224EE5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224EE60: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224EE64: 419A0008  beq cr6, 0x8224ee6c
	if ctx.cr[6].eq {
	pc = 0x8224EE6C; continue 'dispatch;
	}
	// 8224EE68: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8224EE6C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8224EE70: 90640004  stw r3, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8224EE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EE78 size=32
    let mut pc: u32 = 0x8224EE78;
    'dispatch: loop {
        match pc {
            0x8224EE78 => {
    //   block [0x8224EE78..0x8224EE98)
	// 8224EE78: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EE7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224EE80: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8224EE84: 419A0008  beq cr6, 0x8224ee8c
	if ctx.cr[6].eq {
	pc = 0x8224EE8C; continue 'dispatch;
	}
	// 8224EE88: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8224EE8C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8224EE90: 90640008  stw r3, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8224EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EE98 size=28
    let mut pc: u32 = 0x8224EE98;
    'dispatch: loop {
        match pc {
            0x8224EE98 => {
    //   block [0x8224EE98..0x8224EEB4)
	// 8224EE98: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224EE9C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EEA0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224EEA4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EEA8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224EEAC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224EEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EEB8 size=52
    let mut pc: u32 = 0x8224EEB8;
    'dispatch: loop {
        match pc {
            0x8224EEB8 => {
    //   block [0x8224EEB8..0x8224EEEC)
	// 8224EEB8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8224EEBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224EEC0: 392A03A8  addi r9, r10, 0x3a8
	ctx.r[9].s64 = ctx.r[10].s64 + 936;
	// 8224EEC4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8224EEC8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8224EECC: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8224EED0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8224EED4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8224EED8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224EEDC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224EEE0: 90630010  stw r3, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8224EEE4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8224EEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EEF0 size=16
    let mut pc: u32 = 0x8224EEF0;
    'dispatch: loop {
        match pc {
            0x8224EEF0 => {
    //   block [0x8224EEF0..0x8224EF00)
	// 8224EEF0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EEF4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8224EEF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224EEFC: 4800000C  b 0x8224ef08
	sub_8224EF00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EF00 size=20
    let mut pc: u32 = 0x8224EF00;
    'dispatch: loop {
        match pc {
            0x8224EF00 => {
    //   block [0x8224EF00..0x8224EF14)
	// 8224EF00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EF04: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8224EF08: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8224EF0C: 409AFFF4  bne cr6, 0x8224ef00
	if !ctx.cr[6].eq {
	pc = 0x8224EF00; continue 'dispatch;
	}
	// 8224EF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EF18 size=36
    let mut pc: u32 = 0x8224EF18;
    'dispatch: loop {
        match pc {
            0x8224EF18 => {
    //   block [0x8224EF18..0x8224EF3C)
	// 8224EF18: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224EF1C: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8224EF20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224EF24: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224EF28: 419A0008  beq cr6, 0x8224ef30
	if ctx.cr[6].eq {
	pc = 0x8224EF30; continue 'dispatch;
	}
	// 8224EF2C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8224EF30: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224EF34: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8224EF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224EF40 size=32
    let mut pc: u32 = 0x8224EF40;
    'dispatch: loop {
        match pc {
            0x8224EF40 => {
    //   block [0x8224EF40..0x8224EF60)
	// 8224EF40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EF44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224EF48: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8224EF4C: 419A0008  beq cr6, 0x8224ef54
	if ctx.cr[6].eq {
	pc = 0x8224EF54; continue 'dispatch;
	}
	// 8224EF50: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8224EF54: 90640004  stw r3, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8224EF58: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8224EF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224EF60 size=84
    let mut pc: u32 = 0x8224EF60;
    'dispatch: loop {
        match pc {
            0x8224EF60 => {
    //   block [0x8224EF60..0x8224EFB4)
	// 8224EF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224EF64: 4BE3FB39  bl 0x8208ea9c
	ctx.lr = 0x8224EF68;
	sub_8208EA60(ctx, base);
	// 8224EF68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224EF6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224EF70: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8224EF74: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EF78: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8224EF7C: 419A0028  beq cr6, 0x8224efa4
	if ctx.cr[6].eq {
	pc = 0x8224EFA4; continue 'dispatch;
	}
	// 8224EF80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224EF84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8224EF88: 83A30008  lwz r29, 8(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EF8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224EF90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224EF94: 4E800421  bctrl
	ctx.lr = 0x8224EF98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224EF98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8224EF9C: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8224EFA0: 409AFFE0  bne cr6, 0x8224ef80
	if !ctx.cr[6].eq {
	pc = 0x8224EF80; continue 'dispatch;
	}
	// 8224EFA4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8224EFA8: 93FF0010  stw r31, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 8224EFAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224EFB0: 4BE3FB3C  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224EFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224EFB8 size=212
    let mut pc: u32 = 0x8224EFB8;
    'dispatch: loop {
        match pc {
            0x8224EFB8 => {
    //   block [0x8224EFB8..0x8224F08C)
	// 8224EFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224EFBC: 4BE3FAB9  bl 0x8208ea74
	ctx.lr = 0x8224EFC0;
	sub_8208EA60(ctx, base);
	// 8224EFC0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224EFC4: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 8224EFC8: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 8224EFCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8224EFD0: 82D50008  lwz r22, 8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224EFD4: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224EFD8: 1D4A0003  mulli r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 * 3;
	// 8224EFDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8224EFE0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224EFE4: 4198FFF4  blt cr6, 0x8224efd8
	if ctx.cr[6].lt {
	pc = 0x8224EFD8; continue 'dispatch;
	}
	// 8224EFE8: 3A800003  li r20, 3
	ctx.r[20].s64 = 3;
	// 8224EFEC: 7FEAA397  divwu. r31, r10, r20
	ctx.r[31].u32 = ctx.r[10].u32 / ctx.r[20].u32;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224EFF0: 41820094  beq 0x8224f084
	if ctx.cr[0].eq {
	pc = 0x8224F084; continue 'dispatch;
	}
	// 8224EFF4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224EFF8: 40980084  bge cr6, 0x8224f07c
	if !ctx.cr[6].lt {
	pc = 0x8224F07C; continue 'dispatch;
	}
	// 8224EFFC: 57F7103A  slwi r23, r31, 2
	ctx.r[23].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 8224F000: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8224F004: 7FB7B214  add r29, r23, r22
	ctx.r[29].u64 = ctx.r[23].u64 + ctx.r[22].u64;
	// 8224F008: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 8224F00C: 831D0000  lwz r24, 0(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F010: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 8224F014: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8224F018: 4198003C  blt cr6, 0x8224f054
	if ctx.cr[6].lt {
	pc = 0x8224F054; continue 'dispatch;
	}
	// 8224F01C: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 8224F020: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 8224F024: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F028: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8224F02C: 7E6903A6  mtctr r19
	ctx.ctr.u64 = ctx.r[19].u64;
	// 8224F030: 4E800421  bctrl
	ctx.lr = 0x8224F034;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F034: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8224F038: 4081001C  ble 0x8224f054
	if !ctx.cr[0].gt {
	pc = 0x8224F054; continue 'dispatch;
	}
	// 8224F03C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F040: 7F7FD851  subf. r27, r31, r27
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[31].s64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8224F044: 7F97E050  subf r28, r23, r28
	ctx.r[28].s64 = ctx.r[28].s64 - ctx.r[23].s64;
	// 8224F048: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8224F04C: 7F37C850  subf r25, r23, r25
	ctx.r[25].s64 = ctx.r[25].s64 - ctx.r[23].s64;
	// 8224F050: 4080FFD4  bge 0x8224f024
	if !ctx.cr[0].lt {
	pc = 0x8224F024; continue 'dispatch;
	}
	// 8224F054: 7D7BFA14  add r11, r27, r31
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 8224F058: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8224F05C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F060: 7D5EFA14  add r10, r30, r31
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8224F064: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8224F068: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8224F06C: 7F0BB12E  stwx r24, r11, r22
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[22].u32), ctx.r[24].u32) };
	// 8224F070: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F074: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F078: 4198FF94  blt cr6, 0x8224f00c
	if ctx.cr[6].lt {
	pc = 0x8224F00C; continue 'dispatch;
	}
	// 8224F07C: 7FFFA397  divwu. r31, r31, r20
	ctx.r[31].u32 = ctx.r[31].u32 / ctx.r[20].u32;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224F080: 4082FF74  bne 0x8224eff4
	if !ctx.cr[0].eq {
	pc = 0x8224EFF4; continue 'dispatch;
	}
	// 8224F084: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8224F088: 4BE3FA3C  b 0x8208eac4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F090 size=12
    let mut pc: u32 = 0x8224F090;
    'dispatch: loop {
        match pc {
            0x8224F090 => {
    //   block [0x8224F090..0x8224F09C)
	// 8224F090: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F094: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F098: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F09C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F09C size=36
    let mut pc: u32 = 0x8224F09C;
    'dispatch: loop {
        match pc {
            0x8224F09C => {
    //   block [0x8224F09C..0x8224F0C0)
	// 8224F09C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224F0A0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F0A4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224F0A8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F0AC: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8224F0B0: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8224F0B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8224F0B8: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 8224F0BC: 4BE40134  b 0x8208f1f0
	sub_8208F1F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F0C0 size=4
    let mut pc: u32 = 0x8224F0C0;
    'dispatch: loop {
        match pc {
            0x8224F0C0 => {
    //   block [0x8224F0C0..0x8224F0C4)
	// 8224F0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F0C8 size=136
    let mut pc: u32 = 0x8224F0C8;
    'dispatch: loop {
        match pc {
            0x8224F0C8 => {
    //   block [0x8224F0C8..0x8224F150)
	// 8224F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224F0D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F0D4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224F0DC: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F0E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8224F0E4: 419A0028  beq cr6, 0x8224f10c
	if ctx.cr[6].eq {
	pc = 0x8224F10C; continue 'dispatch;
	}
	// 8224F0E8: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8224F0EC: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F0F0: 7F043840  cmplw cr6, r4, r7
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8224F0F4: 419A002C  beq cr6, 0x8224f120
	if ctx.cr[6].eq {
	pc = 0x8224F120; continue 'dispatch;
	}
	// 8224F0F8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F0FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224F100: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8224F104: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8224F108: 4198FFE4  blt cr6, 0x8224f0ec
	if ctx.cr[6].lt {
	pc = 0x8224F0EC; continue 'dispatch;
	}
	// 8224F10C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224F110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8224F114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224F118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224F11C: 4E800020  blr
	return;
	// 8224F120: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8224F124: 40980024  bge cr6, 0x8224f148
	if !ctx.cr[6].lt {
	pc = 0x8224F148; continue 'dispatch;
	}
	// 8224F128: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8224F12C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8224F130: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224F134: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8224F138: 7C694214  add r3, r9, r8
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8224F13C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8224F140: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 8224F144: 4BE400AD  bl 0x8208f1f0
	ctx.lr = 0x8224F148;
	sub_8208F1F0(ctx, base);
	// 8224F148: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8224F14C: 4BFFFFC4  b 0x8224f110
	pc = 0x8224F110; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F150 size=132
    let mut pc: u32 = 0x8224F150;
    'dispatch: loop {
        match pc {
            0x8224F150 => {
    //   block [0x8224F150..0x8224F1D4)
	// 8224F150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F154: 4BE3F949  bl 0x8208ea9c
	ctx.lr = 0x8224F158;
	sub_8208EA60(ctx, base);
	// 8224F158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F15C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224F160: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8224F164: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 8224F168: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F16C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8224F170: 40990014  ble cr6, 0x8224f184
	if !ctx.cr[6].gt {
	pc = 0x8224F184; continue 'dispatch;
	}
	// 8224F174: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F178: 4800000C  b 0x8224f184
	pc = 0x8224F184; continue 'dispatch;
	// 8224F17C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F180: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8224F184: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F188: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F18C: 4098FFF0  bge cr6, 0x8224f17c
	if !ctx.cr[6].lt {
	pc = 0x8224F17C; continue 'dispatch;
	}
	// 8224F190: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8224F194: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F198: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224F19C: 4BFC8CBD  bl 0x82217e58
	ctx.lr = 0x8224F1A0;
	sub_82217E58(ctx, base);
	// 8224F1A0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8224F1A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F1A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8224F1AC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8224F1B0: 4BE40041  bl 0x8208f1f0
	ctx.lr = 0x8224F1B4;
	sub_8208F1F0(ctx, base);
	// 8224F1B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8224F1B8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224F1BC: 4BFC8DCD  bl 0x82217f88
	ctx.lr = 0x8224F1C0;
	sub_82217F88(ctx, base);
	// 8224F1C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F1C4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224F1C8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8224F1CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224F1D0: 4BE3F91C  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F1D8 size=172
    let mut pc: u32 = 0x8224F1D8;
    'dispatch: loop {
        match pc {
            0x8224F1D8 => {
    //   block [0x8224F1D8..0x8224F284)
	// 8224F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224F1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8224F1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224F1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224F1F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8224F1F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F1F8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F1FC: 40990008  ble cr6, 0x8224f204
	if !ctx.cr[6].gt {
	pc = 0x8224F204; continue 'dispatch;
	}
	// 8224F200: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8224F204: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F208: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F20C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224F210: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F214: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8224F218: 40990010  ble cr6, 0x8224f228
	if !ctx.cr[6].gt {
	pc = 0x8224F228; continue 'dispatch;
	}
	// 8224F21C: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 8224F220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224F224: 4BFFFF2D  bl 0x8224f150
	ctx.lr = 0x8224F228;
	sub_8224F150(ctx, base);
	// 8224F228: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F22C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F230: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F234: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 8224F238: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224F23C: 3569FFFF  addic. r11, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224F240: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224F244: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8224F248: 4182001C  beq 0x8224f264
	if ctx.cr[0].eq {
	pc = 0x8224F264; continue 'dispatch;
	}
	// 8224F24C: 810AFFFC  lwz r8, -4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8224F250: 392AFFFC  addi r9, r10, -4
	ctx.r[9].s64 = ctx.r[10].s64 + -4;
	// 8224F254: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224F258: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8224F25C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8224F260: 4082FFEC  bne 0x8224f24c
	if !ctx.cr[0].eq {
	pc = 0x8224F24C; continue 'dispatch;
	}
	// 8224F264: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224F268: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8224F26C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224F270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224F274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224F278: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8224F27C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224F280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F288 size=384
    let mut pc: u32 = 0x8224F288;
    'dispatch: loop {
        match pc {
            0x8224F288 => {
    //   block [0x8224F288..0x8224F408)
	// 8224F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F28C: 4BE3F7ED  bl 0x8208ea78
	ctx.lr = 0x8224F290;
	sub_8208EA60(ctx, base);
	// 8224F290: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8224F298: 82DE0000  lwz r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F29C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F2A0: 56CB083C  slwi r11, r22, 1
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F2A4: 56C41838  slwi r4, r22, 3
	ctx.r[4].u32 = ctx.r[22].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8224F2A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8224F2AC: 4BFC8BAD  bl 0x82217e58
	ctx.lr = 0x8224F2B0;
	sub_82217E58(ctx, base);
	// 8224F2B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F2B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224F2B8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8224F2BC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8224F2C0: 4BE3F8C1  bl 0x8208eb80
	ctx.lr = 0x8224F2C4;
	sub_8208EB80(ctx, base);
	// 8224F2C4: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8224F2C8: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8224F2CC: 40990130  ble cr6, 0x8224f3fc
	if !ctx.cr[6].gt {
	pc = 0x8224F3FC; continue 'dispatch;
	}
	// 8224F2D0: 56CB103A  slwi r11, r22, 2
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F2D4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8224F2D8: 7EABBA14  add r21, r11, r23
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 8224F2DC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F2E0: 7F3A582E  lwzx r25, r26, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8224F2E4: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8224F2E8: 419A0100  beq cr6, 0x8224f3e8
	if ctx.cr[6].eq {
	pc = 0x8224F3E8; continue 'dispatch;
	}
	// 8224F2EC: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F2F0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8224F2F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8224F2F8: 4BFC8B61  bl 0x82217e58
	ctx.lr = 0x8224F2FC;
	sub_82217E58(ctx, base);
	// 8224F2FC: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224F300: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8224F304: 41820018  beq 0x8224f31c
	if ctx.cr[0].eq {
	pc = 0x8224F31C; continue 'dispatch;
	}
	// 8224F308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224F30C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F310: 4BFE2421  bl 0x82231730
	ctx.lr = 0x8224F314;
	sub_82231730(ctx, base);
	// 8224F314: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8224F318: 48000008  b 0x8224f320
	pc = 0x8224F320; continue 'dispatch;
	// 8224F31C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224F320: 7D7AB92E  stwx r11, r26, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[23].u32), ctx.r[11].u32) };
	// 8224F324: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8224F328: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F32C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8224F330: 4BFC8B29  bl 0x82217e58
	ctx.lr = 0x8224F334;
	sub_82217E58(ctx, base);
	// 8224F334: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224F338: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8224F33C: 41820018  beq 0x8224f354
	if ctx.cr[0].eq {
	pc = 0x8224F354; continue 'dispatch;
	}
	// 8224F340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224F344: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F348: 4BFE23E9  bl 0x82231730
	ctx.lr = 0x8224F34C;
	sub_82231730(ctx, base);
	// 8224F34C: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8224F350: 48000008  b 0x8224f358
	pc = 0x8224F358; continue 'dispatch;
	// 8224F354: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8224F358: 93750000  stw r27, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224F35C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F360: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224F364: 7F1AB82E  lwzx r24, r26, r23
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8224F368: 419A0080  beq cr6, 0x8224f3e8
	if ctx.cr[6].eq {
	pc = 0x8224F3E8; continue 'dispatch;
	}
	// 8224F36C: 57FC103A  slwi r28, r31, 2
	ctx.r[28].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8224F370: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F374: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8224F378: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 8224F37C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F380: 40980010  bge cr6, 0x8224f390
	if !ctx.cr[6].lt {
	pc = 0x8224F390; continue 'dispatch;
	}
	// 8224F384: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F388: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8224F38C: 48000010  b 0x8224f39c
	pc = 0x8224F39C; continue 'dispatch;
	// 8224F390: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224F394: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8224F398: 4BFFFDB9  bl 0x8224f150
	ctx.lr = 0x8224F39C;
	sub_8224F150(ctx, base);
	// 8224F39C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F3A0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224F3A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8224F3A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F3AC: 4E800421  bctrl
	ctx.lr = 0x8224F3B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F3B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F3B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224F3B8: 7C6B5838  and r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[11].u64;
	// 8224F3BC: 7F0BA040  cmplw cr6, r11, r20
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[20].u32, &mut ctx.xer);
	// 8224F3C0: 409A0010  bne cr6, 0x8224f3d0
	if !ctx.cr[6].eq {
	pc = 0x8224F3D0; continue 'dispatch;
	}
	// 8224F3C4: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F3C8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8224F3CC: 4800000C  b 0x8224f3d8
	pc = 0x8224F3D8; continue 'dispatch;
	// 8224F3D0: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F3D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8224F3D8: 4BFFFD79  bl 0x8224f150
	ctx.lr = 0x8224F3DC;
	sub_8224F150(ctx, base);
	// 8224F3DC: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8224F3E0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8224F3E4: 409AFF8C  bne cr6, 0x8224f370
	if !ctx.cr[6].eq {
	pc = 0x8224F370; continue 'dispatch;
	}
	// 8224F3E8: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 8224F3EC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8224F3F0: 3AB50004  addi r21, r21, 4
	ctx.r[21].s64 = ctx.r[21].s64 + 4;
	// 8224F3F4: 7F14B000  cmpw cr6, r20, r22
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[22].s32, &mut ctx.xer);
	// 8224F3F8: 4198FEE4  blt cr6, 0x8224f2dc
	if ctx.cr[6].lt {
	pc = 0x8224F2DC; continue 'dispatch;
	}
	// 8224F3FC: 92FE0008  stw r23, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 8224F400: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8224F404: 4BE3F6C4  b 0x8208eac8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F408 size=192
    let mut pc: u32 = 0x8224F408;
    'dispatch: loop {
        match pc {
            0x8224F408 => {
    //   block [0x8224F408..0x8224F4C8)
	// 8224F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F40C: 4BE3F685  bl 0x8208ea90
	ctx.lr = 0x8224F410;
	sub_8208EA60(ctx, base);
	// 8224F410: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F414: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8224F418: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8224F41C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8224F420: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224F424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F428: 4E800421  bctrl
	ctx.lr = 0x8224F42C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F42C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F430: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F434: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224F438: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 8224F43C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F440: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8224F444: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8224F448: 419A006C  beq cr6, 0x8224f4b4
	if ctx.cr[6].eq {
	pc = 0x8224F4B4; continue 'dispatch;
	}
	// 8224F44C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F450: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8224F454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F458: 419A005C  beq cr6, 0x8224f4b4
	if ctx.cr[6].eq {
	pc = 0x8224F4B4; continue 'dispatch;
	}
	// 8224F45C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8224F460: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F464: 40980010  bge cr6, 0x8224f474
	if !ctx.cr[6].lt {
	pc = 0x8224F474; continue 'dispatch;
	}
	// 8224F468: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F46C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8224F470: 48000010  b 0x8224f480
	pc = 0x8224F480; continue 'dispatch;
	// 8224F474: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8224F478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224F47C: 4BFFFCD5  bl 0x8224f150
	ctx.lr = 0x8224F480;
	sub_8224F150(ctx, base);
	// 8224F480: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F484: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8224F488: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224F48C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8224F490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F494: 4E800421  bctrl
	ctx.lr = 0x8224F498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F498: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8224F49C: 41820024  beq 0x8224f4c0
	if ctx.cr[0].eq {
	pc = 0x8224F4C0; continue 'dispatch;
	}
	// 8224F4A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F4A4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8224F4A8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8224F4AC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F4B0: 4198FFB8  blt cr6, 0x8224f468
	if ctx.cr[6].lt {
	pc = 0x8224F468; continue 'dispatch;
	}
	// 8224F4B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224F4B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8224F4BC: 4BE3F624  b 0x8208eae0
	sub_8208EAB0(ctx, base);
	return;
	// 8224F4C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8224F4C4: 4BFFFFF4  b 0x8224f4b8
	pc = 0x8224F4B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F4C8 size=192
    let mut pc: u32 = 0x8224F4C8;
    'dispatch: loop {
        match pc {
            0x8224F4C8 => {
    //   block [0x8224F4C8..0x8224F588)
	// 8224F4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F4CC: 4BE3F5C9  bl 0x8208ea94
	ctx.lr = 0x8224F4D0;
	sub_8208EA60(ctx, base);
	// 8224F4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F4D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224F4D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8224F4DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8224F4E0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224F4E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F4E8: 4E800421  bctrl
	ctx.lr = 0x8224F4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F4EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F4F0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F4F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224F4F8: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 8224F4FC: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8224F500: 7D6AE82E  lwzx r11, r10, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8224F504: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224F508: 409A003C  bne cr6, 0x8224f544
	if !ctx.cr[6].eq {
	pc = 0x8224F544; continue 'dispatch;
	}
	// 8224F50C: 837F0014  lwz r27, 0x14(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F510: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8224F514: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8224F518: 4BFC8941  bl 0x82217e58
	ctx.lr = 0x8224F51C;
	sub_82217E58(ctx, base);
	// 8224F51C: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8224F520: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224F524: 41820014  beq 0x8224f538
	if ctx.cr[0].eq {
	pc = 0x8224F538; continue 'dispatch;
	}
	// 8224F528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8224F52C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224F530: 4BFE2201  bl 0x82231730
	ctx.lr = 0x8224F534;
	sub_82231730(ctx, base);
	// 8224F534: 48000008  b 0x8224f53c
	pc = 0x8224F53C; continue 'dispatch;
	// 8224F538: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8224F53C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F540: 7FCBE92E  stwx r30, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[30].u32) };
	// 8224F544: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F548: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224F54C: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8224F550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8224F554: 4BFFFC85  bl 0x8224f1d8
	ctx.lr = 0x8224F558;
	sub_8224F1D8(ctx, base);
	// 8224F558: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8224F55C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F560: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F564: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8224F568: 4099000C  ble cr6, 0x8224f574
	if !ctx.cr[6].gt {
	pc = 0x8224F574; continue 'dispatch;
	}
	// 8224F56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224F570: 4BFFFD19  bl 0x8224f288
	ctx.lr = 0x8224F574;
	sub_8224F288(ctx, base);
	// 8224F574: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F578: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224F57C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224F584: 4BE3F560  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F588 size=228
    let mut pc: u32 = 0x8224F588;
    'dispatch: loop {
        match pc {
            0x8224F588 => {
    //   block [0x8224F588..0x8224F66C)
	// 8224F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F58C: 4BE3F509  bl 0x8208ea94
	ctx.lr = 0x8224F590;
	sub_8208EA60(ctx, base);
	// 8224F590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F594: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8224F598: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8224F59C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8224F5A0: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224F5A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F5A8: 4E800421  bctrl
	ctx.lr = 0x8224F5AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F5AC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F5B0: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F5B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224F5B8: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 8224F5BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F5C0: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8224F5C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8224F5C8: 419A009C  beq cr6, 0x8224f664
	if ctx.cr[6].eq {
	pc = 0x8224F664; continue 'dispatch;
	}
	// 8224F5CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F5D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8224F5D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F5D8: 4099008C  ble cr6, 0x8224f664
	if !ctx.cr[6].gt {
	pc = 0x8224F664; continue 'dispatch;
	}
	// 8224F5DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8224F5E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F5E4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F5E8: 40980010  bge cr6, 0x8224f5f8
	if !ctx.cr[6].lt {
	pc = 0x8224F5F8; continue 'dispatch;
	}
	// 8224F5EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F5F0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8224F5F4: 48000010  b 0x8224f604
	pc = 0x8224F604; continue 'dispatch;
	// 8224F5F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8224F5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224F600: 4BFFFB51  bl 0x8224f150
	ctx.lr = 0x8224F604;
	sub_8224F150(ctx, base);
	// 8224F604: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F608: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8224F60C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224F610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F614: 4E800421  bctrl
	ctx.lr = 0x8224F618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F618: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F61C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8224F620: 41820018  beq 0x8224f638
	if ctx.cr[0].eq {
	pc = 0x8224F638; continue 'dispatch;
	}
	// 8224F624: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8224F628: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8224F62C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F630: 4198FFB0  blt cr6, 0x8224f5e0
	if ctx.cr[6].lt {
	pc = 0x8224F5E0; continue 'dispatch;
	}
	// 8224F634: 48000030  b 0x8224f664
	pc = 0x8224F664; continue 'dispatch;
	// 8224F638: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F63C: 40980028  bge cr6, 0x8224f664
	if !ctx.cr[6].lt {
	pc = 0x8224F664; continue 'dispatch;
	}
	// 8224F640: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8224F644: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F648: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224F64C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F650: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8224F654: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8224F658: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8224F65C: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 8224F660: 4BE3FB91  bl 0x8208f1f0
	ctx.lr = 0x8224F664;
	sub_8208F1F0(ctx, base);
	// 8224F664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224F668: 4BE3F47C  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F670 size=196
    let mut pc: u32 = 0x8224F670;
    'dispatch: loop {
        match pc {
            0x8224F670 => {
    //   block [0x8224F670..0x8224F734)
	// 8224F670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224F678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224F67C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F680: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224F684: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224F688: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8224F68C: 41980038  blt cr6, 0x8224f6c4
	if ctx.cr[6].lt {
	pc = 0x8224F6C4; continue 'dispatch;
	}
	// 8224F690: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F694: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 8224F698: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8224F69C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F6A0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224F6A4: 40980014  bge cr6, 0x8224f6b8
	if !ctx.cr[6].lt {
	pc = 0x8224F6B8; continue 'dispatch;
	}
	// 8224F6A8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F6AC: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F6B0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224F6B4: 48000008  b 0x8224f6bc
	pc = 0x8224F6BC; continue 'dispatch;
	// 8224F6B8: 4BFFFA99  bl 0x8224f150
	ctx.lr = 0x8224F6BC;
	sub_8224F150(ctx, base);
	// 8224F6BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F6C0: 4800005C  b 0x8224f71c
	pc = 0x8224F71C; continue 'dispatch;
	// 8224F6C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F6C8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F6CC: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 8224F6D0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F6D4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8224F6D8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F6DC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8224F6E0: 40980038  bge cr6, 0x8224f718
	if !ctx.cr[6].lt {
	pc = 0x8224F718; continue 'dispatch;
	}
	// 8224F6E4: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F6E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224F6EC: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8224F6F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F6F4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8224F6F8: 419AFFD4  beq cr6, 0x8224f6cc
	if ctx.cr[6].eq {
	pc = 0x8224F6CC; continue 'dispatch;
	}
	// 8224F6FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F700: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F704: 419AFFC8  beq cr6, 0x8224f6cc
	if ctx.cr[6].eq {
	pc = 0x8224F6CC; continue 'dispatch;
	}
	// 8224F708: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F70C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F710: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 8224F714: 4BFFFF7C  b 0x8224f690
	pc = 0x8224F690; continue 'dispatch;
	// 8224F718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224F71C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8224F720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8224F724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224F728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224F72C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224F730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F738 size=24
    let mut pc: u32 = 0x8224F738;
    'dispatch: loop {
        match pc {
            0x8224F738 => {
    //   block [0x8224F738..0x8224F750)
	// 8224F738: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8224F73C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8224F740: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8224F744: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F748: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8224F74C: 4BFFFF24  b 0x8224f670
	sub_8224F670(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F750 size=28
    let mut pc: u32 = 0x8224F750;
    'dispatch: loop {
        match pc {
            0x8224F750 => {
    //   block [0x8224F750..0x8224F76C)
	// 8224F750: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8224F754: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 8224F758: 396B9638  addi r11, r11, -0x69c8
	ctx.r[11].s64 = ctx.r[11].s64 + -27080;
	// 8224F75C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8224F760: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8224F764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F768: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F770 size=120
    let mut pc: u32 = 0x8224F770;
    'dispatch: loop {
        match pc {
            0x8224F770 => {
    //   block [0x8224F770..0x8224F7E8)
	// 8224F770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224F778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8224F77C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224F780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F784: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8224F788: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8224F78C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8224F790: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 8224F794: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8224F798: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8224F79C: 40820024  bne 0x8224f7c0
	if !ctx.cr[0].eq {
	pc = 0x8224F7C0; continue 'dispatch;
	}
	// 8224F7A0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8224F7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8224F7A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8224F7AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224F7B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F7B4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8224F7B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F7BC: 4E800421  bctrl
	ctx.lr = 0x8224F7C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F7C0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8224F7C4: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8224F7C8: 4198FFC8  blt cr6, 0x8224f790
	if ctx.cr[6].lt {
	pc = 0x8224F790; continue 'dispatch;
	}
	// 8224F7CC: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8224F7D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224F7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224F7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224F7DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8224F7E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224F7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F7E8 size=144
    let mut pc: u32 = 0x8224F7E8;
    'dispatch: loop {
        match pc {
            0x8224F7E8 => {
    //   block [0x8224F7E8..0x8224F878)
	// 8224F7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224F7F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8224F7F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224F7F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F7FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8224F800: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8224F804: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8224F808: 4BFE1EE1  bl 0x822316e8
	ctx.lr = 0x8224F80C;
	sub_822316E8(ctx, base);
	// 8224F80C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224F810: 4182004C  beq 0x8224f85c
	if ctx.cr[0].eq {
	pc = 0x8224F85C; continue 'dispatch;
	}
	// 8224F814: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8224F818: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 8224F81C: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8224F820: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8224F824: 40820024  bne 0x8224f848
	if !ctx.cr[0].eq {
	pc = 0x8224F848; continue 'dispatch;
	}
	// 8224F828: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8224F82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8224F830: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8224F834: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224F838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F83C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8224F840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224F844: 4E800421  bctrl
	ctx.lr = 0x8224F848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224F848: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8224F84C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8224F850: 4198FFC8  blt cr6, 0x8224f818
	if ctx.cr[6].lt {
	pc = 0x8224F818; continue 'dispatch;
	}
	// 8224F854: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8224F858: 48000008  b 0x8224f860
	pc = 0x8224F860; continue 'dispatch;
	// 8224F85C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8224F860: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224F864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224F868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224F86C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8224F870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224F874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F878 size=100
    let mut pc: u32 = 0x8224F878;
    'dispatch: loop {
        match pc {
            0x8224F878 => {
    //   block [0x8224F878..0x8224F8DC)
	// 8224F878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F87C: 4BE3F21D  bl 0x8208ea98
	ctx.lr = 0x8224F880;
	sub_8208EA60(ctx, base);
	// 8224F880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F884: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8224F888: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8224F88C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8224F890: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8224F894: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224F898: 838B05AC  lwz r28, 0x5ac(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224F89C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8224F8A0: 4BFC85B9  bl 0x82217e58
	ctx.lr = 0x8224F8A4;
	sub_82217E58(ctx, base);
	// 8224F8A4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224F8A8: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8224F8AC: 41820018  beq 0x8224f8c4
	if ctx.cr[0].eq {
	pc = 0x8224F8C4; continue 'dispatch;
	}
	// 8224F8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8224F8B4: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8224F8B8: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8224F8BC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224F8C0: 48000008  b 0x8224f8c8
	pc = 0x8224F8C8; continue 'dispatch;
	// 8224F8C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8224F8C8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F8CC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8224F8D0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8224F8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224F8D8: 4BE3F210  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F8E0 size=16
    let mut pc: u32 = 0x8224F8E0;
    'dispatch: loop {
        match pc {
            0x8224F8E0 => {
    //   block [0x8224F8E0..0x8224F8F0)
	// 8224F8E0: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8224F8E4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F8E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F8EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F8F0 size=20
    let mut pc: u32 = 0x8224F8F0;
    'dispatch: loop {
        match pc {
            0x8224F8F0 => {
    //   block [0x8224F8F0..0x8224F904)
	// 8224F8F0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F8F4: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8224F8F8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8224F8FC: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8224F900: 4BFC8688  b 0x82217f88
	sub_82217F88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F904 size=4
    let mut pc: u32 = 0x8224F904;
    'dispatch: loop {
        match pc {
            0x8224F904 => {
    //   block [0x8224F904..0x8224F908)
	// 8224F904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F908 size=20
    let mut pc: u32 = 0x8224F908;
    'dispatch: loop {
        match pc {
            0x8224F908 => {
    //   block [0x8224F908..0x8224F91C)
	// 8224F908: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8224F90C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F914: 409A0008  bne cr6, 0x8224f91c
	if !ctx.cr[6].eq {
		sub_8224F91C(ctx, base);
		return;
	}
	// 8224F918: 4BFFFF60  b 0x8224f878
	sub_8224F878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F91C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F91C size=24
    let mut pc: u32 = 0x8224F91C;
    'dispatch: loop {
        match pc {
            0x8224F91C => {
    //   block [0x8224F91C..0x8224F934)
	// 8224F91C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F920: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F924: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8224F928: 409A000C  bne cr6, 0x8224f934
	if !ctx.cr[6].eq {
		sub_8224F934(ctx, base);
		return;
	}
	// 8224F92C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8224F930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224F934 size=4
    let mut pc: u32 = 0x8224F934;
    'dispatch: loop {
        match pc {
            0x8224F934 => {
    //   block [0x8224F934..0x8224F938)
	// 8224F934: 4BFFFF44  b 0x8224f878
	sub_8224F878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F938 size=88
    let mut pc: u32 = 0x8224F938;
    'dispatch: loop {
        match pc {
            0x8224F938 => {
    //   block [0x8224F938..0x8224F990)
	// 8224F938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224F940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224F944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224F94C: 48000024  b 0x8224f970
	pc = 0x8224F970; continue 'dispatch;
	// 8224F950: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F958: 419A0018  beq cr6, 0x8224f970
	if ctx.cr[6].eq {
	pc = 0x8224F970; continue 'dispatch;
	}
	// 8224F95C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224F960: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8224F964: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8224F968: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8224F96C: 4BFC861D  bl 0x82217f88
	ctx.lr = 0x8224F970;
	sub_82217F88(ctx, base);
	// 8224F970: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224F974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224F978: 409AFFD8  bne cr6, 0x8224f950
	if !ctx.cr[6].eq {
	pc = 0x8224F950; continue 'dispatch;
	}
	// 8224F97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8224F980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224F984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224F988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224F98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224F990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224F990 size=256
    let mut pc: u32 = 0x8224F990;
    'dispatch: loop {
        match pc {
            0x8224F990 => {
    //   block [0x8224F990..0x8224FA90)
	// 8224F990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224F994: 4BE3F101  bl 0x8208ea94
	ctx.lr = 0x8224F998;
	sub_8208EA60(ctx, base);
	// 8224F998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224F99C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224F9A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8224F9A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8224F9A8: 396B4AF4  addi r11, r11, 0x4af4
	ctx.r[11].s64 = ctx.r[11].s64 + 19188;
	// 8224F9AC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8224F9B0: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8224F9B4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8224F9B8: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 8224F9BC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8224F9C0: 9BBF0004  stb r29, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 8224F9C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8224F9C8: 9BBF0005  stb r29, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[29].u8 ) };
	// 8224F9CC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8224F9D0: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 8224F9D4: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8224F9D8: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8224F9DC: 9BBF001D  stb r29, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[29].u8 ) };
	// 8224F9E0: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224F9E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8224F9E8: 4BFC8471  bl 0x82217e58
	ctx.lr = 0x8224F9EC;
	sub_82217E58(ctx, base);
	// 8224F9EC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224F9F0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8224F9F4: 41820010  beq 0x8224fa04
	if ctx.cr[0].eq {
	pc = 0x8224FA04; continue 'dispatch;
	}
	// 8224F9F8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8224F9FC: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8224FA00: 48000008  b 0x8224fa08
	pc = 0x8224FA08; continue 'dispatch;
	// 8224FA04: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8224FA08: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8224FA0C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8224FA10: 837E05AC  lwz r27, 0x5ac(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224FA14: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8224FA18: 4BFC8441  bl 0x82217e58
	ctx.lr = 0x8224FA1C;
	sub_82217E58(ctx, base);
	// 8224FA1C: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8224FA20: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224FA24: 41820014  beq 0x8224fa38
	if ctx.cr[0].eq {
	pc = 0x8224FA38; continue 'dispatch;
	}
	// 8224FA28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8224FA2C: 809E05AC  lwz r4, 0x5ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224FA30: 4BFE1D01  bl 0x82231730
	ctx.lr = 0x8224FA34;
	sub_82231730(ctx, base);
	// 8224FA34: 48000008  b 0x8224fa3c
	pc = 0x8224FA3C; continue 'dispatch;
	// 8224FA38: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 8224FA3C: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 8224FA40: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8224FA44: 837E05AC  lwz r27, 0x5ac(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224FA48: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8224FA4C: 4BFC840D  bl 0x82217e58
	ctx.lr = 0x8224FA50;
	sub_82217E58(ctx, base);
	// 8224FA50: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8224FA54: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224FA58: 41820014  beq 0x8224fa6c
	if ctx.cr[0].eq {
	pc = 0x8224FA6C; continue 'dispatch;
	}
	// 8224FA5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8224FA60: 809E05AC  lwz r4, 0x5ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224FA64: 4BFE1CCD  bl 0x82231730
	ctx.lr = 0x8224FA68;
	sub_82231730(ctx, base);
	// 8224FA68: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8224FA6C: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 8224FA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224FA74: 817E0598  lwz r11, 0x598(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1432 as u32) ) } as u64;
	// 8224FA78: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8224FA7C: 817E0598  lwz r11, 0x598(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1432 as u32) ) } as u64;
	// 8224FA80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224FA84: 917E0598  stw r11, 0x598(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1432 as u32), ctx.r[11].u32 ) };
	// 8224FA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224FA8C: 4BE3F058  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224FA90 size=8
    let mut pc: u32 = 0x8224FA90;
    'dispatch: loop {
        match pc {
            0x8224FA90 => {
    //   block [0x8224FA90..0x8224FA98)
	// 8224FA90: 98830004  stb r4, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u8 ) };
	// 8224FA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8224FA98 size=8
    let mut pc: u32 = 0x8224FA98;
    'dispatch: loop {
        match pc {
            0x8224FA98 => {
    //   block [0x8224FA98..0x8224FAA0)
	// 8224FA98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224FAA0 size=80
    let mut pc: u32 = 0x8224FAA0;
    'dispatch: loop {
        match pc {
            0x8224FAA0 => {
    //   block [0x8224FAA0..0x8224FAF0)
	// 8224FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224FAA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8224FAAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224FAB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224FAB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224FAB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8224FABC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8224FAC0: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FAC4: 4BFFF68D  bl 0x8224f150
	ctx.lr = 0x8224FAC8;
	sub_8224F150(ctx, base);
	// 8224FAC8: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8224FACC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8224FAD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224FAD4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8224FAD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224FADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224FAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224FAE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8224FAE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224FAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224FAF0 size=124
    let mut pc: u32 = 0x8224FAF0;
    'dispatch: loop {
        match pc {
            0x8224FAF0 => {
    //   block [0x8224FAF0..0x8224FB6C)
	// 8224FAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224FAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8224FAF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8224FAFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8224FB00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224FB04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224FB08: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8224FB0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8224FB10: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8224FB14: 40990024  ble cr6, 0x8224fb38
	if !ctx.cr[6].gt {
	pc = 0x8224FB38; continue 'dispatch;
	}
	// 8224FB18: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 8224FB1C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FB20: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8224FB24: 419A0030  beq cr6, 0x8224fb54
	if ctx.cr[6].eq {
	pc = 0x8224FB54; continue 'dispatch;
	}
	// 8224FB28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8224FB2C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8224FB30: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8224FB34: 4198FFE8  blt cr6, 0x8224fb1c
	if ctx.cr[6].lt {
	pc = 0x8224FB1C; continue 'dispatch;
	}
	// 8224FB38: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8224FB3C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FB40: 4BFFF611  bl 0x8224f150
	ctx.lr = 0x8224FB44;
	sub_8224F150(ctx, base);
	// 8224FB44: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8224FB48: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8224FB4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8224FB50: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8224FB54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8224FB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8224FB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8224FB60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8224FB64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8224FB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8224FB70 size=728
    let mut pc: u32 = 0x8224FB70;
    'dispatch: loop {
        match pc {
            0x8224FB70 => {
    //   block [0x8224FB70..0x8224FE48)
	// 8224FB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224FB74: 4BE3EF11  bl 0x8208ea84
	ctx.lr = 0x8224FB78;
	sub_8208EA60(ctx, base);
	// 8224FB78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224FB7C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8224FB80: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8224FB84: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8224FB88: 817A002C  lwz r11, 0x2c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(44 as u32) ) } as u64;
	// 8224FB8C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FB90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8224FB94: 419A0010  beq cr6, 0x8224fba4
	if ctx.cr[6].eq {
	pc = 0x8224FBA4; continue 'dispatch;
	}
	// 8224FB98: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224FB9C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FBA0: 480002A0  b 0x8224fe40
	pc = 0x8224FE40; continue 'dispatch;
	// 8224FBA4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FBA8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8224FBAC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224FBB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224FBB4: 4E800421  bctrl
	ctx.lr = 0x8224FBB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224FBB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224FBBC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8224FBC0: 3B600003  li r27, 3
	ctx.r[27].s64 = 3;
	// 8224FBC4: 41820110  beq 0x8224fcd4
	if ctx.cr[0].eq {
	pc = 0x8224FCD4; continue 'dispatch;
	}
	// 8224FBC8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8224FBCC: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 8224FBD0: 48003DB1  bl 0x82253980
	ctx.lr = 0x8224FBD4;
	sub_82253980(ctx, base);
	// 8224FBD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8224FBD8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8224FBDC: 809C0AB0  lwz r4, 0xab0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2736 as u32) ) } as u64;
	// 8224FBE0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8224FBE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224FBE8: C06B06F8  lfs f3, 0x6f8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1784 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8224FBEC: C08A06E0  lfs f4, 0x6e0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1760 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8224FBF0: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 8224FBF4: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 8224FBF8: 48002949  bl 0x82252540
	ctx.lr = 0x8224FBFC;
	sub_82252540(ctx, base);
	// 8224FBFC: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 8224FC00: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8224FC04: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8224FC08: 806A0028  lwz r3, 0x28(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 8224FC0C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FC10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224FC14: 4099000C  ble cr6, 0x8224fc20
	if !ctx.cr[6].gt {
	pc = 0x8224FC20; continue 'dispatch;
	}
	// 8224FC18: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224FC1C: 4800000C  b 0x8224fc28
	pc = 0x8224FC28; continue 'dispatch;
	// 8224FC20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224FC24: 4BFFF52D  bl 0x8224f150
	ctx.lr = 0x8224FC28;
	sub_8224F150(ctx, base);
	// 8224FC28: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FC2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8224FC30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224FC34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8224FC38: 4BFE1D49  bl 0x82231980
	ctx.lr = 0x8224FC3C;
	sub_82231980(ctx, base);
	// 8224FC3C: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8224FC40: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8224FC44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FC48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224FC4C: 409A0100  bne cr6, 0x8224fd4c
	if !ctx.cr[6].eq {
	pc = 0x8224FD4C; continue 'dispatch;
	}
	// 8224FC50: 82FC05B0  lwz r23, 0x5b0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8224FC54: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 8224FC58: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8224FC5C: 4BFC81FD  bl 0x82217e58
	ctx.lr = 0x8224FC60;
	sub_82217E58(ctx, base);
	// 8224FC60: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224FC64: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8224FC68: 41820028  beq 0x8224fc90
	if ctx.cr[0].eq {
	pc = 0x8224FC90; continue 'dispatch;
	}
	// 8224FC6C: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8224FC70: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8224FC74: 938B0104  stw r28, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[28].u32 ) };
	// 8224FC78: 9B2B0108  stb r25, 0x108(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[25].u8 ) };
	// 8224FC7C: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224FC80: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8224FC84: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8224FC88: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 8224FC8C: 48000008  b 0x8224fc94
	pc = 0x8224FC94; continue 'dispatch;
	// 8224FC90: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 8224FC94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8224FC98: 4BFF48E9  bl 0x82244580
	ctx.lr = 0x8224FC9C;
	sub_82244580(ctx, base);
	// 8224FC9C: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8224FCA0: 80980030  lwz r4, 0x30(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(48 as u32) ) } as u64;
	// 8224FCA4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FCA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224FCAC: 419A001C  beq cr6, 0x8224fcc8
	if ctx.cr[6].eq {
	pc = 0x8224FCC8; continue 'dispatch;
	}
	// 8224FCB0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8224FCB4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FCB8: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8224FCBC: 409A000C  bne cr6, 0x8224fcc8
	if !ctx.cr[6].eq {
	pc = 0x8224FCC8; continue 'dispatch;
	}
	// 8224FCC0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8224FCC4: 48000088  b 0x8224fd4c
	pc = 0x8224FD4C; continue 'dispatch;
	// 8224FCC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8224FCCC: 4BFFFBAD  bl 0x8224f878
	ctx.lr = 0x8224FCD0;
	sub_8224F878(ctx, base);
	// 8224FCD0: 4800007C  b 0x8224fd4c
	pc = 0x8224FD4C; continue 'dispatch;
	// 8224FCD4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FCD8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8224FCDC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8224FCE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224FCE4: 4E800421  bctrl
	ctx.lr = 0x8224FCE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224FCE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224FCEC: 40820028  bne 0x8224fd14
	if !ctx.cr[0].eq {
	pc = 0x8224FD14; continue 'dispatch;
	}
	// 8224FCF0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8224FCF4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8224FCF8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 8224FCFC: 38CB4A60  addi r6, r11, 0x4a60
	ctx.r[6].s64 = ctx.r[11].s64 + 19040;
	// 8224FD00: 38AAFE08  addi r5, r10, -0x1f8
	ctx.r[5].s64 = ctx.r[10].s64 + -504;
	// 8224FD04: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 8224FD08: 38E000EC  li r7, 0xec
	ctx.r[7].s64 = 236;
	// 8224FD0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8224FD10: 4BF05CF9  bl 0x82155a08
	ctx.lr = 0x8224FD14;
	sub_82155A08(ctx, base);
	// 8224FD14: 83FC05AC  lwz r31, 0x5ac(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224FD18: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8224FD1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224FD20: 4BFC8139  bl 0x82217e58
	ctx.lr = 0x8224FD24;
	sub_82217E58(ctx, base);
	// 8224FD24: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8224FD28: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8224FD2C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8224FD30: 41820018  beq 0x8224fd48
	if ctx.cr[0].eq {
	pc = 0x8224FD48; continue 'dispatch;
	}
	// 8224FD34: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8224FD38: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8224FD3C: 48002CBD  bl 0x822529f8
	ctx.lr = 0x8224FD40;
	sub_822529F8(ctx, base);
	// 8224FD40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224FD44: 48000008  b 0x8224fd4c
	pc = 0x8224FD4C; continue 'dispatch;
	// 8224FD48: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8224FD4C: 935F001C  stw r26, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 8224FD50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224FD54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8224FD58: 4BFFFD49  bl 0x8224faa0
	ctx.lr = 0x8224FD5C;
	sub_8224FAA0(ctx, base);
	// 8224FD5C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FD60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224FD64: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8224FD68: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8224FD6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224FD70: 4E800421  bctrl
	ctx.lr = 0x8224FD74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224FD74: 897A0005  lbz r11, 5(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(5 as u32) ) } as u64;
	// 8224FD78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8224FD7C: 41820024  beq 0x8224fda0
	if ctx.cr[0].eq {
	pc = 0x8224FDA0; continue 'dispatch;
	}
	// 8224FD80: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 8224FD84: 815A0010  lwz r10, 0x10(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224FD88: 813F00E4  lwz r9, 0xe4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8224FD8C: 61290040  ori r9, r9, 0x40
	ctx.r[9].u64 = ctx.r[9].u64 | 64;
	// 8224FD90: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8224FD94: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8224FD98: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 8224FD9C: 4800002C  b 0x8224fdc8
	pc = 0x8224FDC8; continue 'dispatch;
	// 8224FDA0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FDA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8224FDA8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8224FDAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8224FDB0: 4E800421  bctrl
	ctx.lr = 0x8224FDB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8224FDB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224FDB8: 41820010  beq 0x8224fdc8
	if ctx.cr[0].eq {
	pc = 0x8224FDC8; continue 'dispatch;
	}
	// 8224FDBC: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 8224FDC0: 933F0050  stw r25, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8224FDC4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8224FDC8: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FDCC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8224FDD0: 41820010  beq 0x8224fde0
	if ctx.cr[0].eq {
	pc = 0x8224FDE0; continue 'dispatch;
	}
	// 8224FDD4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8224FDD8: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8224FDDC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8224FDE0: 83DC05B0  lwz r30, 0x5b0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8224FDE4: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 8224FDE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8224FDEC: 4BFC806D  bl 0x82217e58
	ctx.lr = 0x8224FDF0;
	sub_82217E58(ctx, base);
	// 8224FDF0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8224FDF4: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8224FDF8: 41820028  beq 0x8224fe20
	if ctx.cr[0].eq {
	pc = 0x8224FE20; continue 'dispatch;
	}
	// 8224FDFC: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 8224FE00: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8224FE04: 938B0104  stw r28, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[28].u32 ) };
	// 8224FE08: 9B2B0108  stb r25, 0x108(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[25].u8 ) };
	// 8224FE0C: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224FE10: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8224FE14: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8224FE18: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 8224FE1C: 48000008  b 0x8224fe24
	pc = 0x8224FE24; continue 'dispatch;
	// 8224FE20: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8224FE24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224FE28: 4BFF4759  bl 0x82244580
	ctx.lr = 0x8224FE2C;
	sub_82244580(ctx, base);
	// 8224FE2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8224FE30: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8224FE34: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8224FE38: 480052B9  bl 0x822550f0
	ctx.lr = 0x8224FE3C;
	sub_822550F0(ctx, base);
	// 8224FE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224FE40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8224FE44: 4BE3EC90  b 0x8208ead4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224FE48 size=164
    let mut pc: u32 = 0x8224FE48;
    'dispatch: loop {
        match pc {
            0x8224FE48 => {
    //   block [0x8224FE48..0x8224FEEC)
	// 8224FE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224FE4C: 4BE3EC49  bl 0x8208ea94
	ctx.lr = 0x8224FE50;
	sub_8208EA60(ctx, base);
	// 8224FE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224FE54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8224FE58: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8224FE5C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8224FE60: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8224FE64: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8224FE68: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FE6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224FE70: 40990074  ble cr6, 0x8224fee4
	if !ctx.cr[6].gt {
	pc = 0x8224FEE4; continue 'dispatch;
	}
	// 8224FE74: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8224FE78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FE7C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224FE80: 40980010  bge cr6, 0x8224fe90
	if !ctx.cr[6].lt {
	pc = 0x8224FE90; continue 'dispatch;
	}
	// 8224FE84: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224FE88: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8224FE8C: 4800000C  b 0x8224fe98
	pc = 0x8224FE98; continue 'dispatch;
	// 8224FE90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224FE94: 4BFFF2BD  bl 0x8224f150
	ctx.lr = 0x8224FE98;
	sub_8224F150(ctx, base);
	// 8224FE98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FE9C: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8224FEA0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8224FEA4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FEA8: 419A0018  beq cr6, 0x8224fec0
	if ctx.cr[6].eq {
	pc = 0x8224FEC0; continue 'dispatch;
	}
	// 8224FEAC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8224FEB0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8224FEB4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224FEB8: 4198FFC0  blt cr6, 0x8224fe78
	if ctx.cr[6].lt {
	pc = 0x8224FE78; continue 'dispatch;
	}
	// 8224FEBC: 48000028  b 0x8224fee4
	pc = 0x8224FEE4; continue 'dispatch;
	// 8224FEC0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224FEC4: 40980014  bge cr6, 0x8224fed8
	if !ctx.cr[6].lt {
	pc = 0x8224FED8; continue 'dispatch;
	}
	// 8224FEC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224FECC: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8224FED0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8224FED4: 4800000C  b 0x8224fee0
	pc = 0x8224FEE0; continue 'dispatch;
	// 8224FED8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224FEDC: 4BFFF275  bl 0x8224f150
	ctx.lr = 0x8224FEE0;
	sub_8224F150(ctx, base);
	// 8224FEE0: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8224FEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224FEE8: 4BE3EBFC  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224FEF0 size=132
    let mut pc: u32 = 0x8224FEF0;
    'dispatch: loop {
        match pc {
            0x8224FEF0 => {
    //   block [0x8224FEF0..0x8224FF74)
	// 8224FEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224FEF4: 4BE3EBA5  bl 0x8208ea98
	ctx.lr = 0x8224FEF8;
	sub_8208EA60(ctx, base);
	// 8224FEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224FEFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8224FF00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8224FF04: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8224FF08: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8224FF0C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FF10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224FF14: 40990058  ble cr6, 0x8224ff6c
	if !ctx.cr[6].gt {
	pc = 0x8224FF6C; continue 'dispatch;
	}
	// 8224FF18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8224FF1C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FF20: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224FF24: 40980010  bge cr6, 0x8224ff34
	if !ctx.cr[6].lt {
	pc = 0x8224FF34; continue 'dispatch;
	}
	// 8224FF28: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8224FF2C: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8224FF30: 4800000C  b 0x8224ff3c
	pc = 0x8224FF3C; continue 'dispatch;
	// 8224FF34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224FF38: 4BFFF219  bl 0x8224f150
	ctx.lr = 0x8224FF3C;
	sub_8224F150(ctx, base);
	// 8224FF3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8224FF40: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8224FF44: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8224FF48: 419A001C  beq cr6, 0x8224ff64
	if ctx.cr[6].eq {
	pc = 0x8224FF64; continue 'dispatch;
	}
	// 8224FF4C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8224FF50: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8224FF54: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8224FF58: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8224FF5C: 4198FFC0  blt cr6, 0x8224ff1c
	if ctx.cr[6].lt {
	pc = 0x8224FF1C; continue 'dispatch;
	}
	// 8224FF60: 4800000C  b 0x8224ff6c
	pc = 0x8224FF6C; continue 'dispatch;
	// 8224FF64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8224FF68: 4BFFF129  bl 0x8224f090
	ctx.lr = 0x8224FF6C;
	sub_8224F090(ctx, base);
	// 8224FF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8224FF70: 4BE3EB78  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8224FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8224FF78 size=440
    let mut pc: u32 = 0x8224FF78;
    'dispatch: loop {
        match pc {
            0x8224FF78 => {
    //   block [0x8224FF78..0x82250130)
	// 8224FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8224FF7C: 4BE3EB0D  bl 0x8208ea88
	ctx.lr = 0x8224FF80;
	sub_8208EA60(ctx, base);
	// 8224FF80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8224FF84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8224FF88: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8224FF8C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8224FF90: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8224FF94: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8224FF98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8224FF9C: 409A0020  bne cr6, 0x8224ffbc
	if !ctx.cr[6].eq {
	pc = 0x8224FFBC; continue 'dispatch;
	}
	// 8224FFA0: 817F05E4  lwz r11, 0x5e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1508 as u32) ) } as u64;
	// 8224FFA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224FFA8: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8224FFAC: 90BF05E4  stw r5, 0x5e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 8224FFB0: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 8224FFB4: 4BFF09DD  bl 0x82240990
	ctx.lr = 0x8224FFB8;
	sub_82240990(ctx, base);
	// 8224FFB8: 907D0034  stw r3, 0x34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 8224FFBC: 83FF05AC  lwz r31, 0x5ac(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8224FFC0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8224FFC4: 831D0034  lwz r24, 0x34(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8224FFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8224FFCC: 4BFC7E8D  bl 0x82217e58
	ctx.lr = 0x8224FFD0;
	sub_82217E58(ctx, base);
	// 8224FFD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8224FFD4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8224FFD8: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8224FFDC: 41820014  beq 0x8224fff0
	if ctx.cr[0].eq {
	pc = 0x8224FFF0; continue 'dispatch;
	}
	// 8224FFE0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8224FFE4: 48002725  bl 0x82252708
	ctx.lr = 0x8224FFE8;
	sub_82252708(ctx, base);
	// 8224FFE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8224FFEC: 48000008  b 0x8224fff4
	pc = 0x8224FFF4; continue 'dispatch;
	// 8224FFF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8224FFF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8224FFF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8224FFFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250000: 48001921  bl 0x82251920
	ctx.lr = 0x82250004;
	sub_82251920(ctx, base);
	// 82250004: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82250008: 93FD0030  stw r31, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 8225000C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82250010: 409A0008  bne cr6, 0x82250018
	if !ctx.cr[6].eq {
	pc = 0x82250018; continue 'dispatch;
	}
	// 82250014: 93FE0074  stw r31, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82250018: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8225001C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82250020: 409A0010  bne cr6, 0x82250030
	if !ctx.cr[6].eq {
	pc = 0x82250030; continue 'dispatch;
	}
	// 82250024: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82250028: 93FE0078  stw r31, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 8225002C: 997E007E  stb r11, 0x7e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(126 as u32), ctx.r[11].u8 ) };
	// 82250030: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82250034: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250038: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 8225003C: 3B8B4A58  addi r28, r11, 0x4a58
	ctx.r[28].s64 = ctx.r[11].s64 + 19032;
	// 82250040: 409A0020  bne cr6, 0x82250060
	if !ctx.cr[6].eq {
	pc = 0x82250060; continue 'dispatch;
	}
	// 82250044: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82250048: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8225004C: 93FE0070  stw r31, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82250050: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82250054: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82250058: 4BFFF0F9  bl 0x8224f150
	ctx.lr = 0x8225005C;
	sub_8224F150(ctx, base);
	// 8225005C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82250060: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82250064: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82250068: 409A0018  bne cr6, 0x82250080
	if !ctx.cr[6].eq {
	pc = 0x82250080; continue 'dispatch;
	}
	// 8225006C: 93FE006C  stw r31, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82250070: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82250074: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82250078: 4BFFF0D9  bl 0x8224f150
	ctx.lr = 0x8225007C;
	sub_8224F150(ctx, base);
	// 8225007C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82250080: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82250084: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8225008C: 48001895  bl 0x82251920
	ctx.lr = 0x82250090;
	sub_82251920(ctx, base);
	// 82250090: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250094: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250098: 4BFE1F41  bl 0x82231fd8
	ctx.lr = 0x8225009C;
	sub_82231FD8(ctx, base);
	// 8225009C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822500A0: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 822500A4: 4800446D  bl 0x82254510
	ctx.lr = 0x822500A8;
	sub_82254510(ctx, base);
	// 822500A8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822500AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 822500B0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 822500B4: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 822500B8: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 822500BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822500C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822500C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822500C8: 3B8A54C0  addi r28, r10, 0x54c0
	ctx.r[28].s64 = ctx.r[10].s64 + 21696;
	// 822500CC: 3BAB4AD4  addi r29, r11, 0x4ad4
	ctx.r[29].s64 = ctx.r[11].s64 + 19156;
	// 822500D0: 3B694AB4  addi r27, r9, 0x4ab4
	ctx.r[27].s64 = ctx.r[9].s64 + 19124;
	// 822500D4: 3B484A60  addi r26, r8, 0x4a60
	ctx.r[26].s64 = ctx.r[8].s64 + 19040;
	// 822500D8: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 822500DC: 7F3E58AE  lbzx r25, r30, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822500E0: 2B190008  cmplwi cr6, r25, 8
	ctx.cr[6].compare_u32(ctx.r[25].u32, 8 as u32, &mut ctx.xer);
	// 822500E4: 4198001C  blt cr6, 0x82250100
	if ctx.cr[6].lt {
	pc = 0x82250100; continue 'dispatch;
	}
	// 822500E8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 822500EC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822500F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822500F4: 38E00169  li r7, 0x169
	ctx.r[7].s64 = 361;
	// 822500F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822500FC: 4BF0590D  bl 0x82155a08
	ctx.lr = 0x82250100;
	sub_82155A08(ctx, base);
	// 82250100: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82250104: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82250108: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8225010C: 7D7E51AE  stbx r11, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82250110: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82250114: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82250118: 4198FFC0  blt cr6, 0x822500d8
	if ctx.cr[6].lt {
	pc = 0x822500D8; continue 'dispatch;
	}
	// 8225011C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250120: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82250124: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82250128: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8225012C: 4BE3E9AC  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250130 size=364
    let mut pc: u32 = 0x82250130;
    'dispatch: loop {
        match pc {
            0x82250130 => {
    //   block [0x82250130..0x8225029C)
	// 82250130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250134: 4BE3E955  bl 0x8208ea88
	ctx.lr = 0x82250138;
	sub_8208EA60(ctx, base);
	// 82250138: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225013C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250140: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82250144: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82250148: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8225014C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82250150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82250154: 409A0020  bne cr6, 0x82250174
	if !ctx.cr[6].eq {
	pc = 0x82250174; continue 'dispatch;
	}
	// 82250158: 817D05E4  lwz r11, 0x5e4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1508 as u32) ) } as u64;
	// 8225015C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250160: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82250164: 90BD05E4  stw r5, 0x5e4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 82250168: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 8225016C: 4BFF0825  bl 0x82240990
	ctx.lr = 0x82250170;
	sub_82240990(ctx, base);
	// 82250170: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82250174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250178: 4BFE1571  bl 0x822316e8
	ctx.lr = 0x8225017C;
	sub_822316E8(ctx, base);
	// 8225017C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82250180: 41820110  beq 0x82250290
	if ctx.cr[0].eq {
	pc = 0x82250290; continue 'dispatch;
	}
	// 82250184: 83BD05AC  lwz r29, 0x5ac(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250188: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225018C: 831F003C  lwz r24, 0x3c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82250190: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250194: 4BFC7CC5  bl 0x82217e58
	ctx.lr = 0x82250198;
	sub_82217E58(ctx, base);
	// 82250198: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8225019C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822501A0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822501A4: 41820010  beq 0x822501b4
	if ctx.cr[0].eq {
	pc = 0x822501B4; continue 'dispatch;
	}
	// 822501A8: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 822501AC: 4800255D  bl 0x82252708
	ctx.lr = 0x822501B0;
	sub_82252708(ctx, base);
	// 822501B0: 48000008  b 0x822501b8
	pc = 0x822501B8; continue 'dispatch;
	// 822501B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822501B8: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 822501BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822501C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822501C4: 4800175D  bl 0x82251920
	ctx.lr = 0x822501C8;
	sub_82251920(ctx, base);
	// 822501C8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 822501CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822501D0: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822501D4: 4800174D  bl 0x82251920
	ctx.lr = 0x822501D8;
	sub_82251920(ctx, base);
	// 822501D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822501DC: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822501E0: 4BFE1DF9  bl 0x82231fd8
	ctx.lr = 0x822501E4;
	sub_82231FD8(ctx, base);
	// 822501E4: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822501E8: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 822501EC: 480042DD  bl 0x822544c8
	ctx.lr = 0x822501F0;
	sub_822544C8(ctx, base);
	// 822501F0: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 822501F4: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822501F8: 83DF0038  lwz r30, 0x38(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822501FC: 4BFFEF55  bl 0x8224f150
	ctx.lr = 0x82250200;
	sub_8224F150(ctx, base);
	// 82250200: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250204: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82250208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8225020C: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82250210: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82250214: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82250218: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8225021C: 816B4A58  lwz r11, 0x4a58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19032 as u32) ) } as u64;
	// 82250220: 3B8A54C0  addi r28, r10, 0x54c0
	ctx.r[28].s64 = ctx.r[10].s64 + 21696;
	// 82250224: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82250228: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225022C: 3BAB4AD4  addi r29, r11, 0x4ad4
	ctx.r[29].s64 = ctx.r[11].s64 + 19156;
	// 82250230: 3B694AB4  addi r27, r9, 0x4ab4
	ctx.r[27].s64 = ctx.r[9].s64 + 19124;
	// 82250234: 3B484A60  addi r26, r8, 0x4a60
	ctx.r[26].s64 = ctx.r[8].s64 + 19040;
	// 82250238: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8225023C: 7F3E58AE  lbzx r25, r30, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82250240: 2B190008  cmplwi cr6, r25, 8
	ctx.cr[6].compare_u32(ctx.r[25].u32, 8 as u32, &mut ctx.xer);
	// 82250244: 4198001C  blt cr6, 0x82250260
	if ctx.cr[6].lt {
	pc = 0x82250260; continue 'dispatch;
	}
	// 82250248: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8225024C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82250250: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82250254: 38E00169  li r7, 0x169
	ctx.r[7].s64 = 361;
	// 82250258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8225025C: 4BF057AD  bl 0x82155a08
	ctx.lr = 0x82250260;
	sub_82155A08(ctx, base);
	// 82250260: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82250264: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82250268: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8225026C: 7D7E51AE  stbx r11, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82250270: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82250274: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82250278: 4198FFC0  blt cr6, 0x82250238
	if ctx.cr[6].lt {
	pc = 0x82250238; continue 'dispatch;
	}
	// 8225027C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82250280: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82250284: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250288: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8225028C: 48000008  b 0x82250294
	pc = 0x82250294; continue 'dispatch;
	// 82250290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250294: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82250298: 4BE3E840  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822502A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822502A0 size=80
    let mut pc: u32 = 0x822502A0;
    'dispatch: loop {
        match pc {
            0x822502A0 => {
    //   block [0x822502A0..0x822502F0)
	// 822502A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822502A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822502A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822502AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822502B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822502B4: 83E3002C  lwz r31, 0x2c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 822502B8: 396B4AF4  addi r11, r11, 0x4af4
	ctx.r[11].s64 = ctx.r[11].s64 + 19188;
	// 822502BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822502C0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822502C4: 419A0018  beq cr6, 0x822502dc
	if ctx.cr[6].eq {
	pc = 0x822502DC; continue 'dispatch;
	}
	// 822502C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822502CC: 4BFFF66D  bl 0x8224f938
	ctx.lr = 0x822502D0;
	sub_8224F938(ctx, base);
	// 822502D0: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 822502D4: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 822502D8: 4BFC7CB1  bl 0x82217f88
	ctx.lr = 0x822502DC;
	sub_82217F88(ctx, base);
	// 822502DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822502E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822502E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822502E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822502EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822502F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822502F0 size=316
    let mut pc: u32 = 0x822502F0;
    'dispatch: loop {
        match pc {
            0x822502F0 => {
    //   block [0x822502F0..0x8225042C)
	// 822502F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822502F4: 4BE3E7A5  bl 0x8208ea98
	ctx.lr = 0x822502F8;
	sub_8208EA60(ctx, base);
	// 822502F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822502FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82250300: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82250304: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82250308: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8225030C: 4BFFF685  bl 0x8224f990
	ctx.lr = 0x82250310;
	sub_8224F990(ctx, base);
	// 82250310: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250314: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82250318: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 8225031C: 396B4B18  addi r11, r11, 0x4b18
	ctx.r[11].s64 = ctx.r[11].s64 + 19224;
	// 82250320: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250324: 817F0588  lwz r11, 0x588(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82250328: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8225032C: 917F0588  stw r11, 0x588(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1416 as u32), ctx.r[11].u32 ) };
	// 82250330: 419A005C  beq cr6, 0x8225038c
	if ctx.cr[6].eq {
	pc = 0x8225038C; continue 'dispatch;
	}
	// 82250334: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 82250338: 419A0030  beq cr6, 0x82250368
	if ctx.cr[6].eq {
	pc = 0x82250368; continue 'dispatch;
	}
	// 8225033C: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 82250340: 409A0070  bne cr6, 0x822503b0
	if !ctx.cr[6].eq {
	pc = 0x822503B0; continue 'dispatch;
	}
	// 82250344: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 82250348: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8225034C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82250350: 41980060  blt cr6, 0x822503b0
	if ctx.cr[6].lt {
	pc = 0x822503B0; continue 'dispatch;
	}
	// 82250354: 3960001E  li r11, 0x1e
	ctx.r[11].s64 = 30;
	// 82250358: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8225035C: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 82250360: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82250364: 480149CD  bl 0x82264d30
	ctx.lr = 0x82250368;
	sub_82264D30(ctx, base);
	// 82250368: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 8225036C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82250370: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82250374: 4198003C  blt cr6, 0x822503b0
	if ctx.cr[6].lt {
	pc = 0x822503B0; continue 'dispatch;
	}
	// 82250378: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 8225037C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250380: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 82250384: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82250388: 480149A9  bl 0x82264d30
	ctx.lr = 0x8225038C;
	sub_82264D30(ctx, base);
	// 8225038C: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 82250390: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82250394: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82250398: 41980018  blt cr6, 0x822503b0
	if ctx.cr[6].lt {
	pc = 0x822503B0; continue 'dispatch;
	}
	// 8225039C: 3960001D  li r11, 0x1d
	ctx.r[11].s64 = 29;
	// 822503A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822503A4: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 822503A8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 822503AC: 48014985  bl 0x82264d30
	ctx.lr = 0x822503B0;
	sub_82264D30(ctx, base);
	// 822503B0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822503B4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822503B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822503BC: 4BFC7A9D  bl 0x82217e58
	ctx.lr = 0x822503C0;
	sub_82217E58(ctx, base);
	// 822503C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822503C4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822503C8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822503CC: 41820018  beq 0x822503e4
	if ctx.cr[0].eq {
	pc = 0x822503E4; continue 'dispatch;
	}
	// 822503D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822503D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822503D8: 48002541  bl 0x82252918
	ctx.lr = 0x822503DC;
	sub_82252918(ctx, base);
	// 822503DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822503E0: 48000008  b 0x822503e8
	pc = 0x822503E8; continue 'dispatch;
	// 822503E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822503E8: 817F0AB0  lwz r11, 0xab0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 822503EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822503F0: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 822503F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822503F8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 822503FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82250400: 4E800421  bctrl
	ctx.lr = 0x82250404;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82250404: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82250408: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8225040C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250410: 48001511  bl 0x82251920
	ctx.lr = 0x82250414;
	sub_82251920(ctx, base);
	// 82250414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8225041C: 4BFFF685  bl 0x8224faa0
	ctx.lr = 0x82250420;
	sub_8224FAA0(ctx, base);
	// 82250420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82250428: 4BE3E6C0  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250430 size=96
    let mut pc: u32 = 0x82250430;
    'dispatch: loop {
        match pc {
            0x82250430 => {
    //   block [0x82250430..0x82250490)
	// 82250430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82250438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225043C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82250440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250448: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8225044C: 4BFFF545  bl 0x8224f990
	ctx.lr = 0x82250450;
	sub_8224F990(ctx, base);
	// 82250450: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250458: 396B4BE8  addi r11, r11, 0x4be8
	ctx.r[11].s64 = ctx.r[11].s64 + 19432;
	// 8225045C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250460: 817E0584  lwz r11, 0x584(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82250464: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82250468: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8225046C: 817E0584  lwz r11, 0x584(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82250470: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82250474: 917E0584  stw r11, 0x584(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1412 as u32), ctx.r[11].u32 ) };
	// 82250478: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82250480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82250484: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82250488: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225048C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250490 size=68
    let mut pc: u32 = 0x82250490;
    'dispatch: loop {
        match pc {
            0x82250490 => {
    //   block [0x82250490..0x822504D4)
	// 82250490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82250498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8225049C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822504A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822504A4: 4BFFF4ED  bl 0x8224f990
	ctx.lr = 0x822504A8;
	sub_8224F990(ctx, base);
	// 822504A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822504AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822504B0: 396B4B3C  addi r11, r11, 0x4b3c
	ctx.r[11].s64 = ctx.r[11].s64 + 19260;
	// 822504B4: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 822504B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822504BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822504C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822504C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822504C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822504CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822504D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822504D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822504D8 size=216
    let mut pc: u32 = 0x822504D8;
    'dispatch: loop {
        match pc {
            0x822504D8 => {
    //   block [0x822504D8..0x822505B0)
	// 822504D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822504DC: 4BE3E5BD  bl 0x8208ea98
	ctx.lr = 0x822504E0;
	sub_8208EA60(ctx, base);
	// 822504E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822504E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822504E8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822504EC: 4BFFF4A5  bl 0x8224f990
	ctx.lr = 0x822504F0;
	sub_8224F990(ctx, base);
	// 822504F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822504F4: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822504F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822504FC: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82250500: 396B4B60  addi r11, r11, 0x4b60
	ctx.r[11].s64 = ctx.r[11].s64 + 19296;
	// 82250504: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82250508: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8225050C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250510: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82250514: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82250518: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 8225051C: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82250520: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 82250524: 4BFFCD5D  bl 0x8224d280
	ctx.lr = 0x82250528;
	sub_8224D280(ctx, base);
	// 82250528: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8225052C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82250530: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82250534: 897E0564  lbz r11, 0x564(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1380 as u32) ) } as u64;
	// 82250538: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8225053C: 40820068  bne 0x822505a4
	if !ctx.cr[0].eq {
	pc = 0x822505A4; continue 'dispatch;
	}
	// 82250540: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250544: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82250548: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8225054C: 4BFC790D  bl 0x82217e58
	ctx.lr = 0x82250550;
	sub_82217E58(ctx, base);
	// 82250550: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250554: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82250558: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8225055C: 41820010  beq 0x8225056c
	if ctx.cr[0].eq {
	pc = 0x8225056C; continue 'dispatch;
	}
	// 82250560: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250564: 48002505  bl 0x82252a68
	ctx.lr = 0x82250568;
	sub_82252A68(ctx, base);
	// 82250568: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8225056C: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82250570: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250574: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250578: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8225057C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82250580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82250584: 4E800421  bctrl
	ctx.lr = 0x82250588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82250588: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8225058C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250590: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250594: 4800138D  bl 0x82251920
	ctx.lr = 0x82250598;
	sub_82251920(ctx, base);
	// 82250598: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8225059C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822505A0: 4BFFF501  bl 0x8224faa0
	ctx.lr = 0x822505A4;
	sub_8224FAA0(ctx, base);
	// 822505A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822505A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822505AC: 4BE3E53C  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822505B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822505B0 size=268
    let mut pc: u32 = 0x822505B0;
    'dispatch: loop {
        match pc {
            0x822505B0 => {
    //   block [0x822505B0..0x822506BC)
	// 822505B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822505B4: 4BE3E4E1  bl 0x8208ea94
	ctx.lr = 0x822505B8;
	sub_8208EA60(ctx, base);
	// 822505B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822505BC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 822505C0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822505C4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822505C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822505CC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822505D0: 4BFFF3C1  bl 0x8224f990
	ctx.lr = 0x822505D4;
	sub_8224F990(ctx, base);
	// 822505D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822505D8: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822505DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822505E0: 396B4B60  addi r11, r11, 0x4b60
	ctx.r[11].s64 = ctx.r[11].s64 + 19296;
	// 822505E4: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 822505E8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822505EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822505F0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 822505F4: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 822505F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822505FC: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82250600: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82250604: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82250608: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 8225060C: 4BFFCD65  bl 0x8224d370
	ctx.lr = 0x82250610;
	sub_8224D370(ctx, base);
	// 82250610: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82250614: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82250618: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 8225061C: 897E0564  lbz r11, 0x564(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1380 as u32) ) } as u64;
	// 82250620: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82250624: 41820028  beq 0x8225064c
	if ctx.cr[0].eq {
	pc = 0x8225064C; continue 'dispatch;
	}
	// 82250628: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225062C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82250630: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82250634: 38CB4A60  addi r6, r11, 0x4a60
	ctx.r[6].s64 = ctx.r[11].s64 + 19040;
	// 82250638: 38AA4B84  addi r5, r10, 0x4b84
	ctx.r[5].s64 = ctx.r[10].s64 + 19332;
	// 8225063C: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 82250640: 38E001F1  li r7, 0x1f1
	ctx.r[7].s64 = 497;
	// 82250644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82250648: 4BF053C1  bl 0x82155a08
	ctx.lr = 0x8225064C;
	sub_82155A08(ctx, base);
	// 8225064C: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250650: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82250654: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82250658: 4BFC7801  bl 0x82217e58
	ctx.lr = 0x8225065C;
	sub_82217E58(ctx, base);
	// 8225065C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250660: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82250664: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82250668: 41820010  beq 0x82250678
	if ctx.cr[0].eq {
	pc = 0x82250678; continue 'dispatch;
	}
	// 8225066C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250670: 480023F9  bl 0x82252a68
	ctx.lr = 0x82250674;
	sub_82252A68(ctx, base);
	// 82250674: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82250678: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 8225067C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250680: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82250688: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8225068C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82250690: 4E800421  bctrl
	ctx.lr = 0x82250694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82250694: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82250698: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8225069C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822506A0: 48001281  bl 0x82251920
	ctx.lr = 0x822506A4;
	sub_82251920(ctx, base);
	// 822506A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822506A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822506AC: 4BFFF3F5  bl 0x8224faa0
	ctx.lr = 0x822506B0;
	sub_8224FAA0(ctx, base);
	// 822506B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822506B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822506B8: 4BE3E42C  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822506C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822506C0 size=84
    let mut pc: u32 = 0x822506C0;
    'dispatch: loop {
        match pc {
            0x822506C0 => {
    //   block [0x822506C0..0x82250714)
	// 822506C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822506C4: 4BE3E3D9  bl 0x8208ea9c
	ctx.lr = 0x822506C8;
	sub_8208EA60(ctx, base);
	// 822506C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822506CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822506D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822506D4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822506D8: 4BFFF2B9  bl 0x8224f990
	ctx.lr = 0x822506DC;
	sub_8224F990(ctx, base);
	// 822506DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822506E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822506E4: 396B4AF4  addi r11, r11, 0x4af4
	ctx.r[11].s64 = ctx.r[11].s64 + 19188;
	// 822506E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822506EC: 807D0AB0  lwz r3, 0xab0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2736 as u32) ) } as u64;
	// 822506F0: 4BFF28E1  bl 0x82242fd0
	ctx.lr = 0x822506F4;
	sub_82242FD0(ctx, base);
	// 822506F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822506F8: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 822506FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250700: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82250704: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 82250708: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8225070C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82250710: 4BE3E3DC  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250718 size=164
    let mut pc: u32 = 0x82250718;
    'dispatch: loop {
        match pc {
            0x82250718 => {
    //   block [0x82250718..0x822507BC)
	// 82250718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225071C: 4BE3E379  bl 0x8208ea94
	ctx.lr = 0x82250720;
	sub_8208EA60(ctx, base);
	// 82250720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250728: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8225072C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82250730: 4BFFF261  bl 0x8224f990
	ctx.lr = 0x82250734;
	sub_8224F990(ctx, base);
	// 82250734: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250738: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8225073C: 392B4BC4  addi r9, r11, 0x4bc4
	ctx.r[9].s64 = ctx.r[11].s64 + 19396;
	// 82250740: 61488000  ori r8, r10, 0x8000
	ctx.r[8].u64 = ctx.r[10].u64 | 32768;
	// 82250744: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82250748: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8225074C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82250750: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82250754: 3B8B54C0  addi r28, r11, 0x54c0
	ctx.r[28].s64 = ctx.r[11].s64 + 21696;
	// 82250758: 3B6A4A60  addi r27, r10, 0x4a60
	ctx.r[27].s64 = ctx.r[10].s64 + 19040;
	// 8225075C: 41980020  blt cr6, 0x8225077c
	if ctx.cr[6].lt {
	pc = 0x8225077C; continue 'dispatch;
	}
	// 82250760: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250764: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82250768: 38AB4BB4  addi r5, r11, 0x4bb4
	ctx.r[5].s64 = ctx.r[11].s64 + 19380;
	// 8225076C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82250770: 38E00241  li r7, 0x241
	ctx.r[7].s64 = 577;
	// 82250774: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82250778: 4BF05291  bl 0x82155a08
	ctx.lr = 0x8225077C;
	sub_82155A08(ctx, base);
	// 8225077C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82250780: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82250784: 41980020  blt cr6, 0x822507a4
	if ctx.cr[6].lt {
	pc = 0x822507A4; continue 'dispatch;
	}
	// 82250788: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225078C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82250790: 38AB4BA0  addi r5, r11, 0x4ba0
	ctx.r[5].s64 = ctx.r[11].s64 + 19360;
	// 82250794: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82250798: 38E00242  li r7, 0x242
	ctx.r[7].s64 = 578;
	// 8225079C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822507A0: 4BF05269  bl 0x82155a08
	ctx.lr = 0x822507A4;
	sub_82155A08(ctx, base);
	// 822507A4: 57CB801E  slwi r11, r30, 0x10
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822507A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822507AC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 822507B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822507B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822507B8: 4BE3E32C  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822507C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822507C0 size=1104
    let mut pc: u32 = 0x822507C0;
    'dispatch: loop {
        match pc {
            0x822507C0 => {
    //   block [0x822507C0..0x82250C10)
	// 822507C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822507C4: 4BE3E2B9  bl 0x8208ea7c
	ctx.lr = 0x822507C8;
	sub_8208EA60(ctx, base);
	// 822507C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822507CC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 822507D0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822507D4: 4BFFFC5D  bl 0x82250430
	ctx.lr = 0x822507D8;
	sub_82250430(ctx, base);
	// 822507D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822507DC: 3D408222  lis r10, -0x7dde
	ctx.r[10].s64 = -2111700992;
	// 822507E0: 396B4BE8  addi r11, r11, 0x4be8
	ctx.r[11].s64 = ctx.r[11].s64 + 19432;
	// 822507E4: 3BAA8930  addi r29, r10, -0x76d0
	ctx.r[29].s64 = ctx.r[10].s64 + -30416;
	// 822507E8: 91760000  stw r11, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822507EC: 807E05D0  lwz r3, 0x5d0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1488 as u32) ) } as u64;
	// 822507F0: 83FE0AB0  lwz r31, 0xab0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 822507F4: 839E0600  lwz r28, 0x600(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1536 as u32) ) } as u64;
	// 822507F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822507FC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82250800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82250804: 4E800421  bctrl
	ctx.lr = 0x82250808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82250808: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8225080C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250810: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82250814: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82250818: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8225081C: 4BFDD295  bl 0x8222dab0
	ctx.lr = 0x82250820;
	sub_8222DAB0(ctx, base);
	// 82250820: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82250824: 408200BC  bne 0x822508e0
	if !ctx.cr[0].eq {
	pc = 0x822508E0; continue 'dispatch;
	}
	// 82250828: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225082C: 80AB4CB0  lwz r5, 0x4cb0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19632 as u32) ) } as u64;
	// 82250830: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82250834: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82250838: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8225083C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82250840: 409A007C  bne cr6, 0x822508bc
	if !ctx.cr[6].eq {
	pc = 0x822508BC; continue 'dispatch;
	}
	// 82250844: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82250848: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8225084C: 4BFF0995  bl 0x822411e0
	ctx.lr = 0x82250850;
	sub_822411E0(ctx, base);
	// 82250850: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250854: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82250858: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225085C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250860: 4BFC75F9  bl 0x82217e58
	ctx.lr = 0x82250864;
	sub_82217E58(ctx, base);
	// 82250864: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250868: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225086C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82250870: 41820018  beq 0x82250888
	if ctx.cr[0].eq {
	pc = 0x82250888; continue 'dispatch;
	}
	// 82250874: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82250878: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8225087C: 4800217D  bl 0x822529f8
	ctx.lr = 0x82250880;
	sub_822529F8(ctx, base);
	// 82250880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82250884: 48000008  b 0x8225088c
	pc = 0x8225088C; continue 'dispatch;
	// 82250888: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8225088C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250890: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250894: 48003C7D  bl 0x82254510
	ctx.lr = 0x82250898;
	sub_82254510(ctx, base);
	// 82250898: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8225089C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822508A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822508A4: 4800107D  bl 0x82251920
	ctx.lr = 0x822508A8;
	sub_82251920(ctx, base);
	// 822508A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822508AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822508B0: 4BFFF1F1  bl 0x8224faa0
	ctx.lr = 0x822508B4;
	sub_8224FAA0(ctx, base);
	// 822508B4: 7FBBF92E  stwx r29, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 822508B8: 48000008  b 0x822508c0
	pc = 0x822508C0; continue 'dispatch;
	// 822508BC: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822508C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822508C4: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 822508C8: 480030B9  bl 0x82253980
	ctx.lr = 0x822508CC;
	sub_82253980(ctx, base);
	// 822508CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822508D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822508D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822508D8: 48001049  bl 0x82251920
	ctx.lr = 0x822508DC;
	sub_82251920(ctx, base);
	// 822508DC: 48000300  b 0x82250bdc
	pc = 0x82250BDC; continue 'dispatch;
	// 822508E0: 3D608222  lis r11, -0x7dde
	ctx.r[11].s64 = -2111700992;
	// 822508E4: 807E0600  lwz r3, 0x600(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1536 as u32) ) } as u64;
	// 822508E8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822508EC: 38AB8930  addi r5, r11, -0x76d0
	ctx.r[5].s64 = ctx.r[11].s64 + -30416;
	// 822508F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822508F4: 4BFDD755  bl 0x8222e048
	ctx.lr = 0x822508F8;
	sub_8222E048(ctx, base);
	// 822508F8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822508FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250900: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250904: 3AAB4D58  addi r21, r11, 0x4d58
	ctx.r[21].s64 = ctx.r[11].s64 + 19800;
	// 82250908: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 8225090C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82250910: 419801EC  blt cr6, 0x82250afc
	if ctx.cr[6].lt {
	pc = 0x82250AFC; continue 'dispatch;
	}
	// 82250914: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250918: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8225091C: 3B6B4E88  addi r27, r11, 0x4e88
	ctx.r[27].s64 = ctx.r[11].s64 + 20104;
	// 82250920: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250924: 393B0008  addi r9, r27, 8
	ctx.r[9].s64 = ctx.r[27].s64 + 8;
	// 82250928: 396B4CB0  addi r11, r11, 0x4cb0
	ctx.r[11].s64 = ctx.r[11].s64 + 19632;
	// 8225092C: 7D0AD82E  lwzx r8, r10, r27
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82250930: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82250934: 55091838  slwi r9, r8, 3
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82250938: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8225093C: 7CA9582E  lwzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82250940: 7F4A582E  lwzx r26, r10, r11
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82250944: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82250948: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8225094C: 397A0006  addi r11, r26, 6
	ctx.r[11].s64 = ctx.r[26].s64 + 6;
	// 82250950: 5577103A  slwi r23, r11, 2
	ctx.r[23].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82250954: 7D7CF82E  lwzx r11, r28, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82250958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8225095C: 7F17F82E  lwzx r24, r23, r31
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82250960: 409A007C  bne cr6, 0x822509dc
	if !ctx.cr[6].eq {
	pc = 0x822509DC; continue 'dispatch;
	}
	// 82250964: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82250968: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8225096C: 4BFF0875  bl 0x822411e0
	ctx.lr = 0x82250970;
	sub_822411E0(ctx, base);
	// 82250970: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250974: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82250978: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225097C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250980: 4BFC74D9  bl 0x82217e58
	ctx.lr = 0x82250984;
	sub_82217E58(ctx, base);
	// 82250984: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250988: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225098C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82250990: 41820018  beq 0x822509a8
	if ctx.cr[0].eq {
	pc = 0x822509A8; continue 'dispatch;
	}
	// 82250994: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82250998: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8225099C: 4800205D  bl 0x822529f8
	ctx.lr = 0x822509A0;
	sub_822529F8(ctx, base);
	// 822509A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822509A4: 48000008  b 0x822509ac
	pc = 0x822509AC; continue 'dispatch;
	// 822509A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822509AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822509B0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 822509B4: 48003B5D  bl 0x82254510
	ctx.lr = 0x822509B8;
	sub_82254510(ctx, base);
	// 822509B8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822509BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822509C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822509C4: 48000F5D  bl 0x82251920
	ctx.lr = 0x822509C8;
	sub_82251920(ctx, base);
	// 822509C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822509CC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822509D0: 4BFFF0D1  bl 0x8224faa0
	ctx.lr = 0x822509D4;
	sub_8224FAA0(ctx, base);
	// 822509D4: 7FBCF92E  stwx r29, r28, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 822509D8: 48000008  b 0x822509e0
	pc = 0x822509E0; continue 'dispatch;
	// 822509DC: 832B001C  lwz r25, 0x1c(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822509E0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 822509E4: 409A0080  bne cr6, 0x82250a64
	if !ctx.cr[6].eq {
	pc = 0x82250A64; continue 'dispatch;
	}
	// 822509E8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822509EC: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822509F0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 822509F4: 4BFF07ED  bl 0x822411e0
	ctx.lr = 0x822509F8;
	sub_822411E0(ctx, base);
	// 822509F8: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822509FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82250A00: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82250A04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250A08: 4BFC7451  bl 0x82217e58
	ctx.lr = 0x82250A0C;
	sub_82217E58(ctx, base);
	// 82250A0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250A10: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82250A14: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82250A18: 41820018  beq 0x82250a30
	if ctx.cr[0].eq {
	pc = 0x82250A30; continue 'dispatch;
	}
	// 82250A1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82250A20: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82250A24: 48001FD5  bl 0x822529f8
	ctx.lr = 0x82250A28;
	sub_822529F8(ctx, base);
	// 82250A28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82250A2C: 48000008  b 0x82250a34
	pc = 0x82250A34; continue 'dispatch;
	// 82250A30: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82250A34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250A38: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250A3C: 48003AD5  bl 0x82254510
	ctx.lr = 0x82250A40;
	sub_82254510(ctx, base);
	// 82250A40: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82250A44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250A48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250A4C: 48000ED5  bl 0x82251920
	ctx.lr = 0x82250A50;
	sub_82251920(ctx, base);
	// 82250A50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250A54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82250A58: 4BFFF049  bl 0x8224faa0
	ctx.lr = 0x82250A5C;
	sub_8224FAA0(ctx, base);
	// 82250A5C: 7FB7F92E  stwx r29, r23, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82250A60: 48000008  b 0x82250a68
	pc = 0x82250A68; continue 'dispatch;
	// 82250A64: 8398001C  lwz r28, 0x1c(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 82250A68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250A6C: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82250A70: 48002F11  bl 0x82253980
	ctx.lr = 0x82250A74;
	sub_82253980(ctx, base);
	// 82250A74: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82250A78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250A7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82250A80: 48000EA1  bl 0x82251920
	ctx.lr = 0x82250A84;
	sub_82251920(ctx, base);
	// 82250A84: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250A88: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 82250A8C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250A90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250A94: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82250A98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250A9C: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82250AA0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250AA4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82250AA8: 917D0084  stw r11, 0x84(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82250AAC: 48000E75  bl 0x82251920
	ctx.lr = 0x82250AB0;
	sub_82251920(ctx, base);
	// 82250AB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250AB4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82250AB8: 4BFFEFE9  bl 0x8224faa0
	ctx.lr = 0x82250ABC;
	sub_8224FAA0(ctx, base);
	// 82250ABC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250AC0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250AC4: 48003A05  bl 0x822544c8
	ctx.lr = 0x82250AC8;
	sub_822544C8(ctx, base);
	// 82250AC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250ACC: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82250AD0: 48002EB1  bl 0x82253980
	ctx.lr = 0x82250AD4;
	sub_82253980(ctx, base);
	// 82250AD4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82250AD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250ADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82250AE0: 48000E41  bl 0x82251920
	ctx.lr = 0x82250AE4;
	sub_82251920(ctx, base);
	// 82250AE4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250AE8: 397B000C  addi r11, r27, 0xc
	ctx.r[11].s64 = ctx.r[27].s64 + 12;
	// 82250AEC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250AF0: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82250AF4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250AF8: 480000DC  b 0x82250bd4
	pc = 0x82250BD4; continue 'dispatch;
	// 82250AFC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250B00: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250B04: 3B4B4EA8  addi r26, r11, 0x4ea8
	ctx.r[26].s64 = ctx.r[11].s64 + 20136;
	// 82250B08: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250B0C: 396B4CB0  addi r11, r11, 0x4cb0
	ctx.r[11].s64 = ctx.r[11].s64 + 19632;
	// 82250B10: 7D4AD02E  lwzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82250B14: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250B18: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82250B1C: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82250B20: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82250B24: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82250B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82250B2C: 409A007C  bne cr6, 0x82250ba8
	if !ctx.cr[6].eq {
	pc = 0x82250BA8; continue 'dispatch;
	}
	// 82250B30: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82250B34: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82250B38: 4BFF06A9  bl 0x822411e0
	ctx.lr = 0x82250B3C;
	sub_822411E0(ctx, base);
	// 82250B3C: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250B40: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82250B44: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82250B48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250B4C: 4BFC730D  bl 0x82217e58
	ctx.lr = 0x82250B50;
	sub_82217E58(ctx, base);
	// 82250B50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250B54: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82250B58: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82250B5C: 41820018  beq 0x82250b74
	if ctx.cr[0].eq {
	pc = 0x82250B74; continue 'dispatch;
	}
	// 82250B60: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82250B64: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82250B68: 48001E91  bl 0x822529f8
	ctx.lr = 0x82250B6C;
	sub_822529F8(ctx, base);
	// 82250B6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82250B70: 48000008  b 0x82250b78
	pc = 0x82250B78; continue 'dispatch;
	// 82250B74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82250B78: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250B7C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250B80: 48003991  bl 0x82254510
	ctx.lr = 0x82250B84;
	sub_82254510(ctx, base);
	// 82250B84: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82250B88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250B8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250B90: 48000D91  bl 0x82251920
	ctx.lr = 0x82250B94;
	sub_82251920(ctx, base);
	// 82250B94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82250B98: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82250B9C: 4BFFEF05  bl 0x8224faa0
	ctx.lr = 0x82250BA0;
	sub_8224FAA0(ctx, base);
	// 82250BA0: 7FBBF92E  stwx r29, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82250BA4: 48000008  b 0x82250bac
	pc = 0x82250BAC; continue 'dispatch;
	// 82250BA8: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82250BAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250BB0: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82250BB4: 48002DCD  bl 0x82253980
	ctx.lr = 0x82250BB8;
	sub_82253980(ctx, base);
	// 82250BB8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82250BBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250BC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82250BC4: 48000D5D  bl 0x82251920
	ctx.lr = 0x82250BC8;
	sub_82251920(ctx, base);
	// 82250BC8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250BCC: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82250BD0: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82250BD4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82250BD8: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82250BDC: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82250BE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250BE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250BE8: 48000D39  bl 0x82251920
	ctx.lr = 0x82250BEC;
	sub_82251920(ctx, base);
	// 82250BEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250BF0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82250BF4: 4BFFEEAD  bl 0x8224faa0
	ctx.lr = 0x82250BF8;
	sub_8224FAA0(ctx, base);
	// 82250BF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250BFC: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250C00: 480038C9  bl 0x822544c8
	ctx.lr = 0x82250C04;
	sub_822544C8(ctx, base);
	// 82250C04: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82250C08: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82250C0C: 4BE3DEC0  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250C10 size=276
    let mut pc: u32 = 0x82250C10;
    'dispatch: loop {
        match pc {
            0x82250C10 => {
    //   block [0x82250C10..0x82250D24)
	// 82250C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250C14: 4BE3DE7D  bl 0x8208ea90
	ctx.lr = 0x82250C18;
	sub_8208EA60(ctx, base);
	// 82250C18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82250C20: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82250C24: 4BFFF80D  bl 0x82250430
	ctx.lr = 0x82250C28;
	sub_82250430(ctx, base);
	// 82250C28: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250C2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82250C30: 396B4BE8  addi r11, r11, 0x4be8
	ctx.r[11].s64 = ctx.r[11].s64 + 19432;
	// 82250C34: 394A4CB0  addi r10, r10, 0x4cb0
	ctx.r[10].s64 = ctx.r[10].s64 + 19632;
	// 82250C38: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250C3C: 80AA00A0  lwz r5, 0xa0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) } as u64;
	// 82250C40: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82250C44: 83BF0AB0  lwz r29, 0xab0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82250C48: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82250C4C: 7D7AE82E  lwzx r11, r26, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82250C50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82250C54: 409A007C  bne cr6, 0x82250cd0
	if !ctx.cr[6].eq {
	pc = 0x82250CD0; continue 'dispatch;
	}
	// 82250C58: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82250C5C: 807D00AC  lwz r3, 0xac(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(172 as u32) ) } as u64;
	// 82250C60: 4BFF0581  bl 0x822411e0
	ctx.lr = 0x82250C64;
	sub_822411E0(ctx, base);
	// 82250C64: 839F05AC  lwz r28, 0x5ac(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250C68: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82250C6C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82250C70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82250C74: 4BFC71E5  bl 0x82217e58
	ctx.lr = 0x82250C78;
	sub_82217E58(ctx, base);
	// 82250C78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250C7C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82250C80: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82250C84: 41820018  beq 0x82250c9c
	if ctx.cr[0].eq {
	pc = 0x82250C9C; continue 'dispatch;
	}
	// 82250C88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82250C8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250C90: 48001D69  bl 0x822529f8
	ctx.lr = 0x82250C94;
	sub_822529F8(ctx, base);
	// 82250C94: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82250C98: 48000008  b 0x82250ca0
	pc = 0x82250CA0; continue 'dispatch;
	// 82250C9C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82250CA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82250CA4: 807D00A4  lwz r3, 0xa4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250CA8: 48003869  bl 0x82254510
	ctx.lr = 0x82250CAC;
	sub_82254510(ctx, base);
	// 82250CAC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82250CB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250CB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82250CB8: 48000C69  bl 0x82251920
	ctx.lr = 0x82250CBC;
	sub_82251920(ctx, base);
	// 82250CBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82250CC0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82250CC4: 4BFFEDDD  bl 0x8224faa0
	ctx.lr = 0x82250CC8;
	sub_8224FAA0(ctx, base);
	// 82250CC8: 7F9AE92E  stwx r28, r26, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	// 82250CCC: 48000008  b 0x82250cd4
	pc = 0x82250CD4; continue 'dispatch;
	// 82250CD0: 836B001C  lwz r27, 0x1c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82250CD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250CD8: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82250CDC: 48002CA5  bl 0x82253980
	ctx.lr = 0x82250CE0;
	sub_82253980(ctx, base);
	// 82250CE0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82250CE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250CEC: 48000C35  bl 0x82251920
	ctx.lr = 0x82250CF0;
	sub_82251920(ctx, base);
	// 82250CF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250CF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82250CF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250CFC: 48000C25  bl 0x82251920
	ctx.lr = 0x82250D00;
	sub_82251920(ctx, base);
	// 82250D00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250D08: 4BFFED99  bl 0x8224faa0
	ctx.lr = 0x82250D0C;
	sub_8224FAA0(ctx, base);
	// 82250D0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250D10: 807D00A4  lwz r3, 0xa4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250D14: 480037B5  bl 0x822544c8
	ctx.lr = 0x82250D18;
	sub_822544C8(ctx, base);
	// 82250D18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250D1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82250D20: 4BE3DDC0  b 0x8208eae0
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250D28 size=516
    let mut pc: u32 = 0x82250D28;
    'dispatch: loop {
        match pc {
            0x82250D28 => {
    //   block [0x82250D28..0x82250F2C)
	// 82250D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250D2C: 4BE3DD51  bl 0x8208ea7c
	ctx.lr = 0x82250D30;
	sub_8208EA60(ctx, base);
	// 82250D30: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250D34: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82250D38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82250D3C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82250D40: 4BFFF6F1  bl 0x82250430
	ctx.lr = 0x82250D44;
	sub_82250430(ctx, base);
	// 82250D44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250D48: 3D408222  lis r10, -0x7dde
	ctx.r[10].s64 = -2111700992;
	// 82250D4C: 396B4BE8  addi r11, r11, 0x4be8
	ctx.r[11].s64 = ctx.r[11].s64 + 19432;
	// 82250D50: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82250D54: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250D58: 38AA8930  addi r5, r10, -0x76d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30416;
	// 82250D5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82250D60: 807F0600  lwz r3, 0x600(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82250D64: 833F0AB0  lwz r25, 0xab0(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82250D68: 4BFDD2E1  bl 0x8222e048
	ctx.lr = 0x82250D6C;
	sub_8222E048(ctx, base);
	// 82250D6C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250D70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82250D74: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82250D78: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82250D7C: 2F1C000C  cmpwi cr6, r28, 0xc
	ctx.cr[6].compare_i32(ctx.r[28].s32, 12, &mut ctx.xer);
	// 82250D80: 3B0B4EA8  addi r24, r11, 0x4ea8
	ctx.r[24].s64 = ctx.r[11].s64 + 20136;
	// 82250D84: 3AEA54C0  addi r23, r10, 0x54c0
	ctx.r[23].s64 = ctx.r[10].s64 + 21696;
	// 82250D88: 3AC9FE08  addi r22, r9, -0x1f8
	ctx.r[22].s64 = ctx.r[9].s64 + -504;
	// 82250D8C: 3AA84A60  addi r21, r8, 0x4a60
	ctx.r[21].s64 = ctx.r[8].s64 + 19040;
	// 82250D90: 419A0044  beq cr6, 0x82250dd4
	if ctx.cr[6].eq {
	pc = 0x82250DD4; continue 'dispatch;
	}
	// 82250D94: 2F1C000D  cmpwi cr6, r28, 0xd
	ctx.cr[6].compare_i32(ctx.r[28].s32, 13, &mut ctx.xer);
	// 82250D98: 419A0034  beq cr6, 0x82250dcc
	if ctx.cr[6].eq {
	pc = 0x82250DCC; continue 'dispatch;
	}
	// 82250D9C: 2F1C000E  cmpwi cr6, r28, 0xe
	ctx.cr[6].compare_i32(ctx.r[28].s32, 14, &mut ctx.xer);
	// 82250DA0: 419A0024  beq cr6, 0x82250dc4
	if ctx.cr[6].eq {
	pc = 0x82250DC4; continue 'dispatch;
	}
	// 82250DA4: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82250DA8: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82250DAC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82250DB0: 38E002F5  li r7, 0x2f5
	ctx.r[7].s64 = 757;
	// 82250DB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82250DB8: 4BF04C51  bl 0x82155a08
	ctx.lr = 0x82250DBC;
	sub_82155A08(ctx, base);
	// 82250DBC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250DC0: 48000024  b 0x82250de4
	pc = 0x82250DE4; continue 'dispatch;
	// 82250DC4: 39580008  addi r10, r24, 8
	ctx.r[10].s64 = ctx.r[24].s64 + 8;
	// 82250DC8: 48000010  b 0x82250dd8
	pc = 0x82250DD8; continue 'dispatch;
	// 82250DCC: 39580018  addi r10, r24, 0x18
	ctx.r[10].s64 = ctx.r[24].s64 + 24;
	// 82250DD0: 48000008  b 0x82250dd8
	pc = 0x82250DD8; continue 'dispatch;
	// 82250DD4: 39580010  addi r10, r24, 0x10
	ctx.r[10].s64 = ctx.r[24].s64 + 16;
	// 82250DD8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250DDC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82250DE0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82250DE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82250DE8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82250DEC: 394A4CB0  addi r10, r10, 0x4cb0
	ctx.r[10].s64 = ctx.r[10].s64 + 19632;
	// 82250DF0: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82250DF4: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82250DF8: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82250DFC: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82250E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82250E04: 409A007C  bne cr6, 0x82250e80
	if !ctx.cr[6].eq {
	pc = 0x82250E80; continue 'dispatch;
	}
	// 82250E08: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82250E0C: 807900AC  lwz r3, 0xac(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(172 as u32) ) } as u64;
	// 82250E10: 4BFF03D1  bl 0x822411e0
	ctx.lr = 0x82250E14;
	sub_822411E0(ctx, base);
	// 82250E14: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82250E18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82250E1C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82250E20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250E24: 4BFC7035  bl 0x82217e58
	ctx.lr = 0x82250E28;
	sub_82217E58(ctx, base);
	// 82250E28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82250E2C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82250E30: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82250E34: 41820018  beq 0x82250e4c
	if ctx.cr[0].eq {
	pc = 0x82250E4C; continue 'dispatch;
	}
	// 82250E38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82250E3C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82250E40: 48001BB9  bl 0x822529f8
	ctx.lr = 0x82250E44;
	sub_822529F8(ctx, base);
	// 82250E44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82250E48: 48000008  b 0x82250e50
	pc = 0x82250E50; continue 'dispatch;
	// 82250E4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82250E50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250E54: 807900A4  lwz r3, 0xa4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250E58: 480036B9  bl 0x82254510
	ctx.lr = 0x82250E5C;
	sub_82254510(ctx, base);
	// 82250E5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82250E60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250E64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82250E68: 48000AB9  bl 0x82251920
	ctx.lr = 0x82250E6C;
	sub_82251920(ctx, base);
	// 82250E6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82250E70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82250E74: 4BFFEC2D  bl 0x8224faa0
	ctx.lr = 0x82250E78;
	sub_8224FAA0(ctx, base);
	// 82250E78: 7FDAC92E  stwx r30, r26, r25
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32), ctx.r[30].u32) };
	// 82250E7C: 48000008  b 0x82250e84
	pc = 0x82250E84; continue 'dispatch;
	// 82250E80: 83AB001C  lwz r29, 0x1c(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82250E84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250E88: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82250E8C: 48002AF5  bl 0x82253980
	ctx.lr = 0x82250E90;
	sub_82253980(ctx, base);
	// 82250E90: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82250E94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82250E98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250E9C: 48000A85  bl 0x82251920
	ctx.lr = 0x82250EA0;
	sub_82251920(ctx, base);
	// 82250EA0: 2F1C000C  cmpwi cr6, r28, 0xc
	ctx.cr[6].compare_i32(ctx.r[28].s32, 12, &mut ctx.xer);
	// 82250EA4: 419A0040  beq cr6, 0x82250ee4
	if ctx.cr[6].eq {
	pc = 0x82250EE4; continue 'dispatch;
	}
	// 82250EA8: 2F1C000D  cmpwi cr6, r28, 0xd
	ctx.cr[6].compare_i32(ctx.r[28].s32, 13, &mut ctx.xer);
	// 82250EAC: 419A0030  beq cr6, 0x82250edc
	if ctx.cr[6].eq {
	pc = 0x82250EDC; continue 'dispatch;
	}
	// 82250EB0: 2F1C000E  cmpwi cr6, r28, 0xe
	ctx.cr[6].compare_i32(ctx.r[28].s32, 14, &mut ctx.xer);
	// 82250EB4: 419A0020  beq cr6, 0x82250ed4
	if ctx.cr[6].eq {
	pc = 0x82250ED4; continue 'dispatch;
	}
	// 82250EB8: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82250EBC: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82250EC0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82250EC4: 38E00314  li r7, 0x314
	ctx.r[7].s64 = 788;
	// 82250EC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82250ECC: 4BF04B3D  bl 0x82155a08
	ctx.lr = 0x82250ED0;
	sub_82155A08(ctx, base);
	// 82250ED0: 48000028  b 0x82250ef8
	pc = 0x82250EF8; continue 'dispatch;
	// 82250ED4: 3958000C  addi r10, r24, 0xc
	ctx.r[10].s64 = ctx.r[24].s64 + 12;
	// 82250ED8: 48000010  b 0x82250ee8
	pc = 0x82250EE8; continue 'dispatch;
	// 82250EDC: 3958001C  addi r10, r24, 0x1c
	ctx.r[10].s64 = ctx.r[24].s64 + 28;
	// 82250EE0: 48000008  b 0x82250ee8
	pc = 0x82250EE8; continue 'dispatch;
	// 82250EE4: 39580014  addi r10, r24, 0x14
	ctx.r[10].s64 = ctx.r[24].s64 + 20;
	// 82250EE8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82250EEC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82250EF0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82250EF4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82250EF8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82250EFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82250F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250F04: 48000A1D  bl 0x82251920
	ctx.lr = 0x82250F08;
	sub_82251920(ctx, base);
	// 82250F08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250F0C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82250F10: 4BFFEB91  bl 0x8224faa0
	ctx.lr = 0x82250F14;
	sub_8224FAA0(ctx, base);
	// 82250F14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82250F18: 807900A4  lwz r3, 0xa4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) } as u64;
	// 82250F1C: 480035AD  bl 0x822544c8
	ctx.lr = 0x82250F20;
	sub_822544C8(ctx, base);
	// 82250F20: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82250F24: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82250F28: 4BE3DBA4  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250F30 size=60
    let mut pc: u32 = 0x82250F30;
    'dispatch: loop {
        match pc {
            0x82250F30 => {
    //   block [0x82250F30..0x82250F6C)
	// 82250F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82250F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82250F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250F44: 4BFFF4ED  bl 0x82250430
	ctx.lr = 0x82250F48;
	sub_82250430(ctx, base);
	// 82250F48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250F50: 396B4BE8  addi r11, r11, 0x4be8
	ctx.r[11].s64 = ctx.r[11].s64 + 19432;
	// 82250F54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82250F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82250F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82250F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82250F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250F70 size=76
    let mut pc: u32 = 0x82250F70;
    'dispatch: loop {
        match pc {
            0x82250F70 => {
    //   block [0x82250F70..0x82250FBC)
	// 82250F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82250F78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82250F7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82250F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250F88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82250F8C: 4BFFEA05  bl 0x8224f990
	ctx.lr = 0x82250F90;
	sub_8224F990(ctx, base);
	// 82250F90: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250F94: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82250F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82250F9C: 396B4AF4  addi r11, r11, 0x4af4
	ctx.r[11].s64 = ctx.r[11].s64 + 19188;
	// 82250FA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250FA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82250FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82250FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82250FB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82250FB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82250FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82250FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82250FC0 size=132
    let mut pc: u32 = 0x82250FC0;
    'dispatch: loop {
        match pc {
            0x82250FC0 => {
    //   block [0x82250FC0..0x82251044)
	// 82250FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82250FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82250FC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82250FCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82250FD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82250FD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82250FD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82250FDC: 4BFFE9B5  bl 0x8224f990
	ctx.lr = 0x82250FE0;
	sub_8224F990(ctx, base);
	// 82250FE0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250FE4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82250FE8: 396B4AF4  addi r11, r11, 0x4af4
	ctx.r[11].s64 = ctx.r[11].s64 + 19188;
	// 82250FEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82250FF0: 419A0028  beq cr6, 0x82251018
	if ctx.cr[6].eq {
	pc = 0x82251018; continue 'dispatch;
	}
	// 82250FF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82250FF8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82250FFC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82251000: 38CB4A60  addi r6, r11, 0x4a60
	ctx.r[6].s64 = ctx.r[11].s64 + 19040;
	// 82251004: 38AA4C0C  addi r5, r10, 0x4c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 19468;
	// 82251008: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 8225100C: 38E00353  li r7, 0x353
	ctx.r[7].s64 = 851;
	// 82251010: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251014: 4BF049F5  bl 0x82155a08
	ctx.lr = 0x82251018;
	sub_82155A08(ctx, base);
	// 82251018: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8225101C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82251020: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82251024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251028: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8225102C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251038: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8225103C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82251040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251048 size=140
    let mut pc: u32 = 0x82251048;
    'dispatch: loop {
        match pc {
            0x82251048 => {
    //   block [0x82251048..0x822510D4)
	// 82251048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82251050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82251054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82251058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225105C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82251064: 4BFFE92D  bl 0x8224f990
	ctx.lr = 0x82251068;
	sub_8224F990(ctx, base);
	// 82251068: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225106C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82251070: 396B4AF4  addi r11, r11, 0x4af4
	ctx.r[11].s64 = ctx.r[11].s64 + 19188;
	// 82251074: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251078: 4198000C  blt cr6, 0x82251084
	if ctx.cr[6].lt {
	pc = 0x82251084; continue 'dispatch;
	}
	// 8225107C: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82251080: 40990028  ble cr6, 0x822510a8
	if !ctx.cr[6].gt {
	pc = 0x822510A8; continue 'dispatch;
	}
	// 82251084: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251088: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8225108C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82251090: 38CB4A60  addi r6, r11, 0x4a60
	ctx.r[6].s64 = ctx.r[11].s64 + 19040;
	// 82251094: 38AA4C18  addi r5, r10, 0x4c18
	ctx.r[5].s64 = ctx.r[10].s64 + 19480;
	// 82251098: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 8225109C: 38E0035E  li r7, 0x35e
	ctx.r[7].s64 = 862;
	// 822510A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822510A4: 4BF04965  bl 0x82155a08
	ctx.lr = 0x822510A8;
	sub_82155A08(ctx, base);
	// 822510A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822510AC: 395E0021  addi r10, r30, 0x21
	ctx.r[10].s64 = ctx.r[30].s64 + 33;
	// 822510B0: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 822510B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822510B8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822510BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822510C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822510C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822510C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822510CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822510D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822510D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822510D8 size=100
    let mut pc: u32 = 0x822510D8;
    'dispatch: loop {
        match pc {
            0x822510D8 => {
    //   block [0x822510D8..0x8225113C)
	// 822510D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822510DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822510E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822510E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822510E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822510EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822510F0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822510F4: 4BFFE89D  bl 0x8224f990
	ctx.lr = 0x822510F8;
	sub_8224F990(ctx, base);
	// 822510F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822510FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82251100: 396B4C38  addi r11, r11, 0x4c38
	ctx.r[11].s64 = ctx.r[11].s64 + 19512;
	// 82251104: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82251108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8225110C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251110: 817E058C  lwz r11, 0x58c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1420 as u32) ) } as u64;
	// 82251114: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82251118: 817E058C  lwz r11, 0x58c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1420 as u32) ) } as u64;
	// 8225111C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82251120: 917E058C  stw r11, 0x58c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1420 as u32), ctx.r[11].u32 ) };
	// 82251124: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8225112C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251130: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82251134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82251138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251140 size=100
    let mut pc: u32 = 0x82251140;
    'dispatch: loop {
        match pc {
            0x82251140 => {
    //   block [0x82251140..0x822511A4)
	// 82251140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82251148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225114C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82251150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251158: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8225115C: 4BFFE835  bl 0x8224f990
	ctx.lr = 0x82251160;
	sub_8224F990(ctx, base);
	// 82251160: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251164: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82251168: 396B4C38  addi r11, r11, 0x4c38
	ctx.r[11].s64 = ctx.r[11].s64 + 19512;
	// 8225116C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82251170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251174: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251178: 817E0590  lwz r11, 0x590(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1424 as u32) ) } as u64;
	// 8225117C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82251180: 817E0590  lwz r11, 0x590(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1424 as u32) ) } as u64;
	// 82251184: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82251188: 917E0590  stw r11, 0x590(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1424 as u32), ctx.r[11].u32 ) };
	// 8225118C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251198: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8225119C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822511A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822511A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822511A8 size=100
    let mut pc: u32 = 0x822511A8;
    'dispatch: loop {
        match pc {
            0x822511A8 => {
    //   block [0x822511A8..0x8225120C)
	// 822511A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822511AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822511B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822511B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822511B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822511BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822511C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822511C4: 4BFFE7CD  bl 0x8224f990
	ctx.lr = 0x822511C8;
	sub_8224F990(ctx, base);
	// 822511C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822511CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822511D0: 396B4C38  addi r11, r11, 0x4c38
	ctx.r[11].s64 = ctx.r[11].s64 + 19512;
	// 822511D4: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 822511D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822511DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822511E0: 817E0594  lwz r11, 0x594(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1428 as u32) ) } as u64;
	// 822511E4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822511E8: 817E0594  lwz r11, 0x594(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1428 as u32) ) } as u64;
	// 822511EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822511F0: 917E0594  stw r11, 0x594(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 822511F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822511F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822511FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82251204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82251208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251210 size=88
    let mut pc: u32 = 0x82251210;
    'dispatch: loop {
        match pc {
            0x82251210 => {
    //   block [0x82251210..0x82251268)
	// 82251210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82251218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225121C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82251220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251228: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8225122C: 4BFFF075  bl 0x822502a0
	ctx.lr = 0x82251230;
	sub_822502A0(ctx, base);
	// 82251230: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82251234: 41820018  beq 0x8225124c
	if ctx.cr[0].eq {
	pc = 0x8225124C; continue 'dispatch;
	}
	// 82251238: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8225123C: 419A0010  beq cr6, 0x8225124c
	if ctx.cr[6].eq {
	pc = 0x8225124C; continue 'dispatch;
	}
	// 82251240: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 82251244: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82251248: 4BFC6D41  bl 0x82217f88
	ctx.lr = 0x8225124C;
	sub_82217F88(ctx, base);
	// 8225124C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8225125C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82251260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82251264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82251268 size=12
    let mut pc: u32 = 0x82251268;
    'dispatch: loop {
        match pc {
            0x82251268 => {
    //   block [0x82251268..0x82251274)
	// 82251268: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8225126C: C02B06F8  lfs f1, 0x6f8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1784 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82251270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251278 size=172
    let mut pc: u32 = 0x82251278;
    'dispatch: loop {
        match pc {
            0x82251278 => {
    //   block [0x82251278..0x82251324)
	// 82251278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225127C: 4BE3D819  bl 0x8208ea94
	ctx.lr = 0x82251280;
	sub_8208EA60(ctx, base);
	// 82251280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251288: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8225128C: 895F00A4  lbz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82251290: 897C00A4  lbz r11, 0xa4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 82251294: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82251298: 419A000C  beq cr6, 0x822512a4
	if ctx.cr[6].eq {
	pc = 0x822512A4; continue 'dispatch;
	}
	// 8225129C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822512A0: 4800007C  b 0x8225131c
	pc = 0x8225131C; continue 'dispatch;
	// 822512A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822512A8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 822512AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822512B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822512B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822512B8: 4E800421  bctrl
	ctx.lr = 0x822512BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822512BC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 822512C0: 41980058  blt cr6, 0x82251318
	if ctx.cr[6].lt {
	pc = 0x82251318; continue 'dispatch;
	}
	// 822512C4: 3BBF0099  addi r29, r31, 0x99
	ctx.r[29].s64 = ctx.r[31].s64 + 153;
	// 822512C8: 7F7FE050  subf r27, r31, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 822512CC: 7D7BE8AE  lbzx r11, r27, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 822512D0: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822512D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822512D8: 409AFFC4  bne cr6, 0x8225129c
	if !ctx.cr[6].eq {
	pc = 0x8225129C; continue 'dispatch;
	}
	// 822512DC: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 822512E0: 7D5EFA14  add r10, r30, r31
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 822512E4: 896B009E  lbz r11, 0x9e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(158 as u32) ) } as u64;
	// 822512E8: 894A009E  lbz r10, 0x9e(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(158 as u32) ) } as u64;
	// 822512EC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822512F0: 409AFFAC  bne cr6, 0x8225129c
	if !ctx.cr[6].eq {
	pc = 0x8225129C; continue 'dispatch;
	}
	// 822512F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822512F8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822512FC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82251300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251304: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8225130C: 4E800421  bctrl
	ctx.lr = 0x82251310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82251310: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82251314: 4099FFB8  ble cr6, 0x822512cc
	if !ctx.cr[6].gt {
	pc = 0x822512CC; continue 'dispatch;
	}
	// 82251318: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8225131C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82251320: 4BE3D7C4  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251328 size=128
    let mut pc: u32 = 0x82251328;
    'dispatch: loop {
        match pc {
            0x82251328 => {
    //   block [0x82251328..0x822513A8)
	// 82251328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225132C: 4BE3D771  bl 0x8208ea9c
	ctx.lr = 0x82251330;
	sub_8208EA60(ctx, base);
	// 82251330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251334: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82251338: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8225133C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82251340: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251344: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82251348: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8225134C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82251350: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82251354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82251358: 4E800421  bctrl
	ctx.lr = 0x8225135C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8225135C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82251360: 419A0028  beq cr6, 0x82251388
	if ctx.cr[6].eq {
	pc = 0x82251388; continue 'dispatch;
	}
	// 82251364: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251368: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8225136C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82251370: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82251374: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82251378: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8225137C: 4E800421  bctrl
	ctx.lr = 0x82251380;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82251380: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82251384: 409A001C  bne cr6, 0x822513a0
	if !ctx.cr[6].eq {
	pc = 0x822513A0; continue 'dispatch;
	}
	// 82251388: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8225138C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82251390: 4198FFB0  blt cr6, 0x82251340
	if ctx.cr[6].lt {
	pc = 0x82251340; continue 'dispatch;
	}
	// 82251394: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82251398: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225139C: 4BE3D750  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
	// 822513A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822513A4: 4BFFFFF4  b 0x82251398
	pc = 0x82251398; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822513A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822513A8 size=88
    let mut pc: u32 = 0x822513A8;
    'dispatch: loop {
        match pc {
            0x822513A8 => {
    //   block [0x822513A8..0x82251400)
	// 822513A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822513AC: 4BE3D6F1  bl 0x8208ea9c
	ctx.lr = 0x822513B0;
	sub_8208EA60(ctx, base);
	// 822513B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822513B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822513B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822513BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822513C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822513C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822513C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822513CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822513D0: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 822513D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822513D8: 4E800421  bctrl
	ctx.lr = 0x822513DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822513DC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 822513E0: 419A0008  beq cr6, 0x822513e8
	if ctx.cr[6].eq {
	pc = 0x822513E8; continue 'dispatch;
	}
	// 822513E4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 822513E8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 822513EC: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 822513F0: 4198FFD0  blt cr6, 0x822513c0
	if ctx.cr[6].lt {
	pc = 0x822513C0; continue 'dispatch;
	}
	// 822513F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822513F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822513FC: 4BE3D6F0  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251400 size=116
    let mut pc: u32 = 0x82251400;
    'dispatch: loop {
        match pc {
            0x82251400 => {
    //   block [0x82251400..0x82251474)
	// 82251400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251404: 4BE3D699  bl 0x8208ea9c
	ctx.lr = 0x82251408;
	sub_8208EA60(ctx, base);
	// 82251408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225140C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82251410: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82251414: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82251418: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8225141C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82251420: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82251424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82251428: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8225142C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82251430: 4E800421  bctrl
	ctx.lr = 0x82251434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82251434: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251438: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8225143C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82251440: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82251444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82251448: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8225144C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82251450: 409A0008  bne cr6, 0x82251458
	if !ctx.cr[6].eq {
	pc = 0x82251458; continue 'dispatch;
	}
	// 82251454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82251458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8225145C: 4E800421  bctrl
	ctx.lr = 0x82251460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82251460: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82251464: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82251468: 4198FFB0  blt cr6, 0x82251418
	if ctx.cr[6].lt {
	pc = 0x82251418; continue 'dispatch;
	}
	// 8225146C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251470: 4BE3D67C  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251478 size=52
    let mut pc: u32 = 0x82251478;
    'dispatch: loop {
        match pc {
            0x82251478 => {
    //   block [0x82251478..0x822514AC)
	// 82251478: 89630080  lbz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 8225147C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82251480: 419A002C  beq cr6, 0x822514ac
	if ctx.cr[6].eq {
		sub_822514AC(ctx, base);
		return;
	}
	// 82251484: 89630081  lbz r11, 0x81(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(129 as u32) ) } as u64;
	// 82251488: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8225148C: 419A0020  beq cr6, 0x822514ac
	if ctx.cr[6].eq {
		sub_822514AC(ctx, base);
		return;
	}
	// 82251490: 89630082  lbz r11, 0x82(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(130 as u32) ) } as u64;
	// 82251494: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82251498: 419A0014  beq cr6, 0x822514ac
	if ctx.cr[6].eq {
		sub_822514AC(ctx, base);
		return;
	}
	// 8225149C: 89630083  lbz r11, 0x83(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(131 as u32) ) } as u64;
	// 822514A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822514A4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822514A8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822514AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822514AC size=8
    let mut pc: u32 = 0x822514AC;
    'dispatch: loop {
        match pc {
            0x822514AC => {
    //   block [0x822514AC..0x822514B4)
	// 822514AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822514B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822514B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822514B8 size=44
    let mut pc: u32 = 0x822514B8;
    'dispatch: loop {
        match pc {
            0x822514B8 => {
    //   block [0x822514B8..0x822514E4)
	// 822514B8: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 822514BC: 1D430034  mulli r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 * 52;
	// 822514C0: 396B4720  addi r11, r11, 0x4720
	ctx.r[11].s64 = ctx.r[11].s64 + 18208;
	// 822514C4: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 822514C8: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 822514CC: 39295108  addi r9, r9, 0x5108
	ctx.r[9].s64 = ctx.r[9].s64 + 20744;
	// 822514D0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822514D4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822514D8: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822514DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822514E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822514E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822514E8 size=96
    let mut pc: u32 = 0x822514E8;
    'dispatch: loop {
        match pc {
            0x822514E8 => {
    //   block [0x822514E8..0x82251548)
	// 822514E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822514EC: 4BE3D5B1  bl 0x8208ea9c
	ctx.lr = 0x822514F0;
	sub_8208EA60(ctx, base);
	// 822514F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822514F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822514F8: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822514FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82251500: 807D05AC  lwz r3, 0x5ac(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82251504: 4BFC6955  bl 0x82217e58
	ctx.lr = 0x82251508;
	sub_82217E58(ctx, base);
	// 82251508: 817D05AC  lwz r11, 0x5ac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225150C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251510: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82251514: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82251518: 38A003C0  li r5, 0x3c0
	ctx.r[5].s64 = 960;
	// 8225151C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82251520: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251524: 4BE3DCCD  bl 0x8208f1f0
	ctx.lr = 0x82251528;
	sub_8208F1F0(ctx, base);
	// 82251528: 817D0560  lwz r11, 0x560(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1376 as u32) ) } as u64;
	// 8225152C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82251530: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82251534: 817D0560  lwz r11, 0x560(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82251538: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8225153C: 917D0560  stw r11, 0x560(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1376 as u32), ctx.r[11].u32 ) };
	// 82251540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251544: 4BE3D5A8  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251548 size=544
    let mut pc: u32 = 0x82251548;
    'dispatch: loop {
        match pc {
            0x82251548 => {
    //   block [0x82251548..0x82251768)
	// 82251548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225154C: 4BE3D53D  bl 0x8208ea88
	ctx.lr = 0x82251550;
	sub_8208EA60(ctx, base);
	// 82251550: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251558: 3CE08204  lis r7, -0x7dfc
	ctx.r[7].s64 = -2113667072;
	// 8225155C: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82251560: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82251564: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82251568: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8225156C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251570: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82251574: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82251578: 3B475260  addi r26, r7, 0x5260
	ctx.r[26].s64 = ctx.r[7].s64 + 21088;
	// 8225157C: 3B285238  addi r25, r8, 0x5238
	ctx.r[25].s64 = ctx.r[8].s64 + 21048;
	// 82251580: 3B8954C0  addi r28, r9, 0x54c0
	ctx.r[28].s64 = ctx.r[9].s64 + 21696;
	// 82251584: 3B0A522C  addi r24, r10, 0x522c
	ctx.r[24].s64 = ctx.r[10].s64 + 21036;
	// 82251588: 3B6B51C8  addi r27, r11, 0x51c8
	ctx.r[27].s64 = ctx.r[11].s64 + 20936;
	// 8225158C: 409900B8  ble cr6, 0x82251644
	if !ctx.cr[6].gt {
	pc = 0x82251644; continue 'dispatch;
	}
	// 82251590: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82251594: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8225159C: 409A001C  bne cr6, 0x822515b8
	if !ctx.cr[6].eq {
	pc = 0x822515B8; continue 'dispatch;
	}
	// 822515A0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822515A4: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 822515A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822515AC: 38E00088  li r7, 0x88
	ctx.r[7].s64 = 136;
	// 822515B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822515B4: 4BF04455  bl 0x82155a08
	ctx.lr = 0x822515B8;
	sub_82155A08(ctx, base);
	// 822515B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822515BC: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 822515C0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 822515C4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822515C8: 419A001C  beq cr6, 0x822515e4
	if ctx.cr[6].eq {
	pc = 0x822515E4; continue 'dispatch;
	}
	// 822515CC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822515D0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822515D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822515D8: 38E00089  li r7, 0x89
	ctx.r[7].s64 = 137;
	// 822515DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822515E0: 4BF04429  bl 0x82155a08
	ctx.lr = 0x822515E4;
	sub_82155A08(ctx, base);
	// 822515E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822515E8: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 822515EC: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822515F0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822515F4: 419A0028  beq cr6, 0x8225161c
	if ctx.cr[6].eq {
	pc = 0x8225161C; continue 'dispatch;
	}
	// 822515F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822515FC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82251600: 419A001C  beq cr6, 0x8225161c
	if ctx.cr[6].eq {
	pc = 0x8225161C; continue 'dispatch;
	}
	// 82251604: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82251608: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8225160C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82251610: 38E0008B  li r7, 0x8b
	ctx.r[7].s64 = 139;
	// 82251614: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251618: 4BF043F1  bl 0x82155a08
	ctx.lr = 0x8225161C;
	sub_82155A08(ctx, base);
	// 8225161C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251624: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82251628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8225162C: 4E800421  bctrl
	ctx.lr = 0x82251630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82251630: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82251634: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82251638: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8225163C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82251640: 4198FF54  blt cr6, 0x82251594
	if ctx.cr[6].lt {
	pc = 0x82251594; continue 'dispatch;
	}
	// 82251644: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82251648: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8225164C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82251650: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82251654: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82251658: 409900DC  ble cr6, 0x82251734
	if !ctx.cr[6].gt {
	pc = 0x82251734; continue 'dispatch;
	}
	// 8225165C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82251660: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251664: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82251668: 409A001C  bne cr6, 0x82251684
	if !ctx.cr[6].eq {
	pc = 0x82251684; continue 'dispatch;
	}
	// 8225166C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82251670: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82251674: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82251678: 38E00091  li r7, 0x91
	ctx.r[7].s64 = 145;
	// 8225167C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251680: 4BF04389  bl 0x82155a08
	ctx.lr = 0x82251684;
	sub_82155A08(ctx, base);
	// 82251684: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251688: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8225168C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82251690: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82251694: 419A001C  beq cr6, 0x822516b0
	if ctx.cr[6].eq {
	pc = 0x822516B0; continue 'dispatch;
	}
	// 82251698: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8225169C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822516A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822516A4: 38E00092  li r7, 0x92
	ctx.r[7].s64 = 146;
	// 822516A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822516AC: 4BF0435D  bl 0x82155a08
	ctx.lr = 0x822516B0;
	sub_82155A08(ctx, base);
	// 822516B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822516B4: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 822516B8: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822516BC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822516C0: 419A0028  beq cr6, 0x822516e8
	if ctx.cr[6].eq {
	pc = 0x822516E8; continue 'dispatch;
	}
	// 822516C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822516C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822516CC: 419A001C  beq cr6, 0x822516e8
	if ctx.cr[6].eq {
	pc = 0x822516E8; continue 'dispatch;
	}
	// 822516D0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822516D4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822516D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822516DC: 38E00094  li r7, 0x94
	ctx.r[7].s64 = 148;
	// 822516E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822516E4: 4BF04325  bl 0x82155a08
	ctx.lr = 0x822516E8;
	sub_82155A08(ctx, base);
	// 822516E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822516EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822516F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822516F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822516F8: 4E800421  bctrl
	ctx.lr = 0x822516FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822516FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82251700: 40820018  bne 0x82251718
	if !ctx.cr[0].eq {
	pc = 0x82251718; continue 'dispatch;
	}
	// 82251704: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8225170C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82251710: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82251714: 4E800421  bctrl
	ctx.lr = 0x82251718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82251718: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8225171C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82251720: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82251724: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82251728: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8225172C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82251730: 4198FF30  blt cr6, 0x82251660
	if ctx.cr[6].lt {
	pc = 0x82251660; continue 'dispatch;
	}
	// 82251734: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82251738: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8225173C: 409A0020  bne cr6, 0x8225175c
	if !ctx.cr[6].eq {
	pc = 0x8225175C; continue 'dispatch;
	}
	// 82251740: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251744: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82251748: 38AB5218  addi r5, r11, 0x5218
	ctx.r[5].s64 = ctx.r[11].s64 + 21016;
	// 8225174C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82251750: 38E00099  li r7, 0x99
	ctx.r[7].s64 = 153;
	// 82251754: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251758: 4BF042B1  bl 0x82155a08
	ctx.lr = 0x8225175C;
	sub_82155A08(ctx, base);
	// 8225175C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82251760: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82251764: 4BE3D374  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251768 size=28
    let mut pc: u32 = 0x82251768;
    'dispatch: loop {
        match pc {
            0x82251768 => {
    //   block [0x82251768..0x82251784)
	// 82251768: 3964003A  addi r11, r4, 0x3a
	ctx.r[11].s64 = ctx.r[4].s64 + 58;
	// 8225176C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82251770: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82251774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82251778: 419A000C  beq cr6, 0x82251784
	if ctx.cr[6].eq {
		sub_82251784(ctx, base);
		return;
	}
	// 8225177C: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82251780: 48000010  b 0x82251790
	sub_82251784(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251784 size=56
    let mut pc: u32 = 0x82251784;
    'dispatch: loop {
        match pc {
            0x82251784 => {
    //   block [0x82251784..0x822517BC)
	// 82251784: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 82251788: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8225178C: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82251790: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82251794: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 82251798: 396A9638  addi r11, r10, -0x69c8
	ctx.r[11].s64 = ctx.r[10].s64 + -27080;
	// 8225179C: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 822517A0: 7D6958AE  lbzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822517A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 822517A8: 419A0014  beq cr6, 0x822517bc
	if ctx.cr[6].eq {
		sub_822517BC(ctx, base);
		return;
	}
	// 822517AC: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 822517B0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822517B4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822517B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822517BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822517BC size=8
    let mut pc: u32 = 0x822517BC;
    'dispatch: loop {
        match pc {
            0x822517BC => {
    //   block [0x822517BC..0x822517C4)
	// 822517BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 822517C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822517C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822517C8 size=344
    let mut pc: u32 = 0x822517C8;
    'dispatch: loop {
        match pc {
            0x822517C8 => {
    //   block [0x822517C8..0x82251920)
	// 822517C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822517CC: 4BE3D2C9  bl 0x8208ea94
	ctx.lr = 0x822517D0;
	sub_8208EA60(ctx, base);
	// 822517D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822517D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822517D8: 392A51C0  addi r9, r10, 0x51c0
	ctx.r[9].s64 = ctx.r[10].s64 + 20928;
	// 822517DC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 822517E0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 822517E4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 822517E8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822517EC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822517F0: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 822517F4: 93E30038  stw r31, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 822517F8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 822517FC: 90830050  stw r4, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82251800: 38C30098  addi r6, r3, 0x98
	ctx.r[6].s64 = ctx.r[3].s64 + 152;
	// 82251804: 8149FFF8  lwz r10, -8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251808: 38A3009E  addi r5, r3, 0x9e
	ctx.r[5].s64 = ctx.r[3].s64 + 158;
	// 8225180C: 91430080  stw r10, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82251810: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82251814: 8109FFF0  lwz r8, -0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82251818: 910303B0  stw r8, 0x3b0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(944 as u32), ctx.r[8].u32 ) };
	// 8225181C: 3D008228  lis r8, -0x7dd8
	ctx.r[8].s64 = -2111307776;
	// 82251820: 7FC33D2C  stwbrx r30, r3, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[7].u32), (ctx.r[30].u32.swap_bytes())) };
	// 82251824: 3BC84720  addi r30, r8, 0x4720
	ctx.r[30].s64 = ctx.r[8].s64 + 18208;
	// 82251828: 83890004  lwz r28, 4(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8225182C: 83A90000  lwz r29, 0(r9)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251830: 99630098  stb r11, 0x98(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 82251834: 9963009E  stb r11, 0x9e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(158 as u32), ctx.r[11].u8 ) };
	// 82251838: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8225183C: 916300C8  stw r11, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82251840: 390A000E  addi r8, r10, 0xe
	ctx.r[8].s64 = ctx.r[10].s64 + 14;
	// 82251844: 38EA0014  addi r7, r10, 0x14
	ctx.r[7].s64 = ctx.r[10].s64 + 20;
	// 82251848: 551B103A  slwi r27, r8, 2
	ctx.r[27].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 8225184C: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 82251850: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82251854: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82251858: 7FFB192E  stwx r31, r27, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[3].u32), ctx.r[31].u32) };
	// 8225185C: 7C87192E  stwx r4, r7, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 82251860: 80E9FFF0  lwz r7, -0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82251864: 7CE8192E  stwx r7, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[7].u32) };
	// 82251868: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8225186C: 1CE70034  mulli r7, r7, 0x34
	ctx.r[7].s64 = ctx.r[7].s64 * 52;
	// 82251870: 7CE7F02E  lwzx r7, r7, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82251874: 54E7F7FF  rlwinm. r7, r7, 0x1e, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82251878: 41820018  beq 0x82251890
	if ctx.cr[0].eq {
	pc = 0x82251890; continue 'dispatch;
	}
	// 8225187C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82251880: 409A000C  bne cr6, 0x8225188c
	if !ctx.cr[6].eq {
	pc = 0x8225188C; continue 'dispatch;
	}
	// 82251884: 93830084  stw r28, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82251888: 48000008  b 0x82251890
	pc = 0x82251890; continue 'dispatch;
	// 8225188C: 7FA8192E  stwx r29, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[29].u32) };
	// 82251890: 390A0007  addi r8, r10, 7
	ctx.r[8].s64 = ctx.r[10].s64 + 7;
	// 82251894: 7D6651AE  stbx r11, r6, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82251898: 38EA0032  addi r7, r10, 0x32
	ctx.r[7].s64 = ctx.r[10].s64 + 50;
	// 8225189C: 7D6551AE  stbx r11, r5, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 822518A0: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 822518A4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 822518A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822518AC: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 822518B0: 7D68192E  stwx r11, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 822518B4: 7D67192E  stwx r11, r7, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 822518B8: 4198FF88  blt cr6, 0x82251840
	if ctx.cr[6].lt {
	pc = 0x82251840; continue 'dispatch;
	}
	// 822518BC: 3943016C  addi r10, r3, 0x16c
	ctx.r[10].s64 = ctx.r[3].s64 + 364;
	// 822518C0: 996300A4  stb r11, 0xa4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u8 ) };
	// 822518C4: 38E300E8  addi r7, r3, 0xe8
	ctx.r[7].s64 = ctx.r[3].s64 + 232;
	// 822518C8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 822518CC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 822518D0: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822518D4: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822518D8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822518DC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 822518E0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822518E4: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822518E8: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 822518EC: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 822518F0: 4082FFE0  bne 0x822518d0
	if !ctx.cr[0].eq {
	pc = 0x822518D0; continue 'dispatch;
	}
	// 822518F4: 390303A0  addi r8, r3, 0x3a0
	ctx.r[8].s64 = ctx.r[3].s64 + 928;
	// 822518F8: 39430384  addi r10, r3, 0x384
	ctx.r[10].s64 = ctx.r[3].s64 + 900;
	// 822518FC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82251900: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82251904: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82251908: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8225190C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82251910: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251914: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82251918: 4082FFE8  bne 0x82251900
	if !ctx.cr[0].eq {
	pc = 0x82251900; continue 'dispatch;
	}
	// 8225191C: 4BE3D1C8  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251920 size=120
    let mut pc: u32 = 0x82251920;
    'dispatch: loop {
        match pc {
            0x82251920 => {
    //   block [0x82251920..0x82251998)
	// 82251920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251924: 4BE3D179  bl 0x8208ea9c
	ctx.lr = 0x82251928;
	sub_8208EA60(ctx, base);
	// 82251928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225192C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82251930: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82251934: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82251938: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 8225193C: 41980028  blt cr6, 0x82251964
	if ctx.cr[6].lt {
	pc = 0x82251964; continue 'dispatch;
	}
	// 82251940: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251944: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82251948: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 8225194C: 38CB51C8  addi r6, r11, 0x51c8
	ctx.r[6].s64 = ctx.r[11].s64 + 20936;
	// 82251950: 38AA52B4  addi r5, r10, 0x52b4
	ctx.r[5].s64 = ctx.r[10].s64 + 21172;
	// 82251954: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 82251958: 38E000FB  li r7, 0xfb
	ctx.r[7].s64 = 251;
	// 8225195C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251960: 4BF040A9  bl 0x82155a08
	ctx.lr = 0x82251964;
	sub_82155A08(ctx, base);
	// 82251964: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82251968: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8225196C: 393F000E  addi r9, r31, 0xe
	ctx.r[9].s64 = ctx.r[31].s64 + 14;
	// 82251970: 391F0007  addi r8, r31, 7
	ctx.r[8].s64 = ctx.r[31].s64 + 7;
	// 82251974: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82251978: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8225197C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82251980: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82251984: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82251988: 7D69F12E  stwx r11, r9, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 8225198C: 7FA8F12E  stwx r29, r8, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 82251990: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82251994: 4BE3D158  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82251998 size=980
    let mut pc: u32 = 0x82251998;
    'dispatch: loop {
        match pc {
            0x82251998 => {
    //   block [0x82251998..0x82251D6C)
	// 82251998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225199C: 4BE3D0F9  bl 0x8208ea94
	ctx.lr = 0x822519A0;
	sub_8208EA60(ctx, base);
	// 822519A0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822519A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822519A8: F8C100D8  std r6, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u64 ) };
	// 822519AC: F8E100E0  std r7, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[7].u64 ) };
	// 822519B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822519B4: F90100E8  std r8, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[8].u64 ) };
	// 822519B8: 390100E0  addi r8, r1, 0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + 224;
	// 822519BC: F92100F0  std r9, 0xf0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[9].u64 ) };
	// 822519C0: 392100D8  addi r9, r1, 0xd8
	ctx.r[9].s64 = ctx.r[1].s64 + 216;
	// 822519C4: 394A4CA8  addi r10, r10, 0x4ca8
	ctx.r[10].s64 = ctx.r[10].s64 + 19624;
	// 822519C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 822519CC: 83EB51B0  lwz r31, 0x51b0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20912 as u32) ) } as u64;
	// 822519D0: 396100E8  addi r11, r1, 0xe8
	ctx.r[11].s64 = ctx.r[1].s64 + 232;
	// 822519D4: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 822519D8: 392100F0  addi r9, r1, 0xf0
	ctx.r[9].s64 = ctx.r[1].s64 + 240;
	// 822519DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822519E0: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 822519E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822519E8: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 822519EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822519F0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 822519F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822519F8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 822519FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82251A00: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82251A04: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82251A08: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82251A0C: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82251A10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82251A14: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82251A18: 41980080  blt cr6, 0x82251a98
	if ctx.cr[6].lt {
	pc = 0x82251A98; continue 'dispatch;
	}
	// 82251A1C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82251A20: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251A24: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82251A28: 419A004C  beq cr6, 0x82251a74
	if ctx.cr[6].eq {
	pc = 0x82251A74; continue 'dispatch;
	}
	// 82251A2C: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251A30: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251A34: 836A0000  lwz r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251A38: 7F1BE040  cmplw cr6, r27, r28
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82251A3C: 409A0018  bne cr6, 0x82251a54
	if !ctx.cr[6].eq {
	pc = 0x82251A54; continue 'dispatch;
	}
	// 82251A40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251A44: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251A48: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82251A4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82251A50: 419A0008  beq cr6, 0x82251a58
	if ctx.cr[6].eq {
	pc = 0x82251A58; continue 'dispatch;
	}
	// 82251A54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82251A58: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82251A5C: 40820034  bne 0x82251a90
	if !ctx.cr[0].eq {
	pc = 0x82251A90; continue 'dispatch;
	}
	// 82251A60: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82251A64: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82251A68: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82251A6C: 4099FFB4  ble cr6, 0x82251a20
	if !ctx.cr[6].gt {
	pc = 0x82251A20; continue 'dispatch;
	}
	// 82251A70: 48000028  b 0x82251a98
	pc = 0x82251A98; continue 'dispatch;
	// 82251A74: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82251A78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251A7C: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82251A80: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82251A84: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82251A88: 7D2551AE  stbx r9, r5, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 82251A8C: 4800000C  b 0x82251a98
	pc = 0x82251A98; continue 'dispatch;
	// 82251A90: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82251A94: 7D2559AE  stbx r9, r5, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82251A98: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82251A9C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82251AA0: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82251AA4: 4198FF6C  blt cr6, 0x82251a10
	if ctx.cr[6].lt {
	pc = 0x82251A10; continue 'dispatch;
	}
	// 82251AA8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82251AAC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82251AB0: 4099008C  ble cr6, 0x82251b3c
	if !ctx.cr[6].gt {
	pc = 0x82251B3C; continue 'dispatch;
	}
	// 82251AB4: 3866FFFF  addi r3, r6, -1
	ctx.r[3].s64 = ctx.r[6].s64 + -1;
	// 82251AB8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82251ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82251AC0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82251AC4: 4099006C  ble cr6, 0x82251b30
	if !ctx.cr[6].gt {
	pc = 0x82251B30; continue 'dispatch;
	}
	// 82251AC8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82251ACC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251AD0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251AD4: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251AD8: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251ADC: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82251AE0: 409A000C  bne cr6, 0x82251aec
	if !ctx.cr[6].eq {
	pc = 0x82251AEC; continue 'dispatch;
	}
	// 82251AE4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251AE8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251AEC: 7D074010  subfc r8, r7, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[7].u32;
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82251AF0: 7D084110  subfe r8, r8, r8
	let x = (!ctx.r[8].u32);
	let y = ctx.r[8].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82251AF4: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82251AF8: 5508063F  clrlwi. r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82251AFC: 41820024  beq 0x82251b20
	if ctx.cr[0].eq {
	pc = 0x82251B20; continue 'dispatch;
	}
	// 82251B00: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82251B04: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82251B08: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82251B0C: 7D654214  add r11, r5, r8
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82251B10: 7D2540AE  lbzx r9, r5, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82251B14: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82251B18: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82251B1C: 7CE541AE  stbx r7, r5, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u8) };
	// 82251B20: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82251B24: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82251B28: 7F051800  cmpw cr6, r5, r3
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82251B2C: 4198FFA0  blt cr6, 0x82251acc
	if ctx.cr[6].lt {
	pc = 0x82251ACC; continue 'dispatch;
	}
	// 82251B30: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82251B34: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82251B38: 4082FF84  bne 0x82251abc
	if !ctx.cr[0].eq {
	pc = 0x82251ABC; continue 'dispatch;
	}
	// 82251B3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82251B40: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82251B44: 40990024  ble cr6, 0x82251b68
	if !ctx.cr[6].gt {
	pc = 0x82251B68; continue 'dispatch;
	}
	// 82251B48: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 82251B4C: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 82251B50: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82251B54: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82251B58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82251B5C: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82251B60: 7D0A49AE  stbx r8, r10, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82251B64: 4198FFE4  blt cr6, 0x82251b48
	if ctx.cr[6].lt {
	pc = 0x82251B48; continue 'dispatch;
	}
	// 82251B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82251B6C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82251B70: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 82251B74: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82251B78: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82251B7C: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82251B80: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82251B84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82251B88: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82251B8C: 4198FFE0  blt cr6, 0x82251b6c
	if ctx.cr[6].lt {
	pc = 0x82251B6C; continue 'dispatch;
	}
	// 82251B90: 817E03B8  lwz r11, 0x3b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(952 as u32) ) } as u64;
	// 82251B94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82251B98: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 82251B9C: 896B0571  lbz r11, 0x571(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1393 as u32) ) } as u64;
	// 82251BA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82251BA4: 418200DC  beq 0x82251c80
	if ctx.cr[0].eq {
	pc = 0x82251C80; continue 'dispatch;
	}
	// 82251BA8: 419A00B0  beq cr6, 0x82251c58
	if ctx.cr[6].eq {
	pc = 0x82251C58; continue 'dispatch;
	}
	// 82251BAC: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 82251BB0: 419A008C  beq cr6, 0x82251c3c
	if ctx.cr[6].eq {
	pc = 0x82251C3C; continue 'dispatch;
	}
	// 82251BB4: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 82251BB8: 419A0060  beq cr6, 0x82251c18
	if ctx.cr[6].eq {
	pc = 0x82251C18; continue 'dispatch;
	}
	// 82251BBC: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82251BC0: 419A002C  beq cr6, 0x82251bec
	if ctx.cr[6].eq {
	pc = 0x82251BEC; continue 'dispatch;
	}
	// 82251BC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251BC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82251BCC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82251BD0: 38CB51C8  addi r6, r11, 0x51c8
	ctx.r[6].s64 = ctx.r[11].s64 + 20936;
	// 82251BD4: 38AAFE08  addi r5, r10, -0x1f8
	ctx.r[5].s64 = ctx.r[10].s64 + -504;
	// 82251BD8: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 82251BDC: 38E00181  li r7, 0x181
	ctx.r[7].s64 = 385;
	// 82251BE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251BE4: 4BF03E25  bl 0x82155a08
	ctx.lr = 0x82251BE8;
	sub_82155A08(ctx, base);
	// 82251BE8: 48000084  b 0x82251c6c
	pc = 0x82251C6C; continue 'dispatch;
	// 82251BEC: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82251BF0: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82251BF4: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82251BF8: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251BFC: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251C00: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251C04: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82251C08: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82251C0C: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82251C10: 4BFEF931  bl 0x82241540
	ctx.lr = 0x82251C14;
	sub_82241540(ctx, base);
	// 82251C14: 48000054  b 0x82251c68
	pc = 0x82251C68; continue 'dispatch;
	// 82251C18: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82251C1C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82251C20: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251C24: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251C28: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251C2C: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82251C30: E8890000  ld r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82251C34: 4BFEF8ED  bl 0x82241520
	ctx.lr = 0x82251C38;
	sub_82241520(ctx, base);
	// 82251C38: 48000030  b 0x82251c68
	pc = 0x82251C68; continue 'dispatch;
	// 82251C3C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82251C40: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251C44: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251C48: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251C4C: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82251C50: 4BFEF8B1  bl 0x82241500
	ctx.lr = 0x82251C54;
	sub_82241500(ctx, base);
	// 82251C54: 48000014  b 0x82251c68
	pc = 0x82251C68; continue 'dispatch;
	// 82251C58: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251C5C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251C60: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251C64: 4BFEF87D  bl 0x822414e0
	ctx.lr = 0x82251C68;
	sub_822414E0(ctx, base);
	// 82251C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251C6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82251C70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82251C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82251C78: 4BFDFD09  bl 0x82231980
	ctx.lr = 0x82251C7C;
	sub_82231980(ctx, base);
	// 82251C7C: 480000D8  b 0x82251d54
	pc = 0x82251D54; continue 'dispatch;
	// 82251C80: 419A00B0  beq cr6, 0x82251d30
	if ctx.cr[6].eq {
	pc = 0x82251D30; continue 'dispatch;
	}
	// 82251C84: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 82251C88: 419A008C  beq cr6, 0x82251d14
	if ctx.cr[6].eq {
	pc = 0x82251D14; continue 'dispatch;
	}
	// 82251C8C: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 82251C90: 419A0060  beq cr6, 0x82251cf0
	if ctx.cr[6].eq {
	pc = 0x82251CF0; continue 'dispatch;
	}
	// 82251C94: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82251C98: 419A002C  beq cr6, 0x82251cc4
	if ctx.cr[6].eq {
	pc = 0x82251CC4; continue 'dispatch;
	}
	// 82251C9C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251CA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82251CA4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82251CA8: 38CB51C8  addi r6, r11, 0x51c8
	ctx.r[6].s64 = ctx.r[11].s64 + 20936;
	// 82251CAC: 38AAFE08  addi r5, r10, -0x1f8
	ctx.r[5].s64 = ctx.r[10].s64 + -504;
	// 82251CB0: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 82251CB4: 38E0019A  li r7, 0x19a
	ctx.r[7].s64 = 410;
	// 82251CB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82251CBC: 4BF03D4D  bl 0x82155a08
	ctx.lr = 0x82251CC0;
	sub_82155A08(ctx, base);
	// 82251CC0: 48000084  b 0x82251d44
	pc = 0x82251D44; continue 'dispatch;
	// 82251CC4: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82251CC8: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82251CCC: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82251CD0: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251CD4: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251CD8: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251CDC: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82251CE0: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82251CE4: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82251CE8: 4BFF11D9  bl 0x82242ec0
	ctx.lr = 0x82251CEC;
	sub_82242EC0(ctx, base);
	// 82251CEC: 48000054  b 0x82251d40
	pc = 0x82251D40; continue 'dispatch;
	// 82251CF0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82251CF4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82251CF8: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251CFC: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251D00: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251D04: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82251D08: E8890000  ld r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82251D0C: 4BFF1195  bl 0x82242ea0
	ctx.lr = 0x82251D10;
	sub_82242EA0(ctx, base);
	// 82251D10: 48000030  b 0x82251d40
	pc = 0x82251D40; continue 'dispatch;
	// 82251D14: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82251D18: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251D1C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251D20: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251D24: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82251D28: 4BFF1159  bl 0x82242e80
	ctx.lr = 0x82251D2C;
	sub_82242E80(ctx, base);
	// 82251D2C: 48000014  b 0x82251d40
	pc = 0x82251D40; continue 'dispatch;
	// 82251D30: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251D34: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251D38: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82251D3C: 4BFF1125  bl 0x82242e60
	ctx.lr = 0x82251D40;
	sub_82242E60(ctx, base);
	// 82251D40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251D44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82251D48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82251D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82251D50: 4BFFFBD1  bl 0x82251920
	ctx.lr = 0x82251D54;
	sub_82251920(ctx, base);
	// 82251D54: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82251D58: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82251D5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82251D60: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82251D64: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82251D68: 4BE3CD7C  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251D70 size=160
    let mut pc: u32 = 0x82251D70;
    'dispatch: loop {
        match pc {
            0x82251D70 => {
    //   block [0x82251D70..0x82251E10)
	// 82251D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251D74: 4BE3CD25  bl 0x8208ea98
	ctx.lr = 0x82251D78;
	sub_8208EA60(ctx, base);
	// 82251D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251D7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82251D80: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 82251D84: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82251D88: 917C00E4  stw r11, 0xe4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82251D8C: 4BFFD10D  bl 0x8224ee98
	ctx.lr = 0x82251D90;
	sub_8224EE98(ctx, base);
	// 82251D90: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 82251D94: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82251D98: 41820070  beq 0x82251e08
	if ctx.cr[0].eq {
	pc = 0x82251E08; continue 'dispatch;
	}
	// 82251D9C: 817C03B8  lwz r11, 0x3b8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(952 as u32) ) } as u64;
	// 82251DA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82251DA4: 816B0AB0  lwz r11, 0xab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82251DA8: 83EB0060  lwz r31, 0x60(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82251DAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82251DB4: 419A0054  beq cr6, 0x82251e08
	if ctx.cr[6].eq {
	pc = 0x82251E08; continue 'dispatch;
	}
	// 82251DB8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82251DBC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82251DC0: 40980010  bge cr6, 0x82251dd0
	if !ctx.cr[6].lt {
	pc = 0x82251DD0; continue 'dispatch;
	}
	// 82251DC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82251DC8: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82251DCC: 48000010  b 0x82251ddc
	pc = 0x82251DDC; continue 'dispatch;
	// 82251DD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82251DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251DD8: 4BFFD379  bl 0x8224f150
	ctx.lr = 0x82251DDC;
	sub_8224F150(ctx, base);
	// 82251DDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82251DE0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82251DE4: 409A0010  bne cr6, 0x82251df4
	if !ctx.cr[6].eq {
	pc = 0x82251DF4; continue 'dispatch;
	}
	// 82251DE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82251DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251DF0: 4BFFD2A1  bl 0x8224f090
	ctx.lr = 0x82251DF4;
	sub_8224F090(ctx, base);
	// 82251DF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82251DF8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82251DFC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82251E00: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82251E04: 4198FFC0  blt cr6, 0x82251dc4
	if ctx.cr[6].lt {
	pc = 0x82251DC4; continue 'dispatch;
	}
	// 82251E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82251E0C: 4BE3CCDC  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251E10 size=92
    let mut pc: u32 = 0x82251E10;
    'dispatch: loop {
        match pc {
            0x82251E10 => {
    //   block [0x82251E10..0x82251E6C)
	// 82251E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82251E18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82251E1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251E24: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82251E28: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 82251E2C: 409A0018  bne cr6, 0x82251e44
	if !ctx.cr[6].eq {
	pc = 0x82251E44; continue 'dispatch;
	}
	// 82251E30: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 82251E34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82251E38: 816B0AB0  lwz r11, 0xab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82251E3C: 806B00AC  lwz r3, 0xac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82251E40: 4BFEEFF9  bl 0x82240e38
	ctx.lr = 0x82251E44;
	sub_82240E38(ctx, base);
	// 82251E44: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82251E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82251E4C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82251E50: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82251E54: 4BFFD045  bl 0x8224ee98
	ctx.lr = 0x82251E58;
	sub_8224EE98(ctx, base);
	// 82251E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82251E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82251E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251E70 size=56
    let mut pc: u32 = 0x82251E70;
    'dispatch: loop {
        match pc {
            0x82251E70 => {
    //   block [0x82251E70..0x82251EA8)
	// 82251E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82251E78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82251E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251E80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82251E84: 4BFFD015  bl 0x8224ee98
	ctx.lr = 0x82251E88;
	sub_8224EE98(ctx, base);
	// 82251E88: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82251E8C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82251E90: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82251E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82251E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251EA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82251EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82251EA8 size=64
    let mut pc: u32 = 0x82251EA8;
    'dispatch: loop {
        match pc {
            0x82251EA8 => {
    //   block [0x82251EA8..0x82251EE8)
	// 82251EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82251EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82251EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82251EB4: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82251EB8: 2F0B0089  cmpwi cr6, r11, 0x89
	ctx.cr[6].compare_i32(ctx.r[11].s32, 137, &mut ctx.xer);
	// 82251EBC: 409A0014  bne cr6, 0x82251ed0
	if !ctx.cr[6].eq {
	pc = 0x82251ED0; continue 'dispatch;
	}
	// 82251EC0: 4BFF21E1  bl 0x822440a0
	ctx.lr = 0x82251EC4;
	sub_822440A0(ctx, base);
	// 82251EC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82251EC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82251ECC: 40820008  bne 0x82251ed4
	if !ctx.cr[0].eq {
	pc = 0x82251ED4; continue 'dispatch;
	}
	// 82251ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82251ED4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82251ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82251EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82251EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82251EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251EE8 size=36
    let mut pc: u32 = 0x82251EE8;
    'dispatch: loop {
        match pc {
            0x82251EE8 => {
    //   block [0x82251EE8..0x82251F0C)
	// 82251EE8: 816303BC  lwz r11, 0x3bc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(956 as u32) ) } as u64;
	// 82251EEC: 81440868  lwz r10, 0x868(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2152 as u32) ) } as u64;
	// 82251EF0: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82251EF4: 40800008  bge 0x82251efc
	if !ctx.cr[0].lt {
	pc = 0x82251EFC; continue 'dispatch;
	}
	// 82251EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82251EFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82251F00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82251F04: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82251F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251F10 size=28
    let mut pc: u32 = 0x82251F10;
    'dispatch: loop {
        match pc {
            0x82251F10 => {
    //   block [0x82251F10..0x82251F2C)
	// 82251F10: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82251F14: 409A0018  bne cr6, 0x82251f2c
	if !ctx.cr[6].eq {
		sub_82251F2C(ctx, base);
		return;
	}
	// 82251F18: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82251F1C: 81430080  lwz r10, 0x80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82251F20: 396B51B0  addi r11, r11, 0x51b0
	ctx.r[11].s64 = ctx.r[11].s64 + 20912;
	// 82251F24: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82251F28: 48000018  b 0x82251f40
	sub_82251F2C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251F2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251F2C size=40
    let mut pc: u32 = 0x82251F2C;
    'dispatch: loop {
        match pc {
            0x82251F2C => {
    //   block [0x82251F2C..0x82251F54)
	// 82251F2C: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82251F30: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82251F34: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82251F38: 816A51B0  lwz r11, 0x51b0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20912 as u32) ) } as u64;
	// 82251F3C: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82251F40: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82251F44: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82251F48: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82251F4C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82251F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251F58 size=52
    let mut pc: u32 = 0x82251F58;
    'dispatch: loop {
        match pc {
            0x82251F58 => {
    //   block [0x82251F58..0x82251F8C)
	// 82251F58: 81640AB0  lwz r11, 0xab0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82251F5C: 896B0860  lbz r11, 0x860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2144 as u32) ) } as u64;
	// 82251F60: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82251F64: 4182008C  beq 0x82251ff0
	if ctx.cr[0].eq {
		sub_82251FF0(ctx, base);
		return;
	}
	// 82251F68: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 82251F6C: 41990034  bgt cr6, 0x82251fa0
	if ctx.cr[6].gt {
		sub_82251FA0(ctx, base);
		return;
	}
	// 82251F70: 2F030013  cmpwi cr6, r3, 0x13
	ctx.cr[6].compare_i32(ctx.r[3].s32, 19, &mut ctx.xer);
	// 82251F74: 40980054  bge cr6, 0x82251fc8
	if !ctx.cr[6].lt {
		sub_82251FA0(ctx, base);
		return;
	}
	// 82251F78: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82251F7C: 419A004C  beq cr6, 0x82251fc8
	if ctx.cr[6].eq {
		sub_82251FA0(ctx, base);
		return;
	}
	// 82251F80: 40990068  ble cr6, 0x82251fe8
	if !ctx.cr[6].gt {
		sub_82251FE8(ctx, base);
		return;
	}
	// 82251F84: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82251F88: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251F8C size=16
    let mut pc: u32 = 0x82251F8C;
    'dispatch: loop {
        match pc {
            0x82251F8C => {
    //   block [0x82251F8C..0x82251F9C)
	// 82251F8C: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82251F90: 419A0040  beq cr6, 0x82251fd0
	if ctx.cr[6].eq {
		sub_82251FD0(ctx, base);
		return;
	}
	// 82251F94: 2F030009  cmpwi cr6, r3, 9
	ctx.cr[6].compare_i32(ctx.r[3].s32, 9, &mut ctx.xer);
	// 82251F98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251F9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251F9C size=4
    let mut pc: u32 = 0x82251F9C;
    'dispatch: loop {
        match pc {
            0x82251F9C => {
    //   block [0x82251F9C..0x82251FA0)
	// 82251F9C: 4800004C  b 0x82251fe8
	sub_82251FE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251FA0 size=48
    let mut pc: u32 = 0x82251FA0;
    'dispatch: loop {
        match pc {
            0x82251FA0 => {
    //   block [0x82251FA0..0x82251FD0)
	// 82251FA0: 2F030021  cmpwi cr6, r3, 0x21
	ctx.cr[6].compare_i32(ctx.r[3].s32, 33, &mut ctx.xer);
	// 82251FA4: 41990034  bgt cr6, 0x82251fd8
	if ctx.cr[6].gt {
		sub_82251FD8(ctx, base);
		return;
	}
	// 82251FA8: 2F030020  cmpwi cr6, r3, 0x20
	ctx.cr[6].compare_i32(ctx.r[3].s32, 32, &mut ctx.xer);
	// 82251FAC: 4098001C  bge cr6, 0x82251fc8
	if !ctx.cr[6].lt {
	pc = 0x82251FC8; continue 'dispatch;
	}
	// 82251FB0: 2F030015  cmpwi cr6, r3, 0x15
	ctx.cr[6].compare_i32(ctx.r[3].s32, 21, &mut ctx.xer);
	// 82251FB4: 41980034  blt cr6, 0x82251fe8
	if ctx.cr[6].lt {
		sub_82251FE8(ctx, base);
		return;
	}
	// 82251FB8: 2F030016  cmpwi cr6, r3, 0x16
	ctx.cr[6].compare_i32(ctx.r[3].s32, 22, &mut ctx.xer);
	// 82251FBC: 40990014  ble cr6, 0x82251fd0
	if !ctx.cr[6].gt {
		sub_82251FD0(ctx, base);
		return;
	}
	// 82251FC0: 2F030018  cmpwi cr6, r3, 0x18
	ctx.cr[6].compare_i32(ctx.r[3].s32, 24, &mut ctx.xer);
	// 82251FC4: 41990024  bgt cr6, 0x82251fe8
	if ctx.cr[6].gt {
		sub_82251FE8(ctx, base);
		return;
	}
	// 82251FC8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82251FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251FD0 size=8
    let mut pc: u32 = 0x82251FD0;
    'dispatch: loop {
        match pc {
            0x82251FD0 => {
    //   block [0x82251FD0..0x82251FD8)
	// 82251FD0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82251FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251FD8 size=16
    let mut pc: u32 = 0x82251FD8;
    'dispatch: loop {
        match pc {
            0x82251FD8 => {
    //   block [0x82251FD8..0x82251FE8)
	// 82251FD8: 2F03002B  cmpwi cr6, r3, 0x2b
	ctx.cr[6].compare_i32(ctx.r[3].s32, 43, &mut ctx.xer);
	// 82251FDC: 4198000C  blt cr6, 0x82251fe8
	if ctx.cr[6].lt {
		sub_82251FE8(ctx, base);
		return;
	}
	// 82251FE0: 2F03002C  cmpwi cr6, r3, 0x2c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 44, &mut ctx.xer);
	// 82251FE4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251FE8 size=8
    let mut pc: u32 = 0x82251FE8;
    'dispatch: loop {
        match pc {
            0x82251FE8 => {
    //   block [0x82251FE8..0x82251FF0)
	// 82251FE8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82251FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82251FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82251FF0 size=20
    let mut pc: u32 = 0x82251FF0;
    'dispatch: loop {
        match pc {
            0x82251FF0 => {
    //   block [0x82251FF0..0x82252004)
	// 82251FF0: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82251FF4: 419A0010  beq cr6, 0x82252004
	if ctx.cr[6].eq {
		sub_82252004(ctx, base);
		return;
	}
	// 82251FF8: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82251FFC: 419AFFD4  beq cr6, 0x82251fd0
	if ctx.cr[6].eq {
		sub_82251FD0(ctx, base);
		return;
	}
	// 82252000: 4BFFFFE8  b 0x82251fe8
	sub_82251FE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252004 size=8
    let mut pc: u32 = 0x82252004;
    'dispatch: loop {
        match pc {
            0x82252004 => {
    //   block [0x82252004..0x8225200C)
	// 82252004: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82252008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252010 size=200
    let mut pc: u32 = 0x82252010;
    'dispatch: loop {
        match pc {
            0x82252010 => {
    //   block [0x82252010..0x822520D8)
	// 82252010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225201C: 3944003A  addi r10, r4, 0x3a
	ctx.r[10].s64 = ctx.r[4].s64 + 58;
	// 82252020: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82252024: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82252028: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8225202C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82252030: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82252034: 4800002C  b 0x82252060
	pc = 0x82252060; continue 'dispatch;
	// 82252038: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8225203C: 4082002C  bne 0x82252068
	if !ctx.cr[0].eq {
	pc = 0x82252068; continue 'dispatch;
	}
	// 82252040: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252044: 554807FF  clrlwi. r8, r10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82252048: 4182000C  beq 0x82252054
	if ctx.cr[0].eq {
	pc = 0x82252054; continue 'dispatch;
	}
	// 8225204C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82252050: 419A002C  beq cr6, 0x8225207c
	if ctx.cr[6].eq {
	pc = 0x8225207C; continue 'dispatch;
	}
	// 82252054: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82252058: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8225205C: 554AF7FE  rlwinm r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82252060: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82252064: 409AFFD4  bne cr6, 0x82252038
	if !ctx.cr[6].eq {
	pc = 0x82252038; continue 'dispatch;
	}
	// 82252068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8225206C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252078: 4E800020  blr
	return;
	// 8225207C: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82252080: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82252084: 409A0010  bne cr6, 0x82252094
	if !ctx.cr[6].eq {
	pc = 0x82252094; continue 'dispatch;
	}
	// 82252088: 816900E4  lwz r11, 0xe4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(228 as u32) ) } as u64;
	// 8225208C: 556BBFFF  rlwinm. r11, r11, 0x17, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82252090: 40820040  bne 0x822520d0
	if !ctx.cr[0].eq {
	pc = 0x822520D0; continue 'dispatch;
	}
	// 82252094: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82252098: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8225209C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822520A0: 4E800421  bctrl
	ctx.lr = 0x822520A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822520A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822520A8: 40820028  bne 0x822520d0
	if !ctx.cr[0].eq {
	pc = 0x822520D0; continue 'dispatch;
	}
	// 822520AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822520B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822520B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 822520B8: 38CB51C8  addi r6, r11, 0x51c8
	ctx.r[6].s64 = ctx.r[11].s64 + 20936;
	// 822520BC: 38AA52C8  addi r5, r10, 0x52c8
	ctx.r[5].s64 = ctx.r[10].s64 + 21192;
	// 822520C0: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 822520C4: 38E00287  li r7, 0x287
	ctx.r[7].s64 = 647;
	// 822520C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822520CC: 4BF0393D  bl 0x82155a08
	ctx.lr = 0x822520D0;
	sub_82155A08(ctx, base);
	// 822520D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822520D4: 4BFFFF98  b 0x8225206c
	pc = 0x8225206C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822520D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822520D8 size=44
    let mut pc: u32 = 0x822520D8;
    'dispatch: loop {
        match pc {
            0x822520D8 => {
    //   block [0x822520D8..0x82252104)
	// 822520D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822520DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822520E0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 822520E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822520E8: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 822520EC: 90CB0038  stw r6, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 822520F0: 614A0042  ori r10, r10, 0x42
	ctx.r[10].u64 = ctx.r[10].u64 | 66;
	// 822520F4: 90AB0050  stw r5, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 822520F8: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 822520FC: 914B00E4  stw r10, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82252100: 4BFDF928  b 0x82231a28
	sub_82231A28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252108 size=28
    let mut pc: u32 = 0x82252108;
    'dispatch: loop {
        match pc {
            0x82252108 => {
    //   block [0x82252108..0x82252124)
	// 82252108: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8225210C: 908300A8  stw r4, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[4].u32 ) };
	// 82252110: 99630084  stb r11, 0x84(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 82252114: 99630085  stb r11, 0x85(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 82252118: 99630086  stb r11, 0x86(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 8225211C: 99630087  stb r11, 0x87(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 82252120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252128 size=120
    let mut pc: u32 = 0x82252128;
    'dispatch: loop {
        match pc {
            0x82252128 => {
    //   block [0x82252128..0x822521A0)
	// 82252128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225212C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82252134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225213C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82252140: 807E00B4  lwz r3, 0xb4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82252144: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82252148: 409A0040  bne cr6, 0x82252188
	if !ctx.cr[6].eq {
	pc = 0x82252188; continue 'dispatch;
	}
	// 8225214C: 83FE00EC  lwz r31, 0xec(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82252150: 48000024  b 0x82252174
	pc = 0x82252174; continue 'dispatch;
	// 82252154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82252158: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8225215C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82252160: 4E800421  bctrl
	ctx.lr = 0x82252164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82252164: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82252168: 41820018  beq 0x82252180
	if ctx.cr[0].eq {
	pc = 0x82252180; continue 'dispatch;
	}
	// 8225216C: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82252170: 83EB00EC  lwz r31, 0xec(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(236 as u32) ) } as u64;
	// 82252174: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82252178: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225217C: 409AFFD8  bne cr6, 0x82252154
	if !ctx.cr[6].eq {
	pc = 0x82252154; continue 'dispatch;
	}
	// 82252180: 93FE00B4  stw r31, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 82252184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82252188: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225218C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252194: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82252198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225219C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822521A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822521A0 size=140
    let mut pc: u32 = 0x822521A0;
    'dispatch: loop {
        match pc {
            0x822521A0 => {
    //   block [0x822521A0..0x8225222C)
	// 822521A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822521A4: 4BE3C8F5  bl 0x8208ea98
	ctx.lr = 0x822521A8;
	sub_8208EA60(ctx, base);
	// 822521A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822521AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822521B0: 3D408222  lis r10, -0x7dde
	ctx.r[10].s64 = -2111700992;
	// 822521B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822521B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822521BC: 38EA8930  addi r7, r10, -0x76d0
	ctx.r[7].s64 = ctx.r[10].s64 + -30416;
	// 822521C0: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 822521C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822521C8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822521CC: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822521D0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 822521D4: 80CB0568  lwz r6, 0x568(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1384 as u32) ) } as u64;
	// 822521D8: 806B0600  lwz r3, 0x600(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 822521DC: 4BFDC085  bl 0x8222e260
	ctx.lr = 0x822521E0;
	sub_8222E260(ctx, base);
	// 822521E0: 3D608222  lis r11, -0x7dde
	ctx.r[11].s64 = -2111700992;
	// 822521E4: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822521E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822521EC: 38EB8930  addi r7, r11, -0x76d0
	ctx.r[7].s64 = ctx.r[11].s64 + -30416;
	// 822521F0: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 822521F4: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 822521F8: 80CB0568  lwz r6, 0x568(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1384 as u32) ) } as u64;
	// 822521FC: 806B0600  lwz r3, 0x600(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82252200: 4BFDC501  bl 0x8222e700
	ctx.lr = 0x82252204;
	sub_8222E700(ctx, base);
	// 82252204: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82252208: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8225220C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82252210: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82252214: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252218: 815F00A8  lwz r10, 0xa8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8225221C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82252220: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252224: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82252228: 4BE3C8C0  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252230 size=296
    let mut pc: u32 = 0x82252230;
    'dispatch: loop {
        match pc {
            0x82252230 => {
    //   block [0x82252230..0x82252358)
	// 82252230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252234: 4BE3C859  bl 0x8208ea8c
	ctx.lr = 0x82252238;
	sub_8208EA60(ctx, base);
	// 82252238: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225223C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252240: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82252244: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82252248: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8225224C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82252250: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82252254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82252258: 4E800421  bctrl
	ctx.lr = 0x8225225C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8225225C: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82252260: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82252264: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82252268: 396B4720  addi r11, r11, 0x4720
	ctx.r[11].s64 = ctx.r[11].s64 + 18208;
	// 8225226C: 1D090034  mulli r8, r9, 0x34
	ctx.r[8].s64 = ctx.r[9].s64 * 52;
	// 82252270: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82252274: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82252278: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8225227C: 3BCA54C0  addi r30, r10, 0x54c0
	ctx.r[30].s64 = ctx.r[10].s64 + 21696;
	// 82252280: 3BA951C8  addi r29, r9, 0x51c8
	ctx.r[29].s64 = ctx.r[9].s64 + 20936;
	// 82252284: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82252288: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8225228C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82252290: 409A0020  bne cr6, 0x822522b0
	if !ctx.cr[6].eq {
	pc = 0x822522B0; continue 'dispatch;
	}
	// 82252294: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252298: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8225229C: 38AB5348  addi r5, r11, 0x5348
	ctx.r[5].s64 = ctx.r[11].s64 + 21320;
	// 822522A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822522A4: 38E00598  li r7, 0x598
	ctx.r[7].s64 = 1432;
	// 822522A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822522AC: 4BF0375D  bl 0x82155a08
	ctx.lr = 0x822522B0;
	sub_82155A08(ctx, base);
	// 822522B0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822522B4: 4182006C  beq 0x82252320
	if ctx.cr[0].eq {
	pc = 0x82252320; continue 'dispatch;
	}
	// 822522B8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822522BC: 2F0B0034  cmpwi cr6, r11, 0x34
	ctx.cr[6].compare_i32(ctx.r[11].s32, 52, &mut ctx.xer);
	// 822522C0: 419A0020  beq cr6, 0x822522e0
	if ctx.cr[6].eq {
	pc = 0x822522E0; continue 'dispatch;
	}
	// 822522C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822522C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 822522CC: 38AB5330  addi r5, r11, 0x5330
	ctx.r[5].s64 = ctx.r[11].s64 + 21296;
	// 822522D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822522D4: 38E0059A  li r7, 0x59a
	ctx.r[7].s64 = 1434;
	// 822522D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822522DC: 4BF0372D  bl 0x82155a08
	ctx.lr = 0x822522E0;
	sub_82155A08(ctx, base);
	// 822522E0: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 822522E4: 39400035  li r10, 0x35
	ctx.r[10].s64 = 53;
	// 822522E8: 893F0099  lbz r9, 0x99(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(153 as u32) ) } as u64;
	// 822522EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822522F0: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 822522F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822522F8: 997F009A  stb r11, 0x9a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 822522FC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82252300: 419A0020  beq cr6, 0x82252320
	if ctx.cr[6].eq {
	pc = 0x82252320; continue 'dispatch;
	}
	// 82252304: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252308: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8225230C: 38AB530C  addi r5, r11, 0x530c
	ctx.r[5].s64 = ctx.r[11].s64 + 21260;
	// 82252310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82252314: 38E0059E  li r7, 0x59e
	ctx.r[7].s64 = 1438;
	// 82252318: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8225231C: 4BF036ED  bl 0x82155a08
	ctx.lr = 0x82252320;
	sub_82155A08(ctx, base);
	// 82252320: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82252324: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 82252328: 997F0084  stb r11, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 8225232C: 997F0085  stb r11, 0x85(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 82252330: 997F0086  stb r11, 0x86(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 82252334: 997F0087  stb r11, 0x87(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 82252338: 409A0018  bne cr6, 0x82252350
	if !ctx.cr[6].eq {
	pc = 0x82252350; continue 'dispatch;
	}
	// 8225233C: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82252340: 997F0088  stb r11, 0x88(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 82252344: 997F0089  stb r11, 0x89(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 82252348: 997F008A  stb r11, 0x8a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 8225234C: 997F008B  stb r11, 0x8b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(139 as u32), ctx.r[11].u8 ) };
	// 82252350: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82252354: 4BE3C788  b 0x8208eadc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252358 size=188
    let mut pc: u32 = 0x82252358;
    'dispatch: loop {
        match pc {
            0x82252358 => {
    //   block [0x82252358..0x82252414)
	// 82252358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225235C: 4BE3C73D  bl 0x8208ea98
	ctx.lr = 0x82252360;
	sub_8208EA60(ctx, base);
	// 82252360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252368: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 8225236C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82252370: 394A4720  addi r10, r10, 0x4720
	ctx.r[10].s64 = ctx.r[10].s64 + 18208;
	// 82252374: 3BC954C0  addi r30, r9, 0x54c0
	ctx.r[30].s64 = ctx.r[9].s64 + 21696;
	// 82252378: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8225237C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82252380: 1D0B0034  mulli r8, r11, 0x34
	ctx.r[8].s64 = ctx.r[11].s64 * 52;
	// 82252384: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82252388: 38EBFFCB  addi r7, r11, -0x35
	ctx.r[7].s64 = ctx.r[11].s64 + -53;
	// 8225238C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252390: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82252394: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82252398: 3BAB51C8  addi r29, r11, 0x51c8
	ctx.r[29].s64 = ctx.r[11].s64 + 20936;
	// 8225239C: 54FCDFFE  rlwinm r28, r7, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 822523A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822523A4: 409A0020  bne cr6, 0x822523c4
	if !ctx.cr[6].eq {
	pc = 0x822523C4; continue 'dispatch;
	}
	// 822523A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822523AC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 822523B0: 38AB5348  addi r5, r11, 0x5348
	ctx.r[5].s64 = ctx.r[11].s64 + 21320;
	// 822523B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822523B8: 38E005B9  li r7, 0x5b9
	ctx.r[7].s64 = 1465;
	// 822523BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822523C0: 4BF03649  bl 0x82155a08
	ctx.lr = 0x822523C4;
	sub_82155A08(ctx, base);
	// 822523C4: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822523C8: 41820040  beq 0x82252408
	if ctx.cr[0].eq {
	pc = 0x82252408; continue 'dispatch;
	}
	// 822523CC: 897F0099  lbz r11, 0x99(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(153 as u32) ) } as u64;
	// 822523D0: 895F009A  lbz r10, 0x9a(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 822523D4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822523D8: 419A0020  beq cr6, 0x822523f8
	if ctx.cr[6].eq {
	pc = 0x822523F8; continue 'dispatch;
	}
	// 822523DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822523E0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 822523E4: 38AB530C  addi r5, r11, 0x530c
	ctx.r[5].s64 = ctx.r[11].s64 + 21260;
	// 822523E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822523EC: 38E005BC  li r7, 0x5bc
	ctx.r[7].s64 = 1468;
	// 822523F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822523F4: 4BF03615  bl 0x82155a08
	ctx.lr = 0x822523F8;
	sub_82155A08(ctx, base);
	// 822523F8: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 822523FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82252400: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82252404: 997F009A  stb r11, 0x9a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 82252408: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8225240C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82252410: 4BE3C6D8  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252418 size=116
    let mut pc: u32 = 0x82252418;
    'dispatch: loop {
        match pc {
            0x82252418 => {
    //   block [0x82252418..0x8225248C)
	// 82252418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8225242C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82252434: 394A1F70  addi r10, r10, 0x1f70
	ctx.r[10].s64 = ctx.r[10].s64 + 8048;
	// 82252438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8225243C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252440: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82252444: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82252448: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8225244C: 81450560  lwz r10, 0x560(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82252450: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82252454: 81450560  lwz r10, 0x560(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82252458: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8225245C: 91450560  stw r10, 0x560(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(1376 as u32), ctx.r[10].u32 ) };
	// 82252460: 917F037C  stw r11, 0x37c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(892 as u32), ctx.r[11].u32 ) };
	// 82252464: 917F03B4  stw r11, 0x3b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(948 as u32), ctx.r[11].u32 ) };
	// 82252468: 90BF03B8  stw r5, 0x3b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(952 as u32), ctx.r[5].u32 ) };
	// 8225246C: 917F03BC  stw r11, 0x3bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(956 as u32), ctx.r[11].u32 ) };
	// 82252470: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 82252474: 4BFFF355  bl 0x822517c8
	ctx.lr = 0x82252478;
	sub_822517C8(ctx, base);
	// 82252478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8225247C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252490 size=176
    let mut pc: u32 = 0x82252490;
    'dispatch: loop {
        match pc {
            0x82252490 => {
    //   block [0x82252490..0x82252540)
	// 82252490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252494: 4BE3C609  bl 0x8208ea9c
	ctx.lr = 0x82252498;
	sub_8208EA60(ctx, base);
	// 82252498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225249C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822524A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822524A4: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 822524A8: 896B0571  lbz r11, 0x571(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1393 as u32) ) } as u64;
	// 822524AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822524B0: 41820028  beq 0x822524d8
	if ctx.cr[0].eq {
	pc = 0x822524D8; continue 'dispatch;
	}
	// 822524B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822524B8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822524BC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 822524C0: 38CB51C8  addi r6, r11, 0x51c8
	ctx.r[6].s64 = ctx.r[11].s64 + 20936;
	// 822524C4: 38AA12F8  addi r5, r10, 0x12f8
	ctx.r[5].s64 = ctx.r[10].s64 + 4856;
	// 822524C8: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 822524CC: 38E000B8  li r7, 0xb8
	ctx.r[7].s64 = 184;
	// 822524D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822524D4: 4BF03535  bl 0x82155a08
	ctx.lr = 0x822524D8;
	sub_82155A08(ctx, base);
	// 822524D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822524DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822524E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822524E4: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 822524E8: 4BFFF439  bl 0x82251920
	ctx.lr = 0x822524EC;
	sub_82251920(ctx, base);
	// 822524EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822524F0: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 822524F4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 822524F8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822524FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82252500: 40990038  ble cr6, 0x82252538
	if !ctx.cr[6].gt {
	pc = 0x82252538; continue 'dispatch;
	}
	// 82252504: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82252508: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8225250C: 409A002C  bne cr6, 0x82252538
	if !ctx.cr[6].eq {
	pc = 0x82252538; continue 'dispatch;
	}
	// 82252510: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82252514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82252518: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8225251C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82252520: 4E800421  bctrl
	ctx.lr = 0x82252524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82252524: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82252528: 40990010  ble cr6, 0x82252538
	if !ctx.cr[6].gt {
	pc = 0x82252538; continue 'dispatch;
	}
	// 8225252C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252530: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82252534: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82252538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225253C: 4BE3C5B0  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82252540 size=84
    let mut pc: u32 = 0x82252540;
    'dispatch: loop {
        match pc {
            0x82252540 => {
    //   block [0x82252540..0x82252594)
	// 82252540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225254C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82252550: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82252554: D041005C  stfs f2, 0x5c(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82252558: D0610064  stfs f3, 0x64(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8225255C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82252560: D081006C  stfs f4, 0x6c(r1)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82252564: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82252568: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8225256C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82252570: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82252574: E8E10058  ld r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82252578: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8225257C: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82252580: 4BFFF419  bl 0x82251998
	ctx.lr = 0x82252584;
	sub_82251998(ctx, base);
	// 82252584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82252588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8225258C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252598 size=84
    let mut pc: u32 = 0x82252598;
    'dispatch: loop {
        match pc {
            0x82252598 => {
    //   block [0x82252598..0x822525EC)
	// 82252598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822525A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822525A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822525A8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 822525AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 822525B0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 822525B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 822525B8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822525BC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 822525C0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822525C4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 822525C8: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 822525CC: E8E10058  ld r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 822525D0: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 822525D4: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 822525D8: 4BFFF3C1  bl 0x82251998
	ctx.lr = 0x822525DC;
	sub_82251998(ctx, base);
	// 822525DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822525E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822525E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822525E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822525F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822525F0 size=92
    let mut pc: u32 = 0x822525F0;
    'dispatch: loop {
        match pc {
            0x822525F0 => {
    //   block [0x822525F0..0x8225264C)
	// 822525F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822525F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822525F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822525FC: 4BFFFE1D  bl 0x82252418
	ctx.lr = 0x82252600;
	sub_82252418(ctx, base);
	// 82252600: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252604: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82252608: 392B5430  addi r9, r11, 0x5430
	ctx.r[9].s64 = ctx.r[11].s64 + 21552;
	// 8225260C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82252610: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82252614: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82252618: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8225261C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82252620: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82252624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82252628: 912BFFE8  stw r9, -0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24 as u32), ctx.r[9].u32 ) };
	// 8225262C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82252630: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82252634: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82252638: 4082FFE8  bne 0x82252620
	if !ctx.cr[0].eq {
	pc = 0x82252620; continue 'dispatch;
	}
	// 8225263C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252650 size=76
    let mut pc: u32 = 0x82252650;
    'dispatch: loop {
        match pc {
            0x82252650 => {
    //   block [0x82252650..0x8225269C)
	// 82252650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8225265C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252660: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82252664: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 82252668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8225266C: 4BFFFF85  bl 0x822525f0
	ctx.lr = 0x82252670;
	sub_822525F0(ctx, base);
	// 82252670: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252674: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82252678: 394A5430  addi r10, r10, 0x5430
	ctx.r[10].s64 = ctx.r[10].s64 + 21552;
	// 8225267C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82252680: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252684: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82252688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8225268C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822526A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822526A0 size=84
    let mut pc: u32 = 0x822526A0;
    'dispatch: loop {
        match pc {
            0x822526A0 => {
    //   block [0x822526A0..0x822526F4)
	// 822526A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822526A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822526A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822526AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822526B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822526B4: 4BFFFD65  bl 0x82252418
	ctx.lr = 0x822526B8;
	sub_82252418(ctx, base);
	// 822526B8: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 822526BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822526C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822526C4: 39295498  addi r9, r9, 0x5498
	ctx.r[9].s64 = ctx.r[9].s64 + 21656;
	// 822526C8: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 822526CC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 822526D0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822526D4: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 822526D8: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 822526DC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822526E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822526E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822526E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822526EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822526F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822526F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822526F8 size=12
    let mut pc: u32 = 0x822526F8;
    'dispatch: loop {
        match pc {
            0x822526F8 => {
    //   block [0x822526F8..0x82252704)
	// 822526F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822526FC: 386B5500  addi r3, r11, 0x5500
	ctx.r[3].s64 = ctx.r[11].s64 + 21760;
	// 82252700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252708 size=108
    let mut pc: u32 = 0x82252708;
    'dispatch: loop {
        match pc {
            0x82252708 => {
    //   block [0x82252708..0x82252774)
	// 82252708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225270C: 4BE3C391  bl 0x8208ea9c
	ctx.lr = 0x82252710;
	sub_8208EA60(ctx, base);
	// 82252710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252714: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82252718: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8225271C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82252720: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82252724: 4BFFFCF5  bl 0x82252418
	ctx.lr = 0x82252728;
	sub_82252418(ctx, base);
	// 82252728: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225272C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82252730: 396B5508  addi r11, r11, 0x5508
	ctx.r[11].s64 = ctx.r[11].s64 + 21768;
	// 82252734: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82252738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225273C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252740: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252744: 616B0012  ori r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u64 | 18;
	// 82252748: 917E00E4  stw r11, 0xe4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8225274C: 807D0AB0  lwz r3, 0xab0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82252750: 4BFDF2D9  bl 0x82231a28
	ctx.lr = 0x82252754;
	sub_82231A28(ctx, base);
	// 82252754: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82252758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8225275C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82252760: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82252764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82252768: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8225276C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252770: 4BE3C37C  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252778 size=12
    let mut pc: u32 = 0x82252778;
    'dispatch: loop {
        match pc {
            0x82252778 => {
    //   block [0x82252778..0x82252784)
	// 82252778: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8225277C: 386B4FF4  addi r3, r11, 0x4ff4
	ctx.r[3].s64 = ctx.r[11].s64 + 20468;
	// 82252780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252788 size=84
    let mut pc: u32 = 0x82252788;
    'dispatch: loop {
        match pc {
            0x82252788 => {
    //   block [0x82252788..0x822527DC)
	// 82252788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225278C: 4BE3C311  bl 0x8208ea9c
	ctx.lr = 0x82252790;
	sub_8208EA60(ctx, base);
	// 82252790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82252798: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8225279C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822527A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822527A4: 4BFFFF65  bl 0x82252708
	ctx.lr = 0x822527A8;
	sub_82252708(ctx, base);
	// 822527A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822527AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822527B0: 396B5570  addi r11, r11, 0x5570
	ctx.r[11].s64 = ctx.r[11].s64 + 21872;
	// 822527B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822527B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822527BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822527C0: 4BFFF161  bl 0x82251920
	ctx.lr = 0x822527C4;
	sub_82251920(ctx, base);
	// 822527C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822527C8: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 822527CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822527D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822527D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822527D8: 4BE3C314  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822527E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822527E0 size=12
    let mut pc: u32 = 0x822527E0;
    'dispatch: loop {
        match pc {
            0x822527E0 => {
    //   block [0x822527E0..0x822527EC)
	// 822527E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822527E4: 386B55D8  addi r3, r11, 0x55d8
	ctx.r[3].s64 = ctx.r[11].s64 + 21976;
	// 822527E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822527F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822527F0 size=12
    let mut pc: u32 = 0x822527F0;
    'dispatch: loop {
        match pc {
            0x822527F0 => {
    //   block [0x822527F0..0x822527FC)
	// 822527F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822527F4: 386B55E4  addi r3, r11, 0x55e4
	ctx.r[3].s64 = ctx.r[11].s64 + 21988;
	// 822527F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252800 size=72
    let mut pc: u32 = 0x82252800;
    'dispatch: loop {
        match pc {
            0x82252800 => {
    //   block [0x82252800..0x82252848)
	// 82252800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8225280C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252810: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82252814: 38800082  li r4, 0x82
	ctx.r[4].s64 = 130;
	// 82252818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8225281C: 4BFFFBFD  bl 0x82252418
	ctx.lr = 0x82252820;
	sub_82252418(ctx, base);
	// 82252820: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82252828: 396B55F0  addi r11, r11, 0x55f0
	ctx.r[11].s64 = ctx.r[11].s64 + 22000;
	// 8225282C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82252830: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8225283C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252848 size=12
    let mut pc: u32 = 0x82252848;
    'dispatch: loop {
        match pc {
            0x82252848 => {
    //   block [0x82252848..0x82252854)
	// 82252848: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8225284C: 386B4F38  addi r3, r11, 0x4f38
	ctx.r[3].s64 = ctx.r[11].s64 + 20280;
	// 82252850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252858 size=72
    let mut pc: u32 = 0x82252858;
    'dispatch: loop {
        match pc {
            0x82252858 => {
    //   block [0x82252858..0x822528A0)
	// 82252858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252868: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8225286C: 38800083  li r4, 0x83
	ctx.r[4].s64 = 131;
	// 82252870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252874: 4BFFFBA5  bl 0x82252418
	ctx.lr = 0x82252878;
	sub_82252418(ctx, base);
	// 82252878: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225287C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82252880: 396B5658  addi r11, r11, 0x5658
	ctx.r[11].s64 = ctx.r[11].s64 + 22104;
	// 82252884: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82252888: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8225288C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225289C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822528A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822528A0 size=12
    let mut pc: u32 = 0x822528A0;
    'dispatch: loop {
        match pc {
            0x822528A0 => {
    //   block [0x822528A0..0x822528AC)
	// 822528A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822528A4: 386B56C0  addi r3, r11, 0x56c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22208;
	// 822528A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822528B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822528B0 size=84
    let mut pc: u32 = 0x822528B0;
    'dispatch: loop {
        match pc {
            0x822528B0 => {
    //   block [0x822528B0..0x82252904)
	// 822528B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822528B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822528B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822528BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822528C0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822528C4: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 822528C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822528CC: 4BFFFB4D  bl 0x82252418
	ctx.lr = 0x822528D0;
	sub_82252418(ctx, base);
	// 822528D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822528D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822528D8: 394A56C8  addi r10, r10, 0x56c8
	ctx.r[10].s64 = ctx.r[10].s64 + 22216;
	// 822528DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822528E0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822528E4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822528E8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822528EC: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 822528F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822528F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822528F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822528FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252908 size=12
    let mut pc: u32 = 0x82252908;
    'dispatch: loop {
        match pc {
            0x82252908 => {
    //   block [0x82252908..0x82252914)
	// 82252908: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225290C: 386B5730  addi r3, r11, 0x5730
	ctx.r[3].s64 = ctx.r[11].s64 + 22320;
	// 82252910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252918 size=116
    let mut pc: u32 = 0x82252918;
    'dispatch: loop {
        match pc {
            0x82252918 => {
    //   block [0x82252918..0x8225298C)
	// 82252918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82252924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225292C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82252930: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 82252934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252938: 4BFFFAE1  bl 0x82252418
	ctx.lr = 0x8225293C;
	sub_82252418(ctx, base);
	// 8225293C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82252944: 392B56C8  addi r9, r11, 0x56c8
	ctx.r[9].s64 = ctx.r[11].s64 + 22216;
	// 82252948: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8225294C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82252950: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82252954: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82252958: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8225295C: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82252960: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82252964: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82252968: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8225296C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82252970: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82252974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8225297C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82252984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252990 size=84
    let mut pc: u32 = 0x82252990;
    'dispatch: loop {
        match pc {
            0x82252990 => {
    //   block [0x82252990..0x822529E4)
	// 82252990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8225299C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822529A0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822529A4: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 822529A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822529AC: 4BFFFA6D  bl 0x82252418
	ctx.lr = 0x822529B0;
	sub_82252418(ctx, base);
	// 822529B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822529B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822529B8: 394A5740  addi r10, r10, 0x5740
	ctx.r[10].s64 = ctx.r[10].s64 + 22336;
	// 822529BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822529C0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822529C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822529C8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 822529CC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822529D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822529D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822529D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822529DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822529E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822529E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822529E8 size=12
    let mut pc: u32 = 0x822529E8;
    'dispatch: loop {
        match pc {
            0x822529E8 => {
    //   block [0x822529E8..0x822529F4)
	// 822529E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822529EC: 386B57A8  addi r3, r11, 0x57a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22440;
	// 822529F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822529F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822529F8 size=112
    let mut pc: u32 = 0x822529F8;
    'dispatch: loop {
        match pc {
            0x822529F8 => {
    //   block [0x822529F8..0x82252A68)
	// 822529F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822529FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252A00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82252A04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252A0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82252A10: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 82252A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252A18: 4BFFFA01  bl 0x82252418
	ctx.lr = 0x82252A1C;
	sub_82252418(ctx, base);
	// 82252A1C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252A20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82252A24: 394A5740  addi r10, r10, 0x5740
	ctx.r[10].s64 = ctx.r[10].s64 + 22336;
	// 82252A28: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82252A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82252A30: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252A34: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82252A38: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82252A3C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82252A40: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82252A44: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82252A48: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82252A4C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82252A50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252A5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82252A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252A68 size=92
    let mut pc: u32 = 0x82252A68;
    'dispatch: loop {
        match pc {
            0x82252A68 => {
    //   block [0x82252A68..0x82252AC4)
	// 82252A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252A78: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82252A7C: 3880007A  li r4, 0x7a
	ctx.r[4].s64 = 122;
	// 82252A80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252A84: 4BFFF995  bl 0x82252418
	ctx.lr = 0x82252A88;
	sub_82252418(ctx, base);
	// 82252A88: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252A8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82252A90: 396B57B8  addi r11, r11, 0x57b8
	ctx.r[11].s64 = ctx.r[11].s64 + 22456;
	// 82252A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82252A98: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82252A9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252AA0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82252AA4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252AA8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82252AAC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82252AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252ABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252AC8 size=12
    let mut pc: u32 = 0x82252AC8;
    'dispatch: loop {
        match pc {
            0x82252AC8 => {
    //   block [0x82252AC8..0x82252AD4)
	// 82252AC8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252ACC: 386B5820  addi r3, r11, 0x5820
	ctx.r[3].s64 = ctx.r[11].s64 + 22560;
	// 82252AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252AD8 size=72
    let mut pc: u32 = 0x82252AD8;
    'dispatch: loop {
        match pc {
            0x82252AD8 => {
    //   block [0x82252AD8..0x82252B20)
	// 82252AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252AE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252AE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252AE8: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82252AEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252AF0: 4BFFF929  bl 0x82252418
	ctx.lr = 0x82252AF4;
	sub_82252418(ctx, base);
	// 82252AF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252AF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82252AFC: 394A5830  addi r10, r10, 0x5830
	ctx.r[10].s64 = ctx.r[10].s64 + 22576;
	// 82252B00: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82252B04: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252B08: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82252B0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252B20 size=12
    let mut pc: u32 = 0x82252B20;
    'dispatch: loop {
        match pc {
            0x82252B20 => {
    //   block [0x82252B20..0x82252B2C)
	// 82252B20: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252B24: 386B5898  addi r3, r11, 0x5898
	ctx.r[3].s64 = ctx.r[11].s64 + 22680;
	// 82252B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252B30 size=72
    let mut pc: u32 = 0x82252B30;
    'dispatch: loop {
        match pc {
            0x82252B30 => {
    //   block [0x82252B30..0x82252B78)
	// 82252B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252B40: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82252B44: 38800089  li r4, 0x89
	ctx.r[4].s64 = 137;
	// 82252B48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252B4C: 4BFFF8CD  bl 0x82252418
	ctx.lr = 0x82252B50;
	sub_82252418(ctx, base);
	// 82252B50: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252B54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82252B58: 396B58A8  addi r11, r11, 0x58a8
	ctx.r[11].s64 = ctx.r[11].s64 + 22696;
	// 82252B5C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82252B60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252B70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252B78 size=12
    let mut pc: u32 = 0x82252B78;
    'dispatch: loop {
        match pc {
            0x82252B78 => {
    //   block [0x82252B78..0x82252B84)
	// 82252B78: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252B7C: 386B5910  addi r3, r11, 0x5910
	ctx.r[3].s64 = ctx.r[11].s64 + 22800;
	// 82252B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252B88 size=108
    let mut pc: u32 = 0x82252B88;
    'dispatch: loop {
        match pc {
            0x82252B88 => {
    //   block [0x82252B88..0x82252BF4)
	// 82252B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82252B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252B9C: 3880007D  li r4, 0x7d
	ctx.r[4].s64 = 125;
	// 82252BA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252BA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82252BA8: 4BFFF871  bl 0x82252418
	ctx.lr = 0x82252BAC;
	sub_82252418(ctx, base);
	// 82252BAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252BB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82252BB4: 396B5918  addi r11, r11, 0x5918
	ctx.r[11].s64 = ctx.r[11].s64 + 22808;
	// 82252BB8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82252BBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82252BC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252BC4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252BC8: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82252BCC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82252BD0: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82252BD4: 4BFDEE55  bl 0x82231a28
	ctx.lr = 0x82252BD8;
	sub_82231A28(ctx, base);
	// 82252BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82252BDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252BE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82252BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252BF8 size=12
    let mut pc: u32 = 0x82252BF8;
    'dispatch: loop {
        match pc {
            0x82252BF8 => {
    //   block [0x82252BF8..0x82252C04)
	// 82252BF8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252BFC: 386B5980  addi r3, r11, 0x5980
	ctx.r[3].s64 = ctx.r[11].s64 + 22912;
	// 82252C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252C08 size=72
    let mut pc: u32 = 0x82252C08;
    'dispatch: loop {
        match pc {
            0x82252C08 => {
    //   block [0x82252C08..0x82252C50)
	// 82252C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252C10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252C14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252C18: 3880007E  li r4, 0x7e
	ctx.r[4].s64 = 126;
	// 82252C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252C20: 4BFFF7F9  bl 0x82252418
	ctx.lr = 0x82252C24;
	sub_82252418(ctx, base);
	// 82252C24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252C28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82252C2C: 394A5988  addi r10, r10, 0x5988
	ctx.r[10].s64 = ctx.r[10].s64 + 22920;
	// 82252C30: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82252C34: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252C38: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82252C3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252C50 size=12
    let mut pc: u32 = 0x82252C50;
    'dispatch: loop {
        match pc {
            0x82252C50 => {
    //   block [0x82252C50..0x82252C5C)
	// 82252C50: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252C54: 386B59F0  addi r3, r11, 0x59f0
	ctx.r[3].s64 = ctx.r[11].s64 + 23024;
	// 82252C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252C60 size=76
    let mut pc: u32 = 0x82252C60;
    'dispatch: loop {
        match pc {
            0x82252C60 => {
    //   block [0x82252C60..0x82252CAC)
	// 82252C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252C74: 4BFFF7A5  bl 0x82252418
	ctx.lr = 0x82252C78;
	sub_82252418(ctx, base);
	// 82252C78: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252C7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82252C80: 394A5A00  addi r10, r10, 0x5a00
	ctx.r[10].s64 = ctx.r[10].s64 + 23040;
	// 82252C84: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82252C88: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82252C8C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252C90: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82252C94: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82252C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252CA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252CB0 size=12
    let mut pc: u32 = 0x82252CB0;
    'dispatch: loop {
        match pc {
            0x82252CB0 => {
    //   block [0x82252CB0..0x82252CBC)
	// 82252CB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252CB4: 386B5AD0  addi r3, r11, 0x5ad0
	ctx.r[3].s64 = ctx.r[11].s64 + 23248;
	// 82252CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252CC0 size=12
    let mut pc: u32 = 0x82252CC0;
    'dispatch: loop {
        match pc {
            0x82252CC0 => {
    //   block [0x82252CC0..0x82252CCC)
	// 82252CC0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252CC4: 386B5B48  addi r3, r11, 0x5b48
	ctx.r[3].s64 = ctx.r[11].s64 + 23368;
	// 82252CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252CD0 size=12
    let mut pc: u32 = 0x82252CD0;
    'dispatch: loop {
        match pc {
            0x82252CD0 => {
    //   block [0x82252CD0..0x82252CDC)
	// 82252CD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252CD4: 386B5BC0  addi r3, r11, 0x5bc0
	ctx.r[3].s64 = ctx.r[11].s64 + 23488;
	// 82252CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252CE0 size=12
    let mut pc: u32 = 0x82252CE0;
    'dispatch: loop {
        match pc {
            0x82252CE0 => {
    //   block [0x82252CE0..0x82252CEC)
	// 82252CE0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252CE4: 386B5C38  addi r3, r11, 0x5c38
	ctx.r[3].s64 = ctx.r[11].s64 + 23608;
	// 82252CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252CF0 size=12
    let mut pc: u32 = 0x82252CF0;
    'dispatch: loop {
        match pc {
            0x82252CF0 => {
    //   block [0x82252CF0..0x82252CFC)
	// 82252CF0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252CF4: 386BA9EC  addi r3, r11, -0x5614
	ctx.r[3].s64 = ctx.r[11].s64 + -22036;
	// 82252CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252D00 size=52
    let mut pc: u32 = 0x82252D00;
    'dispatch: loop {
        match pc {
            0x82252D00 => {
    //   block [0x82252D00..0x82252D34)
	// 82252D00: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82252D04: 41980030  blt cr6, 0x82252d34
	if ctx.cr[6].lt {
		sub_82252D34(ctx, base);
		return;
	}
	// 82252D08: 2F060005  cmpwi cr6, r6, 5
	ctx.cr[6].compare_i32(ctx.r[6].s32, 5, &mut ctx.xer);
	// 82252D0C: 41990028  bgt cr6, 0x82252d34
	if ctx.cr[6].gt {
		sub_82252D34(ctx, base);
		return;
	}
	// 82252D10: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252D14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82252D18: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82252D1C: 38CB03C8  addi r6, r11, 0x3c8
	ctx.r[6].s64 = ctx.r[11].s64 + 968;
	// 82252D20: 38AAFE08  addi r5, r10, -0x1f8
	ctx.r[5].s64 = ctx.r[10].s64 + -504;
	// 82252D24: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 82252D28: 38E006B0  li r7, 0x6b0
	ctx.r[7].s64 = 1712;
	// 82252D2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82252D30: 4BF02CD8  b 0x82155a08
	sub_82155A08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252D34 size=20
    let mut pc: u32 = 0x82252D34;
    'dispatch: loop {
        match pc {
            0x82252D34 => {
    //   block [0x82252D34..0x82252D48)
	// 82252D34: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82252D38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82252D3C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82252D40: 7CCB19AE  stbx r6, r11, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u8) };
	// 82252D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252D48 size=112
    let mut pc: u32 = 0x82252D48;
    'dispatch: loop {
        match pc {
            0x82252D48 => {
    //   block [0x82252D48..0x82252DB8)
	// 82252D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252D4C: 4BE3BD51  bl 0x8208ea9c
	ctx.lr = 0x82252D50;
	sub_8208EA60(ctx, base);
	// 82252D50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252D54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252D58: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82252D5C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82252D60: 4BFFF6B9  bl 0x82252418
	ctx.lr = 0x82252D64;
	sub_82252418(ctx, base);
	// 82252D64: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252D68: 93BF00A8  stw r29, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82252D6C: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82252D70: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82252D74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82252D78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82252D7C: 39085C48  addi r8, r8, 0x5c48
	ctx.r[8].s64 = ctx.r[8].s64 + 23624;
	// 82252D80: 997F00B0  stb r11, 0xb0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u8 ) };
	// 82252D84: 814A51B4  lwz r10, 0x51b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20916 as u32) ) } as u64;
	// 82252D88: 38E0001E  li r7, 0x1e
	ctx.r[7].s64 = 30;
	// 82252D8C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82252D90: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82252D94: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82252D98: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82252D9C: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82252DA0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82252DA4: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82252DA8: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82252DAC: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82252DB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252DB4: 4BE3BD38  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252DB8 size=192
    let mut pc: u32 = 0x82252DB8;
    'dispatch: loop {
        match pc {
            0x82252DB8 => {
    //   block [0x82252DB8..0x82252E78)
	// 82252DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252DBC: 4BE3BCDD  bl 0x8208ea98
	ctx.lr = 0x82252DC0;
	sub_8208EA60(ctx, base);
	// 82252DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252DC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82252DC8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82252DCC: 4BFFF64D  bl 0x82252418
	ctx.lr = 0x82252DD0;
	sub_82252418(ctx, base);
	// 82252DD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252DD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82252DD8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82252DDC: 396B5CB0  addi r11, r11, 0x5cb0
	ctx.r[11].s64 = ctx.r[11].s64 + 23728;
	// 82252DE0: 93FD00AC  stw r31, 0xac(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82252DE4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82252DE8: 93DD0014  stw r30, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82252DEC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252DF0: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 82252DF4: 93DD0010  stw r30, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82252DF8: 915D00B0  stw r10, 0xb0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 82252DFC: 93FD0050  stw r31, 0x50(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82252E00: 93FD0054  stw r31, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82252E04: 93FD006C  stw r31, 0x6c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82252E08: 93FD0068  stw r31, 0x68(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82252E0C: 807C05AC  lwz r3, 0x5ac(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82252E10: 4BFC5049  bl 0x82217e58
	ctx.lr = 0x82252E14;
	sub_82217E58(ctx, base);
	// 82252E14: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82252E18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82252E1C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82252E20: 4BE3BD61  bl 0x8208eb80
	ctx.lr = 0x82252E24;
	sub_8208EB80(ctx, base);
	// 82252E24: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82252E28: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82252E2C: B3DC000E  sth r30, 0xe(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 82252E30: B17C0000  sth r11, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82252E34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82252E38: B17C0002  sth r11, 2(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82252E3C: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82252E40: B15C0006  sth r10, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82252E44: B15C0008  sth r10, 8(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 82252E48: B17C000A  sth r11, 0xa(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82252E4C: B17C000C  sth r11, 0xc(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82252E50: B3FC0010  sth r31, 0x10(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 82252E54: B3DC0012  sth r30, 0x12(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(18 as u32), ctx.r[30].u16 ) };
	// 82252E58: B3FC0016  sth r31, 0x16(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(22 as u32), ctx.r[31].u16 ) };
	// 82252E5C: B3FC0018  sth r31, 0x18(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[31].u16 ) };
	// 82252E60: B3FC001A  sth r31, 0x1a(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(26 as u32), ctx.r[31].u16 ) };
	// 82252E64: B3FC001C  sth r31, 0x1c(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[31].u16 ) };
	// 82252E68: B3FC0014  sth r31, 0x14(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[31].u16 ) };
	// 82252E6C: 939D00B8  stw r28, 0xb8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(184 as u32), ctx.r[28].u32 ) };
	// 82252E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82252E74: 4BE3BC74  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252E78 size=76
    let mut pc: u32 = 0x82252E78;
    'dispatch: loop {
        match pc {
            0x82252E78 => {
    //   block [0x82252E78..0x82252EC4)
	// 82252E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252E8C: 4BFFF87D  bl 0x82252708
	ctx.lr = 0x82252E90;
	sub_82252708(ctx, base);
	// 82252E90: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252E94: 39400015  li r10, 0x15
	ctx.r[10].s64 = 21;
	// 82252E98: 396B5D18  addi r11, r11, 0x5d18
	ctx.r[11].s64 = ctx.r[11].s64 + 23832;
	// 82252E9C: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82252EA0: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82252EA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82252EAC: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82252EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82252EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252EC8 size=12
    let mut pc: u32 = 0x82252EC8;
    'dispatch: loop {
        match pc {
            0x82252EC8 => {
    //   block [0x82252EC8..0x82252ED4)
	// 82252EC8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252ECC: 386B5D80  addi r3, r11, 0x5d80
	ctx.r[3].s64 = ctx.r[11].s64 + 23936;
	// 82252ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252ED8 size=136
    let mut pc: u32 = 0x82252ED8;
    'dispatch: loop {
        match pc {
            0x82252ED8 => {
    //   block [0x82252ED8..0x82252F60)
	// 82252ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82252EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252EE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252EEC: 38800086  li r4, 0x86
	ctx.r[4].s64 = 134;
	// 82252EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252EF4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82252EF8: 4BFFF521  bl 0x82252418
	ctx.lr = 0x82252EFC;
	sub_82252418(ctx, base);
	// 82252EFC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252F00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252F04: 396B5D98  addi r11, r11, 0x5d98
	ctx.r[11].s64 = ctx.r[11].s64 + 23960;
	// 82252F08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82252F0C: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82252F10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82252F14: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82252F18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82252F1C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82252F20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82252F24: 816A51BC  lwz r11, 0x51bc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20924 as u32) ) } as u64;
	// 82252F28: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82252F2C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252F30: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82252F34: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82252F38: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82252F3C: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82252F40: 4BFDEAE9  bl 0x82231a28
	ctx.lr = 0x82252F44;
	sub_82231A28(ctx, base);
	// 82252F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82252F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252F54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82252F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252F60 size=12
    let mut pc: u32 = 0x82252F60;
    'dispatch: loop {
        match pc {
            0x82252F60 => {
    //   block [0x82252F60..0x82252F6C)
	// 82252F60: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252F64: 386B5E00  addi r3, r11, 0x5e00
	ctx.r[3].s64 = ctx.r[11].s64 + 24064;
	// 82252F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252F70 size=116
    let mut pc: u32 = 0x82252F70;
    'dispatch: loop {
        match pc {
            0x82252F70 => {
    //   block [0x82252F70..0x82252FE4)
	// 82252F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82252F78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82252F7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82252F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82252F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82252F88: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82252F8C: 4BFFF48D  bl 0x82252418
	ctx.lr = 0x82252F90;
	sub_82252418(ctx, base);
	// 82252F90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82252F94: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82252F98: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82252F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82252FA0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82252FA4: 394A5E08  addi r10, r10, 0x5e08
	ctx.r[10].s64 = ctx.r[10].s64 + 24072;
	// 82252FA8: 913F00A8  stw r9, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[9].u32 ) };
	// 82252FAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82252FB0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82252FB4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82252FB8: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82252FBC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82252FC0: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82252FC4: 4BFDEA65  bl 0x82231a28
	ctx.lr = 0x82252FC8;
	sub_82231A28(ctx, base);
	// 82252FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82252FCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82252FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82252FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82252FD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82252FDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82252FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82252FE8 size=12
    let mut pc: u32 = 0x82252FE8;
    'dispatch: loop {
        match pc {
            0x82252FE8 => {
    //   block [0x82252FE8..0x82252FF4)
	// 82252FE8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82252FEC: 386B5E70  addi r3, r11, 0x5e70
	ctx.r[3].s64 = ctx.r[11].s64 + 24176;
	// 82252FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82252FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82252FF8 size=120
    let mut pc: u32 = 0x82252FF8;
    'dispatch: loop {
        match pc {
            0x82252FF8 => {
    //   block [0x82252FF8..0x82253070)
	// 82252FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82252FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82253004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225300C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253010: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82253014: 4BFFF405  bl 0x82252418
	ctx.lr = 0x82253018;
	sub_82252418(ctx, base);
	// 82253018: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225301C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82253020: 396B5E78  addi r11, r11, 0x5e78
	ctx.r[11].s64 = ctx.r[11].s64 + 24184;
	// 82253024: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82253028: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8225302C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82253030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82253034: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82253038: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8225303C: 915F00A8  stw r10, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 82253040: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82253044: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82253048: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8225304C: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82253050: 4BFDE9D9  bl 0x82231a28
	ctx.lr = 0x82253054;
	sub_82231A28(ctx, base);
	// 82253054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82253058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225305C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253064: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82253068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82253070 size=8
    let mut pc: u32 = 0x82253070;
    'dispatch: loop {
        match pc {
            0x82253070 => {
    //   block [0x82253070..0x82253078)
	// 82253070: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82253074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82253078 size=12
    let mut pc: u32 = 0x82253078;
    'dispatch: loop {
        match pc {
            0x82253078 => {
    //   block [0x82253078..0x82253084)
	// 82253078: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225307C: 386B5EE0  addi r3, r11, 0x5ee0
	ctx.r[3].s64 = ctx.r[11].s64 + 24288;
	// 82253080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253088 size=116
    let mut pc: u32 = 0x82253088;
    'dispatch: loop {
        match pc {
            0x82253088 => {
    //   block [0x82253088..0x822530FC)
	// 82253088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82253094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225309C: 38800088  li r4, 0x88
	ctx.r[4].s64 = 136;
	// 822530A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822530A4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822530A8: 4BFFF371  bl 0x82252418
	ctx.lr = 0x822530AC;
	sub_82252418(ctx, base);
	// 822530AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822530B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822530B4: 396B5EF0  addi r11, r11, 0x5ef0
	ctx.r[11].s64 = ctx.r[11].s64 + 24304;
	// 822530B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822530BC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822530C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822530C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822530C8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 822530CC: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 822530D0: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 822530D4: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 822530D8: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 822530DC: 4BFDE94D  bl 0x82231a28
	ctx.lr = 0x822530E0;
	sub_82231A28(ctx, base);
	// 822530E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822530E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822530E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822530EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822530F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822530F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822530F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82253100 size=12
    let mut pc: u32 = 0x82253100;
    'dispatch: loop {
        match pc {
            0x82253100 => {
    //   block [0x82253100..0x8225310C)
	// 82253100: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253104: 386B5F58  addi r3, r11, 0x5f58
	ctx.r[3].s64 = ctx.r[11].s64 + 24408;
	// 82253108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253110 size=116
    let mut pc: u32 = 0x82253110;
    'dispatch: loop {
        match pc {
            0x82253110 => {
    //   block [0x82253110..0x82253184)
	// 82253110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225311C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253124: 38800087  li r4, 0x87
	ctx.r[4].s64 = 135;
	// 82253128: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8225312C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82253130: 4BFFF2E9  bl 0x82252418
	ctx.lr = 0x82253134;
	sub_82252418(ctx, base);
	// 82253134: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8225313C: 396B5F68  addi r11, r11, 0x5f68
	ctx.r[11].s64 = ctx.r[11].s64 + 24424;
	// 82253140: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82253144: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82253148: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8225314C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82253150: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82253154: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82253158: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 8225315C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82253160: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82253164: 4BFDE8C5  bl 0x82231a28
	ctx.lr = 0x82253168;
	sub_82231A28(ctx, base);
	// 82253168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8225316C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253178: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8225317C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82253180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82253188 size=12
    let mut pc: u32 = 0x82253188;
    'dispatch: loop {
        match pc {
            0x82253188 => {
    //   block [0x82253188..0x82253194)
	// 82253188: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225318C: 386B5FD0  addi r3, r11, 0x5fd0
	ctx.r[3].s64 = ctx.r[11].s64 + 24528;
	// 82253190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253198 size=84
    let mut pc: u32 = 0x82253198;
    'dispatch: loop {
        match pc {
            0x82253198 => {
    //   block [0x82253198..0x822531EC)
	// 82253198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225319C: 4BE3B901  bl 0x8208ea9c
	ctx.lr = 0x822531A0;
	sub_8208EA60(ctx, base);
	// 822531A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822531A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822531A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822531AC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 822531B0: 3880008B  li r4, 0x8b
	ctx.r[4].s64 = 139;
	// 822531B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822531B8: 4BFFF261  bl 0x82252418
	ctx.lr = 0x822531BC;
	sub_82252418(ctx, base);
	// 822531BC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822531C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822531C4: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 822531C8: 396B5FE0  addi r11, r11, 0x5fe0
	ctx.r[11].s64 = ctx.r[11].s64 + 24544;
	// 822531CC: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 822531D0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822531D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822531D8: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 822531DC: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 822531E0: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 822531E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822531E8: 4BE3B904  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822531F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822531F0 size=84
    let mut pc: u32 = 0x822531F0;
    'dispatch: loop {
        match pc {
            0x822531F0 => {
    //   block [0x822531F0..0x82253244)
	// 822531F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822531F4: 4BE3B8A9  bl 0x8208ea9c
	ctx.lr = 0x822531F8;
	sub_8208EA60(ctx, base);
	// 822531F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822531FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82253200: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82253204: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82253208: 3880008A  li r4, 0x8a
	ctx.r[4].s64 = 138;
	// 8225320C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253210: 4BFFF209  bl 0x82252418
	ctx.lr = 0x82253214;
	sub_82252418(ctx, base);
	// 82253214: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8225321C: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82253220: 396B6048  addi r11, r11, 0x6048
	ctx.r[11].s64 = ctx.r[11].s64 + 24648;
	// 82253224: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 82253228: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8225322C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82253230: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82253234: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82253238: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8225323C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253240: 4BE3B8AC  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253248 size=84
    let mut pc: u32 = 0x82253248;
    'dispatch: loop {
        match pc {
            0x82253248 => {
    //   block [0x82253248..0x8225329C)
	// 82253248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225324C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253258: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8225325C: 3880008D  li r4, 0x8d
	ctx.r[4].s64 = 141;
	// 82253260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253264: 4BFFF1B5  bl 0x82252418
	ctx.lr = 0x82253268;
	sub_82252418(ctx, base);
	// 82253268: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225326C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82253270: 396B60B0  addi r11, r11, 0x60b0
	ctx.r[11].s64 = ctx.r[11].s64 + 24752;
	// 82253274: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82253278: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8225327C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82253280: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82253284: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82253288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8225328C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82253298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822532A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822532A0 size=84
    let mut pc: u32 = 0x822532A0;
    'dispatch: loop {
        match pc {
            0x822532A0 => {
    //   block [0x822532A0..0x822532F4)
	// 822532A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822532A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822532A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822532AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822532B0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822532B4: 3880008C  li r4, 0x8c
	ctx.r[4].s64 = 140;
	// 822532B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822532BC: 4BFFF15D  bl 0x82252418
	ctx.lr = 0x822532C0;
	sub_82252418(ctx, base);
	// 822532C0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822532C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822532C8: 396B6118  addi r11, r11, 0x6118
	ctx.r[11].s64 = ctx.r[11].s64 + 24856;
	// 822532CC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822532D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822532D4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 822532D8: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 822532DC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 822532E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822532E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822532E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822532EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822532F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822532F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822532F8 size=104
    let mut pc: u32 = 0x822532F8;
    'dispatch: loop {
        match pc {
            0x822532F8 => {
    //   block [0x822532F8..0x82253360)
	// 822532F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822532FC: 4BE3B7A1  bl 0x8208ea9c
	ctx.lr = 0x82253300;
	sub_8208EA60(ctx, base);
	// 82253300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253304: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253308: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8225330C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253310: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253314: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253318: 4BFC4B41  bl 0x82217e58
	ctx.lr = 0x8225331C;
	sub_82217E58(ctx, base);
	// 8225331C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253320: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82253324: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253328: 4182002C  beq 0x82253354
	if ctx.cr[0].eq {
	pc = 0x82253354; continue 'dispatch;
	}
	// 8225332C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253330: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253334: 4BFFF0E5  bl 0x82252418
	ctx.lr = 0x82253338;
	sub_82252418(ctx, base);
	// 82253338: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8225333C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82253340: 394A5360  addi r10, r10, 0x5360
	ctx.r[10].s64 = ctx.r[10].s64 + 21344;
	// 82253344: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82253348: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8225334C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82253350: 48000008  b 0x82253358
	pc = 0x82253358; continue 'dispatch;
	// 82253354: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225335C: 4BE3B790  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253360 size=80
    let mut pc: u32 = 0x82253360;
    'dispatch: loop {
        match pc {
            0x82253360 => {
    //   block [0x82253360..0x822533B0)
	// 82253360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253364: 4BE3B739  bl 0x8208ea9c
	ctx.lr = 0x82253368;
	sub_8208EA60(ctx, base);
	// 82253368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225336C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253370: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253374: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253378: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225337C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253380: 4BFC4AD9  bl 0x82217e58
	ctx.lr = 0x82253384;
	sub_82217E58(ctx, base);
	// 82253384: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253388: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225338C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253390: 41820014  beq 0x822533a4
	if ctx.cr[0].eq {
	pc = 0x822533A4; continue 'dispatch;
	}
	// 82253394: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253398: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225339C: 4BFFF305  bl 0x822526a0
	ctx.lr = 0x822533A0;
	sub_822526A0(ctx, base);
	// 822533A0: 48000008  b 0x822533a8
	pc = 0x822533A8; continue 'dispatch;
	// 822533A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822533A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822533AC: 4BE3B740  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822533B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822533B0 size=108
    let mut pc: u32 = 0x822533B0;
    'dispatch: loop {
        match pc {
            0x822533B0 => {
    //   block [0x822533B0..0x8225341C)
	// 822533B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822533B4: 4BE3B6E9  bl 0x8208ea9c
	ctx.lr = 0x822533B8;
	sub_8208EA60(ctx, base);
	// 822533B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822533BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822533C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822533C4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822533C8: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822533CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822533D0: 4BFC4A89  bl 0x82217e58
	ctx.lr = 0x822533D4;
	sub_82217E58(ctx, base);
	// 822533D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822533D8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822533DC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822533E0: 41820030  beq 0x82253410
	if ctx.cr[0].eq {
	pc = 0x82253410; continue 'dispatch;
	}
	// 822533E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822533E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822533EC: 4BFFF02D  bl 0x82252418
	ctx.lr = 0x822533F0;
	sub_82252418(ctx, base);
	// 822533F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822533F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822533F8: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822533FC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82253400: 396B1FE0  addi r11, r11, 0x1fe0
	ctx.r[11].s64 = ctx.r[11].s64 + 8160;
	// 82253404: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82253408: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8225340C: 48000008  b 0x82253414
	pc = 0x82253414; continue 'dispatch;
	// 82253410: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253414: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253418: 4BE3B6D4  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253420 size=108
    let mut pc: u32 = 0x82253420;
    'dispatch: loop {
        match pc {
            0x82253420 => {
    //   block [0x82253420..0x8225348C)
	// 82253420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253424: 4BE3B679  bl 0x8208ea9c
	ctx.lr = 0x82253428;
	sub_8208EA60(ctx, base);
	// 82253428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225342C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253434: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253438: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225343C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253440: 4BFC4A19  bl 0x82217e58
	ctx.lr = 0x82253444;
	sub_82217E58(ctx, base);
	// 82253444: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253448: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225344C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253450: 41820030  beq 0x82253480
	if ctx.cr[0].eq {
	pc = 0x82253480; continue 'dispatch;
	}
	// 82253454: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253458: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225345C: 4BFFEFBD  bl 0x82252418
	ctx.lr = 0x82253460;
	sub_82252418(ctx, base);
	// 82253460: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82253464: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253468: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8225346C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82253470: 396B6180  addi r11, r11, 0x6180
	ctx.r[11].s64 = ctx.r[11].s64 + 24960;
	// 82253474: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82253478: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8225347C: 48000008  b 0x82253484
	pc = 0x82253484; continue 'dispatch;
	// 82253480: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253488: 4BE3B664  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253490 size=80
    let mut pc: u32 = 0x82253490;
    'dispatch: loop {
        match pc {
            0x82253490 => {
    //   block [0x82253490..0x822534E0)
	// 82253490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253494: 4BE3B609  bl 0x8208ea9c
	ctx.lr = 0x82253498;
	sub_8208EA60(ctx, base);
	// 82253498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225349C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822534A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822534A4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822534A8: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822534AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822534B0: 4BFC49A9  bl 0x82217e58
	ctx.lr = 0x822534B4;
	sub_82217E58(ctx, base);
	// 822534B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822534B8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822534BC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822534C0: 41820014  beq 0x822534d4
	if ctx.cr[0].eq {
	pc = 0x822534D4; continue 'dispatch;
	}
	// 822534C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822534C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822534CC: 4BFFFAA5  bl 0x82252f70
	ctx.lr = 0x822534D0;
	sub_82252F70(ctx, base);
	// 822534D0: 48000008  b 0x822534d8
	pc = 0x822534D8; continue 'dispatch;
	// 822534D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822534D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822534DC: 4BE3B610  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822534E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822534E0 size=80
    let mut pc: u32 = 0x822534E0;
    'dispatch: loop {
        match pc {
            0x822534E0 => {
    //   block [0x822534E0..0x82253530)
	// 822534E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822534E4: 4BE3B5B9  bl 0x8208ea9c
	ctx.lr = 0x822534E8;
	sub_8208EA60(ctx, base);
	// 822534E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822534EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822534F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822534F4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822534F8: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822534FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253500: 4BFC4959  bl 0x82217e58
	ctx.lr = 0x82253504;
	sub_82217E58(ctx, base);
	// 82253504: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253508: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225350C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253510: 41820014  beq 0x82253524
	if ctx.cr[0].eq {
	pc = 0x82253524; continue 'dispatch;
	}
	// 82253514: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253518: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225351C: 4BFFFADD  bl 0x82252ff8
	ctx.lr = 0x82253520;
	sub_82252FF8(ctx, base);
	// 82253520: 48000008  b 0x82253528
	pc = 0x82253528; continue 'dispatch;
	// 82253524: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253528: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225352C: 4BE3B5C0  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253530 size=80
    let mut pc: u32 = 0x82253530;
    'dispatch: loop {
        match pc {
            0x82253530 => {
    //   block [0x82253530..0x82253580)
	// 82253530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253534: 4BE3B569  bl 0x8208ea9c
	ctx.lr = 0x82253538;
	sub_8208EA60(ctx, base);
	// 82253538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225353C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253540: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253544: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253548: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225354C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253550: 4BFC4909  bl 0x82217e58
	ctx.lr = 0x82253554;
	sub_82217E58(ctx, base);
	// 82253554: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253558: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225355C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253560: 41820014  beq 0x82253574
	if ctx.cr[0].eq {
	pc = 0x82253574; continue 'dispatch;
	}
	// 82253564: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253568: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225356C: 4BFFF96D  bl 0x82252ed8
	ctx.lr = 0x82253570;
	sub_82252ED8(ctx, base);
	// 82253570: 48000008  b 0x82253578
	pc = 0x82253578; continue 'dispatch;
	// 82253574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253578: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225357C: 4BE3B570  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253580 size=80
    let mut pc: u32 = 0x82253580;
    'dispatch: loop {
        match pc {
            0x82253580 => {
    //   block [0x82253580..0x822535D0)
	// 82253580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253584: 4BE3B519  bl 0x8208ea9c
	ctx.lr = 0x82253588;
	sub_8208EA60(ctx, base);
	// 82253588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225358C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253590: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253594: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253598: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225359C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822535A0: 4BFC48B9  bl 0x82217e58
	ctx.lr = 0x822535A4;
	sub_82217E58(ctx, base);
	// 822535A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822535A8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822535AC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822535B0: 41820014  beq 0x822535c4
	if ctx.cr[0].eq {
	pc = 0x822535C4; continue 'dispatch;
	}
	// 822535B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822535B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822535BC: 4BFFFACD  bl 0x82253088
	ctx.lr = 0x822535C0;
	sub_82253088(ctx, base);
	// 822535C0: 48000008  b 0x822535c8
	pc = 0x822535C8; continue 'dispatch;
	// 822535C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822535C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822535CC: 4BE3B520  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822535D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822535D0 size=80
    let mut pc: u32 = 0x822535D0;
    'dispatch: loop {
        match pc {
            0x822535D0 => {
    //   block [0x822535D0..0x82253620)
	// 822535D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822535D4: 4BE3B4C9  bl 0x8208ea9c
	ctx.lr = 0x822535D8;
	sub_8208EA60(ctx, base);
	// 822535D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822535DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822535E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822535E4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822535E8: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822535EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822535F0: 4BFC4869  bl 0x82217e58
	ctx.lr = 0x822535F4;
	sub_82217E58(ctx, base);
	// 822535F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822535F8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822535FC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253600: 41820014  beq 0x82253614
	if ctx.cr[0].eq {
	pc = 0x82253614; continue 'dispatch;
	}
	// 82253604: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253608: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225360C: 4BFFFB05  bl 0x82253110
	ctx.lr = 0x82253610;
	sub_82253110(ctx, base);
	// 82253610: 48000008  b 0x82253618
	pc = 0x82253618; continue 'dispatch;
	// 82253614: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253618: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225361C: 4BE3B4D0  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253620 size=80
    let mut pc: u32 = 0x82253620;
    'dispatch: loop {
        match pc {
            0x82253620 => {
    //   block [0x82253620..0x82253670)
	// 82253620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253624: 4BE3B479  bl 0x8208ea9c
	ctx.lr = 0x82253628;
	sub_8208EA60(ctx, base);
	// 82253628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225362C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253630: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253634: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253638: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225363C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253640: 4BFC4819  bl 0x82217e58
	ctx.lr = 0x82253644;
	sub_82217E58(ctx, base);
	// 82253644: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253648: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225364C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253650: 41820014  beq 0x82253664
	if ctx.cr[0].eq {
	pc = 0x82253664; continue 'dispatch;
	}
	// 82253654: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225365C: 4BFFF75D  bl 0x82252db8
	ctx.lr = 0x82253660;
	sub_82252DB8(ctx, base);
	// 82253660: 48000008  b 0x82253668
	pc = 0x82253668; continue 'dispatch;
	// 82253664: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253668: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225366C: 4BE3B480  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253670 size=124
    let mut pc: u32 = 0x82253670;
    'dispatch: loop {
        match pc {
            0x82253670 => {
    //   block [0x82253670..0x822536EC)
	// 82253670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253678: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225367C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253680: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253688: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225368C: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253690: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82253694: 4BFC47C5  bl 0x82217e58
	ctx.lr = 0x82253698;
	sub_82217E58(ctx, base);
	// 82253698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8225369C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822536A0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822536A4: 4182002C  beq 0x822536d0
	if ctx.cr[0].eq {
	pc = 0x822536D0; continue 'dispatch;
	}
	// 822536A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822536AC: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 822536B0: 4BFFED69  bl 0x82252418
	ctx.lr = 0x822536B4;
	sub_82252418(ctx, base);
	// 822536B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822536B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822536BC: 394A5830  addi r10, r10, 0x5830
	ctx.r[10].s64 = ctx.r[10].s64 + 22576;
	// 822536C0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822536C4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822536C8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822536CC: 48000008  b 0x822536d4
	pc = 0x822536D4; continue 'dispatch;
	// 822536D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822536D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822536D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822536DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822536E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822536E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822536E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822536F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822536F0 size=80
    let mut pc: u32 = 0x822536F0;
    'dispatch: loop {
        match pc {
            0x822536F0 => {
    //   block [0x822536F0..0x82253740)
	// 822536F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822536F4: 4BE3B3A9  bl 0x8208ea9c
	ctx.lr = 0x822536F8;
	sub_8208EA60(ctx, base);
	// 822536F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822536FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253700: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253704: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253708: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225370C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253710: 4BFC4749  bl 0x82217e58
	ctx.lr = 0x82253714;
	sub_82217E58(ctx, base);
	// 82253714: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253718: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225371C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253720: 41820014  beq 0x82253734
	if ctx.cr[0].eq {
	pc = 0x82253734; continue 'dispatch;
	}
	// 82253724: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253728: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8225372C: 4BFFF45D  bl 0x82252b88
	ctx.lr = 0x82253730;
	sub_82252B88(ctx, base);
	// 82253730: 48000008  b 0x82253738
	pc = 0x82253738; continue 'dispatch;
	// 82253734: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225373C: 4BE3B3B0  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253740 size=124
    let mut pc: u32 = 0x82253740;
    'dispatch: loop {
        match pc {
            0x82253740 => {
    //   block [0x82253740..0x822537BC)
	// 82253740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225374C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253754: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253758: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225375C: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82253764: 4BFC46F5  bl 0x82217e58
	ctx.lr = 0x82253768;
	sub_82217E58(ctx, base);
	// 82253768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8225376C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82253770: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82253774: 4182002C  beq 0x822537a0
	if ctx.cr[0].eq {
	pc = 0x822537A0; continue 'dispatch;
	}
	// 82253778: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8225377C: 3880007E  li r4, 0x7e
	ctx.r[4].s64 = 126;
	// 82253780: 4BFFEC99  bl 0x82252418
	ctx.lr = 0x82253784;
	sub_82252418(ctx, base);
	// 82253784: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82253788: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8225378C: 394A5988  addi r10, r10, 0x5988
	ctx.r[10].s64 = ctx.r[10].s64 + 22920;
	// 82253790: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82253794: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82253798: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8225379C: 48000008  b 0x822537a4
	pc = 0x822537A4; continue 'dispatch;
	// 822537A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822537A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822537A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822537AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822537B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822537B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822537B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822537C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822537C0 size=112
    let mut pc: u32 = 0x822537C0;
    'dispatch: loop {
        match pc {
            0x822537C0 => {
    //   block [0x822537C0..0x82253830)
	// 822537C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822537C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822537C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822537CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822537D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822537D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822537D8: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822537DC: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822537E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822537E4: 4BFC4675  bl 0x82217e58
	ctx.lr = 0x822537E8;
	sub_82217E58(ctx, base);
	// 822537E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822537EC: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822537F0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822537F4: 41820020  beq 0x82253814
	if ctx.cr[0].eq {
	pc = 0x82253814; continue 'dispatch;
	}
	// 822537F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822537FC: 3880008E  li r4, 0x8e
	ctx.r[4].s64 = 142;
	// 82253800: 4BFFF461  bl 0x82252c60
	ctx.lr = 0x82253804;
	sub_82252C60(ctx, base);
	// 82253804: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253808: 396B5AE0  addi r11, r11, 0x5ae0
	ctx.r[11].s64 = ctx.r[11].s64 + 23264;
	// 8225380C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82253810: 48000008  b 0x82253818
	pc = 0x82253818; continue 'dispatch;
	// 82253814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253818: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225381C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253824: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82253828: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225382C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253830 size=112
    let mut pc: u32 = 0x82253830;
    'dispatch: loop {
        match pc {
            0x82253830 => {
    //   block [0x82253830..0x822538A0)
	// 82253830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225383C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253844: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253848: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225384C: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253850: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82253854: 4BFC4605  bl 0x82217e58
	ctx.lr = 0x82253858;
	sub_82217E58(ctx, base);
	// 82253858: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8225385C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82253860: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82253864: 41820020  beq 0x82253884
	if ctx.cr[0].eq {
	pc = 0x82253884; continue 'dispatch;
	}
	// 82253868: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8225386C: 3880008F  li r4, 0x8f
	ctx.r[4].s64 = 143;
	// 82253870: 4BFFF3F1  bl 0x82252c60
	ctx.lr = 0x82253874;
	sub_82252C60(ctx, base);
	// 82253874: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253878: 396B5B58  addi r11, r11, 0x5b58
	ctx.r[11].s64 = ctx.r[11].s64 + 23384;
	// 8225387C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82253880: 48000008  b 0x82253888
	pc = 0x82253888; continue 'dispatch;
	// 82253884: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225388C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253894: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82253898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225389C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822538A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822538A0 size=112
    let mut pc: u32 = 0x822538A0;
    'dispatch: loop {
        match pc {
            0x822538A0 => {
    //   block [0x822538A0..0x82253910)
	// 822538A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822538A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822538A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822538AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822538B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822538B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822538B8: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822538BC: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822538C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822538C4: 4BFC4595  bl 0x82217e58
	ctx.lr = 0x822538C8;
	sub_82217E58(ctx, base);
	// 822538C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822538CC: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822538D0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822538D4: 41820020  beq 0x822538f4
	if ctx.cr[0].eq {
	pc = 0x822538F4; continue 'dispatch;
	}
	// 822538D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822538DC: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 822538E0: 4BFFF381  bl 0x82252c60
	ctx.lr = 0x822538E4;
	sub_82252C60(ctx, base);
	// 822538E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822538E8: 396B5BD0  addi r11, r11, 0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 23504;
	// 822538EC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822538F0: 48000008  b 0x822538f8
	pc = 0x822538F8; continue 'dispatch;
	// 822538F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822538F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822538FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253904: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82253908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225390C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253910 size=112
    let mut pc: u32 = 0x82253910;
    'dispatch: loop {
        match pc {
            0x82253910 => {
    //   block [0x82253910..0x82253980)
	// 82253910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225391C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253924: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253928: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8225392C: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253930: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82253934: 4BFC4525  bl 0x82217e58
	ctx.lr = 0x82253938;
	sub_82217E58(ctx, base);
	// 82253938: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8225393C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82253940: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82253944: 41820020  beq 0x82253964
	if ctx.cr[0].eq {
	pc = 0x82253964; continue 'dispatch;
	}
	// 82253948: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8225394C: 38800091  li r4, 0x91
	ctx.r[4].s64 = 145;
	// 82253950: 4BFFF311  bl 0x82252c60
	ctx.lr = 0x82253954;
	sub_82252C60(ctx, base);
	// 82253954: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253958: 396B5A68  addi r11, r11, 0x5a68
	ctx.r[11].s64 = ctx.r[11].s64 + 23144;
	// 8225395C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82253960: 48000008  b 0x82253968
	pc = 0x82253968; continue 'dispatch;
	// 82253964: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253968: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225396C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253974: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82253978: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225397C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253980 size=80
    let mut pc: u32 = 0x82253980;
    'dispatch: loop {
        match pc {
            0x82253980 => {
    //   block [0x82253980..0x822539D0)
	// 82253980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253984: 4BE3B119  bl 0x8208ea9c
	ctx.lr = 0x82253988;
	sub_8208EA60(ctx, base);
	// 82253988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225398C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253990: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253994: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253998: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225399C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822539A0: 4BFC44B9  bl 0x82217e58
	ctx.lr = 0x822539A4;
	sub_82217E58(ctx, base);
	// 822539A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822539A8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822539AC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822539B0: 41820014  beq 0x822539c4
	if ctx.cr[0].eq {
	pc = 0x822539C4; continue 'dispatch;
	}
	// 822539B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822539B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822539BC: 4BFFEC35  bl 0x822525f0
	ctx.lr = 0x822539C0;
	sub_822525F0(ctx, base);
	// 822539C0: 48000008  b 0x822539c8
	pc = 0x822539C8; continue 'dispatch;
	// 822539C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822539C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822539CC: 4BE3B120  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822539D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822539D0 size=108
    let mut pc: u32 = 0x822539D0;
    'dispatch: loop {
        match pc {
            0x822539D0 => {
    //   block [0x822539D0..0x82253A3C)
	// 822539D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822539D4: 4BE3B0C9  bl 0x8208ea9c
	ctx.lr = 0x822539D8;
	sub_8208EA60(ctx, base);
	// 822539D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822539DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822539E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822539E4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822539E8: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822539EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822539F0: 4BFC4469  bl 0x82217e58
	ctx.lr = 0x822539F4;
	sub_82217E58(ctx, base);
	// 822539F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822539F8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822539FC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253A00: 41820030  beq 0x82253a30
	if ctx.cr[0].eq {
	pc = 0x82253A30; continue 'dispatch;
	}
	// 82253A04: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253A08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253A0C: 4BFFEA0D  bl 0x82252418
	ctx.lr = 0x82253A10;
	sub_82252418(ctx, base);
	// 82253A10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82253A14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82253A18: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82253A1C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82253A20: 396B53C8  addi r11, r11, 0x53c8
	ctx.r[11].s64 = ctx.r[11].s64 + 21448;
	// 82253A24: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82253A28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82253A2C: 48000008  b 0x82253a34
	pc = 0x82253A34; continue 'dispatch;
	// 82253A30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253A34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253A38: 4BE3B0B4  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253A40 size=96
    let mut pc: u32 = 0x82253A40;
    'dispatch: loop {
        match pc {
            0x82253A40 => {
    //   block [0x82253A40..0x82253AA0)
	// 82253A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253A44: 4BE3B051  bl 0x8208ea94
	ctx.lr = 0x82253A48;
	sub_8208EA60(ctx, base);
	// 82253A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253A4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253A50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253A54: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253A58: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82253A5C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82253A60: 837F05AC  lwz r27, 0x5ac(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253A64: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82253A68: 4BFC43F1  bl 0x82217e58
	ctx.lr = 0x82253A6C;
	sub_82217E58(ctx, base);
	// 82253A6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253A70: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82253A74: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82253A78: 4182001C  beq 0x82253a94
	if ctx.cr[0].eq {
	pc = 0x82253A94; continue 'dispatch;
	}
	// 82253A7C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82253A80: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82253A84: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253A88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253A8C: 4BFFF2BD  bl 0x82252d48
	ctx.lr = 0x82253A90;
	sub_82252D48(ctx, base);
	// 82253A90: 48000008  b 0x82253a98
	pc = 0x82253A98; continue 'dispatch;
	// 82253A94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82253A9C: 4BE3B048  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253AA0 size=88
    let mut pc: u32 = 0x82253AA0;
    'dispatch: loop {
        match pc {
            0x82253AA0 => {
    //   block [0x82253AA0..0x82253AF8)
	// 82253AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253AA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253AAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253AB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253AB4: 4BFFE965  bl 0x82252418
	ctx.lr = 0x82253AB8;
	sub_82252418(ctx, base);
	// 82253AB8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82253ABC: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82253AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82253AC4: 394A6180  addi r10, r10, 0x6180
	ctx.r[10].s64 = ctx.r[10].s64 + 24960;
	// 82253AC8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82253ACC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82253AD0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82253AD4: 814951BC  lwz r10, 0x51bc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20924 as u32) ) } as u64;
	// 82253AD8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82253ADC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82253AE0: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82253AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82253AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253AF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82253AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253AF8 size=80
    let mut pc: u32 = 0x82253AF8;
    'dispatch: loop {
        match pc {
            0x82253AF8 => {
    //   block [0x82253AF8..0x82253B48)
	// 82253AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253AFC: 4BE3AFA1  bl 0x8208ea9c
	ctx.lr = 0x82253B00;
	sub_8208EA60(ctx, base);
	// 82253B00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253B04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82253B08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82253B0C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82253B10: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82253B14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253B18: 4BFC4341  bl 0x82217e58
	ctx.lr = 0x82253B1C;
	sub_82217E58(ctx, base);
	// 82253B1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82253B20: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82253B24: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82253B28: 41820014  beq 0x82253b3c
	if ctx.cr[0].eq {
	pc = 0x82253B3C; continue 'dispatch;
	}
	// 82253B2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82253B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253B34: 4BFFFF6D  bl 0x82253aa0
	ctx.lr = 0x82253B38;
	sub_82253AA0(ctx, base);
	// 82253B38: 48000008  b 0x82253b40
	pc = 0x82253B40; continue 'dispatch;
	// 82253B3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82253B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253B44: 4BE3AFA8  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253B48 size=240
    let mut pc: u32 = 0x82253B48;
    'dispatch: loop {
        match pc {
            0x82253B48 => {
    //   block [0x82253B48..0x82253C38)
	// 82253B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82253B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253B60: 38640080  addi r3, r4, 0x80
	ctx.r[3].s64 = ctx.r[4].s64 + 128;
	// 82253B64: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82253B68: 4BFFB389  bl 0x8224eef0
	ctx.lr = 0x82253B6C;
	sub_8224EEF0(ctx, base);
	// 82253B6C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82253B70: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82253B74: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82253B78: 555E103A  slwi r30, r10, 2
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82253B7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253B80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253B84: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82253B88: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253B8C: 4BFC42CD  bl 0x82217e58
	ctx.lr = 0x82253B90;
	sub_82217E58(ctx, base);
	// 82253B90: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82253B94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253B98: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253B9C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253BA0: 4BFC42B9  bl 0x82217e58
	ctx.lr = 0x82253BA4;
	sub_82217E58(ctx, base);
	// 82253BA4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82253BA8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253BAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253BB0: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253BB4: 4BFC42A5  bl 0x82217e58
	ctx.lr = 0x82253BB8;
	sub_82217E58(ctx, base);
	// 82253BB8: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82253BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253BC0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253BC4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253BC8: 4BFC4291  bl 0x82217e58
	ctx.lr = 0x82253BCC;
	sub_82217E58(ctx, base);
	// 82253BCC: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82253BD0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253BD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82253BD8: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253BDC: 4BFC427D  bl 0x82217e58
	ctx.lr = 0x82253BE0;
	sub_82217E58(ctx, base);
	// 82253BE0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82253BE4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82253BE8: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253BEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82253BF0: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82253BF4: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253BF8: 4BFC4261  bl 0x82217e58
	ctx.lr = 0x82253BFC;
	sub_82217E58(ctx, base);
	// 82253BFC: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82253C00: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82253C04: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253C08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82253C0C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82253C10: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253C14: 4BFC4245  bl 0x82217e58
	ctx.lr = 0x82253C18;
	sub_82217E58(ctx, base);
	// 82253C18: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82253C1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82253C20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253C2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82253C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82253C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253C38 size=276
    let mut pc: u32 = 0x82253C38;
    'dispatch: loop {
        match pc {
            0x82253C38 => {
    //   block [0x82253C38..0x82253D4C)
	// 82253C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253C3C: 4BE3AE5D  bl 0x8208ea98
	ctx.lr = 0x82253C40;
	sub_8208EA60(ctx, base);
	// 82253C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253C48: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82253C4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82253C50: 4800003C  b 0x82253c8c
	pc = 0x82253C8C; continue 'dispatch;
	// 82253C54: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82253C58: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253C5C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253C60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82253C64: 419A0038  beq cr6, 0x82253c9c
	if ctx.cr[6].eq {
	pc = 0x82253C9C; continue 'dispatch;
	}
	// 82253C68: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253C6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82253C70: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253C74: 41820010  beq 0x82253c84
	if ctx.cr[0].eq {
	pc = 0x82253C84; continue 'dispatch;
	}
	// 82253C78: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82253C7C: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82253C80: 4BFC4309  bl 0x82217f88
	ctx.lr = 0x82253C84;
	sub_82217F88(ctx, base);
	// 82253C84: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82253C88: 7FBE592E  stwx r29, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82253C8C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82253C90: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253C94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82253C98: 409AFFBC  bne cr6, 0x82253c54
	if !ctx.cr[6].eq {
	pc = 0x82253C54; continue 'dispatch;
	}
	// 82253C9C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82253CA0: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253CA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82253CA8: 419A0010  beq cr6, 0x82253cb8
	if ctx.cr[6].eq {
	pc = 0x82253CB8; continue 'dispatch;
	}
	// 82253CAC: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82253CB0: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82253CB4: 4BFC42D5  bl 0x82217f88
	ctx.lr = 0x82253CB8;
	sub_82217F88(ctx, base);
	// 82253CB8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82253CBC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82253CC0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82253CC4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82253CC8: 4099FFC4  ble cr6, 0x82253c8c
	if !ctx.cr[6].gt {
	pc = 0x82253C8C; continue 'dispatch;
	}
	// 82253CCC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253CD0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82253CD4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253CD8: 4BFC42B1  bl 0x82217f88
	ctx.lr = 0x82253CDC;
	sub_82217F88(ctx, base);
	// 82253CDC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253CE0: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82253CE4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253CE8: 4BFC42A1  bl 0x82217f88
	ctx.lr = 0x82253CEC;
	sub_82217F88(ctx, base);
	// 82253CEC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253CF0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82253CF4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253CF8: 4BFC4291  bl 0x82217f88
	ctx.lr = 0x82253CFC;
	sub_82217F88(ctx, base);
	// 82253CFC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253D00: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253D04: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253D08: 4BFC4281  bl 0x82217f88
	ctx.lr = 0x82253D0C;
	sub_82217F88(ctx, base);
	// 82253D0C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253D10: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82253D14: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253D18: 4BFC4271  bl 0x82217f88
	ctx.lr = 0x82253D1C;
	sub_82217F88(ctx, base);
	// 82253D1C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253D20: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253D24: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253D28: 4BFC4261  bl 0x82217f88
	ctx.lr = 0x82253D2C;
	sub_82217F88(ctx, base);
	// 82253D2C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253D30: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82253D34: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253D38: 4BFC4251  bl 0x82217f88
	ctx.lr = 0x82253D3C;
	sub_82217F88(ctx, base);
	// 82253D3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82253D40: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82253D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82253D48: 4BE3ADA0  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253D50 size=356
    let mut pc: u32 = 0x82253D50;
    'dispatch: loop {
        match pc {
            0x82253D50 => {
    //   block [0x82253D50..0x82253EB4)
	// 82253D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253D54: 4BE3AD29  bl 0x8208ea7c
	ctx.lr = 0x82253D58;
	sub_8208EA60(ctx, base);
	// 82253D58: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253D5C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82253D60: 80770020  lwz r3, 0x20(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 82253D64: 4BFE16BD  bl 0x82235420
	ctx.lr = 0x82253D68;
	sub_82235420(ctx, base);
	// 82253D68: 8177001C  lwz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 82253D6C: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 82253D70: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82253D74: 41980138  blt cr6, 0x82253eac
	if ctx.cr[6].lt {
	pc = 0x82253EAC; continue 'dispatch;
	}
	// 82253D78: 3AC30004  addi r22, r3, 4
	ctx.r[22].s64 = ctx.r[3].s64 + 4;
	// 82253D7C: 83360000  lwz r25, 0(r22)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82253D80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82253D84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82253D88: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82253D8C: 83190048  lwz r24, 0x48(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(72 as u32) ) } as u64;
	// 82253D90: 81790038  lwz r11, 0x38(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 82253D94: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253D98: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82253D9C: 4099000C  ble cr6, 0x82253da8
	if !ctx.cr[6].gt {
	pc = 0x82253DA8; continue 'dispatch;
	}
	// 82253DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82253DA4: 48000010  b 0x82253db4
	pc = 0x82253DB4; continue 'dispatch;
	// 82253DA8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82253DAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82253DB0: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82253DB4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82253DB8: 41820034  beq 0x82253dec
	if ctx.cr[0].eq {
	pc = 0x82253DEC; continue 'dispatch;
	}
	// 82253DBC: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82253DC0: 81570008  lwz r10, 8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82253DC4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82253DC8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82253DCC: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82253DD0: 419A0010  beq cr6, 0x82253de0
	if ctx.cr[6].eq {
	pc = 0x82253DE0; continue 'dispatch;
	}
	// 82253DD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82253DD8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82253DDC: 48000CED  bl 0x82254ac8
	ctx.lr = 0x82253DE0;
	sub_82254AC8(ctx, base);
	// 82253DE0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82253DE4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82253DE8: 4BFFFFA8  b 0x82253d90
	pc = 0x82253D90; continue 'dispatch;
	// 82253DEC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82253DF0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82253DF4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82253DF8: 81790058  lwz r11, 0x58(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(88 as u32) ) } as u64;
	// 82253DFC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253E00: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82253E04: 4099000C  ble cr6, 0x82253e10
	if !ctx.cr[6].gt {
	pc = 0x82253E10; continue 'dispatch;
	}
	// 82253E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82253E0C: 48000010  b 0x82253e1c
	pc = 0x82253E1C; continue 'dispatch;
	// 82253E10: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82253E14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82253E18: 7F6AE02E  lwzx r27, r10, r28
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82253E1C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82253E20: 41820078  beq 0x82253e98
	if ctx.cr[0].eq {
	pc = 0x82253E98; continue 'dispatch;
	}
	// 82253E24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82253E28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82253E2C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82253E30: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 82253E34: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253E38: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82253E3C: 4099000C  ble cr6, 0x82253e48
	if !ctx.cr[6].gt {
	pc = 0x82253E48; continue 'dispatch;
	}
	// 82253E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82253E44: 48000010  b 0x82253e54
	pc = 0x82253E54; continue 'dispatch;
	// 82253E48: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82253E4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82253E50: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82253E54: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82253E58: 41820034  beq 0x82253e8c
	if ctx.cr[0].eq {
	pc = 0x82253E8C; continue 'dispatch;
	}
	// 82253E5C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82253E60: 81570008  lwz r10, 8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82253E64: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82253E68: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82253E6C: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82253E70: 419A0010  beq cr6, 0x82253e80
	if ctx.cr[6].eq {
	pc = 0x82253E80; continue 'dispatch;
	}
	// 82253E74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82253E78: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82253E7C: 48000C4D  bl 0x82254ac8
	ctx.lr = 0x82253E80;
	sub_82254AC8(ctx, base);
	// 82253E80: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82253E84: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82253E88: 4BFFFFA8  b 0x82253e30
	pc = 0x82253E30; continue 'dispatch;
	// 82253E8C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82253E90: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82253E94: 4BFFFF64  b 0x82253df8
	pc = 0x82253DF8; continue 'dispatch;
	// 82253E98: 8177001C  lwz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 82253E9C: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 82253EA0: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82253EA4: 7F155840  cmplw cr6, r21, r11
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82253EA8: 4099FED4  ble cr6, 0x82253d7c
	if !ctx.cr[6].gt {
	pc = 0x82253D7C; continue 'dispatch;
	}
	// 82253EAC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82253EB0: 4BE3AC1C  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253EB8 size=252
    let mut pc: u32 = 0x82253EB8;
    'dispatch: loop {
        match pc {
            0x82253EB8 => {
    //   block [0x82253EB8..0x82253FB4)
	// 82253EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253EBC: 4BE3ABDD  bl 0x8208ea98
	ctx.lr = 0x82253EC0;
	sub_8208EA60(ctx, base);
	// 82253EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253EC8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82253ECC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82253ED0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253ED4: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253ED8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82253EDC: 4BFC3F7D  bl 0x82217e58
	ctx.lr = 0x82253EE0;
	sub_82217E58(ctx, base);
	// 82253EE0: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82253EE4: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82253EE8: 41820018  beq 0x82253f00
	if ctx.cr[0].eq {
	pc = 0x82253F00; continue 'dispatch;
	}
	// 82253EEC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82253EF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253EF4: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82253EF8: 4BFDD839  bl 0x82231730
	ctx.lr = 0x82253EFC;
	sub_82231730(ctx, base);
	// 82253EFC: 48000008  b 0x82253f04
	pc = 0x82253F04; continue 'dispatch;
	// 82253F00: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82253F04: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253F08: 4800001C  b 0x82253f24
	pc = 0x82253F24; continue 'dispatch;
	// 82253F0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253F10: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253F14: 4BFFB23D  bl 0x8224f150
	ctx.lr = 0x82253F18;
	sub_8224F150(ctx, base);
	// 82253F18: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82253F1C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253F20: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82253F24: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82253F28: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82253F2C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82253F30: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253F34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82253F38: 409AFFD4  bne cr6, 0x82253f0c
	if !ctx.cr[6].eq {
	pc = 0x82253F0C; continue 'dispatch;
	}
	// 82253F3C: 48000058  b 0x82253f94
	pc = 0x82253F94; continue 'dispatch;
	// 82253F40: 4BFE9099  bl 0x8223cfd8
	ctx.lr = 0x82253F44;
	sub_8223CFD8(ctx, base);
	// 82253F44: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253F48: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82253F4C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82253F50: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253F54: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253F58: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82253F5C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82253F60: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82253F64: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82253F68: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82253F6C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82253F70: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82253F74: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82253F78: 40980010  bge cr6, 0x82253f88
	if !ctx.cr[6].lt {
	pc = 0x82253F88; continue 'dispatch;
	}
	// 82253F7C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82253F80: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82253F84: 7D09592E  stwx r8, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82253F88: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253F8C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82253F90: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82253F94: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82253F98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82253F9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82253FA0: 409AFFA0  bne cr6, 0x82253f40
	if !ctx.cr[6].eq {
	pc = 0x82253F40; continue 'dispatch;
	}
	// 82253FA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82253FA8: 4BFE7581  bl 0x8223b528
	ctx.lr = 0x82253FAC;
	sub_8223B528(ctx, base);
	// 82253FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82253FB0: 4BE3AB38  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82253FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82253FB8 size=80
    let mut pc: u32 = 0x82253FB8;
    'dispatch: loop {
        match pc {
            0x82253FB8 => {
    //   block [0x82253FB8..0x82254008)
	// 82253FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82253FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82253FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82253FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82253FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82253FCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82253FD0: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82253FD4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82253FD8: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82253FDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82253FE0: 419A0008  beq cr6, 0x82253fe8
	if ctx.cr[6].eq {
	pc = 0x82253FE8; continue 'dispatch;
	}
	// 82253FE4: 4BFFFED5  bl 0x82253eb8
	ctx.lr = 0x82253FE8;
	sub_82253EB8(ctx, base);
	// 82253FE8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82253FEC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82253FF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82253FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82253FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82253FFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82254000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82254004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254008 size=344
    let mut pc: u32 = 0x82254008;
    'dispatch: loop {
        match pc {
            0x82254008 => {
    //   block [0x82254008..0x82254160)
	// 82254008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225400C: 4BE3AA89  bl 0x8208ea94
	ctx.lr = 0x82254010;
	sub_8208EA60(ctx, base);
	// 82254010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82254014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82254018: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8225401C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82254020: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82254024: 814B0864  lwz r10, 0x864(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2148 as u32) ) } as u64;
	// 82254028: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8225402C: 914B0864  stw r10, 0x864(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2148 as u32), ctx.r[10].u32 ) };
	// 82254030: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82254034: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82254038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8225403C: 4BFC3E1D  bl 0x82217e58
	ctx.lr = 0x82254040;
	sub_82217E58(ctx, base);
	// 82254040: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82254044: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82254048: 41820018  beq 0x82254060
	if ctx.cr[0].eq {
	pc = 0x82254060; continue 'dispatch;
	}
	// 8225404C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82254050: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82254054: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82254058: 4BFDD6D9  bl 0x82231730
	ctx.lr = 0x8225405C;
	sub_82231730(ctx, base);
	// 8225405C: 48000008  b 0x82254064
	pc = 0x82254064; continue 'dispatch;
	// 82254060: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82254064: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82254068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8225406C: 917D0048  stw r11, 0x48(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82254070: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82254074: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82254078: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8225407C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82254080: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82254084: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82254088: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8225408C: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254090: 4BFFB0C1  bl 0x8224f150
	ctx.lr = 0x82254094;
	sub_8224F150(ctx, base);
	// 82254094: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82254098: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8225409C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822540A0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822540A4: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 822540A8: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 822540AC: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 822540B0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822540B4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822540B8: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 822540BC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 822540C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822540C4: 808B0864  lwz r4, 0x864(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2148 as u32) ) } as u64;
	// 822540C8: 480008F1  bl 0x822549b8
	ctx.lr = 0x822540CC;
	sub_822549B8(ctx, base);
	// 822540CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822540D0: 41820064  beq 0x82254134
	if ctx.cr[0].eq {
	pc = 0x82254134; continue 'dispatch;
	}
	// 822540D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822540D8: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822540DC: 4BFFB075  bl 0x8224f150
	ctx.lr = 0x822540E0;
	sub_8224F150(ctx, base);
	// 822540E0: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822540E4: 937E0048  stw r27, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[27].u32 ) };
	// 822540E8: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822540EC: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 822540F0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 822540F4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 822540F8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822540FC: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82254100: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82254104: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82254108: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8225410C: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82254110: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82254114: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82254118: 814A0864  lwz r10, 0x864(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2148 as u32) ) } as u64;
	// 8225411C: 915E0080  stw r10, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82254120: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254124: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82254128: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 8225412C: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82254130: 4BFFFF80  b 0x822540b0
	pc = 0x822540B0; continue 'dispatch;
	// 82254134: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254138: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8225413C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82254140: 419A0010  beq cr6, 0x82254150
	if ctx.cr[6].eq {
	pc = 0x82254150; continue 'dispatch;
	}
	// 82254144: 4BFE8E95  bl 0x8223cfd8
	ctx.lr = 0x82254148;
	sub_8223CFD8(ctx, base);
	// 82254148: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8225414C: 4BFFFF70  b 0x822540bc
	pc = 0x822540BC; continue 'dispatch;
	// 82254150: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82254154: 4BFE73D5  bl 0x8223b528
	ctx.lr = 0x82254158;
	sub_8223B528(ctx, base);
	// 82254158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8225415C: 4BE3A988  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254160 size=724
    let mut pc: u32 = 0x82254160;
    'dispatch: loop {
        match pc {
            0x82254160 => {
    //   block [0x82254160..0x82254434)
	// 82254160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82254164: 4BE3A91D  bl 0x8208ea80
	ctx.lr = 0x82254168;
	sub_8208EA60(ctx, base);
	// 82254168: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225416C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82254170: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82254174: 82CB00A4  lwz r22, 0xa4(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82254178: 81560048  lwz r10, 0x48(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(72 as u32) ) } as u64;
	// 8225417C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82254180: 409A0024  bne cr6, 0x822541a4
	if !ctx.cr[6].eq {
	pc = 0x822541A4; continue 'dispatch;
	}
	// 82254184: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82254188: 48000010  b 0x82254198
	pc = 0x82254198; continue 'dispatch;
	// 8225418C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82254190: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82254194: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254198: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8225419C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822541A0: 409AFFEC  bne cr6, 0x8225418c
	if !ctx.cr[6].eq {
	pc = 0x8225418C; continue 'dispatch;
	}
	// 822541A4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 822541A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822541AC: 4BFFFE5D  bl 0x82254008
	ctx.lr = 0x822541B0;
	sub_82254008(ctx, base);
	// 822541B0: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 822541B4: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 822541B8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822541BC: 4BFDDAD5  bl 0x82231c90
	ctx.lr = 0x822541C0;
	sub_82231C90(ctx, base);
	// 822541C0: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 822541C4: 831F001C  lwz r24, 0x1c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 822541C8: 2B180001  cmplwi cr6, r24, 1
	ctx.cr[6].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	// 822541CC: 40990168  ble cr6, 0x82254334
	if !ctx.cr[6].gt {
	pc = 0x82254334; continue 'dispatch;
	}
	// 822541D0: 571D103A  slwi r29, r24, 2
	ctx.r[29].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 822541D4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822541D8: 7EFCBB78  mr r28, r23
	ctx.r[28].u64 = ctx.r[23].u64;
	// 822541DC: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 822541E0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 822541E4: 7F3D582E  lwzx r25, r29, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822541E8: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 822541EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822541F0: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822541F4: 4099000C  ble cr6, 0x82254200
	if !ctx.cr[6].gt {
	pc = 0x82254200; continue 'dispatch;
	}
	// 822541F8: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 822541FC: 48000010  b 0x8225420c
	pc = 0x8225420C; continue 'dispatch;
	// 82254200: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254204: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82254208: 7F8AD82E  lwzx r28, r10, r27
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8225420C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82254210: 41820038  beq 0x82254248
	if ctx.cr[0].eq {
	pc = 0x82254248; continue 'dispatch;
	}
	// 82254214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82254218: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8225421C: 809C0048  lwz r4, 0x48(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82254220: 4BFFFD99  bl 0x82253fb8
	ctx.lr = 0x82254224;
	sub_82253FB8(ctx, base);
	// 82254224: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82254228: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8225422C: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82254230: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82254234: 40980008  bge cr6, 0x8225423c
	if !ctx.cr[6].lt {
	pc = 0x8225423C; continue 'dispatch;
	}
	// 82254238: 7D7DF12E  stwx r11, r29, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 8225423C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82254240: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82254244: 4BFFFFA4  b 0x822541e8
	pc = 0x822541E8; continue 'dispatch;
	// 82254248: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8225424C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82254250: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82254254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82254258: 4BFC3C01  bl 0x82217e58
	ctx.lr = 0x8225425C;
	sub_82217E58(ctx, base);
	// 8225425C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82254260: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82254264: 41820010  beq 0x82254274
	if ctx.cr[0].eq {
	pc = 0x82254274; continue 'dispatch;
	}
	// 82254268: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8225426C: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 82254270: 48000008  b 0x82254278
	pc = 0x82254278; continue 'dispatch;
	// 82254274: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82254278: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 8225427C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254280: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82254284: 7D5D502E  lwzx r10, r29, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82254288: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8225428C: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82254290: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82254294: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254298: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8225429C: 7D5D502E  lwzx r10, r29, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822542A0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822542A4: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 822542A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822542AC: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822542B0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822542B4: 7D7D512E  stwx r11, r29, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 822542B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822542BC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822542C0: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822542C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822542C8: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822542CC: 48000050  b 0x8225431c
	pc = 0x8225431C; continue 'dispatch;
	// 822542D0: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822542D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822542D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822542DC: 4BFFFCDD  bl 0x82253fb8
	ctx.lr = 0x822542E0;
	sub_82253FB8(ctx, base);
	// 822542E0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822542E4: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822542E8: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822542EC: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822542F0: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822542F4: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822542F8: 409A0018  bne cr6, 0x82254310
	if !ctx.cr[6].eq {
	pc = 0x82254310; continue 'dispatch;
	}
	// 822542FC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254300: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254304: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82254308: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8225430C: 4800000C  b 0x82254318
	pc = 0x82254318; continue 'dispatch;
	// 82254310: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254314: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82254318: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8225431C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82254320: 409AFFB0  bne cr6, 0x822542d0
	if !ctx.cr[6].eq {
	pc = 0x822542D0; continue 'dispatch;
	}
	// 82254324: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 82254328: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 8225432C: 2B180001  cmplwi cr6, r24, 1
	ctx.cr[6].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	// 82254330: 4199FEA4  bgt cr6, 0x822541d4
	if ctx.cr[6].gt {
	pc = 0x822541D4; continue 'dispatch;
	}
	// 82254334: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254338: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8225433C: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 82254340: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82254344: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82254348: 41980048  blt cr6, 0x82254390
	if ctx.cr[6].lt {
	pc = 0x82254390; continue 'dispatch;
	}
	// 8225434C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82254350: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254354: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254358: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8225435C: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82254360: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82254364: 419A0018  beq cr6, 0x8225437c
	if ctx.cr[6].eq {
	pc = 0x8225437C; continue 'dispatch;
	}
	// 82254368: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8225436C: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82254370: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82254374: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82254378: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 8225437C: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82254380: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82254384: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82254388: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8225438C: 4099FFC4  ble cr6, 0x82254350
	if !ctx.cr[6].gt {
	pc = 0x82254350; continue 'dispatch;
	}
	// 82254390: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82254394: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82254398: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8225439C: 41980058  blt cr6, 0x822543f4
	if ctx.cr[6].lt {
	pc = 0x822543F4; continue 'dispatch;
	}
	// 822543A0: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 822543A4: 409A000C  bne cr6, 0x822543b0
	if !ctx.cr[6].eq {
	pc = 0x822543B0; continue 'dispatch;
	}
	// 822543A8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 822543AC: 4800001C  b 0x822543c8
	pc = 0x822543C8; continue 'dispatch;
	// 822543B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822543B4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822543B8: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822543BC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822543C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822543C4: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822543C8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822543CC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822543D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822543D4: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822543D8: 90640054  stw r3, 0x54(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 822543DC: 419A0008  beq cr6, 0x822543e4
	if ctx.cr[6].eq {
	pc = 0x822543E4; continue 'dispatch;
	}
	// 822543E0: 480006B1  bl 0x82254a90
	ctx.lr = 0x822543E4;
	sub_82254A90(ctx, base);
	// 822543E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 822543E8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822543EC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822543F0: 4099FFB0  ble cr6, 0x822543a0
	if !ctx.cr[6].gt {
	pc = 0x822543A0; continue 'dispatch;
	}
	// 822543F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822543F8: 4BFFF959  bl 0x82253d50
	ctx.lr = 0x822543FC;
	sub_82253D50(ctx, base);
	// 822543FC: 8176004C  lwz r11, 0x4c(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(76 as u32) ) } as u64;
	// 82254400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82254404: 409A0028  bne cr6, 0x8225442c
	if !ctx.cr[6].eq {
	pc = 0x8225442C; continue 'dispatch;
	}
	// 82254408: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8225440C: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82254410: 48000010  b 0x82254420
	pc = 0x82254420; continue 'dispatch;
	// 82254414: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82254418: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8225441C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254420: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254424: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82254428: 409AFFEC  bne cr6, 0x82254414
	if !ctx.cr[6].eq {
	pc = 0x82254414; continue 'dispatch;
	}
	// 8225442C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82254430: 4BE3A6A0  b 0x8208ead0
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254438 size=140
    let mut pc: u32 = 0x82254438;
    'dispatch: loop {
        match pc {
            0x82254438 => {
    //   block [0x82254438..0x822544C4)
	// 82254438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225443C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82254440: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82254444: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82254448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225444C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82254450: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82254454: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254458: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8225445C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82254460: 4BFC39F9  bl 0x82217e58
	ctx.lr = 0x82254464;
	sub_82217E58(ctx, base);
	// 82254464: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82254468: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8225446C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82254470: 41820014  beq 0x82254484
	if ctx.cr[0].eq {
	pc = 0x82254484; continue 'dispatch;
	}
	// 82254474: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82254478: 4BFFF6D1  bl 0x82253b48
	ctx.lr = 0x8225447C;
	sub_82253B48(ctx, base);
	// 8225447C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82254480: 48000008  b 0x82254488
	pc = 0x82254488; continue 'dispatch;
	// 82254484: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82254488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8225448C: 4BFFFCD5  bl 0x82254160
	ctx.lr = 0x82254490;
	sub_82254160(ctx, base);
	// 82254490: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82254494: 419A0018  beq cr6, 0x822544ac
	if ctx.cr[6].eq {
	pc = 0x822544AC; continue 'dispatch;
	}
	// 82254498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8225449C: 4BFFF79D  bl 0x82253c38
	ctx.lr = 0x822544A0;
	sub_82253C38(ctx, base);
	// 822544A0: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 822544A4: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 822544A8: 4BFC3AE1  bl 0x82217f88
	ctx.lr = 0x822544AC;
	sub_82217F88(ctx, base);
	// 822544AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822544B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822544B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822544B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822544BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822544C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822544C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822544C8 size=68
    let mut pc: u32 = 0x822544C8;
    'dispatch: loop {
        match pc {
            0x822544C8 => {
    //   block [0x822544C8..0x8225450C)
	// 822544C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822544CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822544D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822544D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822544D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822544DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822544E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822544E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822544E8: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 822544EC: 4BFFA96D  bl 0x8224ee58
	ctx.lr = 0x822544F0;
	sub_8224EE58(ctx, base);
	// 822544F0: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 822544F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822544F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822544FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82254500: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82254504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82254508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254510 size=68
    let mut pc: u32 = 0x82254510;
    'dispatch: loop {
        match pc {
            0x82254510 => {
    //   block [0x82254510..0x82254554)
	// 82254510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82254514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82254518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225451C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82254520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82254524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82254528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8225452C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82254530: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82254534: 4BFFA945  bl 0x8224ee78
	ctx.lr = 0x82254538;
	sub_8224EE78(ctx, base);
	// 82254538: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 8225453C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82254540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82254544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82254548: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8225454C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82254550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254558 size=96
    let mut pc: u32 = 0x82254558;
    'dispatch: loop {
        match pc {
            0x82254558 => {
    //   block [0x82254558..0x822545B8)
	// 82254558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225455C: 4BE3A541  bl 0x8208ea9c
	ctx.lr = 0x82254560;
	sub_8208EA60(ctx, base);
	// 82254560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82254564: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82254568: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8225456C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82254570: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82254574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82254578: 409A0028  bne cr6, 0x822545a0
	if !ctx.cr[6].eq {
	pc = 0x822545A0; continue 'dispatch;
	}
	// 8225457C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82254580: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82254584: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82254588: 38CB6208  addi r6, r11, 0x6208
	ctx.r[6].s64 = ctx.r[11].s64 + 25096;
	// 8225458C: 38AA61E8  addi r5, r10, 0x61e8
	ctx.r[5].s64 = ctx.r[10].s64 + 25064;
	// 82254590: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 82254594: 38E000B5  li r7, 0xb5
	ctx.r[7].s64 = 181;
	// 82254598: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8225459C: 4BF0146D  bl 0x82155a08
	ctx.lr = 0x822545A0;
	sub_82155A08(ctx, base);
	// 822545A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822545A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822545A8: 4BFFA8D1  bl 0x8224ee78
	ctx.lr = 0x822545AC;
	sub_8224EE78(ctx, base);
	// 822545AC: 93BE03B4  stw r29, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[29].u32 ) };
	// 822545B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822545B4: 4BE3A538  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822545B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822545B8 size=96
    let mut pc: u32 = 0x822545B8;
    'dispatch: loop {
        match pc {
            0x822545B8 => {
    //   block [0x822545B8..0x82254618)
	// 822545B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822545BC: 4BE3A4E1  bl 0x8208ea9c
	ctx.lr = 0x822545C0;
	sub_8208EA60(ctx, base);
	// 822545C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822545C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822545C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822545CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822545D0: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 822545D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822545D8: 409A0028  bne cr6, 0x82254600
	if !ctx.cr[6].eq {
	pc = 0x82254600; continue 'dispatch;
	}
	// 822545DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822545E0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 822545E4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 822545E8: 38CB6208  addi r6, r11, 0x6208
	ctx.r[6].s64 = ctx.r[11].s64 + 25096;
	// 822545EC: 38AA6258  addi r5, r10, 0x6258
	ctx.r[5].s64 = ctx.r[10].s64 + 25176;
	// 822545F0: 388954C0  addi r4, r9, 0x54c0
	ctx.r[4].s64 = ctx.r[9].s64 + 21696;
	// 822545F4: 38E000C1  li r7, 0xc1
	ctx.r[7].s64 = 193;
	// 822545F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822545FC: 4BF0140D  bl 0x82155a08
	ctx.lr = 0x82254600;
	sub_82155A08(ctx, base);
	// 82254600: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82254604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82254608: 4BFFA851  bl 0x8224ee58
	ctx.lr = 0x8225460C;
	sub_8224EE58(ctx, base);
	// 8225460C: 93BE03B4  stw r29, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[29].u32 ) };
	// 82254610: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82254614: 4BE3A4D8  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254618 size=72
    let mut pc: u32 = 0x82254618;
    'dispatch: loop {
        match pc {
            0x82254618 => {
    //   block [0x82254618..0x82254660)
	// 82254618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8225461C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82254620: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82254624: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82254628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8225462C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82254630: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82254634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82254638: 809F0088  lwz r4, 0x88(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8225463C: 4BFFA83D  bl 0x8224ee78
	ctx.lr = 0x82254640;
	sub_8224EE78(ctx, base);
	// 82254640: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82254644: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 82254648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8225464C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82254650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82254654: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82254658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8225465C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82254660 size=136
    let mut pc: u32 = 0x82254660;
    'dispatch: loop {
        match pc {
            0x82254660 => {
    //   block [0x82254660..0x822546E8)
	// 82254660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82254664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82254668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8225466C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82254670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82254674: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82254678: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8225467C: 4800003C  b 0x822546b8
	pc = 0x822546B8; continue 'dispatch;
	// 82254680: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82254684: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82254688: 4182002C  beq 0x822546b4
	if ctx.cr[0].eq {
	pc = 0x822546B4; continue 'dispatch;
	}
	// 8225468C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82254690: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82254694: 409A0020  bne cr6, 0x822546b4
	if !ctx.cr[6].eq {
	pc = 0x822546B4; continue 'dispatch;
	}
	// 82254698: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8225469C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822546A0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 822546A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822546A8: 4E800421  bctrl
	ctx.lr = 0x822546AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822546AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822546B0: 41820030  beq 0x822546e0
	if ctx.cr[0].eq {
	pc = 0x822546E0; continue 'dispatch;
	}
	// 822546B4: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822546B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822546BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822546C0: 409AFFC0  bne cr6, 0x82254680
	if !ctx.cr[6].eq {
	pc = 0x82254680; continue 'dispatch;
	}
	// 822546C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822546C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822546CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822546D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822546D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822546D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822546DC: 4E800020  blr
	return;
	// 822546E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822546E4: 4BFFFFE4  b 0x822546c8
	pc = 0x822546C8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822546E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822546E8 size=608
    let mut pc: u32 = 0x822546E8;
    'dispatch: loop {
        match pc {
            0x822546E8 => {
    //   block [0x822546E8..0x82254948)
	// 822546E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822546EC: 4BE3A3A9  bl 0x8208ea94
	ctx.lr = 0x822546F0;
	sub_8208EA60(ctx, base);
	// 822546F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822546F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822546F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822546FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82254700: 396B6274  addi r11, r11, 0x6274
	ctx.r[11].s64 = ctx.r[11].s64 + 25204;
	// 82254704: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 82254708: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8225470C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82254710: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82254714: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82254718: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8225471C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82254720: 4BFFA799  bl 0x8224eeb8
	ctx.lr = 0x82254724;
	sub_8224EEB8(ctx, base);
	// 82254724: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254728: 39200400  li r9, 0x400
	ctx.r[9].s64 = 1024;
	// 8225472C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82254730: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82254734: 814B056C  lwz r10, 0x56c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1388 as u32) ) } as u64;
	// 82254738: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8225473C: 814B056C  lwz r10, 0x56c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1388 as u32) ) } as u64;
	// 82254740: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82254744: 914B056C  stw r10, 0x56c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1388 as u32), ctx.r[10].u32 ) };
	// 82254748: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8225474C: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82254750: 913F0048  stw r9, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 82254754: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82254758: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8225475C: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82254760: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82254764: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82254768: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8225476C: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82254770: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82254774: 9BDF007C  stb r30, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u8 ) };
	// 82254778: 911F0080  stw r8, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 8225477C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82254780: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82254784: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82254788: 4BFC36D1  bl 0x82217e58
	ctx.lr = 0x8225478C;
	sub_82217E58(ctx, base);
	// 8225478C: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82254790: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82254794: 41820018  beq 0x822547ac
	if ctx.cr[0].eq {
	pc = 0x822547AC; continue 'dispatch;
	}
	// 82254798: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8225479C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822547A0: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822547A4: 4BFDCF8D  bl 0x82231730
	ctx.lr = 0x822547A8;
	sub_82231730(ctx, base);
	// 822547A8: 48000008  b 0x822547b0
	pc = 0x822547B0; continue 'dispatch;
	// 822547AC: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 822547B0: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 822547B4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 822547B8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822547BC: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822547C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822547C4: 4BFC3695  bl 0x82217e58
	ctx.lr = 0x822547C8;
	sub_82217E58(ctx, base);
	// 822547C8: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822547CC: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 822547D0: 41820018  beq 0x822547e8
	if ctx.cr[0].eq {
	pc = 0x822547E8; continue 'dispatch;
	}
	// 822547D4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822547D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822547DC: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822547E0: 4BFDCF51  bl 0x82231730
	ctx.lr = 0x822547E4;
	sub_82231730(ctx, base);
	// 822547E4: 48000008  b 0x822547ec
	pc = 0x822547EC; continue 'dispatch;
	// 822547E8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 822547EC: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 822547F0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 822547F4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822547F8: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822547FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82254800: 4BFC3659  bl 0x82217e58
	ctx.lr = 0x82254804;
	sub_82217E58(ctx, base);
	// 82254804: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82254808: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8225480C: 41820018  beq 0x82254824
	if ctx.cr[0].eq {
	pc = 0x82254824; continue 'dispatch;
	}
	// 82254810: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82254818: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8225481C: 4BFDCF15  bl 0x82231730
	ctx.lr = 0x82254820;
	sub_82231730(ctx, base);
	// 82254820: 48000008  b 0x82254828
	pc = 0x82254828; continue 'dispatch;
	// 82254824: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82254828: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8225482C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82254830: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254834: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82254838: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8225483C: 4BFC361D  bl 0x82217e58
	ctx.lr = 0x82254840;
	sub_82217E58(ctx, base);
	// 82254840: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82254844: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82254848: 41820018  beq 0x82254860
	if ctx.cr[0].eq {
	pc = 0x82254860; continue 'dispatch;
	}
	// 8225484C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254850: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82254854: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82254858: 4BFDCED9  bl 0x82231730
	ctx.lr = 0x8225485C;
	sub_82231730(ctx, base);
	// 8225485C: 48000008  b 0x82254864
	pc = 0x82254864; continue 'dispatch;
	// 82254860: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82254864: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82254868: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8225486C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254870: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82254874: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82254878: 4BFC35E1  bl 0x82217e58
	ctx.lr = 0x8225487C;
	sub_82217E58(ctx, base);
	// 8225487C: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82254880: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82254884: 41820018  beq 0x8225489c
	if ctx.cr[0].eq {
	pc = 0x8225489C; continue 'dispatch;
	}
	// 82254888: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8225488C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82254890: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82254894: 4BFDCE9D  bl 0x82231730
	ctx.lr = 0x82254898;
	sub_82231730(ctx, base);
	// 82254898: 48000008  b 0x822548a0
	pc = 0x822548A0; continue 'dispatch;
	// 8225489C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 822548A0: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 822548A4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822548A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822548AC: 83AB05AC  lwz r29, 0x5ac(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822548B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822548B4: 4BFC35A5  bl 0x82217e58
	ctx.lr = 0x822548B8;
	sub_82217E58(ctx, base);
	// 822548B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822548BC: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822548C0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822548C4: 41820010  beq 0x822548d4
	if ctx.cr[0].eq {
	pc = 0x822548D4; continue 'dispatch;
	}
	// 822548C8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822548CC: 4BFFDF35  bl 0x82252800
	ctx.lr = 0x822548D0;
	sub_82252800(ctx, base);
	// 822548D0: 48000008  b 0x822548d8
	pc = 0x822548D8; continue 'dispatch;
	// 822548D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822548D8: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 822548DC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 822548E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822548E4: 83AB05AC  lwz r29, 0x5ac(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 822548E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822548EC: 4BFC356D  bl 0x82217e58
	ctx.lr = 0x822548F0;
	sub_82217E58(ctx, base);
	// 822548F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822548F4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822548F8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822548FC: 41820010  beq 0x8225490c
	if ctx.cr[0].eq {
	pc = 0x8225490C; continue 'dispatch;
	}
	// 82254900: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82254904: 4BFFDF55  bl 0x82252858
	ctx.lr = 0x82254908;
	sub_82252858(ctx, base);
	// 82254908: 48000008  b 0x82254910
	pc = 0x82254910; continue 'dispatch;
	// 8225490C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82254910: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82254914: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82254918: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8225491C: 4BFFA625  bl 0x8224ef40
	ctx.lr = 0x82254920;
	sub_8224EF40(ctx, base);
	// 82254920: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82254924: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82254928: 4BFFA5F1  bl 0x8224ef18
	ctx.lr = 0x8225492C;
	sub_8224EF18(ctx, base);
	// 8225492C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82254930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82254934: 93EB03B4  stw r31, 0x3b4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 82254938: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8225493C: 93EB03B4  stw r31, 0x3b4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 82254940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82254944: 4BE3A1A0  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82254948 size=12
    let mut pc: u32 = 0x82254948;
    'dispatch: loop {
        match pc {
            0x82254948 => {
    //   block [0x82254948..0x82254954)
	// 82254948: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8225494C: 386B62A4  addi r3, r11, 0x62a4
	ctx.r[3].s64 = ctx.r[11].s64 + 25252;
	// 82254950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82254958 size=40
    let mut pc: u32 = 0x82254958;
    'dispatch: loop {
        match pc {
            0x82254958 => {
    //   block [0x82254958..0x82254980)
	// 82254958: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8225495C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82254960: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82254964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82254968: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8225496C: 80CA0004  lwz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254970: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82254974: 4099000C  ble cr6, 0x82254980
	if !ctx.cr[6].gt {
		sub_82254980(ctx, base);
		return;
	}
	// 82254978: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8225497C: 48000010  b 0x8225498c
	sub_82254980(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82254980 size=28
    let mut pc: u32 = 0x82254980;
    'dispatch: loop {
        match pc {
            0x82254980 => {
    //   block [0x82254980..0x8225499C)
	// 82254980: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82254984: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82254988: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8225498C: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82254990: 4182001C  beq 0x822549ac
	if ctx.cr[0].eq {
		sub_822549AC(ctx, base);
		return;
	}
	// 82254994: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82254998: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8225499C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8225499C size=16
    let mut pc: u32 = 0x8225499C;
    'dispatch: loop {
        match pc {
            0x8225499C => {
    //   block [0x8225499C..0x822549AC)
	// 8225499C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 822549A0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 822549A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822549A8: 4BFFFFC8  b 0x82254970
	sub_82254958(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822549AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822549AC size=8
    let mut pc: u32 = 0x822549AC;
    'dispatch: loop {
        match pc {
            0x822549AC => {
    //   block [0x822549AC..0x822549B4)
	// 822549AC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 822549B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822549B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822549B8 size=36
    let mut pc: u32 = 0x822549B8;
    'dispatch: loop {
        match pc {
            0x822549B8 => {
    //   block [0x822549B8..0x822549DC)
	// 822549B8: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 822549BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822549C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822549C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 822549C8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822549CC: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 822549D0: 4099000C  ble cr6, 0x822549dc
	if !ctx.cr[6].gt {
		sub_822549DC(ctx, base);
		return;
	}
	// 822549D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822549D8: 48000010  b 0x822549e8
	sub_822549DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822549DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822549DC size=32
    let mut pc: u32 = 0x822549DC;
    'dispatch: loop {
        match pc {
            0x822549DC => {
    //   block [0x822549DC..0x822549FC)
	// 822549DC: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822549E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822549E4: 7C66502E  lwzx r3, r6, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822549E8: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822549EC: 4182001C  beq 0x82254a08
	if ctx.cr[0].eq {
		sub_82254A08(ctx, base);
		return;
	}
	// 822549F0: 81230080  lwz r9, 0x80(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 822549F4: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 822549F8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822549FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822549FC size=12
    let mut pc: u32 = 0x822549FC;
    'dispatch: loop {
        match pc {
            0x822549FC => {
    //   block [0x822549FC..0x82254A08)
	// 822549FC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82254A00: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82254A04: 4BFFFFC8  b 0x822549cc
	sub_822549B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82254A08 size=8
    let mut pc: u32 = 0x82254A08;
    'dispatch: loop {
        match pc {
            0x82254A08 => {
    //   block [0x82254A08..0x82254A10)
	// 82254A08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82254A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82254A10 size=24
    let mut pc: u32 = 0x82254A10;
    'dispatch: loop {
        match pc {
            0x82254A10 => {
    //   block [0x82254A10..0x82254A28)
	// 82254A10: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82254A14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254A18: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 82254A1C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82254A20: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82254A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82254A28 size=24
    let mut pc: u32 = 0x82254A28;
    'dispatch: loop {
        match pc {
            0x82254A28 => {
    //   block [0x82254A28..0x82254A40)
	// 82254A28: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82254A2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254A30: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 82254A34: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82254A38: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82254A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82254A40 size=12
    let mut pc: u32 = 0x82254A40;
    'dispatch: loop {
        match pc {
            0x82254A40 => {
    //   block [0x82254A40..0x82254A4C)
	// 82254A40: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82254A44: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82254A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82254A50 size=12
    let mut pc: u32 = 0x82254A50;
    'dispatch: loop {
        match pc {
            0x82254A50 => {
    //   block [0x82254A50..0x82254A5C)
	// 82254A50: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82254A54: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82254A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


