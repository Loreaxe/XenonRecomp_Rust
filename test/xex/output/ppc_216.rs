pub fn sub_832668B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832668B8 size=64
    let mut pc: u32 = 0x832668B8;
    'dispatch: loop {
        match pc {
            0x832668B8 => {
    //   block [0x832668B8..0x832668F8)
	// 832668B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832668BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832668C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832668C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832668C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832668CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832668D0: 386AB840  addi r3, r10, -0x47c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18368;
	// 832668D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832668D8: 4AFC65F9  bl 0x8222ced0
	ctx.lr = 0x832668DC;
	sub_8222CED0(ctx, base);
	// 832668DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832668E0: 3869D740  addi r3, r9, -0x28c0
	ctx.r[3].s64 = ctx.r[9].s64 + -10432;
	// 832668E4: 4BA4363D  bl 0x82ca9f20
	ctx.lr = 0x832668E8;
	sub_82CA9F20(ctx, base);
	// 832668E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832668EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832668F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832668F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832668F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832668F8 size=64
    let mut pc: u32 = 0x832668F8;
    'dispatch: loop {
        match pc {
            0x832668F8 => {
    //   block [0x832668F8..0x83266938)
	// 832668F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832668FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326690C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266910: 386AB844  addi r3, r10, -0x47bc
	ctx.r[3].s64 = ctx.r[10].s64 + -18364;
	// 83266914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266918: 4AFC65B9  bl 0x8222ced0
	ctx.lr = 0x8326691C;
	sub_8222CED0(ctx, base);
	// 8326691C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266920: 3869D750  addi r3, r9, -0x28b0
	ctx.r[3].s64 = ctx.r[9].s64 + -10416;
	// 83266924: 4BA435FD  bl 0x82ca9f20
	ctx.lr = 0x83266928;
	sub_82CA9F20(ctx, base);
	// 83266928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326692C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266938 size=64
    let mut pc: u32 = 0x83266938;
    'dispatch: loop {
        match pc {
            0x83266938 => {
    //   block [0x83266938..0x83266978)
	// 83266938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326694C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266950: 386AB848  addi r3, r10, -0x47b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18360;
	// 83266954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266958: 4AFC6579  bl 0x8222ced0
	ctx.lr = 0x8326695C;
	sub_8222CED0(ctx, base);
	// 8326695C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266960: 3869D760  addi r3, r9, -0x28a0
	ctx.r[3].s64 = ctx.r[9].s64 + -10400;
	// 83266964: 4BA435BD  bl 0x82ca9f20
	ctx.lr = 0x83266968;
	sub_82CA9F20(ctx, base);
	// 83266968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326696C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266978 size=64
    let mut pc: u32 = 0x83266978;
    'dispatch: loop {
        match pc {
            0x83266978 => {
    //   block [0x83266978..0x832669B8)
	// 83266978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326698C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266990: 386AB84C  addi r3, r10, -0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18356;
	// 83266994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266998: 4AFC6539  bl 0x8222ced0
	ctx.lr = 0x8326699C;
	sub_8222CED0(ctx, base);
	// 8326699C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832669A0: 3869D770  addi r3, r9, -0x2890
	ctx.r[3].s64 = ctx.r[9].s64 + -10384;
	// 832669A4: 4BA4357D  bl 0x82ca9f20
	ctx.lr = 0x832669A8;
	sub_82CA9F20(ctx, base);
	// 832669A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832669AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832669B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832669B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832669B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832669B8 size=64
    let mut pc: u32 = 0x832669B8;
    'dispatch: loop {
        match pc {
            0x832669B8 => {
    //   block [0x832669B8..0x832669F8)
	// 832669B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832669BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832669C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832669C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832669C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832669CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832669D0: 386AB850  addi r3, r10, -0x47b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18352;
	// 832669D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832669D8: 4AFC64F9  bl 0x8222ced0
	ctx.lr = 0x832669DC;
	sub_8222CED0(ctx, base);
	// 832669DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832669E0: 3869D780  addi r3, r9, -0x2880
	ctx.r[3].s64 = ctx.r[9].s64 + -10368;
	// 832669E4: 4BA4353D  bl 0x82ca9f20
	ctx.lr = 0x832669E8;
	sub_82CA9F20(ctx, base);
	// 832669E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832669EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832669F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832669F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832669F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832669F8 size=64
    let mut pc: u32 = 0x832669F8;
    'dispatch: loop {
        match pc {
            0x832669F8 => {
    //   block [0x832669F8..0x83266A38)
	// 832669F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832669FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266A0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266A10: 386AB854  addi r3, r10, -0x47ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18348;
	// 83266A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266A18: 4AFC64B9  bl 0x8222ced0
	ctx.lr = 0x83266A1C;
	sub_8222CED0(ctx, base);
	// 83266A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266A20: 3869D790  addi r3, r9, -0x2870
	ctx.r[3].s64 = ctx.r[9].s64 + -10352;
	// 83266A24: 4BA434FD  bl 0x82ca9f20
	ctx.lr = 0x83266A28;
	sub_82CA9F20(ctx, base);
	// 83266A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266A38 size=64
    let mut pc: u32 = 0x83266A38;
    'dispatch: loop {
        match pc {
            0x83266A38 => {
    //   block [0x83266A38..0x83266A78)
	// 83266A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266A4C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266A50: 386AB858  addi r3, r10, -0x47a8
	ctx.r[3].s64 = ctx.r[10].s64 + -18344;
	// 83266A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266A58: 4AFC6479  bl 0x8222ced0
	ctx.lr = 0x83266A5C;
	sub_8222CED0(ctx, base);
	// 83266A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266A60: 3869D7A0  addi r3, r9, -0x2860
	ctx.r[3].s64 = ctx.r[9].s64 + -10336;
	// 83266A64: 4BA434BD  bl 0x82ca9f20
	ctx.lr = 0x83266A68;
	sub_82CA9F20(ctx, base);
	// 83266A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266A78 size=64
    let mut pc: u32 = 0x83266A78;
    'dispatch: loop {
        match pc {
            0x83266A78 => {
    //   block [0x83266A78..0x83266AB8)
	// 83266A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266A8C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266A90: 386AB85C  addi r3, r10, -0x47a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18340;
	// 83266A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266A98: 4AFC6439  bl 0x8222ced0
	ctx.lr = 0x83266A9C;
	sub_8222CED0(ctx, base);
	// 83266A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266AA0: 3869D7B0  addi r3, r9, -0x2850
	ctx.r[3].s64 = ctx.r[9].s64 + -10320;
	// 83266AA4: 4BA4347D  bl 0x82ca9f20
	ctx.lr = 0x83266AA8;
	sub_82CA9F20(ctx, base);
	// 83266AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266AB8 size=64
    let mut pc: u32 = 0x83266AB8;
    'dispatch: loop {
        match pc {
            0x83266AB8 => {
    //   block [0x83266AB8..0x83266AF8)
	// 83266AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266ACC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266AD0: 386AB860  addi r3, r10, -0x47a0
	ctx.r[3].s64 = ctx.r[10].s64 + -18336;
	// 83266AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266AD8: 4AFC63F9  bl 0x8222ced0
	ctx.lr = 0x83266ADC;
	sub_8222CED0(ctx, base);
	// 83266ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266AE0: 3869D7C0  addi r3, r9, -0x2840
	ctx.r[3].s64 = ctx.r[9].s64 + -10304;
	// 83266AE4: 4BA4343D  bl 0x82ca9f20
	ctx.lr = 0x83266AE8;
	sub_82CA9F20(ctx, base);
	// 83266AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266AF8 size=64
    let mut pc: u32 = 0x83266AF8;
    'dispatch: loop {
        match pc {
            0x83266AF8 => {
    //   block [0x83266AF8..0x83266B38)
	// 83266AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266B0C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266B10: 386AB864  addi r3, r10, -0x479c
	ctx.r[3].s64 = ctx.r[10].s64 + -18332;
	// 83266B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266B18: 4AFC63B9  bl 0x8222ced0
	ctx.lr = 0x83266B1C;
	sub_8222CED0(ctx, base);
	// 83266B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266B20: 3869D7D0  addi r3, r9, -0x2830
	ctx.r[3].s64 = ctx.r[9].s64 + -10288;
	// 83266B24: 4BA433FD  bl 0x82ca9f20
	ctx.lr = 0x83266B28;
	sub_82CA9F20(ctx, base);
	// 83266B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266B38 size=64
    let mut pc: u32 = 0x83266B38;
    'dispatch: loop {
        match pc {
            0x83266B38 => {
    //   block [0x83266B38..0x83266B78)
	// 83266B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266B4C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266B50: 386AB868  addi r3, r10, -0x4798
	ctx.r[3].s64 = ctx.r[10].s64 + -18328;
	// 83266B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266B58: 4AFC6379  bl 0x8222ced0
	ctx.lr = 0x83266B5C;
	sub_8222CED0(ctx, base);
	// 83266B5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266B60: 3869D7E0  addi r3, r9, -0x2820
	ctx.r[3].s64 = ctx.r[9].s64 + -10272;
	// 83266B64: 4BA433BD  bl 0x82ca9f20
	ctx.lr = 0x83266B68;
	sub_82CA9F20(ctx, base);
	// 83266B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266B78 size=64
    let mut pc: u32 = 0x83266B78;
    'dispatch: loop {
        match pc {
            0x83266B78 => {
    //   block [0x83266B78..0x83266BB8)
	// 83266B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266B8C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266B90: 386AB86C  addi r3, r10, -0x4794
	ctx.r[3].s64 = ctx.r[10].s64 + -18324;
	// 83266B94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266B98: 4AFC6339  bl 0x8222ced0
	ctx.lr = 0x83266B9C;
	sub_8222CED0(ctx, base);
	// 83266B9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266BA0: 3869D7F0  addi r3, r9, -0x2810
	ctx.r[3].s64 = ctx.r[9].s64 + -10256;
	// 83266BA4: 4BA4337D  bl 0x82ca9f20
	ctx.lr = 0x83266BA8;
	sub_82CA9F20(ctx, base);
	// 83266BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266BB8 size=56
    let mut pc: u32 = 0x83266BB8;
    'dispatch: loop {
        match pc {
            0x83266BB8 => {
    //   block [0x83266BB8..0x83266BF0)
	// 83266BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266BCC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83266BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266BD4: 4AF8D185  bl 0x821f3d58
	ctx.lr = 0x83266BD8;
	sub_821F3D58(ctx, base);
	// 83266BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266BDC: 906AB870  stw r3, -0x4790(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18320 as u32), ctx.r[3].u32 ) };
	// 83266BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266BF0 size=56
    let mut pc: u32 = 0x83266BF0;
    'dispatch: loop {
        match pc {
            0x83266BF0 => {
    //   block [0x83266BF0..0x83266C28)
	// 83266BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266C00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266C04: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83266C08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266C0C: 4AF8D14D  bl 0x821f3d58
	ctx.lr = 0x83266C10;
	sub_821F3D58(ctx, base);
	// 83266C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266C14: 906AB874  stw r3, -0x478c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18316 as u32), ctx.r[3].u32 ) };
	// 83266C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266C28 size=56
    let mut pc: u32 = 0x83266C28;
    'dispatch: loop {
        match pc {
            0x83266C28 => {
    //   block [0x83266C28..0x83266C60)
	// 83266C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266C34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266C38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266C3C: 386B3508  addi r3, r11, 0x3508
	ctx.r[3].s64 = ctx.r[11].s64 + 13576;
	// 83266C40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266C44: 4AF8D115  bl 0x821f3d58
	ctx.lr = 0x83266C48;
	sub_821F3D58(ctx, base);
	// 83266C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266C4C: 906AB878  stw r3, -0x4788(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18312 as u32), ctx.r[3].u32 ) };
	// 83266C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266C60 size=56
    let mut pc: u32 = 0x83266C60;
    'dispatch: loop {
        match pc {
            0x83266C60 => {
    //   block [0x83266C60..0x83266C98)
	// 83266C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266C6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266C70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266C74: 386B3520  addi r3, r11, 0x3520
	ctx.r[3].s64 = ctx.r[11].s64 + 13600;
	// 83266C78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266C7C: 4AF8D0DD  bl 0x821f3d58
	ctx.lr = 0x83266C80;
	sub_821F3D58(ctx, base);
	// 83266C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266C84: 906AB87C  stw r3, -0x4784(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18308 as u32), ctx.r[3].u32 ) };
	// 83266C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266C98 size=56
    let mut pc: u32 = 0x83266C98;
    'dispatch: loop {
        match pc {
            0x83266C98 => {
    //   block [0x83266C98..0x83266CD0)
	// 83266C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266CA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266CA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266CAC: 386B3530  addi r3, r11, 0x3530
	ctx.r[3].s64 = ctx.r[11].s64 + 13616;
	// 83266CB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266CB4: 4AF8D0A5  bl 0x821f3d58
	ctx.lr = 0x83266CB8;
	sub_821F3D58(ctx, base);
	// 83266CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266CBC: 906AB880  stw r3, -0x4780(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18304 as u32), ctx.r[3].u32 ) };
	// 83266CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266CD0 size=56
    let mut pc: u32 = 0x83266CD0;
    'dispatch: loop {
        match pc {
            0x83266CD0 => {
    //   block [0x83266CD0..0x83266D08)
	// 83266CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266CDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266CE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266CE4: 386B3544  addi r3, r11, 0x3544
	ctx.r[3].s64 = ctx.r[11].s64 + 13636;
	// 83266CE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266CEC: 4AF8D06D  bl 0x821f3d58
	ctx.lr = 0x83266CF0;
	sub_821F3D58(ctx, base);
	// 83266CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266CF4: 906AB884  stw r3, -0x477c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18300 as u32), ctx.r[3].u32 ) };
	// 83266CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266D08 size=56
    let mut pc: u32 = 0x83266D08;
    'dispatch: loop {
        match pc {
            0x83266D08 => {
    //   block [0x83266D08..0x83266D40)
	// 83266D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266D14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266D18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266D1C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83266D20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266D24: 4AF8D035  bl 0x821f3d58
	ctx.lr = 0x83266D28;
	sub_821F3D58(ctx, base);
	// 83266D28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266D2C: 906AB888  stw r3, -0x4778(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18296 as u32), ctx.r[3].u32 ) };
	// 83266D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266D40 size=56
    let mut pc: u32 = 0x83266D40;
    'dispatch: loop {
        match pc {
            0x83266D40 => {
    //   block [0x83266D40..0x83266D78)
	// 83266D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266D4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266D50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266D54: 386B355C  addi r3, r11, 0x355c
	ctx.r[3].s64 = ctx.r[11].s64 + 13660;
	// 83266D58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266D5C: 4AF8CFFD  bl 0x821f3d58
	ctx.lr = 0x83266D60;
	sub_821F3D58(ctx, base);
	// 83266D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266D64: 906AB88C  stw r3, -0x4774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18292 as u32), ctx.r[3].u32 ) };
	// 83266D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266D78 size=64
    let mut pc: u32 = 0x83266D78;
    'dispatch: loop {
        match pc {
            0x83266D78 => {
    //   block [0x83266D78..0x83266DB8)
	// 83266D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266D8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266D90: 386AB890  addi r3, r10, -0x4770
	ctx.r[3].s64 = ctx.r[10].s64 + -18288;
	// 83266D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266D98: 4AFC6139  bl 0x8222ced0
	ctx.lr = 0x83266D9C;
	sub_8222CED0(ctx, base);
	// 83266D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266DA0: 3869D800  addi r3, r9, -0x2800
	ctx.r[3].s64 = ctx.r[9].s64 + -10240;
	// 83266DA4: 4BA4317D  bl 0x82ca9f20
	ctx.lr = 0x83266DA8;
	sub_82CA9F20(ctx, base);
	// 83266DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266DB8 size=64
    let mut pc: u32 = 0x83266DB8;
    'dispatch: loop {
        match pc {
            0x83266DB8 => {
    //   block [0x83266DB8..0x83266DF8)
	// 83266DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266DCC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266DD0: 386AB894  addi r3, r10, -0x476c
	ctx.r[3].s64 = ctx.r[10].s64 + -18284;
	// 83266DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266DD8: 4AFC60F9  bl 0x8222ced0
	ctx.lr = 0x83266DDC;
	sub_8222CED0(ctx, base);
	// 83266DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266DE0: 3869D810  addi r3, r9, -0x27f0
	ctx.r[3].s64 = ctx.r[9].s64 + -10224;
	// 83266DE4: 4BA4313D  bl 0x82ca9f20
	ctx.lr = 0x83266DE8;
	sub_82CA9F20(ctx, base);
	// 83266DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266DF8 size=64
    let mut pc: u32 = 0x83266DF8;
    'dispatch: loop {
        match pc {
            0x83266DF8 => {
    //   block [0x83266DF8..0x83266E38)
	// 83266DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266E0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266E10: 386AB898  addi r3, r10, -0x4768
	ctx.r[3].s64 = ctx.r[10].s64 + -18280;
	// 83266E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266E18: 4AFC60B9  bl 0x8222ced0
	ctx.lr = 0x83266E1C;
	sub_8222CED0(ctx, base);
	// 83266E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266E20: 3869D820  addi r3, r9, -0x27e0
	ctx.r[3].s64 = ctx.r[9].s64 + -10208;
	// 83266E24: 4BA430FD  bl 0x82ca9f20
	ctx.lr = 0x83266E28;
	sub_82CA9F20(ctx, base);
	// 83266E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266E38 size=56
    let mut pc: u32 = 0x83266E38;
    'dispatch: loop {
        match pc {
            0x83266E38 => {
    //   block [0x83266E38..0x83266E70)
	// 83266E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266E44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266E48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266E4C: 386B35C0  addi r3, r11, 0x35c0
	ctx.r[3].s64 = ctx.r[11].s64 + 13760;
	// 83266E50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266E54: 4AF8CF05  bl 0x821f3d58
	ctx.lr = 0x83266E58;
	sub_821F3D58(ctx, base);
	// 83266E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266E5C: 906AB89C  stw r3, -0x4764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18276 as u32), ctx.r[3].u32 ) };
	// 83266E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266E70 size=56
    let mut pc: u32 = 0x83266E70;
    'dispatch: loop {
        match pc {
            0x83266E70 => {
    //   block [0x83266E70..0x83266EA8)
	// 83266E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266E80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266E84: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83266E88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266E8C: 4AF8CECD  bl 0x821f3d58
	ctx.lr = 0x83266E90;
	sub_821F3D58(ctx, base);
	// 83266E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266E94: 906AB8A0  stw r3, -0x4760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18272 as u32), ctx.r[3].u32 ) };
	// 83266E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266EA8 size=56
    let mut pc: u32 = 0x83266EA8;
    'dispatch: loop {
        match pc {
            0x83266EA8 => {
    //   block [0x83266EA8..0x83266EE0)
	// 83266EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266EB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266EB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266EBC: 386B35D4  addi r3, r11, 0x35d4
	ctx.r[3].s64 = ctx.r[11].s64 + 13780;
	// 83266EC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266EC4: 4AF8CE95  bl 0x821f3d58
	ctx.lr = 0x83266EC8;
	sub_821F3D58(ctx, base);
	// 83266EC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266ECC: 906AB8A4  stw r3, -0x475c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18268 as u32), ctx.r[3].u32 ) };
	// 83266ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266EE0 size=56
    let mut pc: u32 = 0x83266EE0;
    'dispatch: loop {
        match pc {
            0x83266EE0 => {
    //   block [0x83266EE0..0x83266F18)
	// 83266EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266EEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266EF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266EF4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83266EF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266EFC: 4AF8CE5D  bl 0x821f3d58
	ctx.lr = 0x83266F00;
	sub_821F3D58(ctx, base);
	// 83266F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F04: 906AB8A8  stw r3, -0x4758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18264 as u32), ctx.r[3].u32 ) };
	// 83266F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266F18 size=56
    let mut pc: u32 = 0x83266F18;
    'dispatch: loop {
        match pc {
            0x83266F18 => {
    //   block [0x83266F18..0x83266F50)
	// 83266F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266F24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266F28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266F2C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83266F30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266F34: 4AF8CE25  bl 0x821f3d58
	ctx.lr = 0x83266F38;
	sub_821F3D58(ctx, base);
	// 83266F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F3C: 906AB8AC  stw r3, -0x4754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18260 as u32), ctx.r[3].u32 ) };
	// 83266F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266F50 size=56
    let mut pc: u32 = 0x83266F50;
    'dispatch: loop {
        match pc {
            0x83266F50 => {
    //   block [0x83266F50..0x83266F88)
	// 83266F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266F5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83266F60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266F64: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 83266F68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266F6C: 4AF8CDED  bl 0x821f3d58
	ctx.lr = 0x83266F70;
	sub_821F3D58(ctx, base);
	// 83266F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F74: 906AB8B0  stw r3, -0x4750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18256 as u32), ctx.r[3].u32 ) };
	// 83266F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266F88 size=64
    let mut pc: u32 = 0x83266F88;
    'dispatch: loop {
        match pc {
            0x83266F88 => {
    //   block [0x83266F88..0x83266FC8)
	// 83266F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266F94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266F98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F9C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266FA0: 386AB8B4  addi r3, r10, -0x474c
	ctx.r[3].s64 = ctx.r[10].s64 + -18252;
	// 83266FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266FA8: 4AFC5F29  bl 0x8222ced0
	ctx.lr = 0x83266FAC;
	sub_8222CED0(ctx, base);
	// 83266FAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266FB0: 3869D830  addi r3, r9, -0x27d0
	ctx.r[3].s64 = ctx.r[9].s64 + -10192;
	// 83266FB4: 4BA42F6D  bl 0x82ca9f20
	ctx.lr = 0x83266FB8;
	sub_82CA9F20(ctx, base);
	// 83266FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266FC8 size=64
    let mut pc: u32 = 0x83266FC8;
    'dispatch: loop {
        match pc {
            0x83266FC8 => {
    //   block [0x83266FC8..0x83267008)
	// 83266FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266FD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266FDC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266FE0: 386AB8B8  addi r3, r10, -0x4748
	ctx.r[3].s64 = ctx.r[10].s64 + -18248;
	// 83266FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266FE8: 4AFC5EE9  bl 0x8222ced0
	ctx.lr = 0x83266FEC;
	sub_8222CED0(ctx, base);
	// 83266FEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266FF0: 3869D840  addi r3, r9, -0x27c0
	ctx.r[3].s64 = ctx.r[9].s64 + -10176;
	// 83266FF4: 4BA42F2D  bl 0x82ca9f20
	ctx.lr = 0x83266FF8;
	sub_82CA9F20(ctx, base);
	// 83266FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267008 size=64
    let mut pc: u32 = 0x83267008;
    'dispatch: loop {
        match pc {
            0x83267008 => {
    //   block [0x83267008..0x83267048)
	// 83267008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267014: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326701C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267020: 386AB8BC  addi r3, r10, -0x4744
	ctx.r[3].s64 = ctx.r[10].s64 + -18244;
	// 83267024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267028: 4AFC5EA9  bl 0x8222ced0
	ctx.lr = 0x8326702C;
	sub_8222CED0(ctx, base);
	// 8326702C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267030: 3869D850  addi r3, r9, -0x27b0
	ctx.r[3].s64 = ctx.r[9].s64 + -10160;
	// 83267034: 4BA42EED  bl 0x82ca9f20
	ctx.lr = 0x83267038;
	sub_82CA9F20(ctx, base);
	// 83267038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326703C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267048 size=56
    let mut pc: u32 = 0x83267048;
    'dispatch: loop {
        match pc {
            0x83267048 => {
    //   block [0x83267048..0x83267080)
	// 83267048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326704C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267054: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267058: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326705C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83267060: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267064: 4AF8CCF5  bl 0x821f3d58
	ctx.lr = 0x83267068;
	sub_821F3D58(ctx, base);
	// 83267068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326706C: 906AB8C0  stw r3, -0x4740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18240 as u32), ctx.r[3].u32 ) };
	// 83267070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326707C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267080 size=56
    let mut pc: u32 = 0x83267080;
    'dispatch: loop {
        match pc {
            0x83267080 => {
    //   block [0x83267080..0x832670B8)
	// 83267080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326708C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267090: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267094: 386B365C  addi r3, r11, 0x365c
	ctx.r[3].s64 = ctx.r[11].s64 + 13916;
	// 83267098: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326709C: 4AF8CCBD  bl 0x821f3d58
	ctx.lr = 0x832670A0;
	sub_821F3D58(ctx, base);
	// 832670A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832670A4: 906AB8C4  stw r3, -0x473c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18236 as u32), ctx.r[3].u32 ) };
	// 832670A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832670AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832670B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832670B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832670B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832670B8 size=64
    let mut pc: u32 = 0x832670B8;
    'dispatch: loop {
        match pc {
            0x832670B8 => {
    //   block [0x832670B8..0x832670F8)
	// 832670B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832670BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832670C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832670C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832670C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832670CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832670D0: 386AB8C8  addi r3, r10, -0x4738
	ctx.r[3].s64 = ctx.r[10].s64 + -18232;
	// 832670D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832670D8: 4AFC5DF9  bl 0x8222ced0
	ctx.lr = 0x832670DC;
	sub_8222CED0(ctx, base);
	// 832670DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832670E0: 3869D860  addi r3, r9, -0x27a0
	ctx.r[3].s64 = ctx.r[9].s64 + -10144;
	// 832670E4: 4BA42E3D  bl 0x82ca9f20
	ctx.lr = 0x832670E8;
	sub_82CA9F20(ctx, base);
	// 832670E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832670EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832670F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832670F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832670F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832670F8 size=64
    let mut pc: u32 = 0x832670F8;
    'dispatch: loop {
        match pc {
            0x832670F8 => {
    //   block [0x832670F8..0x83267138)
	// 832670F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832670FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326710C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83267110: 386AB8CC  addi r3, r10, -0x4734
	ctx.r[3].s64 = ctx.r[10].s64 + -18228;
	// 83267114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267118: 4AFC5DB9  bl 0x8222ced0
	ctx.lr = 0x8326711C;
	sub_8222CED0(ctx, base);
	// 8326711C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267120: 3869D870  addi r3, r9, -0x2790
	ctx.r[3].s64 = ctx.r[9].s64 + -10128;
	// 83267124: 4BA42DFD  bl 0x82ca9f20
	ctx.lr = 0x83267128;
	sub_82CA9F20(ctx, base);
	// 83267128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326712C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267138 size=64
    let mut pc: u32 = 0x83267138;
    'dispatch: loop {
        match pc {
            0x83267138 => {
    //   block [0x83267138..0x83267178)
	// 83267138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326714C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267150: 386AB8D0  addi r3, r10, -0x4730
	ctx.r[3].s64 = ctx.r[10].s64 + -18224;
	// 83267154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267158: 4AFC5D79  bl 0x8222ced0
	ctx.lr = 0x8326715C;
	sub_8222CED0(ctx, base);
	// 8326715C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267160: 3869D880  addi r3, r9, -0x2780
	ctx.r[3].s64 = ctx.r[9].s64 + -10112;
	// 83267164: 4BA42DBD  bl 0x82ca9f20
	ctx.lr = 0x83267168;
	sub_82CA9F20(ctx, base);
	// 83267168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326716C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267178 size=56
    let mut pc: u32 = 0x83267178;
    'dispatch: loop {
        match pc {
            0x83267178 => {
    //   block [0x83267178..0x832671B0)
	// 83267178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326717C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326718C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83267190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267194: 4AF8CBC5  bl 0x821f3d58
	ctx.lr = 0x83267198;
	sub_821F3D58(ctx, base);
	// 83267198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326719C: 906AB8D4  stw r3, -0x472c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18220 as u32), ctx.r[3].u32 ) };
	// 832671A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832671A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832671A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832671AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832671B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832671B0 size=56
    let mut pc: u32 = 0x832671B0;
    'dispatch: loop {
        match pc {
            0x832671B0 => {
    //   block [0x832671B0..0x832671E8)
	// 832671B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832671B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832671B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832671BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832671C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832671C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832671C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832671CC: 4AF8CB8D  bl 0x821f3d58
	ctx.lr = 0x832671D0;
	sub_821F3D58(ctx, base);
	// 832671D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832671D4: 906AB8D8  stw r3, -0x4728(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18216 as u32), ctx.r[3].u32 ) };
	// 832671D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832671DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832671E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832671E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832671E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832671E8 size=56
    let mut pc: u32 = 0x832671E8;
    'dispatch: loop {
        match pc {
            0x832671E8 => {
    //   block [0x832671E8..0x83267220)
	// 832671E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832671EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832671F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832671F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832671F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832671FC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83267200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267204: 4AF8CB55  bl 0x821f3d58
	ctx.lr = 0x83267208;
	sub_821F3D58(ctx, base);
	// 83267208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326720C: 906AB8DC  stw r3, -0x4724(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18212 as u32), ctx.r[3].u32 ) };
	// 83267210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326721C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267220 size=56
    let mut pc: u32 = 0x83267220;
    'dispatch: loop {
        match pc {
            0x83267220 => {
    //   block [0x83267220..0x83267258)
	// 83267220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326722C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267234: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83267238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326723C: 4AF8CB1D  bl 0x821f3d58
	ctx.lr = 0x83267240;
	sub_821F3D58(ctx, base);
	// 83267240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267244: 906AB8E0  stw r3, -0x4720(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18208 as u32), ctx.r[3].u32 ) };
	// 83267248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326724C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267258 size=56
    let mut pc: u32 = 0x83267258;
    'dispatch: loop {
        match pc {
            0x83267258 => {
    //   block [0x83267258..0x83267290)
	// 83267258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326725C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267264: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326726C: 386B3C80  addi r3, r11, 0x3c80
	ctx.r[3].s64 = ctx.r[11].s64 + 15488;
	// 83267270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267274: 4AF8CAE5  bl 0x821f3d58
	ctx.lr = 0x83267278;
	sub_821F3D58(ctx, base);
	// 83267278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326727C: 906AB8E4  stw r3, -0x471c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18204 as u32), ctx.r[3].u32 ) };
	// 83267280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326728C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267290 size=64
    let mut pc: u32 = 0x83267290;
    'dispatch: loop {
        match pc {
            0x83267290 => {
    //   block [0x83267290..0x832672D0)
	// 83267290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326729C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832672A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832672A4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832672A8: 386AB8E8  addi r3, r10, -0x4718
	ctx.r[3].s64 = ctx.r[10].s64 + -18200;
	// 832672AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832672B0: 4AFC5C21  bl 0x8222ced0
	ctx.lr = 0x832672B4;
	sub_8222CED0(ctx, base);
	// 832672B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832672B8: 3869D890  addi r3, r9, -0x2770
	ctx.r[3].s64 = ctx.r[9].s64 + -10096;
	// 832672BC: 4BA42C65  bl 0x82ca9f20
	ctx.lr = 0x832672C0;
	sub_82CA9F20(ctx, base);
	// 832672C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832672C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832672C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832672CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832672D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832672D0 size=64
    let mut pc: u32 = 0x832672D0;
    'dispatch: loop {
        match pc {
            0x832672D0 => {
    //   block [0x832672D0..0x83267310)
	// 832672D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832672D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832672D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832672DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832672E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832672E4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832672E8: 386AB8EC  addi r3, r10, -0x4714
	ctx.r[3].s64 = ctx.r[10].s64 + -18196;
	// 832672EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832672F0: 4AFC5BE1  bl 0x8222ced0
	ctx.lr = 0x832672F4;
	sub_8222CED0(ctx, base);
	// 832672F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832672F8: 3869D8A0  addi r3, r9, -0x2760
	ctx.r[3].s64 = ctx.r[9].s64 + -10080;
	// 832672FC: 4BA42C25  bl 0x82ca9f20
	ctx.lr = 0x83267300;
	sub_82CA9F20(ctx, base);
	// 83267300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267310 size=64
    let mut pc: u32 = 0x83267310;
    'dispatch: loop {
        match pc {
            0x83267310 => {
    //   block [0x83267310..0x83267350)
	// 83267310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326731C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267324: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267328: 386AB8F0  addi r3, r10, -0x4710
	ctx.r[3].s64 = ctx.r[10].s64 + -18192;
	// 8326732C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267330: 4AFC5BA1  bl 0x8222ced0
	ctx.lr = 0x83267334;
	sub_8222CED0(ctx, base);
	// 83267334: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267338: 3869D8B0  addi r3, r9, -0x2750
	ctx.r[3].s64 = ctx.r[9].s64 + -10064;
	// 8326733C: 4BA42BE5  bl 0x82ca9f20
	ctx.lr = 0x83267340;
	sub_82CA9F20(ctx, base);
	// 83267340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326734C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267350 size=64
    let mut pc: u32 = 0x83267350;
    'dispatch: loop {
        match pc {
            0x83267350 => {
    //   block [0x83267350..0x83267390)
	// 83267350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326735C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267364: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267368: 386AB8F4  addi r3, r10, -0x470c
	ctx.r[3].s64 = ctx.r[10].s64 + -18188;
	// 8326736C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267370: 4AFC5B61  bl 0x8222ced0
	ctx.lr = 0x83267374;
	sub_8222CED0(ctx, base);
	// 83267374: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267378: 3869D8C0  addi r3, r9, -0x2740
	ctx.r[3].s64 = ctx.r[9].s64 + -10048;
	// 8326737C: 4BA42BA5  bl 0x82ca9f20
	ctx.lr = 0x83267380;
	sub_82CA9F20(ctx, base);
	// 83267380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326738C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267390 size=64
    let mut pc: u32 = 0x83267390;
    'dispatch: loop {
        match pc {
            0x83267390 => {
    //   block [0x83267390..0x832673D0)
	// 83267390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326739C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832673A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832673A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832673A8: 386AB8F8  addi r3, r10, -0x4708
	ctx.r[3].s64 = ctx.r[10].s64 + -18184;
	// 832673AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832673B0: 4AFC5B21  bl 0x8222ced0
	ctx.lr = 0x832673B4;
	sub_8222CED0(ctx, base);
	// 832673B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832673B8: 3869D8D0  addi r3, r9, -0x2730
	ctx.r[3].s64 = ctx.r[9].s64 + -10032;
	// 832673BC: 4BA42B65  bl 0x82ca9f20
	ctx.lr = 0x832673C0;
	sub_82CA9F20(ctx, base);
	// 832673C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832673C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832673C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832673CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832673D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832673D0 size=64
    let mut pc: u32 = 0x832673D0;
    'dispatch: loop {
        match pc {
            0x832673D0 => {
    //   block [0x832673D0..0x83267410)
	// 832673D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832673D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832673D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832673DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832673E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832673E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832673E8: 386AB8FC  addi r3, r10, -0x4704
	ctx.r[3].s64 = ctx.r[10].s64 + -18180;
	// 832673EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832673F0: 4AFC5AE1  bl 0x8222ced0
	ctx.lr = 0x832673F4;
	sub_8222CED0(ctx, base);
	// 832673F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832673F8: 3869D8E0  addi r3, r9, -0x2720
	ctx.r[3].s64 = ctx.r[9].s64 + -10016;
	// 832673FC: 4BA42B25  bl 0x82ca9f20
	ctx.lr = 0x83267400;
	sub_82CA9F20(ctx, base);
	// 83267400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326740C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267410 size=64
    let mut pc: u32 = 0x83267410;
    'dispatch: loop {
        match pc {
            0x83267410 => {
    //   block [0x83267410..0x83267450)
	// 83267410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326741C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267424: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267428: 386AB900  addi r3, r10, -0x4700
	ctx.r[3].s64 = ctx.r[10].s64 + -18176;
	// 8326742C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267430: 4AFC5AA1  bl 0x8222ced0
	ctx.lr = 0x83267434;
	sub_8222CED0(ctx, base);
	// 83267434: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267438: 3869D8F0  addi r3, r9, -0x2710
	ctx.r[3].s64 = ctx.r[9].s64 + -10000;
	// 8326743C: 4BA42AE5  bl 0x82ca9f20
	ctx.lr = 0x83267440;
	sub_82CA9F20(ctx, base);
	// 83267440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326744C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267450 size=64
    let mut pc: u32 = 0x83267450;
    'dispatch: loop {
        match pc {
            0x83267450 => {
    //   block [0x83267450..0x83267490)
	// 83267450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326745C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267464: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83267468: 386AB904  addi r3, r10, -0x46fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18172;
	// 8326746C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267470: 4AFC5A61  bl 0x8222ced0
	ctx.lr = 0x83267474;
	sub_8222CED0(ctx, base);
	// 83267474: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267478: 3869D900  addi r3, r9, -0x2700
	ctx.r[3].s64 = ctx.r[9].s64 + -9984;
	// 8326747C: 4BA42AA5  bl 0x82ca9f20
	ctx.lr = 0x83267480;
	sub_82CA9F20(ctx, base);
	// 83267480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326748C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267490 size=64
    let mut pc: u32 = 0x83267490;
    'dispatch: loop {
        match pc {
            0x83267490 => {
    //   block [0x83267490..0x832674D0)
	// 83267490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326749C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832674A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832674A4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832674A8: 386AB908  addi r3, r10, -0x46f8
	ctx.r[3].s64 = ctx.r[10].s64 + -18168;
	// 832674AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832674B0: 4AFC5A21  bl 0x8222ced0
	ctx.lr = 0x832674B4;
	sub_8222CED0(ctx, base);
	// 832674B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832674B8: 3869D910  addi r3, r9, -0x26f0
	ctx.r[3].s64 = ctx.r[9].s64 + -9968;
	// 832674BC: 4BA42A65  bl 0x82ca9f20
	ctx.lr = 0x832674C0;
	sub_82CA9F20(ctx, base);
	// 832674C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832674C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832674C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832674CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832674D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832674D0 size=56
    let mut pc: u32 = 0x832674D0;
    'dispatch: loop {
        match pc {
            0x832674D0 => {
    //   block [0x832674D0..0x83267508)
	// 832674D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832674D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832674D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832674DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832674E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832674E4: 386B430C  addi r3, r11, 0x430c
	ctx.r[3].s64 = ctx.r[11].s64 + 17164;
	// 832674E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832674EC: 4AF8C86D  bl 0x821f3d58
	ctx.lr = 0x832674F0;
	sub_821F3D58(ctx, base);
	// 832674F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832674F4: 906AB90C  stw r3, -0x46f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18164 as u32), ctx.r[3].u32 ) };
	// 832674F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832674FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267508 size=56
    let mut pc: u32 = 0x83267508;
    'dispatch: loop {
        match pc {
            0x83267508 => {
    //   block [0x83267508..0x83267540)
	// 83267508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326750C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267514: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326751C: 386B4320  addi r3, r11, 0x4320
	ctx.r[3].s64 = ctx.r[11].s64 + 17184;
	// 83267520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267524: 4AF8C835  bl 0x821f3d58
	ctx.lr = 0x83267528;
	sub_821F3D58(ctx, base);
	// 83267528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326752C: 906AB910  stw r3, -0x46f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18160 as u32), ctx.r[3].u32 ) };
	// 83267530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326753C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267540 size=64
    let mut pc: u32 = 0x83267540;
    'dispatch: loop {
        match pc {
            0x83267540 => {
    //   block [0x83267540..0x83267580)
	// 83267540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326754C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267554: 388BA3AC  addi r4, r11, -0x5c54
	ctx.r[4].s64 = ctx.r[11].s64 + -23636;
	// 83267558: 386AB914  addi r3, r10, -0x46ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18156;
	// 8326755C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267560: 4AFC5971  bl 0x8222ced0
	ctx.lr = 0x83267564;
	sub_8222CED0(ctx, base);
	// 83267564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267568: 3869D920  addi r3, r9, -0x26e0
	ctx.r[3].s64 = ctx.r[9].s64 + -9952;
	// 8326756C: 4BA429B5  bl 0x82ca9f20
	ctx.lr = 0x83267570;
	sub_82CA9F20(ctx, base);
	// 83267570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326757C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267580 size=64
    let mut pc: u32 = 0x83267580;
    'dispatch: loop {
        match pc {
            0x83267580 => {
    //   block [0x83267580..0x832675C0)
	// 83267580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326758C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267594: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267598: 386AB918  addi r3, r10, -0x46e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18152;
	// 8326759C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832675A0: 4AFC5931  bl 0x8222ced0
	ctx.lr = 0x832675A4;
	sub_8222CED0(ctx, base);
	// 832675A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832675A8: 3869D930  addi r3, r9, -0x26d0
	ctx.r[3].s64 = ctx.r[9].s64 + -9936;
	// 832675AC: 4BA42975  bl 0x82ca9f20
	ctx.lr = 0x832675B0;
	sub_82CA9F20(ctx, base);
	// 832675B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832675B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832675B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832675BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832675C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832675C0 size=64
    let mut pc: u32 = 0x832675C0;
    'dispatch: loop {
        match pc {
            0x832675C0 => {
    //   block [0x832675C0..0x83267600)
	// 832675C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832675C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832675C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832675CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832675D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832675D4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832675D8: 386AB91C  addi r3, r10, -0x46e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18148;
	// 832675DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832675E0: 4AFC58F1  bl 0x8222ced0
	ctx.lr = 0x832675E4;
	sub_8222CED0(ctx, base);
	// 832675E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832675E8: 3869D940  addi r3, r9, -0x26c0
	ctx.r[3].s64 = ctx.r[9].s64 + -9920;
	// 832675EC: 4BA42935  bl 0x82ca9f20
	ctx.lr = 0x832675F0;
	sub_82CA9F20(ctx, base);
	// 832675F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832675F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832675F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832675FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267600 size=64
    let mut pc: u32 = 0x83267600;
    'dispatch: loop {
        match pc {
            0x83267600 => {
    //   block [0x83267600..0x83267640)
	// 83267600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326760C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267614: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267618: 386AB920  addi r3, r10, -0x46e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18144;
	// 8326761C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267620: 4AFC58B1  bl 0x8222ced0
	ctx.lr = 0x83267624;
	sub_8222CED0(ctx, base);
	// 83267624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267628: 3869D950  addi r3, r9, -0x26b0
	ctx.r[3].s64 = ctx.r[9].s64 + -9904;
	// 8326762C: 4BA428F5  bl 0x82ca9f20
	ctx.lr = 0x83267630;
	sub_82CA9F20(ctx, base);
	// 83267630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326763C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267640 size=64
    let mut pc: u32 = 0x83267640;
    'dispatch: loop {
        match pc {
            0x83267640 => {
    //   block [0x83267640..0x83267680)
	// 83267640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326764C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267654: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267658: 386AB924  addi r3, r10, -0x46dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18140;
	// 8326765C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267660: 4AFC5871  bl 0x8222ced0
	ctx.lr = 0x83267664;
	sub_8222CED0(ctx, base);
	// 83267664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267668: 3869D970  addi r3, r9, -0x2690
	ctx.r[3].s64 = ctx.r[9].s64 + -9872;
	// 8326766C: 4BA428B5  bl 0x82ca9f20
	ctx.lr = 0x83267670;
	sub_82CA9F20(ctx, base);
	// 83267670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326767C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267680 size=64
    let mut pc: u32 = 0x83267680;
    'dispatch: loop {
        match pc {
            0x83267680 => {
    //   block [0x83267680..0x832676C0)
	// 83267680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326768C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267694: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83267698: 386AB928  addi r3, r10, -0x46d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18136;
	// 8326769C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832676A0: 4AFC5831  bl 0x8222ced0
	ctx.lr = 0x832676A4;
	sub_8222CED0(ctx, base);
	// 832676A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832676A8: 3869D980  addi r3, r9, -0x2680
	ctx.r[3].s64 = ctx.r[9].s64 + -9856;
	// 832676AC: 4BA42875  bl 0x82ca9f20
	ctx.lr = 0x832676B0;
	sub_82CA9F20(ctx, base);
	// 832676B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832676B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832676B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832676BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832676C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832676C0 size=64
    let mut pc: u32 = 0x832676C0;
    'dispatch: loop {
        match pc {
            0x832676C0 => {
    //   block [0x832676C0..0x83267700)
	// 832676C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832676C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832676C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832676CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832676D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832676D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832676D8: 386AB92C  addi r3, r10, -0x46d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18132;
	// 832676DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832676E0: 4AFC57F1  bl 0x8222ced0
	ctx.lr = 0x832676E4;
	sub_8222CED0(ctx, base);
	// 832676E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832676E8: 3869D990  addi r3, r9, -0x2670
	ctx.r[3].s64 = ctx.r[9].s64 + -9840;
	// 832676EC: 4BA42835  bl 0x82ca9f20
	ctx.lr = 0x832676F0;
	sub_82CA9F20(ctx, base);
	// 832676F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832676F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832676F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832676FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267700 size=156
    let mut pc: u32 = 0x83267700;
    'dispatch: loop {
        match pc {
            0x83267700 => {
    //   block [0x83267700..0x8326779C)
	// 83267700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326770C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267710: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83267714: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83267718: 3BEBB930  addi r31, r11, -0x46d0
	ctx.r[31].s64 = ctx.r[11].s64 + -18128;
	// 8326771C: 388A8A00  addi r4, r10, -0x7600
	ctx.r[4].s64 = ctx.r[10].s64 + -30208;
	// 83267720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83267724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267728: 4AFC57A9  bl 0x8222ced0
	ctx.lr = 0x8326772C;
	sub_8222CED0(ctx, base);
	// 8326772C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83267730: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267734: 38898A18  addi r4, r9, -0x75e8
	ctx.r[4].s64 = ctx.r[9].s64 + -30184;
	// 83267738: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8326773C: 4AFC5795  bl 0x8222ced0
	ctx.lr = 0x83267740;
	sub_8222CED0(ctx, base);
	// 83267740: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 83267744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267748: 38888A24  addi r4, r8, -0x75dc
	ctx.r[4].s64 = ctx.r[8].s64 + -30172;
	// 8326774C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83267750: 4AFC5781  bl 0x8222ced0
	ctx.lr = 0x83267754;
	sub_8222CED0(ctx, base);
	// 83267754: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 83267758: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326775C: 38878A38  addi r4, r7, -0x75c8
	ctx.r[4].s64 = ctx.r[7].s64 + -30152;
	// 83267760: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83267764: 4AFC576D  bl 0x8222ced0
	ctx.lr = 0x83267768;
	sub_8222CED0(ctx, base);
	// 83267768: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8326776C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267770: 38868A48  addi r4, r6, -0x75b8
	ctx.r[4].s64 = ctx.r[6].s64 + -30136;
	// 83267774: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83267778: 4AFC5759  bl 0x8222ced0
	ctx.lr = 0x8326777C;
	sub_8222CED0(ctx, base);
	// 8326777C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83267780: 3865D9A0  addi r3, r5, -0x2660
	ctx.r[3].s64 = ctx.r[5].s64 + -9824;
	// 83267784: 4BA4279D  bl 0x82ca9f20
	ctx.lr = 0x83267788;
	sub_82CA9F20(ctx, base);
	// 83267788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326778C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83267798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832677A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832677A0 size=32
    let mut pc: u32 = 0x832677A0;
    'dispatch: loop {
        match pc {
            0x832677A0 => {
    //   block [0x832677A0..0x832677C0)
	// 832677A0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832677A4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832677A8: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 832677AC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 832677B0: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 832677B4: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832677B8: 916AB944  stw r11, -0x46bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18108 as u32), ctx.r[11].u32 ) };
	// 832677BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832677C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832677C0 size=44
    let mut pc: u32 = 0x832677C0;
    'dispatch: loop {
        match pc {
            0x832677C0 => {
    //   block [0x832677C0..0x832677EC)
	// 832677C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832677C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832677C8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832677CC: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 832677D0: C9AA0DF0  lfd f13, 0xdf0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3568 as u32) ) };
	// 832677D4: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 832677D8: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 832677DC: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 832677E0: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832677E4: 9169B948  stw r11, -0x46b8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18104 as u32), ctx.r[11].u32 ) };
	// 832677E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832677F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832677F0 size=44
    let mut pc: u32 = 0x832677F0;
    'dispatch: loop {
        match pc {
            0x832677F0 => {
    //   block [0x832677F0..0x8326781C)
	// 832677F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832677F4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832677F8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832677FC: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83267800: C9AA9630  lfd f13, -0x69d0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-27088 as u32) ) };
	// 83267804: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83267808: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326780C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83267810: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83267814: 9169B94C  stw r11, -0x46b4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18100 as u32), ctx.r[11].u32 ) };
	// 83267818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83267820 size=44
    let mut pc: u32 = 0x83267820;
    'dispatch: loop {
        match pc {
            0x83267820 => {
    //   block [0x83267820..0x8326784C)
	// 83267820: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83267824: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83267828: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326782C: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83267830: C9AA11E8  lfd f13, 0x11e8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4584 as u32) ) };
	// 83267834: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83267838: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326783C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83267840: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83267844: 9169B950  stw r11, -0x46b0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18096 as u32), ctx.r[11].u32 ) };
	// 83267848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83267850 size=44
    let mut pc: u32 = 0x83267850;
    'dispatch: loop {
        match pc {
            0x83267850 => {
    //   block [0x83267850..0x8326787C)
	// 83267850: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83267854: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83267858: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326785C: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83267860: C9AA9610  lfd f13, -0x69f0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-27120 as u32) ) };
	// 83267864: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83267868: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326786C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83267870: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83267874: 9169B954  stw r11, -0x46ac(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18092 as u32), ctx.r[11].u32 ) };
	// 83267878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267880 size=56
    let mut pc: u32 = 0x83267880;
    'dispatch: loop {
        match pc {
            0x83267880 => {
    //   block [0x83267880..0x832678B8)
	// 83267880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326788C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83267890: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267894: 386B0F58  addi r3, r11, 0xf58
	ctx.r[3].s64 = ctx.r[11].s64 + 3928;
	// 83267898: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326789C: 4AF8C4BD  bl 0x821f3d58
	ctx.lr = 0x832678A0;
	sub_821F3D58(ctx, base);
	// 832678A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832678A4: 906AB958  stw r3, -0x46a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18088 as u32), ctx.r[3].u32 ) };
	// 832678A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832678AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832678B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832678B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832678B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832678B8 size=56
    let mut pc: u32 = 0x832678B8;
    'dispatch: loop {
        match pc {
            0x832678B8 => {
    //   block [0x832678B8..0x832678F0)
	// 832678B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832678BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832678C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832678C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832678C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832678CC: 386B5BB4  addi r3, r11, 0x5bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 23476;
	// 832678D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832678D4: 4AF8C485  bl 0x821f3d58
	ctx.lr = 0x832678D8;
	sub_821F3D58(ctx, base);
	// 832678D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832678DC: 906AB95C  stw r3, -0x46a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18084 as u32), ctx.r[3].u32 ) };
	// 832678E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832678E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832678E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832678EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832678F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832678F0 size=56
    let mut pc: u32 = 0x832678F0;
    'dispatch: loop {
        match pc {
            0x832678F0 => {
    //   block [0x832678F0..0x83267928)
	// 832678F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832678F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832678F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832678FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267900: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267904: 386B0484  addi r3, r11, 0x484
	ctx.r[3].s64 = ctx.r[11].s64 + 1156;
	// 83267908: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326790C: 4AF8C44D  bl 0x821f3d58
	ctx.lr = 0x83267910;
	sub_821F3D58(ctx, base);
	// 83267910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267914: 906AB960  stw r3, -0x46a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18080 as u32), ctx.r[3].u32 ) };
	// 83267918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326791C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267928 size=56
    let mut pc: u32 = 0x83267928;
    'dispatch: loop {
        match pc {
            0x83267928 => {
    //   block [0x83267928..0x83267960)
	// 83267928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326792C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267934: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267938: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326793C: 386B5BC8  addi r3, r11, 0x5bc8
	ctx.r[3].s64 = ctx.r[11].s64 + 23496;
	// 83267940: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267944: 4AF8C415  bl 0x821f3d58
	ctx.lr = 0x83267948;
	sub_821F3D58(ctx, base);
	// 83267948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326794C: 906AB964  stw r3, -0x469c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18076 as u32), ctx.r[3].u32 ) };
	// 83267950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326795C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267960 size=56
    let mut pc: u32 = 0x83267960;
    'dispatch: loop {
        match pc {
            0x83267960 => {
    //   block [0x83267960..0x83267998)
	// 83267960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326796C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267970: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267974: 386B5BDC  addi r3, r11, 0x5bdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23516;
	// 83267978: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326797C: 4AF8C3DD  bl 0x821f3d58
	ctx.lr = 0x83267980;
	sub_821F3D58(ctx, base);
	// 83267980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267984: 906AB968  stw r3, -0x4698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18072 as u32), ctx.r[3].u32 ) };
	// 83267988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326798C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267998 size=56
    let mut pc: u32 = 0x83267998;
    'dispatch: loop {
        match pc {
            0x83267998 => {
    //   block [0x83267998..0x832679D0)
	// 83267998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326799C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832679A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832679A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832679A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832679AC: 386B5BE8  addi r3, r11, 0x5be8
	ctx.r[3].s64 = ctx.r[11].s64 + 23528;
	// 832679B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832679B4: 4AF8C3A5  bl 0x821f3d58
	ctx.lr = 0x832679B8;
	sub_821F3D58(ctx, base);
	// 832679B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832679BC: 906AB96C  stw r3, -0x4694(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18068 as u32), ctx.r[3].u32 ) };
	// 832679C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832679C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832679C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832679CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832679D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832679D0 size=56
    let mut pc: u32 = 0x832679D0;
    'dispatch: loop {
        match pc {
            0x832679D0 => {
    //   block [0x832679D0..0x83267A08)
	// 832679D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832679D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832679D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832679DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832679E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832679E4: 386B5C04  addi r3, r11, 0x5c04
	ctx.r[3].s64 = ctx.r[11].s64 + 23556;
	// 832679E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832679EC: 4AF8C36D  bl 0x821f3d58
	ctx.lr = 0x832679F0;
	sub_821F3D58(ctx, base);
	// 832679F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832679F4: 906AB970  stw r3, -0x4690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18064 as u32), ctx.r[3].u32 ) };
	// 832679F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832679FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267A08 size=56
    let mut pc: u32 = 0x83267A08;
    'dispatch: loop {
        match pc {
            0x83267A08 => {
    //   block [0x83267A08..0x83267A40)
	// 83267A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267A10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267A14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267A18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267A1C: 386B29C8  addi r3, r11, 0x29c8
	ctx.r[3].s64 = ctx.r[11].s64 + 10696;
	// 83267A20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267A24: 4AF8C335  bl 0x821f3d58
	ctx.lr = 0x83267A28;
	sub_821F3D58(ctx, base);
	// 83267A28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267A2C: 906AB974  stw r3, -0x468c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18060 as u32), ctx.r[3].u32 ) };
	// 83267A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267A40 size=56
    let mut pc: u32 = 0x83267A40;
    'dispatch: loop {
        match pc {
            0x83267A40 => {
    //   block [0x83267A40..0x83267A78)
	// 83267A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267A4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267A50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267A54: 386BF11C  addi r3, r11, -0xee4
	ctx.r[3].s64 = ctx.r[11].s64 + -3812;
	// 83267A58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267A5C: 4AF8C2FD  bl 0x821f3d58
	ctx.lr = 0x83267A60;
	sub_821F3D58(ctx, base);
	// 83267A60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267A64: 906AB978  stw r3, -0x4688(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18056 as u32), ctx.r[3].u32 ) };
	// 83267A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267A78 size=56
    let mut pc: u32 = 0x83267A78;
    'dispatch: loop {
        match pc {
            0x83267A78 => {
    //   block [0x83267A78..0x83267AB0)
	// 83267A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267A84: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267A88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267A8C: 386B8C24  addi r3, r11, -0x73dc
	ctx.r[3].s64 = ctx.r[11].s64 + -29660;
	// 83267A90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267A94: 4AF8C2C5  bl 0x821f3d58
	ctx.lr = 0x83267A98;
	sub_821F3D58(ctx, base);
	// 83267A98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267A9C: 906AB97C  stw r3, -0x4684(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18052 as u32), ctx.r[3].u32 ) };
	// 83267AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267AB0 size=56
    let mut pc: u32 = 0x83267AB0;
    'dispatch: loop {
        match pc {
            0x83267AB0 => {
    //   block [0x83267AB0..0x83267AE8)
	// 83267AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267ABC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267AC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267AC4: 386B5C1C  addi r3, r11, 0x5c1c
	ctx.r[3].s64 = ctx.r[11].s64 + 23580;
	// 83267AC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267ACC: 4AF8C28D  bl 0x821f3d58
	ctx.lr = 0x83267AD0;
	sub_821F3D58(ctx, base);
	// 83267AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267AD4: 906AB980  stw r3, -0x4680(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18048 as u32), ctx.r[3].u32 ) };
	// 83267AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267AE8 size=56
    let mut pc: u32 = 0x83267AE8;
    'dispatch: loop {
        match pc {
            0x83267AE8 => {
    //   block [0x83267AE8..0x83267B20)
	// 83267AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267AF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267AF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267AFC: 386B5C30  addi r3, r11, 0x5c30
	ctx.r[3].s64 = ctx.r[11].s64 + 23600;
	// 83267B00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267B04: 4AF8C255  bl 0x821f3d58
	ctx.lr = 0x83267B08;
	sub_821F3D58(ctx, base);
	// 83267B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267B0C: 906AB984  stw r3, -0x467c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18044 as u32), ctx.r[3].u32 ) };
	// 83267B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267B20 size=56
    let mut pc: u32 = 0x83267B20;
    'dispatch: loop {
        match pc {
            0x83267B20 => {
    //   block [0x83267B20..0x83267B58)
	// 83267B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267B2C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267B30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267B34: 386BF0FC  addi r3, r11, -0xf04
	ctx.r[3].s64 = ctx.r[11].s64 + -3844;
	// 83267B38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267B3C: 4AF8C21D  bl 0x821f3d58
	ctx.lr = 0x83267B40;
	sub_821F3D58(ctx, base);
	// 83267B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267B44: 906AB988  stw r3, -0x4678(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18040 as u32), ctx.r[3].u32 ) };
	// 83267B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267B58 size=56
    let mut pc: u32 = 0x83267B58;
    'dispatch: loop {
        match pc {
            0x83267B58 => {
    //   block [0x83267B58..0x83267B90)
	// 83267B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267B64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267B68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267B6C: 386BE9C4  addi r3, r11, -0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + -5692;
	// 83267B70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267B74: 4AF8C1E5  bl 0x821f3d58
	ctx.lr = 0x83267B78;
	sub_821F3D58(ctx, base);
	// 83267B78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267B7C: 906AB98C  stw r3, -0x4674(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18036 as u32), ctx.r[3].u32 ) };
	// 83267B80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267B90 size=56
    let mut pc: u32 = 0x83267B90;
    'dispatch: loop {
        match pc {
            0x83267B90 => {
    //   block [0x83267B90..0x83267BC8)
	// 83267B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267B9C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267BA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267BA4: 386B8EB4  addi r3, r11, -0x714c
	ctx.r[3].s64 = ctx.r[11].s64 + -29004;
	// 83267BA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267BAC: 4AF8C1AD  bl 0x821f3d58
	ctx.lr = 0x83267BB0;
	sub_821F3D58(ctx, base);
	// 83267BB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267BB4: 906AB990  stw r3, -0x4670(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18032 as u32), ctx.r[3].u32 ) };
	// 83267BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267BC8 size=56
    let mut pc: u32 = 0x83267BC8;
    'dispatch: loop {
        match pc {
            0x83267BC8 => {
    //   block [0x83267BC8..0x83267C00)
	// 83267BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267BD4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267BD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267BDC: 386B5C48  addi r3, r11, 0x5c48
	ctx.r[3].s64 = ctx.r[11].s64 + 23624;
	// 83267BE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267BE4: 4AF8C175  bl 0x821f3d58
	ctx.lr = 0x83267BE8;
	sub_821F3D58(ctx, base);
	// 83267BE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267BEC: 906AB994  stw r3, -0x466c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18028 as u32), ctx.r[3].u32 ) };
	// 83267BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267C00 size=56
    let mut pc: u32 = 0x83267C00;
    'dispatch: loop {
        match pc {
            0x83267C00 => {
    //   block [0x83267C00..0x83267C38)
	// 83267C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267C0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267C10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267C14: 386B5C60  addi r3, r11, 0x5c60
	ctx.r[3].s64 = ctx.r[11].s64 + 23648;
	// 83267C18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267C1C: 4AF8C13D  bl 0x821f3d58
	ctx.lr = 0x83267C20;
	sub_821F3D58(ctx, base);
	// 83267C20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267C24: 906AB998  stw r3, -0x4668(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18024 as u32), ctx.r[3].u32 ) };
	// 83267C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267C38 size=56
    let mut pc: u32 = 0x83267C38;
    'dispatch: loop {
        match pc {
            0x83267C38 => {
    //   block [0x83267C38..0x83267C70)
	// 83267C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267C44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267C48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267C4C: 386B5C78  addi r3, r11, 0x5c78
	ctx.r[3].s64 = ctx.r[11].s64 + 23672;
	// 83267C50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267C54: 4AF8C105  bl 0x821f3d58
	ctx.lr = 0x83267C58;
	sub_821F3D58(ctx, base);
	// 83267C58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267C5C: 906AB99C  stw r3, -0x4664(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18020 as u32), ctx.r[3].u32 ) };
	// 83267C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267C70 size=56
    let mut pc: u32 = 0x83267C70;
    'dispatch: loop {
        match pc {
            0x83267C70 => {
    //   block [0x83267C70..0x83267CA8)
	// 83267C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267C7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267C80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267C84: 386B5C94  addi r3, r11, 0x5c94
	ctx.r[3].s64 = ctx.r[11].s64 + 23700;
	// 83267C88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267C8C: 4AF8C0CD  bl 0x821f3d58
	ctx.lr = 0x83267C90;
	sub_821F3D58(ctx, base);
	// 83267C90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267C94: 906AB9A0  stw r3, -0x4660(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18016 as u32), ctx.r[3].u32 ) };
	// 83267C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267CA8 size=56
    let mut pc: u32 = 0x83267CA8;
    'dispatch: loop {
        match pc {
            0x83267CA8 => {
    //   block [0x83267CA8..0x83267CE0)
	// 83267CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267CB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267CB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267CBC: 386B5CAC  addi r3, r11, 0x5cac
	ctx.r[3].s64 = ctx.r[11].s64 + 23724;
	// 83267CC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267CC4: 4AF8C095  bl 0x821f3d58
	ctx.lr = 0x83267CC8;
	sub_821F3D58(ctx, base);
	// 83267CC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267CCC: 906AB9A4  stw r3, -0x465c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18012 as u32), ctx.r[3].u32 ) };
	// 83267CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267CE0 size=56
    let mut pc: u32 = 0x83267CE0;
    'dispatch: loop {
        match pc {
            0x83267CE0 => {
    //   block [0x83267CE0..0x83267D18)
	// 83267CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267CEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267CF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267CF4: 386B8E7C  addi r3, r11, -0x7184
	ctx.r[3].s64 = ctx.r[11].s64 + -29060;
	// 83267CF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267CFC: 4AF8C05D  bl 0x821f3d58
	ctx.lr = 0x83267D00;
	sub_821F3D58(ctx, base);
	// 83267D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267D04: 906AB9A8  stw r3, -0x4658(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18008 as u32), ctx.r[3].u32 ) };
	// 83267D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D18 size=56
    let mut pc: u32 = 0x83267D18;
    'dispatch: loop {
        match pc {
            0x83267D18 => {
    //   block [0x83267D18..0x83267D50)
	// 83267D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267D24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267D28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267D2C: 386B5CB4  addi r3, r11, 0x5cb4
	ctx.r[3].s64 = ctx.r[11].s64 + 23732;
	// 83267D30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267D34: 4AF8C025  bl 0x821f3d58
	ctx.lr = 0x83267D38;
	sub_821F3D58(ctx, base);
	// 83267D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267D3C: 906AB9AC  stw r3, -0x4654(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18004 as u32), ctx.r[3].u32 ) };
	// 83267D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D50 size=56
    let mut pc: u32 = 0x83267D50;
    'dispatch: loop {
        match pc {
            0x83267D50 => {
    //   block [0x83267D50..0x83267D88)
	// 83267D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267D5C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267D60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267D64: 386B9C20  addi r3, r11, -0x63e0
	ctx.r[3].s64 = ctx.r[11].s64 + -25568;
	// 83267D68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267D6C: 4AF8BFED  bl 0x821f3d58
	ctx.lr = 0x83267D70;
	sub_821F3D58(ctx, base);
	// 83267D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267D74: 906AB9B0  stw r3, -0x4650(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18000 as u32), ctx.r[3].u32 ) };
	// 83267D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D88 size=56
    let mut pc: u32 = 0x83267D88;
    'dispatch: loop {
        match pc {
            0x83267D88 => {
    //   block [0x83267D88..0x83267DC0)
	// 83267D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267D94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267D98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267D9C: 386B5CC4  addi r3, r11, 0x5cc4
	ctx.r[3].s64 = ctx.r[11].s64 + 23748;
	// 83267DA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267DA4: 4AF8BFB5  bl 0x821f3d58
	ctx.lr = 0x83267DA8;
	sub_821F3D58(ctx, base);
	// 83267DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267DAC: 906AB9B4  stw r3, -0x464c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17996 as u32), ctx.r[3].u32 ) };
	// 83267DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267DC0 size=56
    let mut pc: u32 = 0x83267DC0;
    'dispatch: loop {
        match pc {
            0x83267DC0 => {
    //   block [0x83267DC0..0x83267DF8)
	// 83267DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267DCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267DD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267DD4: 386B5CDC  addi r3, r11, 0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23772;
	// 83267DD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267DDC: 4AF8BF7D  bl 0x821f3d58
	ctx.lr = 0x83267DE0;
	sub_821F3D58(ctx, base);
	// 83267DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267DE4: 906AB9B8  stw r3, -0x4648(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17992 as u32), ctx.r[3].u32 ) };
	// 83267DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267DF8 size=56
    let mut pc: u32 = 0x83267DF8;
    'dispatch: loop {
        match pc {
            0x83267DF8 => {
    //   block [0x83267DF8..0x83267E30)
	// 83267DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267E04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267E08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267E0C: 386BA50C  addi r3, r11, -0x5af4
	ctx.r[3].s64 = ctx.r[11].s64 + -23284;
	// 83267E10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267E14: 4AF8BF45  bl 0x821f3d58
	ctx.lr = 0x83267E18;
	sub_821F3D58(ctx, base);
	// 83267E18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267E1C: 906AB9BC  stw r3, -0x4644(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17988 as u32), ctx.r[3].u32 ) };
	// 83267E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267E30 size=56
    let mut pc: u32 = 0x83267E30;
    'dispatch: loop {
        match pc {
            0x83267E30 => {
    //   block [0x83267E30..0x83267E68)
	// 83267E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267E3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267E40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267E44: 386B5CF4  addi r3, r11, 0x5cf4
	ctx.r[3].s64 = ctx.r[11].s64 + 23796;
	// 83267E48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267E4C: 4AF8BF0D  bl 0x821f3d58
	ctx.lr = 0x83267E50;
	sub_821F3D58(ctx, base);
	// 83267E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267E54: 906AB9C0  stw r3, -0x4640(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17984 as u32), ctx.r[3].u32 ) };
	// 83267E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267E68 size=56
    let mut pc: u32 = 0x83267E68;
    'dispatch: loop {
        match pc {
            0x83267E68 => {
    //   block [0x83267E68..0x83267EA0)
	// 83267E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267E74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267E78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267E7C: 386B5D08  addi r3, r11, 0x5d08
	ctx.r[3].s64 = ctx.r[11].s64 + 23816;
	// 83267E80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267E84: 4AF8BED5  bl 0x821f3d58
	ctx.lr = 0x83267E88;
	sub_821F3D58(ctx, base);
	// 83267E88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267E8C: 906AB9C4  stw r3, -0x463c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17980 as u32), ctx.r[3].u32 ) };
	// 83267E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267EA0 size=56
    let mut pc: u32 = 0x83267EA0;
    'dispatch: loop {
        match pc {
            0x83267EA0 => {
    //   block [0x83267EA0..0x83267ED8)
	// 83267EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267EAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267EB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267EB4: 386B5D18  addi r3, r11, 0x5d18
	ctx.r[3].s64 = ctx.r[11].s64 + 23832;
	// 83267EB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267EBC: 4AF8BE9D  bl 0x821f3d58
	ctx.lr = 0x83267EC0;
	sub_821F3D58(ctx, base);
	// 83267EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267EC4: 906AB9C8  stw r3, -0x4638(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17976 as u32), ctx.r[3].u32 ) };
	// 83267EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267ED8 size=56
    let mut pc: u32 = 0x83267ED8;
    'dispatch: loop {
        match pc {
            0x83267ED8 => {
    //   block [0x83267ED8..0x83267F10)
	// 83267ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267EE4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267EE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267EEC: 386B9CE0  addi r3, r11, -0x6320
	ctx.r[3].s64 = ctx.r[11].s64 + -25376;
	// 83267EF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267EF4: 4AF8BE65  bl 0x821f3d58
	ctx.lr = 0x83267EF8;
	sub_821F3D58(ctx, base);
	// 83267EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267EFC: 906AB9CC  stw r3, -0x4634(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17972 as u32), ctx.r[3].u32 ) };
	// 83267F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F10 size=56
    let mut pc: u32 = 0x83267F10;
    'dispatch: loop {
        match pc {
            0x83267F10 => {
    //   block [0x83267F10..0x83267F48)
	// 83267F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267F1C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267F20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267F24: 386B9CF8  addi r3, r11, -0x6308
	ctx.r[3].s64 = ctx.r[11].s64 + -25352;
	// 83267F28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267F2C: 4AF8BE2D  bl 0x821f3d58
	ctx.lr = 0x83267F30;
	sub_821F3D58(ctx, base);
	// 83267F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267F34: 906AB9D0  stw r3, -0x4630(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17968 as u32), ctx.r[3].u32 ) };
	// 83267F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F48 size=56
    let mut pc: u32 = 0x83267F48;
    'dispatch: loop {
        match pc {
            0x83267F48 => {
    //   block [0x83267F48..0x83267F80)
	// 83267F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267F54: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267F58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267F5C: 386B9CEC  addi r3, r11, -0x6314
	ctx.r[3].s64 = ctx.r[11].s64 + -25364;
	// 83267F60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267F64: 4AF8BDF5  bl 0x821f3d58
	ctx.lr = 0x83267F68;
	sub_821F3D58(ctx, base);
	// 83267F68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267F6C: 906AB9D4  stw r3, -0x462c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17964 as u32), ctx.r[3].u32 ) };
	// 83267F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F80 size=56
    let mut pc: u32 = 0x83267F80;
    'dispatch: loop {
        match pc {
            0x83267F80 => {
    //   block [0x83267F80..0x83267FB8)
	// 83267F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267F8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267F90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267F94: 386B5D28  addi r3, r11, 0x5d28
	ctx.r[3].s64 = ctx.r[11].s64 + 23848;
	// 83267F98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267F9C: 4AF8BDBD  bl 0x821f3d58
	ctx.lr = 0x83267FA0;
	sub_821F3D58(ctx, base);
	// 83267FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267FA4: 906AB9D8  stw r3, -0x4628(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17960 as u32), ctx.r[3].u32 ) };
	// 83267FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267FB8 size=56
    let mut pc: u32 = 0x83267FB8;
    'dispatch: loop {
        match pc {
            0x83267FB8 => {
    //   block [0x83267FB8..0x83267FF0)
	// 83267FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267FC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267FC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267FCC: 386B5D38  addi r3, r11, 0x5d38
	ctx.r[3].s64 = ctx.r[11].s64 + 23864;
	// 83267FD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267FD4: 4AF8BD85  bl 0x821f3d58
	ctx.lr = 0x83267FD8;
	sub_821F3D58(ctx, base);
	// 83267FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267FDC: 906AB9DC  stw r3, -0x4624(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17956 as u32), ctx.r[3].u32 ) };
	// 83267FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267FF0 size=56
    let mut pc: u32 = 0x83267FF0;
    'dispatch: loop {
        match pc {
            0x83267FF0 => {
    //   block [0x83267FF0..0x83268028)
	// 83267FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267FFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268000: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268004: 386B5D48  addi r3, r11, 0x5d48
	ctx.r[3].s64 = ctx.r[11].s64 + 23880;
	// 83268008: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326800C: 4AF8BD4D  bl 0x821f3d58
	ctx.lr = 0x83268010;
	sub_821F3D58(ctx, base);
	// 83268010: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268014: 906AB9E0  stw r3, -0x4620(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17952 as u32), ctx.r[3].u32 ) };
	// 83268018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268028 size=56
    let mut pc: u32 = 0x83268028;
    'dispatch: loop {
        match pc {
            0x83268028 => {
    //   block [0x83268028..0x83268060)
	// 83268028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268034: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268038: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326803C: 386B5D58  addi r3, r11, 0x5d58
	ctx.r[3].s64 = ctx.r[11].s64 + 23896;
	// 83268040: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268044: 4AF8BD15  bl 0x821f3d58
	ctx.lr = 0x83268048;
	sub_821F3D58(ctx, base);
	// 83268048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326804C: 906AB9E4  stw r3, -0x461c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17948 as u32), ctx.r[3].u32 ) };
	// 83268050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326805C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268060 size=56
    let mut pc: u32 = 0x83268060;
    'dispatch: loop {
        match pc {
            0x83268060 => {
    //   block [0x83268060..0x83268098)
	// 83268060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326806C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268070: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268074: 386B5D68  addi r3, r11, 0x5d68
	ctx.r[3].s64 = ctx.r[11].s64 + 23912;
	// 83268078: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326807C: 4AF8BCDD  bl 0x821f3d58
	ctx.lr = 0x83268080;
	sub_821F3D58(ctx, base);
	// 83268080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268084: 906AB9E8  stw r3, -0x4618(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17944 as u32), ctx.r[3].u32 ) };
	// 83268088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326808C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268098 size=56
    let mut pc: u32 = 0x83268098;
    'dispatch: loop {
        match pc {
            0x83268098 => {
    //   block [0x83268098..0x832680D0)
	// 83268098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326809C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832680A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832680A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832680A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832680AC: 386B5D80  addi r3, r11, 0x5d80
	ctx.r[3].s64 = ctx.r[11].s64 + 23936;
	// 832680B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832680B4: 4AF8BCA5  bl 0x821f3d58
	ctx.lr = 0x832680B8;
	sub_821F3D58(ctx, base);
	// 832680B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832680BC: 906AB9EC  stw r3, -0x4614(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17940 as u32), ctx.r[3].u32 ) };
	// 832680C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832680C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832680C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832680CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832680D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832680D0 size=56
    let mut pc: u32 = 0x832680D0;
    'dispatch: loop {
        match pc {
            0x832680D0 => {
    //   block [0x832680D0..0x83268108)
	// 832680D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832680D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832680D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832680DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832680E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832680E4: 386B894C  addi r3, r11, -0x76b4
	ctx.r[3].s64 = ctx.r[11].s64 + -30388;
	// 832680E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832680EC: 4AF8BC6D  bl 0x821f3d58
	ctx.lr = 0x832680F0;
	sub_821F3D58(ctx, base);
	// 832680F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832680F4: 906AB9F0  stw r3, -0x4610(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17936 as u32), ctx.r[3].u32 ) };
	// 832680F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832680FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268108 size=56
    let mut pc: u32 = 0x83268108;
    'dispatch: loop {
        match pc {
            0x83268108 => {
    //   block [0x83268108..0x83268140)
	// 83268108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326810C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268114: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268118: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326811C: 386B8960  addi r3, r11, -0x76a0
	ctx.r[3].s64 = ctx.r[11].s64 + -30368;
	// 83268120: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268124: 4AF8BC35  bl 0x821f3d58
	ctx.lr = 0x83268128;
	sub_821F3D58(ctx, base);
	// 83268128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326812C: 906AB9F4  stw r3, -0x460c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17932 as u32), ctx.r[3].u32 ) };
	// 83268130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268140 size=56
    let mut pc: u32 = 0x83268140;
    'dispatch: loop {
        match pc {
            0x83268140 => {
    //   block [0x83268140..0x83268178)
	// 83268140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326814C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268150: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268154: 386B896C  addi r3, r11, -0x7694
	ctx.r[3].s64 = ctx.r[11].s64 + -30356;
	// 83268158: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326815C: 4AF8BBFD  bl 0x821f3d58
	ctx.lr = 0x83268160;
	sub_821F3D58(ctx, base);
	// 83268160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268164: 906AB9F8  stw r3, -0x4608(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17928 as u32), ctx.r[3].u32 ) };
	// 83268168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326816C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268178 size=56
    let mut pc: u32 = 0x83268178;
    'dispatch: loop {
        match pc {
            0x83268178 => {
    //   block [0x83268178..0x832681B0)
	// 83268178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326817C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268184: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326818C: 386B897C  addi r3, r11, -0x7684
	ctx.r[3].s64 = ctx.r[11].s64 + -30340;
	// 83268190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268194: 4AF8BBC5  bl 0x821f3d58
	ctx.lr = 0x83268198;
	sub_821F3D58(ctx, base);
	// 83268198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326819C: 906AB9FC  stw r3, -0x4604(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17924 as u32), ctx.r[3].u32 ) };
	// 832681A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832681A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832681A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832681AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832681B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832681B0 size=56
    let mut pc: u32 = 0x832681B0;
    'dispatch: loop {
        match pc {
            0x832681B0 => {
    //   block [0x832681B0..0x832681E8)
	// 832681B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832681B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832681B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832681BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832681C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832681C4: 386B8988  addi r3, r11, -0x7678
	ctx.r[3].s64 = ctx.r[11].s64 + -30328;
	// 832681C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832681CC: 4AF8BB8D  bl 0x821f3d58
	ctx.lr = 0x832681D0;
	sub_821F3D58(ctx, base);
	// 832681D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832681D4: 906ABA00  stw r3, -0x4600(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17920 as u32), ctx.r[3].u32 ) };
	// 832681D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832681DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832681E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832681E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832681E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832681E8 size=56
    let mut pc: u32 = 0x832681E8;
    'dispatch: loop {
        match pc {
            0x832681E8 => {
    //   block [0x832681E8..0x83268220)
	// 832681E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832681EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832681F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832681F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832681F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832681FC: 386B8998  addi r3, r11, -0x7668
	ctx.r[3].s64 = ctx.r[11].s64 + -30312;
	// 83268200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268204: 4AF8BB55  bl 0x821f3d58
	ctx.lr = 0x83268208;
	sub_821F3D58(ctx, base);
	// 83268208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326820C: 906ABA04  stw r3, -0x45fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17916 as u32), ctx.r[3].u32 ) };
	// 83268210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326821C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268220 size=56
    let mut pc: u32 = 0x83268220;
    'dispatch: loop {
        match pc {
            0x83268220 => {
    //   block [0x83268220..0x83268258)
	// 83268220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326822C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268234: 386B89A8  addi r3, r11, -0x7658
	ctx.r[3].s64 = ctx.r[11].s64 + -30296;
	// 83268238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326823C: 4AF8BB1D  bl 0x821f3d58
	ctx.lr = 0x83268240;
	sub_821F3D58(ctx, base);
	// 83268240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268244: 906ABA08  stw r3, -0x45f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17912 as u32), ctx.r[3].u32 ) };
	// 83268248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326824C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268258 size=52
    let mut pc: u32 = 0x83268258;
    'dispatch: loop {
        match pc {
            0x83268258 => {
    //   block [0x83268258..0x8326828C)
	// 83268258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326825C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268264: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83268268: 386BA580  addi r3, r11, -0x5a80
	ctx.r[3].s64 = ctx.r[11].s64 + -23168;
	// 8326826C: 4AF2BBCD  bl 0x82193e38
	ctx.lr = 0x83268270;
	sub_82193E38(ctx, base);
	// 83268270: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83268274: 386ADA08  addi r3, r10, -0x25f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9720;
	// 83268278: 4BA41CA9  bl 0x82ca9f20
	ctx.lr = 0x8326827C;
	sub_82CA9F20(ctx, base);
	// 8326827C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268290 size=64
    let mut pc: u32 = 0x83268290;
    'dispatch: loop {
        match pc {
            0x83268290 => {
    //   block [0x83268290..0x832682D0)
	// 83268290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326829C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832682A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832682A4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 832682A8: 386ABA0C  addi r3, r10, -0x45f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17908;
	// 832682AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832682B0: 4AFC4C21  bl 0x8222ced0
	ctx.lr = 0x832682B4;
	sub_8222CED0(ctx, base);
	// 832682B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832682B8: 3869DA50  addi r3, r9, -0x25b0
	ctx.r[3].s64 = ctx.r[9].s64 + -9648;
	// 832682BC: 4BA41C65  bl 0x82ca9f20
	ctx.lr = 0x832682C0;
	sub_82CA9F20(ctx, base);
	// 832682C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832682C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832682C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832682CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832682D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832682D0 size=64
    let mut pc: u32 = 0x832682D0;
    'dispatch: loop {
        match pc {
            0x832682D0 => {
    //   block [0x832682D0..0x83268310)
	// 832682D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832682D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832682D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832682DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832682E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832682E4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832682E8: 386ABA10  addi r3, r10, -0x45f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17904;
	// 832682EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832682F0: 4AFC4BE1  bl 0x8222ced0
	ctx.lr = 0x832682F4;
	sub_8222CED0(ctx, base);
	// 832682F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832682F8: 3869DA60  addi r3, r9, -0x25a0
	ctx.r[3].s64 = ctx.r[9].s64 + -9632;
	// 832682FC: 4BA41C25  bl 0x82ca9f20
	ctx.lr = 0x83268300;
	sub_82CA9F20(ctx, base);
	// 83268300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326830C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268310 size=64
    let mut pc: u32 = 0x83268310;
    'dispatch: loop {
        match pc {
            0x83268310 => {
    //   block [0x83268310..0x83268350)
	// 83268310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326831C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268324: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83268328: 386ABA14  addi r3, r10, -0x45ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17900;
	// 8326832C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268330: 4AFC4BA1  bl 0x8222ced0
	ctx.lr = 0x83268334;
	sub_8222CED0(ctx, base);
	// 83268334: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268338: 3869DA70  addi r3, r9, -0x2590
	ctx.r[3].s64 = ctx.r[9].s64 + -9616;
	// 8326833C: 4BA41BE5  bl 0x82ca9f20
	ctx.lr = 0x83268340;
	sub_82CA9F20(ctx, base);
	// 83268340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326834C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268350 size=64
    let mut pc: u32 = 0x83268350;
    'dispatch: loop {
        match pc {
            0x83268350 => {
    //   block [0x83268350..0x83268390)
	// 83268350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326835C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268364: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83268368: 386ABA18  addi r3, r10, -0x45e8
	ctx.r[3].s64 = ctx.r[10].s64 + -17896;
	// 8326836C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268370: 4AFC4B61  bl 0x8222ced0
	ctx.lr = 0x83268374;
	sub_8222CED0(ctx, base);
	// 83268374: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268378: 3869DA80  addi r3, r9, -0x2580
	ctx.r[3].s64 = ctx.r[9].s64 + -9600;
	// 8326837C: 4BA41BA5  bl 0x82ca9f20
	ctx.lr = 0x83268380;
	sub_82CA9F20(ctx, base);
	// 83268380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326838C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268390 size=64
    let mut pc: u32 = 0x83268390;
    'dispatch: loop {
        match pc {
            0x83268390 => {
    //   block [0x83268390..0x832683D0)
	// 83268390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326839C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832683A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832683A4: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 832683A8: 386ABA1C  addi r3, r10, -0x45e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17892;
	// 832683AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832683B0: 4AFC4B21  bl 0x8222ced0
	ctx.lr = 0x832683B4;
	sub_8222CED0(ctx, base);
	// 832683B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832683B8: 3869DA90  addi r3, r9, -0x2570
	ctx.r[3].s64 = ctx.r[9].s64 + -9584;
	// 832683BC: 4BA41B65  bl 0x82ca9f20
	ctx.lr = 0x832683C0;
	sub_82CA9F20(ctx, base);
	// 832683C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832683C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832683C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832683CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832683D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832683D0 size=64
    let mut pc: u32 = 0x832683D0;
    'dispatch: loop {
        match pc {
            0x832683D0 => {
    //   block [0x832683D0..0x83268410)
	// 832683D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832683D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832683D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832683DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832683E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832683E4: 388B9630  addi r4, r11, -0x69d0
	ctx.r[4].s64 = ctx.r[11].s64 + -27088;
	// 832683E8: 386ABA20  addi r3, r10, -0x45e0
	ctx.r[3].s64 = ctx.r[10].s64 + -17888;
	// 832683EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832683F0: 4AFC4AE1  bl 0x8222ced0
	ctx.lr = 0x832683F4;
	sub_8222CED0(ctx, base);
	// 832683F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832683F8: 3869DAA0  addi r3, r9, -0x2560
	ctx.r[3].s64 = ctx.r[9].s64 + -9568;
	// 832683FC: 4BA41B25  bl 0x82ca9f20
	ctx.lr = 0x83268400;
	sub_82CA9F20(ctx, base);
	// 83268400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326840C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268410 size=64
    let mut pc: u32 = 0x83268410;
    'dispatch: loop {
        match pc {
            0x83268410 => {
    //   block [0x83268410..0x83268450)
	// 83268410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326841C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268424: 388B9648  addi r4, r11, -0x69b8
	ctx.r[4].s64 = ctx.r[11].s64 + -27064;
	// 83268428: 386ABA24  addi r3, r10, -0x45dc
	ctx.r[3].s64 = ctx.r[10].s64 + -17884;
	// 8326842C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268430: 4AFC4AA1  bl 0x8222ced0
	ctx.lr = 0x83268434;
	sub_8222CED0(ctx, base);
	// 83268434: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268438: 3869DAB0  addi r3, r9, -0x2550
	ctx.r[3].s64 = ctx.r[9].s64 + -9552;
	// 8326843C: 4BA41AE5  bl 0x82ca9f20
	ctx.lr = 0x83268440;
	sub_82CA9F20(ctx, base);
	// 83268440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326844C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268450 size=64
    let mut pc: u32 = 0x83268450;
    'dispatch: loop {
        match pc {
            0x83268450 => {
    //   block [0x83268450..0x83268490)
	// 83268450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326845C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268464: 388B9660  addi r4, r11, -0x69a0
	ctx.r[4].s64 = ctx.r[11].s64 + -27040;
	// 83268468: 386ABA28  addi r3, r10, -0x45d8
	ctx.r[3].s64 = ctx.r[10].s64 + -17880;
	// 8326846C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268470: 4AFC4A61  bl 0x8222ced0
	ctx.lr = 0x83268474;
	sub_8222CED0(ctx, base);
	// 83268474: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268478: 3869DAC0  addi r3, r9, -0x2540
	ctx.r[3].s64 = ctx.r[9].s64 + -9536;
	// 8326847C: 4BA41AA5  bl 0x82ca9f20
	ctx.lr = 0x83268480;
	sub_82CA9F20(ctx, base);
	// 83268480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326848C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268490 size=64
    let mut pc: u32 = 0x83268490;
    'dispatch: loop {
        match pc {
            0x83268490 => {
    //   block [0x83268490..0x832684D0)
	// 83268490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326849C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832684A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832684A4: 388B9678  addi r4, r11, -0x6988
	ctx.r[4].s64 = ctx.r[11].s64 + -27016;
	// 832684A8: 386ABA2C  addi r3, r10, -0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17876;
	// 832684AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832684B0: 4AFC4A21  bl 0x8222ced0
	ctx.lr = 0x832684B4;
	sub_8222CED0(ctx, base);
	// 832684B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832684B8: 3869DAD0  addi r3, r9, -0x2530
	ctx.r[3].s64 = ctx.r[9].s64 + -9520;
	// 832684BC: 4BA41A65  bl 0x82ca9f20
	ctx.lr = 0x832684C0;
	sub_82CA9F20(ctx, base);
	// 832684C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832684C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832684C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832684CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832684D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832684D0 size=64
    let mut pc: u32 = 0x832684D0;
    'dispatch: loop {
        match pc {
            0x832684D0 => {
    //   block [0x832684D0..0x83268510)
	// 832684D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832684D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832684D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832684DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832684E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832684E4: 388B969C  addi r4, r11, -0x6964
	ctx.r[4].s64 = ctx.r[11].s64 + -26980;
	// 832684E8: 386ABA30  addi r3, r10, -0x45d0
	ctx.r[3].s64 = ctx.r[10].s64 + -17872;
	// 832684EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832684F0: 4AFC49E1  bl 0x8222ced0
	ctx.lr = 0x832684F4;
	sub_8222CED0(ctx, base);
	// 832684F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832684F8: 3869DAE0  addi r3, r9, -0x2520
	ctx.r[3].s64 = ctx.r[9].s64 + -9504;
	// 832684FC: 4BA41A25  bl 0x82ca9f20
	ctx.lr = 0x83268500;
	sub_82CA9F20(ctx, base);
	// 83268500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326850C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268510 size=64
    let mut pc: u32 = 0x83268510;
    'dispatch: loop {
        match pc {
            0x83268510 => {
    //   block [0x83268510..0x83268550)
	// 83268510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326851C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268520: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268524: 388B9C04  addi r4, r11, -0x63fc
	ctx.r[4].s64 = ctx.r[11].s64 + -25596;
	// 83268528: 386ABA34  addi r3, r10, -0x45cc
	ctx.r[3].s64 = ctx.r[10].s64 + -17868;
	// 8326852C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268530: 4AFC49A1  bl 0x8222ced0
	ctx.lr = 0x83268534;
	sub_8222CED0(ctx, base);
	// 83268534: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268538: 3869DAF0  addi r3, r9, -0x2510
	ctx.r[3].s64 = ctx.r[9].s64 + -9488;
	// 8326853C: 4BA419E5  bl 0x82ca9f20
	ctx.lr = 0x83268540;
	sub_82CA9F20(ctx, base);
	// 83268540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326854C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268550 size=64
    let mut pc: u32 = 0x83268550;
    'dispatch: loop {
        match pc {
            0x83268550 => {
    //   block [0x83268550..0x83268590)
	// 83268550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326855C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268564: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83268568: 386ABA38  addi r3, r10, -0x45c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17864;
	// 8326856C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268570: 4AFC4961  bl 0x8222ced0
	ctx.lr = 0x83268574;
	sub_8222CED0(ctx, base);
	// 83268574: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268578: 3869DB00  addi r3, r9, -0x2500
	ctx.r[3].s64 = ctx.r[9].s64 + -9472;
	// 8326857C: 4BA419A5  bl 0x82ca9f20
	ctx.lr = 0x83268580;
	sub_82CA9F20(ctx, base);
	// 83268580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268590 size=64
    let mut pc: u32 = 0x83268590;
    'dispatch: loop {
        match pc {
            0x83268590 => {
    //   block [0x83268590..0x832685D0)
	// 83268590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326859C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832685A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832685A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832685A8: 386ABA3C  addi r3, r10, -0x45c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17860;
	// 832685AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832685B0: 4AFC4921  bl 0x8222ced0
	ctx.lr = 0x832685B4;
	sub_8222CED0(ctx, base);
	// 832685B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832685B8: 3869DB10  addi r3, r9, -0x24f0
	ctx.r[3].s64 = ctx.r[9].s64 + -9456;
	// 832685BC: 4BA41965  bl 0x82ca9f20
	ctx.lr = 0x832685C0;
	sub_82CA9F20(ctx, base);
	// 832685C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832685C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832685C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832685CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832685D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832685D0 size=64
    let mut pc: u32 = 0x832685D0;
    'dispatch: loop {
        match pc {
            0x832685D0 => {
    //   block [0x832685D0..0x83268610)
	// 832685D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832685D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832685D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832685DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832685E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832685E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832685E8: 386ABA40  addi r3, r10, -0x45c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17856;
	// 832685EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832685F0: 4AFC48E1  bl 0x8222ced0
	ctx.lr = 0x832685F4;
	sub_8222CED0(ctx, base);
	// 832685F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832685F8: 3869DB20  addi r3, r9, -0x24e0
	ctx.r[3].s64 = ctx.r[9].s64 + -9440;
	// 832685FC: 4BA41925  bl 0x82ca9f20
	ctx.lr = 0x83268600;
	sub_82CA9F20(ctx, base);
	// 83268600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268610 size=56
    let mut pc: u32 = 0x83268610;
    'dispatch: loop {
        match pc {
            0x83268610 => {
    //   block [0x83268610..0x83268648)
	// 83268610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326861C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83268620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268624: 386BD418  addi r3, r11, -0x2be8
	ctx.r[3].s64 = ctx.r[11].s64 + -11240;
	// 83268628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326862C: 4AF8B72D  bl 0x821f3d58
	ctx.lr = 0x83268630;
	sub_821F3D58(ctx, base);
	// 83268630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268634: 906ABA44  stw r3, -0x45bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17852 as u32), ctx.r[3].u32 ) };
	// 83268638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326863C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268648 size=64
    let mut pc: u32 = 0x83268648;
    'dispatch: loop {
        match pc {
            0x83268648 => {
    //   block [0x83268648..0x83268688)
	// 83268648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326864C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268654: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326865C: 388B9DA4  addi r4, r11, -0x625c
	ctx.r[4].s64 = ctx.r[11].s64 + -25180;
	// 83268660: 386ABA48  addi r3, r10, -0x45b8
	ctx.r[3].s64 = ctx.r[10].s64 + -17848;
	// 83268664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268668: 4AFC4869  bl 0x8222ced0
	ctx.lr = 0x8326866C;
	sub_8222CED0(ctx, base);
	// 8326866C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268670: 3869DB30  addi r3, r9, -0x24d0
	ctx.r[3].s64 = ctx.r[9].s64 + -9424;
	// 83268674: 4BA418AD  bl 0x82ca9f20
	ctx.lr = 0x83268678;
	sub_82CA9F20(ctx, base);
	// 83268678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326867C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268688 size=56
    let mut pc: u32 = 0x83268688;
    'dispatch: loop {
        match pc {
            0x83268688 => {
    //   block [0x83268688..0x832686C0)
	// 83268688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326868C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268694: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268698: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326869C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832686A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832686A4: 4AF8B6B5  bl 0x821f3d58
	ctx.lr = 0x832686A8;
	sub_821F3D58(ctx, base);
	// 832686A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832686AC: 906ABA4C  stw r3, -0x45b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17844 as u32), ctx.r[3].u32 ) };
	// 832686B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832686B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832686B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832686BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832686C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832686C0 size=56
    let mut pc: u32 = 0x832686C0;
    'dispatch: loop {
        match pc {
            0x832686C0 => {
    //   block [0x832686C0..0x832686F8)
	// 832686C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832686C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832686C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832686CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832686D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832686D4: 386B9DB8  addi r3, r11, -0x6248
	ctx.r[3].s64 = ctx.r[11].s64 + -25160;
	// 832686D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832686DC: 4AF8B67D  bl 0x821f3d58
	ctx.lr = 0x832686E0;
	sub_821F3D58(ctx, base);
	// 832686E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832686E4: 906ABA50  stw r3, -0x45b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17840 as u32), ctx.r[3].u32 ) };
	// 832686E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832686EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832686F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832686F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832686F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832686F8 size=56
    let mut pc: u32 = 0x832686F8;
    'dispatch: loop {
        match pc {
            0x832686F8 => {
    //   block [0x832686F8..0x83268730)
	// 832686F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832686FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268708: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326870C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83268710: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268714: 4AF8B645  bl 0x821f3d58
	ctx.lr = 0x83268718;
	sub_821F3D58(ctx, base);
	// 83268718: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326871C: 906ABA54  stw r3, -0x45ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17836 as u32), ctx.r[3].u32 ) };
	// 83268720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326872C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268730 size=56
    let mut pc: u32 = 0x83268730;
    'dispatch: loop {
        match pc {
            0x83268730 => {
    //   block [0x83268730..0x83268768)
	// 83268730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326873C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268740: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268744: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83268748: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326874C: 4AF8B60D  bl 0x821f3d58
	ctx.lr = 0x83268750;
	sub_821F3D58(ctx, base);
	// 83268750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268754: 906ABA58  stw r3, -0x45a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17832 as u32), ctx.r[3].u32 ) };
	// 83268758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326875C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268768 size=56
    let mut pc: u32 = 0x83268768;
    'dispatch: loop {
        match pc {
            0x83268768 => {
    //   block [0x83268768..0x832687A0)
	// 83268768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268774: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268778: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326877C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83268780: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268784: 4AF8B5D5  bl 0x821f3d58
	ctx.lr = 0x83268788;
	sub_821F3D58(ctx, base);
	// 83268788: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326878C: 906ABA5C  stw r3, -0x45a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17828 as u32), ctx.r[3].u32 ) };
	// 83268790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326879C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832687A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832687A0 size=56
    let mut pc: u32 = 0x832687A0;
    'dispatch: loop {
        match pc {
            0x832687A0 => {
    //   block [0x832687A0..0x832687D8)
	// 832687A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832687A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832687A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832687AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832687B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832687B4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832687B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832687BC: 4AF8B59D  bl 0x821f3d58
	ctx.lr = 0x832687C0;
	sub_821F3D58(ctx, base);
	// 832687C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832687C4: 906ABA60  stw r3, -0x45a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17824 as u32), ctx.r[3].u32 ) };
	// 832687C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832687CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832687D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832687D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832687D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832687D8 size=56
    let mut pc: u32 = 0x832687D8;
    'dispatch: loop {
        match pc {
            0x832687D8 => {
    //   block [0x832687D8..0x83268810)
	// 832687D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832687DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832687E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832687E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832687E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832687EC: 386B9DD0  addi r3, r11, -0x6230
	ctx.r[3].s64 = ctx.r[11].s64 + -25136;
	// 832687F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832687F4: 4AF8B565  bl 0x821f3d58
	ctx.lr = 0x832687F8;
	sub_821F3D58(ctx, base);
	// 832687F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832687FC: 906ABA64  stw r3, -0x459c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17820 as u32), ctx.r[3].u32 ) };
	// 83268800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326880C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268810 size=56
    let mut pc: u32 = 0x83268810;
    'dispatch: loop {
        match pc {
            0x83268810 => {
    //   block [0x83268810..0x83268848)
	// 83268810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326881C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268820: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268824: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83268828: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326882C: 4AF8B52D  bl 0x821f3d58
	ctx.lr = 0x83268830;
	sub_821F3D58(ctx, base);
	// 83268830: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268834: 906ABA68  stw r3, -0x4598(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17816 as u32), ctx.r[3].u32 ) };
	// 83268838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326883C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268848 size=56
    let mut pc: u32 = 0x83268848;
    'dispatch: loop {
        match pc {
            0x83268848 => {
    //   block [0x83268848..0x83268880)
	// 83268848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326884C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268854: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268858: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326885C: 386B9DDC  addi r3, r11, -0x6224
	ctx.r[3].s64 = ctx.r[11].s64 + -25124;
	// 83268860: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268864: 4AF8B4F5  bl 0x821f3d58
	ctx.lr = 0x83268868;
	sub_821F3D58(ctx, base);
	// 83268868: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326886C: 906ABA6C  stw r3, -0x4594(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17812 as u32), ctx.r[3].u32 ) };
	// 83268870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326887C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268880 size=56
    let mut pc: u32 = 0x83268880;
    'dispatch: loop {
        match pc {
            0x83268880 => {
    //   block [0x83268880..0x832688B8)
	// 83268880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326888C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268890: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268894: 386B9DEC  addi r3, r11, -0x6214
	ctx.r[3].s64 = ctx.r[11].s64 + -25108;
	// 83268898: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326889C: 4AF8B4BD  bl 0x821f3d58
	ctx.lr = 0x832688A0;
	sub_821F3D58(ctx, base);
	// 832688A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832688A4: 906ABA70  stw r3, -0x4590(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17808 as u32), ctx.r[3].u32 ) };
	// 832688A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832688AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832688B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832688B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832688B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832688B8 size=56
    let mut pc: u32 = 0x832688B8;
    'dispatch: loop {
        match pc {
            0x832688B8 => {
    //   block [0x832688B8..0x832688F0)
	// 832688B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832688BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832688C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832688C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832688C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832688CC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832688D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832688D4: 4AF8B485  bl 0x821f3d58
	ctx.lr = 0x832688D8;
	sub_821F3D58(ctx, base);
	// 832688D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832688DC: 906ABA74  stw r3, -0x458c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17804 as u32), ctx.r[3].u32 ) };
	// 832688E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832688E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832688E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832688EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832688F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832688F0 size=56
    let mut pc: u32 = 0x832688F0;
    'dispatch: loop {
        match pc {
            0x832688F0 => {
    //   block [0x832688F0..0x83268928)
	// 832688F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832688F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832688F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832688FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268900: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268904: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83268908: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326890C: 4AF8B44D  bl 0x821f3d58
	ctx.lr = 0x83268910;
	sub_821F3D58(ctx, base);
	// 83268910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268914: 906ABA78  stw r3, -0x4588(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17800 as u32), ctx.r[3].u32 ) };
	// 83268918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268928 size=604
    let mut pc: u32 = 0x83268928;
    'dispatch: loop {
        match pc {
            0x83268928 => {
    //   block [0x83268928..0x83268B84)
	// 83268928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326892C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83268934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83268938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326893C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83268940: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83268944: 3BEBBA80  addi r31, r11, -0x4580
	ctx.r[31].s64 = ctx.r[11].s64 + -17792;
	// 83268948: 388A0830  addi r4, r10, 0x830
	ctx.r[4].s64 = ctx.r[10].s64 + 2096;
	// 8326894C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83268950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268954: 4AFC457D  bl 0x8222ced0
	ctx.lr = 0x83268958;
	sub_8222CED0(ctx, base);
	// 83268958: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 8326895C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268960: 3889081C  addi r4, r9, 0x81c
	ctx.r[4].s64 = ctx.r[9].s64 + 2076;
	// 83268964: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83268968: 4AFC4569  bl 0x8222ced0
	ctx.lr = 0x8326896C;
	sub_8222CED0(ctx, base);
	// 8326896C: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83268970: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268974: 388807FC  addi r4, r8, 0x7fc
	ctx.r[4].s64 = ctx.r[8].s64 + 2044;
	// 83268978: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8326897C: 4AFC4555  bl 0x8222ced0
	ctx.lr = 0x83268980;
	sub_8222CED0(ctx, base);
	// 83268980: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83268984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268988: 388707E8  addi r4, r7, 0x7e8
	ctx.r[4].s64 = ctx.r[7].s64 + 2024;
	// 8326898C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83268990: 4AFC4541  bl 0x8222ced0
	ctx.lr = 0x83268994;
	sub_8222CED0(ctx, base);
	// 83268994: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83268998: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326899C: 388606CC  addi r4, r6, 0x6cc
	ctx.r[4].s64 = ctx.r[6].s64 + 1740;
	// 832689A0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832689A4: 4AFC452D  bl 0x8222ced0
	ctx.lr = 0x832689A8;
	sub_8222CED0(ctx, base);
	// 832689A8: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832689AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832689B0: 388406B4  addi r4, r4, 0x6b4
	ctx.r[4].s64 = ctx.r[4].s64 + 1716;
	// 832689B4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832689B8: 4AFC4519  bl 0x8222ced0
	ctx.lr = 0x832689BC;
	sub_8222CED0(ctx, base);
	// 832689BC: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832689C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832689C4: 388306A0  addi r4, r3, 0x6a0
	ctx.r[4].s64 = ctx.r[3].s64 + 1696;
	// 832689C8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832689CC: 4AFC4505  bl 0x8222ced0
	ctx.lr = 0x832689D0;
	sub_8222CED0(ctx, base);
	// 832689D0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832689D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832689D8: 388B0688  addi r4, r11, 0x688
	ctx.r[4].s64 = ctx.r[11].s64 + 1672;
	// 832689DC: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832689E0: 4AFC44F1  bl 0x8222ced0
	ctx.lr = 0x832689E4;
	sub_8222CED0(ctx, base);
	// 832689E4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832689E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832689EC: 388A07C4  addi r4, r10, 0x7c4
	ctx.r[4].s64 = ctx.r[10].s64 + 1988;
	// 832689F0: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832689F4: 4AFC44DD  bl 0x8222ced0
	ctx.lr = 0x832689F8;
	sub_8222CED0(ctx, base);
	// 832689F8: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832689FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A00: 388907AC  addi r4, r9, 0x7ac
	ctx.r[4].s64 = ctx.r[9].s64 + 1964;
	// 83268A04: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83268A08: 4AFC44C9  bl 0x8222ced0
	ctx.lr = 0x83268A0C;
	sub_8222CED0(ctx, base);
	// 83268A0C: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83268A10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A14: 38880798  addi r4, r8, 0x798
	ctx.r[4].s64 = ctx.r[8].s64 + 1944;
	// 83268A18: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83268A1C: 4AFC44B5  bl 0x8222ced0
	ctx.lr = 0x83268A20;
	sub_8222CED0(ctx, base);
	// 83268A20: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83268A24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A28: 38870780  addi r4, r7, 0x780
	ctx.r[4].s64 = ctx.r[7].s64 + 1920;
	// 83268A2C: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83268A30: 4AFC44A1  bl 0x8222ced0
	ctx.lr = 0x83268A34;
	sub_8222CED0(ctx, base);
	// 83268A34: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83268A38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A3C: 38860840  addi r4, r6, 0x840
	ctx.r[4].s64 = ctx.r[6].s64 + 2112;
	// 83268A40: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83268A44: 4AFC448D  bl 0x8222ced0
	ctx.lr = 0x83268A48;
	sub_8222CED0(ctx, base);
	// 83268A48: 3C808200  lis r4, -0x7e00
	ctx.r[4].s64 = -2113929216;
	// 83268A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A50: 3BC40CA0  addi r30, r4, 0xca0
	ctx.r[30].s64 = ctx.r[4].s64 + 3232;
	// 83268A54: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83268A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268A5C: 4AFC4475  bl 0x8222ced0
	ctx.lr = 0x83268A60;
	sub_8222CED0(ctx, base);
	// 83268A60: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83268A64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A68: 38830768  addi r4, r3, 0x768
	ctx.r[4].s64 = ctx.r[3].s64 + 1896;
	// 83268A6C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83268A70: 4AFC4461  bl 0x8222ced0
	ctx.lr = 0x83268A74;
	sub_8222CED0(ctx, base);
	// 83268A74: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83268A78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A7C: 388B0754  addi r4, r11, 0x754
	ctx.r[4].s64 = ctx.r[11].s64 + 1876;
	// 83268A80: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83268A84: 4AFC444D  bl 0x8222ced0
	ctx.lr = 0x83268A88;
	sub_8222CED0(ctx, base);
	// 83268A88: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83268A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268A90: 388A0738  addi r4, r10, 0x738
	ctx.r[4].s64 = ctx.r[10].s64 + 1848;
	// 83268A94: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83268A98: 4AFC4439  bl 0x8222ced0
	ctx.lr = 0x83268A9C;
	sub_8222CED0(ctx, base);
	// 83268A9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268AA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AA4: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83268AA8: 4AFC4429  bl 0x8222ced0
	ctx.lr = 0x83268AAC;
	sub_8222CED0(ctx, base);
	// 83268AAC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83268AB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AB4: 38890728  addi r4, r9, 0x728
	ctx.r[4].s64 = ctx.r[9].s64 + 1832;
	// 83268AB8: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83268ABC: 4AFC4415  bl 0x8222ced0
	ctx.lr = 0x83268AC0;
	sub_8222CED0(ctx, base);
	// 83268AC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AC8: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83268ACC: 4AFC4405  bl 0x8222ced0
	ctx.lr = 0x83268AD0;
	sub_8222CED0(ctx, base);
	// 83268AD0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83268AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AD8: 38880718  addi r4, r8, 0x718
	ctx.r[4].s64 = ctx.r[8].s64 + 1816;
	// 83268ADC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83268AE0: 4AFC43F1  bl 0x8222ced0
	ctx.lr = 0x83268AE4;
	sub_8222CED0(ctx, base);
	// 83268AE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268AE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AEC: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83268AF0: 4AFC43E1  bl 0x8222ced0
	ctx.lr = 0x83268AF4;
	sub_8222CED0(ctx, base);
	// 83268AF4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83268AF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AFC: 38870708  addi r4, r7, 0x708
	ctx.r[4].s64 = ctx.r[7].s64 + 1800;
	// 83268B00: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83268B04: 4AFC43CD  bl 0x8222ced0
	ctx.lr = 0x83268B08;
	sub_8222CED0(ctx, base);
	// 83268B08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B10: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83268B14: 4AFC43BD  bl 0x8222ced0
	ctx.lr = 0x83268B18;
	sub_8222CED0(ctx, base);
	// 83268B18: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 83268B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B20: 388613B0  addi r4, r6, 0x13b0
	ctx.r[4].s64 = ctx.r[6].s64 + 5040;
	// 83268B24: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83268B28: 4AFC43A9  bl 0x8222ced0
	ctx.lr = 0x83268B2C;
	sub_8222CED0(ctx, base);
	// 83268B2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268B30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B34: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83268B38: 4AFC4399  bl 0x8222ced0
	ctx.lr = 0x83268B3C;
	sub_8222CED0(ctx, base);
	// 83268B3C: 3C808200  lis r4, -0x7e00
	ctx.r[4].s64 = -2113929216;
	// 83268B40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B44: 388413A0  addi r4, r4, 0x13a0
	ctx.r[4].s64 = ctx.r[4].s64 + 5024;
	// 83268B48: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83268B4C: 4AFC4385  bl 0x8222ced0
	ctx.lr = 0x83268B50;
	sub_8222CED0(ctx, base);
	// 83268B50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B58: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83268B5C: 4AFC4375  bl 0x8222ced0
	ctx.lr = 0x83268B60;
	sub_8222CED0(ctx, base);
	// 83268B60: 3C60832B  lis r3, -0x7cd5
	ctx.r[3].s64 = -2094333952;
	// 83268B64: 3863DB40  addi r3, r3, -0x24c0
	ctx.r[3].s64 = ctx.r[3].s64 + -9408;
	// 83268B68: 4BA413B9  bl 0x82ca9f20
	ctx.lr = 0x83268B6C;
	sub_82CA9F20(ctx, base);
	// 83268B6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83268B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268B78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83268B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83268B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268B88 size=64
    let mut pc: u32 = 0x83268B88;
    'dispatch: loop {
        match pc {
            0x83268B88 => {
    //   block [0x83268B88..0x83268BC8)
	// 83268B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268B90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268B94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268B98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268B9C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83268BA0: 386ABAF0  addi r3, r10, -0x4510
	ctx.r[3].s64 = ctx.r[10].s64 + -17680;
	// 83268BA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268BA8: 4AFC4329  bl 0x8222ced0
	ctx.lr = 0x83268BAC;
	sub_8222CED0(ctx, base);
	// 83268BAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268BB0: 3869DBA8  addi r3, r9, -0x2458
	ctx.r[3].s64 = ctx.r[9].s64 + -9304;
	// 83268BB4: 4BA4136D  bl 0x82ca9f20
	ctx.lr = 0x83268BB8;
	sub_82CA9F20(ctx, base);
	// 83268BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268BC8 size=64
    let mut pc: u32 = 0x83268BC8;
    'dispatch: loop {
        match pc {
            0x83268BC8 => {
    //   block [0x83268BC8..0x83268C08)
	// 83268BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268BD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268BDC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83268BE0: 386ABAF4  addi r3, r10, -0x450c
	ctx.r[3].s64 = ctx.r[10].s64 + -17676;
	// 83268BE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268BE8: 4AFC42E9  bl 0x8222ced0
	ctx.lr = 0x83268BEC;
	sub_8222CED0(ctx, base);
	// 83268BEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268BF0: 3869DBB8  addi r3, r9, -0x2448
	ctx.r[3].s64 = ctx.r[9].s64 + -9288;
	// 83268BF4: 4BA4132D  bl 0x82ca9f20
	ctx.lr = 0x83268BF8;
	sub_82CA9F20(ctx, base);
	// 83268BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268C08 size=64
    let mut pc: u32 = 0x83268C08;
    'dispatch: loop {
        match pc {
            0x83268C08 => {
    //   block [0x83268C08..0x83268C48)
	// 83268C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268C14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268C18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268C1C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83268C20: 386ABAF8  addi r3, r10, -0x4508
	ctx.r[3].s64 = ctx.r[10].s64 + -17672;
	// 83268C24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268C28: 4AFC42A9  bl 0x8222ced0
	ctx.lr = 0x83268C2C;
	sub_8222CED0(ctx, base);
	// 83268C2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268C30: 3869DBC8  addi r3, r9, -0x2438
	ctx.r[3].s64 = ctx.r[9].s64 + -9272;
	// 83268C34: 4BA412ED  bl 0x82ca9f20
	ctx.lr = 0x83268C38;
	sub_82CA9F20(ctx, base);
	// 83268C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268C48 size=144
    let mut pc: u32 = 0x83268C48;
    'dispatch: loop {
        match pc {
            0x83268C48 => {
    //   block [0x83268C48..0x83268C6C)
	// 83268C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268C54: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83268C58: 4AFB6601  bl 0x8221f258
	ctx.lr = 0x83268C5C;
	sub_8221F258(ctx, base);
	// 83268C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83268C60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83268C64: 419A0008  beq cr6, 0x83268c6c
	if ctx.cr[6].eq {
	pc = 0x83268C6C; continue 'dispatch;
	}
	// 83268C68: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83268C6C; continue 'dispatch;
            }
            0x83268C6C => {
    //   block [0x83268C6C..0x83268C78)
	// 83268C6C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83268C70: 41820008  beq 0x83268c78
	if ctx.cr[0].eq {
	pc = 0x83268C78; continue 'dispatch;
	}
	// 83268C74: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83268C78; continue 'dispatch;
            }
            0x83268C78 => {
    //   block [0x83268C78..0x83268C84)
	// 83268C78: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83268C7C: 41820008  beq 0x83268c84
	if ctx.cr[0].eq {
	pc = 0x83268C84; continue 'dispatch;
	}
	// 83268C80: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83268C84; continue 'dispatch;
            }
            0x83268C84 => {
    //   block [0x83268C84..0x83268CD8)
	// 83268C84: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83268C88: 99430019  stb r10, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 83268C8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83268C90: 3909BAFC  addi r8, r9, -0x4504
	ctx.r[8].s64 = ctx.r[9].s64 + -17668;
	// 83268C94: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 83268C98: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83268C9C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83268CA0: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 83268CA4: 3867DBD8  addi r3, r7, -0x2428
	ctx.r[3].s64 = ctx.r[7].s64 + -9256;
	// 83268CA8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83268CAC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83268CB0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83268CB4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83268CB8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83268CBC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83268CC0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83268CC4: 4BA4125D  bl 0x82ca9f20
	ctx.lr = 0x83268CC8;
	sub_82CA9F20(ctx, base);
	// 83268CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83268CD8 size=12
    let mut pc: u32 = 0x83268CD8;
    'dispatch: loop {
        match pc {
            0x83268CD8 => {
    //   block [0x83268CD8..0x83268CE4)
	// 83268CD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83268CDC: 386BDBE8  addi r3, r11, -0x2418
	ctx.r[3].s64 = ctx.r[11].s64 + -9240;
	// 83268CE0: 4BA41240  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83268CE8 size=12
    let mut pc: u32 = 0x83268CE8;
    'dispatch: loop {
        match pc {
            0x83268CE8 => {
    //   block [0x83268CE8..0x83268CF4)
	// 83268CE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83268CEC: 386BDBF8  addi r3, r11, -0x2408
	ctx.r[3].s64 = ctx.r[11].s64 + -9224;
	// 83268CF0: 4BA41230  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83268CF8 size=12
    let mut pc: u32 = 0x83268CF8;
    'dispatch: loop {
        match pc {
            0x83268CF8 => {
    //   block [0x83268CF8..0x83268D04)
	// 83268CF8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83268CFC: 386BDC08  addi r3, r11, -0x23f8
	ctx.r[3].s64 = ctx.r[11].s64 + -9208;
	// 83268D00: 4BA41220  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83268D08 size=12
    let mut pc: u32 = 0x83268D08;
    'dispatch: loop {
        match pc {
            0x83268D08 => {
    //   block [0x83268D08..0x83268D14)
	// 83268D08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83268D0C: 386BDC18  addi r3, r11, -0x23e8
	ctx.r[3].s64 = ctx.r[11].s64 + -9192;
	// 83268D10: 4BA41210  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268D18 size=64
    let mut pc: u32 = 0x83268D18;
    'dispatch: loop {
        match pc {
            0x83268D18 => {
    //   block [0x83268D18..0x83268D58)
	// 83268D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268D24: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268D28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268D2C: 388BA75C  addi r4, r11, -0x58a4
	ctx.r[4].s64 = ctx.r[11].s64 + -22692;
	// 83268D30: 386ABB28  addi r3, r10, -0x44d8
	ctx.r[3].s64 = ctx.r[10].s64 + -17624;
	// 83268D34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268D38: 4AFC4199  bl 0x8222ced0
	ctx.lr = 0x83268D3C;
	sub_8222CED0(ctx, base);
	// 83268D3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268D40: 3869DC28  addi r3, r9, -0x23d8
	ctx.r[3].s64 = ctx.r[9].s64 + -9176;
	// 83268D44: 4BA411DD  bl 0x82ca9f20
	ctx.lr = 0x83268D48;
	sub_82CA9F20(ctx, base);
	// 83268D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268D58 size=64
    let mut pc: u32 = 0x83268D58;
    'dispatch: loop {
        match pc {
            0x83268D58 => {
    //   block [0x83268D58..0x83268D98)
	// 83268D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268D64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268D68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268D6C: 388BA774  addi r4, r11, -0x588c
	ctx.r[4].s64 = ctx.r[11].s64 + -22668;
	// 83268D70: 386ABB2C  addi r3, r10, -0x44d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17620;
	// 83268D74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268D78: 4AFC4159  bl 0x8222ced0
	ctx.lr = 0x83268D7C;
	sub_8222CED0(ctx, base);
	// 83268D7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268D80: 3869DC38  addi r3, r9, -0x23c8
	ctx.r[3].s64 = ctx.r[9].s64 + -9160;
	// 83268D84: 4BA4119D  bl 0x82ca9f20
	ctx.lr = 0x83268D88;
	sub_82CA9F20(ctx, base);
	// 83268D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268D98 size=64
    let mut pc: u32 = 0x83268D98;
    'dispatch: loop {
        match pc {
            0x83268D98 => {
    //   block [0x83268D98..0x83268DD8)
	// 83268D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268DAC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83268DB0: 386ABB30  addi r3, r10, -0x44d0
	ctx.r[3].s64 = ctx.r[10].s64 + -17616;
	// 83268DB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268DB8: 4AFC4119  bl 0x8222ced0
	ctx.lr = 0x83268DBC;
	sub_8222CED0(ctx, base);
	// 83268DBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268DC0: 3869DC48  addi r3, r9, -0x23b8
	ctx.r[3].s64 = ctx.r[9].s64 + -9144;
	// 83268DC4: 4BA4115D  bl 0x82ca9f20
	ctx.lr = 0x83268DC8;
	sub_82CA9F20(ctx, base);
	// 83268DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268DD8 size=64
    let mut pc: u32 = 0x83268DD8;
    'dispatch: loop {
        match pc {
            0x83268DD8 => {
    //   block [0x83268DD8..0x83268E18)
	// 83268DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268DE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268DEC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83268DF0: 386ABB34  addi r3, r10, -0x44cc
	ctx.r[3].s64 = ctx.r[10].s64 + -17612;
	// 83268DF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268DF8: 4AFC40D9  bl 0x8222ced0
	ctx.lr = 0x83268DFC;
	sub_8222CED0(ctx, base);
	// 83268DFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268E00: 3869DC58  addi r3, r9, -0x23a8
	ctx.r[3].s64 = ctx.r[9].s64 + -9128;
	// 83268E04: 4BA4111D  bl 0x82ca9f20
	ctx.lr = 0x83268E08;
	sub_82CA9F20(ctx, base);
	// 83268E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268E18 size=64
    let mut pc: u32 = 0x83268E18;
    'dispatch: loop {
        match pc {
            0x83268E18 => {
    //   block [0x83268E18..0x83268E58)
	// 83268E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268E24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83268E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268E2C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83268E30: 386ABB38  addi r3, r10, -0x44c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17608;
	// 83268E34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268E38: 4AFC4099  bl 0x8222ced0
	ctx.lr = 0x83268E3C;
	sub_8222CED0(ctx, base);
	// 83268E3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268E40: 3869DC68  addi r3, r9, -0x2398
	ctx.r[3].s64 = ctx.r[9].s64 + -9112;
	// 83268E44: 4BA410DD  bl 0x82ca9f20
	ctx.lr = 0x83268E48;
	sub_82CA9F20(ctx, base);
	// 83268E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268E58 size=60
    let mut pc: u32 = 0x83268E58;
    'dispatch: loop {
        match pc {
            0x83268E58 => {
    //   block [0x83268E58..0x83268E94)
	// 83268E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268E64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268E68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268E6C: 388BA824  addi r4, r11, -0x57dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22492;
	// 83268E70: 386ABB3C  addi r3, r10, -0x44c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17604;
	// 83268E74: 4B06D595  bl 0x822d6408
	ctx.lr = 0x83268E78;
	sub_822D6408(ctx, base);
	// 83268E78: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268E7C: 3869DC78  addi r3, r9, -0x2388
	ctx.r[3].s64 = ctx.r[9].s64 + -9096;
	// 83268E80: 4BA410A1  bl 0x82ca9f20
	ctx.lr = 0x83268E84;
	sub_82CA9F20(ctx, base);
	// 83268E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268E98 size=60
    let mut pc: u32 = 0x83268E98;
    'dispatch: loop {
        match pc {
            0x83268E98 => {
    //   block [0x83268E98..0x83268ED4)
	// 83268E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268EA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268EA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268EA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268EAC: 388BA828  addi r4, r11, -0x57d8
	ctx.r[4].s64 = ctx.r[11].s64 + -22488;
	// 83268EB0: 386ABB40  addi r3, r10, -0x44c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17600;
	// 83268EB4: 4B06D555  bl 0x822d6408
	ctx.lr = 0x83268EB8;
	sub_822D6408(ctx, base);
	// 83268EB8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268EBC: 3869DC88  addi r3, r9, -0x2378
	ctx.r[3].s64 = ctx.r[9].s64 + -9080;
	// 83268EC0: 4BA41061  bl 0x82ca9f20
	ctx.lr = 0x83268EC4;
	sub_82CA9F20(ctx, base);
	// 83268EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268ED8 size=60
    let mut pc: u32 = 0x83268ED8;
    'dispatch: loop {
        match pc {
            0x83268ED8 => {
    //   block [0x83268ED8..0x83268F14)
	// 83268ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268EE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268EEC: 388BA830  addi r4, r11, -0x57d0
	ctx.r[4].s64 = ctx.r[11].s64 + -22480;
	// 83268EF0: 386ABB44  addi r3, r10, -0x44bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17596;
	// 83268EF4: 4B06D515  bl 0x822d6408
	ctx.lr = 0x83268EF8;
	sub_822D6408(ctx, base);
	// 83268EF8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268EFC: 3869DC98  addi r3, r9, -0x2368
	ctx.r[3].s64 = ctx.r[9].s64 + -9064;
	// 83268F00: 4BA41021  bl 0x82ca9f20
	ctx.lr = 0x83268F04;
	sub_82CA9F20(ctx, base);
	// 83268F04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268F18 size=60
    let mut pc: u32 = 0x83268F18;
    'dispatch: loop {
        match pc {
            0x83268F18 => {
    //   block [0x83268F18..0x83268F54)
	// 83268F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268F24: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268F28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268F2C: 388BA838  addi r4, r11, -0x57c8
	ctx.r[4].s64 = ctx.r[11].s64 + -22472;
	// 83268F30: 386ABB48  addi r3, r10, -0x44b8
	ctx.r[3].s64 = ctx.r[10].s64 + -17592;
	// 83268F34: 4B06D4D5  bl 0x822d6408
	ctx.lr = 0x83268F38;
	sub_822D6408(ctx, base);
	// 83268F38: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268F3C: 3869DCA8  addi r3, r9, -0x2358
	ctx.r[3].s64 = ctx.r[9].s64 + -9048;
	// 83268F40: 4BA40FE1  bl 0x82ca9f20
	ctx.lr = 0x83268F44;
	sub_82CA9F20(ctx, base);
	// 83268F44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268F58 size=60
    let mut pc: u32 = 0x83268F58;
    'dispatch: loop {
        match pc {
            0x83268F58 => {
    //   block [0x83268F58..0x83268F94)
	// 83268F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268F64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268F68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268F6C: 388BA840  addi r4, r11, -0x57c0
	ctx.r[4].s64 = ctx.r[11].s64 + -22464;
	// 83268F70: 386ABB4C  addi r3, r10, -0x44b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17588;
	// 83268F74: 4B06D495  bl 0x822d6408
	ctx.lr = 0x83268F78;
	sub_822D6408(ctx, base);
	// 83268F78: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268F7C: 3869DCB8  addi r3, r9, -0x2348
	ctx.r[3].s64 = ctx.r[9].s64 + -9032;
	// 83268F80: 4BA40FA1  bl 0x82ca9f20
	ctx.lr = 0x83268F84;
	sub_82CA9F20(ctx, base);
	// 83268F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268F98 size=60
    let mut pc: u32 = 0x83268F98;
    'dispatch: loop {
        match pc {
            0x83268F98 => {
    //   block [0x83268F98..0x83268FD4)
	// 83268F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268FA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268FA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268FAC: 388BA848  addi r4, r11, -0x57b8
	ctx.r[4].s64 = ctx.r[11].s64 + -22456;
	// 83268FB0: 386ABB50  addi r3, r10, -0x44b0
	ctx.r[3].s64 = ctx.r[10].s64 + -17584;
	// 83268FB4: 4B06D455  bl 0x822d6408
	ctx.lr = 0x83268FB8;
	sub_822D6408(ctx, base);
	// 83268FB8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268FBC: 3869DCC8  addi r3, r9, -0x2338
	ctx.r[3].s64 = ctx.r[9].s64 + -9016;
	// 83268FC0: 4BA40F61  bl 0x82ca9f20
	ctx.lr = 0x83268FC4;
	sub_82CA9F20(ctx, base);
	// 83268FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268FD8 size=60
    let mut pc: u32 = 0x83268FD8;
    'dispatch: loop {
        match pc {
            0x83268FD8 => {
    //   block [0x83268FD8..0x83269014)
	// 83268FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268FE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83268FE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268FEC: 388BA854  addi r4, r11, -0x57ac
	ctx.r[4].s64 = ctx.r[11].s64 + -22444;
	// 83268FF0: 386ABB54  addi r3, r10, -0x44ac
	ctx.r[3].s64 = ctx.r[10].s64 + -17580;
	// 83268FF4: 4B06D415  bl 0x822d6408
	ctx.lr = 0x83268FF8;
	sub_822D6408(ctx, base);
	// 83268FF8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268FFC: 3869DCD8  addi r3, r9, -0x2328
	ctx.r[3].s64 = ctx.r[9].s64 + -9000;
	// 83269000: 4BA40F21  bl 0x82ca9f20
	ctx.lr = 0x83269004;
	sub_82CA9F20(ctx, base);
	// 83269004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326900C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269018 size=60
    let mut pc: u32 = 0x83269018;
    'dispatch: loop {
        match pc {
            0x83269018 => {
    //   block [0x83269018..0x83269054)
	// 83269018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326901C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269024: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326902C: 388BA860  addi r4, r11, -0x57a0
	ctx.r[4].s64 = ctx.r[11].s64 + -22432;
	// 83269030: 386ABB58  addi r3, r10, -0x44a8
	ctx.r[3].s64 = ctx.r[10].s64 + -17576;
	// 83269034: 4B06D3D5  bl 0x822d6408
	ctx.lr = 0x83269038;
	sub_822D6408(ctx, base);
	// 83269038: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326903C: 3869DCE8  addi r3, r9, -0x2318
	ctx.r[3].s64 = ctx.r[9].s64 + -8984;
	// 83269040: 4BA40EE1  bl 0x82ca9f20
	ctx.lr = 0x83269044;
	sub_82CA9F20(ctx, base);
	// 83269044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326904C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269058 size=60
    let mut pc: u32 = 0x83269058;
    'dispatch: loop {
        match pc {
            0x83269058 => {
    //   block [0x83269058..0x83269094)
	// 83269058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269064: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326906C: 388BA86C  addi r4, r11, -0x5794
	ctx.r[4].s64 = ctx.r[11].s64 + -22420;
	// 83269070: 386ABB5C  addi r3, r10, -0x44a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17572;
	// 83269074: 4B06D395  bl 0x822d6408
	ctx.lr = 0x83269078;
	sub_822D6408(ctx, base);
	// 83269078: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326907C: 3869DCF8  addi r3, r9, -0x2308
	ctx.r[3].s64 = ctx.r[9].s64 + -8968;
	// 83269080: 4BA40EA1  bl 0x82ca9f20
	ctx.lr = 0x83269084;
	sub_82CA9F20(ctx, base);
	// 83269084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326908C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269098 size=60
    let mut pc: u32 = 0x83269098;
    'dispatch: loop {
        match pc {
            0x83269098 => {
    //   block [0x83269098..0x832690D4)
	// 83269098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326909C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832690A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832690A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832690A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832690AC: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 832690B0: 386ABB60  addi r3, r10, -0x44a0
	ctx.r[3].s64 = ctx.r[10].s64 + -17568;
	// 832690B4: 4B06D355  bl 0x822d6408
	ctx.lr = 0x832690B8;
	sub_822D6408(ctx, base);
	// 832690B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832690BC: 3869DD08  addi r3, r9, -0x22f8
	ctx.r[3].s64 = ctx.r[9].s64 + -8952;
	// 832690C0: 4BA40E61  bl 0x82ca9f20
	ctx.lr = 0x832690C4;
	sub_82CA9F20(ctx, base);
	// 832690C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832690C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832690CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832690D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832690D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832690D8 size=60
    let mut pc: u32 = 0x832690D8;
    'dispatch: loop {
        match pc {
            0x832690D8 => {
    //   block [0x832690D8..0x83269114)
	// 832690D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832690DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832690E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832690E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832690E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832690EC: 388BA88C  addi r4, r11, -0x5774
	ctx.r[4].s64 = ctx.r[11].s64 + -22388;
	// 832690F0: 386ABB64  addi r3, r10, -0x449c
	ctx.r[3].s64 = ctx.r[10].s64 + -17564;
	// 832690F4: 4B06D315  bl 0x822d6408
	ctx.lr = 0x832690F8;
	sub_822D6408(ctx, base);
	// 832690F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832690FC: 3869DD18  addi r3, r9, -0x22e8
	ctx.r[3].s64 = ctx.r[9].s64 + -8936;
	// 83269100: 4BA40E21  bl 0x82ca9f20
	ctx.lr = 0x83269104;
	sub_82CA9F20(ctx, base);
	// 83269104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269118 size=60
    let mut pc: u32 = 0x83269118;
    'dispatch: loop {
        match pc {
            0x83269118 => {
    //   block [0x83269118..0x83269154)
	// 83269118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326911C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269124: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326912C: 388BA894  addi r4, r11, -0x576c
	ctx.r[4].s64 = ctx.r[11].s64 + -22380;
	// 83269130: 386ABB68  addi r3, r10, -0x4498
	ctx.r[3].s64 = ctx.r[10].s64 + -17560;
	// 83269134: 4B06D2D5  bl 0x822d6408
	ctx.lr = 0x83269138;
	sub_822D6408(ctx, base);
	// 83269138: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326913C: 3869DD28  addi r3, r9, -0x22d8
	ctx.r[3].s64 = ctx.r[9].s64 + -8920;
	// 83269140: 4BA40DE1  bl 0x82ca9f20
	ctx.lr = 0x83269144;
	sub_82CA9F20(ctx, base);
	// 83269144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326914C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269158 size=60
    let mut pc: u32 = 0x83269158;
    'dispatch: loop {
        match pc {
            0x83269158 => {
    //   block [0x83269158..0x83269194)
	// 83269158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326915C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269164: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326916C: 388BA8AC  addi r4, r11, -0x5754
	ctx.r[4].s64 = ctx.r[11].s64 + -22356;
	// 83269170: 386ABB6C  addi r3, r10, -0x4494
	ctx.r[3].s64 = ctx.r[10].s64 + -17556;
	// 83269174: 4B06D295  bl 0x822d6408
	ctx.lr = 0x83269178;
	sub_822D6408(ctx, base);
	// 83269178: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326917C: 3869DD38  addi r3, r9, -0x22c8
	ctx.r[3].s64 = ctx.r[9].s64 + -8904;
	// 83269180: 4BA40DA1  bl 0x82ca9f20
	ctx.lr = 0x83269184;
	sub_82CA9F20(ctx, base);
	// 83269184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326918C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269198 size=60
    let mut pc: u32 = 0x83269198;
    'dispatch: loop {
        match pc {
            0x83269198 => {
    //   block [0x83269198..0x832691D4)
	// 83269198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832691A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832691A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832691A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832691AC: 388BA8C0  addi r4, r11, -0x5740
	ctx.r[4].s64 = ctx.r[11].s64 + -22336;
	// 832691B0: 386ABB70  addi r3, r10, -0x4490
	ctx.r[3].s64 = ctx.r[10].s64 + -17552;
	// 832691B4: 4B06D255  bl 0x822d6408
	ctx.lr = 0x832691B8;
	sub_822D6408(ctx, base);
	// 832691B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832691BC: 3869DD48  addi r3, r9, -0x22b8
	ctx.r[3].s64 = ctx.r[9].s64 + -8888;
	// 832691C0: 4BA40D61  bl 0x82ca9f20
	ctx.lr = 0x832691C4;
	sub_82CA9F20(ctx, base);
	// 832691C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832691C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832691CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832691D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832691D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832691D8 size=60
    let mut pc: u32 = 0x832691D8;
    'dispatch: loop {
        match pc {
            0x832691D8 => {
    //   block [0x832691D8..0x83269214)
	// 832691D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832691DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832691E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832691E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832691E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832691EC: 388BA8D4  addi r4, r11, -0x572c
	ctx.r[4].s64 = ctx.r[11].s64 + -22316;
	// 832691F0: 386ABB74  addi r3, r10, -0x448c
	ctx.r[3].s64 = ctx.r[10].s64 + -17548;
	// 832691F4: 4B06D215  bl 0x822d6408
	ctx.lr = 0x832691F8;
	sub_822D6408(ctx, base);
	// 832691F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832691FC: 3869DD58  addi r3, r9, -0x22a8
	ctx.r[3].s64 = ctx.r[9].s64 + -8872;
	// 83269200: 4BA40D21  bl 0x82ca9f20
	ctx.lr = 0x83269204;
	sub_82CA9F20(ctx, base);
	// 83269204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326920C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269218 size=60
    let mut pc: u32 = 0x83269218;
    'dispatch: loop {
        match pc {
            0x83269218 => {
    //   block [0x83269218..0x83269254)
	// 83269218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326921C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269224: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326922C: 388BA8E8  addi r4, r11, -0x5718
	ctx.r[4].s64 = ctx.r[11].s64 + -22296;
	// 83269230: 386ABB78  addi r3, r10, -0x4488
	ctx.r[3].s64 = ctx.r[10].s64 + -17544;
	// 83269234: 4B06D1D5  bl 0x822d6408
	ctx.lr = 0x83269238;
	sub_822D6408(ctx, base);
	// 83269238: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326923C: 3869DD68  addi r3, r9, -0x2298
	ctx.r[3].s64 = ctx.r[9].s64 + -8856;
	// 83269240: 4BA40CE1  bl 0x82ca9f20
	ctx.lr = 0x83269244;
	sub_82CA9F20(ctx, base);
	// 83269244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326924C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269258 size=60
    let mut pc: u32 = 0x83269258;
    'dispatch: loop {
        match pc {
            0x83269258 => {
    //   block [0x83269258..0x83269294)
	// 83269258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326925C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269264: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326926C: 388BA8FC  addi r4, r11, -0x5704
	ctx.r[4].s64 = ctx.r[11].s64 + -22276;
	// 83269270: 386ABB7C  addi r3, r10, -0x4484
	ctx.r[3].s64 = ctx.r[10].s64 + -17540;
	// 83269274: 4B06D195  bl 0x822d6408
	ctx.lr = 0x83269278;
	sub_822D6408(ctx, base);
	// 83269278: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326927C: 3869DD78  addi r3, r9, -0x2288
	ctx.r[3].s64 = ctx.r[9].s64 + -8840;
	// 83269280: 4BA40CA1  bl 0x82ca9f20
	ctx.lr = 0x83269284;
	sub_82CA9F20(ctx, base);
	// 83269284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326928C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269298 size=60
    let mut pc: u32 = 0x83269298;
    'dispatch: loop {
        match pc {
            0x83269298 => {
    //   block [0x83269298..0x832692D4)
	// 83269298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326929C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832692A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832692A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832692A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832692AC: 388BA91C  addi r4, r11, -0x56e4
	ctx.r[4].s64 = ctx.r[11].s64 + -22244;
	// 832692B0: 386ABB80  addi r3, r10, -0x4480
	ctx.r[3].s64 = ctx.r[10].s64 + -17536;
	// 832692B4: 4B06D155  bl 0x822d6408
	ctx.lr = 0x832692B8;
	sub_822D6408(ctx, base);
	// 832692B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832692BC: 3869DD88  addi r3, r9, -0x2278
	ctx.r[3].s64 = ctx.r[9].s64 + -8824;
	// 832692C0: 4BA40C61  bl 0x82ca9f20
	ctx.lr = 0x832692C4;
	sub_82CA9F20(ctx, base);
	// 832692C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832692C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832692CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832692D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832692D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832692D8 size=60
    let mut pc: u32 = 0x832692D8;
    'dispatch: loop {
        match pc {
            0x832692D8 => {
    //   block [0x832692D8..0x83269314)
	// 832692D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832692DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832692E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832692E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832692E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832692EC: 388BA938  addi r4, r11, -0x56c8
	ctx.r[4].s64 = ctx.r[11].s64 + -22216;
	// 832692F0: 386ABB84  addi r3, r10, -0x447c
	ctx.r[3].s64 = ctx.r[10].s64 + -17532;
	// 832692F4: 4B06D115  bl 0x822d6408
	ctx.lr = 0x832692F8;
	sub_822D6408(ctx, base);
	// 832692F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832692FC: 3869DD98  addi r3, r9, -0x2268
	ctx.r[3].s64 = ctx.r[9].s64 + -8808;
	// 83269300: 4BA40C21  bl 0x82ca9f20
	ctx.lr = 0x83269304;
	sub_82CA9F20(ctx, base);
	// 83269304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326930C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269318 size=60
    let mut pc: u32 = 0x83269318;
    'dispatch: loop {
        match pc {
            0x83269318 => {
    //   block [0x83269318..0x83269354)
	// 83269318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326931C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269324: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326932C: 388BA954  addi r4, r11, -0x56ac
	ctx.r[4].s64 = ctx.r[11].s64 + -22188;
	// 83269330: 386ABB88  addi r3, r10, -0x4478
	ctx.r[3].s64 = ctx.r[10].s64 + -17528;
	// 83269334: 4B06D0D5  bl 0x822d6408
	ctx.lr = 0x83269338;
	sub_822D6408(ctx, base);
	// 83269338: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326933C: 3869DDA8  addi r3, r9, -0x2258
	ctx.r[3].s64 = ctx.r[9].s64 + -8792;
	// 83269340: 4BA40BE1  bl 0x82ca9f20
	ctx.lr = 0x83269344;
	sub_82CA9F20(ctx, base);
	// 83269344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326934C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269358 size=60
    let mut pc: u32 = 0x83269358;
    'dispatch: loop {
        match pc {
            0x83269358 => {
    //   block [0x83269358..0x83269394)
	// 83269358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326935C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269364: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326936C: 388BA970  addi r4, r11, -0x5690
	ctx.r[4].s64 = ctx.r[11].s64 + -22160;
	// 83269370: 386ABB8C  addi r3, r10, -0x4474
	ctx.r[3].s64 = ctx.r[10].s64 + -17524;
	// 83269374: 4B06D095  bl 0x822d6408
	ctx.lr = 0x83269378;
	sub_822D6408(ctx, base);
	// 83269378: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326937C: 3869DDB8  addi r3, r9, -0x2248
	ctx.r[3].s64 = ctx.r[9].s64 + -8776;
	// 83269380: 4BA40BA1  bl 0x82ca9f20
	ctx.lr = 0x83269384;
	sub_82CA9F20(ctx, base);
	// 83269384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326938C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269398 size=60
    let mut pc: u32 = 0x83269398;
    'dispatch: loop {
        match pc {
            0x83269398 => {
    //   block [0x83269398..0x832693D4)
	// 83269398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832693A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832693A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832693A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832693AC: 388BA990  addi r4, r11, -0x5670
	ctx.r[4].s64 = ctx.r[11].s64 + -22128;
	// 832693B0: 386ABB90  addi r3, r10, -0x4470
	ctx.r[3].s64 = ctx.r[10].s64 + -17520;
	// 832693B4: 4B06D055  bl 0x822d6408
	ctx.lr = 0x832693B8;
	sub_822D6408(ctx, base);
	// 832693B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832693BC: 3869DDC8  addi r3, r9, -0x2238
	ctx.r[3].s64 = ctx.r[9].s64 + -8760;
	// 832693C0: 4BA40B61  bl 0x82ca9f20
	ctx.lr = 0x832693C4;
	sub_82CA9F20(ctx, base);
	// 832693C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832693C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832693CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832693D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832693D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832693D8 size=60
    let mut pc: u32 = 0x832693D8;
    'dispatch: loop {
        match pc {
            0x832693D8 => {
    //   block [0x832693D8..0x83269414)
	// 832693D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832693DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832693E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832693E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832693E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832693EC: 388BA9A8  addi r4, r11, -0x5658
	ctx.r[4].s64 = ctx.r[11].s64 + -22104;
	// 832693F0: 386ABB94  addi r3, r10, -0x446c
	ctx.r[3].s64 = ctx.r[10].s64 + -17516;
	// 832693F4: 4B06D015  bl 0x822d6408
	ctx.lr = 0x832693F8;
	sub_822D6408(ctx, base);
	// 832693F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832693FC: 3869DDD8  addi r3, r9, -0x2228
	ctx.r[3].s64 = ctx.r[9].s64 + -8744;
	// 83269400: 4BA40B21  bl 0x82ca9f20
	ctx.lr = 0x83269404;
	sub_82CA9F20(ctx, base);
	// 83269404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326940C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269418 size=60
    let mut pc: u32 = 0x83269418;
    'dispatch: loop {
        match pc {
            0x83269418 => {
    //   block [0x83269418..0x83269454)
	// 83269418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326941C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269424: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326942C: 388BA9BC  addi r4, r11, -0x5644
	ctx.r[4].s64 = ctx.r[11].s64 + -22084;
	// 83269430: 386ABB98  addi r3, r10, -0x4468
	ctx.r[3].s64 = ctx.r[10].s64 + -17512;
	// 83269434: 4B06CFD5  bl 0x822d6408
	ctx.lr = 0x83269438;
	sub_822D6408(ctx, base);
	// 83269438: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326943C: 3869DDE8  addi r3, r9, -0x2218
	ctx.r[3].s64 = ctx.r[9].s64 + -8728;
	// 83269440: 4BA40AE1  bl 0x82ca9f20
	ctx.lr = 0x83269444;
	sub_82CA9F20(ctx, base);
	// 83269444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326944C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269458 size=60
    let mut pc: u32 = 0x83269458;
    'dispatch: loop {
        match pc {
            0x83269458 => {
    //   block [0x83269458..0x83269494)
	// 83269458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326945C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269464: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326946C: 388BA9DC  addi r4, r11, -0x5624
	ctx.r[4].s64 = ctx.r[11].s64 + -22052;
	// 83269470: 386ABB9C  addi r3, r10, -0x4464
	ctx.r[3].s64 = ctx.r[10].s64 + -17508;
	// 83269474: 4B06CF95  bl 0x822d6408
	ctx.lr = 0x83269478;
	sub_822D6408(ctx, base);
	// 83269478: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326947C: 3869DDF8  addi r3, r9, -0x2208
	ctx.r[3].s64 = ctx.r[9].s64 + -8712;
	// 83269480: 4BA40AA1  bl 0x82ca9f20
	ctx.lr = 0x83269484;
	sub_82CA9F20(ctx, base);
	// 83269484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269498 size=60
    let mut pc: u32 = 0x83269498;
    'dispatch: loop {
        match pc {
            0x83269498 => {
    //   block [0x83269498..0x832694D4)
	// 83269498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832694A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832694A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832694A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832694AC: 388BA9F8  addi r4, r11, -0x5608
	ctx.r[4].s64 = ctx.r[11].s64 + -22024;
	// 832694B0: 386ABBA0  addi r3, r10, -0x4460
	ctx.r[3].s64 = ctx.r[10].s64 + -17504;
	// 832694B4: 4B06CF55  bl 0x822d6408
	ctx.lr = 0x832694B8;
	sub_822D6408(ctx, base);
	// 832694B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832694BC: 3869DE08  addi r3, r9, -0x21f8
	ctx.r[3].s64 = ctx.r[9].s64 + -8696;
	// 832694C0: 4BA40A61  bl 0x82ca9f20
	ctx.lr = 0x832694C4;
	sub_82CA9F20(ctx, base);
	// 832694C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832694C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832694CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832694D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832694D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832694D8 size=60
    let mut pc: u32 = 0x832694D8;
    'dispatch: loop {
        match pc {
            0x832694D8 => {
    //   block [0x832694D8..0x83269514)
	// 832694D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832694DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832694E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832694E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832694E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832694EC: 388BAA14  addi r4, r11, -0x55ec
	ctx.r[4].s64 = ctx.r[11].s64 + -21996;
	// 832694F0: 386ABBA4  addi r3, r10, -0x445c
	ctx.r[3].s64 = ctx.r[10].s64 + -17500;
	// 832694F4: 4B06CF15  bl 0x822d6408
	ctx.lr = 0x832694F8;
	sub_822D6408(ctx, base);
	// 832694F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832694FC: 3869DE18  addi r3, r9, -0x21e8
	ctx.r[3].s64 = ctx.r[9].s64 + -8680;
	// 83269500: 4BA40A21  bl 0x82ca9f20
	ctx.lr = 0x83269504;
	sub_82CA9F20(ctx, base);
	// 83269504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326950C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269518 size=60
    let mut pc: u32 = 0x83269518;
    'dispatch: loop {
        match pc {
            0x83269518 => {
    //   block [0x83269518..0x83269554)
	// 83269518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326951C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269524: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326952C: 388BAA34  addi r4, r11, -0x55cc
	ctx.r[4].s64 = ctx.r[11].s64 + -21964;
	// 83269530: 386ABBA8  addi r3, r10, -0x4458
	ctx.r[3].s64 = ctx.r[10].s64 + -17496;
	// 83269534: 4B06CED5  bl 0x822d6408
	ctx.lr = 0x83269538;
	sub_822D6408(ctx, base);
	// 83269538: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326953C: 3869DE28  addi r3, r9, -0x21d8
	ctx.r[3].s64 = ctx.r[9].s64 + -8664;
	// 83269540: 4BA409E1  bl 0x82ca9f20
	ctx.lr = 0x83269544;
	sub_82CA9F20(ctx, base);
	// 83269544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326954C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269558 size=60
    let mut pc: u32 = 0x83269558;
    'dispatch: loop {
        match pc {
            0x83269558 => {
    //   block [0x83269558..0x83269594)
	// 83269558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269564: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326956C: 388BA894  addi r4, r11, -0x576c
	ctx.r[4].s64 = ctx.r[11].s64 + -22380;
	// 83269570: 386ABBAC  addi r3, r10, -0x4454
	ctx.r[3].s64 = ctx.r[10].s64 + -17492;
	// 83269574: 4B06CE95  bl 0x822d6408
	ctx.lr = 0x83269578;
	sub_822D6408(ctx, base);
	// 83269578: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326957C: 3869DE38  addi r3, r9, -0x21c8
	ctx.r[3].s64 = ctx.r[9].s64 + -8648;
	// 83269580: 4BA409A1  bl 0x82ca9f20
	ctx.lr = 0x83269584;
	sub_82CA9F20(ctx, base);
	// 83269584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269598 size=60
    let mut pc: u32 = 0x83269598;
    'dispatch: loop {
        match pc {
            0x83269598 => {
    //   block [0x83269598..0x832695D4)
	// 83269598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326959C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832695A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832695A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832695A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832695AC: 388BAA58  addi r4, r11, -0x55a8
	ctx.r[4].s64 = ctx.r[11].s64 + -21928;
	// 832695B0: 386ABBB0  addi r3, r10, -0x4450
	ctx.r[3].s64 = ctx.r[10].s64 + -17488;
	// 832695B4: 4B06CE55  bl 0x822d6408
	ctx.lr = 0x832695B8;
	sub_822D6408(ctx, base);
	// 832695B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832695BC: 3869DE48  addi r3, r9, -0x21b8
	ctx.r[3].s64 = ctx.r[9].s64 + -8632;
	// 832695C0: 4BA40961  bl 0x82ca9f20
	ctx.lr = 0x832695C4;
	sub_82CA9F20(ctx, base);
	// 832695C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832695C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832695CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832695D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832695D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832695D8 size=60
    let mut pc: u32 = 0x832695D8;
    'dispatch: loop {
        match pc {
            0x832695D8 => {
    //   block [0x832695D8..0x83269614)
	// 832695D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832695DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832695E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832695E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832695E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832695EC: 388BAA70  addi r4, r11, -0x5590
	ctx.r[4].s64 = ctx.r[11].s64 + -21904;
	// 832695F0: 386ABBB4  addi r3, r10, -0x444c
	ctx.r[3].s64 = ctx.r[10].s64 + -17484;
	// 832695F4: 4B06CE15  bl 0x822d6408
	ctx.lr = 0x832695F8;
	sub_822D6408(ctx, base);
	// 832695F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832695FC: 3869DE58  addi r3, r9, -0x21a8
	ctx.r[3].s64 = ctx.r[9].s64 + -8616;
	// 83269600: 4BA40921  bl 0x82ca9f20
	ctx.lr = 0x83269604;
	sub_82CA9F20(ctx, base);
	// 83269604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326960C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269618 size=60
    let mut pc: u32 = 0x83269618;
    'dispatch: loop {
        match pc {
            0x83269618 => {
    //   block [0x83269618..0x83269654)
	// 83269618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269624: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326962C: 388BAA88  addi r4, r11, -0x5578
	ctx.r[4].s64 = ctx.r[11].s64 + -21880;
	// 83269630: 386ABBB8  addi r3, r10, -0x4448
	ctx.r[3].s64 = ctx.r[10].s64 + -17480;
	// 83269634: 4B06CDD5  bl 0x822d6408
	ctx.lr = 0x83269638;
	sub_822D6408(ctx, base);
	// 83269638: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326963C: 3869DE68  addi r3, r9, -0x2198
	ctx.r[3].s64 = ctx.r[9].s64 + -8600;
	// 83269640: 4BA408E1  bl 0x82ca9f20
	ctx.lr = 0x83269644;
	sub_82CA9F20(ctx, base);
	// 83269644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269658 size=60
    let mut pc: u32 = 0x83269658;
    'dispatch: loop {
        match pc {
            0x83269658 => {
    //   block [0x83269658..0x83269694)
	// 83269658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326965C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269664: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326966C: 388BAAA0  addi r4, r11, -0x5560
	ctx.r[4].s64 = ctx.r[11].s64 + -21856;
	// 83269670: 386ABBBC  addi r3, r10, -0x4444
	ctx.r[3].s64 = ctx.r[10].s64 + -17476;
	// 83269674: 4B06CD95  bl 0x822d6408
	ctx.lr = 0x83269678;
	sub_822D6408(ctx, base);
	// 83269678: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326967C: 3869DE78  addi r3, r9, -0x2188
	ctx.r[3].s64 = ctx.r[9].s64 + -8584;
	// 83269680: 4BA408A1  bl 0x82ca9f20
	ctx.lr = 0x83269684;
	sub_82CA9F20(ctx, base);
	// 83269684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326968C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269698 size=60
    let mut pc: u32 = 0x83269698;
    'dispatch: loop {
        match pc {
            0x83269698 => {
    //   block [0x83269698..0x832696D4)
	// 83269698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326969C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832696A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832696A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832696A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832696AC: 388BAAB8  addi r4, r11, -0x5548
	ctx.r[4].s64 = ctx.r[11].s64 + -21832;
	// 832696B0: 386ABBC0  addi r3, r10, -0x4440
	ctx.r[3].s64 = ctx.r[10].s64 + -17472;
	// 832696B4: 4B06CD55  bl 0x822d6408
	ctx.lr = 0x832696B8;
	sub_822D6408(ctx, base);
	// 832696B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832696BC: 3869DE88  addi r3, r9, -0x2178
	ctx.r[3].s64 = ctx.r[9].s64 + -8568;
	// 832696C0: 4BA40861  bl 0x82ca9f20
	ctx.lr = 0x832696C4;
	sub_82CA9F20(ctx, base);
	// 832696C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832696C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832696CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832696D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832696D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832696D8 size=60
    let mut pc: u32 = 0x832696D8;
    'dispatch: loop {
        match pc {
            0x832696D8 => {
    //   block [0x832696D8..0x83269714)
	// 832696D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832696DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832696E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832696E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832696E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832696EC: 388BAAD0  addi r4, r11, -0x5530
	ctx.r[4].s64 = ctx.r[11].s64 + -21808;
	// 832696F0: 386ABBC4  addi r3, r10, -0x443c
	ctx.r[3].s64 = ctx.r[10].s64 + -17468;
	// 832696F4: 4B06CD15  bl 0x822d6408
	ctx.lr = 0x832696F8;
	sub_822D6408(ctx, base);
	// 832696F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832696FC: 3869DE98  addi r3, r9, -0x2168
	ctx.r[3].s64 = ctx.r[9].s64 + -8552;
	// 83269700: 4BA40821  bl 0x82ca9f20
	ctx.lr = 0x83269704;
	sub_82CA9F20(ctx, base);
	// 83269704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269718 size=60
    let mut pc: u32 = 0x83269718;
    'dispatch: loop {
        match pc {
            0x83269718 => {
    //   block [0x83269718..0x83269754)
	// 83269718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269724: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326972C: 388BAAE4  addi r4, r11, -0x551c
	ctx.r[4].s64 = ctx.r[11].s64 + -21788;
	// 83269730: 386ABBC8  addi r3, r10, -0x4438
	ctx.r[3].s64 = ctx.r[10].s64 + -17464;
	// 83269734: 4B06CCD5  bl 0x822d6408
	ctx.lr = 0x83269738;
	sub_822D6408(ctx, base);
	// 83269738: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326973C: 3869DEA8  addi r3, r9, -0x2158
	ctx.r[3].s64 = ctx.r[9].s64 + -8536;
	// 83269740: 4BA407E1  bl 0x82ca9f20
	ctx.lr = 0x83269744;
	sub_82CA9F20(ctx, base);
	// 83269744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326974C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269758 size=60
    let mut pc: u32 = 0x83269758;
    'dispatch: loop {
        match pc {
            0x83269758 => {
    //   block [0x83269758..0x83269794)
	// 83269758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269764: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326976C: 388BAAF8  addi r4, r11, -0x5508
	ctx.r[4].s64 = ctx.r[11].s64 + -21768;
	// 83269770: 386ABBCC  addi r3, r10, -0x4434
	ctx.r[3].s64 = ctx.r[10].s64 + -17460;
	// 83269774: 4B06CC95  bl 0x822d6408
	ctx.lr = 0x83269778;
	sub_822D6408(ctx, base);
	// 83269778: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326977C: 3869DEB8  addi r3, r9, -0x2148
	ctx.r[3].s64 = ctx.r[9].s64 + -8520;
	// 83269780: 4BA407A1  bl 0x82ca9f20
	ctx.lr = 0x83269784;
	sub_82CA9F20(ctx, base);
	// 83269784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326978C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269798 size=60
    let mut pc: u32 = 0x83269798;
    'dispatch: loop {
        match pc {
            0x83269798 => {
    //   block [0x83269798..0x832697D4)
	// 83269798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326979C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832697A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832697A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832697A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832697AC: 388BAB0C  addi r4, r11, -0x54f4
	ctx.r[4].s64 = ctx.r[11].s64 + -21748;
	// 832697B0: 386ABBD0  addi r3, r10, -0x4430
	ctx.r[3].s64 = ctx.r[10].s64 + -17456;
	// 832697B4: 4B06CC55  bl 0x822d6408
	ctx.lr = 0x832697B8;
	sub_822D6408(ctx, base);
	// 832697B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832697BC: 3869DEC8  addi r3, r9, -0x2138
	ctx.r[3].s64 = ctx.r[9].s64 + -8504;
	// 832697C0: 4BA40761  bl 0x82ca9f20
	ctx.lr = 0x832697C4;
	sub_82CA9F20(ctx, base);
	// 832697C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832697C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832697CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832697D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832697D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832697D8 size=60
    let mut pc: u32 = 0x832697D8;
    'dispatch: loop {
        match pc {
            0x832697D8 => {
    //   block [0x832697D8..0x83269814)
	// 832697D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832697DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832697E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832697E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832697E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832697EC: 388BAB20  addi r4, r11, -0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + -21728;
	// 832697F0: 386ABBD4  addi r3, r10, -0x442c
	ctx.r[3].s64 = ctx.r[10].s64 + -17452;
	// 832697F4: 4B06CC15  bl 0x822d6408
	ctx.lr = 0x832697F8;
	sub_822D6408(ctx, base);
	// 832697F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832697FC: 3869DED8  addi r3, r9, -0x2128
	ctx.r[3].s64 = ctx.r[9].s64 + -8488;
	// 83269800: 4BA40721  bl 0x82ca9f20
	ctx.lr = 0x83269804;
	sub_82CA9F20(ctx, base);
	// 83269804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326980C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269818 size=60
    let mut pc: u32 = 0x83269818;
    'dispatch: loop {
        match pc {
            0x83269818 => {
    //   block [0x83269818..0x83269854)
	// 83269818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269824: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326982C: 388BAB34  addi r4, r11, -0x54cc
	ctx.r[4].s64 = ctx.r[11].s64 + -21708;
	// 83269830: 386ABBD8  addi r3, r10, -0x4428
	ctx.r[3].s64 = ctx.r[10].s64 + -17448;
	// 83269834: 4B06CBD5  bl 0x822d6408
	ctx.lr = 0x83269838;
	sub_822D6408(ctx, base);
	// 83269838: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326983C: 3869DEE8  addi r3, r9, -0x2118
	ctx.r[3].s64 = ctx.r[9].s64 + -8472;
	// 83269840: 4BA406E1  bl 0x82ca9f20
	ctx.lr = 0x83269844;
	sub_82CA9F20(ctx, base);
	// 83269844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326984C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269858 size=60
    let mut pc: u32 = 0x83269858;
    'dispatch: loop {
        match pc {
            0x83269858 => {
    //   block [0x83269858..0x83269894)
	// 83269858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326985C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269864: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269868: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326986C: 388BAB48  addi r4, r11, -0x54b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21688;
	// 83269870: 386ABBDC  addi r3, r10, -0x4424
	ctx.r[3].s64 = ctx.r[10].s64 + -17444;
	// 83269874: 4B06CB95  bl 0x822d6408
	ctx.lr = 0x83269878;
	sub_822D6408(ctx, base);
	// 83269878: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326987C: 3869DEF8  addi r3, r9, -0x2108
	ctx.r[3].s64 = ctx.r[9].s64 + -8456;
	// 83269880: 4BA406A1  bl 0x82ca9f20
	ctx.lr = 0x83269884;
	sub_82CA9F20(ctx, base);
	// 83269884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326988C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269898 size=60
    let mut pc: u32 = 0x83269898;
    'dispatch: loop {
        match pc {
            0x83269898 => {
    //   block [0x83269898..0x832698D4)
	// 83269898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326989C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832698A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832698A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832698A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832698AC: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 832698B0: 386ABBE0  addi r3, r10, -0x4420
	ctx.r[3].s64 = ctx.r[10].s64 + -17440;
	// 832698B4: 4B06CB55  bl 0x822d6408
	ctx.lr = 0x832698B8;
	sub_822D6408(ctx, base);
	// 832698B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832698BC: 3869DF08  addi r3, r9, -0x20f8
	ctx.r[3].s64 = ctx.r[9].s64 + -8440;
	// 832698C0: 4BA40661  bl 0x82ca9f20
	ctx.lr = 0x832698C4;
	sub_82CA9F20(ctx, base);
	// 832698C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832698C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832698CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832698D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832698D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832698D8 size=60
    let mut pc: u32 = 0x832698D8;
    'dispatch: loop {
        match pc {
            0x832698D8 => {
    //   block [0x832698D8..0x83269914)
	// 832698D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832698DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832698E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832698E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832698E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832698EC: 388BAB7C  addi r4, r11, -0x5484
	ctx.r[4].s64 = ctx.r[11].s64 + -21636;
	// 832698F0: 386ABBE4  addi r3, r10, -0x441c
	ctx.r[3].s64 = ctx.r[10].s64 + -17436;
	// 832698F4: 4B06CB15  bl 0x822d6408
	ctx.lr = 0x832698F8;
	sub_822D6408(ctx, base);
	// 832698F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832698FC: 3869DF18  addi r3, r9, -0x20e8
	ctx.r[3].s64 = ctx.r[9].s64 + -8424;
	// 83269900: 4BA40621  bl 0x82ca9f20
	ctx.lr = 0x83269904;
	sub_82CA9F20(ctx, base);
	// 83269904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326990C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269918 size=60
    let mut pc: u32 = 0x83269918;
    'dispatch: loop {
        match pc {
            0x83269918 => {
    //   block [0x83269918..0x83269954)
	// 83269918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326991C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269924: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326992C: 388BAB94  addi r4, r11, -0x546c
	ctx.r[4].s64 = ctx.r[11].s64 + -21612;
	// 83269930: 386ABBE8  addi r3, r10, -0x4418
	ctx.r[3].s64 = ctx.r[10].s64 + -17432;
	// 83269934: 4B06CAD5  bl 0x822d6408
	ctx.lr = 0x83269938;
	sub_822D6408(ctx, base);
	// 83269938: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326993C: 3869DF28  addi r3, r9, -0x20d8
	ctx.r[3].s64 = ctx.r[9].s64 + -8408;
	// 83269940: 4BA405E1  bl 0x82ca9f20
	ctx.lr = 0x83269944;
	sub_82CA9F20(ctx, base);
	// 83269944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326994C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269958 size=60
    let mut pc: u32 = 0x83269958;
    'dispatch: loop {
        match pc {
            0x83269958 => {
    //   block [0x83269958..0x83269994)
	// 83269958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326995C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269964: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326996C: 388BABA8  addi r4, r11, -0x5458
	ctx.r[4].s64 = ctx.r[11].s64 + -21592;
	// 83269970: 386ABBEC  addi r3, r10, -0x4414
	ctx.r[3].s64 = ctx.r[10].s64 + -17428;
	// 83269974: 4B06CA95  bl 0x822d6408
	ctx.lr = 0x83269978;
	sub_822D6408(ctx, base);
	// 83269978: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326997C: 3869DF38  addi r3, r9, -0x20c8
	ctx.r[3].s64 = ctx.r[9].s64 + -8392;
	// 83269980: 4BA405A1  bl 0x82ca9f20
	ctx.lr = 0x83269984;
	sub_82CA9F20(ctx, base);
	// 83269984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326998C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269998 size=60
    let mut pc: u32 = 0x83269998;
    'dispatch: loop {
        match pc {
            0x83269998 => {
    //   block [0x83269998..0x832699D4)
	// 83269998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832699A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832699A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832699A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832699AC: 388BABC8  addi r4, r11, -0x5438
	ctx.r[4].s64 = ctx.r[11].s64 + -21560;
	// 832699B0: 386ABBF0  addi r3, r10, -0x4410
	ctx.r[3].s64 = ctx.r[10].s64 + -17424;
	// 832699B4: 4B06CA55  bl 0x822d6408
	ctx.lr = 0x832699B8;
	sub_822D6408(ctx, base);
	// 832699B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832699BC: 3869DF48  addi r3, r9, -0x20b8
	ctx.r[3].s64 = ctx.r[9].s64 + -8376;
	// 832699C0: 4BA40561  bl 0x82ca9f20
	ctx.lr = 0x832699C4;
	sub_82CA9F20(ctx, base);
	// 832699C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832699C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832699CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832699D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832699D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832699D8 size=144
    let mut pc: u32 = 0x832699D8;
    'dispatch: loop {
        match pc {
            0x832699D8 => {
    //   block [0x832699D8..0x832699FC)
	// 832699D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832699DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832699E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832699E4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 832699E8: 4AFB5871  bl 0x8221f258
	ctx.lr = 0x832699EC;
	sub_8221F258(ctx, base);
	// 832699EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832699F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832699F4: 419A0008  beq cr6, 0x832699fc
	if ctx.cr[6].eq {
	pc = 0x832699FC; continue 'dispatch;
	}
	// 832699F8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832699FC; continue 'dispatch;
            }
            0x832699FC => {
    //   block [0x832699FC..0x83269A08)
	// 832699FC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83269A00: 41820008  beq 0x83269a08
	if ctx.cr[0].eq {
	pc = 0x83269A08; continue 'dispatch;
	}
	// 83269A04: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83269A08; continue 'dispatch;
            }
            0x83269A08 => {
    //   block [0x83269A08..0x83269A14)
	// 83269A08: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83269A0C: 41820008  beq 0x83269a14
	if ctx.cr[0].eq {
	pc = 0x83269A14; continue 'dispatch;
	}
	// 83269A10: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83269A14; continue 'dispatch;
            }
            0x83269A14 => {
    //   block [0x83269A14..0x83269A68)
	// 83269A14: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83269A18: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83269A1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83269A20: 3909BBF4  addi r8, r9, -0x440c
	ctx.r[8].s64 = ctx.r[9].s64 + -17420;
	// 83269A24: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83269A28: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83269A2C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83269A30: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83269A34: 3867DF58  addi r3, r7, -0x20a8
	ctx.r[3].s64 = ctx.r[7].s64 + -8360;
	// 83269A38: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83269A3C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83269A40: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83269A44: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83269A48: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83269A4C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83269A50: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83269A54: 4BA404CD  bl 0x82ca9f20
	ctx.lr = 0x83269A58;
	sub_82CA9F20(ctx, base);
	// 83269A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269A68 size=64
    let mut pc: u32 = 0x83269A68;
    'dispatch: loop {
        match pc {
            0x83269A68 => {
    //   block [0x83269A68..0x83269AA8)
	// 83269A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269A74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269A7C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83269A80: 386ABC00  addi r3, r10, -0x4400
	ctx.r[3].s64 = ctx.r[10].s64 + -17408;
	// 83269A84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269A88: 4AFC3449  bl 0x8222ced0
	ctx.lr = 0x83269A8C;
	sub_8222CED0(ctx, base);
	// 83269A8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269A90: 3869DF68  addi r3, r9, -0x2098
	ctx.r[3].s64 = ctx.r[9].s64 + -8344;
	// 83269A94: 4BA4048D  bl 0x82ca9f20
	ctx.lr = 0x83269A98;
	sub_82CA9F20(ctx, base);
	// 83269A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269AA8 size=64
    let mut pc: u32 = 0x83269AA8;
    'dispatch: loop {
        match pc {
            0x83269AA8 => {
    //   block [0x83269AA8..0x83269AE8)
	// 83269AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269AB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269ABC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83269AC0: 386ABC04  addi r3, r10, -0x43fc
	ctx.r[3].s64 = ctx.r[10].s64 + -17404;
	// 83269AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269AC8: 4AFC3409  bl 0x8222ced0
	ctx.lr = 0x83269ACC;
	sub_8222CED0(ctx, base);
	// 83269ACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269AD0: 3869DF78  addi r3, r9, -0x2088
	ctx.r[3].s64 = ctx.r[9].s64 + -8328;
	// 83269AD4: 4BA4044D  bl 0x82ca9f20
	ctx.lr = 0x83269AD8;
	sub_82CA9F20(ctx, base);
	// 83269AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269AE8 size=64
    let mut pc: u32 = 0x83269AE8;
    'dispatch: loop {
        match pc {
            0x83269AE8 => {
    //   block [0x83269AE8..0x83269B28)
	// 83269AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269AF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269AFC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83269B00: 386ABC08  addi r3, r10, -0x43f8
	ctx.r[3].s64 = ctx.r[10].s64 + -17400;
	// 83269B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269B08: 4AFC33C9  bl 0x8222ced0
	ctx.lr = 0x83269B0C;
	sub_8222CED0(ctx, base);
	// 83269B0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269B10: 3869DF88  addi r3, r9, -0x2078
	ctx.r[3].s64 = ctx.r[9].s64 + -8312;
	// 83269B14: 4BA4040D  bl 0x82ca9f20
	ctx.lr = 0x83269B18;
	sub_82CA9F20(ctx, base);
	// 83269B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269B28 size=64
    let mut pc: u32 = 0x83269B28;
    'dispatch: loop {
        match pc {
            0x83269B28 => {
    //   block [0x83269B28..0x83269B68)
	// 83269B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269B34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269B38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269B3C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83269B40: 386ABC0C  addi r3, r10, -0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17396;
	// 83269B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269B48: 4AFC3389  bl 0x8222ced0
	ctx.lr = 0x83269B4C;
	sub_8222CED0(ctx, base);
	// 83269B4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269B50: 3869DF98  addi r3, r9, -0x2068
	ctx.r[3].s64 = ctx.r[9].s64 + -8296;
	// 83269B54: 4BA403CD  bl 0x82ca9f20
	ctx.lr = 0x83269B58;
	sub_82CA9F20(ctx, base);
	// 83269B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269B68 size=64
    let mut pc: u32 = 0x83269B68;
    'dispatch: loop {
        match pc {
            0x83269B68 => {
    //   block [0x83269B68..0x83269BA8)
	// 83269B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269B74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83269B78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269B7C: 388BE1C8  addi r4, r11, -0x1e38
	ctx.r[4].s64 = ctx.r[11].s64 + -7736;
	// 83269B80: 386ABC10  addi r3, r10, -0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17392;
	// 83269B84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269B88: 4AFC3349  bl 0x8222ced0
	ctx.lr = 0x83269B8C;
	sub_8222CED0(ctx, base);
	// 83269B8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269B90: 3869DFA8  addi r3, r9, -0x2058
	ctx.r[3].s64 = ctx.r[9].s64 + -8280;
	// 83269B94: 4BA4038D  bl 0x82ca9f20
	ctx.lr = 0x83269B98;
	sub_82CA9F20(ctx, base);
	// 83269B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269BA8 size=64
    let mut pc: u32 = 0x83269BA8;
    'dispatch: loop {
        match pc {
            0x83269BA8 => {
    //   block [0x83269BA8..0x83269BE8)
	// 83269BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269BB4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83269BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269BBC: 388B0840  addi r4, r11, 0x840
	ctx.r[4].s64 = ctx.r[11].s64 + 2112;
	// 83269BC0: 386ABC14  addi r3, r10, -0x43ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17388;
	// 83269BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269BC8: 4AFC3309  bl 0x8222ced0
	ctx.lr = 0x83269BCC;
	sub_8222CED0(ctx, base);
	// 83269BCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269BD0: 3869DFB8  addi r3, r9, -0x2048
	ctx.r[3].s64 = ctx.r[9].s64 + -8264;
	// 83269BD4: 4BA4034D  bl 0x82ca9f20
	ctx.lr = 0x83269BD8;
	sub_82CA9F20(ctx, base);
	// 83269BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269BE8 size=96
    let mut pc: u32 = 0x83269BE8;
    'dispatch: loop {
        match pc {
            0x83269BE8 => {
    //   block [0x83269BE8..0x83269C0C)
	// 83269BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269BF4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83269BF8: 4AFB5661  bl 0x8221f258
	ctx.lr = 0x83269BFC;
	sub_8221F258(ctx, base);
	// 83269BFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83269C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83269C04: 419A0008  beq cr6, 0x83269c0c
	if ctx.cr[6].eq {
	pc = 0x83269C0C; continue 'dispatch;
	}
	// 83269C08: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83269C0C; continue 'dispatch;
            }
            0x83269C0C => {
    //   block [0x83269C0C..0x83269C18)
	// 83269C0C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83269C10: 41820008  beq 0x83269c18
	if ctx.cr[0].eq {
	pc = 0x83269C18; continue 'dispatch;
	}
	// 83269C14: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83269C18; continue 'dispatch;
            }
            0x83269C18 => {
    //   block [0x83269C18..0x83269C48)
	// 83269C18: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83269C1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83269C20: 3909BC18  addi r8, r9, -0x43e8
	ctx.r[8].s64 = ctx.r[9].s64 + -17384;
	// 83269C24: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83269C28: 3867DFC8  addi r3, r7, -0x2038
	ctx.r[3].s64 = ctx.r[7].s64 + -8248;
	// 83269C2C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83269C30: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83269C34: 4BA402ED  bl 0x82ca9f20
	ctx.lr = 0x83269C38;
	sub_82CA9F20(ctx, base);
	// 83269C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269C48 size=96
    let mut pc: u32 = 0x83269C48;
    'dispatch: loop {
        match pc {
            0x83269C48 => {
    //   block [0x83269C48..0x83269C6C)
	// 83269C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269C54: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 83269C58: 4AFB5601  bl 0x8221f258
	ctx.lr = 0x83269C5C;
	sub_8221F258(ctx, base);
	// 83269C5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83269C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83269C64: 419A0008  beq cr6, 0x83269c6c
	if ctx.cr[6].eq {
	pc = 0x83269C6C; continue 'dispatch;
	}
	// 83269C68: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83269C6C; continue 'dispatch;
            }
            0x83269C6C => {
    //   block [0x83269C6C..0x83269C78)
	// 83269C6C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83269C70: 41820008  beq 0x83269c78
	if ctx.cr[0].eq {
	pc = 0x83269C78; continue 'dispatch;
	}
	// 83269C74: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83269C78; continue 'dispatch;
            }
            0x83269C78 => {
    //   block [0x83269C78..0x83269CA8)
	// 83269C78: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83269C7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83269C80: 3909BC24  addi r8, r9, -0x43dc
	ctx.r[8].s64 = ctx.r[9].s64 + -17372;
	// 83269C84: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83269C88: 3867DFD8  addi r3, r7, -0x2028
	ctx.r[3].s64 = ctx.r[7].s64 + -8232;
	// 83269C8C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83269C90: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83269C94: 4BA4028D  bl 0x82ca9f20
	ctx.lr = 0x83269C98;
	sub_82CA9F20(ctx, base);
	// 83269C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269CA8 size=64
    let mut pc: u32 = 0x83269CA8;
    'dispatch: loop {
        match pc {
            0x83269CA8 => {
    //   block [0x83269CA8..0x83269CE8)
	// 83269CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269CB4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269CBC: 388BC288  addi r4, r11, -0x3d78
	ctx.r[4].s64 = ctx.r[11].s64 + -15736;
	// 83269CC0: 386ABC30  addi r3, r10, -0x43d0
	ctx.r[3].s64 = ctx.r[10].s64 + -17360;
	// 83269CC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269CC8: 4AFC3209  bl 0x8222ced0
	ctx.lr = 0x83269CCC;
	sub_8222CED0(ctx, base);
	// 83269CCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269CD0: 3869E098  addi r3, r9, -0x1f68
	ctx.r[3].s64 = ctx.r[9].s64 + -8040;
	// 83269CD4: 4BA4024D  bl 0x82ca9f20
	ctx.lr = 0x83269CD8;
	sub_82CA9F20(ctx, base);
	// 83269CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269CE8 size=64
    let mut pc: u32 = 0x83269CE8;
    'dispatch: loop {
        match pc {
            0x83269CE8 => {
    //   block [0x83269CE8..0x83269D28)
	// 83269CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269CF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269CF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269CFC: 388BC288  addi r4, r11, -0x3d78
	ctx.r[4].s64 = ctx.r[11].s64 + -15736;
	// 83269D00: 386ABC34  addi r3, r10, -0x43cc
	ctx.r[3].s64 = ctx.r[10].s64 + -17356;
	// 83269D04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269D08: 4AFC31C9  bl 0x8222ced0
	ctx.lr = 0x83269D0C;
	sub_8222CED0(ctx, base);
	// 83269D0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269D10: 3869E0A8  addi r3, r9, -0x1f58
	ctx.r[3].s64 = ctx.r[9].s64 + -8024;
	// 83269D14: 4BA4020D  bl 0x82ca9f20
	ctx.lr = 0x83269D18;
	sub_82CA9F20(ctx, base);
	// 83269D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269D28 size=64
    let mut pc: u32 = 0x83269D28;
    'dispatch: loop {
        match pc {
            0x83269D28 => {
    //   block [0x83269D28..0x83269D68)
	// 83269D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269D34: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269D3C: 388BC294  addi r4, r11, -0x3d6c
	ctx.r[4].s64 = ctx.r[11].s64 + -15724;
	// 83269D40: 386ABC38  addi r3, r10, -0x43c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17352;
	// 83269D44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269D48: 4AFC3189  bl 0x8222ced0
	ctx.lr = 0x83269D4C;
	sub_8222CED0(ctx, base);
	// 83269D4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269D50: 3869E0B8  addi r3, r9, -0x1f48
	ctx.r[3].s64 = ctx.r[9].s64 + -8008;
	// 83269D54: 4BA401CD  bl 0x82ca9f20
	ctx.lr = 0x83269D58;
	sub_82CA9F20(ctx, base);
	// 83269D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269D68 size=64
    let mut pc: u32 = 0x83269D68;
    'dispatch: loop {
        match pc {
            0x83269D68 => {
    //   block [0x83269D68..0x83269DA8)
	// 83269D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269D74: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269D7C: 388BC29C  addi r4, r11, -0x3d64
	ctx.r[4].s64 = ctx.r[11].s64 + -15716;
	// 83269D80: 386ABC3C  addi r3, r10, -0x43c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17348;
	// 83269D84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269D88: 4AFC3149  bl 0x8222ced0
	ctx.lr = 0x83269D8C;
	sub_8222CED0(ctx, base);
	// 83269D8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269D90: 3869E0C8  addi r3, r9, -0x1f38
	ctx.r[3].s64 = ctx.r[9].s64 + -7992;
	// 83269D94: 4BA4018D  bl 0x82ca9f20
	ctx.lr = 0x83269D98;
	sub_82CA9F20(ctx, base);
	// 83269D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269DA8 size=64
    let mut pc: u32 = 0x83269DA8;
    'dispatch: loop {
        match pc {
            0x83269DA8 => {
    //   block [0x83269DA8..0x83269DE8)
	// 83269DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269DB4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269DBC: 388BC2A4  addi r4, r11, -0x3d5c
	ctx.r[4].s64 = ctx.r[11].s64 + -15708;
	// 83269DC0: 386ABC40  addi r3, r10, -0x43c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17344;
	// 83269DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269DC8: 4AFC3109  bl 0x8222ced0
	ctx.lr = 0x83269DCC;
	sub_8222CED0(ctx, base);
	// 83269DCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269DD0: 3869E0D8  addi r3, r9, -0x1f28
	ctx.r[3].s64 = ctx.r[9].s64 + -7976;
	// 83269DD4: 4BA4014D  bl 0x82ca9f20
	ctx.lr = 0x83269DD8;
	sub_82CA9F20(ctx, base);
	// 83269DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269DE8 size=64
    let mut pc: u32 = 0x83269DE8;
    'dispatch: loop {
        match pc {
            0x83269DE8 => {
    //   block [0x83269DE8..0x83269E28)
	// 83269DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269DF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269DFC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83269E00: 386ABC44  addi r3, r10, -0x43bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17340;
	// 83269E04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269E08: 4AFC30C9  bl 0x8222ced0
	ctx.lr = 0x83269E0C;
	sub_8222CED0(ctx, base);
	// 83269E0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269E10: 3869E0E8  addi r3, r9, -0x1f18
	ctx.r[3].s64 = ctx.r[9].s64 + -7960;
	// 83269E14: 4BA4010D  bl 0x82ca9f20
	ctx.lr = 0x83269E18;
	sub_82CA9F20(ctx, base);
	// 83269E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269E28 size=64
    let mut pc: u32 = 0x83269E28;
    'dispatch: loop {
        match pc {
            0x83269E28 => {
    //   block [0x83269E28..0x83269E68)
	// 83269E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269E34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269E3C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83269E40: 386ABC48  addi r3, r10, -0x43b8
	ctx.r[3].s64 = ctx.r[10].s64 + -17336;
	// 83269E44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269E48: 4AFC3089  bl 0x8222ced0
	ctx.lr = 0x83269E4C;
	sub_8222CED0(ctx, base);
	// 83269E4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269E50: 3869E0F8  addi r3, r9, -0x1f08
	ctx.r[3].s64 = ctx.r[9].s64 + -7944;
	// 83269E54: 4BA400CD  bl 0x82ca9f20
	ctx.lr = 0x83269E58;
	sub_82CA9F20(ctx, base);
	// 83269E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269E68 size=64
    let mut pc: u32 = 0x83269E68;
    'dispatch: loop {
        match pc {
            0x83269E68 => {
    //   block [0x83269E68..0x83269EA8)
	// 83269E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269E74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269E7C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83269E80: 386ABC4C  addi r3, r10, -0x43b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17332;
	// 83269E84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269E88: 4AFC3049  bl 0x8222ced0
	ctx.lr = 0x83269E8C;
	sub_8222CED0(ctx, base);
	// 83269E8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269E90: 3869E108  addi r3, r9, -0x1ef8
	ctx.r[3].s64 = ctx.r[9].s64 + -7928;
	// 83269E94: 4BA4008D  bl 0x82ca9f20
	ctx.lr = 0x83269E98;
	sub_82CA9F20(ctx, base);
	// 83269E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269EA8 size=64
    let mut pc: u32 = 0x83269EA8;
    'dispatch: loop {
        match pc {
            0x83269EA8 => {
    //   block [0x83269EA8..0x83269EE8)
	// 83269EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269EB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269EB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269EBC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83269EC0: 386ABC50  addi r3, r10, -0x43b0
	ctx.r[3].s64 = ctx.r[10].s64 + -17328;
	// 83269EC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269EC8: 4AFC3009  bl 0x8222ced0
	ctx.lr = 0x83269ECC;
	sub_8222CED0(ctx, base);
	// 83269ECC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269ED0: 3869E118  addi r3, r9, -0x1ee8
	ctx.r[3].s64 = ctx.r[9].s64 + -7912;
	// 83269ED4: 4BA4004D  bl 0x82ca9f20
	ctx.lr = 0x83269ED8;
	sub_82CA9F20(ctx, base);
	// 83269ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269EE8 size=64
    let mut pc: u32 = 0x83269EE8;
    'dispatch: loop {
        match pc {
            0x83269EE8 => {
    //   block [0x83269EE8..0x83269F28)
	// 83269EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269EF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269EFC: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 83269F00: 386ABC54  addi r3, r10, -0x43ac
	ctx.r[3].s64 = ctx.r[10].s64 + -17324;
	// 83269F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269F08: 4AFC2FC9  bl 0x8222ced0
	ctx.lr = 0x83269F0C;
	sub_8222CED0(ctx, base);
	// 83269F0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269F10: 3869E128  addi r3, r9, -0x1ed8
	ctx.r[3].s64 = ctx.r[9].s64 + -7896;
	// 83269F14: 4BA4000D  bl 0x82ca9f20
	ctx.lr = 0x83269F18;
	sub_82CA9F20(ctx, base);
	// 83269F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269F28 size=64
    let mut pc: u32 = 0x83269F28;
    'dispatch: loop {
        match pc {
            0x83269F28 => {
    //   block [0x83269F28..0x83269F68)
	// 83269F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269F34: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269F3C: 388BCEE4  addi r4, r11, -0x311c
	ctx.r[4].s64 = ctx.r[11].s64 + -12572;
	// 83269F40: 386ABC58  addi r3, r10, -0x43a8
	ctx.r[3].s64 = ctx.r[10].s64 + -17320;
	// 83269F44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269F48: 4AFC2F89  bl 0x8222ced0
	ctx.lr = 0x83269F4C;
	sub_8222CED0(ctx, base);
	// 83269F4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269F50: 3869E138  addi r3, r9, -0x1ec8
	ctx.r[3].s64 = ctx.r[9].s64 + -7880;
	// 83269F54: 4BA3FFCD  bl 0x82ca9f20
	ctx.lr = 0x83269F58;
	sub_82CA9F20(ctx, base);
	// 83269F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269F68 size=64
    let mut pc: u32 = 0x83269F68;
    'dispatch: loop {
        match pc {
            0x83269F68 => {
    //   block [0x83269F68..0x83269FA8)
	// 83269F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269F74: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83269F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269F7C: 388BCEEC  addi r4, r11, -0x3114
	ctx.r[4].s64 = ctx.r[11].s64 + -12564;
	// 83269F80: 386ABC5C  addi r3, r10, -0x43a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17316;
	// 83269F84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269F88: 4AFC2F49  bl 0x8222ced0
	ctx.lr = 0x83269F8C;
	sub_8222CED0(ctx, base);
	// 83269F8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269F90: 3869E148  addi r3, r9, -0x1eb8
	ctx.r[3].s64 = ctx.r[9].s64 + -7864;
	// 83269F94: 4BA3FF8D  bl 0x82ca9f20
	ctx.lr = 0x83269F98;
	sub_82CA9F20(ctx, base);
	// 83269F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269FA8 size=64
    let mut pc: u32 = 0x83269FA8;
    'dispatch: loop {
        match pc {
            0x83269FA8 => {
    //   block [0x83269FA8..0x83269FE8)
	// 83269FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269FB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83269FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269FBC: 388B5650  addi r4, r11, 0x5650
	ctx.r[4].s64 = ctx.r[11].s64 + 22096;
	// 83269FC0: 386ABC60  addi r3, r10, -0x43a0
	ctx.r[3].s64 = ctx.r[10].s64 + -17312;
	// 83269FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83269FC8: 4AFC2F09  bl 0x8222ced0
	ctx.lr = 0x83269FCC;
	sub_8222CED0(ctx, base);
	// 83269FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83269FD0: 3869E158  addi r3, r9, -0x1ea8
	ctx.r[3].s64 = ctx.r[9].s64 + -7848;
	// 83269FD4: 4BA3FF4D  bl 0x82ca9f20
	ctx.lr = 0x83269FD8;
	sub_82CA9F20(ctx, base);
	// 83269FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83269FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269FE8 size=64
    let mut pc: u32 = 0x83269FE8;
    'dispatch: loop {
        match pc {
            0x83269FE8 => {
    //   block [0x83269FE8..0x8326A028)
	// 83269FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83269FF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83269FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83269FFC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326A000: 386ABC64  addi r3, r10, -0x439c
	ctx.r[3].s64 = ctx.r[10].s64 + -17308;
	// 8326A004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A008: 4AFC2EC9  bl 0x8222ced0
	ctx.lr = 0x8326A00C;
	sub_8222CED0(ctx, base);
	// 8326A00C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A010: 3869E168  addi r3, r9, -0x1e98
	ctx.r[3].s64 = ctx.r[9].s64 + -7832;
	// 8326A014: 4BA3FF0D  bl 0x82ca9f20
	ctx.lr = 0x8326A018;
	sub_82CA9F20(ctx, base);
	// 8326A018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A028 size=64
    let mut pc: u32 = 0x8326A028;
    'dispatch: loop {
        match pc {
            0x8326A028 => {
    //   block [0x8326A028..0x8326A068)
	// 8326A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A034: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326A038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A03C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326A040: 386ABC68  addi r3, r10, -0x4398
	ctx.r[3].s64 = ctx.r[10].s64 + -17304;
	// 8326A044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A048: 4AFC2E89  bl 0x8222ced0
	ctx.lr = 0x8326A04C;
	sub_8222CED0(ctx, base);
	// 8326A04C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A050: 3869E178  addi r3, r9, -0x1e88
	ctx.r[3].s64 = ctx.r[9].s64 + -7816;
	// 8326A054: 4BA3FECD  bl 0x82ca9f20
	ctx.lr = 0x8326A058;
	sub_82CA9F20(ctx, base);
	// 8326A058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A068 size=64
    let mut pc: u32 = 0x8326A068;
    'dispatch: loop {
        match pc {
            0x8326A068 => {
    //   block [0x8326A068..0x8326A0A8)
	// 8326A068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A074: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326A078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A07C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326A080: 386ABC6C  addi r3, r10, -0x4394
	ctx.r[3].s64 = ctx.r[10].s64 + -17300;
	// 8326A084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A088: 4AFC2E49  bl 0x8222ced0
	ctx.lr = 0x8326A08C;
	sub_8222CED0(ctx, base);
	// 8326A08C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A090: 3869E188  addi r3, r9, -0x1e78
	ctx.r[3].s64 = ctx.r[9].s64 + -7800;
	// 8326A094: 4BA3FE8D  bl 0x82ca9f20
	ctx.lr = 0x8326A098;
	sub_82CA9F20(ctx, base);
	// 8326A098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A0A8 size=64
    let mut pc: u32 = 0x8326A0A8;
    'dispatch: loop {
        match pc {
            0x8326A0A8 => {
    //   block [0x8326A0A8..0x8326A0E8)
	// 8326A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A0B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A0B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326A0B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A0BC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326A0C0: 386ABC70  addi r3, r10, -0x4390
	ctx.r[3].s64 = ctx.r[10].s64 + -17296;
	// 8326A0C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A0C8: 4AFC2E09  bl 0x8222ced0
	ctx.lr = 0x8326A0CC;
	sub_8222CED0(ctx, base);
	// 8326A0CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A0D0: 3869E198  addi r3, r9, -0x1e68
	ctx.r[3].s64 = ctx.r[9].s64 + -7784;
	// 8326A0D4: 4BA3FE4D  bl 0x82ca9f20
	ctx.lr = 0x8326A0D8;
	sub_82CA9F20(ctx, base);
	// 8326A0D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A0E8 size=64
    let mut pc: u32 = 0x8326A0E8;
    'dispatch: loop {
        match pc {
            0x8326A0E8 => {
    //   block [0x8326A0E8..0x8326A128)
	// 8326A0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A0F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A0F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326A0F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A0FC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326A100: 386ABC74  addi r3, r10, -0x438c
	ctx.r[3].s64 = ctx.r[10].s64 + -17292;
	// 8326A104: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A108: 4AFC2DC9  bl 0x8222ced0
	ctx.lr = 0x8326A10C;
	sub_8222CED0(ctx, base);
	// 8326A10C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A110: 3869E1A8  addi r3, r9, -0x1e58
	ctx.r[3].s64 = ctx.r[9].s64 + -7768;
	// 8326A114: 4BA3FE0D  bl 0x82ca9f20
	ctx.lr = 0x8326A118;
	sub_82CA9F20(ctx, base);
	// 8326A118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A128 size=64
    let mut pc: u32 = 0x8326A128;
    'dispatch: loop {
        match pc {
            0x8326A128 => {
    //   block [0x8326A128..0x8326A168)
	// 8326A128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A134: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326A138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A13C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326A140: 386ABC78  addi r3, r10, -0x4388
	ctx.r[3].s64 = ctx.r[10].s64 + -17288;
	// 8326A144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A148: 4AFC2D89  bl 0x8222ced0
	ctx.lr = 0x8326A14C;
	sub_8222CED0(ctx, base);
	// 8326A14C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A150: 3869E1B8  addi r3, r9, -0x1e48
	ctx.r[3].s64 = ctx.r[9].s64 + -7752;
	// 8326A154: 4BA3FDCD  bl 0x82ca9f20
	ctx.lr = 0x8326A158;
	sub_82CA9F20(ctx, base);
	// 8326A158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326A168 size=64
    let mut pc: u32 = 0x8326A168;
    'dispatch: loop {
        match pc {
            0x8326A168 => {
    //   block [0x8326A168..0x8326A17C)
	// 8326A168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326A16C: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 8326A170: 396BBC80  addi r11, r11, -0x4380
	ctx.r[11].s64 = ctx.r[11].s64 + -17280;
	// 8326A174: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326A178: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	pc = 0x8326A17C; continue 'dispatch;
            }
            0x8326A17C => {
    //   block [0x8326A17C..0x8326A1A8)
	// 8326A17C: 914BFFF8  stw r10, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 8326A180: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8326A184: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 8326A188: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326A18C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8326A190: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326A194: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8326A198: 994B0028  stb r10, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 8326A19C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 8326A1A0: 4080FFDC  bge 0x8326a17c
	if !ctx.cr[0].lt {
	pc = 0x8326A17C; continue 'dispatch;
	}
	// 8326A1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A1A8 size=64
    let mut pc: u32 = 0x8326A1A8;
    'dispatch: loop {
        match pc {
            0x8326A1A8 => {
    //   block [0x8326A1A8..0x8326A1E8)
	// 8326A1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A1B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A1B4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A1B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A1BC: 388BD110  addi r4, r11, -0x2ef0
	ctx.r[4].s64 = ctx.r[11].s64 + -12016;
	// 8326A1C0: 386AC140  addi r3, r10, -0x3ec0
	ctx.r[3].s64 = ctx.r[10].s64 + -16064;
	// 8326A1C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A1C8: 4AFC2D09  bl 0x8222ced0
	ctx.lr = 0x8326A1CC;
	sub_8222CED0(ctx, base);
	// 8326A1CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A1D0: 3869E1C8  addi r3, r9, -0x1e38
	ctx.r[3].s64 = ctx.r[9].s64 + -7736;
	// 8326A1D4: 4BA3FD4D  bl 0x82ca9f20
	ctx.lr = 0x8326A1D8;
	sub_82CA9F20(ctx, base);
	// 8326A1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326A1E8 size=12
    let mut pc: u32 = 0x8326A1E8;
    'dispatch: loop {
        match pc {
            0x8326A1E8 => {
    //   block [0x8326A1E8..0x8326A1F4)
	// 8326A1E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326A1EC: 386BE1D8  addi r3, r11, -0x1e28
	ctx.r[3].s64 = ctx.r[11].s64 + -7720;
	// 8326A1F0: 4BA3FD30  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A1F8 size=64
    let mut pc: u32 = 0x8326A1F8;
    'dispatch: loop {
        match pc {
            0x8326A1F8 => {
    //   block [0x8326A1F8..0x8326A238)
	// 8326A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A204: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326A208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A20C: 388B206C  addi r4, r11, 0x206c
	ctx.r[4].s64 = ctx.r[11].s64 + 8300;
	// 8326A210: 386AC154  addi r3, r10, -0x3eac
	ctx.r[3].s64 = ctx.r[10].s64 + -16044;
	// 8326A214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A218: 4AFC2CB9  bl 0x8222ced0
	ctx.lr = 0x8326A21C;
	sub_8222CED0(ctx, base);
	// 8326A21C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A220: 3869E240  addi r3, r9, -0x1dc0
	ctx.r[3].s64 = ctx.r[9].s64 + -7616;
	// 8326A224: 4BA3FCFD  bl 0x82ca9f20
	ctx.lr = 0x8326A228;
	sub_82CA9F20(ctx, base);
	// 8326A228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A238 size=56
    let mut pc: u32 = 0x8326A238;
    'dispatch: loop {
        match pc {
            0x8326A238 => {
    //   block [0x8326A238..0x8326A270)
	// 8326A238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A244: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326A248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326A24C: 386B8678  addi r3, r11, -0x7988
	ctx.r[3].s64 = ctx.r[11].s64 + -31112;
	// 8326A250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326A254: 4AF89B05  bl 0x821f3d58
	ctx.lr = 0x8326A258;
	sub_821F3D58(ctx, base);
	// 8326A258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A25C: 906AC158  stw r3, -0x3ea8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16040 as u32), ctx.r[3].u32 ) };
	// 8326A260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A270 size=56
    let mut pc: u32 = 0x8326A270;
    'dispatch: loop {
        match pc {
            0x8326A270 => {
    //   block [0x8326A270..0x8326A2A8)
	// 8326A270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A27C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326A284: 386BD83C  addi r3, r11, -0x27c4
	ctx.r[3].s64 = ctx.r[11].s64 + -10180;
	// 8326A288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326A28C: 4AF89ACD  bl 0x821f3d58
	ctx.lr = 0x8326A290;
	sub_821F3D58(ctx, base);
	// 8326A290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A294: 906AC15C  stw r3, -0x3ea4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16036 as u32), ctx.r[3].u32 ) };
	// 8326A298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A2A8 size=56
    let mut pc: u32 = 0x8326A2A8;
    'dispatch: loop {
        match pc {
            0x8326A2A8 => {
    //   block [0x8326A2A8..0x8326A2E0)
	// 8326A2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A2B4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326A2BC: 386BD848  addi r3, r11, -0x27b8
	ctx.r[3].s64 = ctx.r[11].s64 + -10168;
	// 8326A2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326A2C4: 4AF89A95  bl 0x821f3d58
	ctx.lr = 0x8326A2C8;
	sub_821F3D58(ctx, base);
	// 8326A2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A2CC: 906AC160  stw r3, -0x3ea0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16032 as u32), ctx.r[3].u32 ) };
	// 8326A2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A2E0 size=56
    let mut pc: u32 = 0x8326A2E0;
    'dispatch: loop {
        match pc {
            0x8326A2E0 => {
    //   block [0x8326A2E0..0x8326A318)
	// 8326A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A2EC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326A2F4: 386BD858  addi r3, r11, -0x27a8
	ctx.r[3].s64 = ctx.r[11].s64 + -10152;
	// 8326A2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326A2FC: 4AF89A5D  bl 0x821f3d58
	ctx.lr = 0x8326A300;
	sub_821F3D58(ctx, base);
	// 8326A300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A304: 906AC164  stw r3, -0x3e9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16028 as u32), ctx.r[3].u32 ) };
	// 8326A308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A318 size=64
    let mut pc: u32 = 0x8326A318;
    'dispatch: loop {
        match pc {
            0x8326A318 => {
    //   block [0x8326A318..0x8326A358)
	// 8326A318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A324: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326A328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A32C: 388B5D58  addi r4, r11, 0x5d58
	ctx.r[4].s64 = ctx.r[11].s64 + 23896;
	// 8326A330: 386AC168  addi r3, r10, -0x3e98
	ctx.r[3].s64 = ctx.r[10].s64 + -16024;
	// 8326A334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A338: 4AFC2B99  bl 0x8222ced0
	ctx.lr = 0x8326A33C;
	sub_8222CED0(ctx, base);
	// 8326A33C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A340: 3869E250  addi r3, r9, -0x1db0
	ctx.r[3].s64 = ctx.r[9].s64 + -7600;
	// 8326A344: 4BA3FBDD  bl 0x82ca9f20
	ctx.lr = 0x8326A348;
	sub_82CA9F20(ctx, base);
	// 8326A348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A358 size=56
    let mut pc: u32 = 0x8326A358;
    'dispatch: loop {
        match pc {
            0x8326A358 => {
    //   block [0x8326A358..0x8326A390)
	// 8326A358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A364: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A368: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326A36C: 386BD754  addi r3, r11, -0x28ac
	ctx.r[3].s64 = ctx.r[11].s64 + -10412;
	// 8326A370: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326A374: 4AF899E5  bl 0x821f3d58
	ctx.lr = 0x8326A378;
	sub_821F3D58(ctx, base);
	// 8326A378: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A37C: 906AC16C  stw r3, -0x3e94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16020 as u32), ctx.r[3].u32 ) };
	// 8326A380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A390 size=64
    let mut pc: u32 = 0x8326A390;
    'dispatch: loop {
        match pc {
            0x8326A390 => {
    //   block [0x8326A390..0x8326A3D0)
	// 8326A390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A39C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326A3A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A3A4: 388BA54C  addi r4, r11, -0x5ab4
	ctx.r[4].s64 = ctx.r[11].s64 + -23220;
	// 8326A3A8: 386AC170  addi r3, r10, -0x3e90
	ctx.r[3].s64 = ctx.r[10].s64 + -16016;
	// 8326A3AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A3B0: 4AFC2B21  bl 0x8222ced0
	ctx.lr = 0x8326A3B4;
	sub_8222CED0(ctx, base);
	// 8326A3B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A3B8: 3869E260  addi r3, r9, -0x1da0
	ctx.r[3].s64 = ctx.r[9].s64 + -7584;
	// 8326A3BC: 4BA3FB65  bl 0x82ca9f20
	ctx.lr = 0x8326A3C0;
	sub_82CA9F20(ctx, base);
	// 8326A3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A3D0 size=64
    let mut pc: u32 = 0x8326A3D0;
    'dispatch: loop {
        match pc {
            0x8326A3D0 => {
    //   block [0x8326A3D0..0x8326A410)
	// 8326A3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A3DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A3E4: 388BD868  addi r4, r11, -0x2798
	ctx.r[4].s64 = ctx.r[11].s64 + -10136;
	// 8326A3E8: 386AC174  addi r3, r10, -0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -16012;
	// 8326A3EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A3F0: 4AFC2AE1  bl 0x8222ced0
	ctx.lr = 0x8326A3F4;
	sub_8222CED0(ctx, base);
	// 8326A3F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A3F8: 3869E270  addi r3, r9, -0x1d90
	ctx.r[3].s64 = ctx.r[9].s64 + -7568;
	// 8326A3FC: 4BA3FB25  bl 0x82ca9f20
	ctx.lr = 0x8326A400;
	sub_82CA9F20(ctx, base);
	// 8326A400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A410 size=64
    let mut pc: u32 = 0x8326A410;
    'dispatch: loop {
        match pc {
            0x8326A410 => {
    //   block [0x8326A410..0x8326A450)
	// 8326A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A41C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A424: 388BD884  addi r4, r11, -0x277c
	ctx.r[4].s64 = ctx.r[11].s64 + -10108;
	// 8326A428: 386AC178  addi r3, r10, -0x3e88
	ctx.r[3].s64 = ctx.r[10].s64 + -16008;
	// 8326A42C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A430: 4AFC2AA1  bl 0x8222ced0
	ctx.lr = 0x8326A434;
	sub_8222CED0(ctx, base);
	// 8326A434: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A438: 3869E280  addi r3, r9, -0x1d80
	ctx.r[3].s64 = ctx.r[9].s64 + -7552;
	// 8326A43C: 4BA3FAE5  bl 0x82ca9f20
	ctx.lr = 0x8326A440;
	sub_82CA9F20(ctx, base);
	// 8326A440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A450 size=64
    let mut pc: u32 = 0x8326A450;
    'dispatch: loop {
        match pc {
            0x8326A450 => {
    //   block [0x8326A450..0x8326A490)
	// 8326A450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A45C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A464: 388BD8A4  addi r4, r11, -0x275c
	ctx.r[4].s64 = ctx.r[11].s64 + -10076;
	// 8326A468: 386AC17C  addi r3, r10, -0x3e84
	ctx.r[3].s64 = ctx.r[10].s64 + -16004;
	// 8326A46C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A470: 4AFC2A61  bl 0x8222ced0
	ctx.lr = 0x8326A474;
	sub_8222CED0(ctx, base);
	// 8326A474: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A478: 3869E290  addi r3, r9, -0x1d70
	ctx.r[3].s64 = ctx.r[9].s64 + -7536;
	// 8326A47C: 4BA3FAA5  bl 0x82ca9f20
	ctx.lr = 0x8326A480;
	sub_82CA9F20(ctx, base);
	// 8326A480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A490 size=64
    let mut pc: u32 = 0x8326A490;
    'dispatch: loop {
        match pc {
            0x8326A490 => {
    //   block [0x8326A490..0x8326A4D0)
	// 8326A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A49C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A4A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A4A4: 388BD8B4  addi r4, r11, -0x274c
	ctx.r[4].s64 = ctx.r[11].s64 + -10060;
	// 8326A4A8: 386AC180  addi r3, r10, -0x3e80
	ctx.r[3].s64 = ctx.r[10].s64 + -16000;
	// 8326A4AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A4B0: 4AFC2A21  bl 0x8222ced0
	ctx.lr = 0x8326A4B4;
	sub_8222CED0(ctx, base);
	// 8326A4B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A4B8: 3869E2A0  addi r3, r9, -0x1d60
	ctx.r[3].s64 = ctx.r[9].s64 + -7520;
	// 8326A4BC: 4BA3FA65  bl 0x82ca9f20
	ctx.lr = 0x8326A4C0;
	sub_82CA9F20(ctx, base);
	// 8326A4C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A4D0 size=64
    let mut pc: u32 = 0x8326A4D0;
    'dispatch: loop {
        match pc {
            0x8326A4D0 => {
    //   block [0x8326A4D0..0x8326A510)
	// 8326A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A4D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A4DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A4E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A4E4: 388BD8C4  addi r4, r11, -0x273c
	ctx.r[4].s64 = ctx.r[11].s64 + -10044;
	// 8326A4E8: 386AC184  addi r3, r10, -0x3e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15996;
	// 8326A4EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A4F0: 4AFC29E1  bl 0x8222ced0
	ctx.lr = 0x8326A4F4;
	sub_8222CED0(ctx, base);
	// 8326A4F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A4F8: 3869E2B0  addi r3, r9, -0x1d50
	ctx.r[3].s64 = ctx.r[9].s64 + -7504;
	// 8326A4FC: 4BA3FA25  bl 0x82ca9f20
	ctx.lr = 0x8326A500;
	sub_82CA9F20(ctx, base);
	// 8326A500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A510 size=64
    let mut pc: u32 = 0x8326A510;
    'dispatch: loop {
        match pc {
            0x8326A510 => {
    //   block [0x8326A510..0x8326A550)
	// 8326A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A51C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A520: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A524: 388BD8D8  addi r4, r11, -0x2728
	ctx.r[4].s64 = ctx.r[11].s64 + -10024;
	// 8326A528: 386AC188  addi r3, r10, -0x3e78
	ctx.r[3].s64 = ctx.r[10].s64 + -15992;
	// 8326A52C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A530: 4AFC29A1  bl 0x8222ced0
	ctx.lr = 0x8326A534;
	sub_8222CED0(ctx, base);
	// 8326A534: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A538: 3869E2C0  addi r3, r9, -0x1d40
	ctx.r[3].s64 = ctx.r[9].s64 + -7488;
	// 8326A53C: 4BA3F9E5  bl 0x82ca9f20
	ctx.lr = 0x8326A540;
	sub_82CA9F20(ctx, base);
	// 8326A540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A550 size=64
    let mut pc: u32 = 0x8326A550;
    'dispatch: loop {
        match pc {
            0x8326A550 => {
    //   block [0x8326A550..0x8326A590)
	// 8326A550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A55C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A564: 388BD8E8  addi r4, r11, -0x2718
	ctx.r[4].s64 = ctx.r[11].s64 + -10008;
	// 8326A568: 386AC18C  addi r3, r10, -0x3e74
	ctx.r[3].s64 = ctx.r[10].s64 + -15988;
	// 8326A56C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A570: 4AFC2961  bl 0x8222ced0
	ctx.lr = 0x8326A574;
	sub_8222CED0(ctx, base);
	// 8326A574: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A578: 3869E2D0  addi r3, r9, -0x1d30
	ctx.r[3].s64 = ctx.r[9].s64 + -7472;
	// 8326A57C: 4BA3F9A5  bl 0x82ca9f20
	ctx.lr = 0x8326A580;
	sub_82CA9F20(ctx, base);
	// 8326A580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A590 size=64
    let mut pc: u32 = 0x8326A590;
    'dispatch: loop {
        match pc {
            0x8326A590 => {
    //   block [0x8326A590..0x8326A5D0)
	// 8326A590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A59C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A5A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A5A4: 388BD900  addi r4, r11, -0x2700
	ctx.r[4].s64 = ctx.r[11].s64 + -9984;
	// 8326A5A8: 386AC190  addi r3, r10, -0x3e70
	ctx.r[3].s64 = ctx.r[10].s64 + -15984;
	// 8326A5AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A5B0: 4AFC2921  bl 0x8222ced0
	ctx.lr = 0x8326A5B4;
	sub_8222CED0(ctx, base);
	// 8326A5B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A5B8: 3869E2E0  addi r3, r9, -0x1d20
	ctx.r[3].s64 = ctx.r[9].s64 + -7456;
	// 8326A5BC: 4BA3F965  bl 0x82ca9f20
	ctx.lr = 0x8326A5C0;
	sub_82CA9F20(ctx, base);
	// 8326A5C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A5D0 size=64
    let mut pc: u32 = 0x8326A5D0;
    'dispatch: loop {
        match pc {
            0x8326A5D0 => {
    //   block [0x8326A5D0..0x8326A610)
	// 8326A5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A5D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A5DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A5E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A5E4: 388BD914  addi r4, r11, -0x26ec
	ctx.r[4].s64 = ctx.r[11].s64 + -9964;
	// 8326A5E8: 386AC194  addi r3, r10, -0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15980;
	// 8326A5EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A5F0: 4AFC28E1  bl 0x8222ced0
	ctx.lr = 0x8326A5F4;
	sub_8222CED0(ctx, base);
	// 8326A5F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A5F8: 3869E2F0  addi r3, r9, -0x1d10
	ctx.r[3].s64 = ctx.r[9].s64 + -7440;
	// 8326A5FC: 4BA3F925  bl 0x82ca9f20
	ctx.lr = 0x8326A600;
	sub_82CA9F20(ctx, base);
	// 8326A600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A610 size=64
    let mut pc: u32 = 0x8326A610;
    'dispatch: loop {
        match pc {
            0x8326A610 => {
    //   block [0x8326A610..0x8326A650)
	// 8326A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A61C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A624: 388BD930  addi r4, r11, -0x26d0
	ctx.r[4].s64 = ctx.r[11].s64 + -9936;
	// 8326A628: 386AC198  addi r3, r10, -0x3e68
	ctx.r[3].s64 = ctx.r[10].s64 + -15976;
	// 8326A62C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A630: 4AFC28A1  bl 0x8222ced0
	ctx.lr = 0x8326A634;
	sub_8222CED0(ctx, base);
	// 8326A634: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A638: 3869E300  addi r3, r9, -0x1d00
	ctx.r[3].s64 = ctx.r[9].s64 + -7424;
	// 8326A63C: 4BA3F8E5  bl 0x82ca9f20
	ctx.lr = 0x8326A640;
	sub_82CA9F20(ctx, base);
	// 8326A640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A650 size=64
    let mut pc: u32 = 0x8326A650;
    'dispatch: loop {
        match pc {
            0x8326A650 => {
    //   block [0x8326A650..0x8326A690)
	// 8326A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A65C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A660: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A664: 388BD944  addi r4, r11, -0x26bc
	ctx.r[4].s64 = ctx.r[11].s64 + -9916;
	// 8326A668: 386AC19C  addi r3, r10, -0x3e64
	ctx.r[3].s64 = ctx.r[10].s64 + -15972;
	// 8326A66C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A670: 4AFC2861  bl 0x8222ced0
	ctx.lr = 0x8326A674;
	sub_8222CED0(ctx, base);
	// 8326A674: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A678: 3869E310  addi r3, r9, -0x1cf0
	ctx.r[3].s64 = ctx.r[9].s64 + -7408;
	// 8326A67C: 4BA3F8A5  bl 0x82ca9f20
	ctx.lr = 0x8326A680;
	sub_82CA9F20(ctx, base);
	// 8326A680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


