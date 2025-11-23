pub fn sub_8326E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E9B8 size=56
    let mut pc: u32 = 0x8326E9B8;
    'dispatch: loop {
        match pc {
            0x8326E9B8 => {
    //   block [0x8326E9B8..0x8326E9F0)
	// 8326E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E9C4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E9CC: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 8326E9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E9D4: 4AF85385  bl 0x821f3d58
	ctx.lr = 0x8326E9D8;
	sub_821F3D58(ctx, base);
	// 8326E9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E9DC: 906ACC30  stw r3, -0x33d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13264 as u32), ctx.r[3].u32 ) };
	// 8326E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E9F0 size=56
    let mut pc: u32 = 0x8326E9F0;
    'dispatch: loop {
        match pc {
            0x8326E9F0 => {
    //   block [0x8326E9F0..0x8326EA28)
	// 8326E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E9FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EA04: 386B9620  addi r3, r11, -0x69e0
	ctx.r[3].s64 = ctx.r[11].s64 + -27104;
	// 8326EA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EA0C: 4AF8534D  bl 0x821f3d58
	ctx.lr = 0x8326EA10;
	sub_821F3D58(ctx, base);
	// 8326EA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EA14: 906ACC34  stw r3, -0x33cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13260 as u32), ctx.r[3].u32 ) };
	// 8326EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA28 size=56
    let mut pc: u32 = 0x8326EA28;
    'dispatch: loop {
        match pc {
            0x8326EA28 => {
    //   block [0x8326EA28..0x8326EA60)
	// 8326EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EA34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EA3C: 386B962C  addi r3, r11, -0x69d4
	ctx.r[3].s64 = ctx.r[11].s64 + -27092;
	// 8326EA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EA44: 4AF85315  bl 0x821f3d58
	ctx.lr = 0x8326EA48;
	sub_821F3D58(ctx, base);
	// 8326EA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EA4C: 906ACC38  stw r3, -0x33c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13256 as u32), ctx.r[3].u32 ) };
	// 8326EA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA60 size=56
    let mut pc: u32 = 0x8326EA60;
    'dispatch: loop {
        match pc {
            0x8326EA60 => {
    //   block [0x8326EA60..0x8326EA98)
	// 8326EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EA6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EA74: 386B2024  addi r3, r11, 0x2024
	ctx.r[3].s64 = ctx.r[11].s64 + 8228;
	// 8326EA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EA7C: 4AF852DD  bl 0x821f3d58
	ctx.lr = 0x8326EA80;
	sub_821F3D58(ctx, base);
	// 8326EA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EA84: 906ACC3C  stw r3, -0x33c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13252 as u32), ctx.r[3].u32 ) };
	// 8326EA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA98 size=56
    let mut pc: u32 = 0x8326EA98;
    'dispatch: loop {
        match pc {
            0x8326EA98 => {
    //   block [0x8326EA98..0x8326EAD0)
	// 8326EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EAA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EAAC: 386B96A0  addi r3, r11, -0x6960
	ctx.r[3].s64 = ctx.r[11].s64 + -26976;
	// 8326EAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EAB4: 4AF852A5  bl 0x821f3d58
	ctx.lr = 0x8326EAB8;
	sub_821F3D58(ctx, base);
	// 8326EAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EABC: 906ACC40  stw r3, -0x33c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13248 as u32), ctx.r[3].u32 ) };
	// 8326EAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EAD0 size=56
    let mut pc: u32 = 0x8326EAD0;
    'dispatch: loop {
        match pc {
            0x8326EAD0 => {
    //   block [0x8326EAD0..0x8326EB08)
	// 8326EAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EADC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326EAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EAE4: 386B948C  addi r3, r11, -0x6b74
	ctx.r[3].s64 = ctx.r[11].s64 + -27508;
	// 8326EAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EAEC: 4AF8526D  bl 0x821f3d58
	ctx.lr = 0x8326EAF0;
	sub_821F3D58(ctx, base);
	// 8326EAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EAF4: 906ACC44  stw r3, -0x33bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13244 as u32), ctx.r[3].u32 ) };
	// 8326EAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB08 size=56
    let mut pc: u32 = 0x8326EB08;
    'dispatch: loop {
        match pc {
            0x8326EB08 => {
    //   block [0x8326EB08..0x8326EB40)
	// 8326EB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EB14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326EB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EB1C: 386B0F7C  addi r3, r11, 0xf7c
	ctx.r[3].s64 = ctx.r[11].s64 + 3964;
	// 8326EB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EB24: 4AF85235  bl 0x821f3d58
	ctx.lr = 0x8326EB28;
	sub_821F3D58(ctx, base);
	// 8326EB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EB2C: 906ACC48  stw r3, -0x33b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13240 as u32), ctx.r[3].u32 ) };
	// 8326EB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB40 size=56
    let mut pc: u32 = 0x8326EB40;
    'dispatch: loop {
        match pc {
            0x8326EB40 => {
    //   block [0x8326EB40..0x8326EB78)
	// 8326EB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EB4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EB54: 386B9480  addi r3, r11, -0x6b80
	ctx.r[3].s64 = ctx.r[11].s64 + -27520;
	// 8326EB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EB5C: 4AF851FD  bl 0x821f3d58
	ctx.lr = 0x8326EB60;
	sub_821F3D58(ctx, base);
	// 8326EB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EB64: 906ACC4C  stw r3, -0x33b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13236 as u32), ctx.r[3].u32 ) };
	// 8326EB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB78 size=56
    let mut pc: u32 = 0x8326EB78;
    'dispatch: loop {
        match pc {
            0x8326EB78 => {
    //   block [0x8326EB78..0x8326EBB0)
	// 8326EB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EB84: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EB88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EB8C: 386B96AC  addi r3, r11, -0x6954
	ctx.r[3].s64 = ctx.r[11].s64 + -26964;
	// 8326EB90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EB94: 4AF851C5  bl 0x821f3d58
	ctx.lr = 0x8326EB98;
	sub_821F3D58(ctx, base);
	// 8326EB98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EB9C: 906ACC50  stw r3, -0x33b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13232 as u32), ctx.r[3].u32 ) };
	// 8326EBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EBB0 size=56
    let mut pc: u32 = 0x8326EBB0;
    'dispatch: loop {
        match pc {
            0x8326EBB0 => {
    //   block [0x8326EBB0..0x8326EBE8)
	// 8326EBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EBBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EBC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EBC4: 386B96B8  addi r3, r11, -0x6948
	ctx.r[3].s64 = ctx.r[11].s64 + -26952;
	// 8326EBC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EBCC: 4AF8518D  bl 0x821f3d58
	ctx.lr = 0x8326EBD0;
	sub_821F3D58(ctx, base);
	// 8326EBD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EBD4: 906ACC54  stw r3, -0x33ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13228 as u32), ctx.r[3].u32 ) };
	// 8326EBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EBE8 size=56
    let mut pc: u32 = 0x8326EBE8;
    'dispatch: loop {
        match pc {
            0x8326EBE8 => {
    //   block [0x8326EBE8..0x8326EC20)
	// 8326EBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EBF4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326EBF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EBFC: 386BA994  addi r3, r11, -0x566c
	ctx.r[3].s64 = ctx.r[11].s64 + -22124;
	// 8326EC00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EC04: 4AF85155  bl 0x821f3d58
	ctx.lr = 0x8326EC08;
	sub_821F3D58(ctx, base);
	// 8326EC08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EC0C: 906ACC58  stw r3, -0x33a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13224 as u32), ctx.r[3].u32 ) };
	// 8326EC10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC20 size=56
    let mut pc: u32 = 0x8326EC20;
    'dispatch: loop {
        match pc {
            0x8326EC20 => {
    //   block [0x8326EC20..0x8326EC58)
	// 8326EC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EC28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EC2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EC30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EC34: 386B2C84  addi r3, r11, 0x2c84
	ctx.r[3].s64 = ctx.r[11].s64 + 11396;
	// 8326EC38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EC3C: 4AF8511D  bl 0x821f3d58
	ctx.lr = 0x8326EC40;
	sub_821F3D58(ctx, base);
	// 8326EC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EC44: 906ACC5C  stw r3, -0x33a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13220 as u32), ctx.r[3].u32 ) };
	// 8326EC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC58 size=56
    let mut pc: u32 = 0x8326EC58;
    'dispatch: loop {
        match pc {
            0x8326EC58 => {
    //   block [0x8326EC58..0x8326EC90)
	// 8326EC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EC64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EC68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EC6C: 386B96C4  addi r3, r11, -0x693c
	ctx.r[3].s64 = ctx.r[11].s64 + -26940;
	// 8326EC70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EC74: 4AF850E5  bl 0x821f3d58
	ctx.lr = 0x8326EC78;
	sub_821F3D58(ctx, base);
	// 8326EC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EC7C: 906ACC60  stw r3, -0x33a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13216 as u32), ctx.r[3].u32 ) };
	// 8326EC80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC90 size=56
    let mut pc: u32 = 0x8326EC90;
    'dispatch: loop {
        match pc {
            0x8326EC90 => {
    //   block [0x8326EC90..0x8326ECC8)
	// 8326EC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EC98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EC9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326ECA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326ECA4: 386B96D4  addi r3, r11, -0x692c
	ctx.r[3].s64 = ctx.r[11].s64 + -26924;
	// 8326ECA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326ECAC: 4AF850AD  bl 0x821f3d58
	ctx.lr = 0x8326ECB0;
	sub_821F3D58(ctx, base);
	// 8326ECB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ECB4: 906ACC64  stw r3, -0x339c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13212 as u32), ctx.r[3].u32 ) };
	// 8326ECB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ECBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ECC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ECC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ECC8 size=64
    let mut pc: u32 = 0x8326ECC8;
    'dispatch: loop {
        match pc {
            0x8326ECC8 => {
    //   block [0x8326ECC8..0x8326ED08)
	// 8326ECC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ECCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ECD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ECD4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326ECD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ECDC: 388B9B48  addi r4, r11, -0x64b8
	ctx.r[4].s64 = ctx.r[11].s64 + -25784;
	// 8326ECE0: 386ACC68  addi r3, r10, -0x3398
	ctx.r[3].s64 = ctx.r[10].s64 + -13208;
	// 8326ECE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ECE8: 4AFBE1E9  bl 0x8222ced0
	ctx.lr = 0x8326ECEC;
	sub_8222CED0(ctx, base);
	// 8326ECEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ECF0: 3869EF00  addi r3, r9, -0x1100
	ctx.r[3].s64 = ctx.r[9].s64 + -4352;
	// 8326ECF4: 4BA3B22D  bl 0x82ca9f20
	ctx.lr = 0x8326ECF8;
	sub_82CA9F20(ctx, base);
	// 8326ECF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ECFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ED00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ED04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED08 size=64
    let mut pc: u32 = 0x8326ED08;
    'dispatch: loop {
        match pc {
            0x8326ED08 => {
    //   block [0x8326ED08..0x8326ED48)
	// 8326ED08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ED14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326ED18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ED1C: 388B33AC  addi r4, r11, 0x33ac
	ctx.r[4].s64 = ctx.r[11].s64 + 13228;
	// 8326ED20: 386ACC6C  addi r3, r10, -0x3394
	ctx.r[3].s64 = ctx.r[10].s64 + -13204;
	// 8326ED24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ED28: 4AFBE1A9  bl 0x8222ced0
	ctx.lr = 0x8326ED2C;
	sub_8222CED0(ctx, base);
	// 8326ED2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ED30: 3869EF10  addi r3, r9, -0x10f0
	ctx.r[3].s64 = ctx.r[9].s64 + -4336;
	// 8326ED34: 4BA3B1ED  bl 0x82ca9f20
	ctx.lr = 0x8326ED38;
	sub_82CA9F20(ctx, base);
	// 8326ED38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED48 size=64
    let mut pc: u32 = 0x8326ED48;
    'dispatch: loop {
        match pc {
            0x8326ED48 => {
    //   block [0x8326ED48..0x8326ED88)
	// 8326ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ED54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326ED58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ED5C: 388BDCFC  addi r4, r11, -0x2304
	ctx.r[4].s64 = ctx.r[11].s64 + -8964;
	// 8326ED60: 386ACC70  addi r3, r10, -0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + -13200;
	// 8326ED64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ED68: 4AFBE169  bl 0x8222ced0
	ctx.lr = 0x8326ED6C;
	sub_8222CED0(ctx, base);
	// 8326ED6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ED70: 3869EF20  addi r3, r9, -0x10e0
	ctx.r[3].s64 = ctx.r[9].s64 + -4320;
	// 8326ED74: 4BA3B1AD  bl 0x82ca9f20
	ctx.lr = 0x8326ED78;
	sub_82CA9F20(ctx, base);
	// 8326ED78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ED7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ED80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ED84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED88 size=64
    let mut pc: u32 = 0x8326ED88;
    'dispatch: loop {
        match pc {
            0x8326ED88 => {
    //   block [0x8326ED88..0x8326EDC8)
	// 8326ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ED94: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326ED98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ED9C: 388B7700  addi r4, r11, 0x7700
	ctx.r[4].s64 = ctx.r[11].s64 + 30464;
	// 8326EDA0: 386ACC74  addi r3, r10, -0x338c
	ctx.r[3].s64 = ctx.r[10].s64 + -13196;
	// 8326EDA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EDA8: 4AFBE129  bl 0x8222ced0
	ctx.lr = 0x8326EDAC;
	sub_8222CED0(ctx, base);
	// 8326EDAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EDB0: 3869EF30  addi r3, r9, -0x10d0
	ctx.r[3].s64 = ctx.r[9].s64 + -4304;
	// 8326EDB4: 4BA3B16D  bl 0x82ca9f20
	ctx.lr = 0x8326EDB8;
	sub_82CA9F20(ctx, base);
	// 8326EDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EDC8 size=64
    let mut pc: u32 = 0x8326EDC8;
    'dispatch: loop {
        match pc {
            0x8326EDC8 => {
    //   block [0x8326EDC8..0x8326EE08)
	// 8326EDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EDD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326EDD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EDDC: 388B2BAC  addi r4, r11, 0x2bac
	ctx.r[4].s64 = ctx.r[11].s64 + 11180;
	// 8326EDE0: 386ACC78  addi r3, r10, -0x3388
	ctx.r[3].s64 = ctx.r[10].s64 + -13192;
	// 8326EDE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EDE8: 4AFBE0E9  bl 0x8222ced0
	ctx.lr = 0x8326EDEC;
	sub_8222CED0(ctx, base);
	// 8326EDEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EDF0: 3869EF40  addi r3, r9, -0x10c0
	ctx.r[3].s64 = ctx.r[9].s64 + -4288;
	// 8326EDF4: 4BA3B12D  bl 0x82ca9f20
	ctx.lr = 0x8326EDF8;
	sub_82CA9F20(ctx, base);
	// 8326EDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE08 size=64
    let mut pc: u32 = 0x8326EE08;
    'dispatch: loop {
        match pc {
            0x8326EE08 => {
    //   block [0x8326EE08..0x8326EE48)
	// 8326EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EE14: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EE18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EE1C: 388BDD00  addi r4, r11, -0x2300
	ctx.r[4].s64 = ctx.r[11].s64 + -8960;
	// 8326EE20: 386ACC7C  addi r3, r10, -0x3384
	ctx.r[3].s64 = ctx.r[10].s64 + -13188;
	// 8326EE24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EE28: 4AFBE0A9  bl 0x8222ced0
	ctx.lr = 0x8326EE2C;
	sub_8222CED0(ctx, base);
	// 8326EE2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EE30: 3869EF50  addi r3, r9, -0x10b0
	ctx.r[3].s64 = ctx.r[9].s64 + -4272;
	// 8326EE34: 4BA3B0ED  bl 0x82ca9f20
	ctx.lr = 0x8326EE38;
	sub_82CA9F20(ctx, base);
	// 8326EE38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE48 size=64
    let mut pc: u32 = 0x8326EE48;
    'dispatch: loop {
        match pc {
            0x8326EE48 => {
    //   block [0x8326EE48..0x8326EE88)
	// 8326EE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EE54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EE5C: 388BDD0C  addi r4, r11, -0x22f4
	ctx.r[4].s64 = ctx.r[11].s64 + -8948;
	// 8326EE60: 386ACC80  addi r3, r10, -0x3380
	ctx.r[3].s64 = ctx.r[10].s64 + -13184;
	// 8326EE64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EE68: 4AFBE069  bl 0x8222ced0
	ctx.lr = 0x8326EE6C;
	sub_8222CED0(ctx, base);
	// 8326EE6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EE70: 3869EF60  addi r3, r9, -0x10a0
	ctx.r[3].s64 = ctx.r[9].s64 + -4256;
	// 8326EE74: 4BA3B0AD  bl 0x82ca9f20
	ctx.lr = 0x8326EE78;
	sub_82CA9F20(ctx, base);
	// 8326EE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE88 size=64
    let mut pc: u32 = 0x8326EE88;
    'dispatch: loop {
        match pc {
            0x8326EE88 => {
    //   block [0x8326EE88..0x8326EEC8)
	// 8326EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EE94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EE9C: 388BDD18  addi r4, r11, -0x22e8
	ctx.r[4].s64 = ctx.r[11].s64 + -8936;
	// 8326EEA0: 386ACC84  addi r3, r10, -0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + -13180;
	// 8326EEA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EEA8: 4AFBE029  bl 0x8222ced0
	ctx.lr = 0x8326EEAC;
	sub_8222CED0(ctx, base);
	// 8326EEAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EEB0: 3869EF70  addi r3, r9, -0x1090
	ctx.r[3].s64 = ctx.r[9].s64 + -4240;
	// 8326EEB4: 4BA3B06D  bl 0x82ca9f20
	ctx.lr = 0x8326EEB8;
	sub_82CA9F20(ctx, base);
	// 8326EEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EEC8 size=64
    let mut pc: u32 = 0x8326EEC8;
    'dispatch: loop {
        match pc {
            0x8326EEC8 => {
    //   block [0x8326EEC8..0x8326EF08)
	// 8326EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EED4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EEDC: 388B9B50  addi r4, r11, -0x64b0
	ctx.r[4].s64 = ctx.r[11].s64 + -25776;
	// 8326EEE0: 386ACC88  addi r3, r10, -0x3378
	ctx.r[3].s64 = ctx.r[10].s64 + -13176;
	// 8326EEE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EEE8: 4AFBDFE9  bl 0x8222ced0
	ctx.lr = 0x8326EEEC;
	sub_8222CED0(ctx, base);
	// 8326EEEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EEF0: 3869EF80  addi r3, r9, -0x1080
	ctx.r[3].s64 = ctx.r[9].s64 + -4224;
	// 8326EEF4: 4BA3B02D  bl 0x82ca9f20
	ctx.lr = 0x8326EEF8;
	sub_82CA9F20(ctx, base);
	// 8326EEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF08 size=64
    let mut pc: u32 = 0x8326EF08;
    'dispatch: loop {
        match pc {
            0x8326EF08 => {
    //   block [0x8326EF08..0x8326EF48)
	// 8326EF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EF14: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EF18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EF1C: 388B9B74  addi r4, r11, -0x648c
	ctx.r[4].s64 = ctx.r[11].s64 + -25740;
	// 8326EF20: 386ACC8C  addi r3, r10, -0x3374
	ctx.r[3].s64 = ctx.r[10].s64 + -13172;
	// 8326EF24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EF28: 4AFBDFA9  bl 0x8222ced0
	ctx.lr = 0x8326EF2C;
	sub_8222CED0(ctx, base);
	// 8326EF2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EF30: 3869EF90  addi r3, r9, -0x1070
	ctx.r[3].s64 = ctx.r[9].s64 + -4208;
	// 8326EF34: 4BA3AFED  bl 0x82ca9f20
	ctx.lr = 0x8326EF38;
	sub_82CA9F20(ctx, base);
	// 8326EF38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF48 size=64
    let mut pc: u32 = 0x8326EF48;
    'dispatch: loop {
        match pc {
            0x8326EF48 => {
    //   block [0x8326EF48..0x8326EF88)
	// 8326EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EF54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326EF58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EF5C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326EF60: 386ACC90  addi r3, r10, -0x3370
	ctx.r[3].s64 = ctx.r[10].s64 + -13168;
	// 8326EF64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EF68: 4AFBDF69  bl 0x8222ced0
	ctx.lr = 0x8326EF6C;
	sub_8222CED0(ctx, base);
	// 8326EF6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EF70: 3869EFA0  addi r3, r9, -0x1060
	ctx.r[3].s64 = ctx.r[9].s64 + -4192;
	// 8326EF74: 4BA3AFAD  bl 0x82ca9f20
	ctx.lr = 0x8326EF78;
	sub_82CA9F20(ctx, base);
	// 8326EF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF88 size=64
    let mut pc: u32 = 0x8326EF88;
    'dispatch: loop {
        match pc {
            0x8326EF88 => {
    //   block [0x8326EF88..0x8326EFC8)
	// 8326EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EF94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326EF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EF9C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326EFA0: 386ACC94  addi r3, r10, -0x336c
	ctx.r[3].s64 = ctx.r[10].s64 + -13164;
	// 8326EFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EFA8: 4AFBDF29  bl 0x8222ced0
	ctx.lr = 0x8326EFAC;
	sub_8222CED0(ctx, base);
	// 8326EFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EFB0: 3869EFB0  addi r3, r9, -0x1050
	ctx.r[3].s64 = ctx.r[9].s64 + -4176;
	// 8326EFB4: 4BA3AF6D  bl 0x82ca9f20
	ctx.lr = 0x8326EFB8;
	sub_82CA9F20(ctx, base);
	// 8326EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EFC8 size=64
    let mut pc: u32 = 0x8326EFC8;
    'dispatch: loop {
        match pc {
            0x8326EFC8 => {
    //   block [0x8326EFC8..0x8326F008)
	// 8326EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EFD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326EFD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EFDC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326EFE0: 386ACC98  addi r3, r10, -0x3368
	ctx.r[3].s64 = ctx.r[10].s64 + -13160;
	// 8326EFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EFE8: 4AFBDEE9  bl 0x8222ced0
	ctx.lr = 0x8326EFEC;
	sub_8222CED0(ctx, base);
	// 8326EFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EFF0: 3869EFC0  addi r3, r9, -0x1040
	ctx.r[3].s64 = ctx.r[9].s64 + -4160;
	// 8326EFF4: 4BA3AF2D  bl 0x82ca9f20
	ctx.lr = 0x8326EFF8;
	sub_82CA9F20(ctx, base);
	// 8326EFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F008 size=56
    let mut pc: u32 = 0x8326F008;
    'dispatch: loop {
        match pc {
            0x8326F008 => {
    //   block [0x8326F008..0x8326F040)
	// 8326F008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F014: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F018: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F01C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326F020: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F024: 4AF84D35  bl 0x821f3d58
	ctx.lr = 0x8326F028;
	sub_821F3D58(ctx, base);
	// 8326F028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F02C: 906ACC9C  stw r3, -0x3364(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13156 as u32), ctx.r[3].u32 ) };
	// 8326F030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F040 size=56
    let mut pc: u32 = 0x8326F040;
    'dispatch: loop {
        match pc {
            0x8326F040 => {
    //   block [0x8326F040..0x8326F078)
	// 8326F040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F04C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F050: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F054: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326F058: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F05C: 4AF84CFD  bl 0x821f3d58
	ctx.lr = 0x8326F060;
	sub_821F3D58(ctx, base);
	// 8326F060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F064: 906ACCA0  stw r3, -0x3360(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13152 as u32), ctx.r[3].u32 ) };
	// 8326F068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F078 size=56
    let mut pc: u32 = 0x8326F078;
    'dispatch: loop {
        match pc {
            0x8326F078 => {
    //   block [0x8326F078..0x8326F0B0)
	// 8326F078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F084: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F088: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F08C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326F090: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F094: 4AF84CC5  bl 0x821f3d58
	ctx.lr = 0x8326F098;
	sub_821F3D58(ctx, base);
	// 8326F098: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F09C: 906ACCA4  stw r3, -0x335c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13148 as u32), ctx.r[3].u32 ) };
	// 8326F0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F0B0 size=56
    let mut pc: u32 = 0x8326F0B0;
    'dispatch: loop {
        match pc {
            0x8326F0B0 => {
    //   block [0x8326F0B0..0x8326F0E8)
	// 8326F0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F0BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F0C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F0C4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326F0C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F0CC: 4AF84C8D  bl 0x821f3d58
	ctx.lr = 0x8326F0D0;
	sub_821F3D58(ctx, base);
	// 8326F0D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F0D4: 906ACCA8  stw r3, -0x3358(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13144 as u32), ctx.r[3].u32 ) };
	// 8326F0D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F0E8 size=56
    let mut pc: u32 = 0x8326F0E8;
    'dispatch: loop {
        match pc {
            0x8326F0E8 => {
    //   block [0x8326F0E8..0x8326F120)
	// 8326F0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F0F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F0F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F0F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F0FC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326F100: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F104: 4AF84C55  bl 0x821f3d58
	ctx.lr = 0x8326F108;
	sub_821F3D58(ctx, base);
	// 8326F108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F10C: 906ACCAC  stw r3, -0x3354(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13140 as u32), ctx.r[3].u32 ) };
	// 8326F110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F120 size=56
    let mut pc: u32 = 0x8326F120;
    'dispatch: loop {
        match pc {
            0x8326F120 => {
    //   block [0x8326F120..0x8326F158)
	// 8326F120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F12C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F130: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F134: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326F138: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F13C: 4AF84C1D  bl 0x821f3d58
	ctx.lr = 0x8326F140;
	sub_821F3D58(ctx, base);
	// 8326F140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F144: 906ACCB0  stw r3, -0x3350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13136 as u32), ctx.r[3].u32 ) };
	// 8326F148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F158 size=56
    let mut pc: u32 = 0x8326F158;
    'dispatch: loop {
        match pc {
            0x8326F158 => {
    //   block [0x8326F158..0x8326F190)
	// 8326F158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F16C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326F170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F174: 4AF84BE5  bl 0x821f3d58
	ctx.lr = 0x8326F178;
	sub_821F3D58(ctx, base);
	// 8326F178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F17C: 906ACCB4  stw r3, -0x334c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13132 as u32), ctx.r[3].u32 ) };
	// 8326F180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F190 size=56
    let mut pc: u32 = 0x8326F190;
    'dispatch: loop {
        match pc {
            0x8326F190 => {
    //   block [0x8326F190..0x8326F1C8)
	// 8326F190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F1A4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326F1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F1AC: 4AF84BAD  bl 0x821f3d58
	ctx.lr = 0x8326F1B0;
	sub_821F3D58(ctx, base);
	// 8326F1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F1B4: 906ACCB8  stw r3, -0x3348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13128 as u32), ctx.r[3].u32 ) };
	// 8326F1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F1C8 size=56
    let mut pc: u32 = 0x8326F1C8;
    'dispatch: loop {
        match pc {
            0x8326F1C8 => {
    //   block [0x8326F1C8..0x8326F200)
	// 8326F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F1DC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326F1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F1E4: 4AF84B75  bl 0x821f3d58
	ctx.lr = 0x8326F1E8;
	sub_821F3D58(ctx, base);
	// 8326F1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F1EC: 906ACCBC  stw r3, -0x3344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13124 as u32), ctx.r[3].u32 ) };
	// 8326F1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F200 size=56
    let mut pc: u32 = 0x8326F200;
    'dispatch: loop {
        match pc {
            0x8326F200 => {
    //   block [0x8326F200..0x8326F238)
	// 8326F200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F214: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326F218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F21C: 4AF84B3D  bl 0x821f3d58
	ctx.lr = 0x8326F220;
	sub_821F3D58(ctx, base);
	// 8326F220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F224: 906ACCC0  stw r3, -0x3340(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13120 as u32), ctx.r[3].u32 ) };
	// 8326F228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F238 size=56
    let mut pc: u32 = 0x8326F238;
    'dispatch: loop {
        match pc {
            0x8326F238 => {
    //   block [0x8326F238..0x8326F270)
	// 8326F238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F24C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326F250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F254: 4AF84B05  bl 0x821f3d58
	ctx.lr = 0x8326F258;
	sub_821F3D58(ctx, base);
	// 8326F258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F25C: 906ACCC4  stw r3, -0x333c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13116 as u32), ctx.r[3].u32 ) };
	// 8326F260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F270 size=56
    let mut pc: u32 = 0x8326F270;
    'dispatch: loop {
        match pc {
            0x8326F270 => {
    //   block [0x8326F270..0x8326F2A8)
	// 8326F270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F284: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326F288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F28C: 4AF84ACD  bl 0x821f3d58
	ctx.lr = 0x8326F290;
	sub_821F3D58(ctx, base);
	// 8326F290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F294: 906ACCC8  stw r3, -0x3338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13112 as u32), ctx.r[3].u32 ) };
	// 8326F298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F2A8 size=56
    let mut pc: u32 = 0x8326F2A8;
    'dispatch: loop {
        match pc {
            0x8326F2A8 => {
    //   block [0x8326F2A8..0x8326F2E0)
	// 8326F2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F2BC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326F2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F2C4: 4AF84A95  bl 0x821f3d58
	ctx.lr = 0x8326F2C8;
	sub_821F3D58(ctx, base);
	// 8326F2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F2CC: 906ACCCC  stw r3, -0x3334(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13108 as u32), ctx.r[3].u32 ) };
	// 8326F2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F2E0 size=56
    let mut pc: u32 = 0x8326F2E0;
    'dispatch: loop {
        match pc {
            0x8326F2E0 => {
    //   block [0x8326F2E0..0x8326F318)
	// 8326F2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F2F4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326F2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F2FC: 4AF84A5D  bl 0x821f3d58
	ctx.lr = 0x8326F300;
	sub_821F3D58(ctx, base);
	// 8326F300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F304: 906ACCD0  stw r3, -0x3330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13104 as u32), ctx.r[3].u32 ) };
	// 8326F308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F318 size=56
    let mut pc: u32 = 0x8326F318;
    'dispatch: loop {
        match pc {
            0x8326F318 => {
    //   block [0x8326F318..0x8326F350)
	// 8326F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F32C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326F330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F334: 4AF84A25  bl 0x821f3d58
	ctx.lr = 0x8326F338;
	sub_821F3D58(ctx, base);
	// 8326F338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F33C: 906ACCD4  stw r3, -0x332c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13100 as u32), ctx.r[3].u32 ) };
	// 8326F340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F350 size=56
    let mut pc: u32 = 0x8326F350;
    'dispatch: loop {
        match pc {
            0x8326F350 => {
    //   block [0x8326F350..0x8326F388)
	// 8326F350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F364: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326F368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F36C: 4AF849ED  bl 0x821f3d58
	ctx.lr = 0x8326F370;
	sub_821F3D58(ctx, base);
	// 8326F370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F374: 906ACCD8  stw r3, -0x3328(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13096 as u32), ctx.r[3].u32 ) };
	// 8326F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F388 size=56
    let mut pc: u32 = 0x8326F388;
    'dispatch: loop {
        match pc {
            0x8326F388 => {
    //   block [0x8326F388..0x8326F3C0)
	// 8326F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F39C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326F3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F3A4: 4AF849B5  bl 0x821f3d58
	ctx.lr = 0x8326F3A8;
	sub_821F3D58(ctx, base);
	// 8326F3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F3AC: 906ACCDC  stw r3, -0x3324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13092 as u32), ctx.r[3].u32 ) };
	// 8326F3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F3C0 size=56
    let mut pc: u32 = 0x8326F3C0;
    'dispatch: loop {
        match pc {
            0x8326F3C0 => {
    //   block [0x8326F3C0..0x8326F3F8)
	// 8326F3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F3D4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326F3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F3DC: 4AF8497D  bl 0x821f3d58
	ctx.lr = 0x8326F3E0;
	sub_821F3D58(ctx, base);
	// 8326F3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F3E4: 906ACCE0  stw r3, -0x3320(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13088 as u32), ctx.r[3].u32 ) };
	// 8326F3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F3F8 size=56
    let mut pc: u32 = 0x8326F3F8;
    'dispatch: loop {
        match pc {
            0x8326F3F8 => {
    //   block [0x8326F3F8..0x8326F430)
	// 8326F3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F40C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326F410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F414: 4AF84945  bl 0x821f3d58
	ctx.lr = 0x8326F418;
	sub_821F3D58(ctx, base);
	// 8326F418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F41C: 906ACCE4  stw r3, -0x331c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13084 as u32), ctx.r[3].u32 ) };
	// 8326F420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F430 size=56
    let mut pc: u32 = 0x8326F430;
    'dispatch: loop {
        match pc {
            0x8326F430 => {
    //   block [0x8326F430..0x8326F468)
	// 8326F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F444: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326F448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F44C: 4AF8490D  bl 0x821f3d58
	ctx.lr = 0x8326F450;
	sub_821F3D58(ctx, base);
	// 8326F450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F454: 906ACCE8  stw r3, -0x3318(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13080 as u32), ctx.r[3].u32 ) };
	// 8326F458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F468 size=56
    let mut pc: u32 = 0x8326F468;
    'dispatch: loop {
        match pc {
            0x8326F468 => {
    //   block [0x8326F468..0x8326F4A0)
	// 8326F468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F47C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326F480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F484: 4AF848D5  bl 0x821f3d58
	ctx.lr = 0x8326F488;
	sub_821F3D58(ctx, base);
	// 8326F488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F48C: 906ACCEC  stw r3, -0x3314(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13076 as u32), ctx.r[3].u32 ) };
	// 8326F490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F4A0 size=64
    let mut pc: u32 = 0x8326F4A0;
    'dispatch: loop {
        match pc {
            0x8326F4A0 => {
    //   block [0x8326F4A0..0x8326F4E0)
	// 8326F4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F4B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F4B4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8326F4B8: 386ACCF0  addi r3, r10, -0x3310
	ctx.r[3].s64 = ctx.r[10].s64 + -13072;
	// 8326F4BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326F4C0: 4AFBDA11  bl 0x8222ced0
	ctx.lr = 0x8326F4C4;
	sub_8222CED0(ctx, base);
	// 8326F4C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326F4C8: 3869EFD0  addi r3, r9, -0x1030
	ctx.r[3].s64 = ctx.r[9].s64 + -4144;
	// 8326F4CC: 4BA3AA55  bl 0x82ca9f20
	ctx.lr = 0x8326F4D0;
	sub_82CA9F20(ctx, base);
	// 8326F4D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F4E0 size=64
    let mut pc: u32 = 0x8326F4E0;
    'dispatch: loop {
        match pc {
            0x8326F4E0 => {
    //   block [0x8326F4E0..0x8326F520)
	// 8326F4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F4E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F4EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326F4F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F4F4: 388B9D80  addi r4, r11, -0x6280
	ctx.r[4].s64 = ctx.r[11].s64 + -25216;
	// 8326F4F8: 386ACCF4  addi r3, r10, -0x330c
	ctx.r[3].s64 = ctx.r[10].s64 + -13068;
	// 8326F4FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326F500: 4AFBD9D1  bl 0x8222ced0
	ctx.lr = 0x8326F504;
	sub_8222CED0(ctx, base);
	// 8326F504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326F508: 3869EFE0  addi r3, r9, -0x1020
	ctx.r[3].s64 = ctx.r[9].s64 + -4128;
	// 8326F50C: 4BA3AA15  bl 0x82ca9f20
	ctx.lr = 0x8326F510;
	sub_82CA9F20(ctx, base);
	// 8326F510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F520 size=64
    let mut pc: u32 = 0x8326F520;
    'dispatch: loop {
        match pc {
            0x8326F520 => {
    //   block [0x8326F520..0x8326F560)
	// 8326F520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F52C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326F530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F534: 388B9D94  addi r4, r11, -0x626c
	ctx.r[4].s64 = ctx.r[11].s64 + -25196;
	// 8326F538: 386ACCF8  addi r3, r10, -0x3308
	ctx.r[3].s64 = ctx.r[10].s64 + -13064;
	// 8326F53C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326F540: 4AFBD991  bl 0x8222ced0
	ctx.lr = 0x8326F544;
	sub_8222CED0(ctx, base);
	// 8326F544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326F548: 3869EFF0  addi r3, r9, -0x1010
	ctx.r[3].s64 = ctx.r[9].s64 + -4112;
	// 8326F54C: 4BA3A9D5  bl 0x82ca9f20
	ctx.lr = 0x8326F550;
	sub_82CA9F20(ctx, base);
	// 8326F550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F560 size=56
    let mut pc: u32 = 0x8326F560;
    'dispatch: loop {
        match pc {
            0x8326F560 => {
    //   block [0x8326F560..0x8326F598)
	// 8326F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F56C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F570: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F574: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326F578: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F57C: 4AF847DD  bl 0x821f3d58
	ctx.lr = 0x8326F580;
	sub_821F3D58(ctx, base);
	// 8326F580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F584: 906ACCFC  stw r3, -0x3304(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13060 as u32), ctx.r[3].u32 ) };
	// 8326F588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F598 size=56
    let mut pc: u32 = 0x8326F598;
    'dispatch: loop {
        match pc {
            0x8326F598 => {
    //   block [0x8326F598..0x8326F5D0)
	// 8326F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F5A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F5A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F5AC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326F5B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F5B4: 4AF847A5  bl 0x821f3d58
	ctx.lr = 0x8326F5B8;
	sub_821F3D58(ctx, base);
	// 8326F5B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F5BC: 906ACD00  stw r3, -0x3300(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13056 as u32), ctx.r[3].u32 ) };
	// 8326F5C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F5D0 size=56
    let mut pc: u32 = 0x8326F5D0;
    'dispatch: loop {
        match pc {
            0x8326F5D0 => {
    //   block [0x8326F5D0..0x8326F608)
	// 8326F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F5D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F5DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F5E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F5E4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326F5E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F5EC: 4AF8476D  bl 0x821f3d58
	ctx.lr = 0x8326F5F0;
	sub_821F3D58(ctx, base);
	// 8326F5F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F5F4: 906ACD04  stw r3, -0x32fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13052 as u32), ctx.r[3].u32 ) };
	// 8326F5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F608 size=56
    let mut pc: u32 = 0x8326F608;
    'dispatch: loop {
        match pc {
            0x8326F608 => {
    //   block [0x8326F608..0x8326F640)
	// 8326F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F614: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F618: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F61C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326F620: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F624: 4AF84735  bl 0x821f3d58
	ctx.lr = 0x8326F628;
	sub_821F3D58(ctx, base);
	// 8326F628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F62C: 906ACD08  stw r3, -0x32f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13048 as u32), ctx.r[3].u32 ) };
	// 8326F630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F640 size=56
    let mut pc: u32 = 0x8326F640;
    'dispatch: loop {
        match pc {
            0x8326F640 => {
    //   block [0x8326F640..0x8326F678)
	// 8326F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F64C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F650: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F654: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326F658: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F65C: 4AF846FD  bl 0x821f3d58
	ctx.lr = 0x8326F660;
	sub_821F3D58(ctx, base);
	// 8326F660: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F664: 906ACD0C  stw r3, -0x32f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13044 as u32), ctx.r[3].u32 ) };
	// 8326F668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F678 size=56
    let mut pc: u32 = 0x8326F678;
    'dispatch: loop {
        match pc {
            0x8326F678 => {
    //   block [0x8326F678..0x8326F6B0)
	// 8326F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F68C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326F690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F694: 4AF846C5  bl 0x821f3d58
	ctx.lr = 0x8326F698;
	sub_821F3D58(ctx, base);
	// 8326F698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F69C: 906ACD10  stw r3, -0x32f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13040 as u32), ctx.r[3].u32 ) };
	// 8326F6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F6B0 size=56
    let mut pc: u32 = 0x8326F6B0;
    'dispatch: loop {
        match pc {
            0x8326F6B0 => {
    //   block [0x8326F6B0..0x8326F6E8)
	// 8326F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F6BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F6C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F6C4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326F6C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F6CC: 4AF8468D  bl 0x821f3d58
	ctx.lr = 0x8326F6D0;
	sub_821F3D58(ctx, base);
	// 8326F6D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F6D4: 906ACD14  stw r3, -0x32ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13036 as u32), ctx.r[3].u32 ) };
	// 8326F6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F6E8 size=56
    let mut pc: u32 = 0x8326F6E8;
    'dispatch: loop {
        match pc {
            0x8326F6E8 => {
    //   block [0x8326F6E8..0x8326F720)
	// 8326F6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F6F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F6F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F6FC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326F700: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F704: 4AF84655  bl 0x821f3d58
	ctx.lr = 0x8326F708;
	sub_821F3D58(ctx, base);
	// 8326F708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F70C: 906ACD18  stw r3, -0x32e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13032 as u32), ctx.r[3].u32 ) };
	// 8326F710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F720 size=56
    let mut pc: u32 = 0x8326F720;
    'dispatch: loop {
        match pc {
            0x8326F720 => {
    //   block [0x8326F720..0x8326F758)
	// 8326F720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F72C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F730: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F734: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326F738: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F73C: 4AF8461D  bl 0x821f3d58
	ctx.lr = 0x8326F740;
	sub_821F3D58(ctx, base);
	// 8326F740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F744: 906ACD1C  stw r3, -0x32e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13028 as u32), ctx.r[3].u32 ) };
	// 8326F748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F758 size=56
    let mut pc: u32 = 0x8326F758;
    'dispatch: loop {
        match pc {
            0x8326F758 => {
    //   block [0x8326F758..0x8326F790)
	// 8326F758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F76C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326F770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F774: 4AF845E5  bl 0x821f3d58
	ctx.lr = 0x8326F778;
	sub_821F3D58(ctx, base);
	// 8326F778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F77C: 906ACD20  stw r3, -0x32e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13024 as u32), ctx.r[3].u32 ) };
	// 8326F780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F790 size=56
    let mut pc: u32 = 0x8326F790;
    'dispatch: loop {
        match pc {
            0x8326F790 => {
    //   block [0x8326F790..0x8326F7C8)
	// 8326F790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F79C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F7A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F7A4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326F7A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F7AC: 4AF845AD  bl 0x821f3d58
	ctx.lr = 0x8326F7B0;
	sub_821F3D58(ctx, base);
	// 8326F7B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F7B4: 906ACD24  stw r3, -0x32dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13020 as u32), ctx.r[3].u32 ) };
	// 8326F7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F7C8 size=56
    let mut pc: u32 = 0x8326F7C8;
    'dispatch: loop {
        match pc {
            0x8326F7C8 => {
    //   block [0x8326F7C8..0x8326F800)
	// 8326F7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F7D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F7D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F7DC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326F7E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F7E4: 4AF84575  bl 0x821f3d58
	ctx.lr = 0x8326F7E8;
	sub_821F3D58(ctx, base);
	// 8326F7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F7EC: 906ACD28  stw r3, -0x32d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13016 as u32), ctx.r[3].u32 ) };
	// 8326F7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F800 size=56
    let mut pc: u32 = 0x8326F800;
    'dispatch: loop {
        match pc {
            0x8326F800 => {
    //   block [0x8326F800..0x8326F838)
	// 8326F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F814: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326F818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F81C: 4AF8453D  bl 0x821f3d58
	ctx.lr = 0x8326F820;
	sub_821F3D58(ctx, base);
	// 8326F820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F824: 906ACD2C  stw r3, -0x32d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13012 as u32), ctx.r[3].u32 ) };
	// 8326F828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F838 size=56
    let mut pc: u32 = 0x8326F838;
    'dispatch: loop {
        match pc {
            0x8326F838 => {
    //   block [0x8326F838..0x8326F870)
	// 8326F838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F84C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326F850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F854: 4AF84505  bl 0x821f3d58
	ctx.lr = 0x8326F858;
	sub_821F3D58(ctx, base);
	// 8326F858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F85C: 906ACD30  stw r3, -0x32d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13008 as u32), ctx.r[3].u32 ) };
	// 8326F860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F870 size=56
    let mut pc: u32 = 0x8326F870;
    'dispatch: loop {
        match pc {
            0x8326F870 => {
    //   block [0x8326F870..0x8326F8A8)
	// 8326F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F87C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F884: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326F888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F88C: 4AF844CD  bl 0x821f3d58
	ctx.lr = 0x8326F890;
	sub_821F3D58(ctx, base);
	// 8326F890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F894: 906ACD34  stw r3, -0x32cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13004 as u32), ctx.r[3].u32 ) };
	// 8326F898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F8A8 size=56
    let mut pc: u32 = 0x8326F8A8;
    'dispatch: loop {
        match pc {
            0x8326F8A8 => {
    //   block [0x8326F8A8..0x8326F8E0)
	// 8326F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F8B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F8BC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326F8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F8C4: 4AF84495  bl 0x821f3d58
	ctx.lr = 0x8326F8C8;
	sub_821F3D58(ctx, base);
	// 8326F8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F8CC: 906ACD38  stw r3, -0x32c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13000 as u32), ctx.r[3].u32 ) };
	// 8326F8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F8E0 size=56
    let mut pc: u32 = 0x8326F8E0;
    'dispatch: loop {
        match pc {
            0x8326F8E0 => {
    //   block [0x8326F8E0..0x8326F918)
	// 8326F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F8F4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326F8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F8FC: 4AF8445D  bl 0x821f3d58
	ctx.lr = 0x8326F900;
	sub_821F3D58(ctx, base);
	// 8326F900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F904: 906ACD3C  stw r3, -0x32c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12996 as u32), ctx.r[3].u32 ) };
	// 8326F908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F918 size=56
    let mut pc: u32 = 0x8326F918;
    'dispatch: loop {
        match pc {
            0x8326F918 => {
    //   block [0x8326F918..0x8326F950)
	// 8326F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F92C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326F930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F934: 4AF84425  bl 0x821f3d58
	ctx.lr = 0x8326F938;
	sub_821F3D58(ctx, base);
	// 8326F938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F93C: 906ACD40  stw r3, -0x32c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12992 as u32), ctx.r[3].u32 ) };
	// 8326F940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F950 size=56
    let mut pc: u32 = 0x8326F950;
    'dispatch: loop {
        match pc {
            0x8326F950 => {
    //   block [0x8326F950..0x8326F988)
	// 8326F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F95C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F964: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326F968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F96C: 4AF843ED  bl 0x821f3d58
	ctx.lr = 0x8326F970;
	sub_821F3D58(ctx, base);
	// 8326F970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F974: 906ACD44  stw r3, -0x32bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12988 as u32), ctx.r[3].u32 ) };
	// 8326F978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F988 size=56
    let mut pc: u32 = 0x8326F988;
    'dispatch: loop {
        match pc {
            0x8326F988 => {
    //   block [0x8326F988..0x8326F9C0)
	// 8326F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F99C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326F9A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F9A4: 4AF843B5  bl 0x821f3d58
	ctx.lr = 0x8326F9A8;
	sub_821F3D58(ctx, base);
	// 8326F9A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F9AC: 906ACD48  stw r3, -0x32b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12984 as u32), ctx.r[3].u32 ) };
	// 8326F9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F9C0 size=56
    let mut pc: u32 = 0x8326F9C0;
    'dispatch: loop {
        match pc {
            0x8326F9C0 => {
    //   block [0x8326F9C0..0x8326F9F8)
	// 8326F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F9CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F9D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F9D4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326F9D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F9DC: 4AF8437D  bl 0x821f3d58
	ctx.lr = 0x8326F9E0;
	sub_821F3D58(ctx, base);
	// 8326F9E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F9E4: 906ACD4C  stw r3, -0x32b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12980 as u32), ctx.r[3].u32 ) };
	// 8326F9E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F9F8 size=64
    let mut pc: u32 = 0x8326F9F8;
    'dispatch: loop {
        match pc {
            0x8326F9F8 => {
    //   block [0x8326F9F8..0x8326FA38)
	// 8326F9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FA04: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FA08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FA0C: 388B9EAC  addi r4, r11, -0x6154
	ctx.r[4].s64 = ctx.r[11].s64 + -24916;
	// 8326FA10: 386ACD50  addi r3, r10, -0x32b0
	ctx.r[3].s64 = ctx.r[10].s64 + -12976;
	// 8326FA14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FA18: 4AFBD4B9  bl 0x8222ced0
	ctx.lr = 0x8326FA1C;
	sub_8222CED0(ctx, base);
	// 8326FA1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FA20: 3869F000  addi r3, r9, -0x1000
	ctx.r[3].s64 = ctx.r[9].s64 + -4096;
	// 8326FA24: 4BA3A4FD  bl 0x82ca9f20
	ctx.lr = 0x8326FA28;
	sub_82CA9F20(ctx, base);
	// 8326FA28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FA38 size=64
    let mut pc: u32 = 0x8326FA38;
    'dispatch: loop {
        match pc {
            0x8326FA38 => {
    //   block [0x8326FA38..0x8326FA78)
	// 8326FA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FA44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326FA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FA4C: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8326FA50: 386ACD54  addi r3, r10, -0x32ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	// 8326FA54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FA58: 4AFBD479  bl 0x8222ced0
	ctx.lr = 0x8326FA5C;
	sub_8222CED0(ctx, base);
	// 8326FA5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FA60: 3869F010  addi r3, r9, -0xff0
	ctx.r[3].s64 = ctx.r[9].s64 + -4080;
	// 8326FA64: 4BA3A4BD  bl 0x82ca9f20
	ctx.lr = 0x8326FA68;
	sub_82CA9F20(ctx, base);
	// 8326FA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FA78 size=64
    let mut pc: u32 = 0x8326FA78;
    'dispatch: loop {
        match pc {
            0x8326FA78 => {
    //   block [0x8326FA78..0x8326FAB8)
	// 8326FA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FA84: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FA88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FA8C: 388B9ED8  addi r4, r11, -0x6128
	ctx.r[4].s64 = ctx.r[11].s64 + -24872;
	// 8326FA90: 386ACD58  addi r3, r10, -0x32a8
	ctx.r[3].s64 = ctx.r[10].s64 + -12968;
	// 8326FA94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FA98: 4AFBD439  bl 0x8222ced0
	ctx.lr = 0x8326FA9C;
	sub_8222CED0(ctx, base);
	// 8326FA9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FAA0: 3869F020  addi r3, r9, -0xfe0
	ctx.r[3].s64 = ctx.r[9].s64 + -4064;
	// 8326FAA4: 4BA3A47D  bl 0x82ca9f20
	ctx.lr = 0x8326FAA8;
	sub_82CA9F20(ctx, base);
	// 8326FAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FAB8 size=56
    let mut pc: u32 = 0x8326FAB8;
    'dispatch: loop {
        match pc {
            0x8326FAB8 => {
    //   block [0x8326FAB8..0x8326FAF0)
	// 8326FAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FACC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326FAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FAD4: 4AF84285  bl 0x821f3d58
	ctx.lr = 0x8326FAD8;
	sub_821F3D58(ctx, base);
	// 8326FAD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FADC: 906ACD5C  stw r3, -0x32a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12964 as u32), ctx.r[3].u32 ) };
	// 8326FAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FAF0 size=56
    let mut pc: u32 = 0x8326FAF0;
    'dispatch: loop {
        match pc {
            0x8326FAF0 => {
    //   block [0x8326FAF0..0x8326FB28)
	// 8326FAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FB04: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326FB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FB0C: 4AF8424D  bl 0x821f3d58
	ctx.lr = 0x8326FB10;
	sub_821F3D58(ctx, base);
	// 8326FB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FB14: 906ACD60  stw r3, -0x32a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12960 as u32), ctx.r[3].u32 ) };
	// 8326FB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB28 size=56
    let mut pc: u32 = 0x8326FB28;
    'dispatch: loop {
        match pc {
            0x8326FB28 => {
    //   block [0x8326FB28..0x8326FB60)
	// 8326FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FB3C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326FB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FB44: 4AF84215  bl 0x821f3d58
	ctx.lr = 0x8326FB48;
	sub_821F3D58(ctx, base);
	// 8326FB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FB4C: 906ACD64  stw r3, -0x329c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12956 as u32), ctx.r[3].u32 ) };
	// 8326FB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB60 size=56
    let mut pc: u32 = 0x8326FB60;
    'dispatch: loop {
        match pc {
            0x8326FB60 => {
    //   block [0x8326FB60..0x8326FB98)
	// 8326FB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FB74: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326FB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FB7C: 4AF841DD  bl 0x821f3d58
	ctx.lr = 0x8326FB80;
	sub_821F3D58(ctx, base);
	// 8326FB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FB84: 906ACD68  stw r3, -0x3298(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12952 as u32), ctx.r[3].u32 ) };
	// 8326FB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB98 size=56
    let mut pc: u32 = 0x8326FB98;
    'dispatch: loop {
        match pc {
            0x8326FB98 => {
    //   block [0x8326FB98..0x8326FBD0)
	// 8326FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FBA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FBA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FBAC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326FBB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FBB4: 4AF841A5  bl 0x821f3d58
	ctx.lr = 0x8326FBB8;
	sub_821F3D58(ctx, base);
	// 8326FBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FBBC: 906ACD6C  stw r3, -0x3294(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12948 as u32), ctx.r[3].u32 ) };
	// 8326FBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FBD0 size=56
    let mut pc: u32 = 0x8326FBD0;
    'dispatch: loop {
        match pc {
            0x8326FBD0 => {
    //   block [0x8326FBD0..0x8326FC08)
	// 8326FBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FBD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FBDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FBE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FBE4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326FBE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FBEC: 4AF8416D  bl 0x821f3d58
	ctx.lr = 0x8326FBF0;
	sub_821F3D58(ctx, base);
	// 8326FBF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FBF4: 906ACD70  stw r3, -0x3290(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12944 as u32), ctx.r[3].u32 ) };
	// 8326FBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC08 size=56
    let mut pc: u32 = 0x8326FC08;
    'dispatch: loop {
        match pc {
            0x8326FC08 => {
    //   block [0x8326FC08..0x8326FC40)
	// 8326FC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FC14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FC18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FC1C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326FC20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FC24: 4AF84135  bl 0x821f3d58
	ctx.lr = 0x8326FC28;
	sub_821F3D58(ctx, base);
	// 8326FC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FC2C: 906ACD74  stw r3, -0x328c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12940 as u32), ctx.r[3].u32 ) };
	// 8326FC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC40 size=56
    let mut pc: u32 = 0x8326FC40;
    'dispatch: loop {
        match pc {
            0x8326FC40 => {
    //   block [0x8326FC40..0x8326FC78)
	// 8326FC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FC50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FC54: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326FC58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FC5C: 4AF840FD  bl 0x821f3d58
	ctx.lr = 0x8326FC60;
	sub_821F3D58(ctx, base);
	// 8326FC60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FC64: 906ACD78  stw r3, -0x3288(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12936 as u32), ctx.r[3].u32 ) };
	// 8326FC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC78 size=56
    let mut pc: u32 = 0x8326FC78;
    'dispatch: loop {
        match pc {
            0x8326FC78 => {
    //   block [0x8326FC78..0x8326FCB0)
	// 8326FC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FC8C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326FC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FC94: 4AF840C5  bl 0x821f3d58
	ctx.lr = 0x8326FC98;
	sub_821F3D58(ctx, base);
	// 8326FC98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FC9C: 906ACD7C  stw r3, -0x3284(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12932 as u32), ctx.r[3].u32 ) };
	// 8326FCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FCB0 size=56
    let mut pc: u32 = 0x8326FCB0;
    'dispatch: loop {
        match pc {
            0x8326FCB0 => {
    //   block [0x8326FCB0..0x8326FCE8)
	// 8326FCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FCC4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326FCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FCCC: 4AF8408D  bl 0x821f3d58
	ctx.lr = 0x8326FCD0;
	sub_821F3D58(ctx, base);
	// 8326FCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FCD4: 906ACD80  stw r3, -0x3280(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12928 as u32), ctx.r[3].u32 ) };
	// 8326FCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FCE8 size=56
    let mut pc: u32 = 0x8326FCE8;
    'dispatch: loop {
        match pc {
            0x8326FCE8 => {
    //   block [0x8326FCE8..0x8326FD20)
	// 8326FCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FCFC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326FD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FD04: 4AF84055  bl 0x821f3d58
	ctx.lr = 0x8326FD08;
	sub_821F3D58(ctx, base);
	// 8326FD08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FD0C: 906ACD84  stw r3, -0x327c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12924 as u32), ctx.r[3].u32 ) };
	// 8326FD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD20 size=56
    let mut pc: u32 = 0x8326FD20;
    'dispatch: loop {
        match pc {
            0x8326FD20 => {
    //   block [0x8326FD20..0x8326FD58)
	// 8326FD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FD34: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326FD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FD3C: 4AF8401D  bl 0x821f3d58
	ctx.lr = 0x8326FD40;
	sub_821F3D58(ctx, base);
	// 8326FD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FD44: 906ACD88  stw r3, -0x3278(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12920 as u32), ctx.r[3].u32 ) };
	// 8326FD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD58 size=56
    let mut pc: u32 = 0x8326FD58;
    'dispatch: loop {
        match pc {
            0x8326FD58 => {
    //   block [0x8326FD58..0x8326FD90)
	// 8326FD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FD6C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326FD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FD74: 4AF83FE5  bl 0x821f3d58
	ctx.lr = 0x8326FD78;
	sub_821F3D58(ctx, base);
	// 8326FD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FD7C: 906ACD8C  stw r3, -0x3274(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12916 as u32), ctx.r[3].u32 ) };
	// 8326FD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD90 size=56
    let mut pc: u32 = 0x8326FD90;
    'dispatch: loop {
        match pc {
            0x8326FD90 => {
    //   block [0x8326FD90..0x8326FDC8)
	// 8326FD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FDA4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326FDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FDAC: 4AF83FAD  bl 0x821f3d58
	ctx.lr = 0x8326FDB0;
	sub_821F3D58(ctx, base);
	// 8326FDB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FDB4: 906ACD90  stw r3, -0x3270(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12912 as u32), ctx.r[3].u32 ) };
	// 8326FDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FDC8 size=56
    let mut pc: u32 = 0x8326FDC8;
    'dispatch: loop {
        match pc {
            0x8326FDC8 => {
    //   block [0x8326FDC8..0x8326FE00)
	// 8326FDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FDDC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326FDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FDE4: 4AF83F75  bl 0x821f3d58
	ctx.lr = 0x8326FDE8;
	sub_821F3D58(ctx, base);
	// 8326FDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FDEC: 906ACD94  stw r3, -0x326c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12908 as u32), ctx.r[3].u32 ) };
	// 8326FDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE00 size=56
    let mut pc: u32 = 0x8326FE00;
    'dispatch: loop {
        match pc {
            0x8326FE00 => {
    //   block [0x8326FE00..0x8326FE38)
	// 8326FE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FE14: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326FE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FE1C: 4AF83F3D  bl 0x821f3d58
	ctx.lr = 0x8326FE20;
	sub_821F3D58(ctx, base);
	// 8326FE20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FE24: 906ACD98  stw r3, -0x3268(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12904 as u32), ctx.r[3].u32 ) };
	// 8326FE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE38 size=56
    let mut pc: u32 = 0x8326FE38;
    'dispatch: loop {
        match pc {
            0x8326FE38 => {
    //   block [0x8326FE38..0x8326FE70)
	// 8326FE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FE4C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326FE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FE54: 4AF83F05  bl 0x821f3d58
	ctx.lr = 0x8326FE58;
	sub_821F3D58(ctx, base);
	// 8326FE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FE5C: 906ACD9C  stw r3, -0x3264(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12900 as u32), ctx.r[3].u32 ) };
	// 8326FE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE70 size=56
    let mut pc: u32 = 0x8326FE70;
    'dispatch: loop {
        match pc {
            0x8326FE70 => {
    //   block [0x8326FE70..0x8326FEA8)
	// 8326FE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FE84: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326FE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FE8C: 4AF83ECD  bl 0x821f3d58
	ctx.lr = 0x8326FE90;
	sub_821F3D58(ctx, base);
	// 8326FE90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FE94: 906ACDA0  stw r3, -0x3260(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12896 as u32), ctx.r[3].u32 ) };
	// 8326FE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FEA8 size=56
    let mut pc: u32 = 0x8326FEA8;
    'dispatch: loop {
        match pc {
            0x8326FEA8 => {
    //   block [0x8326FEA8..0x8326FEE0)
	// 8326FEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FEB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FEBC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326FEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FEC4: 4AF83E95  bl 0x821f3d58
	ctx.lr = 0x8326FEC8;
	sub_821F3D58(ctx, base);
	// 8326FEC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FECC: 906ACDA4  stw r3, -0x325c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12892 as u32), ctx.r[3].u32 ) };
	// 8326FED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FEE0 size=56
    let mut pc: u32 = 0x8326FEE0;
    'dispatch: loop {
        match pc {
            0x8326FEE0 => {
    //   block [0x8326FEE0..0x8326FF18)
	// 8326FEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FEEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FEF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FEF4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326FEF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FEFC: 4AF83E5D  bl 0x821f3d58
	ctx.lr = 0x8326FF00;
	sub_821F3D58(ctx, base);
	// 8326FF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FF04: 906ACDA8  stw r3, -0x3258(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12888 as u32), ctx.r[3].u32 ) };
	// 8326FF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF18 size=56
    let mut pc: u32 = 0x8326FF18;
    'dispatch: loop {
        match pc {
            0x8326FF18 => {
    //   block [0x8326FF18..0x8326FF50)
	// 8326FF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FF24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FF2C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326FF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FF34: 4AF83E25  bl 0x821f3d58
	ctx.lr = 0x8326FF38;
	sub_821F3D58(ctx, base);
	// 8326FF38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FF3C: 906ACDAC  stw r3, -0x3254(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12884 as u32), ctx.r[3].u32 ) };
	// 8326FF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF50 size=64
    let mut pc: u32 = 0x8326FF50;
    'dispatch: loop {
        match pc {
            0x8326FF50 => {
    //   block [0x8326FF50..0x8326FF90)
	// 8326FF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FF5C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FF60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FF64: 388B9F54  addi r4, r11, -0x60ac
	ctx.r[4].s64 = ctx.r[11].s64 + -24748;
	// 8326FF68: 386ACDB0  addi r3, r10, -0x3250
	ctx.r[3].s64 = ctx.r[10].s64 + -12880;
	// 8326FF6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FF70: 4AFBCF61  bl 0x8222ced0
	ctx.lr = 0x8326FF74;
	sub_8222CED0(ctx, base);
	// 8326FF74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FF78: 3869F030  addi r3, r9, -0xfd0
	ctx.r[3].s64 = ctx.r[9].s64 + -4048;
	// 8326FF7C: 4BA39FA5  bl 0x82ca9f20
	ctx.lr = 0x8326FF80;
	sub_82CA9F20(ctx, base);
	// 8326FF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF90 size=64
    let mut pc: u32 = 0x8326FF90;
    'dispatch: loop {
        match pc {
            0x8326FF90 => {
    //   block [0x8326FF90..0x8326FFD0)
	// 8326FF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FF9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326FFA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FFA4: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8326FFA8: 386ACDB4  addi r3, r10, -0x324c
	ctx.r[3].s64 = ctx.r[10].s64 + -12876;
	// 8326FFAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FFB0: 4AFBCF21  bl 0x8222ced0
	ctx.lr = 0x8326FFB4;
	sub_8222CED0(ctx, base);
	// 8326FFB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FFB8: 3869F040  addi r3, r9, -0xfc0
	ctx.r[3].s64 = ctx.r[9].s64 + -4032;
	// 8326FFBC: 4BA39F65  bl 0x82ca9f20
	ctx.lr = 0x8326FFC0;
	sub_82CA9F20(ctx, base);
	// 8326FFC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FFD0 size=64
    let mut pc: u32 = 0x8326FFD0;
    'dispatch: loop {
        match pc {
            0x8326FFD0 => {
    //   block [0x8326FFD0..0x83270010)
	// 8326FFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FFD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FFDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FFE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FFE4: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 8326FFE8: 386ACDB8  addi r3, r10, -0x3248
	ctx.r[3].s64 = ctx.r[10].s64 + -12872;
	// 8326FFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FFF0: 4AFBCEE1  bl 0x8222ced0
	ctx.lr = 0x8326FFF4;
	sub_8222CED0(ctx, base);
	// 8326FFF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FFF8: 3869F050  addi r3, r9, -0xfb0
	ctx.r[3].s64 = ctx.r[9].s64 + -4016;
	// 8326FFFC: 4BA39F25  bl 0x82ca9f20
	ctx.lr = 0x83270000;
	sub_82CA9F20(ctx, base);
	// 83270000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327000C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270010 size=56
    let mut pc: u32 = 0x83270010;
    'dispatch: loop {
        match pc {
            0x83270010 => {
    //   block [0x83270010..0x83270048)
	// 83270010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327001C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270020: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270024: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83270028: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327002C: 4AF83D2D  bl 0x821f3d58
	ctx.lr = 0x83270030;
	sub_821F3D58(ctx, base);
	// 83270030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270034: 906ACDBC  stw r3, -0x3244(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12868 as u32), ctx.r[3].u32 ) };
	// 83270038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327003C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270048 size=56
    let mut pc: u32 = 0x83270048;
    'dispatch: loop {
        match pc {
            0x83270048 => {
    //   block [0x83270048..0x83270080)
	// 83270048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327004C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270054: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270058: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327005C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83270060: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270064: 4AF83CF5  bl 0x821f3d58
	ctx.lr = 0x83270068;
	sub_821F3D58(ctx, base);
	// 83270068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327006C: 906ACDC0  stw r3, -0x3240(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12864 as u32), ctx.r[3].u32 ) };
	// 83270070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327007C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270080 size=56
    let mut pc: u32 = 0x83270080;
    'dispatch: loop {
        match pc {
            0x83270080 => {
    //   block [0x83270080..0x832700B8)
	// 83270080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327008C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270090: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270094: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83270098: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327009C: 4AF83CBD  bl 0x821f3d58
	ctx.lr = 0x832700A0;
	sub_821F3D58(ctx, base);
	// 832700A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832700A4: 906ACDC4  stw r3, -0x323c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12860 as u32), ctx.r[3].u32 ) };
	// 832700A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832700AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832700B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832700B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832700B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832700B8 size=56
    let mut pc: u32 = 0x832700B8;
    'dispatch: loop {
        match pc {
            0x832700B8 => {
    //   block [0x832700B8..0x832700F0)
	// 832700B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832700BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832700C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832700C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832700C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832700CC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832700D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832700D4: 4AF83C85  bl 0x821f3d58
	ctx.lr = 0x832700D8;
	sub_821F3D58(ctx, base);
	// 832700D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832700DC: 906ACDC8  stw r3, -0x3238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12856 as u32), ctx.r[3].u32 ) };
	// 832700E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832700E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832700E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832700EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832700F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832700F0 size=56
    let mut pc: u32 = 0x832700F0;
    'dispatch: loop {
        match pc {
            0x832700F0 => {
    //   block [0x832700F0..0x83270128)
	// 832700F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832700F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832700F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832700FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270100: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270104: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83270108: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327010C: 4AF83C4D  bl 0x821f3d58
	ctx.lr = 0x83270110;
	sub_821F3D58(ctx, base);
	// 83270110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270114: 906ACDCC  stw r3, -0x3234(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12852 as u32), ctx.r[3].u32 ) };
	// 83270118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327011C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270128 size=56
    let mut pc: u32 = 0x83270128;
    'dispatch: loop {
        match pc {
            0x83270128 => {
    //   block [0x83270128..0x83270160)
	// 83270128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327012C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270134: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270138: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327013C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83270140: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270144: 4AF83C15  bl 0x821f3d58
	ctx.lr = 0x83270148;
	sub_821F3D58(ctx, base);
	// 83270148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327014C: 906ACDD0  stw r3, -0x3230(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12848 as u32), ctx.r[3].u32 ) };
	// 83270150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270160 size=56
    let mut pc: u32 = 0x83270160;
    'dispatch: loop {
        match pc {
            0x83270160 => {
    //   block [0x83270160..0x83270198)
	// 83270160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327016C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270170: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270174: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83270178: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327017C: 4AF83BDD  bl 0x821f3d58
	ctx.lr = 0x83270180;
	sub_821F3D58(ctx, base);
	// 83270180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270184: 906ACDD4  stw r3, -0x322c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12844 as u32), ctx.r[3].u32 ) };
	// 83270188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327018C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270198 size=56
    let mut pc: u32 = 0x83270198;
    'dispatch: loop {
        match pc {
            0x83270198 => {
    //   block [0x83270198..0x832701D0)
	// 83270198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327019C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832701A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832701A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832701A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832701AC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832701B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832701B4: 4AF83BA5  bl 0x821f3d58
	ctx.lr = 0x832701B8;
	sub_821F3D58(ctx, base);
	// 832701B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832701BC: 906ACDD8  stw r3, -0x3228(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12840 as u32), ctx.r[3].u32 ) };
	// 832701C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832701C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832701C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832701CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832701D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832701D0 size=56
    let mut pc: u32 = 0x832701D0;
    'dispatch: loop {
        match pc {
            0x832701D0 => {
    //   block [0x832701D0..0x83270208)
	// 832701D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832701D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832701D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832701DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832701E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832701E4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832701E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832701EC: 4AF83B6D  bl 0x821f3d58
	ctx.lr = 0x832701F0;
	sub_821F3D58(ctx, base);
	// 832701F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832701F4: 906ACDDC  stw r3, -0x3224(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12836 as u32), ctx.r[3].u32 ) };
	// 832701F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832701FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270208 size=56
    let mut pc: u32 = 0x83270208;
    'dispatch: loop {
        match pc {
            0x83270208 => {
    //   block [0x83270208..0x83270240)
	// 83270208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270214: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270218: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327021C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83270220: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270224: 4AF83B35  bl 0x821f3d58
	ctx.lr = 0x83270228;
	sub_821F3D58(ctx, base);
	// 83270228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327022C: 906ACDE0  stw r3, -0x3220(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12832 as u32), ctx.r[3].u32 ) };
	// 83270230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327023C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270240 size=56
    let mut pc: u32 = 0x83270240;
    'dispatch: loop {
        match pc {
            0x83270240 => {
    //   block [0x83270240..0x83270278)
	// 83270240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327024C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270250: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270254: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83270258: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327025C: 4AF83AFD  bl 0x821f3d58
	ctx.lr = 0x83270260;
	sub_821F3D58(ctx, base);
	// 83270260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270264: 906ACDE4  stw r3, -0x321c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12828 as u32), ctx.r[3].u32 ) };
	// 83270268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327026C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270278 size=56
    let mut pc: u32 = 0x83270278;
    'dispatch: loop {
        match pc {
            0x83270278 => {
    //   block [0x83270278..0x832702B0)
	// 83270278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327027C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270288: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327028C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83270290: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270294: 4AF83AC5  bl 0x821f3d58
	ctx.lr = 0x83270298;
	sub_821F3D58(ctx, base);
	// 83270298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327029C: 906ACDE8  stw r3, -0x3218(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12824 as u32), ctx.r[3].u32 ) };
	// 832702A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832702A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832702A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832702AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832702B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832702B0 size=56
    let mut pc: u32 = 0x832702B0;
    'dispatch: loop {
        match pc {
            0x832702B0 => {
    //   block [0x832702B0..0x832702E8)
	// 832702B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832702B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832702B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832702BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832702C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832702C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832702C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832702CC: 4AF83A8D  bl 0x821f3d58
	ctx.lr = 0x832702D0;
	sub_821F3D58(ctx, base);
	// 832702D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832702D4: 906ACDEC  stw r3, -0x3214(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12820 as u32), ctx.r[3].u32 ) };
	// 832702D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832702DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832702E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832702E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832702E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832702E8 size=56
    let mut pc: u32 = 0x832702E8;
    'dispatch: loop {
        match pc {
            0x832702E8 => {
    //   block [0x832702E8..0x83270320)
	// 832702E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832702EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832702F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832702F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832702F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832702FC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83270300: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270304: 4AF83A55  bl 0x821f3d58
	ctx.lr = 0x83270308;
	sub_821F3D58(ctx, base);
	// 83270308: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327030C: 906ACDF0  stw r3, -0x3210(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12816 as u32), ctx.r[3].u32 ) };
	// 83270310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270320 size=56
    let mut pc: u32 = 0x83270320;
    'dispatch: loop {
        match pc {
            0x83270320 => {
    //   block [0x83270320..0x83270358)
	// 83270320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327032C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270330: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270334: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83270338: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327033C: 4AF83A1D  bl 0x821f3d58
	ctx.lr = 0x83270340;
	sub_821F3D58(ctx, base);
	// 83270340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270344: 906ACDF4  stw r3, -0x320c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12812 as u32), ctx.r[3].u32 ) };
	// 83270348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270358 size=56
    let mut pc: u32 = 0x83270358;
    'dispatch: loop {
        match pc {
            0x83270358 => {
    //   block [0x83270358..0x83270390)
	// 83270358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327035C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270364: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270368: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327036C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83270370: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270374: 4AF839E5  bl 0x821f3d58
	ctx.lr = 0x83270378;
	sub_821F3D58(ctx, base);
	// 83270378: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327037C: 906ACDF8  stw r3, -0x3208(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12808 as u32), ctx.r[3].u32 ) };
	// 83270380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270390 size=56
    let mut pc: u32 = 0x83270390;
    'dispatch: loop {
        match pc {
            0x83270390 => {
    //   block [0x83270390..0x832703C8)
	// 83270390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327039C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832703A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832703A4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832703A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832703AC: 4AF839AD  bl 0x821f3d58
	ctx.lr = 0x832703B0;
	sub_821F3D58(ctx, base);
	// 832703B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832703B4: 906ACDFC  stw r3, -0x3204(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12804 as u32), ctx.r[3].u32 ) };
	// 832703B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832703BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832703C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832703C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832703C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832703C8 size=56
    let mut pc: u32 = 0x832703C8;
    'dispatch: loop {
        match pc {
            0x832703C8 => {
    //   block [0x832703C8..0x83270400)
	// 832703C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832703CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832703D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832703D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832703D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832703DC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832703E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832703E4: 4AF83975  bl 0x821f3d58
	ctx.lr = 0x832703E8;
	sub_821F3D58(ctx, base);
	// 832703E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832703EC: 906ACE00  stw r3, -0x3200(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12800 as u32), ctx.r[3].u32 ) };
	// 832703F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832703F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832703F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832703FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270400 size=56
    let mut pc: u32 = 0x83270400;
    'dispatch: loop {
        match pc {
            0x83270400 => {
    //   block [0x83270400..0x83270438)
	// 83270400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327040C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270410: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270414: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83270418: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327041C: 4AF8393D  bl 0x821f3d58
	ctx.lr = 0x83270420;
	sub_821F3D58(ctx, base);
	// 83270420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270424: 906ACE04  stw r3, -0x31fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12796 as u32), ctx.r[3].u32 ) };
	// 83270428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327042C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270438 size=56
    let mut pc: u32 = 0x83270438;
    'dispatch: loop {
        match pc {
            0x83270438 => {
    //   block [0x83270438..0x83270470)
	// 83270438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327043C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270448: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327044C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83270450: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270454: 4AF83905  bl 0x821f3d58
	ctx.lr = 0x83270458;
	sub_821F3D58(ctx, base);
	// 83270458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327045C: 906ACE08  stw r3, -0x31f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12792 as u32), ctx.r[3].u32 ) };
	// 83270460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327046C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270470 size=56
    let mut pc: u32 = 0x83270470;
    'dispatch: loop {
        match pc {
            0x83270470 => {
    //   block [0x83270470..0x832704A8)
	// 83270470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327047C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270480: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270484: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83270488: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327048C: 4AF838CD  bl 0x821f3d58
	ctx.lr = 0x83270490;
	sub_821F3D58(ctx, base);
	// 83270490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270494: 906ACE0C  stw r3, -0x31f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12788 as u32), ctx.r[3].u32 ) };
	// 83270498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832704A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832704A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832704A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832704A8 size=64
    let mut pc: u32 = 0x832704A8;
    'dispatch: loop {
        match pc {
            0x832704A8 => {
    //   block [0x832704A8..0x832704E8)
	// 832704A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832704AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832704B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832704B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832704B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832704BC: 388B9FD4  addi r4, r11, -0x602c
	ctx.r[4].s64 = ctx.r[11].s64 + -24620;
	// 832704C0: 386ACE10  addi r3, r10, -0x31f0
	ctx.r[3].s64 = ctx.r[10].s64 + -12784;
	// 832704C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832704C8: 4AFBCA09  bl 0x8222ced0
	ctx.lr = 0x832704CC;
	sub_8222CED0(ctx, base);
	// 832704CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832704D0: 3869F060  addi r3, r9, -0xfa0
	ctx.r[3].s64 = ctx.r[9].s64 + -4000;
	// 832704D4: 4BA39A4D  bl 0x82ca9f20
	ctx.lr = 0x832704D8;
	sub_82CA9F20(ctx, base);
	// 832704D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832704DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832704E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832704E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832704E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832704E8 size=64
    let mut pc: u32 = 0x832704E8;
    'dispatch: loop {
        match pc {
            0x832704E8 => {
    //   block [0x832704E8..0x83270528)
	// 832704E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832704EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832704F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832704F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832704F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832704FC: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 83270500: 386ACE14  addi r3, r10, -0x31ec
	ctx.r[3].s64 = ctx.r[10].s64 + -12780;
	// 83270504: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270508: 4AFBC9C9  bl 0x8222ced0
	ctx.lr = 0x8327050C;
	sub_8222CED0(ctx, base);
	// 8327050C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270510: 3869F070  addi r3, r9, -0xf90
	ctx.r[3].s64 = ctx.r[9].s64 + -3984;
	// 83270514: 4BA39A0D  bl 0x82ca9f20
	ctx.lr = 0x83270518;
	sub_82CA9F20(ctx, base);
	// 83270518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327051C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270528 size=64
    let mut pc: u32 = 0x83270528;
    'dispatch: loop {
        match pc {
            0x83270528 => {
    //   block [0x83270528..0x83270568)
	// 83270528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327052C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270534: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327053C: 388BA000  addi r4, r11, -0x6000
	ctx.r[4].s64 = ctx.r[11].s64 + -24576;
	// 83270540: 386ACE18  addi r3, r10, -0x31e8
	ctx.r[3].s64 = ctx.r[10].s64 + -12776;
	// 83270544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270548: 4AFBC989  bl 0x8222ced0
	ctx.lr = 0x8327054C;
	sub_8222CED0(ctx, base);
	// 8327054C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270550: 3869F080  addi r3, r9, -0xf80
	ctx.r[3].s64 = ctx.r[9].s64 + -3968;
	// 83270554: 4BA399CD  bl 0x82ca9f20
	ctx.lr = 0x83270558;
	sub_82CA9F20(ctx, base);
	// 83270558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327055C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270568 size=56
    let mut pc: u32 = 0x83270568;
    'dispatch: loop {
        match pc {
            0x83270568 => {
    //   block [0x83270568..0x832705A0)
	// 83270568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270574: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270578: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327057C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83270580: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270584: 4AF837D5  bl 0x821f3d58
	ctx.lr = 0x83270588;
	sub_821F3D58(ctx, base);
	// 83270588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327058C: 906ACE1C  stw r3, -0x31e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12772 as u32), ctx.r[3].u32 ) };
	// 83270590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327059C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832705A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832705A0 size=56
    let mut pc: u32 = 0x832705A0;
    'dispatch: loop {
        match pc {
            0x832705A0 => {
    //   block [0x832705A0..0x832705D8)
	// 832705A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832705A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832705A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832705AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832705B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832705B4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832705B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832705BC: 4AF8379D  bl 0x821f3d58
	ctx.lr = 0x832705C0;
	sub_821F3D58(ctx, base);
	// 832705C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832705C4: 906ACE20  stw r3, -0x31e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12768 as u32), ctx.r[3].u32 ) };
	// 832705C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832705CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832705D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832705D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832705D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832705D8 size=56
    let mut pc: u32 = 0x832705D8;
    'dispatch: loop {
        match pc {
            0x832705D8 => {
    //   block [0x832705D8..0x83270610)
	// 832705D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832705DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832705E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832705E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832705E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832705EC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832705F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832705F4: 4AF83765  bl 0x821f3d58
	ctx.lr = 0x832705F8;
	sub_821F3D58(ctx, base);
	// 832705F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832705FC: 906ACE24  stw r3, -0x31dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12764 as u32), ctx.r[3].u32 ) };
	// 83270600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327060C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270610 size=56
    let mut pc: u32 = 0x83270610;
    'dispatch: loop {
        match pc {
            0x83270610 => {
    //   block [0x83270610..0x83270648)
	// 83270610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327061C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270624: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83270628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327062C: 4AF8372D  bl 0x821f3d58
	ctx.lr = 0x83270630;
	sub_821F3D58(ctx, base);
	// 83270630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270634: 906ACE28  stw r3, -0x31d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12760 as u32), ctx.r[3].u32 ) };
	// 83270638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270648 size=56
    let mut pc: u32 = 0x83270648;
    'dispatch: loop {
        match pc {
            0x83270648 => {
    //   block [0x83270648..0x83270680)
	// 83270648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270654: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270658: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327065C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83270660: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270664: 4AF836F5  bl 0x821f3d58
	ctx.lr = 0x83270668;
	sub_821F3D58(ctx, base);
	// 83270668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327066C: 906ACE2C  stw r3, -0x31d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12756 as u32), ctx.r[3].u32 ) };
	// 83270670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327067C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270680 size=56
    let mut pc: u32 = 0x83270680;
    'dispatch: loop {
        match pc {
            0x83270680 => {
    //   block [0x83270680..0x832706B8)
	// 83270680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327068C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270690: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270694: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83270698: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327069C: 4AF836BD  bl 0x821f3d58
	ctx.lr = 0x832706A0;
	sub_821F3D58(ctx, base);
	// 832706A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832706A4: 906ACE30  stw r3, -0x31d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12752 as u32), ctx.r[3].u32 ) };
	// 832706A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832706AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832706B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832706B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832706B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832706B8 size=56
    let mut pc: u32 = 0x832706B8;
    'dispatch: loop {
        match pc {
            0x832706B8 => {
    //   block [0x832706B8..0x832706F0)
	// 832706B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832706BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832706C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832706C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832706C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832706CC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832706D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832706D4: 4AF83685  bl 0x821f3d58
	ctx.lr = 0x832706D8;
	sub_821F3D58(ctx, base);
	// 832706D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832706DC: 906ACE34  stw r3, -0x31cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12748 as u32), ctx.r[3].u32 ) };
	// 832706E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832706E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832706E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832706EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832706F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832706F0 size=56
    let mut pc: u32 = 0x832706F0;
    'dispatch: loop {
        match pc {
            0x832706F0 => {
    //   block [0x832706F0..0x83270728)
	// 832706F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832706F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832706F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832706FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270700: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270704: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83270708: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327070C: 4AF8364D  bl 0x821f3d58
	ctx.lr = 0x83270710;
	sub_821F3D58(ctx, base);
	// 83270710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270714: 906ACE38  stw r3, -0x31c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12744 as u32), ctx.r[3].u32 ) };
	// 83270718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327071C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270728 size=56
    let mut pc: u32 = 0x83270728;
    'dispatch: loop {
        match pc {
            0x83270728 => {
    //   block [0x83270728..0x83270760)
	// 83270728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270734: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270738: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327073C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83270740: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270744: 4AF83615  bl 0x821f3d58
	ctx.lr = 0x83270748;
	sub_821F3D58(ctx, base);
	// 83270748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327074C: 906ACE3C  stw r3, -0x31c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12740 as u32), ctx.r[3].u32 ) };
	// 83270750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327075C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270760 size=56
    let mut pc: u32 = 0x83270760;
    'dispatch: loop {
        match pc {
            0x83270760 => {
    //   block [0x83270760..0x83270798)
	// 83270760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327076C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270770: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270774: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83270778: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327077C: 4AF835DD  bl 0x821f3d58
	ctx.lr = 0x83270780;
	sub_821F3D58(ctx, base);
	// 83270780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270784: 906ACE40  stw r3, -0x31c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12736 as u32), ctx.r[3].u32 ) };
	// 83270788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270798 size=56
    let mut pc: u32 = 0x83270798;
    'dispatch: loop {
        match pc {
            0x83270798 => {
    //   block [0x83270798..0x832707D0)
	// 83270798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832707A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832707A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832707A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832707AC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832707B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832707B4: 4AF835A5  bl 0x821f3d58
	ctx.lr = 0x832707B8;
	sub_821F3D58(ctx, base);
	// 832707B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832707BC: 906ACE44  stw r3, -0x31bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12732 as u32), ctx.r[3].u32 ) };
	// 832707C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832707C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832707C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832707CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832707D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832707D0 size=56
    let mut pc: u32 = 0x832707D0;
    'dispatch: loop {
        match pc {
            0x832707D0 => {
    //   block [0x832707D0..0x83270808)
	// 832707D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832707D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832707D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832707DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832707E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832707E4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832707E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832707EC: 4AF8356D  bl 0x821f3d58
	ctx.lr = 0x832707F0;
	sub_821F3D58(ctx, base);
	// 832707F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832707F4: 906ACE48  stw r3, -0x31b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12728 as u32), ctx.r[3].u32 ) };
	// 832707F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832707FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270808 size=56
    let mut pc: u32 = 0x83270808;
    'dispatch: loop {
        match pc {
            0x83270808 => {
    //   block [0x83270808..0x83270840)
	// 83270808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270814: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270818: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327081C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83270820: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270824: 4AF83535  bl 0x821f3d58
	ctx.lr = 0x83270828;
	sub_821F3D58(ctx, base);
	// 83270828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327082C: 906ACE4C  stw r3, -0x31b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12724 as u32), ctx.r[3].u32 ) };
	// 83270830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327083C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270840 size=56
    let mut pc: u32 = 0x83270840;
    'dispatch: loop {
        match pc {
            0x83270840 => {
    //   block [0x83270840..0x83270878)
	// 83270840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327084C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270850: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270854: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83270858: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327085C: 4AF834FD  bl 0x821f3d58
	ctx.lr = 0x83270860;
	sub_821F3D58(ctx, base);
	// 83270860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270864: 906ACE50  stw r3, -0x31b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12720 as u32), ctx.r[3].u32 ) };
	// 83270868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270878 size=56
    let mut pc: u32 = 0x83270878;
    'dispatch: loop {
        match pc {
            0x83270878 => {
    //   block [0x83270878..0x832708B0)
	// 83270878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270888: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327088C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83270890: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270894: 4AF834C5  bl 0x821f3d58
	ctx.lr = 0x83270898;
	sub_821F3D58(ctx, base);
	// 83270898: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327089C: 906ACE54  stw r3, -0x31ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12716 as u32), ctx.r[3].u32 ) };
	// 832708A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832708A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832708A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832708AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832708B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832708B0 size=56
    let mut pc: u32 = 0x832708B0;
    'dispatch: loop {
        match pc {
            0x832708B0 => {
    //   block [0x832708B0..0x832708E8)
	// 832708B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832708B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832708B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832708BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832708C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832708C4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832708C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832708CC: 4AF8348D  bl 0x821f3d58
	ctx.lr = 0x832708D0;
	sub_821F3D58(ctx, base);
	// 832708D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832708D4: 906ACE58  stw r3, -0x31a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12712 as u32), ctx.r[3].u32 ) };
	// 832708D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832708DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832708E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832708E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832708E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832708E8 size=56
    let mut pc: u32 = 0x832708E8;
    'dispatch: loop {
        match pc {
            0x832708E8 => {
    //   block [0x832708E8..0x83270920)
	// 832708E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832708EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832708F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832708F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832708F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832708FC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83270900: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270904: 4AF83455  bl 0x821f3d58
	ctx.lr = 0x83270908;
	sub_821F3D58(ctx, base);
	// 83270908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327090C: 906ACE5C  stw r3, -0x31a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12708 as u32), ctx.r[3].u32 ) };
	// 83270910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327091C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270920 size=56
    let mut pc: u32 = 0x83270920;
    'dispatch: loop {
        match pc {
            0x83270920 => {
    //   block [0x83270920..0x83270958)
	// 83270920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327092C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270930: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270934: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83270938: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327093C: 4AF8341D  bl 0x821f3d58
	ctx.lr = 0x83270940;
	sub_821F3D58(ctx, base);
	// 83270940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270944: 906ACE60  stw r3, -0x31a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12704 as u32), ctx.r[3].u32 ) };
	// 83270948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327094C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270958 size=56
    let mut pc: u32 = 0x83270958;
    'dispatch: loop {
        match pc {
            0x83270958 => {
    //   block [0x83270958..0x83270990)
	// 83270958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327095C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270964: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270968: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327096C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83270970: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270974: 4AF833E5  bl 0x821f3d58
	ctx.lr = 0x83270978;
	sub_821F3D58(ctx, base);
	// 83270978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327097C: 906ACE64  stw r3, -0x319c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12700 as u32), ctx.r[3].u32 ) };
	// 83270980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327098C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270990 size=56
    let mut pc: u32 = 0x83270990;
    'dispatch: loop {
        match pc {
            0x83270990 => {
    //   block [0x83270990..0x832709C8)
	// 83270990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327099C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832709A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832709A4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832709A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832709AC: 4AF833AD  bl 0x821f3d58
	ctx.lr = 0x832709B0;
	sub_821F3D58(ctx, base);
	// 832709B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832709B4: 906ACE68  stw r3, -0x3198(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12696 as u32), ctx.r[3].u32 ) };
	// 832709B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832709BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832709C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832709C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832709C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832709C8 size=56
    let mut pc: u32 = 0x832709C8;
    'dispatch: loop {
        match pc {
            0x832709C8 => {
    //   block [0x832709C8..0x83270A00)
	// 832709C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832709CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832709D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832709D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832709D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832709DC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832709E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832709E4: 4AF83375  bl 0x821f3d58
	ctx.lr = 0x832709E8;
	sub_821F3D58(ctx, base);
	// 832709E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832709EC: 906ACE6C  stw r3, -0x3194(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12692 as u32), ctx.r[3].u32 ) };
	// 832709F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832709F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832709F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832709FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270A00 size=60
    let mut pc: u32 = 0x83270A00;
    'dispatch: loop {
        match pc {
            0x83270A00 => {
    //   block [0x83270A00..0x83270A3C)
	// 83270A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270A0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270A14: 388B2DEC  addi r4, r11, 0x2dec
	ctx.r[4].s64 = ctx.r[11].s64 + 11756;
	// 83270A18: 386ACE70  addi r3, r10, -0x3190
	ctx.r[3].s64 = ctx.r[10].s64 + -12688;
	// 83270A1C: 4B0659ED  bl 0x822d6408
	ctx.lr = 0x83270A20;
	sub_822D6408(ctx, base);
	// 83270A20: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270A24: 3869F090  addi r3, r9, -0xf70
	ctx.r[3].s64 = ctx.r[9].s64 + -3952;
	// 83270A28: 4BA394F9  bl 0x82ca9f20
	ctx.lr = 0x83270A2C;
	sub_82CA9F20(ctx, base);
	// 83270A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270A40 size=64
    let mut pc: u32 = 0x83270A40;
    'dispatch: loop {
        match pc {
            0x83270A40 => {
    //   block [0x83270A40..0x83270A80)
	// 83270A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270A4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270A54: 388BA080  addi r4, r11, -0x5f80
	ctx.r[4].s64 = ctx.r[11].s64 + -24448;
	// 83270A58: 386ACE74  addi r3, r10, -0x318c
	ctx.r[3].s64 = ctx.r[10].s64 + -12684;
	// 83270A5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270A60: 4AFBC471  bl 0x8222ced0
	ctx.lr = 0x83270A64;
	sub_8222CED0(ctx, base);
	// 83270A64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270A68: 3869F0A0  addi r3, r9, -0xf60
	ctx.r[3].s64 = ctx.r[9].s64 + -3936;
	// 83270A6C: 4BA394B5  bl 0x82ca9f20
	ctx.lr = 0x83270A70;
	sub_82CA9F20(ctx, base);
	// 83270A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270A80 size=64
    let mut pc: u32 = 0x83270A80;
    'dispatch: loop {
        match pc {
            0x83270A80 => {
    //   block [0x83270A80..0x83270AC0)
	// 83270A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270A8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270A94: 388BA098  addi r4, r11, -0x5f68
	ctx.r[4].s64 = ctx.r[11].s64 + -24424;
	// 83270A98: 386ACE78  addi r3, r10, -0x3188
	ctx.r[3].s64 = ctx.r[10].s64 + -12680;
	// 83270A9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270AA0: 4AFBC431  bl 0x8222ced0
	ctx.lr = 0x83270AA4;
	sub_8222CED0(ctx, base);
	// 83270AA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270AA8: 3869F0B0  addi r3, r9, -0xf50
	ctx.r[3].s64 = ctx.r[9].s64 + -3920;
	// 83270AAC: 4BA39475  bl 0x82ca9f20
	ctx.lr = 0x83270AB0;
	sub_82CA9F20(ctx, base);
	// 83270AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270AC0 size=64
    let mut pc: u32 = 0x83270AC0;
    'dispatch: loop {
        match pc {
            0x83270AC0 => {
    //   block [0x83270AC0..0x83270B00)
	// 83270AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270ACC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270AD4: 388BA0C0  addi r4, r11, -0x5f40
	ctx.r[4].s64 = ctx.r[11].s64 + -24384;
	// 83270AD8: 386ACE7C  addi r3, r10, -0x3184
	ctx.r[3].s64 = ctx.r[10].s64 + -12676;
	// 83270ADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270AE0: 4AFBC3F1  bl 0x8222ced0
	ctx.lr = 0x83270AE4;
	sub_8222CED0(ctx, base);
	// 83270AE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270AE8: 3869F0C0  addi r3, r9, -0xf40
	ctx.r[3].s64 = ctx.r[9].s64 + -3904;
	// 83270AEC: 4BA39435  bl 0x82ca9f20
	ctx.lr = 0x83270AF0;
	sub_82CA9F20(ctx, base);
	// 83270AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270B00 size=64
    let mut pc: u32 = 0x83270B00;
    'dispatch: loop {
        match pc {
            0x83270B00 => {
    //   block [0x83270B00..0x83270B40)
	// 83270B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270B0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270B14: 388BA0EC  addi r4, r11, -0x5f14
	ctx.r[4].s64 = ctx.r[11].s64 + -24340;
	// 83270B18: 386ACE80  addi r3, r10, -0x3180
	ctx.r[3].s64 = ctx.r[10].s64 + -12672;
	// 83270B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270B20: 4AFBC3B1  bl 0x8222ced0
	ctx.lr = 0x83270B24;
	sub_8222CED0(ctx, base);
	// 83270B24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270B28: 3869F0D0  addi r3, r9, -0xf30
	ctx.r[3].s64 = ctx.r[9].s64 + -3888;
	// 83270B2C: 4BA393F5  bl 0x82ca9f20
	ctx.lr = 0x83270B30;
	sub_82CA9F20(ctx, base);
	// 83270B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270B40 size=64
    let mut pc: u32 = 0x83270B40;
    'dispatch: loop {
        match pc {
            0x83270B40 => {
    //   block [0x83270B40..0x83270B80)
	// 83270B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270B4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270B54: 388BA124  addi r4, r11, -0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + -24284;
	// 83270B58: 386ACE84  addi r3, r10, -0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + -12668;
	// 83270B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270B60: 4AFBC371  bl 0x8222ced0
	ctx.lr = 0x83270B64;
	sub_8222CED0(ctx, base);
	// 83270B64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270B68: 3869F0E0  addi r3, r9, -0xf20
	ctx.r[3].s64 = ctx.r[9].s64 + -3872;
	// 83270B6C: 4BA393B5  bl 0x82ca9f20
	ctx.lr = 0x83270B70;
	sub_82CA9F20(ctx, base);
	// 83270B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270B80 size=64
    let mut pc: u32 = 0x83270B80;
    'dispatch: loop {
        match pc {
            0x83270B80 => {
    //   block [0x83270B80..0x83270BC0)
	// 83270B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270B8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270B94: 388BA158  addi r4, r11, -0x5ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -24232;
	// 83270B98: 386ACE88  addi r3, r10, -0x3178
	ctx.r[3].s64 = ctx.r[10].s64 + -12664;
	// 83270B9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270BA0: 4AFBC331  bl 0x8222ced0
	ctx.lr = 0x83270BA4;
	sub_8222CED0(ctx, base);
	// 83270BA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270BA8: 3869F0F0  addi r3, r9, -0xf10
	ctx.r[3].s64 = ctx.r[9].s64 + -3856;
	// 83270BAC: 4BA39375  bl 0x82ca9f20
	ctx.lr = 0x83270BB0;
	sub_82CA9F20(ctx, base);
	// 83270BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270BC0 size=64
    let mut pc: u32 = 0x83270BC0;
    'dispatch: loop {
        match pc {
            0x83270BC0 => {
    //   block [0x83270BC0..0x83270C00)
	// 83270BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270BCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270BD4: 388BA198  addi r4, r11, -0x5e68
	ctx.r[4].s64 = ctx.r[11].s64 + -24168;
	// 83270BD8: 386ACE8C  addi r3, r10, -0x3174
	ctx.r[3].s64 = ctx.r[10].s64 + -12660;
	// 83270BDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270BE0: 4AFBC2F1  bl 0x8222ced0
	ctx.lr = 0x83270BE4;
	sub_8222CED0(ctx, base);
	// 83270BE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270BE8: 3869F100  addi r3, r9, -0xf00
	ctx.r[3].s64 = ctx.r[9].s64 + -3840;
	// 83270BEC: 4BA39335  bl 0x82ca9f20
	ctx.lr = 0x83270BF0;
	sub_82CA9F20(ctx, base);
	// 83270BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270C00 size=64
    let mut pc: u32 = 0x83270C00;
    'dispatch: loop {
        match pc {
            0x83270C00 => {
    //   block [0x83270C00..0x83270C40)
	// 83270C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270C0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270C14: 388BA1D8  addi r4, r11, -0x5e28
	ctx.r[4].s64 = ctx.r[11].s64 + -24104;
	// 83270C18: 386ACE90  addi r3, r10, -0x3170
	ctx.r[3].s64 = ctx.r[10].s64 + -12656;
	// 83270C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270C20: 4AFBC2B1  bl 0x8222ced0
	ctx.lr = 0x83270C24;
	sub_8222CED0(ctx, base);
	// 83270C24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270C28: 3869F110  addi r3, r9, -0xef0
	ctx.r[3].s64 = ctx.r[9].s64 + -3824;
	// 83270C2C: 4BA392F5  bl 0x82ca9f20
	ctx.lr = 0x83270C30;
	sub_82CA9F20(ctx, base);
	// 83270C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270C40 size=64
    let mut pc: u32 = 0x83270C40;
    'dispatch: loop {
        match pc {
            0x83270C40 => {
    //   block [0x83270C40..0x83270C80)
	// 83270C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270C4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270C50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270C54: 388BA210  addi r4, r11, -0x5df0
	ctx.r[4].s64 = ctx.r[11].s64 + -24048;
	// 83270C58: 386ACE94  addi r3, r10, -0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + -12652;
	// 83270C5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270C60: 4AFBC271  bl 0x8222ced0
	ctx.lr = 0x83270C64;
	sub_8222CED0(ctx, base);
	// 83270C64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270C68: 3869F120  addi r3, r9, -0xee0
	ctx.r[3].s64 = ctx.r[9].s64 + -3808;
	// 83270C6C: 4BA392B5  bl 0x82ca9f20
	ctx.lr = 0x83270C70;
	sub_82CA9F20(ctx, base);
	// 83270C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270C80 size=64
    let mut pc: u32 = 0x83270C80;
    'dispatch: loop {
        match pc {
            0x83270C80 => {
    //   block [0x83270C80..0x83270CC0)
	// 83270C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270C8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270C90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270C94: 388BA258  addi r4, r11, -0x5da8
	ctx.r[4].s64 = ctx.r[11].s64 + -23976;
	// 83270C98: 386ACE98  addi r3, r10, -0x3168
	ctx.r[3].s64 = ctx.r[10].s64 + -12648;
	// 83270C9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270CA0: 4AFBC231  bl 0x8222ced0
	ctx.lr = 0x83270CA4;
	sub_8222CED0(ctx, base);
	// 83270CA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270CA8: 3869F130  addi r3, r9, -0xed0
	ctx.r[3].s64 = ctx.r[9].s64 + -3792;
	// 83270CAC: 4BA39275  bl 0x82ca9f20
	ctx.lr = 0x83270CB0;
	sub_82CA9F20(ctx, base);
	// 83270CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270CC0 size=64
    let mut pc: u32 = 0x83270CC0;
    'dispatch: loop {
        match pc {
            0x83270CC0 => {
    //   block [0x83270CC0..0x83270D00)
	// 83270CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270CCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270CD4: 388BA2A0  addi r4, r11, -0x5d60
	ctx.r[4].s64 = ctx.r[11].s64 + -23904;
	// 83270CD8: 386ACE9C  addi r3, r10, -0x3164
	ctx.r[3].s64 = ctx.r[10].s64 + -12644;
	// 83270CDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270CE0: 4AFBC1F1  bl 0x8222ced0
	ctx.lr = 0x83270CE4;
	sub_8222CED0(ctx, base);
	// 83270CE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270CE8: 3869F140  addi r3, r9, -0xec0
	ctx.r[3].s64 = ctx.r[9].s64 + -3776;
	// 83270CEC: 4BA39235  bl 0x82ca9f20
	ctx.lr = 0x83270CF0;
	sub_82CA9F20(ctx, base);
	// 83270CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270D00 size=64
    let mut pc: u32 = 0x83270D00;
    'dispatch: loop {
        match pc {
            0x83270D00 => {
    //   block [0x83270D00..0x83270D40)
	// 83270D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270D0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270D14: 388BA2E4  addi r4, r11, -0x5d1c
	ctx.r[4].s64 = ctx.r[11].s64 + -23836;
	// 83270D18: 386ACEA0  addi r3, r10, -0x3160
	ctx.r[3].s64 = ctx.r[10].s64 + -12640;
	// 83270D1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270D20: 4AFBC1B1  bl 0x8222ced0
	ctx.lr = 0x83270D24;
	sub_8222CED0(ctx, base);
	// 83270D24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270D28: 3869F150  addi r3, r9, -0xeb0
	ctx.r[3].s64 = ctx.r[9].s64 + -3760;
	// 83270D2C: 4BA391F5  bl 0x82ca9f20
	ctx.lr = 0x83270D30;
	sub_82CA9F20(ctx, base);
	// 83270D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270D40 size=64
    let mut pc: u32 = 0x83270D40;
    'dispatch: loop {
        match pc {
            0x83270D40 => {
    //   block [0x83270D40..0x83270D80)
	// 83270D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270D4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270D50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270D54: 388BA318  addi r4, r11, -0x5ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -23784;
	// 83270D58: 386ACEA4  addi r3, r10, -0x315c
	ctx.r[3].s64 = ctx.r[10].s64 + -12636;
	// 83270D5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270D60: 4AFBC171  bl 0x8222ced0
	ctx.lr = 0x83270D64;
	sub_8222CED0(ctx, base);
	// 83270D64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270D68: 3869F160  addi r3, r9, -0xea0
	ctx.r[3].s64 = ctx.r[9].s64 + -3744;
	// 83270D6C: 4BA391B5  bl 0x82ca9f20
	ctx.lr = 0x83270D70;
	sub_82CA9F20(ctx, base);
	// 83270D70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270D80 size=64
    let mut pc: u32 = 0x83270D80;
    'dispatch: loop {
        match pc {
            0x83270D80 => {
    //   block [0x83270D80..0x83270DC0)
	// 83270D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270D8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270D94: 388BA350  addi r4, r11, -0x5cb0
	ctx.r[4].s64 = ctx.r[11].s64 + -23728;
	// 83270D98: 386ACEA8  addi r3, r10, -0x3158
	ctx.r[3].s64 = ctx.r[10].s64 + -12632;
	// 83270D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270DA0: 4AFBC131  bl 0x8222ced0
	ctx.lr = 0x83270DA4;
	sub_8222CED0(ctx, base);
	// 83270DA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270DA8: 3869F170  addi r3, r9, -0xe90
	ctx.r[3].s64 = ctx.r[9].s64 + -3728;
	// 83270DAC: 4BA39175  bl 0x82ca9f20
	ctx.lr = 0x83270DB0;
	sub_82CA9F20(ctx, base);
	// 83270DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270DC0 size=64
    let mut pc: u32 = 0x83270DC0;
    'dispatch: loop {
        match pc {
            0x83270DC0 => {
    //   block [0x83270DC0..0x83270E00)
	// 83270DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270DCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270DD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270DD4: 388BA390  addi r4, r11, -0x5c70
	ctx.r[4].s64 = ctx.r[11].s64 + -23664;
	// 83270DD8: 386ACEAC  addi r3, r10, -0x3154
	ctx.r[3].s64 = ctx.r[10].s64 + -12628;
	// 83270DDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270DE0: 4AFBC0F1  bl 0x8222ced0
	ctx.lr = 0x83270DE4;
	sub_8222CED0(ctx, base);
	// 83270DE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270DE8: 3869F180  addi r3, r9, -0xe80
	ctx.r[3].s64 = ctx.r[9].s64 + -3712;
	// 83270DEC: 4BA39135  bl 0x82ca9f20
	ctx.lr = 0x83270DF0;
	sub_82CA9F20(ctx, base);
	// 83270DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270E00 size=64
    let mut pc: u32 = 0x83270E00;
    'dispatch: loop {
        match pc {
            0x83270E00 => {
    //   block [0x83270E00..0x83270E40)
	// 83270E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270E0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270E10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270E14: 388BA3D4  addi r4, r11, -0x5c2c
	ctx.r[4].s64 = ctx.r[11].s64 + -23596;
	// 83270E18: 386ACEB0  addi r3, r10, -0x3150
	ctx.r[3].s64 = ctx.r[10].s64 + -12624;
	// 83270E1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270E20: 4AFBC0B1  bl 0x8222ced0
	ctx.lr = 0x83270E24;
	sub_8222CED0(ctx, base);
	// 83270E24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270E28: 3869F190  addi r3, r9, -0xe70
	ctx.r[3].s64 = ctx.r[9].s64 + -3696;
	// 83270E2C: 4BA390F5  bl 0x82ca9f20
	ctx.lr = 0x83270E30;
	sub_82CA9F20(ctx, base);
	// 83270E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270E40 size=64
    let mut pc: u32 = 0x83270E40;
    'dispatch: loop {
        match pc {
            0x83270E40 => {
    //   block [0x83270E40..0x83270E80)
	// 83270E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270E4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270E54: 388BA408  addi r4, r11, -0x5bf8
	ctx.r[4].s64 = ctx.r[11].s64 + -23544;
	// 83270E58: 386ACEB4  addi r3, r10, -0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + -12620;
	// 83270E5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270E60: 4AFBC071  bl 0x8222ced0
	ctx.lr = 0x83270E64;
	sub_8222CED0(ctx, base);
	// 83270E64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270E68: 3869F1A0  addi r3, r9, -0xe60
	ctx.r[3].s64 = ctx.r[9].s64 + -3680;
	// 83270E6C: 4BA390B5  bl 0x82ca9f20
	ctx.lr = 0x83270E70;
	sub_82CA9F20(ctx, base);
	// 83270E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270E80 size=64
    let mut pc: u32 = 0x83270E80;
    'dispatch: loop {
        match pc {
            0x83270E80 => {
    //   block [0x83270E80..0x83270EC0)
	// 83270E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270E8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270E94: 388BA458  addi r4, r11, -0x5ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -23464;
	// 83270E98: 386ACEB8  addi r3, r10, -0x3148
	ctx.r[3].s64 = ctx.r[10].s64 + -12616;
	// 83270E9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270EA0: 4AFBC031  bl 0x8222ced0
	ctx.lr = 0x83270EA4;
	sub_8222CED0(ctx, base);
	// 83270EA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270EA8: 3869F1B0  addi r3, r9, -0xe50
	ctx.r[3].s64 = ctx.r[9].s64 + -3664;
	// 83270EAC: 4BA39075  bl 0x82ca9f20
	ctx.lr = 0x83270EB0;
	sub_82CA9F20(ctx, base);
	// 83270EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270EC0 size=64
    let mut pc: u32 = 0x83270EC0;
    'dispatch: loop {
        match pc {
            0x83270EC0 => {
    //   block [0x83270EC0..0x83270F00)
	// 83270EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270ECC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270ED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270ED4: 388BA490  addi r4, r11, -0x5b70
	ctx.r[4].s64 = ctx.r[11].s64 + -23408;
	// 83270ED8: 386ACEBC  addi r3, r10, -0x3144
	ctx.r[3].s64 = ctx.r[10].s64 + -12612;
	// 83270EDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270EE0: 4AFBBFF1  bl 0x8222ced0
	ctx.lr = 0x83270EE4;
	sub_8222CED0(ctx, base);
	// 83270EE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270EE8: 3869F1C0  addi r3, r9, -0xe40
	ctx.r[3].s64 = ctx.r[9].s64 + -3648;
	// 83270EEC: 4BA39035  bl 0x82ca9f20
	ctx.lr = 0x83270EF0;
	sub_82CA9F20(ctx, base);
	// 83270EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270F00 size=64
    let mut pc: u32 = 0x83270F00;
    'dispatch: loop {
        match pc {
            0x83270F00 => {
    //   block [0x83270F00..0x83270F40)
	// 83270F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270F0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270F10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270F14: 388BA4E0  addi r4, r11, -0x5b20
	ctx.r[4].s64 = ctx.r[11].s64 + -23328;
	// 83270F18: 386ACEC0  addi r3, r10, -0x3140
	ctx.r[3].s64 = ctx.r[10].s64 + -12608;
	// 83270F1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270F20: 4AFBBFB1  bl 0x8222ced0
	ctx.lr = 0x83270F24;
	sub_8222CED0(ctx, base);
	// 83270F24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270F28: 3869F1D0  addi r3, r9, -0xe30
	ctx.r[3].s64 = ctx.r[9].s64 + -3632;
	// 83270F2C: 4BA38FF5  bl 0x82ca9f20
	ctx.lr = 0x83270F30;
	sub_82CA9F20(ctx, base);
	// 83270F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270F40 size=64
    let mut pc: u32 = 0x83270F40;
    'dispatch: loop {
        match pc {
            0x83270F40 => {
    //   block [0x83270F40..0x83270F80)
	// 83270F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270F4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270F50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270F54: 388BA50C  addi r4, r11, -0x5af4
	ctx.r[4].s64 = ctx.r[11].s64 + -23284;
	// 83270F58: 386ACEC4  addi r3, r10, -0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + -12604;
	// 83270F5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270F60: 4AFBBF71  bl 0x8222ced0
	ctx.lr = 0x83270F64;
	sub_8222CED0(ctx, base);
	// 83270F64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270F68: 3869F1E0  addi r3, r9, -0xe20
	ctx.r[3].s64 = ctx.r[9].s64 + -3616;
	// 83270F6C: 4BA38FB5  bl 0x82ca9f20
	ctx.lr = 0x83270F70;
	sub_82CA9F20(ctx, base);
	// 83270F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270F80 size=56
    let mut pc: u32 = 0x83270F80;
    'dispatch: loop {
        match pc {
            0x83270F80 => {
    //   block [0x83270F80..0x83270FB8)
	// 83270F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270F8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270F90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270F94: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83270F98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270F9C: 4AF82DBD  bl 0x821f3d58
	ctx.lr = 0x83270FA0;
	sub_821F3D58(ctx, base);
	// 83270FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270FA4: 906ACEC8  stw r3, -0x3138(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12600 as u32), ctx.r[3].u32 ) };
	// 83270FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270FB8 size=56
    let mut pc: u32 = 0x83270FB8;
    'dispatch: loop {
        match pc {
            0x83270FB8 => {
    //   block [0x83270FB8..0x83270FF0)
	// 83270FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270FC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270FC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270FCC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83270FD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270FD4: 4AF82D85  bl 0x821f3d58
	ctx.lr = 0x83270FD8;
	sub_821F3D58(ctx, base);
	// 83270FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270FDC: 906ACECC  stw r3, -0x3134(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12596 as u32), ctx.r[3].u32 ) };
	// 83270FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270FF0 size=56
    let mut pc: u32 = 0x83270FF0;
    'dispatch: loop {
        match pc {
            0x83270FF0 => {
    //   block [0x83270FF0..0x83271028)
	// 83270FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271000: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271004: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83271008: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327100C: 4AF82D4D  bl 0x821f3d58
	ctx.lr = 0x83271010;
	sub_821F3D58(ctx, base);
	// 83271010: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271014: 906ACED0  stw r3, -0x3130(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12592 as u32), ctx.r[3].u32 ) };
	// 83271018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327101C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271028 size=56
    let mut pc: u32 = 0x83271028;
    'dispatch: loop {
        match pc {
            0x83271028 => {
    //   block [0x83271028..0x83271060)
	// 83271028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327102C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271034: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271038: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327103C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83271040: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271044: 4AF82D15  bl 0x821f3d58
	ctx.lr = 0x83271048;
	sub_821F3D58(ctx, base);
	// 83271048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327104C: 906ACED4  stw r3, -0x312c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12588 as u32), ctx.r[3].u32 ) };
	// 83271050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271060 size=56
    let mut pc: u32 = 0x83271060;
    'dispatch: loop {
        match pc {
            0x83271060 => {
    //   block [0x83271060..0x83271098)
	// 83271060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327106C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271070: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271074: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83271078: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327107C: 4AF82CDD  bl 0x821f3d58
	ctx.lr = 0x83271080;
	sub_821F3D58(ctx, base);
	// 83271080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271084: 906ACED8  stw r3, -0x3128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12584 as u32), ctx.r[3].u32 ) };
	// 83271088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327108C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271098 size=56
    let mut pc: u32 = 0x83271098;
    'dispatch: loop {
        match pc {
            0x83271098 => {
    //   block [0x83271098..0x832710D0)
	// 83271098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327109C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832710A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832710A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832710A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832710AC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832710B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832710B4: 4AF82CA5  bl 0x821f3d58
	ctx.lr = 0x832710B8;
	sub_821F3D58(ctx, base);
	// 832710B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832710BC: 906ACEDC  stw r3, -0x3124(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12580 as u32), ctx.r[3].u32 ) };
	// 832710C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832710C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832710C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832710CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832710D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832710D0 size=56
    let mut pc: u32 = 0x832710D0;
    'dispatch: loop {
        match pc {
            0x832710D0 => {
    //   block [0x832710D0..0x83271108)
	// 832710D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832710D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832710D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832710DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832710E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832710E4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832710E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832710EC: 4AF82C6D  bl 0x821f3d58
	ctx.lr = 0x832710F0;
	sub_821F3D58(ctx, base);
	// 832710F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832710F4: 906ACEE0  stw r3, -0x3120(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12576 as u32), ctx.r[3].u32 ) };
	// 832710F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832710FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271108 size=56
    let mut pc: u32 = 0x83271108;
    'dispatch: loop {
        match pc {
            0x83271108 => {
    //   block [0x83271108..0x83271140)
	// 83271108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271114: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271118: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327111C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83271120: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271124: 4AF82C35  bl 0x821f3d58
	ctx.lr = 0x83271128;
	sub_821F3D58(ctx, base);
	// 83271128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327112C: 906ACEE4  stw r3, -0x311c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12572 as u32), ctx.r[3].u32 ) };
	// 83271130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327113C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271140 size=56
    let mut pc: u32 = 0x83271140;
    'dispatch: loop {
        match pc {
            0x83271140 => {
    //   block [0x83271140..0x83271178)
	// 83271140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327114C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271150: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271154: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83271158: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327115C: 4AF82BFD  bl 0x821f3d58
	ctx.lr = 0x83271160;
	sub_821F3D58(ctx, base);
	// 83271160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271164: 906ACEE8  stw r3, -0x3118(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12568 as u32), ctx.r[3].u32 ) };
	// 83271168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327116C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271178 size=56
    let mut pc: u32 = 0x83271178;
    'dispatch: loop {
        match pc {
            0x83271178 => {
    //   block [0x83271178..0x832711B0)
	// 83271178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327117C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327118C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83271190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271194: 4AF82BC5  bl 0x821f3d58
	ctx.lr = 0x83271198;
	sub_821F3D58(ctx, base);
	// 83271198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327119C: 906ACEEC  stw r3, -0x3114(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12564 as u32), ctx.r[3].u32 ) };
	// 832711A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832711A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832711A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832711AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832711B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832711B0 size=56
    let mut pc: u32 = 0x832711B0;
    'dispatch: loop {
        match pc {
            0x832711B0 => {
    //   block [0x832711B0..0x832711E8)
	// 832711B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832711B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832711B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832711BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832711C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832711C4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832711C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832711CC: 4AF82B8D  bl 0x821f3d58
	ctx.lr = 0x832711D0;
	sub_821F3D58(ctx, base);
	// 832711D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832711D4: 906ACEF0  stw r3, -0x3110(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12560 as u32), ctx.r[3].u32 ) };
	// 832711D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832711DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832711E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832711E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832711E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832711E8 size=56
    let mut pc: u32 = 0x832711E8;
    'dispatch: loop {
        match pc {
            0x832711E8 => {
    //   block [0x832711E8..0x83271220)
	// 832711E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832711EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832711F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832711F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832711F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832711FC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83271200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271204: 4AF82B55  bl 0x821f3d58
	ctx.lr = 0x83271208;
	sub_821F3D58(ctx, base);
	// 83271208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327120C: 906ACEF4  stw r3, -0x310c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12556 as u32), ctx.r[3].u32 ) };
	// 83271210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271220 size=56
    let mut pc: u32 = 0x83271220;
    'dispatch: loop {
        match pc {
            0x83271220 => {
    //   block [0x83271220..0x83271258)
	// 83271220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327122C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271234: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83271238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327123C: 4AF82B1D  bl 0x821f3d58
	ctx.lr = 0x83271240;
	sub_821F3D58(ctx, base);
	// 83271240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271244: 906ACEF8  stw r3, -0x3108(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12552 as u32), ctx.r[3].u32 ) };
	// 83271248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327124C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271258 size=56
    let mut pc: u32 = 0x83271258;
    'dispatch: loop {
        match pc {
            0x83271258 => {
    //   block [0x83271258..0x83271290)
	// 83271258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327125C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327126C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83271270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271274: 4AF82AE5  bl 0x821f3d58
	ctx.lr = 0x83271278;
	sub_821F3D58(ctx, base);
	// 83271278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327127C: 906ACEFC  stw r3, -0x3104(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12548 as u32), ctx.r[3].u32 ) };
	// 83271280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327128C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271290 size=56
    let mut pc: u32 = 0x83271290;
    'dispatch: loop {
        match pc {
            0x83271290 => {
    //   block [0x83271290..0x832712C8)
	// 83271290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327129C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832712A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832712A4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832712A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832712AC: 4AF82AAD  bl 0x821f3d58
	ctx.lr = 0x832712B0;
	sub_821F3D58(ctx, base);
	// 832712B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832712B4: 906ACF00  stw r3, -0x3100(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12544 as u32), ctx.r[3].u32 ) };
	// 832712B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832712BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832712C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832712C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832712C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832712C8 size=56
    let mut pc: u32 = 0x832712C8;
    'dispatch: loop {
        match pc {
            0x832712C8 => {
    //   block [0x832712C8..0x83271300)
	// 832712C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832712CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832712D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832712D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832712D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832712DC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832712E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832712E4: 4AF82A75  bl 0x821f3d58
	ctx.lr = 0x832712E8;
	sub_821F3D58(ctx, base);
	// 832712E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832712EC: 906ACF04  stw r3, -0x30fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12540 as u32), ctx.r[3].u32 ) };
	// 832712F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832712F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832712F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832712FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271300 size=56
    let mut pc: u32 = 0x83271300;
    'dispatch: loop {
        match pc {
            0x83271300 => {
    //   block [0x83271300..0x83271338)
	// 83271300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327130C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271314: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83271318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327131C: 4AF82A3D  bl 0x821f3d58
	ctx.lr = 0x83271320;
	sub_821F3D58(ctx, base);
	// 83271320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271324: 906ACF08  stw r3, -0x30f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12536 as u32), ctx.r[3].u32 ) };
	// 83271328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327132C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271338 size=56
    let mut pc: u32 = 0x83271338;
    'dispatch: loop {
        match pc {
            0x83271338 => {
    //   block [0x83271338..0x83271370)
	// 83271338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327133C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327134C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83271350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271354: 4AF82A05  bl 0x821f3d58
	ctx.lr = 0x83271358;
	sub_821F3D58(ctx, base);
	// 83271358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327135C: 906ACF0C  stw r3, -0x30f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12532 as u32), ctx.r[3].u32 ) };
	// 83271360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327136C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271370 size=56
    let mut pc: u32 = 0x83271370;
    'dispatch: loop {
        match pc {
            0x83271370 => {
    //   block [0x83271370..0x832713A8)
	// 83271370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327137C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271384: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83271388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327138C: 4AF829CD  bl 0x821f3d58
	ctx.lr = 0x83271390;
	sub_821F3D58(ctx, base);
	// 83271390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271394: 906ACF10  stw r3, -0x30f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12528 as u32), ctx.r[3].u32 ) };
	// 83271398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327139C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832713A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832713A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832713A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832713A8 size=56
    let mut pc: u32 = 0x832713A8;
    'dispatch: loop {
        match pc {
            0x832713A8 => {
    //   block [0x832713A8..0x832713E0)
	// 832713A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832713AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832713B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832713B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832713B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832713BC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832713C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832713C4: 4AF82995  bl 0x821f3d58
	ctx.lr = 0x832713C8;
	sub_821F3D58(ctx, base);
	// 832713C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832713CC: 906ACF14  stw r3, -0x30ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12524 as u32), ctx.r[3].u32 ) };
	// 832713D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832713D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832713D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832713DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832713E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832713E0 size=56
    let mut pc: u32 = 0x832713E0;
    'dispatch: loop {
        match pc {
            0x832713E0 => {
    //   block [0x832713E0..0x83271418)
	// 832713E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832713E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832713E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832713EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832713F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832713F4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832713F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832713FC: 4AF8295D  bl 0x821f3d58
	ctx.lr = 0x83271400;
	sub_821F3D58(ctx, base);
	// 83271400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271404: 906ACF18  stw r3, -0x30e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12520 as u32), ctx.r[3].u32 ) };
	// 83271408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327140C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271418 size=64
    let mut pc: u32 = 0x83271418;
    'dispatch: loop {
        match pc {
            0x83271418 => {
    //   block [0x83271418..0x83271458)
	// 83271418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327141C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327142C: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 83271430: 386ACF1C  addi r3, r10, -0x30e4
	ctx.r[3].s64 = ctx.r[10].s64 + -12516;
	// 83271434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271438: 4AFBBA99  bl 0x8222ced0
	ctx.lr = 0x8327143C;
	sub_8222CED0(ctx, base);
	// 8327143C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271440: 3869F1F0  addi r3, r9, -0xe10
	ctx.r[3].s64 = ctx.r[9].s64 + -3600;
	// 83271444: 4BA38ADD  bl 0x82ca9f20
	ctx.lr = 0x83271448;
	sub_82CA9F20(ctx, base);
	// 83271448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271458 size=64
    let mut pc: u32 = 0x83271458;
    'dispatch: loop {
        match pc {
            0x83271458 => {
    //   block [0x83271458..0x83271498)
	// 83271458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327146C: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 83271470: 386ACF20  addi r3, r10, -0x30e0
	ctx.r[3].s64 = ctx.r[10].s64 + -12512;
	// 83271474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271478: 4AFBBA59  bl 0x8222ced0
	ctx.lr = 0x8327147C;
	sub_8222CED0(ctx, base);
	// 8327147C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271480: 3869F200  addi r3, r9, -0xe00
	ctx.r[3].s64 = ctx.r[9].s64 + -3584;
	// 83271484: 4BA38A9D  bl 0x82ca9f20
	ctx.lr = 0x83271488;
	sub_82CA9F20(ctx, base);
	// 83271488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327148C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271498 size=64
    let mut pc: u32 = 0x83271498;
    'dispatch: loop {
        match pc {
            0x83271498 => {
    //   block [0x83271498..0x832714D8)
	// 83271498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327149C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832714A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832714A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832714A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832714AC: 388BA6F4  addi r4, r11, -0x590c
	ctx.r[4].s64 = ctx.r[11].s64 + -22796;
	// 832714B0: 386ACF24  addi r3, r10, -0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12508;
	// 832714B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832714B8: 4AFBBA19  bl 0x8222ced0
	ctx.lr = 0x832714BC;
	sub_8222CED0(ctx, base);
	// 832714BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832714C0: 3869F210  addi r3, r9, -0xdf0
	ctx.r[3].s64 = ctx.r[9].s64 + -3568;
	// 832714C4: 4BA38A5D  bl 0x82ca9f20
	ctx.lr = 0x832714C8;
	sub_82CA9F20(ctx, base);
	// 832714C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832714CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832714D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832714D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832714D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832714D8 size=64
    let mut pc: u32 = 0x832714D8;
    'dispatch: loop {
        match pc {
            0x832714D8 => {
    //   block [0x832714D8..0x83271518)
	// 832714D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832714DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832714E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832714E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832714E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832714EC: 388BA724  addi r4, r11, -0x58dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22748;
	// 832714F0: 386ACF28  addi r3, r10, -0x30d8
	ctx.r[3].s64 = ctx.r[10].s64 + -12504;
	// 832714F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832714F8: 4AFBB9D9  bl 0x8222ced0
	ctx.lr = 0x832714FC;
	sub_8222CED0(ctx, base);
	// 832714FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271500: 3869F220  addi r3, r9, -0xde0
	ctx.r[3].s64 = ctx.r[9].s64 + -3552;
	// 83271504: 4BA38A1D  bl 0x82ca9f20
	ctx.lr = 0x83271508;
	sub_82CA9F20(ctx, base);
	// 83271508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271518 size=64
    let mut pc: u32 = 0x83271518;
    'dispatch: loop {
        match pc {
            0x83271518 => {
    //   block [0x83271518..0x83271558)
	// 83271518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271524: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327152C: 388BA760  addi r4, r11, -0x58a0
	ctx.r[4].s64 = ctx.r[11].s64 + -22688;
	// 83271530: 386ACF2C  addi r3, r10, -0x30d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12500;
	// 83271534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271538: 4AFBB999  bl 0x8222ced0
	ctx.lr = 0x8327153C;
	sub_8222CED0(ctx, base);
	// 8327153C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271540: 3869F230  addi r3, r9, -0xdd0
	ctx.r[3].s64 = ctx.r[9].s64 + -3536;
	// 83271544: 4BA389DD  bl 0x82ca9f20
	ctx.lr = 0x83271548;
	sub_82CA9F20(ctx, base);
	// 83271548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327154C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271558 size=64
    let mut pc: u32 = 0x83271558;
    'dispatch: loop {
        match pc {
            0x83271558 => {
    //   block [0x83271558..0x83271598)
	// 83271558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327156C: 388BA7A0  addi r4, r11, -0x5860
	ctx.r[4].s64 = ctx.r[11].s64 + -22624;
	// 83271570: 386ACF30  addi r3, r10, -0x30d0
	ctx.r[3].s64 = ctx.r[10].s64 + -12496;
	// 83271574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271578: 4AFBB959  bl 0x8222ced0
	ctx.lr = 0x8327157C;
	sub_8222CED0(ctx, base);
	// 8327157C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271580: 3869F240  addi r3, r9, -0xdc0
	ctx.r[3].s64 = ctx.r[9].s64 + -3520;
	// 83271584: 4BA3899D  bl 0x82ca9f20
	ctx.lr = 0x83271588;
	sub_82CA9F20(ctx, base);
	// 83271588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327158C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271598 size=64
    let mut pc: u32 = 0x83271598;
    'dispatch: loop {
        match pc {
            0x83271598 => {
    //   block [0x83271598..0x832715D8)
	// 83271598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832715A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832715A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832715A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832715AC: 388BA7E8  addi r4, r11, -0x5818
	ctx.r[4].s64 = ctx.r[11].s64 + -22552;
	// 832715B0: 386ACF34  addi r3, r10, -0x30cc
	ctx.r[3].s64 = ctx.r[10].s64 + -12492;
	// 832715B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832715B8: 4AFBB919  bl 0x8222ced0
	ctx.lr = 0x832715BC;
	sub_8222CED0(ctx, base);
	// 832715BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832715C0: 3869F250  addi r3, r9, -0xdb0
	ctx.r[3].s64 = ctx.r[9].s64 + -3504;
	// 832715C4: 4BA3895D  bl 0x82ca9f20
	ctx.lr = 0x832715C8;
	sub_82CA9F20(ctx, base);
	// 832715C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832715CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832715D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832715D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832715D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832715D8 size=64
    let mut pc: u32 = 0x832715D8;
    'dispatch: loop {
        match pc {
            0x832715D8 => {
    //   block [0x832715D8..0x83271618)
	// 832715D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832715DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832715E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832715E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832715E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832715EC: 388BA830  addi r4, r11, -0x57d0
	ctx.r[4].s64 = ctx.r[11].s64 + -22480;
	// 832715F0: 386ACF38  addi r3, r10, -0x30c8
	ctx.r[3].s64 = ctx.r[10].s64 + -12488;
	// 832715F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832715F8: 4AFBB8D9  bl 0x8222ced0
	ctx.lr = 0x832715FC;
	sub_8222CED0(ctx, base);
	// 832715FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271600: 3869F260  addi r3, r9, -0xda0
	ctx.r[3].s64 = ctx.r[9].s64 + -3488;
	// 83271604: 4BA3891D  bl 0x82ca9f20
	ctx.lr = 0x83271608;
	sub_82CA9F20(ctx, base);
	// 83271608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271618 size=64
    let mut pc: u32 = 0x83271618;
    'dispatch: loop {
        match pc {
            0x83271618 => {
    //   block [0x83271618..0x83271658)
	// 83271618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271624: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327162C: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 83271630: 386ACF3C  addi r3, r10, -0x30c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12484;
	// 83271634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271638: 4AFBB899  bl 0x8222ced0
	ctx.lr = 0x8327163C;
	sub_8222CED0(ctx, base);
	// 8327163C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271640: 3869F270  addi r3, r9, -0xd90
	ctx.r[3].s64 = ctx.r[9].s64 + -3472;
	// 83271644: 4BA388DD  bl 0x82ca9f20
	ctx.lr = 0x83271648;
	sub_82CA9F20(ctx, base);
	// 83271648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271658 size=64
    let mut pc: u32 = 0x83271658;
    'dispatch: loop {
        match pc {
            0x83271658 => {
    //   block [0x83271658..0x83271698)
	// 83271658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271664: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327166C: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 83271670: 386ACF40  addi r3, r10, -0x30c0
	ctx.r[3].s64 = ctx.r[10].s64 + -12480;
	// 83271674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271678: 4AFBB859  bl 0x8222ced0
	ctx.lr = 0x8327167C;
	sub_8222CED0(ctx, base);
	// 8327167C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271680: 3869F280  addi r3, r9, -0xd80
	ctx.r[3].s64 = ctx.r[9].s64 + -3456;
	// 83271684: 4BA3889D  bl 0x82ca9f20
	ctx.lr = 0x83271688;
	sub_82CA9F20(ctx, base);
	// 83271688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271698 size=64
    let mut pc: u32 = 0x83271698;
    'dispatch: loop {
        match pc {
            0x83271698 => {
    //   block [0x83271698..0x832716D8)
	// 83271698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327169C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832716A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832716A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832716A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832716AC: 388BA8EC  addi r4, r11, -0x5714
	ctx.r[4].s64 = ctx.r[11].s64 + -22292;
	// 832716B0: 386ACF44  addi r3, r10, -0x30bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12476;
	// 832716B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832716B8: 4AFBB819  bl 0x8222ced0
	ctx.lr = 0x832716BC;
	sub_8222CED0(ctx, base);
	// 832716BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832716C0: 3869F290  addi r3, r9, -0xd70
	ctx.r[3].s64 = ctx.r[9].s64 + -3440;
	// 832716C4: 4BA3885D  bl 0x82ca9f20
	ctx.lr = 0x832716C8;
	sub_82CA9F20(ctx, base);
	// 832716C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832716CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832716D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832716D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832716D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832716D8 size=64
    let mut pc: u32 = 0x832716D8;
    'dispatch: loop {
        match pc {
            0x832716D8 => {
    //   block [0x832716D8..0x83271718)
	// 832716D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832716DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832716E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832716E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832716E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832716EC: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 832716F0: 386ACF48  addi r3, r10, -0x30b8
	ctx.r[3].s64 = ctx.r[10].s64 + -12472;
	// 832716F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832716F8: 4AFBB7D9  bl 0x8222ced0
	ctx.lr = 0x832716FC;
	sub_8222CED0(ctx, base);
	// 832716FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271700: 3869F2A0  addi r3, r9, -0xd60
	ctx.r[3].s64 = ctx.r[9].s64 + -3424;
	// 83271704: 4BA3881D  bl 0x82ca9f20
	ctx.lr = 0x83271708;
	sub_82CA9F20(ctx, base);
	// 83271708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271718 size=64
    let mut pc: u32 = 0x83271718;
    'dispatch: loop {
        match pc {
            0x83271718 => {
    //   block [0x83271718..0x83271758)
	// 83271718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271724: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83271728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327172C: 388BDF1C  addi r4, r11, -0x20e4
	ctx.r[4].s64 = ctx.r[11].s64 + -8420;
	// 83271730: 386ACF4C  addi r3, r10, -0x30b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12468;
	// 83271734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271738: 4AFBB799  bl 0x8222ced0
	ctx.lr = 0x8327173C;
	sub_8222CED0(ctx, base);
	// 8327173C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271740: 3869F2B0  addi r3, r9, -0xd50
	ctx.r[3].s64 = ctx.r[9].s64 + -3408;
	// 83271744: 4BA387DD  bl 0x82ca9f20
	ctx.lr = 0x83271748;
	sub_82CA9F20(ctx, base);
	// 83271748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327174C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271758 size=56
    let mut pc: u32 = 0x83271758;
    'dispatch: loop {
        match pc {
            0x83271758 => {
    //   block [0x83271758..0x83271790)
	// 83271758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327175C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327176C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83271770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271774: 4AF825E5  bl 0x821f3d58
	ctx.lr = 0x83271778;
	sub_821F3D58(ctx, base);
	// 83271778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327177C: 906ACF50  stw r3, -0x30b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12464 as u32), ctx.r[3].u32 ) };
	// 83271780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271790 size=56
    let mut pc: u32 = 0x83271790;
    'dispatch: loop {
        match pc {
            0x83271790 => {
    //   block [0x83271790..0x832717C8)
	// 83271790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327179C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832717A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832717A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832717A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832717AC: 4AF825AD  bl 0x821f3d58
	ctx.lr = 0x832717B0;
	sub_821F3D58(ctx, base);
	// 832717B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832717B4: 906ACF54  stw r3, -0x30ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12460 as u32), ctx.r[3].u32 ) };
	// 832717B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832717BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832717C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832717C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832717C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832717C8 size=56
    let mut pc: u32 = 0x832717C8;
    'dispatch: loop {
        match pc {
            0x832717C8 => {
    //   block [0x832717C8..0x83271800)
	// 832717C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832717CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832717D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832717D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832717D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832717DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832717E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832717E4: 4AF82575  bl 0x821f3d58
	ctx.lr = 0x832717E8;
	sub_821F3D58(ctx, base);
	// 832717E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832717EC: 906ACF58  stw r3, -0x30a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12456 as u32), ctx.r[3].u32 ) };
	// 832717F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832717F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832717F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832717FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271800 size=56
    let mut pc: u32 = 0x83271800;
    'dispatch: loop {
        match pc {
            0x83271800 => {
    //   block [0x83271800..0x83271838)
	// 83271800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327180C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271814: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83271818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327181C: 4AF8253D  bl 0x821f3d58
	ctx.lr = 0x83271820;
	sub_821F3D58(ctx, base);
	// 83271820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271824: 906ACF5C  stw r3, -0x30a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12452 as u32), ctx.r[3].u32 ) };
	// 83271828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327182C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271838 size=56
    let mut pc: u32 = 0x83271838;
    'dispatch: loop {
        match pc {
            0x83271838 => {
    //   block [0x83271838..0x83271870)
	// 83271838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327183C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327184C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83271850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271854: 4AF82505  bl 0x821f3d58
	ctx.lr = 0x83271858;
	sub_821F3D58(ctx, base);
	// 83271858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327185C: 906ACF60  stw r3, -0x30a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12448 as u32), ctx.r[3].u32 ) };
	// 83271860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271870 size=56
    let mut pc: u32 = 0x83271870;
    'dispatch: loop {
        match pc {
            0x83271870 => {
    //   block [0x83271870..0x832718A8)
	// 83271870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327187C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271884: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83271888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327188C: 4AF824CD  bl 0x821f3d58
	ctx.lr = 0x83271890;
	sub_821F3D58(ctx, base);
	// 83271890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271894: 906ACF64  stw r3, -0x309c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12444 as u32), ctx.r[3].u32 ) };
	// 83271898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327189C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832718A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832718A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832718A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832718A8 size=56
    let mut pc: u32 = 0x832718A8;
    'dispatch: loop {
        match pc {
            0x832718A8 => {
    //   block [0x832718A8..0x832718E0)
	// 832718A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832718AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832718B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832718B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832718B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832718BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832718C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832718C4: 4AF82495  bl 0x821f3d58
	ctx.lr = 0x832718C8;
	sub_821F3D58(ctx, base);
	// 832718C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832718CC: 906ACF68  stw r3, -0x3098(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12440 as u32), ctx.r[3].u32 ) };
	// 832718D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832718D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832718D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832718DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832718E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832718E0 size=56
    let mut pc: u32 = 0x832718E0;
    'dispatch: loop {
        match pc {
            0x832718E0 => {
    //   block [0x832718E0..0x83271918)
	// 832718E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832718E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832718E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832718EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832718F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832718F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832718F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832718FC: 4AF8245D  bl 0x821f3d58
	ctx.lr = 0x83271900;
	sub_821F3D58(ctx, base);
	// 83271900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271904: 906ACF6C  stw r3, -0x3094(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12436 as u32), ctx.r[3].u32 ) };
	// 83271908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327190C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271918 size=56
    let mut pc: u32 = 0x83271918;
    'dispatch: loop {
        match pc {
            0x83271918 => {
    //   block [0x83271918..0x83271950)
	// 83271918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327191C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327192C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83271930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271934: 4AF82425  bl 0x821f3d58
	ctx.lr = 0x83271938;
	sub_821F3D58(ctx, base);
	// 83271938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327193C: 906ACF70  stw r3, -0x3090(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12432 as u32), ctx.r[3].u32 ) };
	// 83271940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271950 size=56
    let mut pc: u32 = 0x83271950;
    'dispatch: loop {
        match pc {
            0x83271950 => {
    //   block [0x83271950..0x83271988)
	// 83271950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327195C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271964: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83271968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327196C: 4AF823ED  bl 0x821f3d58
	ctx.lr = 0x83271970;
	sub_821F3D58(ctx, base);
	// 83271970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271974: 906ACF74  stw r3, -0x308c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12428 as u32), ctx.r[3].u32 ) };
	// 83271978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327197C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271988 size=56
    let mut pc: u32 = 0x83271988;
    'dispatch: loop {
        match pc {
            0x83271988 => {
    //   block [0x83271988..0x832719C0)
	// 83271988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327198C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327199C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832719A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832719A4: 4AF823B5  bl 0x821f3d58
	ctx.lr = 0x832719A8;
	sub_821F3D58(ctx, base);
	// 832719A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832719AC: 906ACF78  stw r3, -0x3088(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12424 as u32), ctx.r[3].u32 ) };
	// 832719B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832719B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832719B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832719BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832719C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832719C0 size=56
    let mut pc: u32 = 0x832719C0;
    'dispatch: loop {
        match pc {
            0x832719C0 => {
    //   block [0x832719C0..0x832719F8)
	// 832719C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832719C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832719C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832719CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832719D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832719D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832719D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832719DC: 4AF8237D  bl 0x821f3d58
	ctx.lr = 0x832719E0;
	sub_821F3D58(ctx, base);
	// 832719E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832719E4: 906ACF7C  stw r3, -0x3084(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12420 as u32), ctx.r[3].u32 ) };
	// 832719E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832719EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832719F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832719F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832719F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832719F8 size=56
    let mut pc: u32 = 0x832719F8;
    'dispatch: loop {
        match pc {
            0x832719F8 => {
    //   block [0x832719F8..0x83271A30)
	// 832719F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832719FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271A08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271A0C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83271A10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271A14: 4AF82345  bl 0x821f3d58
	ctx.lr = 0x83271A18;
	sub_821F3D58(ctx, base);
	// 83271A18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271A1C: 906ACF80  stw r3, -0x3080(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12416 as u32), ctx.r[3].u32 ) };
	// 83271A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271A30 size=56
    let mut pc: u32 = 0x83271A30;
    'dispatch: loop {
        match pc {
            0x83271A30 => {
    //   block [0x83271A30..0x83271A68)
	// 83271A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271A40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271A44: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83271A48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271A4C: 4AF8230D  bl 0x821f3d58
	ctx.lr = 0x83271A50;
	sub_821F3D58(ctx, base);
	// 83271A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271A54: 906ACF84  stw r3, -0x307c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12412 as u32), ctx.r[3].u32 ) };
	// 83271A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271A68 size=56
    let mut pc: u32 = 0x83271A68;
    'dispatch: loop {
        match pc {
            0x83271A68 => {
    //   block [0x83271A68..0x83271AA0)
	// 83271A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271A74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271A78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271A7C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83271A80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271A84: 4AF822D5  bl 0x821f3d58
	ctx.lr = 0x83271A88;
	sub_821F3D58(ctx, base);
	// 83271A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271A8C: 906ACF88  stw r3, -0x3078(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12408 as u32), ctx.r[3].u32 ) };
	// 83271A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271AA0 size=56
    let mut pc: u32 = 0x83271AA0;
    'dispatch: loop {
        match pc {
            0x83271AA0 => {
    //   block [0x83271AA0..0x83271AD8)
	// 83271AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271AAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271AB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271AB4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83271AB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271ABC: 4AF8229D  bl 0x821f3d58
	ctx.lr = 0x83271AC0;
	sub_821F3D58(ctx, base);
	// 83271AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271AC4: 906ACF8C  stw r3, -0x3074(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12404 as u32), ctx.r[3].u32 ) };
	// 83271AC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271AD8 size=56
    let mut pc: u32 = 0x83271AD8;
    'dispatch: loop {
        match pc {
            0x83271AD8 => {
    //   block [0x83271AD8..0x83271B10)
	// 83271AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271AE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271AE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271AEC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83271AF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271AF4: 4AF82265  bl 0x821f3d58
	ctx.lr = 0x83271AF8;
	sub_821F3D58(ctx, base);
	// 83271AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271AFC: 906ACF90  stw r3, -0x3070(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12400 as u32), ctx.r[3].u32 ) };
	// 83271B00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271B10 size=56
    let mut pc: u32 = 0x83271B10;
    'dispatch: loop {
        match pc {
            0x83271B10 => {
    //   block [0x83271B10..0x83271B48)
	// 83271B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271B1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271B20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271B24: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83271B28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271B2C: 4AF8222D  bl 0x821f3d58
	ctx.lr = 0x83271B30;
	sub_821F3D58(ctx, base);
	// 83271B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271B34: 906ACF94  stw r3, -0x306c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12396 as u32), ctx.r[3].u32 ) };
	// 83271B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271B48 size=56
    let mut pc: u32 = 0x83271B48;
    'dispatch: loop {
        match pc {
            0x83271B48 => {
    //   block [0x83271B48..0x83271B80)
	// 83271B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271B54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271B58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271B5C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83271B60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271B64: 4AF821F5  bl 0x821f3d58
	ctx.lr = 0x83271B68;
	sub_821F3D58(ctx, base);
	// 83271B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271B6C: 906ACF98  stw r3, -0x3068(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12392 as u32), ctx.r[3].u32 ) };
	// 83271B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271B80 size=56
    let mut pc: u32 = 0x83271B80;
    'dispatch: loop {
        match pc {
            0x83271B80 => {
    //   block [0x83271B80..0x83271BB8)
	// 83271B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271B8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271B90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271B94: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83271B98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271B9C: 4AF821BD  bl 0x821f3d58
	ctx.lr = 0x83271BA0;
	sub_821F3D58(ctx, base);
	// 83271BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271BA4: 906ACF9C  stw r3, -0x3064(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12388 as u32), ctx.r[3].u32 ) };
	// 83271BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271BB8 size=56
    let mut pc: u32 = 0x83271BB8;
    'dispatch: loop {
        match pc {
            0x83271BB8 => {
    //   block [0x83271BB8..0x83271BF0)
	// 83271BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271BCC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83271BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271BD4: 4AF82185  bl 0x821f3d58
	ctx.lr = 0x83271BD8;
	sub_821F3D58(ctx, base);
	// 83271BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271BDC: 906ACFA0  stw r3, -0x3060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12384 as u32), ctx.r[3].u32 ) };
	// 83271BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271BF0 size=64
    let mut pc: u32 = 0x83271BF0;
    'dispatch: loop {
        match pc {
            0x83271BF0 => {
    //   block [0x83271BF0..0x83271C30)
	// 83271BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271BFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271C04: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 83271C08: 386ACFA4  addi r3, r10, -0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + -12380;
	// 83271C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271C10: 4AFBB2C1  bl 0x8222ced0
	ctx.lr = 0x83271C14;
	sub_8222CED0(ctx, base);
	// 83271C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271C18: 3869F2C0  addi r3, r9, -0xd40
	ctx.r[3].s64 = ctx.r[9].s64 + -3392;
	// 83271C1C: 4BA38305  bl 0x82ca9f20
	ctx.lr = 0x83271C20;
	sub_82CA9F20(ctx, base);
	// 83271C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271C30 size=64
    let mut pc: u32 = 0x83271C30;
    'dispatch: loop {
        match pc {
            0x83271C30 => {
    //   block [0x83271C30..0x83271C70)
	// 83271C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271C3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271C44: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 83271C48: 386ACFA8  addi r3, r10, -0x3058
	ctx.r[3].s64 = ctx.r[10].s64 + -12376;
	// 83271C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271C50: 4AFBB281  bl 0x8222ced0
	ctx.lr = 0x83271C54;
	sub_8222CED0(ctx, base);
	// 83271C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271C58: 3869F2D0  addi r3, r9, -0xd30
	ctx.r[3].s64 = ctx.r[9].s64 + -3376;
	// 83271C5C: 4BA382C5  bl 0x82ca9f20
	ctx.lr = 0x83271C60;
	sub_82CA9F20(ctx, base);
	// 83271C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271C70 size=64
    let mut pc: u32 = 0x83271C70;
    'dispatch: loop {
        match pc {
            0x83271C70 => {
    //   block [0x83271C70..0x83271CB0)
	// 83271C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271C7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271C84: 388BAA08  addi r4, r11, -0x55f8
	ctx.r[4].s64 = ctx.r[11].s64 + -22008;
	// 83271C88: 386ACFAC  addi r3, r10, -0x3054
	ctx.r[3].s64 = ctx.r[10].s64 + -12372;
	// 83271C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271C90: 4AFBB241  bl 0x8222ced0
	ctx.lr = 0x83271C94;
	sub_8222CED0(ctx, base);
	// 83271C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271C98: 3869F2E0  addi r3, r9, -0xd20
	ctx.r[3].s64 = ctx.r[9].s64 + -3360;
	// 83271C9C: 4BA38285  bl 0x82ca9f20
	ctx.lr = 0x83271CA0;
	sub_82CA9F20(ctx, base);
	// 83271CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271CB0 size=64
    let mut pc: u32 = 0x83271CB0;
    'dispatch: loop {
        match pc {
            0x83271CB0 => {
    //   block [0x83271CB0..0x83271CF0)
	// 83271CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271CBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271CC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271CC4: 388BAA50  addi r4, r11, -0x55b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21936;
	// 83271CC8: 386ACFB0  addi r3, r10, -0x3050
	ctx.r[3].s64 = ctx.r[10].s64 + -12368;
	// 83271CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271CD0: 4AFBB201  bl 0x8222ced0
	ctx.lr = 0x83271CD4;
	sub_8222CED0(ctx, base);
	// 83271CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271CD8: 3869F2F0  addi r3, r9, -0xd10
	ctx.r[3].s64 = ctx.r[9].s64 + -3344;
	// 83271CDC: 4BA38245  bl 0x82ca9f20
	ctx.lr = 0x83271CE0;
	sub_82CA9F20(ctx, base);
	// 83271CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271CF0 size=64
    let mut pc: u32 = 0x83271CF0;
    'dispatch: loop {
        match pc {
            0x83271CF0 => {
    //   block [0x83271CF0..0x83271D30)
	// 83271CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271CFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271D04: 388BAA88  addi r4, r11, -0x5578
	ctx.r[4].s64 = ctx.r[11].s64 + -21880;
	// 83271D08: 386ACFB4  addi r3, r10, -0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + -12364;
	// 83271D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271D10: 4AFBB1C1  bl 0x8222ced0
	ctx.lr = 0x83271D14;
	sub_8222CED0(ctx, base);
	// 83271D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271D18: 3869F300  addi r3, r9, -0xd00
	ctx.r[3].s64 = ctx.r[9].s64 + -3328;
	// 83271D1C: 4BA38205  bl 0x82ca9f20
	ctx.lr = 0x83271D20;
	sub_82CA9F20(ctx, base);
	// 83271D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271D30 size=64
    let mut pc: u32 = 0x83271D30;
    'dispatch: loop {
        match pc {
            0x83271D30 => {
    //   block [0x83271D30..0x83271D70)
	// 83271D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271D3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271D44: 388BAAC4  addi r4, r11, -0x553c
	ctx.r[4].s64 = ctx.r[11].s64 + -21820;
	// 83271D48: 386ACFB8  addi r3, r10, -0x3048
	ctx.r[3].s64 = ctx.r[10].s64 + -12360;
	// 83271D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271D50: 4AFBB181  bl 0x8222ced0
	ctx.lr = 0x83271D54;
	sub_8222CED0(ctx, base);
	// 83271D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271D58: 3869F310  addi r3, r9, -0xcf0
	ctx.r[3].s64 = ctx.r[9].s64 + -3312;
	// 83271D5C: 4BA381C5  bl 0x82ca9f20
	ctx.lr = 0x83271D60;
	sub_82CA9F20(ctx, base);
	// 83271D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271D70 size=64
    let mut pc: u32 = 0x83271D70;
    'dispatch: loop {
        match pc {
            0x83271D70 => {
    //   block [0x83271D70..0x83271DB0)
	// 83271D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271D7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271D84: 388BAAF4  addi r4, r11, -0x550c
	ctx.r[4].s64 = ctx.r[11].s64 + -21772;
	// 83271D88: 386ACFBC  addi r3, r10, -0x3044
	ctx.r[3].s64 = ctx.r[10].s64 + -12356;
	// 83271D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271D90: 4AFBB141  bl 0x8222ced0
	ctx.lr = 0x83271D94;
	sub_8222CED0(ctx, base);
	// 83271D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271D98: 3869F320  addi r3, r9, -0xce0
	ctx.r[3].s64 = ctx.r[9].s64 + -3296;
	// 83271D9C: 4BA38185  bl 0x82ca9f20
	ctx.lr = 0x83271DA0;
	sub_82CA9F20(ctx, base);
	// 83271DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271DB0 size=64
    let mut pc: u32 = 0x83271DB0;
    'dispatch: loop {
        match pc {
            0x83271DB0 => {
    //   block [0x83271DB0..0x83271DF0)
	// 83271DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271DBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271DC4: 388BAB2C  addi r4, r11, -0x54d4
	ctx.r[4].s64 = ctx.r[11].s64 + -21716;
	// 83271DC8: 386ACFC0  addi r3, r10, -0x3040
	ctx.r[3].s64 = ctx.r[10].s64 + -12352;
	// 83271DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271DD0: 4AFBB101  bl 0x8222ced0
	ctx.lr = 0x83271DD4;
	sub_8222CED0(ctx, base);
	// 83271DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271DD8: 3869F330  addi r3, r9, -0xcd0
	ctx.r[3].s64 = ctx.r[9].s64 + -3280;
	// 83271DDC: 4BA38145  bl 0x82ca9f20
	ctx.lr = 0x83271DE0;
	sub_82CA9F20(ctx, base);
	// 83271DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271DF0 size=64
    let mut pc: u32 = 0x83271DF0;
    'dispatch: loop {
        match pc {
            0x83271DF0 => {
    //   block [0x83271DF0..0x83271E30)
	// 83271DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271DFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271E04: 388BAB64  addi r4, r11, -0x549c
	ctx.r[4].s64 = ctx.r[11].s64 + -21660;
	// 83271E08: 386ACFC4  addi r3, r10, -0x303c
	ctx.r[3].s64 = ctx.r[10].s64 + -12348;
	// 83271E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271E10: 4AFBB0C1  bl 0x8222ced0
	ctx.lr = 0x83271E14;
	sub_8222CED0(ctx, base);
	// 83271E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271E18: 3869F340  addi r3, r9, -0xcc0
	ctx.r[3].s64 = ctx.r[9].s64 + -3264;
	// 83271E1C: 4BA38105  bl 0x82ca9f20
	ctx.lr = 0x83271E20;
	sub_82CA9F20(ctx, base);
	// 83271E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271E30 size=64
    let mut pc: u32 = 0x83271E30;
    'dispatch: loop {
        match pc {
            0x83271E30 => {
    //   block [0x83271E30..0x83271E70)
	// 83271E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271E3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271E44: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 83271E48: 386ACFC8  addi r3, r10, -0x3038
	ctx.r[3].s64 = ctx.r[10].s64 + -12344;
	// 83271E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271E50: 4AFBB081  bl 0x8222ced0
	ctx.lr = 0x83271E54;
	sub_8222CED0(ctx, base);
	// 83271E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271E58: 3869F350  addi r3, r9, -0xcb0
	ctx.r[3].s64 = ctx.r[9].s64 + -3248;
	// 83271E5C: 4BA380C5  bl 0x82ca9f20
	ctx.lr = 0x83271E60;
	sub_82CA9F20(ctx, base);
	// 83271E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271E70 size=64
    let mut pc: u32 = 0x83271E70;
    'dispatch: loop {
        match pc {
            0x83271E70 => {
    //   block [0x83271E70..0x83271EB0)
	// 83271E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271E7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271E80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271E84: 388BABE4  addi r4, r11, -0x541c
	ctx.r[4].s64 = ctx.r[11].s64 + -21532;
	// 83271E88: 386ACFCC  addi r3, r10, -0x3034
	ctx.r[3].s64 = ctx.r[10].s64 + -12340;
	// 83271E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271E90: 4AFBB041  bl 0x8222ced0
	ctx.lr = 0x83271E94;
	sub_8222CED0(ctx, base);
	// 83271E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271E98: 3869F360  addi r3, r9, -0xca0
	ctx.r[3].s64 = ctx.r[9].s64 + -3232;
	// 83271E9C: 4BA38085  bl 0x82ca9f20
	ctx.lr = 0x83271EA0;
	sub_82CA9F20(ctx, base);
	// 83271EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271EB0 size=64
    let mut pc: u32 = 0x83271EB0;
    'dispatch: loop {
        match pc {
            0x83271EB0 => {
    //   block [0x83271EB0..0x83271EF0)
	// 83271EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271EBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271EC4: 388BAC20  addi r4, r11, -0x53e0
	ctx.r[4].s64 = ctx.r[11].s64 + -21472;
	// 83271EC8: 386ACFD0  addi r3, r10, -0x3030
	ctx.r[3].s64 = ctx.r[10].s64 + -12336;
	// 83271ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271ED0: 4AFBB001  bl 0x8222ced0
	ctx.lr = 0x83271ED4;
	sub_8222CED0(ctx, base);
	// 83271ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271ED8: 3869F370  addi r3, r9, -0xc90
	ctx.r[3].s64 = ctx.r[9].s64 + -3216;
	// 83271EDC: 4BA38045  bl 0x82ca9f20
	ctx.lr = 0x83271EE0;
	sub_82CA9F20(ctx, base);
	// 83271EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271EF0 size=64
    let mut pc: u32 = 0x83271EF0;
    'dispatch: loop {
        match pc {
            0x83271EF0 => {
    //   block [0x83271EF0..0x83271F30)
	// 83271EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271EFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271F04: 388BAC58  addi r4, r11, -0x53a8
	ctx.r[4].s64 = ctx.r[11].s64 + -21416;
	// 83271F08: 386ACFD4  addi r3, r10, -0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + -12332;
	// 83271F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271F10: 4AFBAFC1  bl 0x8222ced0
	ctx.lr = 0x83271F14;
	sub_8222CED0(ctx, base);
	// 83271F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271F18: 3869F380  addi r3, r9, -0xc80
	ctx.r[3].s64 = ctx.r[9].s64 + -3200;
	// 83271F1C: 4BA38005  bl 0x82ca9f20
	ctx.lr = 0x83271F20;
	sub_82CA9F20(ctx, base);
	// 83271F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271F30 size=64
    let mut pc: u32 = 0x83271F30;
    'dispatch: loop {
        match pc {
            0x83271F30 => {
    //   block [0x83271F30..0x83271F70)
	// 83271F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271F3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271F44: 388BAC90  addi r4, r11, -0x5370
	ctx.r[4].s64 = ctx.r[11].s64 + -21360;
	// 83271F48: 386ACFD8  addi r3, r10, -0x3028
	ctx.r[3].s64 = ctx.r[10].s64 + -12328;
	// 83271F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271F50: 4AFBAF81  bl 0x8222ced0
	ctx.lr = 0x83271F54;
	sub_8222CED0(ctx, base);
	// 83271F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271F58: 3869F390  addi r3, r9, -0xc70
	ctx.r[3].s64 = ctx.r[9].s64 + -3184;
	// 83271F5C: 4BA37FC5  bl 0x82ca9f20
	ctx.lr = 0x83271F60;
	sub_82CA9F20(ctx, base);
	// 83271F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271F70 size=64
    let mut pc: u32 = 0x83271F70;
    'dispatch: loop {
        match pc {
            0x83271F70 => {
    //   block [0x83271F70..0x83271FB0)
	// 83271F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271F7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271F84: 388BACD0  addi r4, r11, -0x5330
	ctx.r[4].s64 = ctx.r[11].s64 + -21296;
	// 83271F88: 386ACFDC  addi r3, r10, -0x3024
	ctx.r[3].s64 = ctx.r[10].s64 + -12324;
	// 83271F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271F90: 4AFBAF41  bl 0x8222ced0
	ctx.lr = 0x83271F94;
	sub_8222CED0(ctx, base);
	// 83271F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271F98: 3869F3A0  addi r3, r9, -0xc60
	ctx.r[3].s64 = ctx.r[9].s64 + -3168;
	// 83271F9C: 4BA37F85  bl 0x82ca9f20
	ctx.lr = 0x83271FA0;
	sub_82CA9F20(ctx, base);
	// 83271FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271FB0 size=64
    let mut pc: u32 = 0x83271FB0;
    'dispatch: loop {
        match pc {
            0x83271FB0 => {
    //   block [0x83271FB0..0x83271FF0)
	// 83271FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271FBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271FC4: 388BAD08  addi r4, r11, -0x52f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21240;
	// 83271FC8: 386ACFE0  addi r3, r10, -0x3020
	ctx.r[3].s64 = ctx.r[10].s64 + -12320;
	// 83271FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271FD0: 4AFBAF01  bl 0x8222ced0
	ctx.lr = 0x83271FD4;
	sub_8222CED0(ctx, base);
	// 83271FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271FD8: 3869F3B0  addi r3, r9, -0xc50
	ctx.r[3].s64 = ctx.r[9].s64 + -3152;
	// 83271FDC: 4BA37F45  bl 0x82ca9f20
	ctx.lr = 0x83271FE0;
	sub_82CA9F20(ctx, base);
	// 83271FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271FF0 size=64
    let mut pc: u32 = 0x83271FF0;
    'dispatch: loop {
        match pc {
            0x83271FF0 => {
    //   block [0x83271FF0..0x83272030)
	// 83271FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271FFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272004: 388BAD48  addi r4, r11, -0x52b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21176;
	// 83272008: 386ACFE4  addi r3, r10, -0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + -12316;
	// 8327200C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272010: 4AFBAEC1  bl 0x8222ced0
	ctx.lr = 0x83272014;
	sub_8222CED0(ctx, base);
	// 83272014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272018: 3869F3C0  addi r3, r9, -0xc40
	ctx.r[3].s64 = ctx.r[9].s64 + -3136;
	// 8327201C: 4BA37F05  bl 0x82ca9f20
	ctx.lr = 0x83272020;
	sub_82CA9F20(ctx, base);
	// 83272020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327202C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272030 size=64
    let mut pc: u32 = 0x83272030;
    'dispatch: loop {
        match pc {
            0x83272030 => {
    //   block [0x83272030..0x83272070)
	// 83272030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327203C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272044: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83272048: 386ACFE8  addi r3, r10, -0x3018
	ctx.r[3].s64 = ctx.r[10].s64 + -12312;
	// 8327204C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272050: 4AFBAE81  bl 0x8222ced0
	ctx.lr = 0x83272054;
	sub_8222CED0(ctx, base);
	// 83272054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272058: 3869F3D0  addi r3, r9, -0xc30
	ctx.r[3].s64 = ctx.r[9].s64 + -3120;
	// 8327205C: 4BA37EC5  bl 0x82ca9f20
	ctx.lr = 0x83272060;
	sub_82CA9F20(ctx, base);
	// 83272060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327206C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272070 size=56
    let mut pc: u32 = 0x83272070;
    'dispatch: loop {
        match pc {
            0x83272070 => {
    //   block [0x83272070..0x832720A8)
	// 83272070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327207C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272080: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272084: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83272088: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327208C: 4AF81CCD  bl 0x821f3d58
	ctx.lr = 0x83272090;
	sub_821F3D58(ctx, base);
	// 83272090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272094: 906ACFEC  stw r3, -0x3014(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12308 as u32), ctx.r[3].u32 ) };
	// 83272098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832720A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832720A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832720A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832720A8 size=56
    let mut pc: u32 = 0x832720A8;
    'dispatch: loop {
        match pc {
            0x832720A8 => {
    //   block [0x832720A8..0x832720E0)
	// 832720A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832720AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832720B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832720B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832720B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832720BC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832720C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832720C4: 4AF81C95  bl 0x821f3d58
	ctx.lr = 0x832720C8;
	sub_821F3D58(ctx, base);
	// 832720C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832720CC: 906ACFF0  stw r3, -0x3010(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12304 as u32), ctx.r[3].u32 ) };
	// 832720D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832720D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832720D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832720DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832720E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832720E0 size=56
    let mut pc: u32 = 0x832720E0;
    'dispatch: loop {
        match pc {
            0x832720E0 => {
    //   block [0x832720E0..0x83272118)
	// 832720E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832720E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832720E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832720EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832720F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832720F4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832720F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832720FC: 4AF81C5D  bl 0x821f3d58
	ctx.lr = 0x83272100;
	sub_821F3D58(ctx, base);
	// 83272100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272104: 906ACFF4  stw r3, -0x300c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12300 as u32), ctx.r[3].u32 ) };
	// 83272108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327210C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272118 size=56
    let mut pc: u32 = 0x83272118;
    'dispatch: loop {
        match pc {
            0x83272118 => {
    //   block [0x83272118..0x83272150)
	// 83272118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327212C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83272130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272134: 4AF81C25  bl 0x821f3d58
	ctx.lr = 0x83272138;
	sub_821F3D58(ctx, base);
	// 83272138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327213C: 906ACFF8  stw r3, -0x3008(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12296 as u32), ctx.r[3].u32 ) };
	// 83272140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272150 size=56
    let mut pc: u32 = 0x83272150;
    'dispatch: loop {
        match pc {
            0x83272150 => {
    //   block [0x83272150..0x83272188)
	// 83272150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327215C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272164: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83272168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327216C: 4AF81BED  bl 0x821f3d58
	ctx.lr = 0x83272170;
	sub_821F3D58(ctx, base);
	// 83272170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272174: 906ACFFC  stw r3, -0x3004(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12292 as u32), ctx.r[3].u32 ) };
	// 83272178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327217C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272188 size=56
    let mut pc: u32 = 0x83272188;
    'dispatch: loop {
        match pc {
            0x83272188 => {
    //   block [0x83272188..0x832721C0)
	// 83272188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272194: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272198: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327219C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832721A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832721A4: 4AF81BB5  bl 0x821f3d58
	ctx.lr = 0x832721A8;
	sub_821F3D58(ctx, base);
	// 832721A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832721AC: 906AD000  stw r3, -0x3000(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12288 as u32), ctx.r[3].u32 ) };
	// 832721B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832721B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832721B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832721BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832721C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832721C0 size=56
    let mut pc: u32 = 0x832721C0;
    'dispatch: loop {
        match pc {
            0x832721C0 => {
    //   block [0x832721C0..0x832721F8)
	// 832721C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832721C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832721C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832721CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832721D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832721D4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832721D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832721DC: 4AF81B7D  bl 0x821f3d58
	ctx.lr = 0x832721E0;
	sub_821F3D58(ctx, base);
	// 832721E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832721E4: 906AD004  stw r3, -0x2ffc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12284 as u32), ctx.r[3].u32 ) };
	// 832721E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832721EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832721F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832721F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832721F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832721F8 size=56
    let mut pc: u32 = 0x832721F8;
    'dispatch: loop {
        match pc {
            0x832721F8 => {
    //   block [0x832721F8..0x83272230)
	// 832721F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832721FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272208: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327220C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83272210: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272214: 4AF81B45  bl 0x821f3d58
	ctx.lr = 0x83272218;
	sub_821F3D58(ctx, base);
	// 83272218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327221C: 906AD008  stw r3, -0x2ff8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12280 as u32), ctx.r[3].u32 ) };
	// 83272220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327222C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272230 size=56
    let mut pc: u32 = 0x83272230;
    'dispatch: loop {
        match pc {
            0x83272230 => {
    //   block [0x83272230..0x83272268)
	// 83272230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327223C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272240: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272244: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83272248: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327224C: 4AF81B0D  bl 0x821f3d58
	ctx.lr = 0x83272250;
	sub_821F3D58(ctx, base);
	// 83272250: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272254: 906AD00C  stw r3, -0x2ff4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12276 as u32), ctx.r[3].u32 ) };
	// 83272258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272268 size=56
    let mut pc: u32 = 0x83272268;
    'dispatch: loop {
        match pc {
            0x83272268 => {
    //   block [0x83272268..0x832722A0)
	// 83272268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272274: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272278: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327227C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83272280: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272284: 4AF81AD5  bl 0x821f3d58
	ctx.lr = 0x83272288;
	sub_821F3D58(ctx, base);
	// 83272288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327228C: 906AD010  stw r3, -0x2ff0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12272 as u32), ctx.r[3].u32 ) };
	// 83272290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327229C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832722A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832722A0 size=56
    let mut pc: u32 = 0x832722A0;
    'dispatch: loop {
        match pc {
            0x832722A0 => {
    //   block [0x832722A0..0x832722D8)
	// 832722A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832722A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832722A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832722AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832722B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832722B4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832722B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832722BC: 4AF81A9D  bl 0x821f3d58
	ctx.lr = 0x832722C0;
	sub_821F3D58(ctx, base);
	// 832722C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832722C4: 906AD014  stw r3, -0x2fec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12268 as u32), ctx.r[3].u32 ) };
	// 832722C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832722CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832722D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832722D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


