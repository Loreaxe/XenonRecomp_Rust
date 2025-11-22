pub fn sub_82DDB9AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB9AC size=8
    let mut pc: u32 = 0x82DDB9AC;
    'dispatch: loop {
        match pc {
            0x82DDB9AC => {
    //   block [0x82DDB9AC..0x82DDB9B4)
	// 82DDB9AC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDB9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB9B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB9B4 size=8
    let mut pc: u32 = 0x82DDB9B4;
    'dispatch: loop {
        match pc {
            0x82DDB9B4 => {
    //   block [0x82DDB9B4..0x82DDB9BC)
	// 82DDB9B4: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDB9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB9BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB9BC size=8
    let mut pc: u32 = 0x82DDB9BC;
    'dispatch: loop {
        match pc {
            0x82DDB9BC => {
    //   block [0x82DDB9BC..0x82DDB9C4)
	// 82DDB9BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DDB9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB9C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB9C4 size=8
    let mut pc: u32 = 0x82DDB9C4;
    'dispatch: loop {
        match pc {
            0x82DDB9C4 => {
    //   block [0x82DDB9C4..0x82DDB9CC)
	// 82DDB9C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDB9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB9CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB9CC size=8
    let mut pc: u32 = 0x82DDB9CC;
    'dispatch: loop {
        match pc {
            0x82DDB9CC => {
    //   block [0x82DDB9CC..0x82DDB9D4)
	// 82DDB9CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDB9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB9D8 size=52
    let mut pc: u32 = 0x82DDB9D8;
    'dispatch: loop {
        match pc {
            0x82DDB9D8 => {
    //   block [0x82DDB9D8..0x82DDBA0C)
	// 82DDB9D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDB9DC: 2B04003D  cmplwi cr6, r4, 0x3d
	ctx.cr[6].compare_u32(ctx.r[4].u32, 61 as u32, &mut ctx.xer);
	// 82DDB9E0: 41990174  bgt cr6, 0x82ddbb54
	if ctx.cr[6].gt {
		sub_82DDBB54(ctx, base);
		return;
	}
	// 82DDB9E4: 3D808211  lis r12, -0x7def
	ctx.r[12].s64 = -2112815104;
	// 82DDB9E8: 398C3918  addi r12, r12, 0x3918
	ctx.r[12].s64 = ctx.r[12].s64 + 14616;
	// 82DDB9EC: 7C0C20AE  lbzx r0, r12, r4
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DDB9F0: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DDB9F4: 3D8082DE  lis r12, -0x7d22
	ctx.r[12].s64 = -2099380224;
	// 82DDB9F8: 398CBA0C  addi r12, r12, -0x45f4
	ctx.r[12].s64 = ctx.r[12].s64 + -17908;
	// 82DDB9FC: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82DDBA00: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82DDBA04: 60000000  nop
	// 82DDBA08: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBA0C size=20
    let mut pc: u32 = 0x82DDBA0C;
    'dispatch: loop {
        match pc {
            0x82DDBA0C => {
    //   block [0x82DDBA0C..0x82DDBA20)
	// 82DDBA0C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBA14: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBA18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDBA1C: 48000144  b 0x82ddbb60
	sub_82DDBB54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBA20 size=24
    let mut pc: u32 = 0x82DDBA20;
    'dispatch: loop {
        match pc {
            0x82DDBA20 => {
    //   block [0x82DDBA20..0x82DDBA38)
	// 82DDBA20: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82DDBA24: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDBA28: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBA2C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDBA30: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDBA34: 48000130  b 0x82ddbb64
	sub_82DDBB54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBA38 size=32
    let mut pc: u32 = 0x82DDBA38;
    'dispatch: loop {
        match pc {
            0x82DDBA38 => {
    //   block [0x82DDBA38..0x82DDBA58)
	// 82DDBA38: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82DDBA3C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DDBA40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBA44: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBA48: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBA4C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBA50: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDBA54: 48000114  b 0x82ddbb68
	sub_82DDBB54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBA58 size=36
    let mut pc: u32 = 0x82DDBA58;
    'dispatch: loop {
        match pc {
            0x82DDBA58 => {
    //   block [0x82DDBA58..0x82DDBA7C)
	// 82DDBA58: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82DDBA5C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DDBA60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBA64: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBA68: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDBA6C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBA70: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDBA74: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDBA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBA7C size=28
    let mut pc: u32 = 0x82DDBA7C;
    'dispatch: loop {
        match pc {
            0x82DDBA7C => {
    //   block [0x82DDBA7C..0x82DDBA98)
	// 82DDBA7C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBA80: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBA88: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBA8C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DDBA90: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDBA94: 480000D4  b 0x82ddbb68
	sub_82DDBB54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBA98 size=12
    let mut pc: u32 = 0x82DDBA98;
    'dispatch: loop {
        match pc {
            0x82DDBA98 => {
    //   block [0x82DDBA98..0x82DDBAA4)
	// 82DDBA98: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDBA9C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDBAA0: 480000BC  b 0x82ddbb5c
	sub_82DDBB54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBAA4 size=8
    let mut pc: u32 = 0x82DDBAA4;
    'dispatch: loop {
        match pc {
            0x82DDBAA4 => {
    //   block [0x82DDBAA4..0x82DDBAAC)
	// 82DDBAA4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DDBAA8: 4BFFFF68  b 0x82ddba10
	sub_82DDBA0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBAAC size=28
    let mut pc: u32 = 0x82DDBAAC;
    'dispatch: loop {
        match pc {
            0x82DDBAAC => {
    //   block [0x82DDBAAC..0x82DDBAC8)
	// 82DDBAAC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBAB0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDBAB4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBAB8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBABC: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDBAC0: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDBAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBAC8 size=12
    let mut pc: u32 = 0x82DDBAC8;
    'dispatch: loop {
        match pc {
            0x82DDBAC8 => {
    //   block [0x82DDBAC8..0x82DDBAD4)
	// 82DDBAC8: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82DDBACC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82DDBAD0: 4BFFFF58  b 0x82ddba28
	sub_82DDBA20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBAD4 size=12
    let mut pc: u32 = 0x82DDBAD4;
    'dispatch: loop {
        match pc {
            0x82DDBAD4 => {
    //   block [0x82DDBAD4..0x82DDBAE0)
	// 82DDBAD4: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82DDBAD8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DDBADC: 4BFFFF84  b 0x82ddba60
	sub_82DDBA58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBAE0 size=28
    let mut pc: u32 = 0x82DDBAE0;
    'dispatch: loop {
        match pc {
            0x82DDBAE0 => {
    //   block [0x82DDBAE0..0x82DDBAFC)
	// 82DDBAE0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DDBAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBAE8: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82DDBAEC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBAF0: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBAF4: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDBAF8: 4800006C  b 0x82ddbb64
	sub_82DDBB54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBAFC size=8
    let mut pc: u32 = 0x82DDBAFC;
    'dispatch: loop {
        match pc {
            0x82DDBAFC => {
    //   block [0x82DDBAFC..0x82DDBB04)
	// 82DDBAFC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DDBB00: 4BFFFF80  b 0x82ddba80
	sub_82DDBA7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB04 size=8
    let mut pc: u32 = 0x82DDBB04;
    'dispatch: loop {
        match pc {
            0x82DDBB04 => {
    //   block [0x82DDBB04..0x82DDBB0C)
	// 82DDBB04: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBB08: 4BFFFF08  b 0x82ddba10
	sub_82DDBA0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB0C size=16
    let mut pc: u32 = 0x82DDBB0C;
    'dispatch: loop {
        match pc {
            0x82DDBB0C => {
    //   block [0x82DDBB0C..0x82DDBB1C)
	// 82DDBB0C: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 82DDBB10: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBB14: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DDBB18: 4BFFFF6C  b 0x82ddba84
	sub_82DDBA7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB1C size=8
    let mut pc: u32 = 0x82DDBB1C;
    'dispatch: loop {
        match pc {
            0x82DDBB1C => {
    //   block [0x82DDBB1C..0x82DDBB24)
	// 82DDBB1C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DDBB20: 4BFFFF90  b 0x82ddbab0
	sub_82DDBAAC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB24 size=8
    let mut pc: u32 = 0x82DDBB24;
    'dispatch: loop {
        match pc {
            0x82DDBB24 => {
    //   block [0x82DDBB24..0x82DDBB2C)
	// 82DDBB24: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBB28: 4BFFFF58  b 0x82ddba80
	sub_82DDBA7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB2C size=8
    let mut pc: u32 = 0x82DDBB2C;
    'dispatch: loop {
        match pc {
            0x82DDBB2C => {
    //   block [0x82DDBB2C..0x82DDBB34)
	// 82DDBB2C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBB30: 4BFFFF80  b 0x82ddbab0
	sub_82DDBAAC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB34 size=32
    let mut pc: u32 = 0x82DDBB34;
    'dispatch: loop {
        match pc {
            0x82DDBB34 => {
    //   block [0x82DDBB34..0x82DDBB54)
	// 82DDBB34: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBB3C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBB40: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBB44: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBB48: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDBB4C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDBB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB54 size=28
    let mut pc: u32 = 0x82DDBB54;
    'dispatch: loop {
        match pc {
            0x82DDBB54 => {
    //   block [0x82DDBB54..0x82DDBB70)
	// 82DDBB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBB58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDBB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBB60: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDBB64: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDBB68: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDBB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB70 size=56
    let mut pc: u32 = 0x82DDBB70;
    'dispatch: loop {
        match pc {
            0x82DDBB70 => {
    //   block [0x82DDBB70..0x82DDBBA8)
	// 82DDBB70: 3944FFFE  addi r10, r4, -2
	ctx.r[10].s64 = ctx.r[4].s64 + -2;
	// 82DDBB74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDBB78: 2B0A0024  cmplwi cr6, r10, 0x24
	ctx.cr[6].compare_u32(ctx.r[10].u32, 36 as u32, &mut ctx.xer);
	// 82DDBB7C: 41990144  bgt cr6, 0x82ddbcc0
	if ctx.cr[6].gt {
		sub_82DDBCC0(ctx, base);
		return;
	}
	// 82DDBB80: 3D808211  lis r12, -0x7def
	ctx.r[12].s64 = -2112815104;
	// 82DDBB84: 398C3958  addi r12, r12, 0x3958
	ctx.r[12].s64 = ctx.r[12].s64 + 14680;
	// 82DDBB88: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDBB8C: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DDBB90: 3D8082DE  lis r12, -0x7d22
	ctx.r[12].s64 = -2099380224;
	// 82DDBB94: 398CBBA8  addi r12, r12, -0x4458
	ctx.r[12].s64 = ctx.r[12].s64 + -17496;
	// 82DDBB98: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82DDBB9C: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82DDBBA0: 60000000  nop
	// 82DDBBA4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBBA8 size=20
    let mut pc: u32 = 0x82DDBBA8;
    'dispatch: loop {
        match pc {
            0x82DDBBA8 => {
    //   block [0x82DDBBA8..0x82DDBBBC)
	// 82DDBBA8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBBAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBBB0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBBB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDBBB8: 48000114  b 0x82ddbccc
	sub_82DDBCC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBBBC size=24
    let mut pc: u32 = 0x82DDBBBC;
    'dispatch: loop {
        match pc {
            0x82DDBBBC => {
    //   block [0x82DDBBBC..0x82DDBBD4)
	// 82DDBBBC: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82DDBBC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDBBC4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBBC8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDBBCC: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDBBD0: 48000100  b 0x82ddbcd0
	sub_82DDBCC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBBD4 size=32
    let mut pc: u32 = 0x82DDBBD4;
    'dispatch: loop {
        match pc {
            0x82DDBBD4 => {
    //   block [0x82DDBBD4..0x82DDBBF4)
	// 82DDBBD4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82DDBBD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DDBBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBBE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBBE4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBBE8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBBEC: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDBBF0: 480000E4  b 0x82ddbcd4
	sub_82DDBCC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBBF4 size=36
    let mut pc: u32 = 0x82DDBBF4;
    'dispatch: loop {
        match pc {
            0x82DDBBF4 => {
    //   block [0x82DDBBF4..0x82DDBC18)
	// 82DDBBF4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82DDBBF8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DDBBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBC00: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBC04: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDBC08: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBC0C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDBC10: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDBC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC18 size=28
    let mut pc: u32 = 0x82DDBC18;
    'dispatch: loop {
        match pc {
            0x82DDBC18 => {
    //   block [0x82DDBC18..0x82DDBC34)
	// 82DDBC18: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBC1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBC20: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBC24: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DDBC28: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBC2C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDBC30: 480000A4  b 0x82ddbcd4
	sub_82DDBCC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC34 size=12
    let mut pc: u32 = 0x82DDBC34;
    'dispatch: loop {
        match pc {
            0x82DDBC34 => {
    //   block [0x82DDBC34..0x82DDBC40)
	// 82DDBC34: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDBC38: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDBC3C: 4800008C  b 0x82ddbcc8
	sub_82DDBCC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC40 size=8
    let mut pc: u32 = 0x82DDBC40;
    'dispatch: loop {
        match pc {
            0x82DDBC40 => {
    //   block [0x82DDBC40..0x82DDBC48)
	// 82DDBC40: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DDBC44: 4BFFFF68  b 0x82ddbbac
	sub_82DDBBA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC48 size=28
    let mut pc: u32 = 0x82DDBC48;
    'dispatch: loop {
        match pc {
            0x82DDBC48 => {
    //   block [0x82DDBC48..0x82DDBC64)
	// 82DDBC48: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DDBC4C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDBC50: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBC54: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBC58: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDBC5C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDBC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC64 size=12
    let mut pc: u32 = 0x82DDBC64;
    'dispatch: loop {
        match pc {
            0x82DDBC64 => {
    //   block [0x82DDBC64..0x82DDBC70)
	// 82DDBC64: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82DDBC68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82DDBC6C: 4BFFFF58  b 0x82ddbbc4
	sub_82DDBBBC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC70 size=12
    let mut pc: u32 = 0x82DDBC70;
    'dispatch: loop {
        match pc {
            0x82DDBC70 => {
    //   block [0x82DDBC70..0x82DDBC7C)
	// 82DDBC70: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82DDBC74: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DDBC78: 4BFFFF84  b 0x82ddbbfc
	sub_82DDBBF4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC7C size=28
    let mut pc: u32 = 0x82DDBC7C;
    'dispatch: loop {
        match pc {
            0x82DDBC7C => {
    //   block [0x82DDBC7C..0x82DDBC98)
	// 82DDBC7C: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DDBC80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBC84: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82DDBC88: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBC8C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DDBC90: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDBC94: 4800003C  b 0x82ddbcd0
	sub_82DDBCC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBC98 size=8
    let mut pc: u32 = 0x82DDBC98;
    'dispatch: loop {
        match pc {
            0x82DDBC98 => {
    //   block [0x82DDBC98..0x82DDBCA0)
	// 82DDBC98: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DDBC9C: 4BFFFF80  b 0x82ddbc1c
	sub_82DDBC18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBCA0 size=8
    let mut pc: u32 = 0x82DDBCA0;
    'dispatch: loop {
        match pc {
            0x82DDBCA0 => {
    //   block [0x82DDBCA0..0x82DDBCA8)
	// 82DDBCA0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBCA4: 4BFFFF08  b 0x82ddbbac
	sub_82DDBBA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBCA8 size=8
    let mut pc: u32 = 0x82DDBCA8;
    'dispatch: loop {
        match pc {
            0x82DDBCA8 => {
    //   block [0x82DDBCA8..0x82DDBCB0)
	// 82DDBCA8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DDBCAC: 4BFFFFA0  b 0x82ddbc4c
	sub_82DDBC48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBCB0 size=8
    let mut pc: u32 = 0x82DDBCB0;
    'dispatch: loop {
        match pc {
            0x82DDBCB0 => {
    //   block [0x82DDBCB0..0x82DDBCB8)
	// 82DDBCB0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBCB4: 4BFFFF68  b 0x82ddbc1c
	sub_82DDBC18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBCB8 size=8
    let mut pc: u32 = 0x82DDBCB8;
    'dispatch: loop {
        match pc {
            0x82DDBCB8 => {
    //   block [0x82DDBCB8..0x82DDBCC0)
	// 82DDBCB8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DDBCBC: 4BFFFF90  b 0x82ddbc4c
	sub_82DDBC48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBCC0 size=28
    let mut pc: u32 = 0x82DDBCC0;
    'dispatch: loop {
        match pc {
            0x82DDBCC0 => {
    //   block [0x82DDBCC0..0x82DDBCDC)
	// 82DDBCC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBCC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDBCC8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDBCCC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDBCD0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDBCD4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDBCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDBCE0 size=372
    let mut pc: u32 = 0x82DDBCE0;
    'dispatch: loop {
        match pc {
            0x82DDBCE0 => {
    //   block [0x82DDBCE0..0x82DDBE54)
	// 82DDBCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBCE4: 483CC481  bl 0x831a8164
	ctx.lr = 0x82DDBCE8;
	sub_831A8130(ctx, base);
	// 82DDBCE8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBCEC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DDBCF0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DDBCF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDBCF8: 4BFFFBB1  bl 0x82ddb8a8
	ctx.lr = 0x82DDBCFC;
	sub_82DDB8A8(ctx, base);
	// 82DDBCFC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DDBD00: 4082000C  bne 0x82ddbd0c
	if !ctx.cr[0].eq {
	pc = 0x82DDBD0C; continue 'dispatch;
	}
	// 82DDBD04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDBD08: 48000144  b 0x82ddbe4c
	pc = 0x82DDBE4C; continue 'dispatch;
	// 82DDBD0C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDBD10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDBD14: 4BFFFCC5  bl 0x82ddb9d8
	ctx.lr = 0x82DDBD18;
	sub_82DDB9D8(ctx, base);
	// 82DDBD18: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82DDBD1C: 4099006C  ble cr6, 0x82ddbd88
	if !ctx.cr[6].gt {
	pc = 0x82DDBD88; continue 'dispatch;
	}
	// 82DDBD20: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDBD24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDBD28: 409900EC  ble cr6, 0x82ddbe14
	if !ctx.cr[6].gt {
	pc = 0x82DDBE14; continue 'dispatch;
	}
	// 82DDBD2C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DDBD30: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DDBD34: 7FC75850  subf r30, r7, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82DDBD38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DDBD3C: 7D5E402E  lwzx r10, r30, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DDBD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDBD44: 80A80000  lwz r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBD48: 7D491E71  srawi. r9, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DDBD4C: 40810028  ble 0x82ddbd74
	if !ctx.cr[0].gt {
	pc = 0x82DDBD74; continue 'dispatch;
	}
	// 82DDBD50: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DDBD54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBD58: 7CFF3A14  add r7, r31, r7
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 82DDBD5C: 7CBB5630  sraw r27, r5, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[5].s32 >> tmp.u32) as i64;
	// 82DDBD60: 7F6759AE  stbx r27, r7, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u8) };
	// 82DDBD64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDBD68: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDBD6C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DDBD70: 4198FFEC  blt cr6, 0x82ddbd5c
	if ctx.cr[6].lt {
	pc = 0x82DDBD5C; continue 'dispatch;
	}
	// 82DDBD74: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DDBD78: 7FE9FA14  add r31, r9, r31
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82DDBD7C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DDBD80: 4082FFBC  bne 0x82ddbd3c
	if !ctx.cr[0].eq {
	pc = 0x82DDBD3C; continue 'dispatch;
	}
	// 82DDBD84: 48000090  b 0x82ddbe14
	pc = 0x82DDBE14; continue 'dispatch;
	// 82DDBD88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DDBD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBD90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDBD94: 40990054  ble cr6, 0x82ddbde8
	if !ctx.cr[6].gt {
	pc = 0x82DDBDE8; continue 'dispatch;
	}
	// 82DDBD98: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DDBD9C: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82DDBDA0: 7C875050  subf r4, r7, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82DDBDA4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DDBDA8: 7D2B202E  lwzx r9, r11, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DDBDAC: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 82DDBDB0: 40980014  bge cr6, 0x82ddbdc4
	if !ctx.cr[6].lt {
	pc = 0x82DDBDC4; continue 'dispatch;
	}
	// 82DDBDB4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DDBDB8: 7CE74830  slw r7, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDBDBC: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82DDBDC0: 48000008  b 0x82ddbdc8
	pc = 0x82DDBDC8; continue 'dispatch;
	// 82DDBDC4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82DDBDC8: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBDCC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDBDD0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DDBDD4: 7FE73838  and r7, r31, r7
	ctx.r[7].u64 = ctx.r[31].u64 & ctx.r[7].u64;
	// 82DDBDD8: 7CE74030  slw r7, r7, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDBDDC: 7CE52B78  or r5, r7, r5
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 82DDBDE0: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DDBDE4: 4082FFC4  bne 0x82ddbda8
	if !ctx.cr[0].eq {
	pc = 0x82DDBDA8; continue 'dispatch;
	}
	// 82DDBDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDBDEC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DDBDF0: 40990024  ble cr6, 0x82ddbe14
	if !ctx.cr[6].gt {
	pc = 0x82DDBE14; continue 'dispatch;
	}
	// 82DDBDF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDBDF8: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DDBDFC: 7CA95630  sraw r9, r5, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[5].s32 >> tmp.u32) as i64;
	// 82DDBE00: 7D2B41AE  stbx r9, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u8) };
	// 82DDBE04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDBE08: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDBE0C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DDBE10: 4198FFE8  blt cr6, 0x82ddbdf8
	if ctx.cr[6].lt {
	pc = 0x82DDBDF8; continue 'dispatch;
	}
	// 82DDBE14: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DDBE18: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDBE1C: 392B0CA8  addi r9, r11, 0xca8
	ctx.r[9].s64 = ctx.r[11].s64 + 3240;
	// 82DDBE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDBE24: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DDBE28: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDBE2C: 40990020  ble cr6, 0x82ddbe4c
	if !ctx.cr[6].gt {
	pc = 0x82DDBE4C; continue 'dispatch;
	}
	// 82DDBE30: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DDBE34: 7D485A78  xor r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82DDBE38: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDBE3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDBE40: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DDBE44: 7D28E1AE  stbx r9, r8, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[28].u32), ctx.r[9].u8) };
	// 82DDBE48: 4198FFE8  blt cr6, 0x82ddbe30
	if ctx.cr[6].lt {
	pc = 0x82DDBE30; continue 'dispatch;
	}
	// 82DDBE4C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDBE50: 483CC364  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBE58 size=56
    let mut pc: u32 = 0x82DDBE58;
    'dispatch: loop {
        match pc {
            0x82DDBE58 => {
    //   block [0x82DDBE58..0x82DDBE90)
	// 82DDBE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDBE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBE64: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DDBE68: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DDBE6C: 4BFFFAA5  bl 0x82ddb910
	ctx.lr = 0x82DDBE70;
	sub_82DDB910(ctx, base);
	// 82DDBE70: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DDBE74: 4182000C  beq 0x82ddbe80
	if ctx.cr[0].eq {
	pc = 0x82DDBE80; continue 'dispatch;
	}
	// 82DDBE78: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82DDBE7C: 4BFFFE65  bl 0x82ddbce0
	ctx.lr = 0x82DDBE80;
	sub_82DDBCE0(ctx, base);
	// 82DDBE80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDBE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDBE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDBE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBE90 size=104
    let mut pc: u32 = 0x82DDBE90;
    'dispatch: loop {
        match pc {
            0x82DDBE90 => {
    //   block [0x82DDBE90..0x82DDBEF8)
	// 82DDBE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDBE98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBE9C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DDBEA0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82DDBEA4: 409A0044  bne cr6, 0x82ddbee8
	if !ctx.cr[6].eq {
	pc = 0x82DDBEE8; continue 'dispatch;
	}
	// 82DDBEA8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDBEAC: 4BFFFACD  bl 0x82ddb978
	ctx.lr = 0x82DDBEB0;
	sub_82DDB978(ctx, base);
	// 82DDBEB0: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82DDBEB4: 41980018  blt cr6, 0x82ddbecc
	if ctx.cr[6].lt {
	pc = 0x82DDBECC; continue 'dispatch;
	}
	// 82DDBEB8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBEBC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDBEC0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DDBEC4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBEC8: 48000020  b 0x82ddbee8
	pc = 0x82DDBEE8; continue 'dispatch;
	// 82DDBECC: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDBED0: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBED4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDBED8: 392BFFFC  addi r9, r11, -4
	ctx.r[9].s64 = ctx.r[11].s64 + -4;
	// 82DDBEDC: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DDBEE0: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 82DDBEE4: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDBEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDBEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDBEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDBEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DDBEF8 size=64
    let mut pc: u32 = 0x82DDBEF8;
    'dispatch: loop {
        match pc {
            0x82DDBEF8 => {
    //   block [0x82DDBEF8..0x82DDBF38)
	// 82DDBEF8: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 82DDBEFC: 3D404E80  lis r10, 0x4e80
	ctx.r[10].s64 = 1317011456;
	// 82DDBF00: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82DDBF04: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDBF08: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDBF0C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDBF10: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82DDBF14: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DDBF18: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82DDBF1C: 55690050  rlwinm r9, r11, 0, 1, 8
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DDBF20: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DDBF24: 40990020  ble cr6, 0x82ddbf44
	if !ctx.cr[6].gt {
		sub_82DDBF44(ctx, base);
		return;
	}
	// 82DDBF28: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDBF2C: 4182000C  beq 0x82ddbf38
	if ctx.cr[0].eq {
		sub_82DDBF38(ctx, base);
		return;
	}
	// 82DDBF30: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82DDBF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBF38 size=12
    let mut pc: u32 = 0x82DDBF38;
    'dispatch: loop {
        match pc {
            0x82DDBF38 => {
    //   block [0x82DDBF38..0x82DDBF44)
	// 82DDBF38: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 82DDBF3C: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 82DDBF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBF44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBF44 size=16
    let mut pc: u32 = 0x82DDBF44;
    'dispatch: loop {
        match pc {
            0x82DDBF44 => {
    //   block [0x82DDBF44..0x82DDBF54)
	// 82DDBF44: FC00001E  fctiwz f0, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDBF48: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82DDBF4C: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82DDBF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DDBF58 size=28
    let mut pc: u32 = 0x82DDBF58;
    'dispatch: loop {
        match pc {
            0x82DDBF58 => {
    //   block [0x82DDBF58..0x82DDBF74)
	// 82DDBF58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DDBF5C: C00B0A98  lfs f0, 0xa98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDBF60: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82DDBF64: 41980010  blt cr6, 0x82ddbf74
	if ctx.cr[6].lt {
		sub_82DDBF74(ctx, base);
		return;
	}
	// 82DDBF68: 3C6007FF  lis r3, 0x7ff
	ctx.r[3].s64 = 134152192;
	// 82DDBF6C: 6063FFFE  ori r3, r3, 0xfffe
	ctx.r[3].u64 = ctx.r[3].u64 | 65534;
	// 82DDBF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBF74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DDBF74 size=24
    let mut pc: u32 = 0x82DDBF74;
    'dispatch: loop {
        match pc {
            0x82DDBF74 => {
    //   block [0x82DDBF74..0x82DDBF8C)
	// 82DDBF74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DDBF78: C00BDB98  lfs f0, -0x2468(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDBF7C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82DDBF80: 4199000C  bgt cr6, 0x82ddbf8c
	if ctx.cr[6].gt {
		sub_82DDBF8C(ctx, base);
		return;
	}
	// 82DDBF84: 3C600800  lis r3, 0x800
	ctx.r[3].s64 = 134217728;
	// 82DDBF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBF8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DDBF8C size=104
    let mut pc: u32 = 0x82DDBF8C;
    'dispatch: loop {
        match pc {
            0x82DDBF8C => {
    //   block [0x82DDBF8C..0x82DDBFF4)
	// 82DDBF8C: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DDBF90: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82DDBF94: 7D6BBE70  srawi r11, r11, 0x17
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 23) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 23) as i64;
	// 82DDBF98: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DDBF9C: 216B0081  subfic r11, r11, 0x81
	ctx.xer.ca = ctx.r[11].u32 <= 129 as u32;
	ctx.r[11].s64 = (129 as i64) - ctx.r[11].s64;
	// 82DDBFA0: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82DDBFA4: 40990008  ble cr6, 0x82ddbfac
	if !ctx.cr[6].gt {
	pc = 0x82DDBFAC; continue 'dispatch;
	}
	// 82DDBFA8: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82DDBFAC: 394B0019  addi r10, r11, 0x19
	ctx.r[10].s64 = ctx.r[11].s64 + 25;
	// 82DDBFB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDBFB4: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 82DDBFB8: 556AE006  slwi r10, r11, 0x1c
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(28);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDBFBC: 7D2B4036  sld r11, r9, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[9].u64) << ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 82DDBFC0: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82DDBFC4: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDBFC8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDBFCC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDBFD0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82DDBFD4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDBFD8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82DDBFDC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82DDBFE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDBFE4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82DDBFE8: 556B013E  clrlwi r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 82DDBFEC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDBFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDBFF8 size=1248
    let mut pc: u32 = 0x82DDBFF8;
    'dispatch: loop {
        match pc {
            0x82DDBFF8 => {
    //   block [0x82DDBFF8..0x82DDC4D8)
	// 82DDBFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDC004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDC008: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DDC00C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC010: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDC014: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DDC018: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC01C: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 82DDC020: 41990048  bgt cr6, 0x82ddc068
	if ctx.cr[6].gt {
	pc = 0x82DDC068; continue 'dispatch;
	}
	// 82DDC024: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDC028: 409A000C  bne cr6, 0x82ddc034
	if !ctx.cr[6].eq {
	pc = 0x82DDC034; continue 'dispatch;
	}
	// 82DDC02C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDC030: 4800048C  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC034: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82DDC038: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82DDC03C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DDC040: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DDC044: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 82DDC048: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82DDC04C: C80AC4F0  lfd f0, -0x3b10(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-15120 as u32) ) };
	// 82DDC050: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDC054: C9ABE3A0  lfd f13, -0x1c60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 82DDC058: FC0C683C  fnmsub f0, f12, f0, f13
	ctx.f[0].f64 = -(ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64);
	// 82DDC05C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDC060: 4098045C  bge cr6, 0x82ddc4bc
	if !ctx.cr[6].lt {
	pc = 0x82DDC4BC; continue 'dispatch;
	}
	// 82DDC064: 4BFFFFC8  b 0x82ddc02c
	pc = 0x82DDC02C; continue 'dispatch;
	// 82DDC068: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DDC06C: 41980194  blt cr6, 0x82ddc200
	if ctx.cr[6].lt {
	pc = 0x82DDC200; continue 'dispatch;
	}
	// 82DDC070: 419A035C  beq cr6, 0x82ddc3cc
	if ctx.cr[6].eq {
	pc = 0x82DDC3CC; continue 'dispatch;
	}
	// 82DDC074: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 82DDC078: 419802EC  blt cr6, 0x82ddc364
	if ctx.cr[6].lt {
	pc = 0x82DDC364; continue 'dispatch;
	}
	// 82DDC07C: 419A0230  beq cr6, 0x82ddc2ac
	if ctx.cr[6].eq {
	pc = 0x82DDC2AC; continue 'dispatch;
	}
	// 82DDC080: 2B060005  cmplwi cr6, r6, 5
	ctx.cr[6].compare_u32(ctx.r[6].u32, 5 as u32, &mut ctx.xer);
	// 82DDC084: 4198015C  blt cr6, 0x82ddc1e0
	if ctx.cr[6].lt {
	pc = 0x82DDC1E0; continue 'dispatch;
	}
	// 82DDC088: 419A010C  beq cr6, 0x82ddc194
	if ctx.cr[6].eq {
	pc = 0x82DDC194; continue 'dispatch;
	}
	// 82DDC08C: 2B060007  cmplwi cr6, r6, 7
	ctx.cr[6].compare_u32(ctx.r[6].u32, 7 as u32, &mut ctx.xer);
	// 82DDC090: 409AFF9C  bne cr6, 0x82ddc02c
	if !ctx.cr[6].eq {
	pc = 0x82DDC02C; continue 'dispatch;
	}
	// 82DDC094: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DDC098: 2F1F0020  cmpwi cr6, r31, 0x20
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32, &mut ctx.xer);
	// 82DDC09C: 409A000C  bne cr6, 0x82ddc0a8
	if !ctx.cr[6].eq {
	pc = 0x82DDC0A8; continue 'dispatch;
	}
	// 82DDC0A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDC0A4: 48000418  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC0A8: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 82DDC0AC: 409A0068  bne cr6, 0x82ddc114
	if !ctx.cr[6].eq {
	pc = 0x82DDC114; continue 'dispatch;
	}
	// 82DDC0B0: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDC0B4: 57CB2834  slwi r11, r30, 5
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC0B8: C80A39D8  lfd f0, 0x39d8(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(14808 as u32) ) };
	// 82DDC0BC: FC1F0032  fmul f0, f31, f0
	ctx.f[0].f64 = ctx.f[31].f64 * ctx.f[0].f64;
	// 82DDC0C0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC0C4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DDC0C8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDC0CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDC0D0: 556A0047  rlwinm. r10, r11, 0, 1, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDC0D4: 41820028  beq 0x82ddc0fc
	if ctx.cr[0].eq {
	pc = 0x82DDC0FC; continue 'dispatch;
	}
	// 82DDC0D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDC0DC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC0E0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDC0E4: 40980010  bge cr6, 0x82ddc0f4
	if !ctx.cr[6].lt {
	pc = 0x82DDC0F4; continue 'dispatch;
	}
	// 82DDC0E8: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 82DDC0EC: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 82DDC0F0: 480003CC  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC0F4: 38607FFF  li r3, 0x7fff
	ctx.r[3].s64 = 32767;
	// 82DDC0F8: 480003C4  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC0FC: 7D6B6E71  srawi. r11, r11, 0xd
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 13) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 13) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDC100: 4080000C  bge 0x82ddc10c
	if !ctx.cr[0].lt {
	pc = 0x82DDC10C; continue 'dispatch;
	}
	// 82DDC104: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 82DDC108: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 82DDC10C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DDC110: 480003AC  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC114: 2F1F000A  cmpwi cr6, r31, 0xa
	ctx.cr[6].compare_i32(ctx.r[31].s32, 10, &mut ctx.xer);
	// 82DDC118: 4198FF14  blt cr6, 0x82ddc02c
	if ctx.cr[6].lt {
	pc = 0x82DDC02C; continue 'dispatch;
	}
	// 82DDC11C: 3880FF84  li r4, -0x7c
	ctx.r[4].s64 = -124;
	// 82DDC120: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DDC124: 483D16C5  bl 0x831ad7e8
	ctx.lr = 0x82DDC128;
	sub_831AD7E8(ctx, base);
	// 82DDC128: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DDC12C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DDC130: 217F0012  subfic r11, r31, 0x12
	ctx.xer.ca = ctx.r[31].u32 <= 18 as u32;
	ctx.r[11].s64 = (18 as i64) - ctx.r[31].s64;
	// 82DDC134: 7FCB5830  slw r11, r30, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[30].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC138: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDC13C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDC140: 556A004B  rlwinm. r10, r11, 0, 1, 5
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDC144: 41820034  beq 0x82ddc178
	if ctx.cr[0].eq {
	pc = 0x82DDC178; continue 'dispatch;
	}
	// 82DDC148: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDC14C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC150: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDC154: 40980010  bge cr6, 0x82ddc164
	if !ctx.cr[6].lt {
	pc = 0x82DDC164; continue 'dispatch;
	}
	// 82DDC158: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDC15C: 7D6BF830  slw r11, r11, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC160: 48000010  b 0x82ddc170
	pc = 0x82DDC170; continue 'dispatch;
	// 82DDC164: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82DDC168: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDC16C: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC170: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82DDC174: 48000348  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC178: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDC17C: 213F001A  subfic r9, r31, 0x1a
	ctx.xer.ca = ctx.r[31].u32 <= 26 as u32;
	ctx.r[9].s64 = (26 as i64) - ctx.r[31].s64;
	// 82DDC180: 7D4AF830  slw r10, r10, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC184: 7D6B4E30  sraw r11, r11, r9
	tmp.u32 = ctx.r[9].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 82DDC188: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DDC18C: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82DDC190: 4800032C  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC194: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82DDC198: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDC19C: C00B8AD8  lfs f0, -0x7528(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29992 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC1A0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82DDC1A4: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DDC1A8: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DDC1AC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDC1B0: C9ABD760  lfd f13, -0x28a0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10400 as u32) ) };
	// 82DDC1B4: 40990018  ble cr6, 0x82ddc1cc
	if !ctx.cr[6].gt {
	pc = 0x82DDC1CC; continue 'dispatch;
	}
	// 82DDC1B8: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 82DDC1BC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC1C0: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DDC1C4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDC1C8: 480002F4  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC1CC: FC006828  fsub f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82DDC1D0: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC1D4: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DDC1D8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDC1DC: 480002E0  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC1E0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DDC1E4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDC1E8: C80B39F0  lfd f0, 0x39f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(14832 as u32) ) };
	// 82DDC1EC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82DDC1F0: 41990090  bgt cr6, 0x82ddc280
	if ctx.cr[6].gt {
	pc = 0x82DDC280; continue 'dispatch;
	}
	// 82DDC1F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82DDC1F8: C00BD6E8  lfs f0, -0x2918(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10520 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC1FC: EFFF0032  fmuls f31, f31, f0
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DDC200: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82DDC204: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDC208: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC20C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82DDC210: 7D4BF830  slw r11, r10, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC214: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82DDC218: C809C4F0  lfd f0, -0x3b10(r9)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-15120 as u32) ) };
	// 82DDC21C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDC220: 7D0B07B4  extsw r11, r8
	ctx.r[11].s64 = ctx.r[8].s32 as i64;
	// 82DDC224: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC228: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DDC22C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC230: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC234: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82DDC238: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82DDC23C: FDA06018  frsp f13, f12
	ctx.f[13].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82DDC240: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC244: EC2D07FA  fmadds f1, f13, f31, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 82DDC248: 4BFFFCB1  bl 0x82ddbef8
	ctx.lr = 0x82DDC24C;
	sub_82DDBEF8(ctx, base);
	// 82DDC24C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82DDC250: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDC254: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC258: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC25C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDC260: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC264: C18A08A4  lfs f12, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DDC268: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DDC26C: 4099FDC0  ble cr6, 0x82ddc02c
	if !ctx.cr[6].gt {
	pc = 0x82DDC02C; continue 'dispatch;
	}
	// 82DDC270: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDC274: 4198023C  blt cr6, 0x82ddc4b0
	if ctx.cr[6].lt {
	pc = 0x82DDC4B0; continue 'dispatch;
	}
	// 82DDC278: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82DDC27C: 48000240  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC280: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82DDC284: C84BD6E0  lfd f2, -0x2920(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10528 as u32) ) };
	// 82DDC288: 483CF221  bl 0x831ab4a8
	ctx.lr = 0x82DDC28C;
	sub_831AB4A8(ctx, base);
	// 82DDC28C: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DDC290: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDC294: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDC298: C80B39E8  lfd f0, 0x39e8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(14824 as u32) ) };
	// 82DDC29C: C9AA39D0  lfd f13, 0x39d0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(14800 as u32) ) };
	// 82DDC2A0: FC0C6838  fmsub f0, f12, f0, f13
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64;
	// 82DDC2A4: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC2A8: 4BFFFF58  b 0x82ddc200
	pc = 0x82DDC200; continue 'dispatch;
	// 82DDC2AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDC2B0: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82DDC2B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDC2B8: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC2BC: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC2C0: 7FCA4670  srawi r10, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 82DDC2C4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDC2C8: 41980050  blt cr6, 0x82ddc318
	if ctx.cr[6].lt {
	pc = 0x82DDC318; continue 'dispatch;
	}
	// 82DDC2CC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82DDC2D0: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DDC2D4: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 82DDC2D8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DDC2DC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82DDC2E0: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DDC2E4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DDC2E8: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DDC2EC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDC2F0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DDC2F4: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC2F8: FC006818  frsp f0, f13
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DDC2FC: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82DDC300: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DDC304: 409801B8  bge cr6, 0x82ddc4bc
	if !ctx.cr[6].lt {
	pc = 0x82DDC4BC; continue 'dispatch;
	}
	// 82DDC308: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC30C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DDC310: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDC314: 480001A8  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC318: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 82DDC31C: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DDC320: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 82DDC324: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DDC328: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82DDC32C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DDC330: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC334: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC338: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDC33C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DDC340: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC344: FC006818  frsp f0, f13
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DDC348: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DDC34C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DDC350: 4099016C  ble cr6, 0x82ddc4bc
	if !ctx.cr[6].gt {
	pc = 0x82DDC4BC; continue 'dispatch;
	}
	// 82DDC354: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC358: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82DDC35C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DDC360: 4800015C  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC364: 7FCB4670  srawi r11, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 82DDC368: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDC36C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DDC370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDC374: 7D6807B4  extsw r8, r11
	ctx.r[8].s64 = ctx.r[11].s32 as i64;
	// 82DDC378: 7D2BF830  slw r11, r9, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC37C: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 82DDC380: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DDC384: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82DDC388: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC38C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDC390: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC394: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82DDC398: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDC39C: 4099FC90  ble cr6, 0x82ddc02c
	if !ctx.cr[6].gt {
	pc = 0x82DDC02C; continue 'dispatch;
	}
	// 82DDC3A0: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82DDC3A4: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC3A8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC3AC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DDC3B0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DDC3B4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDC3B8: 40980104  bge cr6, 0x82ddc4bc
	if !ctx.cr[6].lt {
	pc = 0x82DDC4BC; continue 'dispatch;
	}
	// 82DDC3BC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC3C0: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82DDC3C4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DDC3C8: 480000F4  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC3CC: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82DDC3D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDC3D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDC3D8: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DDC3DC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82DDC3E0: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC3E4: 57CB083E  rotlwi r11, r30, 1
	ctx.r[11].u64 = ((ctx.r[30].u32).rotate_left(1)) as u64;
	// 82DDC3E8: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDC3EC: 7D5E1BD6  divw r10, r30, r3
	ctx.r[10].s32 = ctx.r[30].s32 / ctx.r[3].s32;
	// 82DDC3F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDC3F4: 7D4A4670  srawi r10, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 82DDC3F8: 7C6B5878  andc r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & !ctx.r[11].u64;
	// 82DDC3FC: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82DDC400: 0CC30000  twi 6, r3, 0
	// 82DDC404: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82DDC408: 0CABFFFF  twi 5, r11, -1
	// 82DDC40C: 41980050  blt cr6, 0x82ddc45c
	if ctx.cr[6].lt {
	pc = 0x82DDC45C; continue 'dispatch;
	}
	// 82DDC410: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82DDC414: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DDC418: C1A908A8  lfs f13, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DDC41C: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC420: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDC424: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC428: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82DDC42C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDC430: 4098008C  bge cr6, 0x82ddc4bc
	if !ctx.cr[6].lt {
	pc = 0x82DDC4BC; continue 'dispatch;
	}
	// 82DDC434: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82DDC438: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC43C: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC440: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DDC444: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DDC448: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DDC44C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC450: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82DDC454: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DDC458: 48000064  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC45C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82DDC460: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82DDC464: C1A99534  lfs f13, -0x6acc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DDC468: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC46C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DDC470: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DDC474: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DDC478: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDC47C: 4199000C  bgt cr6, 0x82ddc488
	if ctx.cr[6].gt {
	pc = 0x82DDC488; continue 'dispatch;
	}
	// 82DDC480: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 82DDC484: 48000038  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC488: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82DDC48C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DDC490: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDC494: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DDC498: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DDC49C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DDC4A0: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC4A4: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82DDC4A8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DDC4AC: 48000010  b 0x82ddc4bc
	pc = 0x82DDC4BC; continue 'dispatch;
	// 82DDC4B0: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DDC4B4: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82DDC4B8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DDC4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDC4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC4C8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DDC4CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDC4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDC4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC4D8 size=24
    let mut pc: u32 = 0x82DDC4D8;
    'dispatch: loop {
        match pc {
            0x82DDC4D8 => {
    //   block [0x82DDC4D8..0x82DDC4F0)
	// 82DDC4D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC4DC: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82DDC4E0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DDC4E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DDC4E8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DDC4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC4F0 size=44
    let mut pc: u32 = 0x82DDC4F0;
    'dispatch: loop {
        match pc {
            0x82DDC4F0 => {
    //   block [0x82DDC4F0..0x82DDC51C)
	// 82DDC4F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC4F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDC4F8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC4FC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDC500: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC504: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC508: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDC50C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC510: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC518: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC520 size=32
    let mut pc: u32 = 0x82DDC520;
    'dispatch: loop {
        match pc {
            0x82DDC520 => {
    //   block [0x82DDC520..0x82DDC540)
	// 82DDC520: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDC528: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDC52C: 419A0008  beq cr6, 0x82ddc534
	if ctx.cr[6].eq {
	pc = 0x82DDC534; continue 'dispatch;
	}
	// 82DDC530: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DDC534: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DDC538: 90640004  stw r3, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DDC53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC540 size=32
    let mut pc: u32 = 0x82DDC540;
    'dispatch: loop {
        match pc {
            0x82DDC540 => {
    //   block [0x82DDC540..0x82DDC560)
	// 82DDC540: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDC548: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DDC54C: 419A0008  beq cr6, 0x82ddc554
	if ctx.cr[6].eq {
	pc = 0x82DDC554; continue 'dispatch;
	}
	// 82DDC550: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DDC554: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DDC558: 90640008  stw r3, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DDC55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC560 size=28
    let mut pc: u32 = 0x82DDC560;
    'dispatch: loop {
        match pc {
            0x82DDC560 => {
    //   block [0x82DDC560..0x82DDC57C)
	// 82DDC560: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC564: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC568: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDC56C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC570: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC574: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDC578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC580 size=52
    let mut pc: u32 = 0x82DDC580;
    'dispatch: loop {
        match pc {
            0x82DDC580 => {
    //   block [0x82DDC580..0x82DDC5B4)
	// 82DDC580: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDC584: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC588: 392A1520  addi r9, r10, 0x1520
	ctx.r[9].s64 = ctx.r[10].s64 + 5408;
	// 82DDC58C: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82DDC590: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDC594: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDC598: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDC59C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DDC5A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDC5A4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDC5A8: 90630010  stw r3, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82DDC5AC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DDC5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC5B8 size=16
    let mut pc: u32 = 0x82DDC5B8;
    'dispatch: loop {
        match pc {
            0x82DDC5B8 => {
    //   block [0x82DDC5B8..0x82DDC5C8)
	// 82DDC5B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC5BC: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82DDC5C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDC5C4: 4800000C  b 0x82ddc5d0
	sub_82DDC5C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC5C8 size=20
    let mut pc: u32 = 0x82DDC5C8;
    'dispatch: loop {
        match pc {
            0x82DDC5C8 => {
    //   block [0x82DDC5C8..0x82DDC5DC)
	// 82DDC5C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC5CC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DDC5D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DDC5D4: 409AFFF4  bne cr6, 0x82ddc5c8
	if !ctx.cr[6].eq {
	pc = 0x82DDC5C8; continue 'dispatch;
	}
	// 82DDC5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC5E0 size=36
    let mut pc: u32 = 0x82DDC5E0;
    'dispatch: loop {
        match pc {
            0x82DDC5E0 => {
    //   block [0x82DDC5E0..0x82DDC604)
	// 82DDC5E0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDC5E4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82DDC5E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDC5EC: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDC5F0: 419A0008  beq cr6, 0x82ddc5f8
	if ctx.cr[6].eq {
	pc = 0x82DDC5F8; continue 'dispatch;
	}
	// 82DDC5F4: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DDC5F8: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDC5FC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DDC600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC608 size=32
    let mut pc: u32 = 0x82DDC608;
    'dispatch: loop {
        match pc {
            0x82DDC608 => {
    //   block [0x82DDC608..0x82DDC628)
	// 82DDC608: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC60C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDC610: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DDC614: 419A0008  beq cr6, 0x82ddc61c
	if ctx.cr[6].eq {
	pc = 0x82DDC61C; continue 'dispatch;
	}
	// 82DDC618: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DDC61C: 90640004  stw r3, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DDC620: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DDC624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC628 size=84
    let mut pc: u32 = 0x82DDC628;
    'dispatch: loop {
        match pc {
            0x82DDC628 => {
    //   block [0x82DDC628..0x82DDC67C)
	// 82DDC628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC62C: 483CBB41  bl 0x831a816c
	ctx.lr = 0x82DDC630;
	sub_831A8130(ctx, base);
	// 82DDC630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC638: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82DDC63C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC640: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DDC644: 419A0028  beq cr6, 0x82ddc66c
	if ctx.cr[6].eq {
	pc = 0x82DDC66C; continue 'dispatch;
	}
	// 82DDC648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC64C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDC650: 83A30008  lwz r29, 8(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC654: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC658: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC65C: 4E800421  bctrl
	ctx.lr = 0x82DDC660;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC660: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDC664: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DDC668: 409AFFE0  bne cr6, 0x82ddc648
	if !ctx.cr[6].eq {
	pc = 0x82DDC648; continue 'dispatch;
	}
	// 82DDC66C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DDC670: 93FF0010  stw r31, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82DDC674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDC678: 483CBB44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC680 size=212
    let mut pc: u32 = 0x82DDC680;
    'dispatch: loop {
        match pc {
            0x82DDC680 => {
    //   block [0x82DDC680..0x82DDC754)
	// 82DDC680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC684: 483CBAC1  bl 0x831a8144
	ctx.lr = 0x82DDC688;
	sub_831A8130(ctx, base);
	// 82DDC688: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC68C: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82DDC690: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82DDC694: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDC698: 82D50008  lwz r22, 8(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC69C: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC6A0: 1D4A0003  mulli r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 * 3;
	// 82DDC6A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DDC6A8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC6AC: 4198FFF4  blt cr6, 0x82ddc6a0
	if ctx.cr[6].lt {
	pc = 0x82DDC6A0; continue 'dispatch;
	}
	// 82DDC6B0: 3A800003  li r20, 3
	ctx.r[20].s64 = 3;
	// 82DDC6B4: 7FEAA397  divwu. r31, r10, r20
	ctx.r[31].u32 = ctx.r[10].u32 / ctx.r[20].u32;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDC6B8: 41820094  beq 0x82ddc74c
	if ctx.cr[0].eq {
	pc = 0x82DDC74C; continue 'dispatch;
	}
	// 82DDC6BC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC6C0: 40980084  bge cr6, 0x82ddc744
	if !ctx.cr[6].lt {
	pc = 0x82DDC744; continue 'dispatch;
	}
	// 82DDC6C4: 57F7103A  slwi r23, r31, 2
	ctx.r[23].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82DDC6C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDC6CC: 7FB7B214  add r29, r23, r22
	ctx.r[29].u64 = ctx.r[23].u64 + ctx.r[22].u64;
	// 82DDC6D0: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 82DDC6D4: 831D0000  lwz r24, 0(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC6D8: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82DDC6DC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDC6E0: 4198003C  blt cr6, 0x82ddc71c
	if ctx.cr[6].lt {
	pc = 0x82DDC71C; continue 'dispatch;
	}
	// 82DDC6E4: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82DDC6E8: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82DDC6EC: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC6F0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82DDC6F4: 7E6903A6  mtctr r19
	ctx.ctr.u64 = ctx.r[19].u64;
	// 82DDC6F8: 4E800421  bctrl
	ctx.lr = 0x82DDC6FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC6FC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DDC700: 4081001C  ble 0x82ddc71c
	if !ctx.cr[0].gt {
	pc = 0x82DDC71C; continue 'dispatch;
	}
	// 82DDC704: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC708: 7F7FD851  subf. r27, r31, r27
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[31].s64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DDC70C: 7F97E050  subf r28, r23, r28
	ctx.r[28].s64 = ctx.r[28].s64 - ctx.r[23].s64;
	// 82DDC710: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDC714: 7F37C850  subf r25, r23, r25
	ctx.r[25].s64 = ctx.r[25].s64 - ctx.r[23].s64;
	// 82DDC718: 4080FFD4  bge 0x82ddc6ec
	if !ctx.cr[0].lt {
	pc = 0x82DDC6EC; continue 'dispatch;
	}
	// 82DDC71C: 7D7BFA14  add r11, r27, r31
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 82DDC720: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDC724: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC728: 7D5EFA14  add r10, r30, r31
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82DDC72C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DDC730: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82DDC734: 7F0BB12E  stwx r24, r11, r22
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[22].u32), ctx.r[24].u32) };
	// 82DDC738: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC73C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC740: 4198FF94  blt cr6, 0x82ddc6d4
	if ctx.cr[6].lt {
	pc = 0x82DDC6D4; continue 'dispatch;
	}
	// 82DDC744: 7FFFA397  divwu. r31, r31, r20
	ctx.r[31].u32 = ctx.r[31].u32 / ctx.r[20].u32;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDC748: 4082FF74  bne 0x82ddc6bc
	if !ctx.cr[0].eq {
	pc = 0x82DDC6BC; continue 'dispatch;
	}
	// 82DDC74C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDC750: 483CBA44  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC758 size=12
    let mut pc: u32 = 0x82DDC758;
    'dispatch: loop {
        match pc {
            0x82DDC758 => {
    //   block [0x82DDC758..0x82DDC764)
	// 82DDC758: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC75C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC760: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC764 size=36
    let mut pc: u32 = 0x82DDC764;
    'dispatch: loop {
        match pc {
            0x82DDC764 => {
    //   block [0x82DDC764..0x82DDC788)
	// 82DDC764: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDC768: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC76C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC770: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDC774: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82DDC778: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC77C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DDC780: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DDC784: 483CBD8C  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDC788 size=4
    let mut pc: u32 = 0x82DDC788;
    'dispatch: loop {
        match pc {
            0x82DDC788 => {
    //   block [0x82DDC788..0x82DDC78C)
	// 82DDC788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC790 size=136
    let mut pc: u32 = 0x82DDC790;
    'dispatch: loop {
        match pc {
            0x82DDC790 => {
    //   block [0x82DDC790..0x82DDC818)
	// 82DDC790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC79C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC7A4: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC7A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDC7AC: 419A0028  beq cr6, 0x82ddc7d4
	if ctx.cr[6].eq {
	pc = 0x82DDC7D4; continue 'dispatch;
	}
	// 82DDC7B0: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82DDC7B4: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC7B8: 7F043840  cmplw cr6, r4, r7
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DDC7BC: 419A002C  beq cr6, 0x82ddc7e8
	if ctx.cr[6].eq {
	pc = 0x82DDC7E8; continue 'dispatch;
	}
	// 82DDC7C0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC7C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDC7C8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DDC7CC: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DDC7D0: 4198FFE4  blt cr6, 0x82ddc7b4
	if ctx.cr[6].lt {
	pc = 0x82DDC7B4; continue 'dispatch;
	}
	// 82DDC7D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDC7D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDC7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC7E4: 4E800020  blr
	return;
	// 82DDC7E8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DDC7EC: 40980024  bge cr6, 0x82ddc810
	if !ctx.cr[6].lt {
	pc = 0x82DDC810; continue 'dispatch;
	}
	// 82DDC7F0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DDC7F4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DDC7F8: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDC7FC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DDC800: 7C694214  add r3, r9, r8
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DDC804: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DDC808: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DDC80C: 483CBD05  bl 0x831a8510
	ctx.lr = 0x82DDC810;
	sub_831A8510(ctx, base);
	// 82DDC810: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDC814: 4BFFFFC4  b 0x82ddc7d8
	pc = 0x82DDC7D8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC818 size=132
    let mut pc: u32 = 0x82DDC818;
    'dispatch: loop {
        match pc {
            0x82DDC818 => {
    //   block [0x82DDC818..0x82DDC89C)
	// 82DDC818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC81C: 483CB951  bl 0x831a816c
	ctx.lr = 0x82DDC820;
	sub_831A8130(ctx, base);
	// 82DDC820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC828: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC82C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82DDC830: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC834: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DDC838: 40990014  ble cr6, 0x82ddc84c
	if !ctx.cr[6].gt {
	pc = 0x82DDC84C; continue 'dispatch;
	}
	// 82DDC83C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDC840: 4800000C  b 0x82ddc84c
	pc = 0x82DDC84C; continue 'dispatch;
	// 82DDC844: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC848: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDC84C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC850: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC854: 4098FFF0  bge cr6, 0x82ddc844
	if !ctx.cr[6].lt {
	pc = 0x82DDC844; continue 'dispatch;
	}
	// 82DDC858: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DDC85C: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC860: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC864: 4BFC8D1D  bl 0x82da5580
	ctx.lr = 0x82DDC868;
	sub_82DA5580(ctx, base);
	// 82DDC868: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DDC86C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC870: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDC874: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DDC878: 483CBC99  bl 0x831a8510
	ctx.lr = 0x82DDC87C;
	sub_831A8510(ctx, base);
	// 82DDC87C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDC880: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC884: 4BFC8E85  bl 0x82da5708
	ctx.lr = 0x82DDC888;
	sub_82DA5708(ctx, base);
	// 82DDC888: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC88C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC890: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDC894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDC898: 483CB924  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC8A0 size=172
    let mut pc: u32 = 0x82DDC8A0;
    'dispatch: loop {
        match pc {
            0x82DDC8A0 => {
    //   block [0x82DDC8A0..0x82DDC94C)
	// 82DDC8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC8A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDC8AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDC8B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC8B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC8BC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC8C0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC8C4: 40990008  ble cr6, 0x82ddc8cc
	if !ctx.cr[6].gt {
	pc = 0x82DDC8CC; continue 'dispatch;
	}
	// 82DDC8C8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DDC8CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC8D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC8D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDC8D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDC8DC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DDC8E0: 40990010  ble cr6, 0x82ddc8f0
	if !ctx.cr[6].gt {
	pc = 0x82DDC8F0; continue 'dispatch;
	}
	// 82DDC8E4: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82DDC8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDC8EC: 4BFFFF2D  bl 0x82ddc818
	ctx.lr = 0x82DDC8F0;
	sub_82DDC818(ctx, base);
	// 82DDC8F0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDC8F4: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC8F8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC8FC: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 82DDC900: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDC904: 3569FFFF  addic. r11, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDC908: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC90C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DDC910: 4182001C  beq 0x82ddc92c
	if ctx.cr[0].eq {
	pc = 0x82DDC92C; continue 'dispatch;
	}
	// 82DDC914: 810AFFFC  lwz r8, -4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DDC918: 392AFFFC  addi r9, r10, -4
	ctx.r[9].s64 = ctx.r[10].s64 + -4;
	// 82DDC91C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDC920: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DDC924: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82DDC928: 4082FFEC  bne 0x82ddc914
	if !ctx.cr[0].eq {
	pc = 0x82DDC914; continue 'dispatch;
	}
	// 82DDC92C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC930: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDC934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDC938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC940: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDC944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDC948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC950 size=384
    let mut pc: u32 = 0x82DDC950;
    'dispatch: loop {
        match pc {
            0x82DDC950 => {
    //   block [0x82DDC950..0x82DDCAD0)
	// 82DDC950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC954: 483CB7F5  bl 0x831a8148
	ctx.lr = 0x82DDC958;
	sub_831A8130(ctx, base);
	// 82DDC958: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC95C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDC960: 82DE0000  lwz r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC964: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC968: 56CB083C  slwi r11, r22, 1
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC96C: 56C41838  slwi r4, r22, 3
	ctx.r[4].u32 = ctx.r[22].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DDC970: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDC974: 4BFC8C0D  bl 0x82da5580
	ctx.lr = 0x82DDC978;
	sub_82DA5580(ctx, base);
	// 82DDC978: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC97C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDC980: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DDC984: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DDC988: 483CB859  bl 0x831a81e0
	ctx.lr = 0x82DDC98C;
	sub_831A81E0(ctx, base);
	// 82DDC98C: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82DDC990: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82DDC994: 40990130  ble cr6, 0x82ddcac4
	if !ctx.cr[6].gt {
	pc = 0x82DDCAC4; continue 'dispatch;
	}
	// 82DDC998: 56CB103A  slwi r11, r22, 2
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC99C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DDC9A0: 7EABBA14  add r21, r11, r23
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82DDC9A4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC9A8: 7F3A582E  lwzx r25, r26, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDC9AC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82DDC9B0: 419A0100  beq cr6, 0x82ddcab0
	if ctx.cr[6].eq {
	pc = 0x82DDCAB0; continue 'dispatch;
	}
	// 82DDC9B4: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC9B8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDC9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDC9C0: 4BFC8BC1  bl 0x82da5580
	ctx.lr = 0x82DDC9C4;
	sub_82DA5580(ctx, base);
	// 82DDC9C4: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDC9C8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDC9CC: 41820018  beq 0x82ddc9e4
	if ctx.cr[0].eq {
	pc = 0x82DDC9E4; continue 'dispatch;
	}
	// 82DDC9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDC9D4: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC9D8: 4BFE2481  bl 0x82dbee58
	ctx.lr = 0x82DDC9DC;
	sub_82DBEE58(ctx, base);
	// 82DDC9DC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DDC9E0: 48000008  b 0x82ddc9e8
	pc = 0x82DDC9E8; continue 'dispatch;
	// 82DDC9E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC9E8: 7D7AB92E  stwx r11, r26, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[23].u32), ctx.r[11].u32) };
	// 82DDC9EC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDC9F0: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC9F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDC9F8: 4BFC8B89  bl 0x82da5580
	ctx.lr = 0x82DDC9FC;
	sub_82DA5580(ctx, base);
	// 82DDC9FC: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDCA00: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDCA04: 41820018  beq 0x82ddca1c
	if ctx.cr[0].eq {
	pc = 0x82DDCA1C; continue 'dispatch;
	}
	// 82DDCA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDCA0C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCA10: 4BFE2449  bl 0x82dbee58
	ctx.lr = 0x82DDCA14;
	sub_82DBEE58(ctx, base);
	// 82DDCA14: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82DDCA18: 48000008  b 0x82ddca20
	pc = 0x82DDCA20; continue 'dispatch;
	// 82DDCA1C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDCA20: 93750000  stw r27, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDCA24: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCA28: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDCA2C: 7F1AB82E  lwzx r24, r26, r23
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCA30: 419A0080  beq cr6, 0x82ddcab0
	if ctx.cr[6].eq {
	pc = 0x82DDCAB0; continue 'dispatch;
	}
	// 82DDCA34: 57FC103A  slwi r28, r31, 2
	ctx.r[28].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DDCA38: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCA3C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82DDCA40: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 82DDCA44: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCA48: 40980010  bge cr6, 0x82ddca58
	if !ctx.cr[6].lt {
	pc = 0x82DDCA58; continue 'dispatch;
	}
	// 82DDCA4C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCA50: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DDCA54: 48000010  b 0x82ddca64
	pc = 0x82DDCA64; continue 'dispatch;
	// 82DDCA58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDCA5C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DDCA60: 4BFFFDB9  bl 0x82ddc818
	ctx.lr = 0x82DDCA64;
	sub_82DDC818(ctx, base);
	// 82DDCA64: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCA68: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCA6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDCA70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCA74: 4E800421  bctrl
	ctx.lr = 0x82DDCA78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCA78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCA7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDCA80: 7C6B5838  and r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[11].u64;
	// 82DDCA84: 7F0BA040  cmplw cr6, r11, r20
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[20].u32, &mut ctx.xer);
	// 82DDCA88: 409A0010  bne cr6, 0x82ddca98
	if !ctx.cr[6].eq {
	pc = 0x82DDCA98; continue 'dispatch;
	}
	// 82DDCA8C: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCA90: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DDCA94: 4800000C  b 0x82ddcaa0
	pc = 0x82DDCAA0; continue 'dispatch;
	// 82DDCA98: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCA9C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDCAA0: 4BFFFD79  bl 0x82ddc818
	ctx.lr = 0x82DDCAA4;
	sub_82DDC818(ctx, base);
	// 82DDCAA4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDCAA8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDCAAC: 409AFF8C  bne cr6, 0x82ddca38
	if !ctx.cr[6].eq {
	pc = 0x82DDCA38; continue 'dispatch;
	}
	// 82DDCAB0: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82DDCAB4: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82DDCAB8: 3AB50004  addi r21, r21, 4
	ctx.r[21].s64 = ctx.r[21].s64 + 4;
	// 82DDCABC: 7F14B000  cmpw cr6, r20, r22
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82DDCAC0: 4198FEE4  blt cr6, 0x82ddc9a4
	if ctx.cr[6].lt {
	pc = 0x82DDC9A4; continue 'dispatch;
	}
	// 82DDCAC4: 92FE0008  stw r23, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82DDCAC8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDCACC: 483CB6CC  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCAD0 size=192
    let mut pc: u32 = 0x82DDCAD0;
    'dispatch: loop {
        match pc {
            0x82DDCAD0 => {
    //   block [0x82DDCAD0..0x82DDCB90)
	// 82DDCAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCAD4: 483CB68D  bl 0x831a8160
	ctx.lr = 0x82DDCAD8;
	sub_831A8130(ctx, base);
	// 82DDCAD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCADC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DDCAE0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DDCAE4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDCAE8: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCAEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCAF0: 4E800421  bctrl
	ctx.lr = 0x82DDCAF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCAF4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCAF8: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCAFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDCB00: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 82DDCB04: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDCB08: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDCB0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DDCB10: 419A006C  beq cr6, 0x82ddcb7c
	if ctx.cr[6].eq {
	pc = 0x82DDCB7C; continue 'dispatch;
	}
	// 82DDCB14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCB18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDCB1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDCB20: 419A005C  beq cr6, 0x82ddcb7c
	if ctx.cr[6].eq {
	pc = 0x82DDCB7C; continue 'dispatch;
	}
	// 82DDCB24: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DDCB28: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCB2C: 40980010  bge cr6, 0x82ddcb3c
	if !ctx.cr[6].lt {
	pc = 0x82DDCB3C; continue 'dispatch;
	}
	// 82DDCB30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCB34: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DDCB38: 48000010  b 0x82ddcb48
	pc = 0x82DDCB48; continue 'dispatch;
	// 82DDCB3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDCB40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDCB44: 4BFFFCD5  bl 0x82ddc818
	ctx.lr = 0x82DDCB48;
	sub_82DDC818(ctx, base);
	// 82DDCB48: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCB4C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DDCB50: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCB54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDCB58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCB5C: 4E800421  bctrl
	ctx.lr = 0x82DDCB60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCB60: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DDCB64: 41820024  beq 0x82ddcb88
	if ctx.cr[0].eq {
	pc = 0x82DDCB88; continue 'dispatch;
	}
	// 82DDCB68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCB6C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDCB70: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DDCB74: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCB78: 4198FFB8  blt cr6, 0x82ddcb30
	if ctx.cr[6].lt {
	pc = 0x82DDCB30; continue 'dispatch;
	}
	// 82DDCB7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDCB80: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DDCB84: 483CB62C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82DDCB88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDCB8C: 4BFFFFF4  b 0x82ddcb80
	pc = 0x82DDCB80; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCB90 size=192
    let mut pc: u32 = 0x82DDCB90;
    'dispatch: loop {
        match pc {
            0x82DDCB90 => {
    //   block [0x82DDCB90..0x82DDCC50)
	// 82DDCB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCB94: 483CB5D1  bl 0x831a8164
	ctx.lr = 0x82DDCB98;
	sub_831A8130(ctx, base);
	// 82DDCB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCB9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDCBA0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDCBA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDCBA8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCBAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCBB0: 4E800421  bctrl
	ctx.lr = 0x82DDCBB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCBB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCBB8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCBBC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDCBC0: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 82DDCBC4: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DDCBC8: 7D6AE82E  lwzx r11, r10, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DDCBCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDCBD0: 409A003C  bne cr6, 0x82ddcc0c
	if !ctx.cr[6].eq {
	pc = 0x82DDCC0C; continue 'dispatch;
	}
	// 82DDCBD4: 837F0014  lwz r27, 0x14(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCBD8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDCBDC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDCBE0: 4BFC89A1  bl 0x82da5580
	ctx.lr = 0x82DDCBE4;
	sub_82DA5580(ctx, base);
	// 82DDCBE4: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDCBE8: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDCBEC: 41820014  beq 0x82ddcc00
	if ctx.cr[0].eq {
	pc = 0x82DDCC00; continue 'dispatch;
	}
	// 82DDCBF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDCBF4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCBF8: 4BFE2261  bl 0x82dbee58
	ctx.lr = 0x82DDCBFC;
	sub_82DBEE58(ctx, base);
	// 82DDCBFC: 48000008  b 0x82ddcc04
	pc = 0x82DDCC04; continue 'dispatch;
	// 82DDCC00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDCC04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCC08: 7FCBE92E  stwx r30, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[30].u32) };
	// 82DDCC0C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCC10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDCC14: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DDCC18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDCC1C: 4BFFFC85  bl 0x82ddc8a0
	ctx.lr = 0x82DDCC20;
	sub_82DDC8A0(ctx, base);
	// 82DDCC20: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DDCC24: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCC28: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCC2C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DDCC30: 4099000C  ble cr6, 0x82ddcc3c
	if !ctx.cr[6].gt {
	pc = 0x82DDCC3C; continue 'dispatch;
	}
	// 82DDCC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDCC38: 4BFFFD19  bl 0x82ddc950
	ctx.lr = 0x82DDCC3C;
	sub_82DDC950(ctx, base);
	// 82DDCC3C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCC40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDCC44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDCC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDCC4C: 483CB568  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCC50 size=228
    let mut pc: u32 = 0x82DDCC50;
    'dispatch: loop {
        match pc {
            0x82DDCC50 => {
    //   block [0x82DDCC50..0x82DDCD34)
	// 82DDCC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCC54: 483CB511  bl 0x831a8164
	ctx.lr = 0x82DDCC58;
	sub_831A8130(ctx, base);
	// 82DDCC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCC5C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDCC60: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DDCC64: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDCC68: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCC6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCC70: 4E800421  bctrl
	ctx.lr = 0x82DDCC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCC74: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCC78: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCC7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDCC80: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 82DDCC84: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDCC88: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDCC8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DDCC90: 419A009C  beq cr6, 0x82ddcd2c
	if ctx.cr[6].eq {
	pc = 0x82DDCD2C; continue 'dispatch;
	}
	// 82DDCC94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCC98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDCC9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDCCA0: 4099008C  ble cr6, 0x82ddcd2c
	if !ctx.cr[6].gt {
	pc = 0x82DDCD2C; continue 'dispatch;
	}
	// 82DDCCA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDCCA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCCAC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCCB0: 40980010  bge cr6, 0x82ddccc0
	if !ctx.cr[6].lt {
	pc = 0x82DDCCC0; continue 'dispatch;
	}
	// 82DDCCB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCCB8: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DDCCBC: 48000010  b 0x82ddcccc
	pc = 0x82DDCCCC; continue 'dispatch;
	// 82DDCCC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDCCC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDCCC8: 4BFFFB51  bl 0x82ddc818
	ctx.lr = 0x82DDCCCC;
	sub_82DDC818(ctx, base);
	// 82DDCCCC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCCD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DDCCD4: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCCD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCCDC: 4E800421  bctrl
	ctx.lr = 0x82DDCCE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCCE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCCE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DDCCE8: 41820018  beq 0x82ddcd00
	if ctx.cr[0].eq {
	pc = 0x82DDCD00; continue 'dispatch;
	}
	// 82DDCCEC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDCCF0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DDCCF4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCCF8: 4198FFB0  blt cr6, 0x82ddcca8
	if ctx.cr[6].lt {
	pc = 0x82DDCCA8; continue 'dispatch;
	}
	// 82DDCCFC: 48000030  b 0x82ddcd2c
	pc = 0x82DDCD2C; continue 'dispatch;
	// 82DDCD00: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCD04: 40980028  bge cr6, 0x82ddcd2c
	if !ctx.cr[6].lt {
	pc = 0x82DDCD2C; continue 'dispatch;
	}
	// 82DDCD08: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDCD0C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCD10: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDCD14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDCD18: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82DDCD1C: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDCD20: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DDCD24: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DDCD28: 483CB7E9  bl 0x831a8510
	ctx.lr = 0x82DDCD2C;
	sub_831A8510(ctx, base);
	// 82DDCD2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDCD30: 483CB484  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCD38 size=112
    let mut pc: u32 = 0x82DDCD38;
    'dispatch: loop {
        match pc {
            0x82DDCD38 => {
    //   block [0x82DDCD38..0x82DDCDA8)
	// 82DDCD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCD3C: 483CB431  bl 0x831a816c
	ctx.lr = 0x82DDCD40;
	sub_831A8130(ctx, base);
	// 82DDCD40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCD44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDCD48: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDCD4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCD50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDCD54: 40990040  ble cr6, 0x82ddcd94
	if !ctx.cr[6].gt {
	pc = 0x82DDCD94; continue 'dispatch;
	}
	// 82DDCD58: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDCD5C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCD60: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDCD64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDCD68: 419A0018  beq cr6, 0x82ddcd80
	if ctx.cr[6].eq {
	pc = 0x82DDCD80; continue 'dispatch;
	}
	// 82DDCD6C: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDCD70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DDCD74: 419A000C  beq cr6, 0x82ddcd80
	if ctx.cr[6].eq {
	pc = 0x82DDCD80; continue 'dispatch;
	}
	// 82DDCD78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDCD7C: 4BFF57BD  bl 0x82dd2538
	ctx.lr = 0x82DDCD80;
	sub_82DD2538(ctx, base);
	// 82DDCD80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCD84: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DDCD88: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DDCD8C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCD90: 4198FFCC  blt cr6, 0x82ddcd5c
	if ctx.cr[6].lt {
	pc = 0x82DDCD5C; continue 'dispatch;
	}
	// 82DDCD94: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCD98: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCD9C: 4BFC896D  bl 0x82da5708
	ctx.lr = 0x82DDCDA0;
	sub_82DA5708(ctx, base);
	// 82DDCDA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDCDA4: 483CB418  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCDA8 size=196
    let mut pc: u32 = 0x82DDCDA8;
    'dispatch: loop {
        match pc {
            0x82DDCDA8 => {
    //   block [0x82DDCDA8..0x82DDCE6C)
	// 82DDCDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDCDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDCDB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCDB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDCDBC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCDC0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DDCDC4: 41980038  blt cr6, 0x82ddcdfc
	if ctx.cr[6].lt {
	pc = 0x82DDCDFC; continue 'dispatch;
	}
	// 82DDCDC8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCDCC: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82DDCDD0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DDCDD4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCDD8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDCDDC: 40980014  bge cr6, 0x82ddcdf0
	if !ctx.cr[6].lt {
	pc = 0x82DDCDF0; continue 'dispatch;
	}
	// 82DDCDE0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCDE4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDCDE8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDCDEC: 48000008  b 0x82ddcdf4
	pc = 0x82DDCDF4; continue 'dispatch;
	// 82DDCDF0: 4BFFFA29  bl 0x82ddc818
	ctx.lr = 0x82DDCDF4;
	sub_82DDC818(ctx, base);
	// 82DDCDF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCDF8: 4800005C  b 0x82ddce54
	pc = 0x82DDCE54; continue 'dispatch;
	// 82DDCDFC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE00: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCE04: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82DDCE08: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDCE0C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82DDCE10: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE14: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DDCE18: 40980038  bge cr6, 0x82ddce50
	if !ctx.cr[6].lt {
	pc = 0x82DDCE50; continue 'dispatch;
	}
	// 82DDCE1C: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCE20: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDCE24: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDCE28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDCE2C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DDCE30: 419AFFD4  beq cr6, 0x82ddce04
	if ctx.cr[6].eq {
	pc = 0x82DDCE04; continue 'dispatch;
	}
	// 82DDCE34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCE38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDCE3C: 419AFFC8  beq cr6, 0x82ddce04
	if ctx.cr[6].eq {
	pc = 0x82DDCE04; continue 'dispatch;
	}
	// 82DDCE40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCE44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCE48: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82DDCE4C: 4BFFFF7C  b 0x82ddcdc8
	pc = 0x82DDCDC8; continue 'dispatch;
	// 82DDCE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDCE54: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDCE58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDCE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDCE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDCE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDCE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDCE70 size=24
    let mut pc: u32 = 0x82DDCE70;
    'dispatch: loop {
        match pc {
            0x82DDCE70 => {
    //   block [0x82DDCE70..0x82DDCE88)
	// 82DDCE70: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DDCE74: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DDCE78: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DDCE7C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDCE80: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDCE84: 4BFFFF24  b 0x82ddcda8
	sub_82DDCDA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDCE88 size=28
    let mut pc: u32 = 0x82DDCE88;
    'dispatch: loop {
        match pc {
            0x82DDCE88 => {
    //   block [0x82DDCE88..0x82DDCEA4)
	// 82DDCE88: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DDCE8C: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 82DDCE90: 396BA778  addi r11, r11, -0x5888
	ctx.r[11].s64 = ctx.r[11].s64 + -22664;
	// 82DDCE94: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDCE98: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDCE9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCEA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCEA8 size=120
    let mut pc: u32 = 0x82DDCEA8;
    'dispatch: loop {
        match pc {
            0x82DDCEA8 => {
    //   block [0x82DDCEA8..0x82DDCF20)
	// 82DDCEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDCEB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDCEB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDCEB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCEBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDCEC0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82DDCEC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDCEC8: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 82DDCECC: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDCED0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDCED4: 40820024  bne 0x82ddcef8
	if !ctx.cr[0].eq {
	pc = 0x82DDCEF8; continue 'dispatch;
	}
	// 82DDCED8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DDCEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DDCEE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDCEE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDCEE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCEEC: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDCEF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCEF4: 4E800421  bctrl
	ctx.lr = 0x82DDCEF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCEF8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDCEFC: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DDCF00: 4198FFC8  blt cr6, 0x82ddcec8
	if ctx.cr[6].lt {
	pc = 0x82DDCEC8; continue 'dispatch;
	}
	// 82DDCF04: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDCF08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDCF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDCF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDCF14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDCF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDCF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCF20 size=144
    let mut pc: u32 = 0x82DDCF20;
    'dispatch: loop {
        match pc {
            0x82DDCF20 => {
    //   block [0x82DDCF20..0x82DDCFB0)
	// 82DDCF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDCF28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDCF2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDCF30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCF34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDCF38: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82DDCF3C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DDCF40: 4BFE1ED1  bl 0x82dbee10
	ctx.lr = 0x82DDCF44;
	sub_82DBEE10(ctx, base);
	// 82DDCF44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDCF48: 4182004C  beq 0x82ddcf94
	if ctx.cr[0].eq {
	pc = 0x82DDCF94; continue 'dispatch;
	}
	// 82DDCF4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDCF50: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 82DDCF54: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDCF58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDCF5C: 40820024  bne 0x82ddcf80
	if !ctx.cr[0].eq {
	pc = 0x82DDCF80; continue 'dispatch;
	}
	// 82DDCF60: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DDCF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DDCF68: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDCF6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDCF70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCF74: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDCF78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCF7C: 4E800421  bctrl
	ctx.lr = 0x82DDCF80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCF80: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDCF84: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DDCF88: 4198FFC8  blt cr6, 0x82ddcf50
	if ctx.cr[6].lt {
	pc = 0x82DDCF50; continue 'dispatch;
	}
	// 82DDCF8C: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DDCF90: 48000008  b 0x82ddcf98
	pc = 0x82DDCF98; continue 'dispatch;
	// 82DDCF94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDCF98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDCF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDCFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDCFA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDCFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDCFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCFB0 size=100
    let mut pc: u32 = 0x82DDCFB0;
    'dispatch: loop {
        match pc {
            0x82DDCFB0 => {
    //   block [0x82DDCFB0..0x82DDD014)
	// 82DDCFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCFB4: 483CB1B5  bl 0x831a8168
	ctx.lr = 0x82DDCFB8;
	sub_831A8130(ctx, base);
	// 82DDCFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCFBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDCFC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDCFC4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DDCFC8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDCFCC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCFD0: 838B05AC  lwz r28, 0x5ac(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDCFD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDCFD8: 4BFC85A9  bl 0x82da5580
	ctx.lr = 0x82DDCFDC;
	sub_82DA5580(ctx, base);
	// 82DDCFDC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDCFE0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DDCFE4: 41820018  beq 0x82ddcffc
	if ctx.cr[0].eq {
	pc = 0x82DDCFFC; continue 'dispatch;
	}
	// 82DDCFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDCFEC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDCFF0: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DDCFF4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDCFF8: 48000008  b 0x82ddd000
	pc = 0x82DDD000; continue 'dispatch;
	// 82DDCFFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDD000: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD004: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDD008: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDD00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDD010: 483CB1A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD018 size=16
    let mut pc: u32 = 0x82DDD018;
    'dispatch: loop {
        match pc {
            0x82DDD018 => {
    //   block [0x82DDD018..0x82DDD028)
	// 82DDD018: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDD01C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD020: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD024: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD028 size=20
    let mut pc: u32 = 0x82DDD028;
    'dispatch: loop {
        match pc {
            0x82DDD028 => {
    //   block [0x82DDD028..0x82DDD03C)
	// 82DDD028: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD02C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82DDD030: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD034: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DDD038: 4BFC86D0  b 0x82da5708
	sub_82DA5708(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD03C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD03C size=4
    let mut pc: u32 = 0x82DDD03C;
    'dispatch: loop {
        match pc {
            0x82DDD03C => {
    //   block [0x82DDD03C..0x82DDD040)
	// 82DDD03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD040 size=20
    let mut pc: u32 = 0x82DDD040;
    'dispatch: loop {
        match pc {
            0x82DDD040 => {
    //   block [0x82DDD040..0x82DDD054)
	// 82DDD040: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDD044: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD04C: 409A0008  bne cr6, 0x82ddd054
	if !ctx.cr[6].eq {
		sub_82DDD054(ctx, base);
		return;
	}
	// 82DDD050: 4BFFFF60  b 0x82ddcfb0
	sub_82DDCFB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD054(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD054 size=24
    let mut pc: u32 = 0x82DDD054;
    'dispatch: loop {
        match pc {
            0x82DDD054 => {
    //   block [0x82DDD054..0x82DDD06C)
	// 82DDD054: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD058: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD05C: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DDD060: 409A000C  bne cr6, 0x82ddd06c
	if !ctx.cr[6].eq {
		sub_82DDD06C(ctx, base);
		return;
	}
	// 82DDD064: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DDD068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD06C size=4
    let mut pc: u32 = 0x82DDD06C;
    'dispatch: loop {
        match pc {
            0x82DDD06C => {
    //   block [0x82DDD06C..0x82DDD070)
	// 82DDD06C: 4BFFFF44  b 0x82ddcfb0
	sub_82DDCFB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD070 size=88
    let mut pc: u32 = 0x82DDD070;
    'dispatch: loop {
        match pc {
            0x82DDD070 => {
    //   block [0x82DDD070..0x82DDD0C8)
	// 82DDD070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDD078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDD07C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD084: 48000024  b 0x82ddd0a8
	pc = 0x82DDD0A8; continue 'dispatch;
	// 82DDD088: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD08C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD090: 419A0018  beq cr6, 0x82ddd0a8
	if ctx.cr[6].eq {
	pc = 0x82DDD0A8; continue 'dispatch;
	}
	// 82DDD094: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD098: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82DDD09C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDD0A0: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DDD0A4: 4BFC8665  bl 0x82da5708
	ctx.lr = 0x82DDD0A8;
	sub_82DA5708(ctx, base);
	// 82DDD0A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD0AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD0B0: 409AFFD8  bne cr6, 0x82ddd088
	if !ctx.cr[6].eq {
	pc = 0x82DDD088; continue 'dispatch;
	}
	// 82DDD0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDD0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDD0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDD0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDD0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD0C8 size=256
    let mut pc: u32 = 0x82DDD0C8;
    'dispatch: loop {
        match pc {
            0x82DDD0C8 => {
    //   block [0x82DDD0C8..0x82DDD1C8)
	// 82DDD0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD0CC: 483CB099  bl 0x831a8164
	ctx.lr = 0x82DDD0D0;
	sub_831A8130(ctx, base);
	// 82DDD0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD0D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD0D8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD0DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDD0E0: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 82DDD0E4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DDD0E8: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82DDD0EC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDD0F0: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82DDD0F4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82DDD0F8: 9BBF0004  stb r29, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 82DDD0FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDD100: 9BBF0005  stb r29, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[29].u8 ) };
	// 82DDD104: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDD108: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82DDD10C: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82DDD110: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 82DDD114: 9BBF001D  stb r29, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[29].u8 ) };
	// 82DDD118: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD11C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDD120: 4BFC8461  bl 0x82da5580
	ctx.lr = 0x82DDD124;
	sub_82DA5580(ctx, base);
	// 82DDD124: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD128: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DDD12C: 41820010  beq 0x82ddd13c
	if ctx.cr[0].eq {
	pc = 0x82DDD13C; continue 'dispatch;
	}
	// 82DDD130: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDD134: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DDD138: 48000008  b 0x82ddd140
	pc = 0x82DDD140; continue 'dispatch;
	// 82DDD13C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DDD140: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82DDD144: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDD148: 837E05AC  lwz r27, 0x5ac(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD14C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDD150: 4BFC8431  bl 0x82da5580
	ctx.lr = 0x82DDD154;
	sub_82DA5580(ctx, base);
	// 82DDD154: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DDD158: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDD15C: 41820014  beq 0x82ddd170
	if ctx.cr[0].eq {
	pc = 0x82DDD170; continue 'dispatch;
	}
	// 82DDD160: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDD164: 809E05AC  lwz r4, 0x5ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD168: 4BFE1CF1  bl 0x82dbee58
	ctx.lr = 0x82DDD16C;
	sub_82DBEE58(ctx, base);
	// 82DDD16C: 48000008  b 0x82ddd174
	pc = 0x82DDD174; continue 'dispatch;
	// 82DDD170: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82DDD174: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82DDD178: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDD17C: 837E05AC  lwz r27, 0x5ac(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD180: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDD184: 4BFC83FD  bl 0x82da5580
	ctx.lr = 0x82DDD188;
	sub_82DA5580(ctx, base);
	// 82DDD188: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DDD18C: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDD190: 41820014  beq 0x82ddd1a4
	if ctx.cr[0].eq {
	pc = 0x82DDD1A4; continue 'dispatch;
	}
	// 82DDD194: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDD198: 809E05AC  lwz r4, 0x5ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD19C: 4BFE1CBD  bl 0x82dbee58
	ctx.lr = 0x82DDD1A0;
	sub_82DBEE58(ctx, base);
	// 82DDD1A0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82DDD1A4: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82DDD1A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD1AC: 817E0598  lwz r11, 0x598(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1432 as u32) ) } as u64;
	// 82DDD1B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DDD1B4: 817E0598  lwz r11, 0x598(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1432 as u32) ) } as u64;
	// 82DDD1B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDD1BC: 917E0598  stw r11, 0x598(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1432 as u32), ctx.r[11].u32 ) };
	// 82DDD1C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDD1C4: 483CAFF0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDD1C8 size=8
    let mut pc: u32 = 0x82DDD1C8;
    'dispatch: loop {
        match pc {
            0x82DDD1C8 => {
    //   block [0x82DDD1C8..0x82DDD1D0)
	// 82DDD1C8: 98830004  stb r4, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u8 ) };
	// 82DDD1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD1D0 size=80
    let mut pc: u32 = 0x82DDD1D0;
    'dispatch: loop {
        match pc {
            0x82DDD1D0 => {
    //   block [0x82DDD1D0..0x82DDD220)
	// 82DDD1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDD1D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDD1DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDD1E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD1E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD1E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDD1EC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDD1F0: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD1F4: 4BFFF625  bl 0x82ddc818
	ctx.lr = 0x82DDD1F8;
	sub_82DDC818(ctx, base);
	// 82DDD1F8: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDD1FC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDD200: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDD204: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DDD208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDD20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDD210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDD214: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDD218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDD21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD220 size=124
    let mut pc: u32 = 0x82DDD220;
    'dispatch: loop {
        match pc {
            0x82DDD220 => {
    //   block [0x82DDD220..0x82DDD29C)
	// 82DDD220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDD228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDD22C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDD230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD238: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDD23C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDD240: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82DDD244: 40990024  ble cr6, 0x82ddd268
	if !ctx.cr[6].gt {
	pc = 0x82DDD268; continue 'dispatch;
	}
	// 82DDD248: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82DDD24C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD250: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DDD254: 419A0030  beq cr6, 0x82ddd284
	if ctx.cr[6].eq {
	pc = 0x82DDD284; continue 'dispatch;
	}
	// 82DDD258: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DDD25C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DDD260: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82DDD264: 4198FFE8  blt cr6, 0x82ddd24c
	if ctx.cr[6].lt {
	pc = 0x82DDD24C; continue 'dispatch;
	}
	// 82DDD268: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDD26C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD270: 4BFFF5A9  bl 0x82ddc818
	ctx.lr = 0x82DDD274;
	sub_82DDC818(ctx, base);
	// 82DDD274: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDD278: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD27C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDD280: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DDD284: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDD288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDD28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDD290: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDD294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDD298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDD2A0 size=728
    let mut pc: u32 = 0x82DDD2A0;
    'dispatch: loop {
        match pc {
            0x82DDD2A0 => {
    //   block [0x82DDD2A0..0x82DDD578)
	// 82DDD2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD2A4: 483CAEB1  bl 0x831a8154
	ctx.lr = 0x82DDD2A8;
	sub_831A8130(ctx, base);
	// 82DDD2A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD2AC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDD2B0: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DDD2B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDD2B8: 817A002C  lwz r11, 0x2c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDD2BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD2C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDD2C4: 419A0010  beq cr6, 0x82ddd2d4
	if ctx.cr[6].eq {
	pc = 0x82DDD2D4; continue 'dispatch;
	}
	// 82DDD2C8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD2CC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD2D0: 480002A0  b 0x82ddd570
	pc = 0x82DDD570; continue 'dispatch;
	// 82DDD2D4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD2D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD2DC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD2E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD2E4: 4E800421  bctrl
	ctx.lr = 0x82DDD2E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD2E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD2EC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DDD2F0: 3B600003  li r27, 3
	ctx.r[27].s64 = 3;
	// 82DDD2F4: 41820110  beq 0x82ddd404
	if ctx.cr[0].eq {
	pc = 0x82DDD404; continue 'dispatch;
	}
	// 82DDD2F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD2FC: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDD300: 48003E19  bl 0x82de1118
	ctx.lr = 0x82DDD304;
	sub_82DE1118(ctx, base);
	// 82DDD304: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDD308: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDD30C: 809C0AB0  lwz r4, 0xab0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDD310: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DDD314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD318: C06B08A4  lfs f3, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82DDD31C: C08A08A8  lfs f4, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82DDD320: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 82DDD324: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 82DDD328: 48002949  bl 0x82ddfc70
	ctx.lr = 0x82DDD32C;
	sub_82DDFC70(ctx, base);
	// 82DDD32C: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82DDD330: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD334: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82DDD338: 806A0028  lwz r3, 0x28(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDD33C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD340: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD344: 4099000C  ble cr6, 0x82ddd350
	if !ctx.cr[6].gt {
	pc = 0x82DDD350; continue 'dispatch;
	}
	// 82DDD348: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD34C: 4800000C  b 0x82ddd358
	pc = 0x82DDD358; continue 'dispatch;
	// 82DDD350: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDD354: 4BFFF4C5  bl 0x82ddc818
	ctx.lr = 0x82DDD358;
	sub_82DDC818(ctx, base);
	// 82DDD358: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD35C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDD360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD364: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDD368: 4BFE1D81  bl 0x82dbf0e8
	ctx.lr = 0x82DDD36C;
	sub_82DBF0E8(ctx, base);
	// 82DDD36C: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDD370: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDD374: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD37C: 409A0100  bne cr6, 0x82ddd47c
	if !ctx.cr[6].eq {
	pc = 0x82DDD47C; continue 'dispatch;
	}
	// 82DDD380: 82FC05B0  lwz r23, 0x5b0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DDD384: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 82DDD388: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DDD38C: 4BFC81F5  bl 0x82da5580
	ctx.lr = 0x82DDD390;
	sub_82DA5580(ctx, base);
	// 82DDD390: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD394: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82DDD398: 41820028  beq 0x82ddd3c0
	if ctx.cr[0].eq {
	pc = 0x82DDD3C0; continue 'dispatch;
	}
	// 82DDD39C: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DDD3A0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DDD3A4: 938B0104  stw r28, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[28].u32 ) };
	// 82DDD3A8: 9B2B0108  stb r25, 0x108(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[25].u8 ) };
	// 82DDD3AC: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDD3B0: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DDD3B4: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82DDD3B8: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82DDD3BC: 48000008  b 0x82ddd3c4
	pc = 0x82DDD3C4; continue 'dispatch;
	// 82DDD3C0: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82DDD3C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDD3C8: 4BFF48A9  bl 0x82dd1c70
	ctx.lr = 0x82DDD3CC;
	sub_82DD1C70(ctx, base);
	// 82DDD3CC: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDD3D0: 80980030  lwz r4, 0x30(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DDD3D4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD3D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD3DC: 419A001C  beq cr6, 0x82ddd3f8
	if ctx.cr[6].eq {
	pc = 0x82DDD3F8; continue 'dispatch;
	}
	// 82DDD3E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD3E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD3E8: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DDD3EC: 409A000C  bne cr6, 0x82ddd3f8
	if !ctx.cr[6].eq {
	pc = 0x82DDD3F8; continue 'dispatch;
	}
	// 82DDD3F0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDD3F4: 48000088  b 0x82ddd47c
	pc = 0x82DDD47C; continue 'dispatch;
	// 82DDD3F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDD3FC: 4BFFFBB5  bl 0x82ddcfb0
	ctx.lr = 0x82DDD400;
	sub_82DDCFB0(ctx, base);
	// 82DDD400: 4800007C  b 0x82ddd47c
	pc = 0x82DDD47C; continue 'dispatch;
	// 82DDD404: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD408: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD40C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDD410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD414: 4E800421  bctrl
	ctx.lr = 0x82DDD418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD418: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD41C: 40820028  bne 0x82ddd444
	if !ctx.cr[0].eq {
	pc = 0x82DDD444; continue 'dispatch;
	}
	// 82DDD420: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD424: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82DDD428: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDD42C: 38CB5BC8  addi r6, r11, 0x5bc8
	ctx.r[6].s64 = ctx.r[11].s64 + 23496;
	// 82DDD430: 38AA9FF4  addi r5, r10, -0x600c
	ctx.r[5].s64 = ctx.r[10].s64 + -24588;
	// 82DDD434: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDD438: 38E000EC  li r7, 0xec
	ctx.r[7].s64 = 236;
	// 82DDD43C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDD440: 4BF05641  bl 0x82ce2a80
	ctx.lr = 0x82DDD444;
	sub_82CE2A80(ctx, base);
	// 82DDD444: 83FC05AC  lwz r31, 0x5ac(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD448: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDD44C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD450: 4BFC8131  bl 0x82da5580
	ctx.lr = 0x82DDD454;
	sub_82DA5580(ctx, base);
	// 82DDD454: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDD458: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDD45C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DDD460: 41820018  beq 0x82ddd478
	if ctx.cr[0].eq {
	pc = 0x82DDD478; continue 'dispatch;
	}
	// 82DDD464: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDD468: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DDD46C: 48002CBD  bl 0x82de0128
	ctx.lr = 0x82DDD470;
	sub_82DE0128(ctx, base);
	// 82DDD470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD474: 48000008  b 0x82ddd47c
	pc = 0x82DDD47C; continue 'dispatch;
	// 82DDD478: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DDD47C: 935F001C  stw r26, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 82DDD480: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD484: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD488: 4BFFFD49  bl 0x82ddd1d0
	ctx.lr = 0x82DDD48C;
	sub_82DDD1D0(ctx, base);
	// 82DDD48C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD490: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD494: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DDD498: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDD49C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD4A0: 4E800421  bctrl
	ctx.lr = 0x82DDD4A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD4A4: 897A0005  lbz r11, 5(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DDD4A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDD4AC: 41820024  beq 0x82ddd4d0
	if ctx.cr[0].eq {
	pc = 0x82DDD4D0; continue 'dispatch;
	}
	// 82DDD4B0: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD4B4: 815A0010  lwz r10, 0x10(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD4B8: 813F00E4  lwz r9, 0xe4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDD4BC: 61290040  ori r9, r9, 0x40
	ctx.r[9].u64 = ctx.r[9].u64 | 64;
	// 82DDD4C0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDD4C4: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DDD4C8: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 82DDD4CC: 4800002C  b 0x82ddd4f8
	pc = 0x82DDD4F8; continue 'dispatch;
	// 82DDD4D0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD4D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD4D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD4DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD4E0: 4E800421  bctrl
	ctx.lr = 0x82DDD4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD4E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD4E8: 41820010  beq 0x82ddd4f8
	if ctx.cr[0].eq {
	pc = 0x82DDD4F8; continue 'dispatch;
	}
	// 82DDD4EC: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82DDD4F0: 933F0050  stw r25, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 82DDD4F4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82DDD4F8: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD4FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDD500: 41820010  beq 0x82ddd510
	if ctx.cr[0].eq {
	pc = 0x82DDD510; continue 'dispatch;
	}
	// 82DDD504: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDD508: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82DDD50C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDD510: 83DC05B0  lwz r30, 0x5b0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DDD514: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 82DDD518: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDD51C: 4BFC8065  bl 0x82da5580
	ctx.lr = 0x82DDD520;
	sub_82DA5580(ctx, base);
	// 82DDD520: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD524: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDD528: 41820028  beq 0x82ddd550
	if ctx.cr[0].eq {
	pc = 0x82DDD550; continue 'dispatch;
	}
	// 82DDD52C: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82DDD530: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DDD534: 938B0104  stw r28, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[28].u32 ) };
	// 82DDD538: 9B2B0108  stb r25, 0x108(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[25].u8 ) };
	// 82DDD53C: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDD540: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DDD544: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82DDD548: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82DDD54C: 48000008  b 0x82ddd554
	pc = 0x82DDD554; continue 'dispatch;
	// 82DDD550: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DDD554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD558: 4BFF4719  bl 0x82dd1c70
	ctx.lr = 0x82DDD55C;
	sub_82DD1C70(ctx, base);
	// 82DDD55C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDD560: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DDD564: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DDD568: 48005301  bl 0x82de2868
	ctx.lr = 0x82DDD56C;
	sub_82DE2868(ctx, base);
	// 82DDD56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD570: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDD574: 483CAC30  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD578 size=164
    let mut pc: u32 = 0x82DDD578;
    'dispatch: loop {
        match pc {
            0x82DDD578 => {
    //   block [0x82DDD578..0x82DDD61C)
	// 82DDD578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD57C: 483CABE9  bl 0x831a8164
	ctx.lr = 0x82DDD580;
	sub_831A8130(ctx, base);
	// 82DDD580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD584: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDD588: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDD58C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDD590: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDD594: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDD598: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD59C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD5A0: 40990074  ble cr6, 0x82ddd614
	if !ctx.cr[6].gt {
	pc = 0x82DDD614; continue 'dispatch;
	}
	// 82DDD5A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDD5A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD5AC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDD5B0: 40980010  bge cr6, 0x82ddd5c0
	if !ctx.cr[6].lt {
	pc = 0x82DDD5C0; continue 'dispatch;
	}
	// 82DDD5B4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD5B8: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDD5BC: 4800000C  b 0x82ddd5c8
	pc = 0x82DDD5C8; continue 'dispatch;
	// 82DDD5C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD5C4: 4BFFF255  bl 0x82ddc818
	ctx.lr = 0x82DDD5C8;
	sub_82DDC818(ctx, base);
	// 82DDD5C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD5CC: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDD5D0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DDD5D4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD5D8: 419A0018  beq cr6, 0x82ddd5f0
	if ctx.cr[6].eq {
	pc = 0x82DDD5F0; continue 'dispatch;
	}
	// 82DDD5DC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDD5E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DDD5E4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDD5E8: 4198FFC0  blt cr6, 0x82ddd5a8
	if ctx.cr[6].lt {
	pc = 0x82DDD5A8; continue 'dispatch;
	}
	// 82DDD5EC: 48000028  b 0x82ddd614
	pc = 0x82DDD614; continue 'dispatch;
	// 82DDD5F0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDD5F4: 40980014  bge cr6, 0x82ddd608
	if !ctx.cr[6].lt {
	pc = 0x82DDD608; continue 'dispatch;
	}
	// 82DDD5F8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD5FC: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDD600: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDD604: 4800000C  b 0x82ddd610
	pc = 0x82DDD610; continue 'dispatch;
	// 82DDD608: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD60C: 4BFFF20D  bl 0x82ddc818
	ctx.lr = 0x82DDD610;
	sub_82DDC818(ctx, base);
	// 82DDD610: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DDD614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDD618: 483CAB9C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD620 size=132
    let mut pc: u32 = 0x82DDD620;
    'dispatch: loop {
        match pc {
            0x82DDD620 => {
    //   block [0x82DDD620..0x82DDD6A4)
	// 82DDD620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD624: 483CAB45  bl 0x831a8168
	ctx.lr = 0x82DDD628;
	sub_831A8130(ctx, base);
	// 82DDD628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD62C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDD630: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDD634: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDD638: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDD63C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD640: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD644: 40990058  ble cr6, 0x82ddd69c
	if !ctx.cr[6].gt {
	pc = 0x82DDD69C; continue 'dispatch;
	}
	// 82DDD648: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDD64C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD650: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDD654: 40980010  bge cr6, 0x82ddd664
	if !ctx.cr[6].lt {
	pc = 0x82DDD664; continue 'dispatch;
	}
	// 82DDD658: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD65C: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDD660: 4800000C  b 0x82ddd66c
	pc = 0x82DDD66C; continue 'dispatch;
	// 82DDD664: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD668: 4BFFF1B1  bl 0x82ddc818
	ctx.lr = 0x82DDD66C;
	sub_82DDC818(ctx, base);
	// 82DDD66C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD670: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDD674: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DDD678: 419A001C  beq cr6, 0x82ddd694
	if ctx.cr[6].eq {
	pc = 0x82DDD694; continue 'dispatch;
	}
	// 82DDD67C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD680: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDD684: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DDD688: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDD68C: 4198FFC0  blt cr6, 0x82ddd64c
	if ctx.cr[6].lt {
	pc = 0x82DDD64C; continue 'dispatch;
	}
	// 82DDD690: 4800000C  b 0x82ddd69c
	pc = 0x82DDD69C; continue 'dispatch;
	// 82DDD694: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD698: 4BFFF0C1  bl 0x82ddc758
	ctx.lr = 0x82DDD69C;
	sub_82DDC758(ctx, base);
	// 82DDD69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDD6A0: 483CAB18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD6A8 size=440
    let mut pc: u32 = 0x82DDD6A8;
    'dispatch: loop {
        match pc {
            0x82DDD6A8 => {
    //   block [0x82DDD6A8..0x82DDD860)
	// 82DDD6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD6AC: 483CAAAD  bl 0x831a8158
	ctx.lr = 0x82DDD6B0;
	sub_831A8130(ctx, base);
	// 82DDD6B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD6B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDD6B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DDD6BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDD6C0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DDD6C4: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDD6C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD6CC: 409A0020  bne cr6, 0x82ddd6ec
	if !ctx.cr[6].eq {
	pc = 0x82DDD6EC; continue 'dispatch;
	}
	// 82DDD6D0: 817F05E4  lwz r11, 0x5e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1508 as u32) ) } as u64;
	// 82DDD6D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDD6D8: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82DDD6DC: 90BF05E4  stw r5, 0x5e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 82DDD6E0: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDD6E4: 4BFF094D  bl 0x82dce030
	ctx.lr = 0x82DDD6E8;
	sub_82DCE030(ctx, base);
	// 82DDD6E8: 907D0034  stw r3, 0x34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82DDD6EC: 83FF05AC  lwz r31, 0x5ac(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD6F0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDD6F4: 831D0034  lwz r24, 0x34(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDD6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD6FC: 4BFC7E85  bl 0x82da5580
	ctx.lr = 0x82DDD700;
	sub_82DA5580(ctx, base);
	// 82DDD700: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDD704: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDD708: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DDD70C: 41820014  beq 0x82ddd720
	if ctx.cr[0].eq {
	pc = 0x82DDD720; continue 'dispatch;
	}
	// 82DDD710: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD714: 48002725  bl 0x82ddfe38
	ctx.lr = 0x82DDD718;
	sub_82DDFE38(ctx, base);
	// 82DDD718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD71C: 48000008  b 0x82ddd724
	pc = 0x82DDD724; continue 'dispatch;
	// 82DDD720: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDD724: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDD728: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDD72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD730: 48001921  bl 0x82ddf050
	ctx.lr = 0x82DDD734;
	sub_82DDF050(ctx, base);
	// 82DDD734: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD738: 93FD0030  stw r31, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82DDD73C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82DDD740: 409A0008  bne cr6, 0x82ddd748
	if !ctx.cr[6].eq {
	pc = 0x82DDD748; continue 'dispatch;
	}
	// 82DDD744: 93FE0074  stw r31, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82DDD748: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD74C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82DDD750: 409A0010  bne cr6, 0x82ddd760
	if !ctx.cr[6].eq {
	pc = 0x82DDD760; continue 'dispatch;
	}
	// 82DDD754: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDD758: 93FE0078  stw r31, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82DDD75C: 997E007E  stb r11, 0x7e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(126 as u32), ctx.r[11].u8 ) };
	// 82DDD760: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD764: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD768: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 82DDD76C: 3B8B5BC0  addi r28, r11, 0x5bc0
	ctx.r[28].s64 = ctx.r[11].s64 + 23488;
	// 82DDD770: 409A0020  bne cr6, 0x82ddd790
	if !ctx.cr[6].eq {
	pc = 0x82DDD790; continue 'dispatch;
	}
	// 82DDD774: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD778: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DDD77C: 93FE0070  stw r31, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82DDD780: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDD784: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD788: 4BFFF091  bl 0x82ddc818
	ctx.lr = 0x82DDD78C;
	sub_82DDC818(ctx, base);
	// 82DDD78C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DDD790: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD794: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82DDD798: 409A0018  bne cr6, 0x82ddd7b0
	if !ctx.cr[6].eq {
	pc = 0x82DDD7B0; continue 'dispatch;
	}
	// 82DDD79C: 93FE006C  stw r31, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82DDD7A0: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDD7A4: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD7A8: 4BFFF071  bl 0x82ddc818
	ctx.lr = 0x82DDD7AC;
	sub_82DDC818(ctx, base);
	// 82DDD7AC: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DDD7B0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82DDD7B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDD7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD7BC: 48001895  bl 0x82ddf050
	ctx.lr = 0x82DDD7C0;
	sub_82DDF050(ctx, base);
	// 82DDD7C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD7C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDD7C8: 4BFE1F79  bl 0x82dbf740
	ctx.lr = 0x82DDD7CC;
	sub_82DBF740(ctx, base);
	// 82DDD7CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD7D0: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82DDD7D4: 480044B5  bl 0x82de1c88
	ctx.lr = 0x82DDD7D8;
	sub_82DE1C88(ctx, base);
	// 82DDD7D8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD7DC: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82DDD7E0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DDD7E4: 3D208211  lis r9, -0x7def
	ctx.r[9].s64 = -2112815104;
	// 82DDD7E8: 3D008211  lis r8, -0x7def
	ctx.r[8].s64 = -2112815104;
	// 82DDD7EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDD7F0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDD7F4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD7F8: 3B8A5E90  addi r28, r10, 0x5e90
	ctx.r[28].s64 = ctx.r[10].s64 + 24208;
	// 82DDD7FC: 3BAB5C3C  addi r29, r11, 0x5c3c
	ctx.r[29].s64 = ctx.r[11].s64 + 23612;
	// 82DDD800: 3B695C1C  addi r27, r9, 0x5c1c
	ctx.r[27].s64 = ctx.r[9].s64 + 23580;
	// 82DDD804: 3B485BC8  addi r26, r8, 0x5bc8
	ctx.r[26].s64 = ctx.r[8].s64 + 23496;
	// 82DDD808: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DDD80C: 7F3E58AE  lbzx r25, r30, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDD810: 2B190008  cmplwi cr6, r25, 8
	ctx.cr[6].compare_u32(ctx.r[25].u32, 8 as u32, &mut ctx.xer);
	// 82DDD814: 4198001C  blt cr6, 0x82ddd830
	if ctx.cr[6].lt {
	pc = 0x82DDD830; continue 'dispatch;
	}
	// 82DDD818: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDD81C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDD820: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD824: 38E00169  li r7, 0x169
	ctx.r[7].s64 = 361;
	// 82DDD828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDD82C: 4BF05255  bl 0x82ce2a80
	ctx.lr = 0x82DDD830;
	sub_82CE2A80(ctx, base);
	// 82DDD830: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD834: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DDD838: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DDD83C: 7D7E51AE  stbx r11, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82DDD840: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDD844: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82DDD848: 4198FFC0  blt cr6, 0x82ddd808
	if ctx.cr[6].lt {
	pc = 0x82DDD808; continue 'dispatch;
	}
	// 82DDD84C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDD850: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DDD854: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DDD858: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDD85C: 483CA94C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD860 size=364
    let mut pc: u32 = 0x82DDD860;
    'dispatch: loop {
        match pc {
            0x82DDD860 => {
    //   block [0x82DDD860..0x82DDD9CC)
	// 82DDD860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD864: 483CA8F5  bl 0x831a8158
	ctx.lr = 0x82DDD868;
	sub_831A8130(ctx, base);
	// 82DDD868: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD870: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDD874: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDD878: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DDD87C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DDD880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD884: 409A0020  bne cr6, 0x82ddd8a4
	if !ctx.cr[6].eq {
	pc = 0x82DDD8A4; continue 'dispatch;
	}
	// 82DDD888: 817D05E4  lwz r11, 0x5e4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1508 as u32) ) } as u64;
	// 82DDD88C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDD890: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82DDD894: 90BD05E4  stw r5, 0x5e4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 82DDD898: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDD89C: 4BFF0795  bl 0x82dce030
	ctx.lr = 0x82DDD8A0;
	sub_82DCE030(ctx, base);
	// 82DDD8A0: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82DDD8A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDD8A8: 4BFE1569  bl 0x82dbee10
	ctx.lr = 0x82DDD8AC;
	sub_82DBEE10(ctx, base);
	// 82DDD8AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD8B0: 41820110  beq 0x82ddd9c0
	if ctx.cr[0].eq {
	pc = 0x82DDD9C0; continue 'dispatch;
	}
	// 82DDD8B4: 83BD05AC  lwz r29, 0x5ac(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDD8B8: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDD8BC: 831F003C  lwz r24, 0x3c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DDD8C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDD8C4: 4BFC7CBD  bl 0x82da5580
	ctx.lr = 0x82DDD8C8;
	sub_82DA5580(ctx, base);
	// 82DDD8C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDD8CC: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDD8D0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDD8D4: 41820010  beq 0x82ddd8e4
	if ctx.cr[0].eq {
	pc = 0x82DDD8E4; continue 'dispatch;
	}
	// 82DDD8D8: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD8DC: 4800255D  bl 0x82ddfe38
	ctx.lr = 0x82DDD8E0;
	sub_82DDFE38(ctx, base);
	// 82DDD8E0: 48000008  b 0x82ddd8e8
	pc = 0x82DDD8E8; continue 'dispatch;
	// 82DDD8E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDD8E8: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82DDD8EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDD8F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDD8F4: 4800175D  bl 0x82ddf050
	ctx.lr = 0x82DDD8F8;
	sub_82DDF050(ctx, base);
	// 82DDD8F8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82DDD8FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDD900: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DDD904: 4800174D  bl 0x82ddf050
	ctx.lr = 0x82DDD908;
	sub_82DDF050(ctx, base);
	// 82DDD908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDD90C: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DDD910: 4BFE1E31  bl 0x82dbf740
	ctx.lr = 0x82DDD914;
	sub_82DBF740(ctx, base);
	// 82DDD914: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DDD918: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82DDD91C: 48004325  bl 0x82de1c40
	ctx.lr = 0x82DDD920;
	sub_82DE1C40(ctx, base);
	// 82DDD920: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDD924: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD928: 83DF0038  lwz r30, 0x38(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DDD92C: 4BFFEEED  bl 0x82ddc818
	ctx.lr = 0x82DDD930;
	sub_82DDC818(ctx, base);
	// 82DDD930: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD934: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDD938: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82DDD93C: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82DDD940: 3D208211  lis r9, -0x7def
	ctx.r[9].s64 = -2112815104;
	// 82DDD944: 3D008211  lis r8, -0x7def
	ctx.r[8].s64 = -2112815104;
	// 82DDD948: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDD94C: 816B5BC0  lwz r11, 0x5bc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23488 as u32) ) } as u64;
	// 82DDD950: 3B8A5E90  addi r28, r10, 0x5e90
	ctx.r[28].s64 = ctx.r[10].s64 + 24208;
	// 82DDD954: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDD958: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD95C: 3BAB5C3C  addi r29, r11, 0x5c3c
	ctx.r[29].s64 = ctx.r[11].s64 + 23612;
	// 82DDD960: 3B695C1C  addi r27, r9, 0x5c1c
	ctx.r[27].s64 = ctx.r[9].s64 + 23580;
	// 82DDD964: 3B485BC8  addi r26, r8, 0x5bc8
	ctx.r[26].s64 = ctx.r[8].s64 + 23496;
	// 82DDD968: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DDD96C: 7F3E58AE  lbzx r25, r30, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDD970: 2B190008  cmplwi cr6, r25, 8
	ctx.cr[6].compare_u32(ctx.r[25].u32, 8 as u32, &mut ctx.xer);
	// 82DDD974: 4198001C  blt cr6, 0x82ddd990
	if ctx.cr[6].lt {
	pc = 0x82DDD990; continue 'dispatch;
	}
	// 82DDD978: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDD97C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDD980: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD984: 38E00169  li r7, 0x169
	ctx.r[7].s64 = 361;
	// 82DDD988: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDD98C: 4BF050F5  bl 0x82ce2a80
	ctx.lr = 0x82DDD990;
	sub_82CE2A80(ctx, base);
	// 82DDD990: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD994: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DDD998: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DDD99C: 7D7E51AE  stbx r11, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82DDD9A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDD9A4: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82DDD9A8: 4198FFC0  blt cr6, 0x82ddd968
	if ctx.cr[6].lt {
	pc = 0x82DDD968; continue 'dispatch;
	}
	// 82DDD9AC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DDD9B0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DDD9B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDD9B8: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DDD9BC: 48000008  b 0x82ddd9c4
	pc = 0x82DDD9C4; continue 'dispatch;
	// 82DDD9C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD9C4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDD9C8: 483CA7E0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD9D0 size=80
    let mut pc: u32 = 0x82DDD9D0;
    'dispatch: loop {
        match pc {
            0x82DDD9D0 => {
    //   block [0x82DDD9D0..0x82DDDA20)
	// 82DDD9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDD9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDD9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD9E0: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDD9E4: 83E3002C  lwz r31, 0x2c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDD9E8: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 82DDD9EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DDD9F0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDD9F4: 419A0018  beq cr6, 0x82ddda0c
	if ctx.cr[6].eq {
	pc = 0x82DDDA0C; continue 'dispatch;
	}
	// 82DDD9F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD9FC: 4BFFF675  bl 0x82ddd070
	ctx.lr = 0x82DDDA00;
	sub_82DDD070(ctx, base);
	// 82DDDA00: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 82DDDA04: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DDDA08: 4BFC7D01  bl 0x82da5708
	ctx.lr = 0x82DDDA0C;
	sub_82DA5708(ctx, base);
	// 82DDDA0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDDA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDDA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDDA18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDDA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDA20 size=316
    let mut pc: u32 = 0x82DDDA20;
    'dispatch: loop {
        match pc {
            0x82DDDA20 => {
    //   block [0x82DDDA20..0x82DDDB5C)
	// 82DDDA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDA24: 483CA745  bl 0x831a8168
	ctx.lr = 0x82DDDA28;
	sub_831A8130(ctx, base);
	// 82DDDA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDA2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDDA30: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDDA34: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDDA38: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DDDA3C: 4BFFF68D  bl 0x82ddd0c8
	ctx.lr = 0x82DDDA40;
	sub_82DDD0C8(ctx, base);
	// 82DDDA40: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDA44: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82DDDA48: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82DDDA4C: 396B5C80  addi r11, r11, 0x5c80
	ctx.r[11].s64 = ctx.r[11].s64 + 23680;
	// 82DDDA50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDA54: 817F0588  lwz r11, 0x588(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82DDDA58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDDA5C: 917F0588  stw r11, 0x588(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1416 as u32), ctx.r[11].u32 ) };
	// 82DDDA60: 419A005C  beq cr6, 0x82dddabc
	if ctx.cr[6].eq {
	pc = 0x82DDDABC; continue 'dispatch;
	}
	// 82DDDA64: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 82DDDA68: 419A0030  beq cr6, 0x82ddda98
	if ctx.cr[6].eq {
	pc = 0x82DDDA98; continue 'dispatch;
	}
	// 82DDDA6C: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 82DDDA70: 409A0070  bne cr6, 0x82dddae0
	if !ctx.cr[6].eq {
	pc = 0x82DDDAE0; continue 'dispatch;
	}
	// 82DDDA74: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 82DDDA78: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDDA7C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDDA80: 41980060  blt cr6, 0x82dddae0
	if ctx.cr[6].lt {
	pc = 0x82DDDAE0; continue 'dispatch;
	}
	// 82DDDA84: 3960001E  li r11, 0x1e
	ctx.r[11].s64 = 30;
	// 82DDDA88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDDA8C: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 82DDDA90: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DDDA94: 483D024D  bl 0x831adce0
	ctx.lr = 0x82DDDA98;
	sub_831ADCE0(ctx, base);
	// 82DDDA98: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 82DDDA9C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDDAA0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDDAA4: 4198003C  blt cr6, 0x82dddae0
	if ctx.cr[6].lt {
	pc = 0x82DDDAE0; continue 'dispatch;
	}
	// 82DDDAA8: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82DDDAAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDDAB0: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 82DDDAB4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DDDAB8: 483D0229  bl 0x831adce0
	ctx.lr = 0x82DDDABC;
	sub_831ADCE0(ctx, base);
	// 82DDDABC: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 82DDDAC0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDDAC4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDDAC8: 41980018  blt cr6, 0x82dddae0
	if ctx.cr[6].lt {
	pc = 0x82DDDAE0; continue 'dispatch;
	}
	// 82DDDACC: 3960001D  li r11, 0x1d
	ctx.r[11].s64 = 29;
	// 82DDDAD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDDAD4: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 82DDDAD8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DDDADC: 483D0205  bl 0x831adce0
	ctx.lr = 0x82DDDAE0;
	sub_831ADCE0(ctx, base);
	// 82DDDAE0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDDAE4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDDAE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDAEC: 4BFC7A95  bl 0x82da5580
	ctx.lr = 0x82DDDAF0;
	sub_82DA5580(ctx, base);
	// 82DDDAF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDDAF4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDDAF8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDDAFC: 41820018  beq 0x82dddb14
	if ctx.cr[0].eq {
	pc = 0x82DDDB14; continue 'dispatch;
	}
	// 82DDDB00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDDB04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDDB08: 48002541  bl 0x82de0048
	ctx.lr = 0x82DDDB0C;
	sub_82DE0048(ctx, base);
	// 82DDDB0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDDB10: 48000008  b 0x82dddb18
	pc = 0x82DDDB18; continue 'dispatch;
	// 82DDDB14: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDDB18: 817F0AB0  lwz r11, 0xab0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDB1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDB20: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDDB24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDB28: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDDB2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDDB30: 4E800421  bctrl
	ctx.lr = 0x82DDDB34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDDB34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDDB38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDDB3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDB40: 48001511  bl 0x82ddf050
	ctx.lr = 0x82DDDB44;
	sub_82DDF050(ctx, base);
	// 82DDDB44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDB48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDDB4C: 4BFFF685  bl 0x82ddd1d0
	ctx.lr = 0x82DDDB50;
	sub_82DDD1D0(ctx, base);
	// 82DDDB50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDDB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDDB58: 483CA660  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDB60 size=96
    let mut pc: u32 = 0x82DDDB60;
    'dispatch: loop {
        match pc {
            0x82DDDB60 => {
    //   block [0x82DDDB60..0x82DDDBC0)
	// 82DDDB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDDB68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDDB6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDDB70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDB74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDB78: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDDB7C: 4BFFF54D  bl 0x82ddd0c8
	ctx.lr = 0x82DDDB80;
	sub_82DDD0C8(ctx, base);
	// 82DDDB80: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDB84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDB88: 396B5D50  addi r11, r11, 0x5d50
	ctx.r[11].s64 = ctx.r[11].s64 + 23888;
	// 82DDDB8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDB90: 817E0584  lwz r11, 0x584(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82DDDB94: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDDB98: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DDDB9C: 817E0584  lwz r11, 0x584(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82DDDBA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDDBA4: 917E0584  stw r11, 0x584(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1412 as u32), ctx.r[11].u32 ) };
	// 82DDDBA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDDBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDDBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDDBB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDDBB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDDBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDBC0 size=68
    let mut pc: u32 = 0x82DDDBC0;
    'dispatch: loop {
        match pc {
            0x82DDDBC0 => {
    //   block [0x82DDDBC0..0x82DDDC04)
	// 82DDDBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDDBC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDDBCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDBD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDBD4: 4BFFF4F5  bl 0x82ddd0c8
	ctx.lr = 0x82DDDBD8;
	sub_82DDD0C8(ctx, base);
	// 82DDDBD8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDBDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDDBE0: 396B5CA4  addi r11, r11, 0x5ca4
	ctx.r[11].s64 = ctx.r[11].s64 + 23716;
	// 82DDDBE4: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82DDDBE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDBEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDDBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDDBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDDBFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDDC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDC08 size=216
    let mut pc: u32 = 0x82DDDC08;
    'dispatch: loop {
        match pc {
            0x82DDDC08 => {
    //   block [0x82DDDC08..0x82DDDCE0)
	// 82DDDC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDC0C: 483CA55D  bl 0x831a8168
	ctx.lr = 0x82DDDC10;
	sub_831A8130(ctx, base);
	// 82DDDC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDC14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDC18: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDDC1C: 4BFFF4AD  bl 0x82ddd0c8
	ctx.lr = 0x82DDDC20;
	sub_82DDD0C8(ctx, base);
	// 82DDDC20: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDC24: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDC28: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDDC2C: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDDC30: 396B5CC8  addi r11, r11, 0x5cc8
	ctx.r[11].s64 = ctx.r[11].s64 + 23752;
	// 82DDDC34: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82DDDC38: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DDDC3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDC40: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82DDDC44: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82DDDC48: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82DDDC4C: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDC50: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 82DDDC54: 4BFFCCF5  bl 0x82dda948
	ctx.lr = 0x82DDDC58;
	sub_82DDA948(ctx, base);
	// 82DDDC58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDDC5C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82DDDC60: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82DDDC64: 897E0564  lbz r11, 0x564(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1380 as u32) ) } as u64;
	// 82DDDC68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDDC6C: 40820068  bne 0x82dddcd4
	if !ctx.cr[0].eq {
	pc = 0x82DDDCD4; continue 'dispatch;
	}
	// 82DDDC70: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDDC74: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDDC78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDDC7C: 4BFC7905  bl 0x82da5580
	ctx.lr = 0x82DDDC80;
	sub_82DA5580(ctx, base);
	// 82DDDC80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDDC84: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDDC88: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DDDC8C: 41820010  beq 0x82dddc9c
	if ctx.cr[0].eq {
	pc = 0x82DDDC9C; continue 'dispatch;
	}
	// 82DDDC90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDDC94: 48002505  bl 0x82de0198
	ctx.lr = 0x82DDDC98;
	sub_82DE0198(ctx, base);
	// 82DDDC98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDDC9C: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDCA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDCA4: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDDCA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDCAC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDDCB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDDCB4: 4E800421  bctrl
	ctx.lr = 0x82DDDCB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDDCB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDDCBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDDCC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDCC4: 4800138D  bl 0x82ddf050
	ctx.lr = 0x82DDDCC8;
	sub_82DDF050(ctx, base);
	// 82DDDCC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDCCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDCD0: 4BFFF501  bl 0x82ddd1d0
	ctx.lr = 0x82DDDCD4;
	sub_82DDD1D0(ctx, base);
	// 82DDDCD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDCD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDDCDC: 483CA4DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDCE0 size=268
    let mut pc: u32 = 0x82DDDCE0;
    'dispatch: loop {
        match pc {
            0x82DDDCE0 => {
    //   block [0x82DDDCE0..0x82DDDDEC)
	// 82DDDCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDCE4: 483CA481  bl 0x831a8164
	ctx.lr = 0x82DDDCE8;
	sub_831A8130(ctx, base);
	// 82DDDCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDCEC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DDDCF0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDDCF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDDCF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDCFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDDD00: 4BFFF3C9  bl 0x82ddd0c8
	ctx.lr = 0x82DDDD04;
	sub_82DDD0C8(ctx, base);
	// 82DDDD04: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDD08: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDD0C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDDD10: 396B5CC8  addi r11, r11, 0x5cc8
	ctx.r[11].s64 = ctx.r[11].s64 + 23752;
	// 82DDDD14: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82DDDD18: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DDDD1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDD20: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DDDD24: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82DDDD28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDDD2C: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82DDDD30: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82DDDD34: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDD38: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 82DDDD3C: 4BFFCCFD  bl 0x82ddaa38
	ctx.lr = 0x82DDDD40;
	sub_82DDAA38(ctx, base);
	// 82DDDD40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDDD44: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82DDDD48: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82DDDD4C: 897E0564  lbz r11, 0x564(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1380 as u32) ) } as u64;
	// 82DDDD50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDDD54: 41820028  beq 0x82dddd7c
	if ctx.cr[0].eq {
	pc = 0x82DDDD7C; continue 'dispatch;
	}
	// 82DDDD58: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDD5C: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDDD60: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDDD64: 38CB5BC8  addi r6, r11, 0x5bc8
	ctx.r[6].s64 = ctx.r[11].s64 + 23496;
	// 82DDDD68: 38AA5CEC  addi r5, r10, 0x5cec
	ctx.r[5].s64 = ctx.r[10].s64 + 23788;
	// 82DDDD6C: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDDD70: 38E001F1  li r7, 0x1f1
	ctx.r[7].s64 = 497;
	// 82DDDD74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDDD78: 4BF04D09  bl 0x82ce2a80
	ctx.lr = 0x82DDDD7C;
	sub_82CE2A80(ctx, base);
	// 82DDDD7C: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDDD80: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDDD84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDDD88: 4BFC77F9  bl 0x82da5580
	ctx.lr = 0x82DDDD8C;
	sub_82DA5580(ctx, base);
	// 82DDDD8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDDD90: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDDD94: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DDDD98: 41820010  beq 0x82dddda8
	if ctx.cr[0].eq {
	pc = 0x82DDDDA8; continue 'dispatch;
	}
	// 82DDDD9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDDDA0: 480023F9  bl 0x82de0198
	ctx.lr = 0x82DDDDA4;
	sub_82DE0198(ctx, base);
	// 82DDDDA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDDDA8: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDDAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDDB0: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDDDB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDDB8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDDDBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDDDC0: 4E800421  bctrl
	ctx.lr = 0x82DDDDC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDDDC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDDDC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDDDCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDDD0: 48001281  bl 0x82ddf050
	ctx.lr = 0x82DDDDD4;
	sub_82DDF050(ctx, base);
	// 82DDDDD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDDD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDDDC: 4BFFF3F5  bl 0x82ddd1d0
	ctx.lr = 0x82DDDDE0;
	sub_82DDD1D0(ctx, base);
	// 82DDDDE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDDE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDDDE8: 483CA3CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDDF0 size=84
    let mut pc: u32 = 0x82DDDDF0;
    'dispatch: loop {
        match pc {
            0x82DDDDF0 => {
    //   block [0x82DDDDF0..0x82DDDE44)
	// 82DDDDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDDF4: 483CA379  bl 0x831a816c
	ctx.lr = 0x82DDDDF8;
	sub_831A8130(ctx, base);
	// 82DDDDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDDFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDE00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDDE04: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DDDE08: 4BFFF2C1  bl 0x82ddd0c8
	ctx.lr = 0x82DDDE0C;
	sub_82DDD0C8(ctx, base);
	// 82DDDE0C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDE10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDDE14: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 82DDDE18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDE1C: 807D0AB0  lwz r3, 0xab0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDE20: 4BFF28A1  bl 0x82dd06c0
	ctx.lr = 0x82DDDE24;
	sub_82DD06C0(ctx, base);
	// 82DDDE24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDDE28: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82DDDE2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDE30: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DDDE34: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 82DDDE38: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82DDDE3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDDE40: 483CA37C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDE48 size=164
    let mut pc: u32 = 0x82DDDE48;
    'dispatch: loop {
        match pc {
            0x82DDDE48 => {
    //   block [0x82DDDE48..0x82DDDEEC)
	// 82DDDE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDE4C: 483CA319  bl 0x831a8164
	ctx.lr = 0x82DDDE50;
	sub_831A8130(ctx, base);
	// 82DDDE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDE54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDE58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDDE5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDDE60: 4BFFF269  bl 0x82ddd0c8
	ctx.lr = 0x82DDDE64;
	sub_82DDD0C8(ctx, base);
	// 82DDDE64: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDE68: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82DDDE6C: 392B5D2C  addi r9, r11, 0x5d2c
	ctx.r[9].s64 = ctx.r[11].s64 + 23852;
	// 82DDDE70: 61488000  ori r8, r10, 0x8000
	ctx.r[8].u64 = ctx.r[10].u64 | 32768;
	// 82DDDE74: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DDDE78: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDDE7C: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDDE80: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DDDE84: 3B8B5E90  addi r28, r11, 0x5e90
	ctx.r[28].s64 = ctx.r[11].s64 + 24208;
	// 82DDDE88: 3B6A5BC8  addi r27, r10, 0x5bc8
	ctx.r[27].s64 = ctx.r[10].s64 + 23496;
	// 82DDDE8C: 41980020  blt cr6, 0x82dddeac
	if ctx.cr[6].lt {
	pc = 0x82DDDEAC; continue 'dispatch;
	}
	// 82DDDE90: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDE94: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDDE98: 38AB5D1C  addi r5, r11, 0x5d1c
	ctx.r[5].s64 = ctx.r[11].s64 + 23836;
	// 82DDDE9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDDEA0: 38E00241  li r7, 0x241
	ctx.r[7].s64 = 577;
	// 82DDDEA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDDEA8: 4BF04BD9  bl 0x82ce2a80
	ctx.lr = 0x82DDDEAC;
	sub_82CE2A80(ctx, base);
	// 82DDDEAC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82DDDEB0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDDEB4: 41980020  blt cr6, 0x82ddded4
	if ctx.cr[6].lt {
	pc = 0x82DDDED4; continue 'dispatch;
	}
	// 82DDDEB8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDEBC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDDEC0: 38AB5D08  addi r5, r11, 0x5d08
	ctx.r[5].s64 = ctx.r[11].s64 + 23816;
	// 82DDDEC4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDDEC8: 38E00242  li r7, 0x242
	ctx.r[7].s64 = 578;
	// 82DDDECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDDED0: 4BF04BB1  bl 0x82ce2a80
	ctx.lr = 0x82DDDED4;
	sub_82CE2A80(ctx, base);
	// 82DDDED4: 57CB801E  slwi r11, r30, 0x10
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDDED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDEDC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DDDEE0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDDEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDDEE8: 483CA2CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDEF0 size=1104
    let mut pc: u32 = 0x82DDDEF0;
    'dispatch: loop {
        match pc {
            0x82DDDEF0 => {
    //   block [0x82DDDEF0..0x82DDE340)
	// 82DDDEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDEF4: 483CA259  bl 0x831a814c
	ctx.lr = 0x82DDDEF8;
	sub_831A8130(ctx, base);
	// 82DDDEF8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDEFC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DDDF00: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDDF04: 4BFFFC5D  bl 0x82dddb60
	ctx.lr = 0x82DDDF08;
	sub_82DDDB60(ctx, base);
	// 82DDDF08: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDF0C: 3D4082DA  lis r10, -0x7d26
	ctx.r[10].s64 = -2099642368;
	// 82DDDF10: 396B5D50  addi r11, r11, 0x5d50
	ctx.r[11].s64 = ctx.r[11].s64 + 23888;
	// 82DDDF14: 3BAA6058  addi r29, r10, 0x6058
	ctx.r[29].s64 = ctx.r[10].s64 + 24664;
	// 82DDDF18: 91760000  stw r11, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDDF1C: 807E05D0  lwz r3, 0x5d0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1488 as u32) ) } as u64;
	// 82DDDF20: 83FE0AB0  lwz r31, 0xab0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDDF24: 839E0600  lwz r28, 0x600(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82DDDF28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDF2C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDDF30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDDF34: 4E800421  bctrl
	ctx.lr = 0x82DDDF38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDDF38: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DDDF3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDDF40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDDF44: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDDF48: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82DDDF4C: 4BFDD28D  bl 0x82dbb1d8
	ctx.lr = 0x82DDDF50;
	sub_82DBB1D8(ctx, base);
	// 82DDDF50: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DDDF54: 408200BC  bne 0x82dde010
	if !ctx.cr[0].eq {
	pc = 0x82DDE010; continue 'dispatch;
	}
	// 82DDDF58: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDDF5C: 80AB5E18  lwz r5, 0x5e18(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24088 as u32) ) } as u64;
	// 82DDDF60: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82DDDF64: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82DDDF68: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DDDF6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDDF70: 409A007C  bne cr6, 0x82dddfec
	if !ctx.cr[6].eq {
	pc = 0x82DDDFEC; continue 'dispatch;
	}
	// 82DDDF74: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DDDF78: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDDF7C: 4BFF0995  bl 0x82dce910
	ctx.lr = 0x82DDDF80;
	sub_82DCE910(ctx, base);
	// 82DDDF80: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDDF84: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDDF88: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDDF8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDF90: 4BFC75F1  bl 0x82da5580
	ctx.lr = 0x82DDDF94;
	sub_82DA5580(ctx, base);
	// 82DDDF94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDDF98: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDDF9C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDDFA0: 41820018  beq 0x82dddfb8
	if ctx.cr[0].eq {
	pc = 0x82DDDFB8; continue 'dispatch;
	}
	// 82DDDFA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDDFA8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DDDFAC: 4800217D  bl 0x82de0128
	ctx.lr = 0x82DDDFB0;
	sub_82DE0128(ctx, base);
	// 82DDDFB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDDFB4: 48000008  b 0x82dddfbc
	pc = 0x82DDDFBC; continue 'dispatch;
	// 82DDDFB8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDDFBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDFC0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDDFC4: 48003CC5  bl 0x82de1c88
	ctx.lr = 0x82DDDFC8;
	sub_82DE1C88(ctx, base);
	// 82DDDFC8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDDFCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDDFD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDFD4: 4800107D  bl 0x82ddf050
	ctx.lr = 0x82DDDFD8;
	sub_82DDF050(ctx, base);
	// 82DDDFD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDDFDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDDFE0: 4BFFF1F1  bl 0x82ddd1d0
	ctx.lr = 0x82DDDFE4;
	sub_82DDD1D0(ctx, base);
	// 82DDDFE4: 7FBBF92E  stwx r29, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82DDDFE8: 48000008  b 0x82dddff0
	pc = 0x82DDDFF0; continue 'dispatch;
	// 82DDDFEC: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDDFF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDDFF4: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDDFF8: 48003121  bl 0x82de1118
	ctx.lr = 0x82DDDFFC;
	sub_82DE1118(ctx, base);
	// 82DDDFFC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDE000: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDE004: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDE008: 48001049  bl 0x82ddf050
	ctx.lr = 0x82DDE00C;
	sub_82DDF050(ctx, base);
	// 82DDE00C: 48000300  b 0x82dde30c
	pc = 0x82DDE30C; continue 'dispatch;
	// 82DDE010: 3D6082DA  lis r11, -0x7d26
	ctx.r[11].s64 = -2099642368;
	// 82DDE014: 807E0600  lwz r3, 0x600(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82DDE018: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDE01C: 38AB6058  addi r5, r11, 0x6058
	ctx.r[5].s64 = ctx.r[11].s64 + 24664;
	// 82DDE020: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDE024: 4BFDD74D  bl 0x82dbb770
	ctx.lr = 0x82DDE028;
	sub_82DBB770(ctx, base);
	// 82DDE028: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE02C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE030: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE034: 3AAB5EC0  addi r21, r11, 0x5ec0
	ctx.r[21].s64 = ctx.r[11].s64 + 24256;
	// 82DDE038: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DDE03C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDE040: 419801EC  blt cr6, 0x82dde22c
	if ctx.cr[6].lt {
	pc = 0x82DDE22C; continue 'dispatch;
	}
	// 82DDE044: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE048: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE04C: 3B6B5FF0  addi r27, r11, 0x5ff0
	ctx.r[27].s64 = ctx.r[11].s64 + 24560;
	// 82DDE050: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE054: 393B0008  addi r9, r27, 8
	ctx.r[9].s64 = ctx.r[27].s64 + 8;
	// 82DDE058: 396B5E18  addi r11, r11, 0x5e18
	ctx.r[11].s64 = ctx.r[11].s64 + 24088;
	// 82DDE05C: 7D0AD82E  lwzx r8, r10, r27
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DDE060: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDE064: 55091838  slwi r9, r8, 3
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DDE068: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE06C: 7CA9582E  lwzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE070: 7F4A582E  lwzx r26, r10, r11
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE074: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82DDE078: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DDE07C: 397A0006  addi r11, r26, 6
	ctx.r[11].s64 = ctx.r[26].s64 + 6;
	// 82DDE080: 5577103A  slwi r23, r11, 2
	ctx.r[23].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82DDE084: 7D7CF82E  lwzx r11, r28, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DDE088: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDE08C: 7F17F82E  lwzx r24, r23, r31
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DDE090: 409A007C  bne cr6, 0x82dde10c
	if !ctx.cr[6].eq {
	pc = 0x82DDE10C; continue 'dispatch;
	}
	// 82DDE094: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DDE098: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDE09C: 4BFF0875  bl 0x82dce910
	ctx.lr = 0x82DDE0A0;
	sub_82DCE910(ctx, base);
	// 82DDE0A0: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDE0A4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DDE0A8: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDE0AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE0B0: 4BFC74D1  bl 0x82da5580
	ctx.lr = 0x82DDE0B4;
	sub_82DA5580(ctx, base);
	// 82DDE0B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE0B8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDE0BC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDE0C0: 41820018  beq 0x82dde0d8
	if ctx.cr[0].eq {
	pc = 0x82DDE0D8; continue 'dispatch;
	}
	// 82DDE0C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDE0C8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DDE0CC: 4800205D  bl 0x82de0128
	ctx.lr = 0x82DDE0D0;
	sub_82DE0128(ctx, base);
	// 82DDE0D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDE0D4: 48000008  b 0x82dde0dc
	pc = 0x82DDE0DC; continue 'dispatch;
	// 82DDE0D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDE0DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE0E0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE0E4: 48003BA5  bl 0x82de1c88
	ctx.lr = 0x82DDE0E8;
	sub_82DE1C88(ctx, base);
	// 82DDE0E8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DDE0EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE0F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE0F4: 48000F5D  bl 0x82ddf050
	ctx.lr = 0x82DDE0F8;
	sub_82DDF050(ctx, base);
	// 82DDE0F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE0FC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DDE100: 4BFFF0D1  bl 0x82ddd1d0
	ctx.lr = 0x82DDE104;
	sub_82DDD1D0(ctx, base);
	// 82DDE104: 7FBCF92E  stwx r29, r28, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82DDE108: 48000008  b 0x82dde110
	pc = 0x82DDE110; continue 'dispatch;
	// 82DDE10C: 832B001C  lwz r25, 0x1c(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDE110: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DDE114: 409A0080  bne cr6, 0x82dde194
	if !ctx.cr[6].eq {
	pc = 0x82DDE194; continue 'dispatch;
	}
	// 82DDE118: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DDE11C: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDE120: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DDE124: 4BFF07ED  bl 0x82dce910
	ctx.lr = 0x82DDE128;
	sub_82DCE910(ctx, base);
	// 82DDE128: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDE12C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDE130: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDE134: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE138: 4BFC7449  bl 0x82da5580
	ctx.lr = 0x82DDE13C;
	sub_82DA5580(ctx, base);
	// 82DDE13C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE140: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDE144: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDE148: 41820018  beq 0x82dde160
	if ctx.cr[0].eq {
	pc = 0x82DDE160; continue 'dispatch;
	}
	// 82DDE14C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDE150: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DDE154: 48001FD5  bl 0x82de0128
	ctx.lr = 0x82DDE158;
	sub_82DE0128(ctx, base);
	// 82DDE158: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDE15C: 48000008  b 0x82dde164
	pc = 0x82DDE164; continue 'dispatch;
	// 82DDE160: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDE164: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE168: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE16C: 48003B1D  bl 0x82de1c88
	ctx.lr = 0x82DDE170;
	sub_82DE1C88(ctx, base);
	// 82DDE170: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDE174: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE178: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE17C: 48000ED5  bl 0x82ddf050
	ctx.lr = 0x82DDE180;
	sub_82DDF050(ctx, base);
	// 82DDE180: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE184: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDE188: 4BFFF049  bl 0x82ddd1d0
	ctx.lr = 0x82DDE18C;
	sub_82DDD1D0(ctx, base);
	// 82DDE18C: 7FB7F92E  stwx r29, r23, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82DDE190: 48000008  b 0x82dde198
	pc = 0x82DDE198; continue 'dispatch;
	// 82DDE194: 8398001C  lwz r28, 0x1c(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDE198: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE19C: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDE1A0: 48002F79  bl 0x82de1118
	ctx.lr = 0x82DDE1A4;
	sub_82DE1118(ctx, base);
	// 82DDE1A4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DDE1A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDE1AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDE1B0: 48000EA1  bl 0x82ddf050
	ctx.lr = 0x82DDE1B4;
	sub_82DDF050(ctx, base);
	// 82DDE1B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE1B8: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 82DDE1BC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE1C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE1C4: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82DDE1C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE1CC: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DDE1D0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE1D4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE1D8: 917D0084  stw r11, 0x84(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DDE1DC: 48000E75  bl 0x82ddf050
	ctx.lr = 0x82DDE1E0;
	sub_82DDF050(ctx, base);
	// 82DDE1E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE1E4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DDE1E8: 4BFFEFE9  bl 0x82ddd1d0
	ctx.lr = 0x82DDE1EC;
	sub_82DDD1D0(ctx, base);
	// 82DDE1EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE1F0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE1F4: 48003A4D  bl 0x82de1c40
	ctx.lr = 0x82DDE1F8;
	sub_82DE1C40(ctx, base);
	// 82DDE1F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE1FC: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDE200: 48002F19  bl 0x82de1118
	ctx.lr = 0x82DDE204;
	sub_82DE1118(ctx, base);
	// 82DDE204: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDE208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDE20C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDE210: 48000E41  bl 0x82ddf050
	ctx.lr = 0x82DDE214;
	sub_82DDF050(ctx, base);
	// 82DDE214: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE218: 397B000C  addi r11, r27, 0xc
	ctx.r[11].s64 = ctx.r[27].s64 + 12;
	// 82DDE21C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE220: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DDE224: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE228: 480000DC  b 0x82dde304
	pc = 0x82DDE304; continue 'dispatch;
	// 82DDE22C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE230: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE234: 3B4B6010  addi r26, r11, 0x6010
	ctx.r[26].s64 = ctx.r[11].s64 + 24592;
	// 82DDE238: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE23C: 396B5E18  addi r11, r11, 0x5e18
	ctx.r[11].s64 = ctx.r[11].s64 + 24088;
	// 82DDE240: 7D4AD02E  lwzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DDE244: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE248: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE24C: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82DDE250: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82DDE254: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DDE258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDE25C: 409A007C  bne cr6, 0x82dde2d8
	if !ctx.cr[6].eq {
	pc = 0x82DDE2D8; continue 'dispatch;
	}
	// 82DDE260: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DDE264: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDE268: 4BFF06A9  bl 0x82dce910
	ctx.lr = 0x82DDE26C;
	sub_82DCE910(ctx, base);
	// 82DDE26C: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDE270: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDE274: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDE278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE27C: 4BFC7305  bl 0x82da5580
	ctx.lr = 0x82DDE280;
	sub_82DA5580(ctx, base);
	// 82DDE280: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE284: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDE288: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DDE28C: 41820018  beq 0x82dde2a4
	if ctx.cr[0].eq {
	pc = 0x82DDE2A4; continue 'dispatch;
	}
	// 82DDE290: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDE294: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DDE298: 48001E91  bl 0x82de0128
	ctx.lr = 0x82DDE29C;
	sub_82DE0128(ctx, base);
	// 82DDE29C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDE2A0: 48000008  b 0x82dde2a8
	pc = 0x82DDE2A8; continue 'dispatch;
	// 82DDE2A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDE2A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE2AC: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE2B0: 480039D9  bl 0x82de1c88
	ctx.lr = 0x82DDE2B4;
	sub_82DE1C88(ctx, base);
	// 82DDE2B4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDE2B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE2BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE2C0: 48000D91  bl 0x82ddf050
	ctx.lr = 0x82DDE2C4;
	sub_82DDF050(ctx, base);
	// 82DDE2C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE2C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDE2CC: 4BFFEF05  bl 0x82ddd1d0
	ctx.lr = 0x82DDE2D0;
	sub_82DDD1D0(ctx, base);
	// 82DDE2D0: 7FBBF92E  stwx r29, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82DDE2D4: 48000008  b 0x82dde2dc
	pc = 0x82DDE2DC; continue 'dispatch;
	// 82DDE2D8: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDE2DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE2E0: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDE2E4: 48002E35  bl 0x82de1118
	ctx.lr = 0x82DDE2E8;
	sub_82DE1118(ctx, base);
	// 82DDE2E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDE2EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDE2F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDE2F4: 48000D5D  bl 0x82ddf050
	ctx.lr = 0x82DDE2F8;
	sub_82DDF050(ctx, base);
	// 82DDE2F8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE2FC: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82DDE300: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDE304: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE308: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DDE30C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82DDE310: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDE318: 48000D39  bl 0x82ddf050
	ctx.lr = 0x82DDE31C;
	sub_82DDF050(ctx, base);
	// 82DDE31C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE320: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DDE324: 4BFFEEAD  bl 0x82ddd1d0
	ctx.lr = 0x82DDE328;
	sub_82DDD1D0(ctx, base);
	// 82DDE328: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE32C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE330: 48003911  bl 0x82de1c40
	ctx.lr = 0x82DDE334;
	sub_82DE1C40(ctx, base);
	// 82DDE334: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DDE338: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDE33C: 483C9E60  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE340 size=276
    let mut pc: u32 = 0x82DDE340;
    'dispatch: loop {
        match pc {
            0x82DDE340 => {
    //   block [0x82DDE340..0x82DDE454)
	// 82DDE340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE344: 483C9E1D  bl 0x831a8160
	ctx.lr = 0x82DDE348;
	sub_831A8130(ctx, base);
	// 82DDE348: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE34C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDE350: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DDE354: 4BFFF80D  bl 0x82dddb60
	ctx.lr = 0x82DDE358;
	sub_82DDDB60(ctx, base);
	// 82DDE358: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE35C: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDE360: 396B5D50  addi r11, r11, 0x5d50
	ctx.r[11].s64 = ctx.r[11].s64 + 23888;
	// 82DDE364: 394A5E18  addi r10, r10, 0x5e18
	ctx.r[10].s64 = ctx.r[10].s64 + 24088;
	// 82DDE368: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE36C: 80AA00A0  lwz r5, 0xa0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) } as u64;
	// 82DDE370: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82DDE374: 83BF0AB0  lwz r29, 0xab0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDE378: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82DDE37C: 7D7AE82E  lwzx r11, r26, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DDE380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDE384: 409A007C  bne cr6, 0x82dde400
	if !ctx.cr[6].eq {
	pc = 0x82DDE400; continue 'dispatch;
	}
	// 82DDE388: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DDE38C: 807D00AC  lwz r3, 0xac(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDE390: 4BFF0581  bl 0x82dce910
	ctx.lr = 0x82DDE394;
	sub_82DCE910(ctx, base);
	// 82DDE394: 839F05AC  lwz r28, 0x5ac(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDE398: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DDE39C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDE3A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDE3A4: 4BFC71DD  bl 0x82da5580
	ctx.lr = 0x82DDE3A8;
	sub_82DA5580(ctx, base);
	// 82DDE3A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE3AC: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDE3B0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DDE3B4: 41820018  beq 0x82dde3cc
	if ctx.cr[0].eq {
	pc = 0x82DDE3CC; continue 'dispatch;
	}
	// 82DDE3B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDE3BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE3C0: 48001D69  bl 0x82de0128
	ctx.lr = 0x82DDE3C4;
	sub_82DE0128(ctx, base);
	// 82DDE3C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDE3C8: 48000008  b 0x82dde3d0
	pc = 0x82DDE3D0; continue 'dispatch;
	// 82DDE3CC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DDE3D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDE3D4: 807D00A4  lwz r3, 0xa4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE3D8: 480038B1  bl 0x82de1c88
	ctx.lr = 0x82DDE3DC;
	sub_82DE1C88(ctx, base);
	// 82DDE3DC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDE3E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE3E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDE3E8: 48000C69  bl 0x82ddf050
	ctx.lr = 0x82DDE3EC;
	sub_82DDF050(ctx, base);
	// 82DDE3EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDE3F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDE3F4: 4BFFEDDD  bl 0x82ddd1d0
	ctx.lr = 0x82DDE3F8;
	sub_82DDD1D0(ctx, base);
	// 82DDE3F8: 7F9AE92E  stwx r28, r26, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	// 82DDE3FC: 48000008  b 0x82dde404
	pc = 0x82DDE404; continue 'dispatch;
	// 82DDE400: 836B001C  lwz r27, 0x1c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDE404: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE408: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDE40C: 48002D0D  bl 0x82de1118
	ctx.lr = 0x82DDE410;
	sub_82DE1118(ctx, base);
	// 82DDE410: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDE414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDE418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE41C: 48000C35  bl 0x82ddf050
	ctx.lr = 0x82DDE420;
	sub_82DDF050(ctx, base);
	// 82DDE420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE424: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDE428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE42C: 48000C25  bl 0x82ddf050
	ctx.lr = 0x82DDE430;
	sub_82DDF050(ctx, base);
	// 82DDE430: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE434: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDE438: 4BFFED99  bl 0x82ddd1d0
	ctx.lr = 0x82DDE43C;
	sub_82DDD1D0(ctx, base);
	// 82DDE43C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE440: 807D00A4  lwz r3, 0xa4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE444: 480037FD  bl 0x82de1c40
	ctx.lr = 0x82DDE448;
	sub_82DE1C40(ctx, base);
	// 82DDE448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDE44C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DDE450: 483C9D60  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE458 size=516
    let mut pc: u32 = 0x82DDE458;
    'dispatch: loop {
        match pc {
            0x82DDE458 => {
    //   block [0x82DDE458..0x82DDE65C)
	// 82DDE458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE45C: 483C9CF1  bl 0x831a814c
	ctx.lr = 0x82DDE460;
	sub_831A8130(ctx, base);
	// 82DDE460: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE464: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DDE468: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDE46C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DDE470: 4BFFF6F1  bl 0x82dddb60
	ctx.lr = 0x82DDE474;
	sub_82DDDB60(ctx, base);
	// 82DDE474: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE478: 3D4082DA  lis r10, -0x7d26
	ctx.r[10].s64 = -2099642368;
	// 82DDE47C: 396B5D50  addi r11, r11, 0x5d50
	ctx.r[11].s64 = ctx.r[11].s64 + 23888;
	// 82DDE480: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DDE484: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE488: 38AA6058  addi r5, r10, 0x6058
	ctx.r[5].s64 = ctx.r[10].s64 + 24664;
	// 82DDE48C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDE490: 807F0600  lwz r3, 0x600(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82DDE494: 833F0AB0  lwz r25, 0xab0(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDE498: 4BFDD2D9  bl 0x82dbb770
	ctx.lr = 0x82DDE49C;
	sub_82DBB770(ctx, base);
	// 82DDE49C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE4A0: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82DDE4A4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82DDE4A8: 3D008211  lis r8, -0x7def
	ctx.r[8].s64 = -2112815104;
	// 82DDE4AC: 2F1C000C  cmpwi cr6, r28, 0xc
	ctx.cr[6].compare_i32(ctx.r[28].s32, 12, &mut ctx.xer);
	// 82DDE4B0: 3B0B6010  addi r24, r11, 0x6010
	ctx.r[24].s64 = ctx.r[11].s64 + 24592;
	// 82DDE4B4: 3AEA5E90  addi r23, r10, 0x5e90
	ctx.r[23].s64 = ctx.r[10].s64 + 24208;
	// 82DDE4B8: 3AC99FF4  addi r22, r9, -0x600c
	ctx.r[22].s64 = ctx.r[9].s64 + -24588;
	// 82DDE4BC: 3AA85BC8  addi r21, r8, 0x5bc8
	ctx.r[21].s64 = ctx.r[8].s64 + 23496;
	// 82DDE4C0: 419A0044  beq cr6, 0x82dde504
	if ctx.cr[6].eq {
	pc = 0x82DDE504; continue 'dispatch;
	}
	// 82DDE4C4: 2F1C000D  cmpwi cr6, r28, 0xd
	ctx.cr[6].compare_i32(ctx.r[28].s32, 13, &mut ctx.xer);
	// 82DDE4C8: 419A0034  beq cr6, 0x82dde4fc
	if ctx.cr[6].eq {
	pc = 0x82DDE4FC; continue 'dispatch;
	}
	// 82DDE4CC: 2F1C000E  cmpwi cr6, r28, 0xe
	ctx.cr[6].compare_i32(ctx.r[28].s32, 14, &mut ctx.xer);
	// 82DDE4D0: 419A0024  beq cr6, 0x82dde4f4
	if ctx.cr[6].eq {
	pc = 0x82DDE4F4; continue 'dispatch;
	}
	// 82DDE4D4: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82DDE4D8: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82DDE4DC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82DDE4E0: 38E002F5  li r7, 0x2f5
	ctx.r[7].s64 = 757;
	// 82DDE4E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDE4E8: 4BF04599  bl 0x82ce2a80
	ctx.lr = 0x82DDE4EC;
	sub_82CE2A80(ctx, base);
	// 82DDE4EC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE4F0: 48000024  b 0x82dde514
	pc = 0x82DDE514; continue 'dispatch;
	// 82DDE4F4: 39580008  addi r10, r24, 8
	ctx.r[10].s64 = ctx.r[24].s64 + 8;
	// 82DDE4F8: 48000010  b 0x82dde508
	pc = 0x82DDE508; continue 'dispatch;
	// 82DDE4FC: 39580018  addi r10, r24, 0x18
	ctx.r[10].s64 = ctx.r[24].s64 + 24;
	// 82DDE500: 48000008  b 0x82dde508
	pc = 0x82DDE508; continue 'dispatch;
	// 82DDE504: 39580010  addi r10, r24, 0x10
	ctx.r[10].s64 = ctx.r[24].s64 + 16;
	// 82DDE508: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE50C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDE510: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDE514: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDE518: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDE51C: 394A5E18  addi r10, r10, 0x5e18
	ctx.r[10].s64 = ctx.r[10].s64 + 24088;
	// 82DDE520: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDE524: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 82DDE528: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82DDE52C: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDE530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDE534: 409A007C  bne cr6, 0x82dde5b0
	if !ctx.cr[6].eq {
	pc = 0x82DDE5B0; continue 'dispatch;
	}
	// 82DDE538: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DDE53C: 807900AC  lwz r3, 0xac(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDE540: 4BFF03D1  bl 0x82dce910
	ctx.lr = 0x82DDE544;
	sub_82DCE910(ctx, base);
	// 82DDE544: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDE548: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDE54C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDE550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDE554: 4BFC702D  bl 0x82da5580
	ctx.lr = 0x82DDE558;
	sub_82DA5580(ctx, base);
	// 82DDE558: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE55C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDE560: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DDE564: 41820018  beq 0x82dde57c
	if ctx.cr[0].eq {
	pc = 0x82DDE57C; continue 'dispatch;
	}
	// 82DDE568: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDE56C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DDE570: 48001BB9  bl 0x82de0128
	ctx.lr = 0x82DDE574;
	sub_82DE0128(ctx, base);
	// 82DDE574: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDE578: 48000008  b 0x82dde580
	pc = 0x82DDE580; continue 'dispatch;
	// 82DDE57C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDE580: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE584: 807900A4  lwz r3, 0xa4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE588: 48003701  bl 0x82de1c88
	ctx.lr = 0x82DDE58C;
	sub_82DE1C88(ctx, base);
	// 82DDE58C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDE590: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDE598: 48000AB9  bl 0x82ddf050
	ctx.lr = 0x82DDE59C;
	sub_82DDF050(ctx, base);
	// 82DDE59C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDE5A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE5A4: 4BFFEC2D  bl 0x82ddd1d0
	ctx.lr = 0x82DDE5A8;
	sub_82DDD1D0(ctx, base);
	// 82DDE5A8: 7FDAC92E  stwx r30, r26, r25
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32), ctx.r[30].u32) };
	// 82DDE5AC: 48000008  b 0x82dde5b4
	pc = 0x82DDE5B4; continue 'dispatch;
	// 82DDE5B0: 83AB001C  lwz r29, 0x1c(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDE5B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE5B8: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 82DDE5BC: 48002B5D  bl 0x82de1118
	ctx.lr = 0x82DDE5C0;
	sub_82DE1118(ctx, base);
	// 82DDE5C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDE5C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDE5C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE5CC: 48000A85  bl 0x82ddf050
	ctx.lr = 0x82DDE5D0;
	sub_82DDF050(ctx, base);
	// 82DDE5D0: 2F1C000C  cmpwi cr6, r28, 0xc
	ctx.cr[6].compare_i32(ctx.r[28].s32, 12, &mut ctx.xer);
	// 82DDE5D4: 419A0040  beq cr6, 0x82dde614
	if ctx.cr[6].eq {
	pc = 0x82DDE614; continue 'dispatch;
	}
	// 82DDE5D8: 2F1C000D  cmpwi cr6, r28, 0xd
	ctx.cr[6].compare_i32(ctx.r[28].s32, 13, &mut ctx.xer);
	// 82DDE5DC: 419A0030  beq cr6, 0x82dde60c
	if ctx.cr[6].eq {
	pc = 0x82DDE60C; continue 'dispatch;
	}
	// 82DDE5E0: 2F1C000E  cmpwi cr6, r28, 0xe
	ctx.cr[6].compare_i32(ctx.r[28].s32, 14, &mut ctx.xer);
	// 82DDE5E4: 419A0020  beq cr6, 0x82dde604
	if ctx.cr[6].eq {
	pc = 0x82DDE604; continue 'dispatch;
	}
	// 82DDE5E8: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82DDE5EC: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82DDE5F0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82DDE5F4: 38E00314  li r7, 0x314
	ctx.r[7].s64 = 788;
	// 82DDE5F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDE5FC: 4BF04485  bl 0x82ce2a80
	ctx.lr = 0x82DDE600;
	sub_82CE2A80(ctx, base);
	// 82DDE600: 48000028  b 0x82dde628
	pc = 0x82DDE628; continue 'dispatch;
	// 82DDE604: 3958000C  addi r10, r24, 0xc
	ctx.r[10].s64 = ctx.r[24].s64 + 12;
	// 82DDE608: 48000010  b 0x82dde618
	pc = 0x82DDE618; continue 'dispatch;
	// 82DDE60C: 3958001C  addi r10, r24, 0x1c
	ctx.r[10].s64 = ctx.r[24].s64 + 28;
	// 82DDE610: 48000008  b 0x82dde618
	pc = 0x82DDE618; continue 'dispatch;
	// 82DDE614: 39580014  addi r10, r24, 0x14
	ctx.r[10].s64 = ctx.r[24].s64 + 20;
	// 82DDE618: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDE61C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDE620: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDE624: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DDE628: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDE62C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDE630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE634: 48000A1D  bl 0x82ddf050
	ctx.lr = 0x82DDE638;
	sub_82DDF050(ctx, base);
	// 82DDE638: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE63C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDE640: 4BFFEB91  bl 0x82ddd1d0
	ctx.lr = 0x82DDE644;
	sub_82DDD1D0(ctx, base);
	// 82DDE644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE648: 807900A4  lwz r3, 0xa4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE64C: 480035F5  bl 0x82de1c40
	ctx.lr = 0x82DDE650;
	sub_82DE1C40(ctx, base);
	// 82DDE650: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDE654: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDE658: 483C9B44  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE660 size=60
    let mut pc: u32 = 0x82DDE660;
    'dispatch: loop {
        match pc {
            0x82DDE660 => {
    //   block [0x82DDE660..0x82DDE69C)
	// 82DDE660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE66C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE674: 4BFFF4ED  bl 0x82dddb60
	ctx.lr = 0x82DDE678;
	sub_82DDDB60(ctx, base);
	// 82DDE678: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE67C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE680: 396B5D50  addi r11, r11, 0x5d50
	ctx.r[11].s64 = ctx.r[11].s64 + 23888;
	// 82DDE684: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDE68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE6A0 size=76
    let mut pc: u32 = 0x82DDE6A0;
    'dispatch: loop {
        match pc {
            0x82DDE6A0 => {
    //   block [0x82DDE6A0..0x82DDE6EC)
	// 82DDE6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE6B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE6B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE6BC: 4BFFEA0D  bl 0x82ddd0c8
	ctx.lr = 0x82DDE6C0;
	sub_82DDD0C8(ctx, base);
	// 82DDE6C0: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE6C4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DDE6C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE6CC: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 82DDE6D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE6D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE6E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE6E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE6F0 size=132
    let mut pc: u32 = 0x82DDE6F0;
    'dispatch: loop {
        match pc {
            0x82DDE6F0 => {
    //   block [0x82DDE6F0..0x82DDE774)
	// 82DDE6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE6F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE6FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE708: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE70C: 4BFFE9BD  bl 0x82ddd0c8
	ctx.lr = 0x82DDE710;
	sub_82DDD0C8(ctx, base);
	// 82DDE710: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE714: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDE718: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 82DDE71C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE720: 419A0028  beq cr6, 0x82dde748
	if ctx.cr[6].eq {
	pc = 0x82DDE748; continue 'dispatch;
	}
	// 82DDE724: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE728: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDE72C: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDE730: 38CB5BC8  addi r6, r11, 0x5bc8
	ctx.r[6].s64 = ctx.r[11].s64 + 23496;
	// 82DDE734: 38AA5D74  addi r5, r10, 0x5d74
	ctx.r[5].s64 = ctx.r[10].s64 + 23924;
	// 82DDE738: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDE73C: 38E00353  li r7, 0x353
	ctx.r[7].s64 = 851;
	// 82DDE740: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDE744: 4BF0433D  bl 0x82ce2a80
	ctx.lr = 0x82DDE748;
	sub_82CE2A80(ctx, base);
	// 82DDE748: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDE74C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DDE750: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82DDE754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE758: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDE75C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE76C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE778 size=140
    let mut pc: u32 = 0x82DDE778;
    'dispatch: loop {
        match pc {
            0x82DDE778 => {
    //   block [0x82DDE778..0x82DDE804)
	// 82DDE778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE794: 4BFFE935  bl 0x82ddd0c8
	ctx.lr = 0x82DDE798;
	sub_82DDD0C8(ctx, base);
	// 82DDE798: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE79C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDE7A0: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 82DDE7A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE7A8: 4198000C  blt cr6, 0x82dde7b4
	if ctx.cr[6].lt {
	pc = 0x82DDE7B4; continue 'dispatch;
	}
	// 82DDE7AC: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82DDE7B0: 40990028  ble cr6, 0x82dde7d8
	if !ctx.cr[6].gt {
	pc = 0x82DDE7D8; continue 'dispatch;
	}
	// 82DDE7B4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE7B8: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDE7BC: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDE7C0: 38CB5BC8  addi r6, r11, 0x5bc8
	ctx.r[6].s64 = ctx.r[11].s64 + 23496;
	// 82DDE7C4: 38AA5D80  addi r5, r10, 0x5d80
	ctx.r[5].s64 = ctx.r[10].s64 + 23936;
	// 82DDE7C8: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDE7CC: 38E0035E  li r7, 0x35e
	ctx.r[7].s64 = 862;
	// 82DDE7D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDE7D4: 4BF042AD  bl 0x82ce2a80
	ctx.lr = 0x82DDE7D8;
	sub_82CE2A80(ctx, base);
	// 82DDE7D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDE7DC: 395E0021  addi r10, r30, 0x21
	ctx.r[10].s64 = ctx.r[30].s64 + 33;
	// 82DDE7E0: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82DDE7E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE7E8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDE7EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE7F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE7FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE808 size=88
    let mut pc: u32 = 0x82DDE808;
    'dispatch: loop {
        match pc {
            0x82DDE808 => {
    //   block [0x82DDE808..0x82DDE860)
	// 82DDE808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE81C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE820: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE824: 4BFFF1AD  bl 0x82ddd9d0
	ctx.lr = 0x82DDE828;
	sub_82DDD9D0(ctx, base);
	// 82DDE828: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDE82C: 41820018  beq 0x82dde844
	if ctx.cr[0].eq {
	pc = 0x82DDE844; continue 'dispatch;
	}
	// 82DDE830: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DDE834: 419A0010  beq cr6, 0x82dde844
	if ctx.cr[6].eq {
	pc = 0x82DDE844; continue 'dispatch;
	}
	// 82DDE838: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 82DDE83C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DDE840: 4BFC6EC9  bl 0x82da5708
	ctx.lr = 0x82DDE844;
	sub_82DA5708(ctx, base);
	// 82DDE844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE848: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE854: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE860 size=100
    let mut pc: u32 = 0x82DDE860;
    'dispatch: loop {
        match pc {
            0x82DDE860 => {
    //   block [0x82DDE860..0x82DDE8C4)
	// 82DDE860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE86C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE878: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDE87C: 4BFFE84D  bl 0x82ddd0c8
	ctx.lr = 0x82DDE880;
	sub_82DDD0C8(ctx, base);
	// 82DDE880: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE884: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDE888: 396B5DA0  addi r11, r11, 0x5da0
	ctx.r[11].s64 = ctx.r[11].s64 + 23968;
	// 82DDE88C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DDE890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE894: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE898: 817E058C  lwz r11, 0x58c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1420 as u32) ) } as u64;
	// 82DDE89C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDE8A0: 817E058C  lwz r11, 0x58c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1420 as u32) ) } as u64;
	// 82DDE8A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDE8A8: 917E058C  stw r11, 0x58c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1420 as u32), ctx.r[11].u32 ) };
	// 82DDE8AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE8B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE8BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE8C8 size=100
    let mut pc: u32 = 0x82DDE8C8;
    'dispatch: loop {
        match pc {
            0x82DDE8C8 => {
    //   block [0x82DDE8C8..0x82DDE92C)
	// 82DDE8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE8DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE8E0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDE8E4: 4BFFE7E5  bl 0x82ddd0c8
	ctx.lr = 0x82DDE8E8;
	sub_82DDD0C8(ctx, base);
	// 82DDE8E8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE8EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDE8F0: 396B5DA0  addi r11, r11, 0x5da0
	ctx.r[11].s64 = ctx.r[11].s64 + 23968;
	// 82DDE8F4: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DDE8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE8FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE900: 817E0590  lwz r11, 0x590(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1424 as u32) ) } as u64;
	// 82DDE904: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDE908: 817E0590  lwz r11, 0x590(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1424 as u32) ) } as u64;
	// 82DDE90C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDE910: 917E0590  stw r11, 0x590(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1424 as u32), ctx.r[11].u32 ) };
	// 82DDE914: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE920: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE930 size=100
    let mut pc: u32 = 0x82DDE930;
    'dispatch: loop {
        match pc {
            0x82DDE930 => {
    //   block [0x82DDE930..0x82DDE994)
	// 82DDE930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE93C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE948: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDE94C: 4BFFE77D  bl 0x82ddd0c8
	ctx.lr = 0x82DDE950;
	sub_82DDD0C8(ctx, base);
	// 82DDE950: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDE954: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDE958: 396B5DA0  addi r11, r11, 0x5da0
	ctx.r[11].s64 = ctx.r[11].s64 + 23968;
	// 82DDE95C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DDE960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE964: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE968: 817E0594  lwz r11, 0x594(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1428 as u32) ) } as u64;
	// 82DDE96C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDE970: 817E0594  lwz r11, 0x594(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1428 as u32) ) } as u64;
	// 82DDE974: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDE978: 917E0594  stw r11, 0x594(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 82DDE97C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE988: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE98C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DDE998 size=12
    let mut pc: u32 = 0x82DDE998;
    'dispatch: loop {
        match pc {
            0x82DDE998 => {
    //   block [0x82DDE998..0x82DDE9A4)
	// 82DDE998: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDE99C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDE9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE9A8 size=172
    let mut pc: u32 = 0x82DDE9A8;
    'dispatch: loop {
        match pc {
            0x82DDE9A8 => {
    //   block [0x82DDE9A8..0x82DDEA54)
	// 82DDE9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE9AC: 483C97B9  bl 0x831a8164
	ctx.lr = 0x82DDE9B0;
	sub_831A8130(ctx, base);
	// 82DDE9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE9B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE9B8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDE9BC: 895F00A4  lbz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE9C0: 897C00A4  lbz r11, 0xa4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DDE9C4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDE9C8: 419A000C  beq cr6, 0x82dde9d4
	if ctx.cr[6].eq {
	pc = 0x82DDE9D4; continue 'dispatch;
	}
	// 82DDE9CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDE9D0: 4800007C  b 0x82ddea4c
	pc = 0x82DDEA4C; continue 'dispatch;
	// 82DDE9D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE9D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DDE9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE9E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDE9E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDE9E8: 4E800421  bctrl
	ctx.lr = 0x82DDE9EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDE9EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DDE9F0: 41980058  blt cr6, 0x82ddea48
	if ctx.cr[6].lt {
	pc = 0x82DDEA48; continue 'dispatch;
	}
	// 82DDE9F4: 3BBF0099  addi r29, r31, 0x99
	ctx.r[29].s64 = ctx.r[31].s64 + 153;
	// 82DDE9F8: 7F7FE050  subf r27, r31, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82DDE9FC: 7D7BE8AE  lbzx r11, r27, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DDEA00: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEA04: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDEA08: 409AFFC4  bne cr6, 0x82dde9cc
	if !ctx.cr[6].eq {
	pc = 0x82DDE9CC; continue 'dispatch;
	}
	// 82DDEA0C: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82DDEA10: 7D5EFA14  add r10, r30, r31
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82DDEA14: 896B009E  lbz r11, 0x9e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(158 as u32) ) } as u64;
	// 82DDEA18: 894A009E  lbz r10, 0x9e(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(158 as u32) ) } as u64;
	// 82DDEA1C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDEA20: 409AFFAC  bne cr6, 0x82dde9cc
	if !ctx.cr[6].eq {
	pc = 0x82DDE9CC; continue 'dispatch;
	}
	// 82DDEA24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEA28: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDEA2C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DDEA30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDEA34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDEA38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEA3C: 4E800421  bctrl
	ctx.lr = 0x82DDEA40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEA40: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82DDEA44: 4099FFB8  ble cr6, 0x82dde9fc
	if !ctx.cr[6].gt {
	pc = 0x82DDE9FC; continue 'dispatch;
	}
	// 82DDEA48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDEA4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDEA50: 483C9764  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEA58 size=128
    let mut pc: u32 = 0x82DDEA58;
    'dispatch: loop {
        match pc {
            0x82DDEA58 => {
    //   block [0x82DDEA58..0x82DDEAD8)
	// 82DDEA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEA5C: 483C9711  bl 0x831a816c
	ctx.lr = 0x82DDEA60;
	sub_831A8130(ctx, base);
	// 82DDEA60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEA64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDEA68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDEA6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDEA70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEA74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDEA78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDEA7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDEA80: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDEA84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEA88: 4E800421  bctrl
	ctx.lr = 0x82DDEA8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEA8C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DDEA90: 419A0028  beq cr6, 0x82ddeab8
	if ctx.cr[6].eq {
	pc = 0x82DDEAB8; continue 'dispatch;
	}
	// 82DDEA94: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEA98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDEA9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDEAA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDEAA4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDEAA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEAAC: 4E800421  bctrl
	ctx.lr = 0x82DDEAB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEAB0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DDEAB4: 409A001C  bne cr6, 0x82ddead0
	if !ctx.cr[6].eq {
	pc = 0x82DDEAD0; continue 'dispatch;
	}
	// 82DDEAB8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDEABC: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DDEAC0: 4198FFB0  blt cr6, 0x82ddea70
	if ctx.cr[6].lt {
	pc = 0x82DDEA70; continue 'dispatch;
	}
	// 82DDEAC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDEAC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEACC: 483C96F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82DDEAD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDEAD4: 4BFFFFF4  b 0x82ddeac8
	pc = 0x82DDEAC8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEAD8 size=88
    let mut pc: u32 = 0x82DDEAD8;
    'dispatch: loop {
        match pc {
            0x82DDEAD8 => {
    //   block [0x82DDEAD8..0x82DDEB30)
	// 82DDEAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEADC: 483C9691  bl 0x831a816c
	ctx.lr = 0x82DDEAE0;
	sub_831A8130(ctx, base);
	// 82DDEAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEAE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDEAE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDEAEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDEAF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEAF4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDEAF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDEAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDEB00: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDEB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEB08: 4E800421  bctrl
	ctx.lr = 0x82DDEB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEB0C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DDEB10: 419A0008  beq cr6, 0x82ddeb18
	if ctx.cr[6].eq {
	pc = 0x82DDEB18; continue 'dispatch;
	}
	// 82DDEB14: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DDEB18: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDEB1C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DDEB20: 4198FFD0  blt cr6, 0x82ddeaf0
	if ctx.cr[6].lt {
	pc = 0x82DDEAF0; continue 'dispatch;
	}
	// 82DDEB24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDEB28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEB2C: 483C9690  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEB30 size=116
    let mut pc: u32 = 0x82DDEB30;
    'dispatch: loop {
        match pc {
            0x82DDEB30 => {
    //   block [0x82DDEB30..0x82DDEBA4)
	// 82DDEB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEB34: 483C9639  bl 0x831a816c
	ctx.lr = 0x82DDEB38;
	sub_831A8130(ctx, base);
	// 82DDEB38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEB3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDEB40: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDEB44: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDEB48: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEB4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDEB50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDEB54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDEB58: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDEB5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEB60: 4E800421  bctrl
	ctx.lr = 0x82DDEB64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEB64: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEB68: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DDEB6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDEB70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDEB74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDEB78: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDEB7C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DDEB80: 409A0008  bne cr6, 0x82ddeb88
	if !ctx.cr[6].eq {
	pc = 0x82DDEB88; continue 'dispatch;
	}
	// 82DDEB84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DDEB88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEB8C: 4E800421  bctrl
	ctx.lr = 0x82DDEB90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEB90: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDEB94: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DDEB98: 4198FFB0  blt cr6, 0x82ddeb48
	if ctx.cr[6].lt {
	pc = 0x82DDEB48; continue 'dispatch;
	}
	// 82DDEB9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEBA0: 483C961C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEBA8 size=52
    let mut pc: u32 = 0x82DDEBA8;
    'dispatch: loop {
        match pc {
            0x82DDEBA8 => {
    //   block [0x82DDEBA8..0x82DDEBDC)
	// 82DDEBA8: 89630080  lbz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DDEBAC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DDEBB0: 419A002C  beq cr6, 0x82ddebdc
	if ctx.cr[6].eq {
		sub_82DDEBDC(ctx, base);
		return;
	}
	// 82DDEBB4: 89630081  lbz r11, 0x81(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(129 as u32) ) } as u64;
	// 82DDEBB8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DDEBBC: 419A0020  beq cr6, 0x82ddebdc
	if ctx.cr[6].eq {
		sub_82DDEBDC(ctx, base);
		return;
	}
	// 82DDEBC0: 89630082  lbz r11, 0x82(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(130 as u32) ) } as u64;
	// 82DDEBC4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DDEBC8: 419A0014  beq cr6, 0x82ddebdc
	if ctx.cr[6].eq {
		sub_82DDEBDC(ctx, base);
		return;
	}
	// 82DDEBCC: 89630083  lbz r11, 0x83(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(131 as u32) ) } as u64;
	// 82DDEBD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDEBD4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DDEBD8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEBDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEBDC size=8
    let mut pc: u32 = 0x82DDEBDC;
    'dispatch: loop {
        match pc {
            0x82DDEBDC => {
    //   block [0x82DDEBDC..0x82DDEBE4)
	// 82DDEBDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDEBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEBE8 size=44
    let mut pc: u32 = 0x82DDEBE8;
    'dispatch: loop {
        match pc {
            0x82DDEBE8 => {
    //   block [0x82DDEBE8..0x82DDEC14)
	// 82DDEBE8: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DDEBEC: 1D430034  mulli r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 * 52;
	// 82DDEBF0: 396B2438  addi r11, r11, 0x2438
	ctx.r[11].s64 = ctx.r[11].s64 + 9272;
	// 82DDEBF4: 3D208211  lis r9, -0x7def
	ctx.r[9].s64 = -2112815104;
	// 82DDEBF8: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82DDEBFC: 39296270  addi r9, r9, 0x6270
	ctx.r[9].s64 = ctx.r[9].s64 + 25200;
	// 82DDEC00: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDEC04: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDEC08: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDEC0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEC10: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEC18 size=96
    let mut pc: u32 = 0x82DDEC18;
    'dispatch: loop {
        match pc {
            0x82DDEC18 => {
    //   block [0x82DDEC18..0x82DDEC78)
	// 82DDEC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEC1C: 483C9551  bl 0x831a816c
	ctx.lr = 0x82DDEC20;
	sub_831A8130(ctx, base);
	// 82DDEC20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEC24: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDEC28: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DDEC2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDEC30: 807D05AC  lwz r3, 0x5ac(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDEC34: 4BFC694D  bl 0x82da5580
	ctx.lr = 0x82DDEC38;
	sub_82DA5580(ctx, base);
	// 82DDEC38: 817D05AC  lwz r11, 0x5ac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DDEC3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEC40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDEC44: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DDEC48: 38A003C0  li r5, 0x3c0
	ctx.r[5].s64 = 960;
	// 82DDEC4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDEC50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDEC54: 483C98BD  bl 0x831a8510
	ctx.lr = 0x82DDEC58;
	sub_831A8510(ctx, base);
	// 82DDEC58: 817D0560  lwz r11, 0x560(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82DDEC5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDEC60: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDEC64: 817D0560  lwz r11, 0x560(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82DDEC68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDEC6C: 917D0560  stw r11, 0x560(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1376 as u32), ctx.r[11].u32 ) };
	// 82DDEC70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEC74: 483C9548  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEC78 size=544
    let mut pc: u32 = 0x82DDEC78;
    'dispatch: loop {
        match pc {
            0x82DDEC78 => {
    //   block [0x82DDEC78..0x82DDEE98)
	// 82DDEC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEC7C: 483C94DD  bl 0x831a8158
	ctx.lr = 0x82DDEC80;
	sub_831A8130(ctx, base);
	// 82DDEC80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEC84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEC88: 3CE08211  lis r7, -0x7def
	ctx.r[7].s64 = -2112815104;
	// 82DDEC8C: 3D008211  lis r8, -0x7def
	ctx.r[8].s64 = -2112815104;
	// 82DDEC90: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDEC94: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDEC98: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEC9C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDECA0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDECA4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DDECA8: 3B4763C8  addi r26, r7, 0x63c8
	ctx.r[26].s64 = ctx.r[7].s64 + 25544;
	// 82DDECAC: 3B2863A0  addi r25, r8, 0x63a0
	ctx.r[25].s64 = ctx.r[8].s64 + 25504;
	// 82DDECB0: 3B895E90  addi r28, r9, 0x5e90
	ctx.r[28].s64 = ctx.r[9].s64 + 24208;
	// 82DDECB4: 3B0A6394  addi r24, r10, 0x6394
	ctx.r[24].s64 = ctx.r[10].s64 + 25492;
	// 82DDECB8: 3B6B6330  addi r27, r11, 0x6330
	ctx.r[27].s64 = ctx.r[11].s64 + 25392;
	// 82DDECBC: 409900B8  ble cr6, 0x82dded74
	if !ctx.cr[6].gt {
	pc = 0x82DDED74; continue 'dispatch;
	}
	// 82DDECC0: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82DDECC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDECC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDECCC: 409A001C  bne cr6, 0x82ddece8
	if !ctx.cr[6].eq {
	pc = 0x82DDECE8; continue 'dispatch;
	}
	// 82DDECD0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDECD4: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82DDECD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDECDC: 38E00088  li r7, 0x88
	ctx.r[7].s64 = 136;
	// 82DDECE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDECE4: 4BF03D9D  bl 0x82ce2a80
	ctx.lr = 0x82DDECE8;
	sub_82CE2A80(ctx, base);
	// 82DDECE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDECEC: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDECF0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDECF4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDECF8: 419A001C  beq cr6, 0x82dded14
	if ctx.cr[6].eq {
	pc = 0x82DDED14; continue 'dispatch;
	}
	// 82DDECFC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDED00: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DDED04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDED08: 38E00089  li r7, 0x89
	ctx.r[7].s64 = 137;
	// 82DDED0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDED10: 4BF03D71  bl 0x82ce2a80
	ctx.lr = 0x82DDED14;
	sub_82CE2A80(ctx, base);
	// 82DDED14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDED18: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDED1C: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDED20: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DDED24: 419A0028  beq cr6, 0x82dded4c
	if ctx.cr[6].eq {
	pc = 0x82DDED4C; continue 'dispatch;
	}
	// 82DDED28: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDED2C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDED30: 419A001C  beq cr6, 0x82dded4c
	if ctx.cr[6].eq {
	pc = 0x82DDED4C; continue 'dispatch;
	}
	// 82DDED34: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDED38: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DDED3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDED40: 38E0008B  li r7, 0x8b
	ctx.r[7].s64 = 139;
	// 82DDED44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDED48: 4BF03D39  bl 0x82ce2a80
	ctx.lr = 0x82DDED4C;
	sub_82CE2A80(ctx, base);
	// 82DDED4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDED50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDED54: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDED58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDED5C: 4E800421  bctrl
	ctx.lr = 0x82DDED60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDED60: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDED64: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DDED68: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DDED6C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDED70: 4198FF54  blt cr6, 0x82ddecc4
	if ctx.cr[6].lt {
	pc = 0x82DDECC4; continue 'dispatch;
	}
	// 82DDED74: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDED78: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DDED7C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDED80: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDED84: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DDED88: 409900DC  ble cr6, 0x82ddee64
	if !ctx.cr[6].gt {
	pc = 0x82DDEE64; continue 'dispatch;
	}
	// 82DDED8C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82DDED90: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDED94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDED98: 409A001C  bne cr6, 0x82ddedb4
	if !ctx.cr[6].eq {
	pc = 0x82DDEDB4; continue 'dispatch;
	}
	// 82DDED9C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDEDA0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82DDEDA4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDEDA8: 38E00091  li r7, 0x91
	ctx.r[7].s64 = 145;
	// 82DDEDAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDEDB0: 4BF03CD1  bl 0x82ce2a80
	ctx.lr = 0x82DDEDB4;
	sub_82CE2A80(ctx, base);
	// 82DDEDB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEDB8: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDEDBC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDEDC0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDEDC4: 419A001C  beq cr6, 0x82ddede0
	if ctx.cr[6].eq {
	pc = 0x82DDEDE0; continue 'dispatch;
	}
	// 82DDEDC8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDEDCC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DDEDD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDEDD4: 38E00092  li r7, 0x92
	ctx.r[7].s64 = 146;
	// 82DDEDD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDEDDC: 4BF03CA5  bl 0x82ce2a80
	ctx.lr = 0x82DDEDE0;
	sub_82CE2A80(ctx, base);
	// 82DDEDE0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEDE4: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDEDE8: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEDEC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DDEDF0: 419A0028  beq cr6, 0x82ddee18
	if ctx.cr[6].eq {
	pc = 0x82DDEE18; continue 'dispatch;
	}
	// 82DDEDF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEDF8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDEDFC: 419A001C  beq cr6, 0x82ddee18
	if ctx.cr[6].eq {
	pc = 0x82DDEE18; continue 'dispatch;
	}
	// 82DDEE00: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDEE04: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DDEE08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDEE0C: 38E00094  li r7, 0x94
	ctx.r[7].s64 = 148;
	// 82DDEE10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDEE14: 4BF03C6D  bl 0x82ce2a80
	ctx.lr = 0x82DDEE18;
	sub_82CE2A80(ctx, base);
	// 82DDEE18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDEE20: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDEE24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEE28: 4E800421  bctrl
	ctx.lr = 0x82DDEE2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEE2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDEE30: 40820018  bne 0x82ddee48
	if !ctx.cr[0].eq {
	pc = 0x82DDEE48; continue 'dispatch;
	}
	// 82DDEE34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEE38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDEE3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDEE40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEE44: 4E800421  bctrl
	ctx.lr = 0x82DDEE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEE48: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDEE4C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DDEE50: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEE54: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DDEE58: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDEE5C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDEE60: 4198FF30  blt cr6, 0x82dded90
	if ctx.cr[6].lt {
	pc = 0x82DDED90; continue 'dispatch;
	}
	// 82DDEE64: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82DDEE68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDEE6C: 409A0020  bne cr6, 0x82ddee8c
	if !ctx.cr[6].eq {
	pc = 0x82DDEE8C; continue 'dispatch;
	}
	// 82DDEE70: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDEE74: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDEE78: 38AB6380  addi r5, r11, 0x6380
	ctx.r[5].s64 = ctx.r[11].s64 + 25472;
	// 82DDEE7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDEE80: 38E00099  li r7, 0x99
	ctx.r[7].s64 = 153;
	// 82DDEE84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDEE88: 4BF03BF9  bl 0x82ce2a80
	ctx.lr = 0x82DDEE8C;
	sub_82CE2A80(ctx, base);
	// 82DDEE8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDEE90: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDEE94: 483C9314  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEE98 size=28
    let mut pc: u32 = 0x82DDEE98;
    'dispatch: loop {
        match pc {
            0x82DDEE98 => {
    //   block [0x82DDEE98..0x82DDEEB4)
	// 82DDEE98: 3964003A  addi r11, r4, 0x3a
	ctx.r[11].s64 = ctx.r[4].s64 + 58;
	// 82DDEE9C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDEEA0: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DDEEA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDEEA8: 419A000C  beq cr6, 0x82ddeeb4
	if ctx.cr[6].eq {
		sub_82DDEEB4(ctx, base);
		return;
	}
	// 82DDEEAC: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDEEB0: 48000010  b 0x82ddeec0
	sub_82DDEEB4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEEB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEEB4 size=56
    let mut pc: u32 = 0x82DDEEB4;
    'dispatch: loop {
        match pc {
            0x82DDEEB4 => {
    //   block [0x82DDEEB4..0x82DDEEEC)
	// 82DDEEB4: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 82DDEEB8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDEEBC: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DDEEC0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DDEEC4: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 82DDEEC8: 396AA778  addi r11, r10, -0x5888
	ctx.r[11].s64 = ctx.r[10].s64 + -22664;
	// 82DDEECC: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 82DDEED0: 7D6958AE  lbzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDEED4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DDEED8: 419A0014  beq cr6, 0x82ddeeec
	if ctx.cr[6].eq {
		sub_82DDEEEC(ctx, base);
		return;
	}
	// 82DDEEDC: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82DDEEE0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DDEEE4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DDEEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEEEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEEEC size=8
    let mut pc: u32 = 0x82DDEEEC;
    'dispatch: loop {
        match pc {
            0x82DDEEEC => {
    //   block [0x82DDEEEC..0x82DDEEF4)
	// 82DDEEEC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DDEEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDEEF8 size=344
    let mut pc: u32 = 0x82DDEEF8;
    'dispatch: loop {
        match pc {
            0x82DDEEF8 => {
    //   block [0x82DDEEF8..0x82DDF050)
	// 82DDEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEEFC: 483C9269  bl 0x831a8164
	ctx.lr = 0x82DDEF00;
	sub_831A8130(ctx, base);
	// 82DDEF00: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDEF04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDEF08: 392A6328  addi r9, r10, 0x6328
	ctx.r[9].s64 = ctx.r[10].s64 + 25384;
	// 82DDEF0C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82DDEF10: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DDEF14: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82DDEF18: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDEF1C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DDEF20: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82DDEF24: 93E30038  stw r31, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 82DDEF28: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DDEF2C: 90830050  stw r4, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82DDEF30: 38C30098  addi r6, r3, 0x98
	ctx.r[6].s64 = ctx.r[3].s64 + 152;
	// 82DDEF34: 8149FFF8  lwz r10, -8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDEF38: 38A3009E  addi r5, r3, 0x9e
	ctx.r[5].s64 = ctx.r[3].s64 + 158;
	// 82DDEF3C: 91430080  stw r10, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DDEF40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDEF44: 8109FFF0  lwz r8, -0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82DDEF48: 910303B0  stw r8, 0x3b0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(944 as u32), ctx.r[8].u32 ) };
	// 82DDEF4C: 3D008331  lis r8, -0x7ccf
	ctx.r[8].s64 = -2093940736;
	// 82DDEF50: 7FC33D2C  stwbrx r30, r3, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[7].u32), (ctx.r[30].u32.swap_bytes())) };
	// 82DDEF54: 3BC82438  addi r30, r8, 0x2438
	ctx.r[30].s64 = ctx.r[8].s64 + 9272;
	// 82DDEF58: 83890004  lwz r28, 4(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDEF5C: 83A90000  lwz r29, 0(r9)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEF60: 99630098  stb r11, 0x98(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 82DDEF64: 9963009E  stb r11, 0x9e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(158 as u32), ctx.r[11].u8 ) };
	// 82DDEF68: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DDEF6C: 916300C8  stw r11, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82DDEF70: 390A000E  addi r8, r10, 0xe
	ctx.r[8].s64 = ctx.r[10].s64 + 14;
	// 82DDEF74: 38EA0014  addi r7, r10, 0x14
	ctx.r[7].s64 = ctx.r[10].s64 + 20;
	// 82DDEF78: 551B103A  slwi r27, r8, 2
	ctx.r[27].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82DDEF7C: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 82DDEF80: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DDEF84: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DDEF88: 7FFB192E  stwx r31, r27, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[3].u32), ctx.r[31].u32) };
	// 82DDEF8C: 7C87192E  stwx r4, r7, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 82DDEF90: 80E9FFF0  lwz r7, -0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82DDEF94: 7CE8192E  stwx r7, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[7].u32) };
	// 82DDEF98: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDEF9C: 1CE70034  mulli r7, r7, 0x34
	ctx.r[7].s64 = ctx.r[7].s64 * 52;
	// 82DDEFA0: 7CE7F02E  lwzx r7, r7, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DDEFA4: 54E7F7FF  rlwinm. r7, r7, 0x1e, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DDEFA8: 41820018  beq 0x82ddefc0
	if ctx.cr[0].eq {
	pc = 0x82DDEFC0; continue 'dispatch;
	}
	// 82DDEFAC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82DDEFB0: 409A000C  bne cr6, 0x82ddefbc
	if !ctx.cr[6].eq {
	pc = 0x82DDEFBC; continue 'dispatch;
	}
	// 82DDEFB4: 93830084  stw r28, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82DDEFB8: 48000008  b 0x82ddefc0
	pc = 0x82DDEFC0; continue 'dispatch;
	// 82DDEFBC: 7FA8192E  stwx r29, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[29].u32) };
	// 82DDEFC0: 390A0007  addi r8, r10, 7
	ctx.r[8].s64 = ctx.r[10].s64 + 7;
	// 82DDEFC4: 7D6651AE  stbx r11, r6, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82DDEFC8: 38EA0032  addi r7, r10, 0x32
	ctx.r[7].s64 = ctx.r[10].s64 + 50;
	// 82DDEFCC: 7D6551AE  stbx r11, r5, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82DDEFD0: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DDEFD4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DDEFD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DDEFDC: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82DDEFE0: 7D68192E  stwx r11, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82DDEFE4: 7D67192E  stwx r11, r7, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82DDEFE8: 4198FF88  blt cr6, 0x82ddef70
	if ctx.cr[6].lt {
	pc = 0x82DDEF70; continue 'dispatch;
	}
	// 82DDEFEC: 3943016C  addi r10, r3, 0x16c
	ctx.r[10].s64 = ctx.r[3].s64 + 364;
	// 82DDEFF0: 996300A4  stb r11, 0xa4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u8 ) };
	// 82DDEFF4: 38E300E8  addi r7, r3, 0xe8
	ctx.r[7].s64 = ctx.r[3].s64 + 232;
	// 82DDEFF8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DDEFFC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82DDF000: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDF004: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF008: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF00C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82DDF010: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF014: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDF018: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DDF01C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DDF020: 4082FFE0  bne 0x82ddf000
	if !ctx.cr[0].eq {
	pc = 0x82DDF000; continue 'dispatch;
	}
	// 82DDF024: 390303A0  addi r8, r3, 0x3a0
	ctx.r[8].s64 = ctx.r[3].s64 + 928;
	// 82DDF028: 39430384  addi r10, r3, 0x384
	ctx.r[10].s64 = ctx.r[3].s64 + 900;
	// 82DDF02C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DDF030: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82DDF034: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DDF038: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDF03C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDF040: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDF044: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DDF048: 4082FFE8  bne 0x82ddf030
	if !ctx.cr[0].eq {
	pc = 0x82DDF030; continue 'dispatch;
	}
	// 82DDF04C: 483C9168  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF050 size=120
    let mut pc: u32 = 0x82DDF050;
    'dispatch: loop {
        match pc {
            0x82DDF050 => {
    //   block [0x82DDF050..0x82DDF0C8)
	// 82DDF050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF054: 483C9119  bl 0x831a816c
	ctx.lr = 0x82DDF058;
	sub_831A8130(ctx, base);
	// 82DDF058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF05C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDF060: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDF064: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDF068: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 82DDF06C: 41980028  blt cr6, 0x82ddf094
	if ctx.cr[6].lt {
	pc = 0x82DDF094; continue 'dispatch;
	}
	// 82DDF070: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF074: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDF078: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDF07C: 38CB6330  addi r6, r11, 0x6330
	ctx.r[6].s64 = ctx.r[11].s64 + 25392;
	// 82DDF080: 38AA641C  addi r5, r10, 0x641c
	ctx.r[5].s64 = ctx.r[10].s64 + 25628;
	// 82DDF084: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDF088: 38E000FB  li r7, 0xfb
	ctx.r[7].s64 = 251;
	// 82DDF08C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDF090: 4BF039F1  bl 0x82ce2a80
	ctx.lr = 0x82DDF094;
	sub_82CE2A80(ctx, base);
	// 82DDF094: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82DDF098: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDF09C: 393F000E  addi r9, r31, 0xe
	ctx.r[9].s64 = ctx.r[31].s64 + 14;
	// 82DDF0A0: 391F0007  addi r8, r31, 7
	ctx.r[8].s64 = ctx.r[31].s64 + 7;
	// 82DDF0A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF0A8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DDF0AC: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DDF0B0: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82DDF0B4: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF0B8: 7D69F12E  stwx r11, r9, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 82DDF0BC: 7FA8F12E  stwx r29, r8, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 82DDF0C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDF0C4: 483C90F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDF0C8 size=980
    let mut pc: u32 = 0x82DDF0C8;
    'dispatch: loop {
        match pc {
            0x82DDF0C8 => {
    //   block [0x82DDF0C8..0x82DDF49C)
	// 82DDF0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF0CC: 483C9099  bl 0x831a8164
	ctx.lr = 0x82DDF0D0;
	sub_831A8130(ctx, base);
	// 82DDF0D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF0D4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF0D8: F8C100D8  std r6, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u64 ) };
	// 82DDF0DC: F8E100E0  std r7, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[7].u64 ) };
	// 82DDF0E0: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDF0E4: F90100E8  std r8, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[8].u64 ) };
	// 82DDF0E8: 390100E0  addi r8, r1, 0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + 224;
	// 82DDF0EC: F92100F0  std r9, 0xf0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[9].u64 ) };
	// 82DDF0F0: 392100D8  addi r9, r1, 0xd8
	ctx.r[9].s64 = ctx.r[1].s64 + 216;
	// 82DDF0F4: 394A5E10  addi r10, r10, 0x5e10
	ctx.r[10].s64 = ctx.r[10].s64 + 24080;
	// 82DDF0F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82DDF0FC: 83EB6318  lwz r31, 0x6318(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25368 as u32) ) } as u64;
	// 82DDF100: 396100E8  addi r11, r1, 0xe8
	ctx.r[11].s64 = ctx.r[1].s64 + 232;
	// 82DDF104: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82DDF108: 392100F0  addi r9, r1, 0xf0
	ctx.r[9].s64 = ctx.r[1].s64 + 240;
	// 82DDF10C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDF110: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82DDF114: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDF118: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82DDF11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DDF120: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDF124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DDF128: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DDF12C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDF130: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82DDF134: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DDF138: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DDF13C: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82DDF140: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DDF144: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DDF148: 41980080  blt cr6, 0x82ddf1c8
	if ctx.cr[6].lt {
	pc = 0x82DDF1C8; continue 'dispatch;
	}
	// 82DDF14C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DDF150: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF154: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DDF158: 419A004C  beq cr6, 0x82ddf1a4
	if ctx.cr[6].eq {
	pc = 0x82DDF1A4; continue 'dispatch;
	}
	// 82DDF15C: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF160: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF164: 836A0000  lwz r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF168: 7F1BE040  cmplw cr6, r27, r28
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DDF16C: 409A0018  bne cr6, 0x82ddf184
	if !ctx.cr[6].eq {
	pc = 0x82DDF184; continue 'dispatch;
	}
	// 82DDF170: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF174: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF178: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDF17C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDF180: 419A0008  beq cr6, 0x82ddf188
	if ctx.cr[6].eq {
	pc = 0x82DDF188; continue 'dispatch;
	}
	// 82DDF184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDF188: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF18C: 40820034  bne 0x82ddf1c0
	if !ctx.cr[0].eq {
	pc = 0x82DDF1C0; continue 'dispatch;
	}
	// 82DDF190: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DDF194: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82DDF198: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DDF19C: 4099FFB4  ble cr6, 0x82ddf150
	if !ctx.cr[6].gt {
	pc = 0x82DDF150; continue 'dispatch;
	}
	// 82DDF1A0: 48000028  b 0x82ddf1c8
	pc = 0x82DDF1C8; continue 'dispatch;
	// 82DDF1A4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DDF1A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDF1AC: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82DDF1B0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82DDF1B4: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82DDF1B8: 7D2551AE  stbx r9, r5, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 82DDF1BC: 4800000C  b 0x82ddf1c8
	pc = 0x82DDF1C8; continue 'dispatch;
	// 82DDF1C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DDF1C4: 7D2559AE  stbx r9, r5, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82DDF1C8: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82DDF1CC: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DDF1D0: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82DDF1D4: 4198FF6C  blt cr6, 0x82ddf140
	if ctx.cr[6].lt {
	pc = 0x82DDF140; continue 'dispatch;
	}
	// 82DDF1D8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DDF1DC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DDF1E0: 4099008C  ble cr6, 0x82ddf26c
	if !ctx.cr[6].gt {
	pc = 0x82DDF26C; continue 'dispatch;
	}
	// 82DDF1E4: 3866FFFF  addi r3, r6, -1
	ctx.r[3].s64 = ctx.r[6].s64 + -1;
	// 82DDF1E8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DDF1EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DDF1F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDF1F4: 4099006C  ble cr6, 0x82ddf260
	if !ctx.cr[6].gt {
	pc = 0x82DDF260; continue 'dispatch;
	}
	// 82DDF1F8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDF1FC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF200: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF204: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF208: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF20C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DDF210: 409A000C  bne cr6, 0x82ddf21c
	if !ctx.cr[6].eq {
	pc = 0x82DDF21C; continue 'dispatch;
	}
	// 82DDF214: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF218: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF21C: 7D074010  subfc r8, r7, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[7].u32;
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82DDF220: 7D084110  subfe r8, r8, r8
	let x = (!ctx.r[8].u32);
	let y = ctx.r[8].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82DDF224: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82DDF228: 5508063F  clrlwi. r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF22C: 41820024  beq 0x82ddf250
	if ctx.cr[0].eq {
	pc = 0x82DDF250; continue 'dispatch;
	}
	// 82DDF230: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82DDF234: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDF238: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF23C: 7D654214  add r11, r5, r8
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82DDF240: 7D2540AE  lbzx r9, r5, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DDF244: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DDF248: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 82DDF24C: 7CE541AE  stbx r7, r5, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u8) };
	// 82DDF250: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82DDF254: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DDF258: 7F051800  cmpw cr6, r5, r3
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82DDF25C: 4198FFA0  blt cr6, 0x82ddf1fc
	if ctx.cr[6].lt {
	pc = 0x82DDF1FC; continue 'dispatch;
	}
	// 82DDF260: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDF264: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82DDF268: 4082FF84  bne 0x82ddf1ec
	if !ctx.cr[0].eq {
	pc = 0x82DDF1EC; continue 'dispatch;
	}
	// 82DDF26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDF270: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DDF274: 40990024  ble cr6, 0x82ddf298
	if !ctx.cr[6].gt {
	pc = 0x82DDF298; continue 'dispatch;
	}
	// 82DDF278: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 82DDF27C: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 82DDF280: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82DDF284: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDF288: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDF28C: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DDF290: 7D0A49AE  stbx r8, r10, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82DDF294: 4198FFE4  blt cr6, 0x82ddf278
	if ctx.cr[6].lt {
	pc = 0x82DDF278; continue 'dispatch;
	}
	// 82DDF298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDF29C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DDF2A0: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 82DDF2A4: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82DDF2A8: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDF2AC: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDF2B0: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82DDF2B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDF2B8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82DDF2BC: 4198FFE0  blt cr6, 0x82ddf29c
	if ctx.cr[6].lt {
	pc = 0x82DDF29C; continue 'dispatch;
	}
	// 82DDF2C0: 817E03B8  lwz r11, 0x3b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(952 as u32) ) } as u64;
	// 82DDF2C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDF2C8: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 82DDF2CC: 896B0571  lbz r11, 0x571(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1393 as u32) ) } as u64;
	// 82DDF2D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDF2D4: 418200DC  beq 0x82ddf3b0
	if ctx.cr[0].eq {
	pc = 0x82DDF3B0; continue 'dispatch;
	}
	// 82DDF2D8: 419A00B0  beq cr6, 0x82ddf388
	if ctx.cr[6].eq {
	pc = 0x82DDF388; continue 'dispatch;
	}
	// 82DDF2DC: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 82DDF2E0: 419A008C  beq cr6, 0x82ddf36c
	if ctx.cr[6].eq {
	pc = 0x82DDF36C; continue 'dispatch;
	}
	// 82DDF2E4: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 82DDF2E8: 419A0060  beq cr6, 0x82ddf348
	if ctx.cr[6].eq {
	pc = 0x82DDF348; continue 'dispatch;
	}
	// 82DDF2EC: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82DDF2F0: 419A002C  beq cr6, 0x82ddf31c
	if ctx.cr[6].eq {
	pc = 0x82DDF31C; continue 'dispatch;
	}
	// 82DDF2F4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF2F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82DDF2FC: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDF300: 38CB6330  addi r6, r11, 0x6330
	ctx.r[6].s64 = ctx.r[11].s64 + 25392;
	// 82DDF304: 38AA9FF4  addi r5, r10, -0x600c
	ctx.r[5].s64 = ctx.r[10].s64 + -24588;
	// 82DDF308: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDF30C: 38E00181  li r7, 0x181
	ctx.r[7].s64 = 385;
	// 82DDF310: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDF314: 4BF0376D  bl 0x82ce2a80
	ctx.lr = 0x82DDF318;
	sub_82CE2A80(ctx, base);
	// 82DDF318: 48000084  b 0x82ddf39c
	pc = 0x82DDF39C; continue 'dispatch;
	// 82DDF31C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DDF320: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DDF324: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DDF328: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF32C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF330: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF334: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DDF338: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DDF33C: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DDF340: 4BFEF931  bl 0x82dcec70
	ctx.lr = 0x82DDF344;
	sub_82DCEC70(ctx, base);
	// 82DDF344: 48000054  b 0x82ddf398
	pc = 0x82DDF398; continue 'dispatch;
	// 82DDF348: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DDF34C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DDF350: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF354: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF358: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF35C: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DDF360: E8890000  ld r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DDF364: 4BFEF8ED  bl 0x82dcec50
	ctx.lr = 0x82DDF368;
	sub_82DCEC50(ctx, base);
	// 82DDF368: 48000030  b 0x82ddf398
	pc = 0x82DDF398; continue 'dispatch;
	// 82DDF36C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DDF370: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF374: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF378: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF37C: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DDF380: 4BFEF8B1  bl 0x82dcec30
	ctx.lr = 0x82DDF384;
	sub_82DCEC30(ctx, base);
	// 82DDF384: 48000014  b 0x82ddf398
	pc = 0x82DDF398; continue 'dispatch;
	// 82DDF388: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF38C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF390: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF394: 4BFEF87D  bl 0x82dcec10
	ctx.lr = 0x82DDF398;
	sub_82DCEC10(ctx, base);
	// 82DDF398: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF39C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDF3A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDF3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDF3A8: 4BFDFD41  bl 0x82dbf0e8
	ctx.lr = 0x82DDF3AC;
	sub_82DBF0E8(ctx, base);
	// 82DDF3AC: 480000D8  b 0x82ddf484
	pc = 0x82DDF484; continue 'dispatch;
	// 82DDF3B0: 419A00B0  beq cr6, 0x82ddf460
	if ctx.cr[6].eq {
	pc = 0x82DDF460; continue 'dispatch;
	}
	// 82DDF3B4: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 82DDF3B8: 419A008C  beq cr6, 0x82ddf444
	if ctx.cr[6].eq {
	pc = 0x82DDF444; continue 'dispatch;
	}
	// 82DDF3BC: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 82DDF3C0: 419A0060  beq cr6, 0x82ddf420
	if ctx.cr[6].eq {
	pc = 0x82DDF420; continue 'dispatch;
	}
	// 82DDF3C4: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82DDF3C8: 419A002C  beq cr6, 0x82ddf3f4
	if ctx.cr[6].eq {
	pc = 0x82DDF3F4; continue 'dispatch;
	}
	// 82DDF3CC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF3D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82DDF3D4: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDF3D8: 38CB6330  addi r6, r11, 0x6330
	ctx.r[6].s64 = ctx.r[11].s64 + 25392;
	// 82DDF3DC: 38AA9FF4  addi r5, r10, -0x600c
	ctx.r[5].s64 = ctx.r[10].s64 + -24588;
	// 82DDF3E0: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDF3E4: 38E0019A  li r7, 0x19a
	ctx.r[7].s64 = 410;
	// 82DDF3E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDF3EC: 4BF03695  bl 0x82ce2a80
	ctx.lr = 0x82DDF3F0;
	sub_82CE2A80(ctx, base);
	// 82DDF3F0: 48000084  b 0x82ddf474
	pc = 0x82DDF474; continue 'dispatch;
	// 82DDF3F4: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DDF3F8: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DDF3FC: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DDF400: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF404: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF408: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF40C: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DDF410: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DDF414: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DDF418: 4BFF1199  bl 0x82dd05b0
	ctx.lr = 0x82DDF41C;
	sub_82DD05B0(ctx, base);
	// 82DDF41C: 48000054  b 0x82ddf470
	pc = 0x82DDF470; continue 'dispatch;
	// 82DDF420: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DDF424: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DDF428: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF42C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF430: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF434: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DDF438: E8890000  ld r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DDF43C: 4BFF1155  bl 0x82dd0590
	ctx.lr = 0x82DDF440;
	sub_82DD0590(ctx, base);
	// 82DDF440: 48000030  b 0x82ddf470
	pc = 0x82DDF470; continue 'dispatch;
	// 82DDF444: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DDF448: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF44C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF450: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF454: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DDF458: 4BFF1119  bl 0x82dd0570
	ctx.lr = 0x82DDF45C;
	sub_82DD0570(ctx, base);
	// 82DDF45C: 48000014  b 0x82ddf470
	pc = 0x82DDF470; continue 'dispatch;
	// 82DDF460: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF464: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF468: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDF46C: 4BFF10E5  bl 0x82dd0550
	ctx.lr = 0x82DDF470;
	sub_82DD0550(ctx, base);
	// 82DDF470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF474: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDF478: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDF47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDF480: 4BFFFBD1  bl 0x82ddf050
	ctx.lr = 0x82DDF484;
	sub_82DDF050(ctx, base);
	// 82DDF484: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82DDF488: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDF48C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF490: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82DDF494: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DDF498: 483C8D1C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF4A0 size=160
    let mut pc: u32 = 0x82DDF4A0;
    'dispatch: loop {
        match pc {
            0x82DDF4A0 => {
    //   block [0x82DDF4A0..0x82DDF540)
	// 82DDF4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF4A4: 483C8CC5  bl 0x831a8168
	ctx.lr = 0x82DDF4A8;
	sub_831A8130(ctx, base);
	// 82DDF4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF4AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDF4B0: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF4B4: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DDF4B8: 917C00E4  stw r11, 0xe4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDF4BC: 4BFFD0A5  bl 0x82ddc560
	ctx.lr = 0x82DDF4C0;
	sub_82DDC560(ctx, base);
	// 82DDF4C0: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF4C4: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF4C8: 41820070  beq 0x82ddf538
	if ctx.cr[0].eq {
	pc = 0x82DDF538; continue 'dispatch;
	}
	// 82DDF4CC: 817C03B8  lwz r11, 0x3b8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(952 as u32) ) } as u64;
	// 82DDF4D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDF4D4: 816B0AB0  lwz r11, 0xab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDF4D8: 83EB0060  lwz r31, 0x60(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDF4DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF4E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF4E4: 419A0054  beq cr6, 0x82ddf538
	if ctx.cr[6].eq {
	pc = 0x82DDF538; continue 'dispatch;
	}
	// 82DDF4E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DDF4EC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDF4F0: 40980010  bge cr6, 0x82ddf500
	if !ctx.cr[6].lt {
	pc = 0x82DDF500; continue 'dispatch;
	}
	// 82DDF4F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF4F8: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DDF4FC: 48000010  b 0x82ddf50c
	pc = 0x82DDF50C; continue 'dispatch;
	// 82DDF500: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDF508: 4BFFD311  bl 0x82ddc818
	ctx.lr = 0x82DDF50C;
	sub_82DDC818(ctx, base);
	// 82DDF50C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF510: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DDF514: 409A0010  bne cr6, 0x82ddf524
	if !ctx.cr[6].eq {
	pc = 0x82DDF524; continue 'dispatch;
	}
	// 82DDF518: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDF520: 4BFFD239  bl 0x82ddc758
	ctx.lr = 0x82DDF524;
	sub_82DDC758(ctx, base);
	// 82DDF524: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF528: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDF52C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DDF530: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDF534: 4198FFC0  blt cr6, 0x82ddf4f4
	if ctx.cr[6].lt {
	pc = 0x82DDF4F4; continue 'dispatch;
	}
	// 82DDF538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDF53C: 483C8C7C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF540 size=92
    let mut pc: u32 = 0x82DDF540;
    'dispatch: loop {
        match pc {
            0x82DDF540 => {
    //   block [0x82DDF540..0x82DDF59C)
	// 82DDF540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDF548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDF54C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF554: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DDF558: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 82DDF55C: 409A0018  bne cr6, 0x82ddf574
	if !ctx.cr[6].eq {
	pc = 0x82DDF574; continue 'dispatch;
	}
	// 82DDF560: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 82DDF564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDF568: 816B0AB0  lwz r11, 0xab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDF56C: 806B00AC  lwz r3, 0xac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF570: 4BFEEF69  bl 0x82dce4d8
	ctx.lr = 0x82DDF574;
	sub_82DCE4D8(ctx, base);
	// 82DDF574: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDF57C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DDF580: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDF584: 4BFFCFDD  bl 0x82ddc560
	ctx.lr = 0x82DDF588;
	sub_82DDC560(ctx, base);
	// 82DDF588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDF58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDF590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDF594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDF598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF5A0 size=56
    let mut pc: u32 = 0x82DDF5A0;
    'dispatch: loop {
        match pc {
            0x82DDF5A0 => {
    //   block [0x82DDF5A0..0x82DDF5D8)
	// 82DDF5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDF5A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDF5AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF5B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF5B4: 4BFFCFAD  bl 0x82ddc560
	ctx.lr = 0x82DDF5B8;
	sub_82DDC560(ctx, base);
	// 82DDF5B8: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF5BC: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DDF5C0: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDF5C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDF5C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDF5CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDF5D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDF5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF5D8 size=64
    let mut pc: u32 = 0x82DDF5D8;
    'dispatch: loop {
        match pc {
            0x82DDF5D8 => {
    //   block [0x82DDF5D8..0x82DDF618)
	// 82DDF5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDF5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF5E4: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF5E8: 2F0B0089  cmpwi cr6, r11, 0x89
	ctx.cr[6].compare_i32(ctx.r[11].s32, 137, &mut ctx.xer);
	// 82DDF5EC: 409A0014  bne cr6, 0x82ddf600
	if !ctx.cr[6].eq {
	pc = 0x82DDF600; continue 'dispatch;
	}
	// 82DDF5F0: 4BFF21A1  bl 0x82dd1790
	ctx.lr = 0x82DDF5F4;
	sub_82DD1790(ctx, base);
	// 82DDF5F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF5F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDF5FC: 40820008  bne 0x82ddf604
	if !ctx.cr[0].eq {
	pc = 0x82DDF604; continue 'dispatch;
	}
	// 82DDF600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDF604: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DDF608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDF60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDF610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDF614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF618 size=36
    let mut pc: u32 = 0x82DDF618;
    'dispatch: loop {
        match pc {
            0x82DDF618 => {
    //   block [0x82DDF618..0x82DDF63C)
	// 82DDF618: 816303BC  lwz r11, 0x3bc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(956 as u32) ) } as u64;
	// 82DDF61C: 81440868  lwz r10, 0x868(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2152 as u32) ) } as u64;
	// 82DDF620: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF624: 40800008  bge 0x82ddf62c
	if !ctx.cr[0].lt {
	pc = 0x82DDF62C; continue 'dispatch;
	}
	// 82DDF628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDF62C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DDF630: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DDF634: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DDF638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF640 size=28
    let mut pc: u32 = 0x82DDF640;
    'dispatch: loop {
        match pc {
            0x82DDF640 => {
    //   block [0x82DDF640..0x82DDF65C)
	// 82DDF640: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DDF644: 409A0018  bne cr6, 0x82ddf65c
	if !ctx.cr[6].eq {
		sub_82DDF65C(ctx, base);
		return;
	}
	// 82DDF648: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF64C: 81430080  lwz r10, 0x80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DDF650: 396B6318  addi r11, r11, 0x6318
	ctx.r[11].s64 = ctx.r[11].s64 + 25368;
	// 82DDF654: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF658: 48000018  b 0x82ddf670
	sub_82DDF65C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF65C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF65C size=40
    let mut pc: u32 = 0x82DDF65C;
    'dispatch: loop {
        match pc {
            0x82DDF65C => {
    //   block [0x82DDF65C..0x82DDF684)
	// 82DDF65C: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82DDF660: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDF664: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DDF668: 816A6318  lwz r11, 0x6318(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25368 as u32) ) } as u64;
	// 82DDF66C: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DDF670: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DDF674: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DDF678: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DDF67C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DDF680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF688 size=52
    let mut pc: u32 = 0x82DDF688;
    'dispatch: loop {
        match pc {
            0x82DDF688 => {
    //   block [0x82DDF688..0x82DDF6BC)
	// 82DDF688: 81640AB0  lwz r11, 0xab0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDF68C: 896B0860  lbz r11, 0x860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2144 as u32) ) } as u64;
	// 82DDF690: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDF694: 4182008C  beq 0x82ddf720
	if ctx.cr[0].eq {
		sub_82DDF720(ctx, base);
		return;
	}
	// 82DDF698: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 82DDF69C: 41990034  bgt cr6, 0x82ddf6d0
	if ctx.cr[6].gt {
		sub_82DDF6D0(ctx, base);
		return;
	}
	// 82DDF6A0: 2F030013  cmpwi cr6, r3, 0x13
	ctx.cr[6].compare_i32(ctx.r[3].s32, 19, &mut ctx.xer);
	// 82DDF6A4: 40980054  bge cr6, 0x82ddf6f8
	if !ctx.cr[6].lt {
		sub_82DDF6D0(ctx, base);
		return;
	}
	// 82DDF6A8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82DDF6AC: 419A004C  beq cr6, 0x82ddf6f8
	if ctx.cr[6].eq {
		sub_82DDF6D0(ctx, base);
		return;
	}
	// 82DDF6B0: 40990068  ble cr6, 0x82ddf718
	if !ctx.cr[6].gt {
		sub_82DDF718(ctx, base);
		return;
	}
	// 82DDF6B4: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82DDF6B8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF6BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF6BC size=16
    let mut pc: u32 = 0x82DDF6BC;
    'dispatch: loop {
        match pc {
            0x82DDF6BC => {
    //   block [0x82DDF6BC..0x82DDF6CC)
	// 82DDF6BC: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82DDF6C0: 419A0040  beq cr6, 0x82ddf700
	if ctx.cr[6].eq {
		sub_82DDF700(ctx, base);
		return;
	}
	// 82DDF6C4: 2F030009  cmpwi cr6, r3, 9
	ctx.cr[6].compare_i32(ctx.r[3].s32, 9, &mut ctx.xer);
	// 82DDF6C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF6CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF6CC size=4
    let mut pc: u32 = 0x82DDF6CC;
    'dispatch: loop {
        match pc {
            0x82DDF6CC => {
    //   block [0x82DDF6CC..0x82DDF6D0)
	// 82DDF6CC: 4800004C  b 0x82ddf718
	sub_82DDF718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF6D0 size=48
    let mut pc: u32 = 0x82DDF6D0;
    'dispatch: loop {
        match pc {
            0x82DDF6D0 => {
    //   block [0x82DDF6D0..0x82DDF700)
	// 82DDF6D0: 2F030021  cmpwi cr6, r3, 0x21
	ctx.cr[6].compare_i32(ctx.r[3].s32, 33, &mut ctx.xer);
	// 82DDF6D4: 41990034  bgt cr6, 0x82ddf708
	if ctx.cr[6].gt {
		sub_82DDF708(ctx, base);
		return;
	}
	// 82DDF6D8: 2F030020  cmpwi cr6, r3, 0x20
	ctx.cr[6].compare_i32(ctx.r[3].s32, 32, &mut ctx.xer);
	// 82DDF6DC: 4098001C  bge cr6, 0x82ddf6f8
	if !ctx.cr[6].lt {
	pc = 0x82DDF6F8; continue 'dispatch;
	}
	// 82DDF6E0: 2F030015  cmpwi cr6, r3, 0x15
	ctx.cr[6].compare_i32(ctx.r[3].s32, 21, &mut ctx.xer);
	// 82DDF6E4: 41980034  blt cr6, 0x82ddf718
	if ctx.cr[6].lt {
		sub_82DDF718(ctx, base);
		return;
	}
	// 82DDF6E8: 2F030016  cmpwi cr6, r3, 0x16
	ctx.cr[6].compare_i32(ctx.r[3].s32, 22, &mut ctx.xer);
	// 82DDF6EC: 40990014  ble cr6, 0x82ddf700
	if !ctx.cr[6].gt {
		sub_82DDF700(ctx, base);
		return;
	}
	// 82DDF6F0: 2F030018  cmpwi cr6, r3, 0x18
	ctx.cr[6].compare_i32(ctx.r[3].s32, 24, &mut ctx.xer);
	// 82DDF6F4: 41990024  bgt cr6, 0x82ddf718
	if ctx.cr[6].gt {
		sub_82DDF718(ctx, base);
		return;
	}
	// 82DDF6F8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DDF6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF700 size=8
    let mut pc: u32 = 0x82DDF700;
    'dispatch: loop {
        match pc {
            0x82DDF700 => {
    //   block [0x82DDF700..0x82DDF708)
	// 82DDF700: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DDF704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF708 size=16
    let mut pc: u32 = 0x82DDF708;
    'dispatch: loop {
        match pc {
            0x82DDF708 => {
    //   block [0x82DDF708..0x82DDF718)
	// 82DDF708: 2F03002B  cmpwi cr6, r3, 0x2b
	ctx.cr[6].compare_i32(ctx.r[3].s32, 43, &mut ctx.xer);
	// 82DDF70C: 4198000C  blt cr6, 0x82ddf718
	if ctx.cr[6].lt {
		sub_82DDF718(ctx, base);
		return;
	}
	// 82DDF710: 2F03002C  cmpwi cr6, r3, 0x2c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 44, &mut ctx.xer);
	// 82DDF714: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF718 size=8
    let mut pc: u32 = 0x82DDF718;
    'dispatch: loop {
        match pc {
            0x82DDF718 => {
    //   block [0x82DDF718..0x82DDF720)
	// 82DDF718: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82DDF71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF720 size=20
    let mut pc: u32 = 0x82DDF720;
    'dispatch: loop {
        match pc {
            0x82DDF720 => {
    //   block [0x82DDF720..0x82DDF734)
	// 82DDF720: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82DDF724: 419A0010  beq cr6, 0x82ddf734
	if ctx.cr[6].eq {
		sub_82DDF734(ctx, base);
		return;
	}
	// 82DDF728: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82DDF72C: 419AFFD4  beq cr6, 0x82ddf700
	if ctx.cr[6].eq {
		sub_82DDF700(ctx, base);
		return;
	}
	// 82DDF730: 4BFFFFE8  b 0x82ddf718
	sub_82DDF718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF734 size=8
    let mut pc: u32 = 0x82DDF734;
    'dispatch: loop {
        match pc {
            0x82DDF734 => {
    //   block [0x82DDF734..0x82DDF73C)
	// 82DDF734: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82DDF738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF740 size=200
    let mut pc: u32 = 0x82DDF740;
    'dispatch: loop {
        match pc {
            0x82DDF740 => {
    //   block [0x82DDF740..0x82DDF808)
	// 82DDF740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDF748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF74C: 3944003A  addi r10, r4, 0x3a
	ctx.r[10].s64 = ctx.r[4].s64 + 58;
	// 82DDF750: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DDF754: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DDF758: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DDF75C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDF760: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDF764: 4800002C  b 0x82ddf790
	pc = 0x82DDF790; continue 'dispatch;
	// 82DDF768: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDF76C: 4082002C  bne 0x82ddf798
	if !ctx.cr[0].eq {
	pc = 0x82DDF798; continue 'dispatch;
	}
	// 82DDF770: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF774: 554807FF  clrlwi. r8, r10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF778: 4182000C  beq 0x82ddf784
	if ctx.cr[0].eq {
	pc = 0x82DDF784; continue 'dispatch;
	}
	// 82DDF77C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDF780: 419A002C  beq cr6, 0x82ddf7ac
	if ctx.cr[6].eq {
	pc = 0x82DDF7AC; continue 'dispatch;
	}
	// 82DDF784: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82DDF788: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF78C: 554AF7FE  rlwinm r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82DDF790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF794: 409AFFD4  bne cr6, 0x82ddf768
	if !ctx.cr[6].eq {
	pc = 0x82DDF768; continue 'dispatch;
	}
	// 82DDF798: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDF79C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDF7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDF7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDF7A8: 4E800020  blr
	return;
	// 82DDF7AC: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDF7B0: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDF7B4: 409A0010  bne cr6, 0x82ddf7c4
	if !ctx.cr[6].eq {
	pc = 0x82DDF7C4; continue 'dispatch;
	}
	// 82DDF7B8: 816900E4  lwz r11, 0xe4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF7BC: 556BBFFF  rlwinm. r11, r11, 0x17, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF7C0: 40820040  bne 0x82ddf800
	if !ctx.cr[0].eq {
	pc = 0x82DDF800; continue 'dispatch;
	}
	// 82DDF7C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF7C8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDF7CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF7D0: 4E800421  bctrl
	ctx.lr = 0x82DDF7D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF7D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF7D8: 40820028  bne 0x82ddf800
	if !ctx.cr[0].eq {
	pc = 0x82DDF800; continue 'dispatch;
	}
	// 82DDF7DC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF7E0: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDF7E4: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDF7E8: 38CB6330  addi r6, r11, 0x6330
	ctx.r[6].s64 = ctx.r[11].s64 + 25392;
	// 82DDF7EC: 38AA6430  addi r5, r10, 0x6430
	ctx.r[5].s64 = ctx.r[10].s64 + 25648;
	// 82DDF7F0: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDF7F4: 38E00287  li r7, 0x287
	ctx.r[7].s64 = 647;
	// 82DDF7F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDF7FC: 4BF03285  bl 0x82ce2a80
	ctx.lr = 0x82DDF800;
	sub_82CE2A80(ctx, base);
	// 82DDF800: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DDF804: 4BFFFF98  b 0x82ddf79c
	pc = 0x82DDF79C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF808 size=44
    let mut pc: u32 = 0x82DDF808;
    'dispatch: loop {
        match pc {
            0x82DDF808 => {
    //   block [0x82DDF808..0x82DDF834)
	// 82DDF808: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDF80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DDF810: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDF814: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDF818: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDF81C: 90CB0038  stw r6, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 82DDF820: 614A0042  ori r10, r10, 0x42
	ctx.r[10].u64 = ctx.r[10].u64 | 66;
	// 82DDF824: 90AB0050  stw r5, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82DDF828: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DDF82C: 914B00E4  stw r10, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82DDF830: 4BFDF960  b 0x82dbf190
	sub_82DBF190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDF838 size=28
    let mut pc: u32 = 0x82DDF838;
    'dispatch: loop {
        match pc {
            0x82DDF838 => {
    //   block [0x82DDF838..0x82DDF854)
	// 82DDF838: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82DDF83C: 908300A8  stw r4, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[4].u32 ) };
	// 82DDF840: 99630084  stb r11, 0x84(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 82DDF844: 99630085  stb r11, 0x85(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 82DDF848: 99630086  stb r11, 0x86(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 82DDF84C: 99630087  stb r11, 0x87(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 82DDF850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF858 size=120
    let mut pc: u32 = 0x82DDF858;
    'dispatch: loop {
        match pc {
            0x82DDF858 => {
    //   block [0x82DDF858..0x82DDF8D0)
	// 82DDF858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDF860: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDF864: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDF868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF86C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDF870: 807E00B4  lwz r3, 0xb4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DDF874: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDF878: 409A0040  bne cr6, 0x82ddf8b8
	if !ctx.cr[6].eq {
	pc = 0x82DDF8B8; continue 'dispatch;
	}
	// 82DDF87C: 83FE00EC  lwz r31, 0xec(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82DDF880: 48000024  b 0x82ddf8a4
	pc = 0x82DDF8A4; continue 'dispatch;
	// 82DDF884: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF888: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DDF88C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF890: 4E800421  bctrl
	ctx.lr = 0x82DDF894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF894: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF898: 41820018  beq 0x82ddf8b0
	if ctx.cr[0].eq {
	pc = 0x82DDF8B0; continue 'dispatch;
	}
	// 82DDF89C: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82DDF8A0: 83EB00EC  lwz r31, 0xec(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(236 as u32) ) } as u64;
	// 82DDF8A4: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82DDF8A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DDF8AC: 409AFFD8  bne cr6, 0x82ddf884
	if !ctx.cr[6].eq {
	pc = 0x82DDF884; continue 'dispatch;
	}
	// 82DDF8B0: 93FE00B4  stw r31, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 82DDF8B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDF8B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDF8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDF8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDF8C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDF8C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDF8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDF8D0 size=140
    let mut pc: u32 = 0x82DDF8D0;
    'dispatch: loop {
        match pc {
            0x82DDF8D0 => {
    //   block [0x82DDF8D0..0x82DDF95C)
	// 82DDF8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF8D4: 483C8895  bl 0x831a8168
	ctx.lr = 0x82DDF8D8;
	sub_831A8130(ctx, base);
	// 82DDF8D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF8DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF8E0: 3D4082DA  lis r10, -0x7d26
	ctx.r[10].s64 = -2099642368;
	// 82DDF8E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDF8E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DDF8EC: 38EA6058  addi r7, r10, 0x6058
	ctx.r[7].s64 = ctx.r[10].s64 + 24664;
	// 82DDF8F0: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 82DDF8F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDF8F8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDF8FC: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF900: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82DDF904: 80CB0568  lwz r6, 0x568(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1384 as u32) ) } as u64;
	// 82DDF908: 806B0600  lwz r3, 0x600(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82DDF90C: 4BFDC07D  bl 0x82dbb988
	ctx.lr = 0x82DDF910;
	sub_82DBB988(ctx, base);
	// 82DDF910: 3D6082DA  lis r11, -0x7d26
	ctx.r[11].s64 = -2099642368;
	// 82DDF914: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DDF918: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDF91C: 38EB6058  addi r7, r11, 0x6058
	ctx.r[7].s64 = ctx.r[11].s64 + 24664;
	// 82DDF920: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 82DDF924: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82DDF928: 80CB0568  lwz r6, 0x568(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1384 as u32) ) } as u64;
	// 82DDF92C: 806B0600  lwz r3, 0x600(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 82DDF930: 4BFDC4F9  bl 0x82dbbe28
	ctx.lr = 0x82DDF934;
	sub_82DBBE28(ctx, base);
	// 82DDF934: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82DDF938: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DDF93C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DDF940: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DDF944: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDF948: 815F00A8  lwz r10, 0xa8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82DDF94C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF950: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDF954: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDF958: 483C8860  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF960 size=296
    let mut pc: u32 = 0x82DDF960;
    'dispatch: loop {
        match pc {
            0x82DDF960 => {
    //   block [0x82DDF960..0x82DDFA88)
	// 82DDF960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF964: 483C87F9  bl 0x831a815c
	ctx.lr = 0x82DDF968;
	sub_831A8130(ctx, base);
	// 82DDF968: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF96C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF970: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DDF974: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DDF978: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDF97C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF980: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF988: 4E800421  bctrl
	ctx.lr = 0x82DDF98C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF98C: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DDF990: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF994: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82DDF998: 396B2438  addi r11, r11, 0x2438
	ctx.r[11].s64 = ctx.r[11].s64 + 9272;
	// 82DDF99C: 1D090034  mulli r8, r9, 0x34
	ctx.r[8].s64 = ctx.r[9].s64 * 52;
	// 82DDF9A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DDF9A4: 3D208211  lis r9, -0x7def
	ctx.r[9].s64 = -2112815104;
	// 82DDF9A8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDF9AC: 3BCA5E90  addi r30, r10, 0x5e90
	ctx.r[30].s64 = ctx.r[10].s64 + 24208;
	// 82DDF9B0: 3BA96330  addi r29, r9, 0x6330
	ctx.r[29].s64 = ctx.r[9].s64 + 25392;
	// 82DDF9B4: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDF9B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF9BC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DDF9C0: 409A0020  bne cr6, 0x82ddf9e0
	if !ctx.cr[6].eq {
	pc = 0x82DDF9E0; continue 'dispatch;
	}
	// 82DDF9C4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF9C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDF9CC: 38AB64B0  addi r5, r11, 0x64b0
	ctx.r[5].s64 = ctx.r[11].s64 + 25776;
	// 82DDF9D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF9D4: 38E00598  li r7, 0x598
	ctx.r[7].s64 = 1432;
	// 82DDF9D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDF9DC: 4BF030A5  bl 0x82ce2a80
	ctx.lr = 0x82DDF9E0;
	sub_82CE2A80(ctx, base);
	// 82DDF9E0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDF9E4: 4182006C  beq 0x82ddfa50
	if ctx.cr[0].eq {
	pc = 0x82DDFA50; continue 'dispatch;
	}
	// 82DDF9E8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF9EC: 2F0B0034  cmpwi cr6, r11, 0x34
	ctx.cr[6].compare_i32(ctx.r[11].s32, 52, &mut ctx.xer);
	// 82DDF9F0: 419A0020  beq cr6, 0x82ddfa10
	if ctx.cr[6].eq {
	pc = 0x82DDFA10; continue 'dispatch;
	}
	// 82DDF9F4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDF9F8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDF9FC: 38AB6498  addi r5, r11, 0x6498
	ctx.r[5].s64 = ctx.r[11].s64 + 25752;
	// 82DDFA00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFA04: 38E0059A  li r7, 0x59a
	ctx.r[7].s64 = 1434;
	// 82DDFA08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDFA0C: 4BF03075  bl 0x82ce2a80
	ctx.lr = 0x82DDFA10;
	sub_82CE2A80(ctx, base);
	// 82DDFA10: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 82DDFA14: 39400035  li r10, 0x35
	ctx.r[10].s64 = 53;
	// 82DDFA18: 893F0099  lbz r9, 0x99(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(153 as u32) ) } as u64;
	// 82DDFA1C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DDFA20: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82DDFA24: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DDFA28: 997F009A  stb r11, 0x9a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 82DDFA2C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDFA30: 419A0020  beq cr6, 0x82ddfa50
	if ctx.cr[6].eq {
	pc = 0x82DDFA50; continue 'dispatch;
	}
	// 82DDFA34: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFA38: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDFA3C: 38AB6474  addi r5, r11, 0x6474
	ctx.r[5].s64 = ctx.r[11].s64 + 25716;
	// 82DDFA40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFA44: 38E0059E  li r7, 0x59e
	ctx.r[7].s64 = 1438;
	// 82DDFA48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDFA4C: 4BF03035  bl 0x82ce2a80
	ctx.lr = 0x82DDFA50;
	sub_82CE2A80(ctx, base);
	// 82DDFA50: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82DDFA54: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 82DDFA58: 997F0084  stb r11, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 82DDFA5C: 997F0085  stb r11, 0x85(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 82DDFA60: 997F0086  stb r11, 0x86(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 82DDFA64: 997F0087  stb r11, 0x87(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 82DDFA68: 409A0018  bne cr6, 0x82ddfa80
	if !ctx.cr[6].eq {
	pc = 0x82DDFA80; continue 'dispatch;
	}
	// 82DDFA6C: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82DDFA70: 997F0088  stb r11, 0x88(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 82DDFA74: 997F0089  stb r11, 0x89(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 82DDFA78: 997F008A  stb r11, 0x8a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 82DDFA7C: 997F008B  stb r11, 0x8b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(139 as u32), ctx.r[11].u8 ) };
	// 82DDFA80: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DDFA84: 483C8728  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFA88 size=188
    let mut pc: u32 = 0x82DDFA88;
    'dispatch: loop {
        match pc {
            0x82DDFA88 => {
    //   block [0x82DDFA88..0x82DDFB44)
	// 82DDFA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFA8C: 483C86DD  bl 0x831a8168
	ctx.lr = 0x82DDFA90;
	sub_831A8130(ctx, base);
	// 82DDFA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFA94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFA98: 3D408331  lis r10, -0x7ccf
	ctx.r[10].s64 = -2093940736;
	// 82DDFA9C: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDFAA0: 394A2438  addi r10, r10, 0x2438
	ctx.r[10].s64 = ctx.r[10].s64 + 9272;
	// 82DDFAA4: 3BC95E90  addi r30, r9, 0x5e90
	ctx.r[30].s64 = ctx.r[9].s64 + 24208;
	// 82DDFAA8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDFAAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DDFAB0: 1D0B0034  mulli r8, r11, 0x34
	ctx.r[8].s64 = ctx.r[11].s64 * 52;
	// 82DDFAB4: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDFAB8: 38EBFFCB  addi r7, r11, -0x35
	ctx.r[7].s64 = ctx.r[11].s64 + -53;
	// 82DDFABC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFAC0: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82DDFAC4: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82DDFAC8: 3BAB6330  addi r29, r11, 0x6330
	ctx.r[29].s64 = ctx.r[11].s64 + 25392;
	// 82DDFACC: 54FCDFFE  rlwinm r28, r7, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82DDFAD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDFAD4: 409A0020  bne cr6, 0x82ddfaf4
	if !ctx.cr[6].eq {
	pc = 0x82DDFAF4; continue 'dispatch;
	}
	// 82DDFAD8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFADC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDFAE0: 38AB64B0  addi r5, r11, 0x64b0
	ctx.r[5].s64 = ctx.r[11].s64 + 25776;
	// 82DDFAE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFAE8: 38E005B9  li r7, 0x5b9
	ctx.r[7].s64 = 1465;
	// 82DDFAEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDFAF0: 4BF02F91  bl 0x82ce2a80
	ctx.lr = 0x82DDFAF4;
	sub_82CE2A80(ctx, base);
	// 82DDFAF4: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDFAF8: 41820040  beq 0x82ddfb38
	if ctx.cr[0].eq {
	pc = 0x82DDFB38; continue 'dispatch;
	}
	// 82DDFAFC: 897F0099  lbz r11, 0x99(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(153 as u32) ) } as u64;
	// 82DDFB00: 895F009A  lbz r10, 0x9a(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 82DDFB04: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DDFB08: 419A0020  beq cr6, 0x82ddfb28
	if ctx.cr[6].eq {
	pc = 0x82DDFB28; continue 'dispatch;
	}
	// 82DDFB0C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFB10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDFB14: 38AB6474  addi r5, r11, 0x6474
	ctx.r[5].s64 = ctx.r[11].s64 + 25716;
	// 82DDFB18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFB1C: 38E005BC  li r7, 0x5bc
	ctx.r[7].s64 = 1468;
	// 82DDFB20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDFB24: 4BF02F5D  bl 0x82ce2a80
	ctx.lr = 0x82DDFB28;
	sub_82CE2A80(ctx, base);
	// 82DDFB28: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 82DDFB2C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DDFB30: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DDFB34: 997F009A  stb r11, 0x9a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 82DDFB38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDFB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDFB40: 483C8678  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFB48 size=116
    let mut pc: u32 = 0x82DDFB48;
    'dispatch: loop {
        match pc {
            0x82DDFB48 => {
    //   block [0x82DDFB48..0x82DDFBBC)
	// 82DDFB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFB50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFB5C: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDFB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFB64: 394A30D8  addi r10, r10, 0x30d8
	ctx.r[10].s64 = ctx.r[10].s64 + 12504;
	// 82DDFB68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDFB6C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDFB70: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DDFB74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDFB78: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DDFB7C: 81450560  lwz r10, 0x560(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82DDFB80: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82DDFB84: 81450560  lwz r10, 0x560(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(1376 as u32) ) } as u64;
	// 82DDFB88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DDFB8C: 91450560  stw r10, 0x560(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(1376 as u32), ctx.r[10].u32 ) };
	// 82DDFB90: 917F037C  stw r11, 0x37c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(892 as u32), ctx.r[11].u32 ) };
	// 82DDFB94: 917F03B4  stw r11, 0x3b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(948 as u32), ctx.r[11].u32 ) };
	// 82DDFB98: 90BF03B8  stw r5, 0x3b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(952 as u32), ctx.r[5].u32 ) };
	// 82DDFB9C: 917F03BC  stw r11, 0x3bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(956 as u32), ctx.r[11].u32 ) };
	// 82DDFBA0: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 82DDFBA4: 4BFFF355  bl 0x82ddeef8
	ctx.lr = 0x82DDFBA8;
	sub_82DDEEF8(ctx, base);
	// 82DDFBA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDFBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFBB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFBC0 size=176
    let mut pc: u32 = 0x82DDFBC0;
    'dispatch: loop {
        match pc {
            0x82DDFBC0 => {
    //   block [0x82DDFBC0..0x82DDFC70)
	// 82DDFBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFBC4: 483C85A9  bl 0x831a816c
	ctx.lr = 0x82DDFBC8;
	sub_831A8130(ctx, base);
	// 82DDFBC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFBCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFBD0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDFBD4: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 82DDFBD8: 896B0571  lbz r11, 0x571(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1393 as u32) ) } as u64;
	// 82DDFBDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DDFBE0: 41820028  beq 0x82ddfc08
	if ctx.cr[0].eq {
	pc = 0x82DDFC08; continue 'dispatch;
	}
	// 82DDFBE4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFBE8: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDFBEC: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DDFBF0: 38CB6330  addi r6, r11, 0x6330
	ctx.r[6].s64 = ctx.r[11].s64 + 25392;
	// 82DDFBF4: 38AA2460  addi r5, r10, 0x2460
	ctx.r[5].s64 = ctx.r[10].s64 + 9312;
	// 82DDFBF8: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DDFBFC: 38E000B8  li r7, 0xb8
	ctx.r[7].s64 = 184;
	// 82DDFC00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDFC04: 4BF02E7D  bl 0x82ce2a80
	ctx.lr = 0x82DDFC08;
	sub_82CE2A80(ctx, base);
	// 82DDFC08: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDFC0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDFC10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDFC14: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82DDFC18: 4BFFF439  bl 0x82ddf050
	ctx.lr = 0x82DDFC1C;
	sub_82DDF050(ctx, base);
	// 82DDFC1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDFC20: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82DDFC24: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82DDFC28: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDFC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDFC30: 40990038  ble cr6, 0x82ddfc68
	if !ctx.cr[6].gt {
	pc = 0x82DDFC68; continue 'dispatch;
	}
	// 82DDFC34: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDFC38: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDFC3C: 409A002C  bne cr6, 0x82ddfc68
	if !ctx.cr[6].eq {
	pc = 0x82DDFC68; continue 'dispatch;
	}
	// 82DDFC40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDFC48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDFC4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDFC50: 4E800421  bctrl
	ctx.lr = 0x82DDFC54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDFC54: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82DDFC58: 40990010  ble cr6, 0x82ddfc68
	if !ctx.cr[6].gt {
	pc = 0x82DDFC68; continue 'dispatch;
	}
	// 82DDFC5C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDFC60: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82DDFC64: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDFC68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFC6C: 483C8550  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDFC70 size=84
    let mut pc: u32 = 0x82DDFC70;
    'dispatch: loop {
        match pc {
            0x82DDFC70 => {
    //   block [0x82DDFC70..0x82DDFCC4)
	// 82DDFC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFC7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFC80: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDFC84: D041005C  stfs f2, 0x5c(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DDFC88: D0610064  stfs f3, 0x64(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DDFC8C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDFC90: D081006C  stfs f4, 0x6c(r1)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DDFC94: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDFC98: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDFC9C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDFCA0: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DDFCA4: E8E10058  ld r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDFCA8: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DDFCAC: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DDFCB0: 4BFFF419  bl 0x82ddf0c8
	ctx.lr = 0x82DDFCB4;
	sub_82DDF0C8(ctx, base);
	// 82DDFCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDFCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFCC8 size=84
    let mut pc: u32 = 0x82DDFCC8;
    'dispatch: loop {
        match pc {
            0x82DDFCC8 => {
    //   block [0x82DDFCC8..0x82DDFD1C)
	// 82DDFCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFCD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDFCD8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82DDFCDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82DDFCE0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82DDFCE4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82DDFCE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDFCEC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDFCF0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDFCF4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDFCF8: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DDFCFC: E8E10058  ld r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DDFD00: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DDFD04: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DDFD08: 4BFFF3C1  bl 0x82ddf0c8
	ctx.lr = 0x82DDFD0C;
	sub_82DDF0C8(ctx, base);
	// 82DDFD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDFD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFD20 size=92
    let mut pc: u32 = 0x82DDFD20;
    'dispatch: loop {
        match pc {
            0x82DDFD20 => {
    //   block [0x82DDFD20..0x82DDFD7C)
	// 82DDFD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFD2C: 4BFFFE1D  bl 0x82ddfb48
	ctx.lr = 0x82DDFD30;
	sub_82DDFB48(ctx, base);
	// 82DDFD30: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFD34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDFD38: 392B6598  addi r9, r11, 0x6598
	ctx.r[9].s64 = ctx.r[11].s64 + 26008;
	// 82DDFD3C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DDFD40: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82DDFD44: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDFD48: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82DDFD4C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDFD50: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DDFD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDFD58: 912BFFE8  stw r9, -0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24 as u32), ctx.r[9].u32 ) };
	// 82DDFD5C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDFD60: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DDFD64: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DDFD68: 4082FFE8  bne 0x82ddfd50
	if !ctx.cr[0].eq {
	pc = 0x82DDFD50; continue 'dispatch;
	}
	// 82DDFD6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDFD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFD80 size=76
    let mut pc: u32 = 0x82DDFD80;
    'dispatch: loop {
        match pc {
            0x82DDFD80 => {
    //   block [0x82DDFD80..0x82DDFDCC)
	// 82DDFD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFD88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFD8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFD90: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DDFD94: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 82DDFD98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFD9C: 4BFFFF85  bl 0x82ddfd20
	ctx.lr = 0x82DDFDA0;
	sub_82DDFD20(ctx, base);
	// 82DDFDA0: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DDFDA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDFDA8: 394A6598  addi r10, r10, 0x6598
	ctx.r[10].s64 = ctx.r[10].s64 + 26008;
	// 82DDFDAC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DDFDB0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDFDB4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DDFDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDFDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFDC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFDD0 size=84
    let mut pc: u32 = 0x82DDFDD0;
    'dispatch: loop {
        match pc {
            0x82DDFDD0 => {
    //   block [0x82DDFDD0..0x82DDFE24)
	// 82DDFDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFDD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFDDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFDE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFDE4: 4BFFFD65  bl 0x82ddfb48
	ctx.lr = 0x82DDFDE8;
	sub_82DDFB48(ctx, base);
	// 82DDFDE8: 3D208211  lis r9, -0x7def
	ctx.r[9].s64 = -2112815104;
	// 82DDFDEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFDF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDFDF4: 39296600  addi r9, r9, 0x6600
	ctx.r[9].s64 = ctx.r[9].s64 + 26112;
	// 82DDFDF8: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82DDFDFC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DDFE00: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDFE04: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82DDFE08: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82DDFE0C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDFE10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDFE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFE1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFE28 size=12
    let mut pc: u32 = 0x82DDFE28;
    'dispatch: loop {
        match pc {
            0x82DDFE28 => {
    //   block [0x82DDFE28..0x82DDFE34)
	// 82DDFE28: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFE2C: 386B6668  addi r3, r11, 0x6668
	ctx.r[3].s64 = ctx.r[11].s64 + 26216;
	// 82DDFE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFE38 size=108
    let mut pc: u32 = 0x82DDFE38;
    'dispatch: loop {
        match pc {
            0x82DDFE38 => {
    //   block [0x82DDFE38..0x82DDFEA4)
	// 82DDFE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFE3C: 483C8331  bl 0x831a816c
	ctx.lr = 0x82DDFE40;
	sub_831A8130(ctx, base);
	// 82DDFE40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFE44: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDFE48: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82DDFE4C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDFE50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDFE54: 4BFFFCF5  bl 0x82ddfb48
	ctx.lr = 0x82DDFE58;
	sub_82DDFB48(ctx, base);
	// 82DDFE58: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFE5C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DDFE60: 396B6670  addi r11, r11, 0x6670
	ctx.r[11].s64 = ctx.r[11].s64 + 26224;
	// 82DDFE64: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82DDFE68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFE6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDFE70: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DDFE74: 616B0012  ori r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u64 | 18;
	// 82DDFE78: 917E00E4  stw r11, 0xe4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DDFE7C: 807D0AB0  lwz r3, 0xab0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DDFE80: 4BFDF311  bl 0x82dbf190
	ctx.lr = 0x82DDFE84;
	sub_82DBF190(ctx, base);
	// 82DDFE84: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82DDFE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDFE8C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DDFE90: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82DDFE94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDFE98: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDFE9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFEA0: 483C831C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFEA8 size=12
    let mut pc: u32 = 0x82DDFEA8;
    'dispatch: loop {
        match pc {
            0x82DDFEA8 => {
    //   block [0x82DDFEA8..0x82DDFEB4)
	// 82DDFEA8: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82DDFEAC: 386B1D2C  addi r3, r11, 0x1d2c
	ctx.r[3].s64 = ctx.r[11].s64 + 7468;
	// 82DDFEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFEB8 size=84
    let mut pc: u32 = 0x82DDFEB8;
    'dispatch: loop {
        match pc {
            0x82DDFEB8 => {
    //   block [0x82DDFEB8..0x82DDFF0C)
	// 82DDFEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFEBC: 483C82B1  bl 0x831a816c
	ctx.lr = 0x82DDFEC0;
	sub_831A8130(ctx, base);
	// 82DDFEC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFEC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDFEC8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82DDFECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFED0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDFED4: 4BFFFF65  bl 0x82ddfe38
	ctx.lr = 0x82DDFED8;
	sub_82DDFE38(ctx, base);
	// 82DDFED8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDFEE0: 396B66D8  addi r11, r11, 0x66d8
	ctx.r[11].s64 = ctx.r[11].s64 + 26328;
	// 82DDFEE4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDFEE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDFEEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DDFEF0: 4BFFF161  bl 0x82ddf050
	ctx.lr = 0x82DDFEF4;
	sub_82DDF050(ctx, base);
	// 82DDFEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFEF8: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82DDFEFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDFF00: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DDFF04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFF08: 483C82B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFF10 size=12
    let mut pc: u32 = 0x82DDFF10;
    'dispatch: loop {
        match pc {
            0x82DDFF10 => {
    //   block [0x82DDFF10..0x82DDFF1C)
	// 82DDFF10: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFF14: 386B6740  addi r3, r11, 0x6740
	ctx.r[3].s64 = ctx.r[11].s64 + 26432;
	// 82DDFF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFF20 size=12
    let mut pc: u32 = 0x82DDFF20;
    'dispatch: loop {
        match pc {
            0x82DDFF20 => {
    //   block [0x82DDFF20..0x82DDFF2C)
	// 82DDFF20: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFF24: 386B674C  addi r3, r11, 0x674c
	ctx.r[3].s64 = ctx.r[11].s64 + 26444;
	// 82DDFF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFF30 size=72
    let mut pc: u32 = 0x82DDFF30;
    'dispatch: loop {
        match pc {
            0x82DDFF30 => {
    //   block [0x82DDFF30..0x82DDFF78)
	// 82DDFF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFF38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFF3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFF40: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DDFF44: 38800082  li r4, 0x82
	ctx.r[4].s64 = 130;
	// 82DDFF48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFF4C: 4BFFFBFD  bl 0x82ddfb48
	ctx.lr = 0x82DDFF50;
	sub_82DDFB48(ctx, base);
	// 82DDFF50: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFF54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDFF58: 396B6758  addi r11, r11, 0x6758
	ctx.r[11].s64 = ctx.r[11].s64 + 26456;
	// 82DDFF5C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDFF60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDFF64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDFF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFF70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFF78 size=12
    let mut pc: u32 = 0x82DDFF78;
    'dispatch: loop {
        match pc {
            0x82DDFF78 => {
    //   block [0x82DDFF78..0x82DDFF84)
	// 82DDFF78: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DDFF7C: 386B5964  addi r3, r11, 0x5964
	ctx.r[3].s64 = ctx.r[11].s64 + 22884;
	// 82DDFF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFF88 size=72
    let mut pc: u32 = 0x82DDFF88;
    'dispatch: loop {
        match pc {
            0x82DDFF88 => {
    //   block [0x82DDFF88..0x82DDFFD0)
	// 82DDFF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFF90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFF94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFF98: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DDFF9C: 38800083  li r4, 0x83
	ctx.r[4].s64 = 131;
	// 82DDFFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFFA4: 4BFFFBA5  bl 0x82ddfb48
	ctx.lr = 0x82DDFFA8;
	sub_82DDFB48(ctx, base);
	// 82DDFFA8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDFFB0: 396B67C0  addi r11, r11, 0x67c0
	ctx.r[11].s64 = ctx.r[11].s64 + 26560;
	// 82DDFFB4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDFFB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDFFBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDFFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFFC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFFD0 size=12
    let mut pc: u32 = 0x82DDFFD0;
    'dispatch: loop {
        match pc {
            0x82DDFFD0 => {
    //   block [0x82DDFFD0..0x82DDFFDC)
	// 82DDFFD0: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DDFFD4: 386B6828  addi r3, r11, 0x6828
	ctx.r[3].s64 = ctx.r[11].s64 + 26664;
	// 82DDFFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFFE0 size=84
    let mut pc: u32 = 0x82DDFFE0;
    'dispatch: loop {
        match pc {
            0x82DDFFE0 => {
    //   block [0x82DDFFE0..0x82DE0034)
	// 82DDFFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFFE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFFEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFFF0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DDFFF4: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 82DDFFF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFFFC: 4BFFFB4D  bl 0x82ddfb48
	ctx.lr = 0x82DE0000;
	sub_82DDFB48(ctx, base);
	// 82DE0000: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0004: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0008: 394A6830  addi r10, r10, 0x6830
	ctx.r[10].s64 = ctx.r[10].s64 + 26672;
	// 82DE000C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE0010: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DE0014: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0018: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE001C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE0020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE002C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0038 size=12
    let mut pc: u32 = 0x82DE0038;
    'dispatch: loop {
        match pc {
            0x82DE0038 => {
    //   block [0x82DE0038..0x82DE0044)
	// 82DE0038: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE003C: 386B6898  addi r3, r11, 0x6898
	ctx.r[3].s64 = ctx.r[11].s64 + 26776;
	// 82DE0040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0048 size=116
    let mut pc: u32 = 0x82DE0048;
    'dispatch: loop {
        match pc {
            0x82DE0048 => {
    //   block [0x82DE0048..0x82DE00BC)
	// 82DE0048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE004C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE005C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE0060: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 82DE0064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0068: 4BFFFAE1  bl 0x82ddfb48
	ctx.lr = 0x82DE006C;
	sub_82DDFB48(ctx, base);
	// 82DE006C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE0074: 392B6830  addi r9, r11, 0x6830
	ctx.r[9].s64 = ctx.r[11].s64 + 26672;
	// 82DE0078: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE007C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0080: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE0084: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE0088: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DE008C: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0090: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82DE0094: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82DE0098: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82DE009C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE00A0: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DE00A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE00A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE00AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE00B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE00B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE00B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE00C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE00C0 size=84
    let mut pc: u32 = 0x82DE00C0;
    'dispatch: loop {
        match pc {
            0x82DE00C0 => {
    //   block [0x82DE00C0..0x82DE0114)
	// 82DE00C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE00C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE00C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE00CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE00D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE00D4: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 82DE00D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE00DC: 4BFFFA6D  bl 0x82ddfb48
	ctx.lr = 0x82DE00E0;
	sub_82DDFB48(ctx, base);
	// 82DE00E0: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE00E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE00E8: 394A68A8  addi r10, r10, 0x68a8
	ctx.r[10].s64 = ctx.r[10].s64 + 26792;
	// 82DE00EC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE00F0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DE00F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE00F8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DE00FC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE0100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE010C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0118 size=12
    let mut pc: u32 = 0x82DE0118;
    'dispatch: loop {
        match pc {
            0x82DE0118 => {
    //   block [0x82DE0118..0x82DE0124)
	// 82DE0118: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE011C: 386B6910  addi r3, r11, 0x6910
	ctx.r[3].s64 = ctx.r[11].s64 + 26896;
	// 82DE0120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0128 size=112
    let mut pc: u32 = 0x82DE0128;
    'dispatch: loop {
        match pc {
            0x82DE0128 => {
    //   block [0x82DE0128..0x82DE0198)
	// 82DE0128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE012C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE013C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE0140: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 82DE0144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0148: 4BFFFA01  bl 0x82ddfb48
	ctx.lr = 0x82DE014C;
	sub_82DDFB48(ctx, base);
	// 82DE014C: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0150: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0154: 394A68A8  addi r10, r10, 0x68a8
	ctx.r[10].s64 = ctx.r[10].s64 + 26792;
	// 82DE0158: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE015C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE0160: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0164: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE0168: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DE016C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0170: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DE0174: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82DE0178: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82DE017C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE0180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE018C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE0190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0198 size=92
    let mut pc: u32 = 0x82DE0198;
    'dispatch: loop {
        match pc {
            0x82DE0198 => {
    //   block [0x82DE0198..0x82DE01F4)
	// 82DE0198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE019C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE01A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE01A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE01A8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE01AC: 3880007A  li r4, 0x7a
	ctx.r[4].s64 = 122;
	// 82DE01B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE01B4: 4BFFF995  bl 0x82ddfb48
	ctx.lr = 0x82DE01B8;
	sub_82DDFB48(ctx, base);
	// 82DE01B8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE01BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE01C0: 396B6920  addi r11, r11, 0x6920
	ctx.r[11].s64 = ctx.r[11].s64 + 26912;
	// 82DE01C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE01C8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE01CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE01D0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE01D4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE01D8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82DE01DC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE01E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE01E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE01E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE01EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE01F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE01F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE01F8 size=12
    let mut pc: u32 = 0x82DE01F8;
    'dispatch: loop {
        match pc {
            0x82DE01F8 => {
    //   block [0x82DE01F8..0x82DE0204)
	// 82DE01F8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE01FC: 386B6988  addi r3, r11, 0x6988
	ctx.r[3].s64 = ctx.r[11].s64 + 27016;
	// 82DE0200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0208 size=72
    let mut pc: u32 = 0x82DE0208;
    'dispatch: loop {
        match pc {
            0x82DE0208 => {
    //   block [0x82DE0208..0x82DE0250)
	// 82DE0208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0214: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0218: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DE021C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0220: 4BFFF929  bl 0x82ddfb48
	ctx.lr = 0x82DE0224;
	sub_82DDFB48(ctx, base);
	// 82DE0224: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0228: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE022C: 394A6998  addi r10, r10, 0x6998
	ctx.r[10].s64 = ctx.r[10].s64 + 27032;
	// 82DE0230: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE0234: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0238: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE023C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0250 size=12
    let mut pc: u32 = 0x82DE0250;
    'dispatch: loop {
        match pc {
            0x82DE0250 => {
    //   block [0x82DE0250..0x82DE025C)
	// 82DE0250: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0254: 386B6A00  addi r3, r11, 0x6a00
	ctx.r[3].s64 = ctx.r[11].s64 + 27136;
	// 82DE0258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0260 size=72
    let mut pc: u32 = 0x82DE0260;
    'dispatch: loop {
        match pc {
            0x82DE0260 => {
    //   block [0x82DE0260..0x82DE02A8)
	// 82DE0260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE026C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0270: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE0274: 38800089  li r4, 0x89
	ctx.r[4].s64 = 137;
	// 82DE0278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE027C: 4BFFF8CD  bl 0x82ddfb48
	ctx.lr = 0x82DE0280;
	sub_82DDFB48(ctx, base);
	// 82DE0280: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE0288: 396B6A10  addi r11, r11, 0x6a10
	ctx.r[11].s64 = ctx.r[11].s64 + 27152;
	// 82DE028C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE0290: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE029C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE02A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE02A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE02A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE02A8 size=12
    let mut pc: u32 = 0x82DE02A8;
    'dispatch: loop {
        match pc {
            0x82DE02A8 => {
    //   block [0x82DE02A8..0x82DE02B4)
	// 82DE02A8: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE02AC: 386B6A78  addi r3, r11, 0x6a78
	ctx.r[3].s64 = ctx.r[11].s64 + 27256;
	// 82DE02B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE02B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE02B8 size=108
    let mut pc: u32 = 0x82DE02B8;
    'dispatch: loop {
        match pc {
            0x82DE02B8 => {
    //   block [0x82DE02B8..0x82DE0324)
	// 82DE02B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE02BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE02C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE02C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE02C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE02CC: 3880007D  li r4, 0x7d
	ctx.r[4].s64 = 125;
	// 82DE02D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE02D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE02D8: 4BFFF871  bl 0x82ddfb48
	ctx.lr = 0x82DE02DC;
	sub_82DDFB48(ctx, base);
	// 82DE02DC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE02E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE02E4: 396B6A80  addi r11, r11, 0x6a80
	ctx.r[11].s64 = ctx.r[11].s64 + 27264;
	// 82DE02E8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE02EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE02F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE02F4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE02F8: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82DE02FC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE0300: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DE0304: 4BFDEE8D  bl 0x82dbf190
	ctx.lr = 0x82DE0308;
	sub_82DBF190(ctx, base);
	// 82DE0308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE030C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE031C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0328 size=12
    let mut pc: u32 = 0x82DE0328;
    'dispatch: loop {
        match pc {
            0x82DE0328 => {
    //   block [0x82DE0328..0x82DE0334)
	// 82DE0328: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE032C: 386B6AE8  addi r3, r11, 0x6ae8
	ctx.r[3].s64 = ctx.r[11].s64 + 27368;
	// 82DE0330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0338 size=72
    let mut pc: u32 = 0x82DE0338;
    'dispatch: loop {
        match pc {
            0x82DE0338 => {
    //   block [0x82DE0338..0x82DE0380)
	// 82DE0338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0348: 3880007E  li r4, 0x7e
	ctx.r[4].s64 = 126;
	// 82DE034C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0350: 4BFFF7F9  bl 0x82ddfb48
	ctx.lr = 0x82DE0354;
	sub_82DDFB48(ctx, base);
	// 82DE0354: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0358: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE035C: 394A6AF0  addi r10, r10, 0x6af0
	ctx.r[10].s64 = ctx.r[10].s64 + 27376;
	// 82DE0360: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE0364: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0368: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE036C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE037C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0380 size=12
    let mut pc: u32 = 0x82DE0380;
    'dispatch: loop {
        match pc {
            0x82DE0380 => {
    //   block [0x82DE0380..0x82DE038C)
	// 82DE0380: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0384: 386B6B58  addi r3, r11, 0x6b58
	ctx.r[3].s64 = ctx.r[11].s64 + 27480;
	// 82DE0388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0390 size=76
    let mut pc: u32 = 0x82DE0390;
    'dispatch: loop {
        match pc {
            0x82DE0390 => {
    //   block [0x82DE0390..0x82DE03DC)
	// 82DE0390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE039C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE03A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE03A4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE03A8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE03AC: 396B1520  addi r11, r11, 0x1520
	ctx.r[11].s64 = ctx.r[11].s64 + 5408;
	// 82DE03B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE03B4: 41820010  beq 0x82de03c4
	if ctx.cr[0].eq {
	pc = 0x82DE03C4; continue 'dispatch;
	}
	// 82DE03B8: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 82DE03BC: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DE03C0: 4BFC5349  bl 0x82da5708
	ctx.lr = 0x82DE03C4;
	sub_82DA5708(ctx, base);
	// 82DE03C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE03C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE03CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE03D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE03D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE03D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE03E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE03E0 size=76
    let mut pc: u32 = 0x82DE03E0;
    'dispatch: loop {
        match pc {
            0x82DE03E0 => {
    //   block [0x82DE03E0..0x82DE042C)
	// 82DE03E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE03E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE03E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE03EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE03F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE03F4: 4BFFF755  bl 0x82ddfb48
	ctx.lr = 0x82DE03F8;
	sub_82DDFB48(ctx, base);
	// 82DE03F8: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE03FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE0400: 394A6B68  addi r10, r10, 0x6b68
	ctx.r[10].s64 = ctx.r[10].s64 + 27496;
	// 82DE0404: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE0408: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82DE040C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0410: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DE0414: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE0418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE041C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0430 size=12
    let mut pc: u32 = 0x82DE0430;
    'dispatch: loop {
        match pc {
            0x82DE0430 => {
    //   block [0x82DE0430..0x82DE043C)
	// 82DE0430: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0434: 386B6C38  addi r3, r11, 0x6c38
	ctx.r[3].s64 = ctx.r[11].s64 + 27704;
	// 82DE0438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0440 size=12
    let mut pc: u32 = 0x82DE0440;
    'dispatch: loop {
        match pc {
            0x82DE0440 => {
    //   block [0x82DE0440..0x82DE044C)
	// 82DE0440: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0444: 386B6CB0  addi r3, r11, 0x6cb0
	ctx.r[3].s64 = ctx.r[11].s64 + 27824;
	// 82DE0448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0450 size=12
    let mut pc: u32 = 0x82DE0450;
    'dispatch: loop {
        match pc {
            0x82DE0450 => {
    //   block [0x82DE0450..0x82DE045C)
	// 82DE0450: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0454: 386B6D28  addi r3, r11, 0x6d28
	ctx.r[3].s64 = ctx.r[11].s64 + 27944;
	// 82DE0458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0460 size=12
    let mut pc: u32 = 0x82DE0460;
    'dispatch: loop {
        match pc {
            0x82DE0460 => {
    //   block [0x82DE0460..0x82DE046C)
	// 82DE0460: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0464: 386B6DA0  addi r3, r11, 0x6da0
	ctx.r[3].s64 = ctx.r[11].s64 + 28064;
	// 82DE0468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0470 size=12
    let mut pc: u32 = 0x82DE0470;
    'dispatch: loop {
        match pc {
            0x82DE0470 => {
    //   block [0x82DE0470..0x82DE047C)
	// 82DE0470: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0474: 386BB384  addi r3, r11, -0x4c7c
	ctx.r[3].s64 = ctx.r[11].s64 + -19580;
	// 82DE0478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0480 size=52
    let mut pc: u32 = 0x82DE0480;
    'dispatch: loop {
        match pc {
            0x82DE0480 => {
    //   block [0x82DE0480..0x82DE04B4)
	// 82DE0480: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82DE0484: 41980030  blt cr6, 0x82de04b4
	if ctx.cr[6].lt {
		sub_82DE04B4(ctx, base);
		return;
	}
	// 82DE0488: 2F060005  cmpwi cr6, r6, 5
	ctx.cr[6].compare_i32(ctx.r[6].s32, 5, &mut ctx.xer);
	// 82DE048C: 41990028  bgt cr6, 0x82de04b4
	if ctx.cr[6].gt {
		sub_82DE04B4(ctx, base);
		return;
	}
	// 82DE0490: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0494: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82DE0498: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DE049C: 38CB1540  addi r6, r11, 0x1540
	ctx.r[6].s64 = ctx.r[11].s64 + 5440;
	// 82DE04A0: 38AA9FF4  addi r5, r10, -0x600c
	ctx.r[5].s64 = ctx.r[10].s64 + -24588;
	// 82DE04A4: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DE04A8: 38E006B0  li r7, 0x6b0
	ctx.r[7].s64 = 1712;
	// 82DE04AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE04B0: 4BF025D0  b 0x82ce2a80
	sub_82CE2A80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE04B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE04B4 size=20
    let mut pc: u32 = 0x82DE04B4;
    'dispatch: loop {
        match pc {
            0x82DE04B4 => {
    //   block [0x82DE04B4..0x82DE04C8)
	// 82DE04B4: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82DE04B8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE04BC: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DE04C0: 7CCB19AE  stbx r6, r11, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u8) };
	// 82DE04C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE04C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE04C8 size=112
    let mut pc: u32 = 0x82DE04C8;
    'dispatch: loop {
        match pc {
            0x82DE04C8 => {
    //   block [0x82DE04C8..0x82DE0538)
	// 82DE04C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE04CC: 483C7CA1  bl 0x831a816c
	ctx.lr = 0x82DE04D0;
	sub_831A8130(ctx, base);
	// 82DE04D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE04D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE04D8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DE04DC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DE04E0: 4BFFF669  bl 0x82ddfb48
	ctx.lr = 0x82DE04E4;
	sub_82DDFB48(ctx, base);
	// 82DE04E4: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE04E8: 93BF00A8  stw r29, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82DE04EC: 3D008211  lis r8, -0x7def
	ctx.r[8].s64 = -2112815104;
	// 82DE04F0: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82DE04F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE04F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE04FC: 39086DB0  addi r8, r8, 0x6db0
	ctx.r[8].s64 = ctx.r[8].s64 + 28080;
	// 82DE0500: 997F00B0  stb r11, 0xb0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u8 ) };
	// 82DE0504: 814A631C  lwz r10, 0x631c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25372 as u32) ) } as u64;
	// 82DE0508: 38E0001E  li r7, 0x1e
	ctx.r[7].s64 = 30;
	// 82DE050C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE0510: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DE0514: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE0518: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82DE051C: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82DE0520: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DE0524: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82DE0528: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82DE052C: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DE0530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0534: 483C7C88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0538 size=192
    let mut pc: u32 = 0x82DE0538;
    'dispatch: loop {
        match pc {
            0x82DE0538 => {
    //   block [0x82DE0538..0x82DE05F8)
	// 82DE0538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE053C: 483C7C2D  bl 0x831a8168
	ctx.lr = 0x82DE0540;
	sub_831A8130(ctx, base);
	// 82DE0540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0544: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE0548: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DE054C: 4BFFF5FD  bl 0x82ddfb48
	ctx.lr = 0x82DE0550;
	sub_82DDFB48(ctx, base);
	// 82DE0550: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0554: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE0558: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DE055C: 396B6E18  addi r11, r11, 0x6e18
	ctx.r[11].s64 = ctx.r[11].s64 + 28184;
	// 82DE0560: 93FD00AC  stw r31, 0xac(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82DE0564: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DE0568: 93DD0014  stw r30, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82DE056C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0570: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 82DE0574: 93DD0010  stw r30, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DE0578: 915D00B0  stw r10, 0xb0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 82DE057C: 93FD0050  stw r31, 0x50(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DE0580: 93FD0054  stw r31, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DE0584: 93FD006C  stw r31, 0x6c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82DE0588: 93FD0068  stw r31, 0x68(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82DE058C: 807C05AC  lwz r3, 0x5ac(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0590: 4BFC4FF1  bl 0x82da5580
	ctx.lr = 0x82DE0594;
	sub_82DA5580(ctx, base);
	// 82DE0594: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DE0598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DE059C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DE05A0: 483C7C41  bl 0x831a81e0
	ctx.lr = 0x82DE05A4;
	sub_831A81E0(ctx, base);
	// 82DE05A4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82DE05A8: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82DE05AC: B3DC000E  sth r30, 0xe(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 82DE05B0: B17C0000  sth r11, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82DE05B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE05B8: B17C0002  sth r11, 2(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82DE05BC: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DE05C0: B15C0006  sth r10, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DE05C4: B15C0008  sth r10, 8(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 82DE05C8: B17C000A  sth r11, 0xa(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82DE05CC: B17C000C  sth r11, 0xc(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82DE05D0: B3FC0010  sth r31, 0x10(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 82DE05D4: B3DC0012  sth r30, 0x12(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(18 as u32), ctx.r[30].u16 ) };
	// 82DE05D8: B3FC0016  sth r31, 0x16(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(22 as u32), ctx.r[31].u16 ) };
	// 82DE05DC: B3FC0018  sth r31, 0x18(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[31].u16 ) };
	// 82DE05E0: B3FC001A  sth r31, 0x1a(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(26 as u32), ctx.r[31].u16 ) };
	// 82DE05E4: B3FC001C  sth r31, 0x1c(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[31].u16 ) };
	// 82DE05E8: B3FC0014  sth r31, 0x14(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[31].u16 ) };
	// 82DE05EC: 939D00B8  stw r28, 0xb8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(184 as u32), ctx.r[28].u32 ) };
	// 82DE05F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE05F4: 483C7BC4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE05F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE05F8 size=76
    let mut pc: u32 = 0x82DE05F8;
    'dispatch: loop {
        match pc {
            0x82DE05F8 => {
    //   block [0x82DE05F8..0x82DE0644)
	// 82DE05F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE05FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE060C: 4BFFF82D  bl 0x82ddfe38
	ctx.lr = 0x82DE0610;
	sub_82DDFE38(ctx, base);
	// 82DE0610: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0614: 39400015  li r10, 0x15
	ctx.r[10].s64 = 21;
	// 82DE0618: 396B6E80  addi r11, r11, 0x6e80
	ctx.r[11].s64 = ctx.r[11].s64 + 28288;
	// 82DE061C: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82DE0620: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DE0624: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE062C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE0630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE063C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0648 size=12
    let mut pc: u32 = 0x82DE0648;
    'dispatch: loop {
        match pc {
            0x82DE0648 => {
    //   block [0x82DE0648..0x82DE0654)
	// 82DE0648: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE064C: 386B6EE8  addi r3, r11, 0x6ee8
	ctx.r[3].s64 = ctx.r[11].s64 + 28392;
	// 82DE0650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0658 size=136
    let mut pc: u32 = 0x82DE0658;
    'dispatch: loop {
        match pc {
            0x82DE0658 => {
    //   block [0x82DE0658..0x82DE06E0)
	// 82DE0658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE065C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE066C: 38800086  li r4, 0x86
	ctx.r[4].s64 = 134;
	// 82DE0670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0674: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE0678: 4BFFF4D1  bl 0x82ddfb48
	ctx.lr = 0x82DE067C;
	sub_82DDFB48(ctx, base);
	// 82DE067C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0680: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0684: 396B6F00  addi r11, r11, 0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + 28416;
	// 82DE0688: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE068C: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82DE0690: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0694: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DE0698: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DE069C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DE06A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE06A4: 816A6324  lwz r11, 0x6324(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25380 as u32) ) } as u64;
	// 82DE06A8: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DE06AC: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE06B0: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82DE06B4: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82DE06B8: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE06BC: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DE06C0: 4BFDEAD1  bl 0x82dbf190
	ctx.lr = 0x82DE06C4;
	sub_82DBF190(ctx, base);
	// 82DE06C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE06C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE06CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE06D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE06D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE06D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE06DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE06E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE06E0 size=12
    let mut pc: u32 = 0x82DE06E0;
    'dispatch: loop {
        match pc {
            0x82DE06E0 => {
    //   block [0x82DE06E0..0x82DE06EC)
	// 82DE06E0: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE06E4: 386B6F68  addi r3, r11, 0x6f68
	ctx.r[3].s64 = ctx.r[11].s64 + 28520;
	// 82DE06E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE06F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE06F0 size=116
    let mut pc: u32 = 0x82DE06F0;
    'dispatch: loop {
        match pc {
            0x82DE06F0 => {
    //   block [0x82DE06F0..0x82DE0764)
	// 82DE06F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE06F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE06F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE06FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0708: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE070C: 4BFFF43D  bl 0x82ddfb48
	ctx.lr = 0x82DE0710;
	sub_82DDFB48(ctx, base);
	// 82DE0710: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0714: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0718: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE071C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE0720: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE0724: 394A6F70  addi r10, r10, 0x6f70
	ctx.r[10].s64 = ctx.r[10].s64 + 28528;
	// 82DE0728: 913F00A8  stw r9, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[9].u32 ) };
	// 82DE072C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE0730: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0734: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE0738: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82DE073C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE0740: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DE0744: 4BFDEA4D  bl 0x82dbf190
	ctx.lr = 0x82DE0748;
	sub_82DBF190(ctx, base);
	// 82DE0748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE074C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0758: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE075C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0768 size=12
    let mut pc: u32 = 0x82DE0768;
    'dispatch: loop {
        match pc {
            0x82DE0768 => {
    //   block [0x82DE0768..0x82DE0774)
	// 82DE0768: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE076C: 386B6FD8  addi r3, r11, 0x6fd8
	ctx.r[3].s64 = ctx.r[11].s64 + 28632;
	// 82DE0770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0778 size=120
    let mut pc: u32 = 0x82DE0778;
    'dispatch: loop {
        match pc {
            0x82DE0778 => {
    //   block [0x82DE0778..0x82DE07F0)
	// 82DE0778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE077C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE078C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0790: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE0794: 4BFFF3B5  bl 0x82ddfb48
	ctx.lr = 0x82DE0798;
	sub_82DDFB48(ctx, base);
	// 82DE0798: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE079C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82DE07A0: 396B6FE0  addi r11, r11, 0x6fe0
	ctx.r[11].s64 = ctx.r[11].s64 + 28640;
	// 82DE07A4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DE07A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE07AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE07B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE07B4: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DE07B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE07BC: 915F00A8  stw r10, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 82DE07C0: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE07C4: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82DE07C8: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE07CC: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DE07D0: 4BFDE9C1  bl 0x82dbf190
	ctx.lr = 0x82DE07D4;
	sub_82DBF190(ctx, base);
	// 82DE07D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE07D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE07DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE07E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE07E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE07E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE07EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE07F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE07F0 size=12
    let mut pc: u32 = 0x82DE07F0;
    'dispatch: loop {
        match pc {
            0x82DE07F0 => {
    //   block [0x82DE07F0..0x82DE07FC)
	// 82DE07F0: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE07F4: 386B7048  addi r3, r11, 0x7048
	ctx.r[3].s64 = ctx.r[11].s64 + 28744;
	// 82DE07F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0800 size=116
    let mut pc: u32 = 0x82DE0800;
    'dispatch: loop {
        match pc {
            0x82DE0800 => {
    //   block [0x82DE0800..0x82DE0874)
	// 82DE0800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE080C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0814: 38800088  li r4, 0x88
	ctx.r[4].s64 = 136;
	// 82DE0818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE081C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE0820: 4BFFF329  bl 0x82ddfb48
	ctx.lr = 0x82DE0824;
	sub_82DDFB48(ctx, base);
	// 82DE0824: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE082C: 396B7058  addi r11, r11, 0x7058
	ctx.r[11].s64 = ctx.r[11].s64 + 28760;
	// 82DE0830: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE0834: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE0838: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE083C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE0840: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE0844: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE0848: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82DE084C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE0850: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DE0854: 4BFDE93D  bl 0x82dbf190
	ctx.lr = 0x82DE0858;
	sub_82DBF190(ctx, base);
	// 82DE0858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE085C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0868: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE086C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0878 size=12
    let mut pc: u32 = 0x82DE0878;
    'dispatch: loop {
        match pc {
            0x82DE0878 => {
    //   block [0x82DE0878..0x82DE0884)
	// 82DE0878: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE087C: 386B70C0  addi r3, r11, 0x70c0
	ctx.r[3].s64 = ctx.r[11].s64 + 28864;
	// 82DE0880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0888 size=116
    let mut pc: u32 = 0x82DE0888;
    'dispatch: loop {
        match pc {
            0x82DE0888 => {
    //   block [0x82DE0888..0x82DE08FC)
	// 82DE0888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE089C: 38800087  li r4, 0x87
	ctx.r[4].s64 = 135;
	// 82DE08A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE08A4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE08A8: 4BFFF2A1  bl 0x82ddfb48
	ctx.lr = 0x82DE08AC;
	sub_82DDFB48(ctx, base);
	// 82DE08AC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE08B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE08B4: 396B70D0  addi r11, r11, 0x70d0
	ctx.r[11].s64 = ctx.r[11].s64 + 28880;
	// 82DE08B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE08BC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE08C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE08C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE08C8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE08CC: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE08D0: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 82DE08D4: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE08D8: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DE08DC: 4BFDE8B5  bl 0x82dbf190
	ctx.lr = 0x82DE08E0;
	sub_82DBF190(ctx, base);
	// 82DE08E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE08E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE08E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE08EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE08F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE08F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE08F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0900 size=12
    let mut pc: u32 = 0x82DE0900;
    'dispatch: loop {
        match pc {
            0x82DE0900 => {
    //   block [0x82DE0900..0x82DE090C)
	// 82DE0900: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0904: 386B7138  addi r3, r11, 0x7138
	ctx.r[3].s64 = ctx.r[11].s64 + 28984;
	// 82DE0908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0910 size=84
    let mut pc: u32 = 0x82DE0910;
    'dispatch: loop {
        match pc {
            0x82DE0910 => {
    //   block [0x82DE0910..0x82DE0964)
	// 82DE0910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0914: 483C7859  bl 0x831a816c
	ctx.lr = 0x82DE0918;
	sub_831A8130(ctx, base);
	// 82DE0918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE091C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE0920: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE0924: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DE0928: 3880008B  li r4, 0x8b
	ctx.r[4].s64 = 139;
	// 82DE092C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0930: 4BFFF219  bl 0x82ddfb48
	ctx.lr = 0x82DE0934;
	sub_82DDFB48(ctx, base);
	// 82DE0934: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE093C: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82DE0940: 396B7148  addi r11, r11, 0x7148
	ctx.r[11].s64 = ctx.r[11].s64 + 29000;
	// 82DE0944: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 82DE0948: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE094C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0950: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE0954: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82DE0958: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE095C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0960: 483C785C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0968 size=84
    let mut pc: u32 = 0x82DE0968;
    'dispatch: loop {
        match pc {
            0x82DE0968 => {
    //   block [0x82DE0968..0x82DE09BC)
	// 82DE0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE096C: 483C7801  bl 0x831a816c
	ctx.lr = 0x82DE0970;
	sub_831A8130(ctx, base);
	// 82DE0970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0974: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE0978: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE097C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DE0980: 3880008A  li r4, 0x8a
	ctx.r[4].s64 = 138;
	// 82DE0984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0988: 4BFFF1C1  bl 0x82ddfb48
	ctx.lr = 0x82DE098C;
	sub_82DDFB48(ctx, base);
	// 82DE098C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE0994: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82DE0998: 396B71B0  addi r11, r11, 0x71b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29104;
	// 82DE099C: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 82DE09A0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE09A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE09A8: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE09AC: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82DE09B0: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE09B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE09B8: 483C7804  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE09C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE09C0 size=84
    let mut pc: u32 = 0x82DE09C0;
    'dispatch: loop {
        match pc {
            0x82DE09C0 => {
    //   block [0x82DE09C0..0x82DE0A14)
	// 82DE09C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE09C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE09C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE09CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE09D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE09D4: 3880008D  li r4, 0x8d
	ctx.r[4].s64 = 141;
	// 82DE09D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE09DC: 4BFFF16D  bl 0x82ddfb48
	ctx.lr = 0x82DE09E0;
	sub_82DDFB48(ctx, base);
	// 82DE09E0: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE09E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE09E8: 396B7218  addi r11, r11, 0x7218
	ctx.r[11].s64 = ctx.r[11].s64 + 29208;
	// 82DE09EC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE09F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE09F4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE09F8: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82DE09FC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE0A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0A18 size=84
    let mut pc: u32 = 0x82DE0A18;
    'dispatch: loop {
        match pc {
            0x82DE0A18 => {
    //   block [0x82DE0A18..0x82DE0A6C)
	// 82DE0A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0A28: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE0A2C: 3880008C  li r4, 0x8c
	ctx.r[4].s64 = 140;
	// 82DE0A30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0A34: 4BFFF115  bl 0x82ddfb48
	ctx.lr = 0x82DE0A38;
	sub_82DDFB48(ctx, base);
	// 82DE0A38: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0A3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE0A40: 396B7280  addi r11, r11, 0x7280
	ctx.r[11].s64 = ctx.r[11].s64 + 29312;
	// 82DE0A44: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE0A48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0A4C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DE0A50: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82DE0A54: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82DE0A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0A64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0A70 size=104
    let mut pc: u32 = 0x82DE0A70;
    'dispatch: loop {
        match pc {
            0x82DE0A70 => {
    //   block [0x82DE0A70..0x82DE0AD8)
	// 82DE0A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0A74: 483C76F9  bl 0x831a816c
	ctx.lr = 0x82DE0A78;
	sub_831A8130(ctx, base);
	// 82DE0A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0A7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0A80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0A84: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0A88: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0A8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0A90: 4BFC4AF1  bl 0x82da5580
	ctx.lr = 0x82DE0A94;
	sub_82DA5580(ctx, base);
	// 82DE0A94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0A98: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0A9C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0AA0: 4182002C  beq 0x82de0acc
	if ctx.cr[0].eq {
	pc = 0x82DE0ACC; continue 'dispatch;
	}
	// 82DE0AA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0AA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0AAC: 4BFFF09D  bl 0x82ddfb48
	ctx.lr = 0x82DE0AB0;
	sub_82DDFB48(ctx, base);
	// 82DE0AB0: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0AB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0AB8: 394A64C8  addi r10, r10, 0x64c8
	ctx.r[10].s64 = ctx.r[10].s64 + 25800;
	// 82DE0ABC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE0AC0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0AC4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE0AC8: 48000008  b 0x82de0ad0
	pc = 0x82DE0AD0; continue 'dispatch;
	// 82DE0ACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0AD4: 483C76E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0AD8 size=80
    let mut pc: u32 = 0x82DE0AD8;
    'dispatch: loop {
        match pc {
            0x82DE0AD8 => {
    //   block [0x82DE0AD8..0x82DE0B28)
	// 82DE0AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0ADC: 483C7691  bl 0x831a816c
	ctx.lr = 0x82DE0AE0;
	sub_831A8130(ctx, base);
	// 82DE0AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0AE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0AE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0AEC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0AF0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0AF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0AF8: 4BFC4A89  bl 0x82da5580
	ctx.lr = 0x82DE0AFC;
	sub_82DA5580(ctx, base);
	// 82DE0AFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0B00: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0B04: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0B08: 41820014  beq 0x82de0b1c
	if ctx.cr[0].eq {
	pc = 0x82DE0B1C; continue 'dispatch;
	}
	// 82DE0B0C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0B10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0B14: 4BFFF2BD  bl 0x82ddfdd0
	ctx.lr = 0x82DE0B18;
	sub_82DDFDD0(ctx, base);
	// 82DE0B18: 48000008  b 0x82de0b20
	pc = 0x82DE0B20; continue 'dispatch;
	// 82DE0B1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0B24: 483C7698  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0B28 size=108
    let mut pc: u32 = 0x82DE0B28;
    'dispatch: loop {
        match pc {
            0x82DE0B28 => {
    //   block [0x82DE0B28..0x82DE0B94)
	// 82DE0B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0B2C: 483C7641  bl 0x831a816c
	ctx.lr = 0x82DE0B30;
	sub_831A8130(ctx, base);
	// 82DE0B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0B34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0B38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0B3C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0B40: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0B44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0B48: 4BFC4A39  bl 0x82da5580
	ctx.lr = 0x82DE0B4C;
	sub_82DA5580(ctx, base);
	// 82DE0B4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0B50: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0B54: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0B58: 41820030  beq 0x82de0b88
	if ctx.cr[0].eq {
	pc = 0x82DE0B88; continue 'dispatch;
	}
	// 82DE0B5C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0B64: 4BFFEFE5  bl 0x82ddfb48
	ctx.lr = 0x82DE0B68;
	sub_82DDFB48(ctx, base);
	// 82DE0B68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE0B6C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0B70: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE0B74: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82DE0B78: 396B6530  addi r11, r11, 0x6530
	ctx.r[11].s64 = ctx.r[11].s64 + 25904;
	// 82DE0B7C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DE0B80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0B84: 48000008  b 0x82de0b8c
	pc = 0x82DE0B8C; continue 'dispatch;
	// 82DE0B88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0B8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0B90: 483C762C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0B98 size=108
    let mut pc: u32 = 0x82DE0B98;
    'dispatch: loop {
        match pc {
            0x82DE0B98 => {
    //   block [0x82DE0B98..0x82DE0C04)
	// 82DE0B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0B9C: 483C75D1  bl 0x831a816c
	ctx.lr = 0x82DE0BA0;
	sub_831A8130(ctx, base);
	// 82DE0BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0BA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0BA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0BAC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0BB0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0BB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0BB8: 4BFC49C9  bl 0x82da5580
	ctx.lr = 0x82DE0BBC;
	sub_82DA5580(ctx, base);
	// 82DE0BBC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0BC0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0BC4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0BC8: 41820030  beq 0x82de0bf8
	if ctx.cr[0].eq {
	pc = 0x82DE0BF8; continue 'dispatch;
	}
	// 82DE0BCC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0BD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0BD4: 4BFFEF75  bl 0x82ddfb48
	ctx.lr = 0x82DE0BD8;
	sub_82DDFB48(ctx, base);
	// 82DE0BD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE0BDC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0BE0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE0BE4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DE0BE8: 396B3148  addi r11, r11, 0x3148
	ctx.r[11].s64 = ctx.r[11].s64 + 12616;
	// 82DE0BEC: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DE0BF0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0BF4: 48000008  b 0x82de0bfc
	pc = 0x82DE0BFC; continue 'dispatch;
	// 82DE0BF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0BFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0C00: 483C75BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0C08 size=80
    let mut pc: u32 = 0x82DE0C08;
    'dispatch: loop {
        match pc {
            0x82DE0C08 => {
    //   block [0x82DE0C08..0x82DE0C58)
	// 82DE0C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0C0C: 483C7561  bl 0x831a816c
	ctx.lr = 0x82DE0C10;
	sub_831A8130(ctx, base);
	// 82DE0C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0C14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0C18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0C1C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0C20: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0C24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0C28: 4BFC4959  bl 0x82da5580
	ctx.lr = 0x82DE0C2C;
	sub_82DA5580(ctx, base);
	// 82DE0C2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0C30: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0C34: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0C38: 41820014  beq 0x82de0c4c
	if ctx.cr[0].eq {
	pc = 0x82DE0C4C; continue 'dispatch;
	}
	// 82DE0C3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0C40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0C44: 4BFFFAAD  bl 0x82de06f0
	ctx.lr = 0x82DE0C48;
	sub_82DE06F0(ctx, base);
	// 82DE0C48: 48000008  b 0x82de0c50
	pc = 0x82DE0C50; continue 'dispatch;
	// 82DE0C4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0C50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0C54: 483C7568  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0C58 size=80
    let mut pc: u32 = 0x82DE0C58;
    'dispatch: loop {
        match pc {
            0x82DE0C58 => {
    //   block [0x82DE0C58..0x82DE0CA8)
	// 82DE0C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0C5C: 483C7511  bl 0x831a816c
	ctx.lr = 0x82DE0C60;
	sub_831A8130(ctx, base);
	// 82DE0C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0C64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0C68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0C6C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0C70: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0C74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0C78: 4BFC4909  bl 0x82da5580
	ctx.lr = 0x82DE0C7C;
	sub_82DA5580(ctx, base);
	// 82DE0C7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0C80: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0C84: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0C88: 41820014  beq 0x82de0c9c
	if ctx.cr[0].eq {
	pc = 0x82DE0C9C; continue 'dispatch;
	}
	// 82DE0C8C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0C90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0C94: 4BFFFAE5  bl 0x82de0778
	ctx.lr = 0x82DE0C98;
	sub_82DE0778(ctx, base);
	// 82DE0C98: 48000008  b 0x82de0ca0
	pc = 0x82DE0CA0; continue 'dispatch;
	// 82DE0C9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0CA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0CA4: 483C7518  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0CA8 size=80
    let mut pc: u32 = 0x82DE0CA8;
    'dispatch: loop {
        match pc {
            0x82DE0CA8 => {
    //   block [0x82DE0CA8..0x82DE0CF8)
	// 82DE0CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0CAC: 483C74C1  bl 0x831a816c
	ctx.lr = 0x82DE0CB0;
	sub_831A8130(ctx, base);
	// 82DE0CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0CB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0CB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0CBC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0CC0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0CC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0CC8: 4BFC48B9  bl 0x82da5580
	ctx.lr = 0x82DE0CCC;
	sub_82DA5580(ctx, base);
	// 82DE0CCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0CD0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0CD4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0CD8: 41820014  beq 0x82de0cec
	if ctx.cr[0].eq {
	pc = 0x82DE0CEC; continue 'dispatch;
	}
	// 82DE0CDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0CE4: 4BFFF975  bl 0x82de0658
	ctx.lr = 0x82DE0CE8;
	sub_82DE0658(ctx, base);
	// 82DE0CE8: 48000008  b 0x82de0cf0
	pc = 0x82DE0CF0; continue 'dispatch;
	// 82DE0CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0CF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0CF4: 483C74C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0CF8 size=80
    let mut pc: u32 = 0x82DE0CF8;
    'dispatch: loop {
        match pc {
            0x82DE0CF8 => {
    //   block [0x82DE0CF8..0x82DE0D48)
	// 82DE0CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0CFC: 483C7471  bl 0x831a816c
	ctx.lr = 0x82DE0D00;
	sub_831A8130(ctx, base);
	// 82DE0D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0D04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0D08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0D0C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0D10: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0D14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0D18: 4BFC4869  bl 0x82da5580
	ctx.lr = 0x82DE0D1C;
	sub_82DA5580(ctx, base);
	// 82DE0D1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0D20: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0D24: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0D28: 41820014  beq 0x82de0d3c
	if ctx.cr[0].eq {
	pc = 0x82DE0D3C; continue 'dispatch;
	}
	// 82DE0D2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0D30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0D34: 4BFFFACD  bl 0x82de0800
	ctx.lr = 0x82DE0D38;
	sub_82DE0800(ctx, base);
	// 82DE0D38: 48000008  b 0x82de0d40
	pc = 0x82DE0D40; continue 'dispatch;
	// 82DE0D3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0D40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0D44: 483C7478  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0D48 size=80
    let mut pc: u32 = 0x82DE0D48;
    'dispatch: loop {
        match pc {
            0x82DE0D48 => {
    //   block [0x82DE0D48..0x82DE0D98)
	// 82DE0D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0D4C: 483C7421  bl 0x831a816c
	ctx.lr = 0x82DE0D50;
	sub_831A8130(ctx, base);
	// 82DE0D50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0D54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0D58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0D5C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0D60: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0D64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0D68: 4BFC4819  bl 0x82da5580
	ctx.lr = 0x82DE0D6C;
	sub_82DA5580(ctx, base);
	// 82DE0D6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0D70: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0D74: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0D78: 41820014  beq 0x82de0d8c
	if ctx.cr[0].eq {
	pc = 0x82DE0D8C; continue 'dispatch;
	}
	// 82DE0D7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0D80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0D84: 4BFFFB05  bl 0x82de0888
	ctx.lr = 0x82DE0D88;
	sub_82DE0888(ctx, base);
	// 82DE0D88: 48000008  b 0x82de0d90
	pc = 0x82DE0D90; continue 'dispatch;
	// 82DE0D8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0D90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0D94: 483C7428  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0D98 size=124
    let mut pc: u32 = 0x82DE0D98;
    'dispatch: loop {
        match pc {
            0x82DE0D98 => {
    //   block [0x82DE0D98..0x82DE0E14)
	// 82DE0D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0DAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0DB0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0DB4: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0DB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE0DBC: 4BFC47C5  bl 0x82da5580
	ctx.lr = 0x82DE0DC0;
	sub_82DA5580(ctx, base);
	// 82DE0DC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0DC4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0DC8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE0DCC: 4182002C  beq 0x82de0df8
	if ctx.cr[0].eq {
	pc = 0x82DE0DF8; continue 'dispatch;
	}
	// 82DE0DD0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0DD4: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DE0DD8: 4BFFED71  bl 0x82ddfb48
	ctx.lr = 0x82DE0DDC;
	sub_82DDFB48(ctx, base);
	// 82DE0DDC: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0DE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0DE4: 394A6998  addi r10, r10, 0x6998
	ctx.r[10].s64 = ctx.r[10].s64 + 27032;
	// 82DE0DE8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE0DEC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0DF0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE0DF4: 48000008  b 0x82de0dfc
	pc = 0x82DE0DFC; continue 'dispatch;
	// 82DE0DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0DFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0E08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE0E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0E18 size=80
    let mut pc: u32 = 0x82DE0E18;
    'dispatch: loop {
        match pc {
            0x82DE0E18 => {
    //   block [0x82DE0E18..0x82DE0E68)
	// 82DE0E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0E1C: 483C7351  bl 0x831a816c
	ctx.lr = 0x82DE0E20;
	sub_831A8130(ctx, base);
	// 82DE0E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0E24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0E28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0E2C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0E30: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0E34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0E38: 4BFC4749  bl 0x82da5580
	ctx.lr = 0x82DE0E3C;
	sub_82DA5580(ctx, base);
	// 82DE0E3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0E40: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0E44: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE0E48: 41820014  beq 0x82de0e5c
	if ctx.cr[0].eq {
	pc = 0x82DE0E5C; continue 'dispatch;
	}
	// 82DE0E4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0E50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE0E54: 4BFFF465  bl 0x82de02b8
	ctx.lr = 0x82DE0E58;
	sub_82DE02B8(ctx, base);
	// 82DE0E58: 48000008  b 0x82de0e60
	pc = 0x82DE0E60; continue 'dispatch;
	// 82DE0E5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0E64: 483C7358  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0E68 size=124
    let mut pc: u32 = 0x82DE0E68;
    'dispatch: loop {
        match pc {
            0x82DE0E68 => {
    //   block [0x82DE0E68..0x82DE0EE4)
	// 82DE0E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0E70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0E74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0E78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0E7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0E80: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0E84: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0E88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE0E8C: 4BFC46F5  bl 0x82da5580
	ctx.lr = 0x82DE0E90;
	sub_82DA5580(ctx, base);
	// 82DE0E90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0E94: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0E98: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE0E9C: 4182002C  beq 0x82de0ec8
	if ctx.cr[0].eq {
	pc = 0x82DE0EC8; continue 'dispatch;
	}
	// 82DE0EA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0EA4: 3880007E  li r4, 0x7e
	ctx.r[4].s64 = 126;
	// 82DE0EA8: 4BFFECA1  bl 0x82ddfb48
	ctx.lr = 0x82DE0EAC;
	sub_82DDFB48(ctx, base);
	// 82DE0EAC: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE0EB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE0EB4: 394A6AF0  addi r10, r10, 0x6af0
	ctx.r[10].s64 = ctx.r[10].s64 + 27376;
	// 82DE0EB8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE0EBC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0EC0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE0EC4: 48000008  b 0x82de0ecc
	pc = 0x82DE0ECC; continue 'dispatch;
	// 82DE0EC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0ECC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0ED8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE0EDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0EE8 size=112
    let mut pc: u32 = 0x82DE0EE8;
    'dispatch: loop {
        match pc {
            0x82DE0EE8 => {
    //   block [0x82DE0EE8..0x82DE0F58)
	// 82DE0EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0EF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0EF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0EFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0F00: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0F04: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0F08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE0F0C: 4BFC4675  bl 0x82da5580
	ctx.lr = 0x82DE0F10;
	sub_82DA5580(ctx, base);
	// 82DE0F10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0F14: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0F18: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE0F1C: 41820020  beq 0x82de0f3c
	if ctx.cr[0].eq {
	pc = 0x82DE0F3C; continue 'dispatch;
	}
	// 82DE0F20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0F24: 3880008E  li r4, 0x8e
	ctx.r[4].s64 = 142;
	// 82DE0F28: 4BFFF4B9  bl 0x82de03e0
	ctx.lr = 0x82DE0F2C;
	sub_82DE03E0(ctx, base);
	// 82DE0F2C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0F30: 396B6C48  addi r11, r11, 0x6c48
	ctx.r[11].s64 = ctx.r[11].s64 + 27720;
	// 82DE0F34: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0F38: 48000008  b 0x82de0f40
	pc = 0x82DE0F40; continue 'dispatch;
	// 82DE0F3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0F40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0F4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE0F50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0F58 size=112
    let mut pc: u32 = 0x82DE0F58;
    'dispatch: loop {
        match pc {
            0x82DE0F58 => {
    //   block [0x82DE0F58..0x82DE0FC8)
	// 82DE0F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0F60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0F64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0F68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0F6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0F70: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0F74: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE0F7C: 4BFC4605  bl 0x82da5580
	ctx.lr = 0x82DE0F80;
	sub_82DA5580(ctx, base);
	// 82DE0F80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0F84: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0F88: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE0F8C: 41820020  beq 0x82de0fac
	if ctx.cr[0].eq {
	pc = 0x82DE0FAC; continue 'dispatch;
	}
	// 82DE0F90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE0F94: 3880008F  li r4, 0x8f
	ctx.r[4].s64 = 143;
	// 82DE0F98: 4BFFF449  bl 0x82de03e0
	ctx.lr = 0x82DE0F9C;
	sub_82DE03E0(ctx, base);
	// 82DE0F9C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE0FA0: 396B6CC0  addi r11, r11, 0x6cc0
	ctx.r[11].s64 = ctx.r[11].s64 + 27840;
	// 82DE0FA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0FA8: 48000008  b 0x82de0fb0
	pc = 0x82DE0FB0; continue 'dispatch;
	// 82DE0FAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE0FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0FBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE0FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0FC8 size=112
    let mut pc: u32 = 0x82DE0FC8;
    'dispatch: loop {
        match pc {
            0x82DE0FC8 => {
    //   block [0x82DE0FC8..0x82DE1038)
	// 82DE0FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0FD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE0FD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0FD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0FDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE0FE0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE0FE4: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE0FE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE0FEC: 4BFC4595  bl 0x82da5580
	ctx.lr = 0x82DE0FF0;
	sub_82DA5580(ctx, base);
	// 82DE0FF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0FF4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE0FF8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE0FFC: 41820020  beq 0x82de101c
	if ctx.cr[0].eq {
	pc = 0x82DE101C; continue 'dispatch;
	}
	// 82DE1000: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE1004: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82DE1008: 4BFFF3D9  bl 0x82de03e0
	ctx.lr = 0x82DE100C;
	sub_82DE03E0(ctx, base);
	// 82DE100C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE1010: 396B6D38  addi r11, r11, 0x6d38
	ctx.r[11].s64 = ctx.r[11].s64 + 27960;
	// 82DE1014: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE1018: 48000008  b 0x82de1020
	pc = 0x82DE1020; continue 'dispatch;
	// 82DE101C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE102C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1038 size=112
    let mut pc: u32 = 0x82DE1038;
    'dispatch: loop {
        match pc {
            0x82DE1038 => {
    //   block [0x82DE1038..0x82DE10A8)
	// 82DE1038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE103C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE1044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE104C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1050: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE1054: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE1058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE105C: 4BFC4525  bl 0x82da5580
	ctx.lr = 0x82DE1060;
	sub_82DA5580(ctx, base);
	// 82DE1060: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1064: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE1068: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE106C: 41820020  beq 0x82de108c
	if ctx.cr[0].eq {
	pc = 0x82DE108C; continue 'dispatch;
	}
	// 82DE1070: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE1074: 38800091  li r4, 0x91
	ctx.r[4].s64 = 145;
	// 82DE1078: 4BFFF369  bl 0x82de03e0
	ctx.lr = 0x82DE107C;
	sub_82DE03E0(ctx, base);
	// 82DE107C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE1080: 396B6BD0  addi r11, r11, 0x6bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 27600;
	// 82DE1084: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE1088: 48000008  b 0x82de1090
	pc = 0x82DE1090; continue 'dispatch;
	// 82DE108C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE109C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE10A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE10A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE10A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE10A8 size=108
    let mut pc: u32 = 0x82DE10A8;
    'dispatch: loop {
        match pc {
            0x82DE10A8 => {
    //   block [0x82DE10A8..0x82DE1114)
	// 82DE10A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE10AC: 483C70C1  bl 0x831a816c
	ctx.lr = 0x82DE10B0;
	sub_831A8130(ctx, base);
	// 82DE10B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE10B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE10B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE10BC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE10C0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE10C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE10C8: 4BFC44B9  bl 0x82da5580
	ctx.lr = 0x82DE10CC;
	sub_82DA5580(ctx, base);
	// 82DE10CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE10D0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE10D4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE10D8: 41820030  beq 0x82de1108
	if ctx.cr[0].eq {
	pc = 0x82DE1108; continue 'dispatch;
	}
	// 82DE10DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE10E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE10E4: 4BFFEA65  bl 0x82ddfb48
	ctx.lr = 0x82DE10E8;
	sub_82DDFB48(ctx, base);
	// 82DE10E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE10EC: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE10F0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DE10F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE10F8: 396B72E8  addi r11, r11, 0x72e8
	ctx.r[11].s64 = ctx.r[11].s64 + 29416;
	// 82DE10FC: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DE1100: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE1104: 48000008  b 0x82de110c
	pc = 0x82DE110C; continue 'dispatch;
	// 82DE1108: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE110C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1110: 483C70AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1118 size=80
    let mut pc: u32 = 0x82DE1118;
    'dispatch: loop {
        match pc {
            0x82DE1118 => {
    //   block [0x82DE1118..0x82DE1168)
	// 82DE1118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE111C: 483C7051  bl 0x831a816c
	ctx.lr = 0x82DE1120;
	sub_831A8130(ctx, base);
	// 82DE1120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1124: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1128: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE112C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE1130: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE1134: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE1138: 4BFC4449  bl 0x82da5580
	ctx.lr = 0x82DE113C;
	sub_82DA5580(ctx, base);
	// 82DE113C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1140: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE1144: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE1148: 41820014  beq 0x82de115c
	if ctx.cr[0].eq {
	pc = 0x82DE115C; continue 'dispatch;
	}
	// 82DE114C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE1150: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1154: 4BFFEBCD  bl 0x82ddfd20
	ctx.lr = 0x82DE1158;
	sub_82DDFD20(ctx, base);
	// 82DE1158: 48000008  b 0x82de1160
	pc = 0x82DE1160; continue 'dispatch;
	// 82DE115C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1164: 483C7058  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1168 size=96
    let mut pc: u32 = 0x82DE1168;
    'dispatch: loop {
        match pc {
            0x82DE1168 => {
    //   block [0x82DE1168..0x82DE11C8)
	// 82DE1168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE116C: 483C6FF9  bl 0x831a8164
	ctx.lr = 0x82DE1170;
	sub_831A8130(ctx, base);
	// 82DE1170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1174: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1178: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE117C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE1180: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE1184: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DE1188: 837F05AC  lwz r27, 0x5ac(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE118C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DE1190: 4BFC43F1  bl 0x82da5580
	ctx.lr = 0x82DE1194;
	sub_82DA5580(ctx, base);
	// 82DE1194: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1198: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE119C: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DE11A0: 4182001C  beq 0x82de11bc
	if ctx.cr[0].eq {
	pc = 0x82DE11BC; continue 'dispatch;
	}
	// 82DE11A4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DE11A8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DE11AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE11B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE11B4: 4BFFF315  bl 0x82de04c8
	ctx.lr = 0x82DE11B8;
	sub_82DE04C8(ctx, base);
	// 82DE11B8: 48000008  b 0x82de11c0
	pc = 0x82DE11C0; continue 'dispatch;
	// 82DE11BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE11C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE11C4: 483C6FF0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE11C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE11C8 size=80
    let mut pc: u32 = 0x82DE11C8;
    'dispatch: loop {
        match pc {
            0x82DE11C8 => {
    //   block [0x82DE11C8..0x82DE1218)
	// 82DE11C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE11CC: 483C6FA1  bl 0x831a816c
	ctx.lr = 0x82DE11D0;
	sub_831A8130(ctx, base);
	// 82DE11D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE11D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE11D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE11DC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE11E0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE11E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE11E8: 4BFC4399  bl 0x82da5580
	ctx.lr = 0x82DE11EC;
	sub_82DA5580(ctx, base);
	// 82DE11EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE11F0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE11F4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE11F8: 41820014  beq 0x82de120c
	if ctx.cr[0].eq {
	pc = 0x82DE120C; continue 'dispatch;
	}
	// 82DE11FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE1200: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1204: 4BFFF335  bl 0x82de0538
	ctx.lr = 0x82DE1208;
	sub_82DE0538(ctx, base);
	// 82DE1208: 48000008  b 0x82de1210
	pc = 0x82DE1210; continue 'dispatch;
	// 82DE120C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1210: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1214: 483C6FA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1218 size=88
    let mut pc: u32 = 0x82DE1218;
    'dispatch: loop {
        match pc {
            0x82DE1218 => {
    //   block [0x82DE1218..0x82DE1270)
	// 82DE1218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE121C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1220: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1224: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1228: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE122C: 4BFFE91D  bl 0x82ddfb48
	ctx.lr = 0x82DE1230;
	sub_82DDFB48(ctx, base);
	// 82DE1230: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE1234: 3D208211  lis r9, -0x7def
	ctx.r[9].s64 = -2112815104;
	// 82DE1238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE123C: 394A72E8  addi r10, r10, 0x72e8
	ctx.r[10].s64 = ctx.r[10].s64 + 29416;
	// 82DE1240: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE1244: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE1248: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE124C: 81496324  lwz r10, 0x6324(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(25380 as u32) ) } as u64;
	// 82DE1250: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE1254: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DE1258: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DE125C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE1260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE126C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1270 size=80
    let mut pc: u32 = 0x82DE1270;
    'dispatch: loop {
        match pc {
            0x82DE1270 => {
    //   block [0x82DE1270..0x82DE12C0)
	// 82DE1270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1274: 483C6EF9  bl 0x831a816c
	ctx.lr = 0x82DE1278;
	sub_831A8130(ctx, base);
	// 82DE1278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE127C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1280: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE1284: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DE1288: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DE128C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE1290: 4BFC42F1  bl 0x82da5580
	ctx.lr = 0x82DE1294;
	sub_82DA5580(ctx, base);
	// 82DE1294: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1298: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE129C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE12A0: 41820014  beq 0x82de12b4
	if ctx.cr[0].eq {
	pc = 0x82DE12B4; continue 'dispatch;
	}
	// 82DE12A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE12A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE12AC: 4BFFFF6D  bl 0x82de1218
	ctx.lr = 0x82DE12B0;
	sub_82DE1218(ctx, base);
	// 82DE12B0: 48000008  b 0x82de12b8
	pc = 0x82DE12B8; continue 'dispatch;
	// 82DE12B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE12B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE12BC: 483C6F00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE12C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE12C0 size=240
    let mut pc: u32 = 0x82DE12C0;
    'dispatch: loop {
        match pc {
            0x82DE12C0 => {
    //   block [0x82DE12C0..0x82DE13B0)
	// 82DE12C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE12C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE12C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE12CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE12D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE12D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE12D8: 38640080  addi r3, r4, 0x80
	ctx.r[3].s64 = ctx.r[4].s64 + 128;
	// 82DE12DC: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82DE12E0: 4BFFB2D9  bl 0x82ddc5b8
	ctx.lr = 0x82DE12E4;
	sub_82DDC5B8(ctx, base);
	// 82DE12E4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE12E8: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82DE12EC: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82DE12F0: 555E103A  slwi r30, r10, 2
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE12F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE12F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE12FC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DE1300: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1304: 4BFC427D  bl 0x82da5580
	ctx.lr = 0x82DE1308;
	sub_82DA5580(ctx, base);
	// 82DE1308: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DE130C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1310: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1314: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1318: 4BFC4269  bl 0x82da5580
	ctx.lr = 0x82DE131C;
	sub_82DA5580(ctx, base);
	// 82DE131C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DE1320: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1324: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1328: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE132C: 4BFC4255  bl 0x82da5580
	ctx.lr = 0x82DE1330;
	sub_82DA5580(ctx, base);
	// 82DE1330: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82DE1334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1338: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE133C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1340: 4BFC4241  bl 0x82da5580
	ctx.lr = 0x82DE1344;
	sub_82DA5580(ctx, base);
	// 82DE1344: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82DE1348: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE134C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1350: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1354: 4BFC422D  bl 0x82da5580
	ctx.lr = 0x82DE1358;
	sub_82DA5580(ctx, base);
	// 82DE1358: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DE135C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1360: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1364: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE1368: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DE136C: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1370: 4BFC4211  bl 0x82da5580
	ctx.lr = 0x82DE1374;
	sub_82DA5580(ctx, base);
	// 82DE1374: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82DE1378: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE137C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1380: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE1384: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DE1388: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE138C: 4BFC41F5  bl 0x82da5580
	ctx.lr = 0x82DE1390;
	sub_82DA5580(ctx, base);
	// 82DE1390: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82DE1394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1398: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE139C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE13A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE13A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE13A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE13AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE13B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE13B0 size=276
    let mut pc: u32 = 0x82DE13B0;
    'dispatch: loop {
        match pc {
            0x82DE13B0 => {
    //   block [0x82DE13B0..0x82DE14C4)
	// 82DE13B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE13B4: 483C6DB5  bl 0x831a8168
	ctx.lr = 0x82DE13B8;
	sub_831A8130(ctx, base);
	// 82DE13B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE13BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE13C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DE13C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE13C8: 4800003C  b 0x82de1404
	pc = 0x82DE1404; continue 'dispatch;
	// 82DE13CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE13D0: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE13D4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE13D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DE13DC: 419A0038  beq cr6, 0x82de1414
	if ctx.cr[6].eq {
	pc = 0x82DE1414; continue 'dispatch;
	}
	// 82DE13E0: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE13E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DE13E8: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE13EC: 41820010  beq 0x82de13fc
	if ctx.cr[0].eq {
	pc = 0x82DE13FC; continue 'dispatch;
	}
	// 82DE13F0: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82DE13F4: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DE13F8: 4BFC4311  bl 0x82da5708
	ctx.lr = 0x82DE13FC;
	sub_82DA5708(ctx, base);
	// 82DE13FC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE1400: 7FBE592E  stwx r29, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82DE1404: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE1408: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE140C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1410: 409AFFBC  bne cr6, 0x82de13cc
	if !ctx.cr[6].eq {
	pc = 0x82DE13CC; continue 'dispatch;
	}
	// 82DE1414: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE1418: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE141C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE1420: 419A0010  beq cr6, 0x82de1430
	if ctx.cr[6].eq {
	pc = 0x82DE1430; continue 'dispatch;
	}
	// 82DE1424: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82DE1428: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DE142C: 4BFC42DD  bl 0x82da5708
	ctx.lr = 0x82DE1430;
	sub_82DA5708(ctx, base);
	// 82DE1430: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1434: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DE1438: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DE143C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DE1440: 4099FFC4  ble cr6, 0x82de1404
	if !ctx.cr[6].gt {
	pc = 0x82DE1404; continue 'dispatch;
	}
	// 82DE1444: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1448: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE144C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1450: 4BFC42B9  bl 0x82da5708
	ctx.lr = 0x82DE1454;
	sub_82DA5708(ctx, base);
	// 82DE1454: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1458: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE145C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1460: 4BFC42A9  bl 0x82da5708
	ctx.lr = 0x82DE1464;
	sub_82DA5708(ctx, base);
	// 82DE1464: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1468: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE146C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1470: 4BFC4299  bl 0x82da5708
	ctx.lr = 0x82DE1474;
	sub_82DA5708(ctx, base);
	// 82DE1474: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1478: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE147C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1480: 4BFC4289  bl 0x82da5708
	ctx.lr = 0x82DE1484;
	sub_82DA5708(ctx, base);
	// 82DE1484: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1488: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE148C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1490: 4BFC4279  bl 0x82da5708
	ctx.lr = 0x82DE1494;
	sub_82DA5708(ctx, base);
	// 82DE1494: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1498: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE149C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE14A0: 4BFC4269  bl 0x82da5708
	ctx.lr = 0x82DE14A4;
	sub_82DA5708(ctx, base);
	// 82DE14A4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE14A8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE14AC: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE14B0: 4BFC4259  bl 0x82da5708
	ctx.lr = 0x82DE14B4;
	sub_82DA5708(ctx, base);
	// 82DE14B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE14B8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DE14BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE14C0: 483C6CF8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE14C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE14C8 size=356
    let mut pc: u32 = 0x82DE14C8;
    'dispatch: loop {
        match pc {
            0x82DE14C8 => {
    //   block [0x82DE14C8..0x82DE162C)
	// 82DE14C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE14CC: 483C6C81  bl 0x831a814c
	ctx.lr = 0x82DE14D0;
	sub_831A8130(ctx, base);
	// 82DE14D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE14D4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DE14D8: 80770020  lwz r3, 0x20(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE14DC: 4BFE16AD  bl 0x82dc2b88
	ctx.lr = 0x82DE14E0;
	sub_82DC2B88(ctx, base);
	// 82DE14E0: 8177001C  lwz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE14E4: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 82DE14E8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DE14EC: 41980138  blt cr6, 0x82de1624
	if ctx.cr[6].lt {
	pc = 0x82DE1624; continue 'dispatch;
	}
	// 82DE14F0: 3AC30004  addi r22, r3, 4
	ctx.r[22].s64 = ctx.r[3].s64 + 4;
	// 82DE14F4: 83360000  lwz r25, 0(r22)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE14F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE14FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE1500: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DE1504: 83190048  lwz r24, 0x48(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1508: 81790038  lwz r11, 0x38(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DE150C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1510: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE1514: 4099000C  ble cr6, 0x82de1520
	if !ctx.cr[6].gt {
	pc = 0x82DE1520; continue 'dispatch;
	}
	// 82DE1518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE151C: 48000010  b 0x82de152c
	pc = 0x82DE152C; continue 'dispatch;
	// 82DE1520: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1524: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE1528: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE152C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE1530: 41820034  beq 0x82de1564
	if ctx.cr[0].eq {
	pc = 0x82DE1564; continue 'dispatch;
	}
	// 82DE1534: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1538: 81570008  lwz r10, 8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE153C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1540: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1544: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DE1548: 419A0010  beq cr6, 0x82de1558
	if ctx.cr[6].eq {
	pc = 0x82DE1558; continue 'dispatch;
	}
	// 82DE154C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE1550: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DE1554: 48000CED  bl 0x82de2240
	ctx.lr = 0x82DE1558;
	sub_82DE2240(ctx, base);
	// 82DE1558: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DE155C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DE1560: 4BFFFFA8  b 0x82de1508
	pc = 0x82DE1508; continue 'dispatch;
	// 82DE1564: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DE1568: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DE156C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DE1570: 81790058  lwz r11, 0x58(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DE1574: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1578: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE157C: 4099000C  ble cr6, 0x82de1588
	if !ctx.cr[6].gt {
	pc = 0x82DE1588; continue 'dispatch;
	}
	// 82DE1580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE1584: 48000010  b 0x82de1594
	pc = 0x82DE1594; continue 'dispatch;
	// 82DE1588: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE158C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE1590: 7F6AE02E  lwzx r27, r10, r28
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DE1594: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE1598: 41820078  beq 0x82de1610
	if ctx.cr[0].eq {
	pc = 0x82DE1610; continue 'dispatch;
	}
	// 82DE159C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE15A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE15A4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DE15A8: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DE15AC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE15B0: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE15B4: 4099000C  ble cr6, 0x82de15c0
	if !ctx.cr[6].gt {
	pc = 0x82DE15C0; continue 'dispatch;
	}
	// 82DE15B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE15BC: 48000010  b 0x82de15cc
	pc = 0x82DE15CC; continue 'dispatch;
	// 82DE15C0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE15C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE15C8: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE15CC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE15D0: 41820034  beq 0x82de1604
	if ctx.cr[0].eq {
	pc = 0x82DE1604; continue 'dispatch;
	}
	// 82DE15D4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE15D8: 81570008  lwz r10, 8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE15DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE15E0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE15E4: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DE15E8: 419A0010  beq cr6, 0x82de15f8
	if ctx.cr[6].eq {
	pc = 0x82DE15F8; continue 'dispatch;
	}
	// 82DE15EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE15F0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DE15F4: 48000C4D  bl 0x82de2240
	ctx.lr = 0x82DE15F8;
	sub_82DE2240(ctx, base);
	// 82DE15F8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DE15FC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DE1600: 4BFFFFA8  b 0x82de15a8
	pc = 0x82DE15A8; continue 'dispatch;
	// 82DE1604: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DE1608: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DE160C: 4BFFFF64  b 0x82de1570
	pc = 0x82DE1570; continue 'dispatch;
	// 82DE1610: 8177001C  lwz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1614: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 82DE1618: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82DE161C: 7F155840  cmplw cr6, r21, r11
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DE1620: 4099FED4  ble cr6, 0x82de14f4
	if !ctx.cr[6].gt {
	pc = 0x82DE14F4; continue 'dispatch;
	}
	// 82DE1624: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DE1628: 483C6B74  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1630 size=252
    let mut pc: u32 = 0x82DE1630;
    'dispatch: loop {
        match pc {
            0x82DE1630 => {
    //   block [0x82DE1630..0x82DE172C)
	// 82DE1630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1634: 483C6B35  bl 0x831a8168
	ctx.lr = 0x82DE1638;
	sub_831A8130(ctx, base);
	// 82DE1638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE163C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1640: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DE1644: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DE1648: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE164C: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1650: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1654: 4BFC3F2D  bl 0x82da5580
	ctx.lr = 0x82DE1658;
	sub_82DA5580(ctx, base);
	// 82DE1658: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DE165C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE1660: 41820018  beq 0x82de1678
	if ctx.cr[0].eq {
	pc = 0x82DE1678; continue 'dispatch;
	}
	// 82DE1664: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE1668: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE166C: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1670: 4BFDD7E9  bl 0x82dbee58
	ctx.lr = 0x82DE1674;
	sub_82DBEE58(ctx, base);
	// 82DE1674: 48000008  b 0x82de167c
	pc = 0x82DE167C; continue 'dispatch;
	// 82DE1678: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DE167C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1680: 4800001C  b 0x82de169c
	pc = 0x82DE169C; continue 'dispatch;
	// 82DE1684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE1688: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE168C: 4BFFB18D  bl 0x82ddc818
	ctx.lr = 0x82DE1690;
	sub_82DDC818(ctx, base);
	// 82DE1690: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DE1694: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1698: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE169C: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE16A0: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE16A4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE16A8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE16AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE16B0: 409AFFD4  bne cr6, 0x82de1684
	if !ctx.cr[6].eq {
	pc = 0x82DE1684; continue 'dispatch;
	}
	// 82DE16B4: 48000058  b 0x82de170c
	pc = 0x82DE170C; continue 'dispatch;
	// 82DE16B8: 4BFE4D69  bl 0x82dc6420
	ctx.lr = 0x82DE16BC;
	sub_82DC6420(ctx, base);
	// 82DE16BC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE16C0: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE16C4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE16C8: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE16CC: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE16D0: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE16D4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE16D8: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DE16DC: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE16E0: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE16E4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DE16E8: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE16EC: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DE16F0: 40980010  bge cr6, 0x82de1700
	if !ctx.cr[6].lt {
	pc = 0x82DE1700; continue 'dispatch;
	}
	// 82DE16F4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE16F8: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE16FC: 7D09592E  stwx r8, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82DE1700: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1704: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1708: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82DE170C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1710: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE1714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1718: 409AFFA0  bne cr6, 0x82de16b8
	if !ctx.cr[6].eq {
	pc = 0x82DE16B8; continue 'dispatch;
	}
	// 82DE171C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DE1720: 4BFF0E19  bl 0x82dd2538
	ctx.lr = 0x82DE1724;
	sub_82DD2538(ctx, base);
	// 82DE1724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE1728: 483C6A90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1730 size=80
    let mut pc: u32 = 0x82DE1730;
    'dispatch: loop {
        match pc {
            0x82DE1730 => {
    //   block [0x82DE1730..0x82DE1780)
	// 82DE1730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE173C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1748: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE174C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1750: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE1754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1758: 419A0008  beq cr6, 0x82de1760
	if ctx.cr[6].eq {
	pc = 0x82DE1760; continue 'dispatch;
	}
	// 82DE175C: 4BFFFED5  bl 0x82de1630
	ctx.lr = 0x82DE1760;
	sub_82DE1630(ctx, base);
	// 82DE1760: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE1764: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE1768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE176C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1774: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1780 size=344
    let mut pc: u32 = 0x82DE1780;
    'dispatch: loop {
        match pc {
            0x82DE1780 => {
    //   block [0x82DE1780..0x82DE18D8)
	// 82DE1780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1784: 483C69E1  bl 0x831a8164
	ctx.lr = 0x82DE1788;
	sub_831A8130(ctx, base);
	// 82DE1788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE178C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1790: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE1794: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DE1798: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE179C: 814B0864  lwz r10, 0x864(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2148 as u32) ) } as u64;
	// 82DE17A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE17A4: 914B0864  stw r10, 0x864(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2148 as u32), ctx.r[10].u32 ) };
	// 82DE17A8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE17AC: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE17B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE17B4: 4BFC3DCD  bl 0x82da5580
	ctx.lr = 0x82DE17B8;
	sub_82DA5580(ctx, base);
	// 82DE17B8: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DE17BC: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE17C0: 41820018  beq 0x82de17d8
	if ctx.cr[0].eq {
	pc = 0x82DE17D8; continue 'dispatch;
	}
	// 82DE17C4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE17C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DE17CC: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE17D0: 4BFDD689  bl 0x82dbee58
	ctx.lr = 0x82DE17D4;
	sub_82DBEE58(ctx, base);
	// 82DE17D4: 48000008  b 0x82de17dc
	pc = 0x82DE17DC; continue 'dispatch;
	// 82DE17D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DE17DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE17E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE17E4: 917D0048  stw r11, 0x48(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82DE17E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DE17EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE17F0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DE17F4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE17F8: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE17FC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1800: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82DE1804: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1808: 4BFFB011  bl 0x82ddc818
	ctx.lr = 0x82DE180C;
	sub_82DDC818(ctx, base);
	// 82DE180C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE1810: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1814: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1818: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE181C: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 82DE1820: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82DE1824: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1828: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE182C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DE1830: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82DE1834: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE1838: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE183C: 808B0864  lwz r4, 0x864(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2148 as u32) ) } as u64;
	// 82DE1840: 480008F1  bl 0x82de2130
	ctx.lr = 0x82DE1844;
	sub_82DE2130(ctx, base);
	// 82DE1844: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DE1848: 41820064  beq 0x82de18ac
	if ctx.cr[0].eq {
	pc = 0x82DE18AC; continue 'dispatch;
	}
	// 82DE184C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DE1850: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1854: 4BFFAFC5  bl 0x82ddc818
	ctx.lr = 0x82DE1858;
	sub_82DDC818(ctx, base);
	// 82DE1858: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE185C: 937E0048  stw r27, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[27].u32 ) };
	// 82DE1860: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1864: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1868: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DE186C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82DE1870: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1874: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82DE1878: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE187C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE1880: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1884: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82DE1888: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE188C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE1890: 814A0864  lwz r10, 0x864(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2148 as u32) ) } as u64;
	// 82DE1894: 915E0080  stw r10, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DE1898: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE189C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DE18A0: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82DE18A4: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE18A8: 4BFFFF80  b 0x82de1828
	pc = 0x82DE1828; continue 'dispatch;
	// 82DE18AC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE18B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DE18B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE18B8: 419A0010  beq cr6, 0x82de18c8
	if ctx.cr[6].eq {
	pc = 0x82DE18C8; continue 'dispatch;
	}
	// 82DE18BC: 4BFE4B65  bl 0x82dc6420
	ctx.lr = 0x82DE18C0;
	sub_82DC6420(ctx, base);
	// 82DE18C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE18C4: 4BFFFF70  b 0x82de1834
	pc = 0x82DE1834; continue 'dispatch;
	// 82DE18C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DE18CC: 4BFF0C6D  bl 0x82dd2538
	ctx.lr = 0x82DE18D0;
	sub_82DD2538(ctx, base);
	// 82DE18D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE18D4: 483C68E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE18D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE18D8 size=724
    let mut pc: u32 = 0x82DE18D8;
    'dispatch: loop {
        match pc {
            0x82DE18D8 => {
    //   block [0x82DE18D8..0x82DE1BAC)
	// 82DE18D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE18DC: 483C6875  bl 0x831a8150
	ctx.lr = 0x82DE18E0;
	sub_831A8130(ctx, base);
	// 82DE18E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE18E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE18E8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE18EC: 82CB00A4  lwz r22, 0xa4(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DE18F0: 81560048  lwz r10, 0x48(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE18F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DE18F8: 409A0024  bne cr6, 0x82de191c
	if !ctx.cr[6].eq {
	pc = 0x82DE191C; continue 'dispatch;
	}
	// 82DE18FC: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DE1900: 48000010  b 0x82de1910
	pc = 0x82DE1910; continue 'dispatch;
	// 82DE1904: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1908: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82DE190C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1910: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1914: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE1918: 409AFFEC  bne cr6, 0x82de1904
	if !ctx.cr[6].eq {
	pc = 0x82DE1904; continue 'dispatch;
	}
	// 82DE191C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DE1920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1924: 4BFFFE5D  bl 0x82de1780
	ctx.lr = 0x82DE1928;
	sub_82DE1780(ctx, base);
	// 82DE1928: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE192C: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1930: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE1934: 4BFDDAC5  bl 0x82dbf3f8
	ctx.lr = 0x82DE1938;
	sub_82DBF3F8(ctx, base);
	// 82DE1938: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82DE193C: 831F001C  lwz r24, 0x1c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1940: 2B180001  cmplwi cr6, r24, 1
	ctx.cr[6].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	// 82DE1944: 40990168  ble cr6, 0x82de1aac
	if !ctx.cr[6].gt {
	pc = 0x82DE1AAC; continue 'dispatch;
	}
	// 82DE1948: 571D103A  slwi r29, r24, 2
	ctx.r[29].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DE194C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE1950: 7EFCBB78  mr r28, r23
	ctx.r[28].u64 = ctx.r[23].u64;
	// 82DE1954: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82DE1958: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DE195C: 7F3D582E  lwzx r25, r29, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE1960: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DE1964: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1968: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE196C: 4099000C  ble cr6, 0x82de1978
	if !ctx.cr[6].gt {
	pc = 0x82DE1978; continue 'dispatch;
	}
	// 82DE1970: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82DE1974: 48000010  b 0x82de1984
	pc = 0x82DE1984; continue 'dispatch;
	// 82DE1978: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE197C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE1980: 7F8AD82E  lwzx r28, r10, r27
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DE1984: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE1988: 41820038  beq 0x82de19c0
	if ctx.cr[0].eq {
	pc = 0x82DE19C0; continue 'dispatch;
	}
	// 82DE198C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1990: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1994: 809C0048  lwz r4, 0x48(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE1998: 4BFFFD99  bl 0x82de1730
	ctx.lr = 0x82DE199C;
	sub_82DE1730(ctx, base);
	// 82DE199C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE19A0: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE19A4: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE19A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE19AC: 40980008  bge cr6, 0x82de19b4
	if !ctx.cr[6].lt {
	pc = 0x82DE19B4; continue 'dispatch;
	}
	// 82DE19B0: 7D7DF12E  stwx r11, r29, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 82DE19B4: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DE19B8: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DE19BC: 4BFFFFA4  b 0x82de1960
	pc = 0x82DE1960; continue 'dispatch;
	// 82DE19C0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE19C4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82DE19C8: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE19CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE19D0: 4BFC3BB1  bl 0x82da5580
	ctx.lr = 0x82DE19D4;
	sub_82DA5580(ctx, base);
	// 82DE19D4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE19D8: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE19DC: 41820010  beq 0x82de19ec
	if ctx.cr[0].eq {
	pc = 0x82DE19EC; continue 'dispatch;
	}
	// 82DE19E0: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82DE19E4: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 82DE19E8: 48000008  b 0x82de19f0
	pc = 0x82DE19F0; continue 'dispatch;
	// 82DE19EC: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82DE19F0: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82DE19F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE19F8: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE19FC: 7D5D502E  lwzx r10, r29, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1A00: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1A04: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE1A08: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DE1A0C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1A10: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE1A14: 7D5D502E  lwzx r10, r29, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1A18: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1A1C: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82DE1A20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1A24: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE1A28: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1A2C: 7D7D512E  stwx r11, r29, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82DE1A30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1A34: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE1A38: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE1A3C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1A40: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1A44: 48000050  b 0x82de1a94
	pc = 0x82DE1A94; continue 'dispatch;
	// 82DE1A48: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1A50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DE1A54: 4BFFFCDD  bl 0x82de1730
	ctx.lr = 0x82DE1A58;
	sub_82DE1730(ctx, base);
	// 82DE1A58: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1A5C: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1A60: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DE1A64: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1A68: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1A6C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE1A70: 409A0018  bne cr6, 0x82de1a88
	if !ctx.cr[6].eq {
	pc = 0x82DE1A88; continue 'dispatch;
	}
	// 82DE1A74: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1A78: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1A7C: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE1A80: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82DE1A84: 4800000C  b 0x82de1a90
	pc = 0x82DE1A90; continue 'dispatch;
	// 82DE1A88: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1A8C: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82DE1A90: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1A94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DE1A98: 409AFFB0  bne cr6, 0x82de1a48
	if !ctx.cr[6].eq {
	pc = 0x82DE1A48; continue 'dispatch;
	}
	// 82DE1A9C: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 82DE1AA0: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82DE1AA4: 2B180001  cmplwi cr6, r24, 1
	ctx.cr[6].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	// 82DE1AA8: 4199FEA4  bgt cr6, 0x82de194c
	if ctx.cr[6].gt {
	pc = 0x82DE194C; continue 'dispatch;
	}
	// 82DE1AAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1AB0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82DE1AB4: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 82DE1AB8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1ABC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82DE1AC0: 41980048  blt cr6, 0x82de1b08
	if ctx.cr[6].lt {
	pc = 0x82DE1B08; continue 'dispatch;
	}
	// 82DE1AC4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DE1AC8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1ACC: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1AD0: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE1AD4: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE1AD8: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE1ADC: 419A0018  beq cr6, 0x82de1af4
	if ctx.cr[6].eq {
	pc = 0x82DE1AF4; continue 'dispatch;
	}
	// 82DE1AE0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1AE4: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE1AE8: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DE1AEC: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE1AF0: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82DE1AF4: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1AF8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE1AFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DE1B00: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE1B04: 4099FFC4  ble cr6, 0x82de1ac8
	if !ctx.cr[6].gt {
	pc = 0x82DE1AC8; continue 'dispatch;
	}
	// 82DE1B08: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1B0C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DE1B10: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DE1B14: 41980058  blt cr6, 0x82de1b6c
	if ctx.cr[6].lt {
	pc = 0x82DE1B6C; continue 'dispatch;
	}
	// 82DE1B18: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 82DE1B1C: 409A000C  bne cr6, 0x82de1b28
	if !ctx.cr[6].eq {
	pc = 0x82DE1B28; continue 'dispatch;
	}
	// 82DE1B20: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DE1B24: 4800001C  b 0x82de1b40
	pc = 0x82DE1B40; continue 'dispatch;
	// 82DE1B28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1B2C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1B30: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE1B34: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE1B38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1B3C: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE1B40: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE1B44: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1B48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DE1B4C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE1B50: 90640054  stw r3, 0x54(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82DE1B54: 419A0008  beq cr6, 0x82de1b5c
	if ctx.cr[6].eq {
	pc = 0x82DE1B5C; continue 'dispatch;
	}
	// 82DE1B58: 480006B1  bl 0x82de2208
	ctx.lr = 0x82DE1B5C;
	sub_82DE2208(ctx, base);
	// 82DE1B5C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE1B60: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DE1B64: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DE1B68: 4099FFB0  ble cr6, 0x82de1b18
	if !ctx.cr[6].gt {
	pc = 0x82DE1B18; continue 'dispatch;
	}
	// 82DE1B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1B70: 4BFFF959  bl 0x82de14c8
	ctx.lr = 0x82DE1B74;
	sub_82DE14C8(ctx, base);
	// 82DE1B74: 8176004C  lwz r11, 0x4c(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DE1B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1B7C: 409A0028  bne cr6, 0x82de1ba4
	if !ctx.cr[6].eq {
	pc = 0x82DE1BA4; continue 'dispatch;
	}
	// 82DE1B80: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE1B84: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DE1B88: 48000010  b 0x82de1b98
	pc = 0x82DE1B98; continue 'dispatch;
	// 82DE1B8C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DE1B90: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82DE1B94: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1B98: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1B9C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE1BA0: 409AFFEC  bne cr6, 0x82de1b8c
	if !ctx.cr[6].eq {
	pc = 0x82DE1B8C; continue 'dispatch;
	}
	// 82DE1BA4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DE1BA8: 483C65F8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1BB0 size=140
    let mut pc: u32 = 0x82DE1BB0;
    'dispatch: loop {
        match pc {
            0x82DE1BB0 => {
    //   block [0x82DE1BB0..0x82DE1C3C)
	// 82DE1BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE1BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1BC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1BC8: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82DE1BCC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1BD0: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DE1BD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1BD8: 4BFC39A9  bl 0x82da5580
	ctx.lr = 0x82DE1BDC;
	sub_82DA5580(ctx, base);
	// 82DE1BDC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1BE0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE1BE4: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DE1BE8: 41820014  beq 0x82de1bfc
	if ctx.cr[0].eq {
	pc = 0x82DE1BFC; continue 'dispatch;
	}
	// 82DE1BEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE1BF0: 4BFFF6D1  bl 0x82de12c0
	ctx.lr = 0x82DE1BF4;
	sub_82DE12C0(ctx, base);
	// 82DE1BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1BF8: 48000008  b 0x82de1c00
	pc = 0x82DE1C00; continue 'dispatch;
	// 82DE1BFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE1C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1C04: 4BFFFCD5  bl 0x82de18d8
	ctx.lr = 0x82DE1C08;
	sub_82DE18D8(ctx, base);
	// 82DE1C08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DE1C0C: 419A0018  beq cr6, 0x82de1c24
	if ctx.cr[6].eq {
	pc = 0x82DE1C24; continue 'dispatch;
	}
	// 82DE1C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1C14: 4BFFF79D  bl 0x82de13b0
	ctx.lr = 0x82DE1C18;
	sub_82DE13B0(ctx, base);
	// 82DE1C18: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 82DE1C1C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DE1C20: 4BFC3AE9  bl 0x82da5708
	ctx.lr = 0x82DE1C24;
	sub_82DA5708(ctx, base);
	// 82DE1C24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1C30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1C34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1C40 size=68
    let mut pc: u32 = 0x82DE1C40;
    'dispatch: loop {
        match pc {
            0x82DE1C40 => {
    //   block [0x82DE1C40..0x82DE1C84)
	// 82DE1C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE1C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1C58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE1C5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1C60: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DE1C64: 4BFFA8BD  bl 0x82ddc520
	ctx.lr = 0x82DE1C68;
	sub_82DDC520(ctx, base);
	// 82DE1C68: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 82DE1C6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1C78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1C7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1C88 size=68
    let mut pc: u32 = 0x82DE1C88;
    'dispatch: loop {
        match pc {
            0x82DE1C88 => {
    //   block [0x82DE1C88..0x82DE1CCC)
	// 82DE1C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1C90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE1C94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1C98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE1CA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE1CA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1CA8: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82DE1CAC: 4BFFA895  bl 0x82ddc540
	ctx.lr = 0x82DE1CB0;
	sub_82DDC540(ctx, base);
	// 82DE1CB0: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 82DE1CB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1CC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1CD0 size=96
    let mut pc: u32 = 0x82DE1CD0;
    'dispatch: loop {
        match pc {
            0x82DE1CD0 => {
    //   block [0x82DE1CD0..0x82DE1D30)
	// 82DE1CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1CD4: 483C6499  bl 0x831a816c
	ctx.lr = 0x82DE1CD8;
	sub_831A8130(ctx, base);
	// 82DE1CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1CDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1CE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE1CE4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE1CE8: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82DE1CEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1CF0: 409A0028  bne cr6, 0x82de1d18
	if !ctx.cr[6].eq {
	pc = 0x82DE1D18; continue 'dispatch;
	}
	// 82DE1CF4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE1CF8: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE1CFC: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DE1D00: 38CB7370  addi r6, r11, 0x7370
	ctx.r[6].s64 = ctx.r[11].s64 + 29552;
	// 82DE1D04: 38AA7350  addi r5, r10, 0x7350
	ctx.r[5].s64 = ctx.r[10].s64 + 29520;
	// 82DE1D08: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DE1D0C: 38E000B5  li r7, 0xb5
	ctx.r[7].s64 = 181;
	// 82DE1D10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1D14: 4BF00D6D  bl 0x82ce2a80
	ctx.lr = 0x82DE1D18;
	sub_82CE2A80(ctx, base);
	// 82DE1D18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE1D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1D20: 4BFFA821  bl 0x82ddc540
	ctx.lr = 0x82DE1D24;
	sub_82DDC540(ctx, base);
	// 82DE1D24: 93BE03B4  stw r29, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[29].u32 ) };
	// 82DE1D28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1D2C: 483C6490  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1D30 size=96
    let mut pc: u32 = 0x82DE1D30;
    'dispatch: loop {
        match pc {
            0x82DE1D30 => {
    //   block [0x82DE1D30..0x82DE1D90)
	// 82DE1D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1D34: 483C6439  bl 0x831a816c
	ctx.lr = 0x82DE1D38;
	sub_831A8130(ctx, base);
	// 82DE1D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1D3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1D40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE1D44: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE1D48: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82DE1D4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1D50: 409A0028  bne cr6, 0x82de1d78
	if !ctx.cr[6].eq {
	pc = 0x82DE1D78; continue 'dispatch;
	}
	// 82DE1D54: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 82DE1D58: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DE1D5C: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DE1D60: 38CB7370  addi r6, r11, 0x7370
	ctx.r[6].s64 = ctx.r[11].s64 + 29552;
	// 82DE1D64: 38AA73C0  addi r5, r10, 0x73c0
	ctx.r[5].s64 = ctx.r[10].s64 + 29632;
	// 82DE1D68: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DE1D6C: 38E000C1  li r7, 0xc1
	ctx.r[7].s64 = 193;
	// 82DE1D70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1D74: 4BF00D0D  bl 0x82ce2a80
	ctx.lr = 0x82DE1D78;
	sub_82CE2A80(ctx, base);
	// 82DE1D78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE1D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1D80: 4BFFA7A1  bl 0x82ddc520
	ctx.lr = 0x82DE1D84;
	sub_82DDC520(ctx, base);
	// 82DE1D84: 93BE03B4  stw r29, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[29].u32 ) };
	// 82DE1D88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1D8C: 483C6430  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


